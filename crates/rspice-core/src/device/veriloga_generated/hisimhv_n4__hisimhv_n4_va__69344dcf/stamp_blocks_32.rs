#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_137(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign41520_e55271, assign41520_e55271_d_n0, assign41520_e55271_d_n2, assign41520_e55271_d_n4, assign41520_e55271_d_n5, assign41520_e55271_d_n6, assign41520_e55271_d_n7, assign41520_e55271_d_n8, assign41520_e55271_d_n9, assign41520_e55271_d_n10, assign41520_e55271_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1031 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign41520_e55271;
        locals.var_t0_dn0 = assign41520_e55271_d_n0;
        locals.var_t0_dn2 = assign41520_e55271_d_n2;
        locals.var_t0_dn4 = assign41520_e55271_d_n4;
        locals.var_t0_dn5 = assign41520_e55271_d_n5;
        locals.var_t0_dn6 = assign41520_e55271_d_n6;
        locals.var_t0_dn7 = assign41520_e55271_d_n7;
        locals.var_t0_dn8 = assign41520_e55271_d_n8;
        locals.var_t0_dn9 = assign41520_e55271_d_n9;
        locals.var_t0_dn10 = assign41520_e55271_d_n10;
        locals.var_t0_dn13 = assign41520_e55271_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign41530_e55286, assign41530_e55286_d_n0, assign41530_e55286_d_n2, assign41530_e55286_d_n4, assign41530_e55286_d_n5, assign41530_e55286_d_n6, assign41530_e55286_d_n7, assign41530_e55286_d_n8, assign41530_e55286_d_n9, assign41530_e55286_d_n10, assign41530_e55286_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1031 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign41530_e55286;
        locals.var_ps0dep_dn0 = assign41530_e55286_d_n0;
        locals.var_ps0dep_dn2 = assign41530_e55286_d_n2;
        locals.var_ps0dep_dn4 = assign41530_e55286_d_n4;
        locals.var_ps0dep_dn5 = assign41530_e55286_d_n5;
        locals.var_ps0dep_dn6 = assign41530_e55286_d_n6;
        locals.var_ps0dep_dn7 = assign41530_e55286_d_n7;
        locals.var_ps0dep_dn8 = assign41530_e55286_d_n8;
        locals.var_ps0dep_dn9 = assign41530_e55286_d_n9;
        locals.var_ps0dep_dn10 = assign41530_e55286_d_n10;
        locals.var_ps0dep_dn13 = assign41530_e55286_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign41540_e55301, assign41540_e55301_d_n0, assign41540_e55301_d_n2, assign41540_e55301_d_n4, assign41540_e55301_d_n5, assign41540_e55301_d_n6, assign41540_e55301_d_n7, assign41540_e55301_d_n8, assign41540_e55301_d_n9, assign41540_e55301_d_n10, assign41540_e55301_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1031 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign41540_e55301;
        locals.var_t0_dn0 = assign41540_e55301_d_n0;
        locals.var_t0_dn2 = assign41540_e55301_d_n2;
        locals.var_t0_dn4 = assign41540_e55301_d_n4;
        locals.var_t0_dn5 = assign41540_e55301_d_n5;
        locals.var_t0_dn6 = assign41540_e55301_d_n6;
        locals.var_t0_dn7 = assign41540_e55301_d_n7;
        locals.var_t0_dn8 = assign41540_e55301_d_n8;
        locals.var_t0_dn9 = assign41540_e55301_d_n9;
        locals.var_t0_dn10 = assign41540_e55301_d_n10;
        locals.var_t0_dn13 = assign41540_e55301_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign41550_e55321, assign41550_e55321_d_n0, assign41550_e55321_d_n2, assign41550_e55321_d_n4, assign41550_e55321_d_n5, assign41550_e55321_d_n6, assign41550_e55321_d_n7, assign41550_e55321_d_n8, assign41550_e55321_d_n9, assign41550_e55321_d_n10, assign41550_e55321_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41550_e55313: f64 = (locals.var_vgs - locals.var_vgp);
        let assign41550_e55316: f64 = (locals.var_uc_vfbc - p.p392);
        let assign41550_e55318: f64 = (assign41550_e55316 - locals.var_vfboffset);
        let assign41550_e55319: f64 = (assign41550_e55313 - assign41550_e55318);
        (assign41550_e55319, (-locals.var_vgp_dn0), (-locals.var_vgp_dn2), (-locals.var_vgp_dn4), (locals.var_vgs_dn5 - locals.var_vgp_dn5), (locals.var_vgs_dn6 - locals.var_vgp_dn6), (locals.var_vgs_dn7 - locals.var_vgp_dn7), (-locals.var_vgp_dn8), (-locals.var_vgp_dn9), (-locals.var_vgp_dn10), (-locals.var_vgp_dn13),)
    } else {
        (locals.var_vfb_res, locals.var_vfb_res_dn0, locals.var_vfb_res_dn2, locals.var_vfb_res_dn4, locals.var_vfb_res_dn5, locals.var_vfb_res_dn6, locals.var_vfb_res_dn7, locals.var_vfb_res_dn8, locals.var_vfb_res_dn9, locals.var_vfb_res_dn10, locals.var_vfb_res_dn13,)
    }
};
        locals.var_vfb_res = assign41550_e55321;
        locals.var_vfb_res_dn0 = assign41550_e55321_d_n0;
        locals.var_vfb_res_dn2 = assign41550_e55321_d_n2;
        locals.var_vfb_res_dn4 = assign41550_e55321_d_n4;
        locals.var_vfb_res_dn5 = assign41550_e55321_d_n5;
        locals.var_vfb_res_dn6 = assign41550_e55321_d_n6;
        locals.var_vfb_res_dn7 = assign41550_e55321_d_n7;
        locals.var_vfb_res_dn8 = assign41550_e55321_d_n8;
        locals.var_vfb_res_dn9 = assign41550_e55321_d_n9;
        locals.var_vfb_res_dn10 = assign41550_e55321_d_n10;
        locals.var_vfb_res_dn13 = assign41550_e55321_d_n13;
        locals.var_vfb_res_rv = 0.0;

        let (assign41560_e55335, assign41560_e55335_d_n0, assign41560_e55335_d_n2, assign41560_e55335_d_n4, assign41560_e55335_d_n5, assign41560_e55335_d_n6, assign41560_e55335_d_n7, assign41560_e55335_d_n8, assign41560_e55335_d_n9, assign41560_e55335_d_n10, assign41560_e55335_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41560_e55333: f64 = (locals.var_vgs - locals.var_vfb_res);
        (assign41560_e55333, (-locals.var_vfb_res_dn0), (-locals.var_vfb_res_dn2), (-locals.var_vfb_res_dn4), (locals.var_vgs_dn5 - locals.var_vfb_res_dn5), (locals.var_vgs_dn6 - locals.var_vfb_res_dn6), (locals.var_vgs_dn7 - locals.var_vfb_res_dn7), (-locals.var_vfb_res_dn8), (-locals.var_vfb_res_dn9), (-locals.var_vfb_res_dn10), (-locals.var_vfb_res_dn13),)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn13,)
    }
};
        locals.var_vgp_res = assign41560_e55335;
        locals.var_vgp_res_dn0 = assign41560_e55335_d_n0;
        locals.var_vgp_res_dn2 = assign41560_e55335_d_n2;
        locals.var_vgp_res_dn4 = assign41560_e55335_d_n4;
        locals.var_vgp_res_dn5 = assign41560_e55335_d_n5;
        locals.var_vgp_res_dn6 = assign41560_e55335_d_n6;
        locals.var_vgp_res_dn7 = assign41560_e55335_d_n7;
        locals.var_vgp_res_dn8 = assign41560_e55335_d_n8;
        locals.var_vgp_res_dn9 = assign41560_e55335_d_n9;
        locals.var_vgp_res_dn10 = assign41560_e55335_d_n10;
        locals.var_vgp_res_dn13 = assign41560_e55335_d_n13;
        locals.var_vgp_res_rv = 0.0;

        let assign41570_e55339: f64 = (-locals.var_vgpdep_dlt);
        let assign41570_e55344: f64 = if ((locals.var_vgp_res > assign41570_e55339) && (locals.var_vgpdep_dlt >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1037 = assign41570_e55344;
        locals.var_guard1037_rv = 0.0;

        let (assign41580_e55362, assign41580_e55362_d_n0, assign41580_e55362_d_n2, assign41580_e55362_d_n4, assign41580_e55362_d_n5, assign41580_e55362_d_n6, assign41580_e55362_d_n7, assign41580_e55362_d_n8, assign41580_e55362_d_n9, assign41580_e55362_d_n10, assign41580_e55362_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41580_e55358: f64 = locals.var_vgp_res;
        let assign41580_e55360: f64 = (assign41580_e55358 + locals.var_vgpdep_dlt);
        (assign41580_e55360, (locals.var_vgp_res_dn0 + locals.var_vgpdep_dlt_dn0), (locals.var_vgp_res_dn2 + locals.var_vgpdep_dlt_dn2), (locals.var_vgp_res_dn4 + locals.var_vgpdep_dlt_dn4), (locals.var_vgp_res_dn5 + locals.var_vgpdep_dlt_dn5), (locals.var_vgp_res_dn6 + locals.var_vgpdep_dlt_dn6), (locals.var_vgp_res_dn7 + locals.var_vgpdep_dlt_dn7), (locals.var_vgp_res_dn8 + locals.var_vgpdep_dlt_dn8), (locals.var_vgp_res_dn9 + locals.var_vgpdep_dlt_dn9), (locals.var_vgp_res_dn10 + locals.var_vgpdep_dlt_dn10), (locals.var_vgp_res_dn13 + locals.var_vgpdep_dlt_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign41580_e55362;
        locals.var_tmf1_dn0 = assign41580_e55362_d_n0;
        locals.var_tmf1_dn2 = assign41580_e55362_d_n2;
        locals.var_tmf1_dn4 = assign41580_e55362_d_n4;
        locals.var_tmf1_dn5 = assign41580_e55362_d_n5;
        locals.var_tmf1_dn6 = assign41580_e55362_d_n6;
        locals.var_tmf1_dn7 = assign41580_e55362_d_n7;
        locals.var_tmf1_dn8 = assign41580_e55362_d_n8;
        locals.var_tmf1_dn9 = assign41580_e55362_d_n9;
        locals.var_tmf1_dn10 = assign41580_e55362_d_n10;
        locals.var_tmf1_dn13 = assign41580_e55362_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign41590_e55378, assign41590_e55378_d_n0, assign41590_e55378_d_n2, assign41590_e55378_d_n4, assign41590_e55378_d_n5, assign41590_e55378_d_n6, assign41590_e55378_d_n7, assign41590_e55378_d_n8, assign41590_e55378_d_n9, assign41590_e55378_d_n10, assign41590_e55378_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41590_e55376: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign41590_e55376, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign41590_e55378;
        locals.var_x2_dn0 = assign41590_e55378_d_n0;
        locals.var_x2_dn2 = assign41590_e55378_d_n2;
        locals.var_x2_dn4 = assign41590_e55378_d_n4;
        locals.var_x2_dn5 = assign41590_e55378_d_n5;
        locals.var_x2_dn6 = assign41590_e55378_d_n6;
        locals.var_x2_dn7 = assign41590_e55378_d_n7;
        locals.var_x2_dn8 = assign41590_e55378_d_n8;
        locals.var_x2_dn9 = assign41590_e55378_d_n9;
        locals.var_x2_dn10 = assign41590_e55378_d_n10;
        locals.var_x2_dn13 = assign41590_e55378_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign41600_e55394, assign41600_e55394_d_n0, assign41600_e55394_d_n2, assign41600_e55394_d_n4, assign41600_e55394_d_n5, assign41600_e55394_d_n6, assign41600_e55394_d_n7, assign41600_e55394_d_n8, assign41600_e55394_d_n9, assign41600_e55394_d_n10, assign41600_e55394_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41600_e55392: f64 = (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt);
        (assign41600_e55392, ((locals.var_vgpdep_dlt_dn0 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn0)), ((locals.var_vgpdep_dlt_dn2 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn2)), ((locals.var_vgpdep_dlt_dn4 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn4)), ((locals.var_vgpdep_dlt_dn5 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn5)), ((locals.var_vgpdep_dlt_dn6 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn6)), ((locals.var_vgpdep_dlt_dn7 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn7)), ((locals.var_vgpdep_dlt_dn8 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn8)), ((locals.var_vgpdep_dlt_dn9 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn9)), ((locals.var_vgpdep_dlt_dn10 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn10)), ((locals.var_vgpdep_dlt_dn13 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign41600_e55394;
        locals.var_xmax2_dn0 = assign41600_e55394_d_n0;
        locals.var_xmax2_dn2 = assign41600_e55394_d_n2;
        locals.var_xmax2_dn4 = assign41600_e55394_d_n4;
        locals.var_xmax2_dn5 = assign41600_e55394_d_n5;
        locals.var_xmax2_dn6 = assign41600_e55394_d_n6;
        locals.var_xmax2_dn7 = assign41600_e55394_d_n7;
        locals.var_xmax2_dn8 = assign41600_e55394_d_n8;
        locals.var_xmax2_dn9 = assign41600_e55394_d_n9;
        locals.var_xmax2_dn10 = assign41600_e55394_d_n10;
        locals.var_xmax2_dn13 = assign41600_e55394_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign41610_e55408, assign41610_e55408_d_n0, assign41610_e55408_d_n2, assign41610_e55408_d_n4, assign41610_e55408_d_n5, assign41610_e55408_d_n6, assign41610_e55408_d_n7, assign41610_e55408_d_n8, assign41610_e55408_d_n9, assign41610_e55408_d_n10, assign41610_e55408_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign41610_e55408;
        locals.var_xp_dn0 = assign41610_e55408_d_n0;
        locals.var_xp_dn2 = assign41610_e55408_d_n2;
        locals.var_xp_dn4 = assign41610_e55408_d_n4;
        locals.var_xp_dn5 = assign41610_e55408_d_n5;
        locals.var_xp_dn6 = assign41610_e55408_d_n6;
        locals.var_xp_dn7 = assign41610_e55408_d_n7;
        locals.var_xp_dn8 = assign41610_e55408_d_n8;
        locals.var_xp_dn9 = assign41610_e55408_d_n9;
        locals.var_xp_dn10 = assign41610_e55408_d_n10;
        locals.var_xp_dn13 = assign41610_e55408_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign41620_e55422, assign41620_e55422_d_n0, assign41620_e55422_d_n2, assign41620_e55422_d_n4, assign41620_e55422_d_n5, assign41620_e55422_d_n6, assign41620_e55422_d_n7, assign41620_e55422_d_n8, assign41620_e55422_d_n9, assign41620_e55422_d_n10, assign41620_e55422_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign41620_e55422;
        locals.var_xmp_dn0 = assign41620_e55422_d_n0;
        locals.var_xmp_dn2 = assign41620_e55422_d_n2;
        locals.var_xmp_dn4 = assign41620_e55422_d_n4;
        locals.var_xmp_dn5 = assign41620_e55422_d_n5;
        locals.var_xmp_dn6 = assign41620_e55422_d_n6;
        locals.var_xmp_dn7 = assign41620_e55422_d_n7;
        locals.var_xmp_dn8 = assign41620_e55422_d_n8;
        locals.var_xmp_dn9 = assign41620_e55422_d_n9;
        locals.var_xmp_dn10 = assign41620_e55422_d_n10;
        locals.var_xmp_dn13 = assign41620_e55422_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign41630_e55436,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41630_e55436;
        locals.var_m0_rv = 0.0;

        let (assign41640_e55450,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41640_e55450;
        locals.var_mm_rv = 0.0;

        let (assign41650_e55464, assign41650_e55464_d_n0, assign41650_e55464_d_n2, assign41650_e55464_d_n4, assign41650_e55464_d_n5, assign41650_e55464_d_n6, assign41650_e55464_d_n7, assign41650_e55464_d_n8, assign41650_e55464_d_n9, assign41650_e55464_d_n10, assign41650_e55464_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign41650_e55464;
        locals.var_arg_dn0 = assign41650_e55464_d_n0;
        locals.var_arg_dn2 = assign41650_e55464_d_n2;
        locals.var_arg_dn4 = assign41650_e55464_d_n4;
        locals.var_arg_dn5 = assign41650_e55464_d_n5;
        locals.var_arg_dn6 = assign41650_e55464_d_n6;
        locals.var_arg_dn7 = assign41650_e55464_d_n7;
        locals.var_arg_dn8 = assign41650_e55464_d_n8;
        locals.var_arg_dn9 = assign41650_e55464_d_n9;
        locals.var_arg_dn10 = assign41650_e55464_d_n10;
        locals.var_arg_dn13 = assign41650_e55464_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign41660_e55478, assign41660_e55478_d_n0, assign41660_e55478_d_n2, assign41660_e55478_d_n4, assign41660_e55478_d_n5, assign41660_e55478_d_n6, assign41660_e55478_d_n7, assign41660_e55478_d_n8, assign41660_e55478_d_n9, assign41660_e55478_d_n10, assign41660_e55478_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign41660_e55478;
        locals.var_dnm_dn0 = assign41660_e55478_d_n0;
        locals.var_dnm_dn2 = assign41660_e55478_d_n2;
        locals.var_dnm_dn4 = assign41660_e55478_d_n4;
        locals.var_dnm_dn5 = assign41660_e55478_d_n5;
        locals.var_dnm_dn6 = assign41660_e55478_d_n6;
        locals.var_dnm_dn7 = assign41660_e55478_d_n7;
        locals.var_dnm_dn8 = assign41660_e55478_d_n8;
        locals.var_dnm_dn9 = assign41660_e55478_d_n9;
        locals.var_dnm_dn10 = assign41660_e55478_d_n10;
        locals.var_dnm_dn13 = assign41660_e55478_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign41670_e55492,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41670_e55492;
        locals.var_m0_rv = 0.0;

        let mut assign41680_loop_guard: usize = 0;
        while {
            let assign41680_cond_e55507: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw)) { 1.0 } else { 0.0 };
            assign41680_cond_e55507 != 0.0
        } {
            assign41680_loop_guard += 1;
            assert!(assign41680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41680_body0_e55523, assign41680_body0_e55523_d_n0, assign41680_body0_e55523_d_n2, assign41680_body0_e55523_d_n4, assign41680_body0_e55523_d_n5, assign41680_body0_e55523_d_n6, assign41680_body0_e55523_d_n7, assign41680_body0_e55523_d_n8, assign41680_body0_e55523_d_n9, assign41680_body0_e55523_d_n10, assign41680_body0_e55523_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41680_body0_e55521: f64 = (locals.var_xp * locals.var_x2);
        (assign41680_body0_e55521, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign41680_body0_e55523;
            locals.var_xp_dn0 = assign41680_body0_e55523_d_n0;
            locals.var_xp_dn2 = assign41680_body0_e55523_d_n2;
            locals.var_xp_dn4 = assign41680_body0_e55523_d_n4;
            locals.var_xp_dn5 = assign41680_body0_e55523_d_n5;
            locals.var_xp_dn6 = assign41680_body0_e55523_d_n6;
            locals.var_xp_dn7 = assign41680_body0_e55523_d_n7;
            locals.var_xp_dn8 = assign41680_body0_e55523_d_n8;
            locals.var_xp_dn9 = assign41680_body0_e55523_d_n9;
            locals.var_xp_dn10 = assign41680_body0_e55523_d_n10;
            locals.var_xp_dn13 = assign41680_body0_e55523_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign41680_body1_e55539, assign41680_body1_e55539_d_n0, assign41680_body1_e55539_d_n2, assign41680_body1_e55539_d_n4, assign41680_body1_e55539_d_n5, assign41680_body1_e55539_d_n6, assign41680_body1_e55539_d_n7, assign41680_body1_e55539_d_n8, assign41680_body1_e55539_d_n9, assign41680_body1_e55539_d_n10, assign41680_body1_e55539_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41680_body1_e55537: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign41680_body1_e55537, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign41680_body1_e55539;
            locals.var_xmp_dn0 = assign41680_body1_e55539_d_n0;
            locals.var_xmp_dn2 = assign41680_body1_e55539_d_n2;
            locals.var_xmp_dn4 = assign41680_body1_e55539_d_n4;
            locals.var_xmp_dn5 = assign41680_body1_e55539_d_n5;
            locals.var_xmp_dn6 = assign41680_body1_e55539_d_n6;
            locals.var_xmp_dn7 = assign41680_body1_e55539_d_n7;
            locals.var_xmp_dn8 = assign41680_body1_e55539_d_n8;
            locals.var_xmp_dn9 = assign41680_body1_e55539_d_n9;
            locals.var_xmp_dn10 = assign41680_body1_e55539_d_n10;
            locals.var_xmp_dn13 = assign41680_body1_e55539_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign41680_body2_e55555,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41680_body2_e55553: f64 = (locals.var_m0 + 1.0);
        (assign41680_body2_e55553,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41680_body2_e55555;
            locals.var_m0_rv = 0.0;
        }

        let (assign41690_e55571, assign41690_e55571_d_n0, assign41690_e55571_d_n2, assign41690_e55571_d_n4, assign41690_e55571_d_n5, assign41690_e55571_d_n6, assign41690_e55571_d_n7, assign41690_e55571_d_n8, assign41690_e55571_d_n9, assign41690_e55571_d_n10, assign41690_e55571_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41690_e55569: f64 = (locals.var_xp + locals.var_xmp);
        (assign41690_e55569, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign41690_e55571;
        locals.var_arg_dn0 = assign41690_e55571_d_n0;
        locals.var_arg_dn2 = assign41690_e55571_d_n2;
        locals.var_arg_dn4 = assign41690_e55571_d_n4;
        locals.var_arg_dn5 = assign41690_e55571_d_n5;
        locals.var_arg_dn6 = assign41690_e55571_d_n6;
        locals.var_arg_dn7 = assign41690_e55571_d_n7;
        locals.var_arg_dn8 = assign41690_e55571_d_n8;
        locals.var_arg_dn9 = assign41690_e55571_d_n9;
        locals.var_arg_dn10 = assign41690_e55571_d_n10;
        locals.var_arg_dn13 = assign41690_e55571_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign41700_e55585, assign41700_e55585_d_n0, assign41700_e55585_d_n2, assign41700_e55585_d_n4, assign41700_e55585_d_n5, assign41700_e55585_d_n6, assign41700_e55585_d_n7, assign41700_e55585_d_n8, assign41700_e55585_d_n9, assign41700_e55585_d_n10, assign41700_e55585_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign41700_e55585;
        locals.var_dnm_dn0 = assign41700_e55585_d_n0;
        locals.var_dnm_dn2 = assign41700_e55585_d_n2;
        locals.var_dnm_dn4 = assign41700_e55585_d_n4;
        locals.var_dnm_dn5 = assign41700_e55585_d_n5;
        locals.var_dnm_dn6 = assign41700_e55585_d_n6;
        locals.var_dnm_dn7 = assign41700_e55585_d_n7;
        locals.var_dnm_dn8 = assign41700_e55585_d_n8;
        locals.var_dnm_dn9 = assign41700_e55585_d_n9;
        locals.var_dnm_dn10 = assign41700_e55585_d_n10;
        locals.var_dnm_dn13 = assign41700_e55585_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign41710_e55600: f64 = if ((((locals.var_vgpdep_pw == 1.0) || (locals.var_vgpdep_pw == 2.0)) || (locals.var_vgpdep_pw == 4.0)) || (locals.var_vgpdep_pw == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1038 = assign41710_e55600;
        locals.var_guard1038_rv = 0.0;

        let assign41720_e55603: f64 = if locals.var_vgpdep_pw == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1039 = assign41720_e55603;
        locals.var_guard1039_rv = 0.0;

        let (assign41730_e55621,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) && (locals.var_guard1039 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41730_e55621;
        locals.var_mm_rv = 0.0;

        let assign41740_e55624: f64 = if locals.var_vgpdep_pw == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1040 = assign41740_e55624;
        locals.var_guard1040_rv = 0.0;

        let (assign41750_e55645,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) && (locals.var_guard1039 == 0.0)) && (locals.var_guard1040 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41750_e55645;
        locals.var_mm_rv = 0.0;

        let assign41760_e55648: f64 = if locals.var_vgpdep_pw == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1041 = assign41760_e55648;
        locals.var_guard1041_rv = 0.0;

        let (assign41770_e55672,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) && (locals.var_guard1039 == 0.0)) && (locals.var_guard1040 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41770_e55672;
        locals.var_mm_rv = 0.0;

        let assign41780_e55675: f64 = if locals.var_vgpdep_pw == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1042 = assign41780_e55675;
        locals.var_guard1042_rv = 0.0;

        let (assign41790_e55702,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) && (locals.var_guard1039 == 0.0)) && (locals.var_guard1040 == 0.0)) && (locals.var_guard1041 == 0.0)) && (locals.var_guard1042 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41790_e55702;
        locals.var_mm_rv = 0.0;

        let (assign41800_e55718,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41800_e55718;
        locals.var_m0_rv = 0.0;

        let mut assign41810_loop_guard: usize = 0;
        while {
            let assign41810_cond_e55735: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign41810_cond_e55735 != 0.0
        } {
            assign41810_loop_guard += 1;
            assert!(assign41810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41810_body0_e55752, assign41810_body0_e55752_d_n0, assign41810_body0_e55752_d_n2, assign41810_body0_e55752_d_n4, assign41810_body0_e55752_d_n5, assign41810_body0_e55752_d_n6, assign41810_body0_e55752_d_n7, assign41810_body0_e55752_d_n8, assign41810_body0_e55752_d_n9, assign41810_body0_e55752_d_n10, assign41810_body0_e55752_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) {
        let assign41810_body0_e55750: f64 = (locals.var_dnm).sqrt();
        (assign41810_body0_e55750, (locals.var_dnm_dn0 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn2 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn4 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn5 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn6 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn7 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn8 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn9 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn10 / (2.0 * assign41810_body0_e55750)), (locals.var_dnm_dn13 / (2.0 * assign41810_body0_e55750)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign41810_body0_e55752;
            locals.var_dnm_dn0 = assign41810_body0_e55752_d_n0;
            locals.var_dnm_dn2 = assign41810_body0_e55752_d_n2;
            locals.var_dnm_dn4 = assign41810_body0_e55752_d_n4;
            locals.var_dnm_dn5 = assign41810_body0_e55752_d_n5;
            locals.var_dnm_dn6 = assign41810_body0_e55752_d_n6;
            locals.var_dnm_dn7 = assign41810_body0_e55752_d_n7;
            locals.var_dnm_dn8 = assign41810_body0_e55752_d_n8;
            locals.var_dnm_dn9 = assign41810_body0_e55752_d_n9;
            locals.var_dnm_dn10 = assign41810_body0_e55752_d_n10;
            locals.var_dnm_dn13 = assign41810_body0_e55752_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign41810_body1_e55770,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 != 0.0)) {
        let assign41810_body1_e55768: f64 = (locals.var_m0 + 1.0);
        (assign41810_body1_e55768,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41810_body1_e55770;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_138(
        locals: &mut StampLocals,
    ) {
        let (assign41820_e55798, assign41820_e55798_d_n0, assign41820_e55798_d_n2, assign41820_e55798_d_n4, assign41820_e55798_d_n5, assign41820_e55798_d_n6, assign41820_e55798_d_n7, assign41820_e55798_d_n8, assign41820_e55798_d_n9, assign41820_e55798_d_n10, assign41820_e55798_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) && (locals.var_guard1038 == 0.0)) {
        let (assign41820_e55796, assign41820_e55796_d_n0, assign41820_e55796_d_n2, assign41820_e55796_d_n4, assign41820_e55796_d_n5, assign41820_e55796_d_n6, assign41820_e55796_d_n7, assign41820_e55796_d_n8, assign41820_e55796_d_n9, assign41820_e55796_d_n10, assign41820_e55796_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign41820_e55793: f64 = (2.0 * locals.var_vgpdep_pw);
                let assign41820_e55794: f64 = (1.0 / assign41820_e55793);
                let assign41820_e55795: f64 = (locals.var_dnm).powf(assign41820_e55794);
                (assign41820_e55795, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn0)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn2)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn4)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn5)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn6)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn7)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn8)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn9)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn10)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41820_e55794) as f64).is_finite() && ((assign41820_e55794) as f64).fract() == 0.0 { if assign41820_e55794 == 0.0 { 0.0 } else { (assign41820_e55794 * ((locals.var_dnm).powf(assign41820_e55794 - 1.0) * locals.var_dnm_dn13)) } } else { (assign41820_e55795 * (assign41820_e55794 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign41820_e55796, assign41820_e55796_d_n0, assign41820_e55796_d_n2, assign41820_e55796_d_n4, assign41820_e55796_d_n5, assign41820_e55796_d_n6, assign41820_e55796_d_n7, assign41820_e55796_d_n8, assign41820_e55796_d_n9, assign41820_e55796_d_n10, assign41820_e55796_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign41820_e55798;
        locals.var_dnm_dn0 = assign41820_e55798_d_n0;
        locals.var_dnm_dn2 = assign41820_e55798_d_n2;
        locals.var_dnm_dn4 = assign41820_e55798_d_n4;
        locals.var_dnm_dn5 = assign41820_e55798_d_n5;
        locals.var_dnm_dn6 = assign41820_e55798_d_n6;
        locals.var_dnm_dn7 = assign41820_e55798_d_n7;
        locals.var_dnm_dn8 = assign41820_e55798_d_n8;
        locals.var_dnm_dn9 = assign41820_e55798_d_n9;
        locals.var_dnm_dn10 = assign41820_e55798_d_n10;
        locals.var_dnm_dn13 = assign41820_e55798_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign41830_e55814, assign41830_e55814_d_n0, assign41830_e55814_d_n2, assign41830_e55814_d_n4, assign41830_e55814_d_n5, assign41830_e55814_d_n6, assign41830_e55814_d_n7, assign41830_e55814_d_n8, assign41830_e55814_d_n9, assign41830_e55814_d_n10, assign41830_e55814_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41830_e55812: f64 = (1.0 / locals.var_dnm);
        (assign41830_e55812, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign41830_e55814;
        locals.var_dnm_dn0 = assign41830_e55814_d_n0;
        locals.var_dnm_dn2 = assign41830_e55814_d_n2;
        locals.var_dnm_dn4 = assign41830_e55814_d_n4;
        locals.var_dnm_dn5 = assign41830_e55814_d_n5;
        locals.var_dnm_dn6 = assign41830_e55814_d_n6;
        locals.var_dnm_dn7 = assign41830_e55814_d_n7;
        locals.var_dnm_dn8 = assign41830_e55814_d_n8;
        locals.var_dnm_dn9 = assign41830_e55814_d_n9;
        locals.var_dnm_dn10 = assign41830_e55814_d_n10;
        locals.var_dnm_dn13 = assign41830_e55814_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign41840_e55832, assign41840_e55832_d_n0, assign41840_e55832_d_n2, assign41840_e55832_d_n4, assign41840_e55832_d_n5, assign41840_e55832_d_n6, assign41840_e55832_d_n7, assign41840_e55832_d_n8, assign41840_e55832_d_n9, assign41840_e55832_d_n10, assign41840_e55832_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41840_e55828: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt);
        let assign41840_e55830: f64 = (assign41840_e55828 * locals.var_dnm);
        (assign41840_e55830, ((((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn0)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn2)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn4)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn5)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn6)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn7)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn8)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn9)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn10)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn13)) * locals.var_dnm) + (assign41840_e55828 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign41840_e55832;
        locals.var_tmf0_dn0 = assign41840_e55832_d_n0;
        locals.var_tmf0_dn2 = assign41840_e55832_d_n2;
        locals.var_tmf0_dn4 = assign41840_e55832_d_n4;
        locals.var_tmf0_dn5 = assign41840_e55832_d_n5;
        locals.var_tmf0_dn6 = assign41840_e55832_d_n6;
        locals.var_tmf0_dn7 = assign41840_e55832_d_n7;
        locals.var_tmf0_dn8 = assign41840_e55832_d_n8;
        locals.var_tmf0_dn9 = assign41840_e55832_d_n9;
        locals.var_tmf0_dn10 = assign41840_e55832_d_n10;
        locals.var_tmf0_dn13 = assign41840_e55832_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign41850_e55852, assign41850_e55852_d_n0, assign41850_e55852_d_n2, assign41850_e55852_d_n4, assign41850_e55852_d_n5, assign41850_e55852_d_n6, assign41850_e55852_d_n7, assign41850_e55852_d_n8, assign41850_e55852_d_n9, assign41850_e55852_d_n10, assign41850_e55852_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41850_e55846: f64 = (locals.var_vgpdep_dlt * locals.var_xmp);
        let assign41850_e55848: f64 = (assign41850_e55846 * locals.var_dnm);
        let assign41850_e55850: f64 = (assign41850_e55848 / locals.var_arg);
        (assign41850_e55850, (((((((locals.var_vgpdep_dlt_dn0 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn0)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn0)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn2 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn2)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn2)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn4 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn4)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn4)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn5 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn5)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn5)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn6 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn6)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn6)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn7 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn7)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn7)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn8 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn8)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn8)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn9 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn9)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn9)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn10 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn10)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn10)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn13 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn13)) * locals.var_dnm) + (assign41850_e55846 * locals.var_dnm_dn13)) * locals.var_arg) - (assign41850_e55848 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign41850_e55852;
        locals.var_t0_dn0 = assign41850_e55852_d_n0;
        locals.var_t0_dn2 = assign41850_e55852_d_n2;
        locals.var_t0_dn4 = assign41850_e55852_d_n4;
        locals.var_t0_dn5 = assign41850_e55852_d_n5;
        locals.var_t0_dn6 = assign41850_e55852_d_n6;
        locals.var_t0_dn7 = assign41850_e55852_d_n7;
        locals.var_t0_dn8 = assign41850_e55852_d_n8;
        locals.var_t0_dn9 = assign41850_e55852_d_n9;
        locals.var_t0_dn10 = assign41850_e55852_d_n10;
        locals.var_t0_dn13 = assign41850_e55852_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign41860_e55870, assign41860_e55870_d_n0, assign41860_e55870_d_n2, assign41860_e55870_d_n4, assign41860_e55870_d_n5, assign41860_e55870_d_n6, assign41860_e55870_d_n7, assign41860_e55870_d_n8, assign41860_e55870_d_n9, assign41860_e55870_d_n10, assign41860_e55870_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        let assign41860_e55866: f64 = (-locals.var_vgpdep_dlt);
        let assign41860_e55868: f64 = (assign41860_e55866 + locals.var_tmf0);
        (assign41860_e55868, ((-locals.var_vgpdep_dlt_dn0) + locals.var_tmf0_dn0), ((-locals.var_vgpdep_dlt_dn2) + locals.var_tmf0_dn2), ((-locals.var_vgpdep_dlt_dn4) + locals.var_tmf0_dn4), ((-locals.var_vgpdep_dlt_dn5) + locals.var_tmf0_dn5), ((-locals.var_vgpdep_dlt_dn6) + locals.var_tmf0_dn6), ((-locals.var_vgpdep_dlt_dn7) + locals.var_tmf0_dn7), ((-locals.var_vgpdep_dlt_dn8) + locals.var_tmf0_dn8), ((-locals.var_vgpdep_dlt_dn9) + locals.var_tmf0_dn9), ((-locals.var_vgpdep_dlt_dn10) + locals.var_tmf0_dn10), ((-locals.var_vgpdep_dlt_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn13,)
    }
};
        locals.var_vgp_res = assign41860_e55870;
        locals.var_vgp_res_dn0 = assign41860_e55870_d_n0;
        locals.var_vgp_res_dn2 = assign41860_e55870_d_n2;
        locals.var_vgp_res_dn4 = assign41860_e55870_d_n4;
        locals.var_vgp_res_dn5 = assign41860_e55870_d_n5;
        locals.var_vgp_res_dn6 = assign41860_e55870_d_n6;
        locals.var_vgp_res_dn7 = assign41860_e55870_d_n7;
        locals.var_vgp_res_dn8 = assign41860_e55870_d_n8;
        locals.var_vgp_res_dn9 = assign41860_e55870_d_n9;
        locals.var_vgp_res_dn10 = assign41860_e55870_d_n10;
        locals.var_vgp_res_dn13 = assign41860_e55870_d_n13;
        locals.var_vgp_res_rv = 0.0;

        let (assign41870_e55884, assign41870_e55884_d_n0, assign41870_e55884_d_n2, assign41870_e55884_d_n4, assign41870_e55884_d_n5, assign41870_e55884_d_n6, assign41870_e55884_d_n7, assign41870_e55884_d_n8, assign41870_e55884_d_n9, assign41870_e55884_d_n10, assign41870_e55884_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign41870_e55884;
        locals.var_t0_dn0 = assign41870_e55884_d_n0;
        locals.var_t0_dn2 = assign41870_e55884_d_n2;
        locals.var_t0_dn4 = assign41870_e55884_d_n4;
        locals.var_t0_dn5 = assign41870_e55884_d_n5;
        locals.var_t0_dn6 = assign41870_e55884_d_n6;
        locals.var_t0_dn7 = assign41870_e55884_d_n7;
        locals.var_t0_dn8 = assign41870_e55884_d_n8;
        locals.var_t0_dn9 = assign41870_e55884_d_n9;
        locals.var_t0_dn10 = assign41870_e55884_d_n10;
        locals.var_t0_dn13 = assign41870_e55884_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign41880_e55899, assign41880_e55899_d_n0, assign41880_e55899_d_n2, assign41880_e55899_d_n4, assign41880_e55899_d_n5, assign41880_e55899_d_n6, assign41880_e55899_d_n7, assign41880_e55899_d_n8, assign41880_e55899_d_n9, assign41880_e55899_d_n10, assign41880_e55899_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 == 0.0)) {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn13,)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn13,)
    }
};
        locals.var_vgp_res = assign41880_e55899;
        locals.var_vgp_res_dn0 = assign41880_e55899_d_n0;
        locals.var_vgp_res_dn2 = assign41880_e55899_d_n2;
        locals.var_vgp_res_dn4 = assign41880_e55899_d_n4;
        locals.var_vgp_res_dn5 = assign41880_e55899_d_n5;
        locals.var_vgp_res_dn6 = assign41880_e55899_d_n6;
        locals.var_vgp_res_dn7 = assign41880_e55899_d_n7;
        locals.var_vgp_res_dn8 = assign41880_e55899_d_n8;
        locals.var_vgp_res_dn9 = assign41880_e55899_d_n9;
        locals.var_vgp_res_dn10 = assign41880_e55899_d_n10;
        locals.var_vgp_res_dn13 = assign41880_e55899_d_n13;
        locals.var_vgp_res_rv = 0.0;

        let (assign41890_e55914, assign41890_e55914_d_n0, assign41890_e55914_d_n2, assign41890_e55914_d_n4, assign41890_e55914_d_n5, assign41890_e55914_d_n6, assign41890_e55914_d_n7, assign41890_e55914_d_n8, assign41890_e55914_d_n9, assign41890_e55914_d_n10, assign41890_e55914_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1037 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign41890_e55914;
        locals.var_t0_dn0 = assign41890_e55914_d_n0;
        locals.var_t0_dn2 = assign41890_e55914_d_n2;
        locals.var_t0_dn4 = assign41890_e55914_d_n4;
        locals.var_t0_dn5 = assign41890_e55914_d_n5;
        locals.var_t0_dn6 = assign41890_e55914_d_n6;
        locals.var_t0_dn7 = assign41890_e55914_d_n7;
        locals.var_t0_dn8 = assign41890_e55914_d_n8;
        locals.var_t0_dn9 = assign41890_e55914_d_n9;
        locals.var_t0_dn10 = assign41890_e55914_d_n10;
        locals.var_t0_dn13 = assign41890_e55914_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign41900_e55926,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign41900_e55926;
        locals.var_flg_conv_rv = 0.0;

        let (assign41910_e55938,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign41910_e55938;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_139(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign41920_loop_guard: usize = 0;
        while {
            let assign41920_cond_e55951: f64 = if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign41920_cond_e55951 != 0.0
        } {
            assign41920_loop_guard += 1;
            assert!(assign41920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41920_body0_e55965, assign41920_body0_e55965_d_n0, assign41920_body0_e55965_d_n2, assign41920_body0_e55965_d_n4, assign41920_body0_e55965_d_n5, assign41920_body0_e55965_d_n6, assign41920_body0_e55965_d_n7, assign41920_body0_e55965_d_n8, assign41920_body0_e55965_d_n9, assign41920_body0_e55965_d_n10, assign41920_body0_e55965_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41920_body0_e55963: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign41920_body0_e55963, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn13 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign41920_body0_e55965;
            locals.var_t1_dn0 = assign41920_body0_e55965_d_n0;
            locals.var_t1_dn2 = assign41920_body0_e55965_d_n2;
            locals.var_t1_dn4 = assign41920_body0_e55965_d_n4;
            locals.var_t1_dn5 = assign41920_body0_e55965_d_n5;
            locals.var_t1_dn6 = assign41920_body0_e55965_d_n6;
            locals.var_t1_dn7 = assign41920_body0_e55965_d_n7;
            locals.var_t1_dn8 = assign41920_body0_e55965_d_n8;
            locals.var_t1_dn9 = assign41920_body0_e55965_d_n9;
            locals.var_t1_dn10 = assign41920_body0_e55965_d_n10;
            locals.var_t1_dn13 = assign41920_body0_e55965_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign41920_body1_e55978, assign41920_body1_e55978_d_n0, assign41920_body1_e55978_d_n2, assign41920_body1_e55978_d_n4, assign41920_body1_e55978_d_n5, assign41920_body1_e55978_d_n6, assign41920_body1_e55978_d_n7, assign41920_body1_e55978_d_n8, assign41920_body1_e55978_d_n9, assign41920_body1_e55978_d_n10, assign41920_body1_e55978_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41920_body1_e55976: f64 = (locals.var_t1).exp();
        (assign41920_body1_e55976, (assign41920_body1_e55976 * locals.var_t1_dn0), (assign41920_body1_e55976 * locals.var_t1_dn2), (assign41920_body1_e55976 * locals.var_t1_dn4), (assign41920_body1_e55976 * locals.var_t1_dn5), (assign41920_body1_e55976 * locals.var_t1_dn6), (assign41920_body1_e55976 * locals.var_t1_dn7), (assign41920_body1_e55976 * locals.var_t1_dn8), (assign41920_body1_e55976 * locals.var_t1_dn9), (assign41920_body1_e55976 * locals.var_t1_dn10), (assign41920_body1_e55976 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign41920_body1_e55978;
            locals.var_t2_dn0 = assign41920_body1_e55978_d_n0;
            locals.var_t2_dn2 = assign41920_body1_e55978_d_n2;
            locals.var_t2_dn4 = assign41920_body1_e55978_d_n4;
            locals.var_t2_dn5 = assign41920_body1_e55978_d_n5;
            locals.var_t2_dn6 = assign41920_body1_e55978_d_n6;
            locals.var_t2_dn7 = assign41920_body1_e55978_d_n7;
            locals.var_t2_dn8 = assign41920_body1_e55978_d_n8;
            locals.var_t2_dn9 = assign41920_body1_e55978_d_n9;
            locals.var_t2_dn10 = assign41920_body1_e55978_d_n10;
            locals.var_t2_dn13 = assign41920_body1_e55978_d_n13;
            locals.var_t2_rv = 0.0;
            let assign41920_body2_e55981: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1043 = assign41920_body2_e55981;
            locals.var_guard1043_rv = 0.0;
            let (assign41920_body3_e56005, assign41920_body3_e56005_d_n0, assign41920_body3_e56005_d_n2, assign41920_body3_e56005_d_n4, assign41920_body3_e56005_d_n5, assign41920_body3_e56005_d_n6, assign41920_body3_e56005_d_n7, assign41920_body3_e56005_d_n8, assign41920_body3_e56005_d_n9, assign41920_body3_e56005_d_n10, assign41920_body3_e56005_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1043 != 0.0)) {
        let assign41920_body3_e55994: f64 = (-locals.var_cnst0);
        let assign41920_body3_e55997: f64 = (locals.var_t2 - 1.0);
        let assign41920_body3_e55999: f64 = (assign41920_body3_e55997 - locals.var_t1);
        let assign41920_body3_e56001: f64 = (assign41920_body3_e55999 + 1e-15);
        let assign41920_body3_e56002: f64 = (assign41920_body3_e56001).sqrt();
        let assign41920_body3_e56003: f64 = (assign41920_body3_e55994 * assign41920_body3_e56002);
        (assign41920_body3_e56003, (((-locals.var_cnst0_dn0) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn2) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn4) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn5) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn6) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn7) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn8) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn9) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn10) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign41920_body3_e56002)))), (((-locals.var_cnst0_dn13) * assign41920_body3_e56002) + (assign41920_body3_e55994 * ((locals.var_t2_dn13 - locals.var_t1_dn13) / (2.0 * assign41920_body3_e56002)))),)
    } else {
        (locals.var_q_s0__blk1028, locals.var_q_s0__blk1028_dn0, locals.var_q_s0__blk1028_dn2, locals.var_q_s0__blk1028_dn4, locals.var_q_s0__blk1028_dn5, locals.var_q_s0__blk1028_dn6, locals.var_q_s0__blk1028_dn7, locals.var_q_s0__blk1028_dn8, locals.var_q_s0__blk1028_dn9, locals.var_q_s0__blk1028_dn10, locals.var_q_s0__blk1028_dn13,)
    }
};
            locals.var_q_s0__blk1028 = assign41920_body3_e56005;
            locals.var_q_s0__blk1028_dn0 = assign41920_body3_e56005_d_n0;
            locals.var_q_s0__blk1028_dn2 = assign41920_body3_e56005_d_n2;
            locals.var_q_s0__blk1028_dn4 = assign41920_body3_e56005_d_n4;
            locals.var_q_s0__blk1028_dn5 = assign41920_body3_e56005_d_n5;
            locals.var_q_s0__blk1028_dn6 = assign41920_body3_e56005_d_n6;
            locals.var_q_s0__blk1028_dn7 = assign41920_body3_e56005_d_n7;
            locals.var_q_s0__blk1028_dn8 = assign41920_body3_e56005_d_n8;
            locals.var_q_s0__blk1028_dn9 = assign41920_body3_e56005_d_n9;
            locals.var_q_s0__blk1028_dn10 = assign41920_body3_e56005_d_n10;
            locals.var_q_s0__blk1028_dn13 = assign41920_body3_e56005_d_n13;
            locals.var_q_s0__blk1028_rv = 0.0;
            let (assign41920_body4_e56031, assign41920_body4_e56031_d_n0, assign41920_body4_e56031_d_n2, assign41920_body4_e56031_d_n4, assign41920_body4_e56031_d_n5, assign41920_body4_e56031_d_n6, assign41920_body4_e56031_d_n7, assign41920_body4_e56031_d_n8, assign41920_body4_e56031_d_n9, assign41920_body4_e56031_d_n10, assign41920_body4_e56031_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1043 != 0.0)) {
        let assign41920_body4_e56019: f64 = (0.5 * locals.var_cnst0);
        let assign41920_body4_e56021: f64 = (assign41920_body4_e56019 * locals.var_cnst0);
        let assign41920_body4_e56023: f64 = (assign41920_body4_e56021 / locals.var_q_s0__blk1028);
        let assign41920_body4_e56026: f64 = (locals.var_beta * locals.var_t2);
        let assign41920_body4_e56028: f64 = (assign41920_body4_e56026 - locals.var_beta);
        let assign41920_body4_e56029: f64 = (assign41920_body4_e56023 * assign41920_body4_e56028);
        (assign41920_body4_e56029, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn0)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn2)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn4)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn5)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn6)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn7)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn8)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn9)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn10)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn13) * locals.var_cnst0) + (assign41920_body4_e56019 * locals.var_cnst0_dn13)) * locals.var_q_s0__blk1028) - (assign41920_body4_e56021 * locals.var_q_s0__blk1028_dn13)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)) * assign41920_body4_e56028) + (assign41920_body4_e56023 * (((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)) - locals.var_beta_dn13))),)
    } else {
        (locals.var_q_s0_dps__blk1029, locals.var_q_s0_dps__blk1029_dn0, locals.var_q_s0_dps__blk1029_dn2, locals.var_q_s0_dps__blk1029_dn4, locals.var_q_s0_dps__blk1029_dn5, locals.var_q_s0_dps__blk1029_dn6, locals.var_q_s0_dps__blk1029_dn7, locals.var_q_s0_dps__blk1029_dn8, locals.var_q_s0_dps__blk1029_dn9, locals.var_q_s0_dps__blk1029_dn10, locals.var_q_s0_dps__blk1029_dn13,)
    }
};
            locals.var_q_s0_dps__blk1029 = assign41920_body4_e56031;
            locals.var_q_s0_dps__blk1029_dn0 = assign41920_body4_e56031_d_n0;
            locals.var_q_s0_dps__blk1029_dn2 = assign41920_body4_e56031_d_n2;
            locals.var_q_s0_dps__blk1029_dn4 = assign41920_body4_e56031_d_n4;
            locals.var_q_s0_dps__blk1029_dn5 = assign41920_body4_e56031_d_n5;
            locals.var_q_s0_dps__blk1029_dn6 = assign41920_body4_e56031_d_n6;
            locals.var_q_s0_dps__blk1029_dn7 = assign41920_body4_e56031_d_n7;
            locals.var_q_s0_dps__blk1029_dn8 = assign41920_body4_e56031_d_n8;
            locals.var_q_s0_dps__blk1029_dn9 = assign41920_body4_e56031_d_n9;
            locals.var_q_s0_dps__blk1029_dn10 = assign41920_body4_e56031_d_n10;
            locals.var_q_s0_dps__blk1029_dn13 = assign41920_body4_e56031_d_n13;
            locals.var_q_s0_dps__blk1029_rv = 0.0;
            let (assign41920_body5_e56052, assign41920_body5_e56052_d_n0, assign41920_body5_e56052_d_n2, assign41920_body5_e56052_d_n4, assign41920_body5_e56052_d_n5, assign41920_body5_e56052_d_n6, assign41920_body5_e56052_d_n7, assign41920_body5_e56052_d_n8, assign41920_body5_e56052_d_n9, assign41920_body5_e56052_d_n10, assign41920_body5_e56052_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1043 == 0.0)) {
        let assign41920_body5_e56045: f64 = (-locals.var_beta);
        let assign41920_body5_e56048: f64 = (locals.var_ps0dep - locals.var_vbsc);
        let assign41920_body5_e56049: f64 = (assign41920_body5_e56045 * assign41920_body5_e56048);
        let assign41920_body5_e56050: f64 = (assign41920_body5_e56049).exp();
        (assign41920_body5_e56050, (assign41920_body5_e56050 * (((-locals.var_beta_dn0) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn0 - locals.var_vbsc_dn0)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn2) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn2 - locals.var_vbsc_dn2)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn4) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn4 - locals.var_vbsc_dn4)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn5) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn5 - locals.var_vbsc_dn5)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn6) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn6 - locals.var_vbsc_dn6)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn7) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn7 - locals.var_vbsc_dn7)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn8) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn8 - locals.var_vbsc_dn8)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn9) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn9 - locals.var_vbsc_dn9)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn10) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn10 - locals.var_vbsc_dn10)))), (assign41920_body5_e56050 * (((-locals.var_beta_dn13) * assign41920_body5_e56048) + (assign41920_body5_e56045 * (locals.var_ps0dep_dn13 - locals.var_vbsc_dn13)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign41920_body5_e56052;
            locals.var_t3_dn0 = assign41920_body5_e56052_d_n0;
            locals.var_t3_dn2 = assign41920_body5_e56052_d_n2;
            locals.var_t3_dn4 = assign41920_body5_e56052_d_n4;
            locals.var_t3_dn5 = assign41920_body5_e56052_d_n5;
            locals.var_t3_dn6 = assign41920_body5_e56052_d_n6;
            locals.var_t3_dn7 = assign41920_body5_e56052_d_n7;
            locals.var_t3_dn8 = assign41920_body5_e56052_d_n8;
            locals.var_t3_dn9 = assign41920_body5_e56052_d_n9;
            locals.var_t3_dn10 = assign41920_body5_e56052_d_n10;
            locals.var_t3_dn13 = assign41920_body5_e56052_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign41920_body6_e56072, assign41920_body6_e56072_d_n0, assign41920_body6_e56072_d_n2, assign41920_body6_e56072_d_n4, assign41920_body6_e56072_d_n5, assign41920_body6_e56072_d_n6, assign41920_body6_e56072_d_n7, assign41920_body6_e56072_d_n8, assign41920_body6_e56072_d_n9, assign41920_body6_e56072_d_n10, assign41920_body6_e56072_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1043 == 0.0)) {
        let assign41920_body6_e56066: f64 = (-locals.var_beta);
        let assign41920_body6_e56068: f64 = (-locals.var_vbsc);
        let assign41920_body6_e56069: f64 = (assign41920_body6_e56066 * assign41920_body6_e56068);
        let assign41920_body6_e56070: f64 = (assign41920_body6_e56069).exp();
        (assign41920_body6_e56070, (assign41920_body6_e56070 * (((-locals.var_beta_dn0) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn0)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn2) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn2)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn4) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn4)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn5) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn5)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn6) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn6)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn7) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn7)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn8) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn8)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn9) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn9)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn10) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn10)))), (assign41920_body6_e56070 * (((-locals.var_beta_dn13) * assign41920_body6_e56068) + (assign41920_body6_e56066 * (-locals.var_vbsc_dn13)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign41920_body6_e56072;
            locals.var_t4_dn0 = assign41920_body6_e56072_d_n0;
            locals.var_t4_dn2 = assign41920_body6_e56072_d_n2;
            locals.var_t4_dn4 = assign41920_body6_e56072_d_n4;
            locals.var_t4_dn5 = assign41920_body6_e56072_d_n5;
            locals.var_t4_dn6 = assign41920_body6_e56072_d_n6;
            locals.var_t4_dn7 = assign41920_body6_e56072_d_n7;
            locals.var_t4_dn8 = assign41920_body6_e56072_d_n8;
            locals.var_t4_dn9 = assign41920_body6_e56072_d_n9;
            locals.var_t4_dn10 = assign41920_body6_e56072_d_n10;
            locals.var_t4_dn13 = assign41920_body6_e56072_d_n13;
            locals.var_t4_rv = 0.0;
            let (assign41920_body7_e56102, assign41920_body7_e56102_d_n0, assign41920_body7_e56102_d_n2, assign41920_body7_e56102_d_n4, assign41920_body7_e56102_d_n5, assign41920_body7_e56102_d_n6, assign41920_body7_e56102_d_n7, assign41920_body7_e56102_d_n8, assign41920_body7_e56102_d_n9, assign41920_body7_e56102_d_n10, assign41920_body7_e56102_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1043 == 0.0)) {
        let assign41920_body7_e56088: f64 = (locals.var_t2 - 1.0);
        let assign41920_body7_e56090: f64 = (assign41920_body7_e56088 - locals.var_t1);
        let assign41920_body7_e56094: f64 = (locals.var_t3 - locals.var_t4);
        let assign41920_body7_e56095: f64 = (locals.var_cnst1 * assign41920_body7_e56094);
        let assign41920_body7_e56096: f64 = (assign41920_body7_e56090 + assign41920_body7_e56095);
        let assign41920_body7_e56098: f64 = (assign41920_body7_e56096 + 1e-15);
        let assign41920_body7_e56099: f64 = (assign41920_body7_e56098).sqrt();
        let assign41920_body7_e56100: f64 = (locals.var_cnst0 * assign41920_body7_e56099);
        (assign41920_body7_e56100, ((locals.var_cnst0_dn0 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn2 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn4 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn5 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn6 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn7 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn8 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn9 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn10 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign41920_body7_e56099)))), ((locals.var_cnst0_dn13 * assign41920_body7_e56099) + (locals.var_cnst0 * (((locals.var_t2_dn13 - locals.var_t1_dn13) + ((locals.var_cnst1_dn13 * assign41920_body7_e56094) + (locals.var_cnst1 * (locals.var_t3_dn13 - locals.var_t4_dn13)))) / (2.0 * assign41920_body7_e56099)))),)
    } else {
        (locals.var_q_s0__blk1028, locals.var_q_s0__blk1028_dn0, locals.var_q_s0__blk1028_dn2, locals.var_q_s0__blk1028_dn4, locals.var_q_s0__blk1028_dn5, locals.var_q_s0__blk1028_dn6, locals.var_q_s0__blk1028_dn7, locals.var_q_s0__blk1028_dn8, locals.var_q_s0__blk1028_dn9, locals.var_q_s0__blk1028_dn10, locals.var_q_s0__blk1028_dn13,)
    }
};
            locals.var_q_s0__blk1028 = assign41920_body7_e56102;
            locals.var_q_s0__blk1028_dn0 = assign41920_body7_e56102_d_n0;
            locals.var_q_s0__blk1028_dn2 = assign41920_body7_e56102_d_n2;
            locals.var_q_s0__blk1028_dn4 = assign41920_body7_e56102_d_n4;
            locals.var_q_s0__blk1028_dn5 = assign41920_body7_e56102_d_n5;
            locals.var_q_s0__blk1028_dn6 = assign41920_body7_e56102_d_n6;
            locals.var_q_s0__blk1028_dn7 = assign41920_body7_e56102_d_n7;
            locals.var_q_s0__blk1028_dn8 = assign41920_body7_e56102_d_n8;
            locals.var_q_s0__blk1028_dn9 = assign41920_body7_e56102_d_n9;
            locals.var_q_s0__blk1028_dn10 = assign41920_body7_e56102_d_n10;
            locals.var_q_s0__blk1028_dn13 = assign41920_body7_e56102_d_n13;
            locals.var_q_s0__blk1028_rv = 0.0;
            let (assign41920_body8_e56123, assign41920_body8_e56123_d_n0, assign41920_body8_e56123_d_n2, assign41920_body8_e56123_d_n4, assign41920_body8_e56123_d_n5, assign41920_body8_e56123_d_n6, assign41920_body8_e56123_d_n7, assign41920_body8_e56123_d_n8, assign41920_body8_e56123_d_n9, assign41920_body8_e56123_d_n10, assign41920_body8_e56123_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1043 == 0.0)) {
        let assign41920_body8_e56117: f64 = (0.5 * locals.var_cnst0);
        let assign41920_body8_e56119: f64 = (assign41920_body8_e56117 * locals.var_cnst0);
        let assign41920_body8_e56121: f64 = (assign41920_body8_e56119 / locals.var_q_s0__blk1028);
        (assign41920_body8_e56121, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn0)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn2)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn4)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn5)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn6)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn7)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn8)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn9)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn10)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)), ((((((0.5 * locals.var_cnst0_dn13) * locals.var_cnst0) + (assign41920_body8_e56117 * locals.var_cnst0_dn13)) * locals.var_q_s0__blk1028) - (assign41920_body8_e56119 * locals.var_q_s0__blk1028_dn13)) / (locals.var_q_s0__blk1028 * locals.var_q_s0__blk1028)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
            locals.var_t5 = assign41920_body8_e56123;
            locals.var_t5_dn0 = assign41920_body8_e56123_d_n0;
            locals.var_t5_dn2 = assign41920_body8_e56123_d_n2;
            locals.var_t5_dn4 = assign41920_body8_e56123_d_n4;
            locals.var_t5_dn5 = assign41920_body8_e56123_d_n5;
            locals.var_t5_dn6 = assign41920_body8_e56123_d_n6;
            locals.var_t5_dn7 = assign41920_body8_e56123_d_n7;
            locals.var_t5_dn8 = assign41920_body8_e56123_d_n8;
            locals.var_t5_dn9 = assign41920_body8_e56123_d_n9;
            locals.var_t5_dn10 = assign41920_body8_e56123_d_n10;
            locals.var_t5_dn13 = assign41920_body8_e56123_d_n13;
            locals.var_t5_rv = 0.0;
            let (assign41920_body9_e56151, assign41920_body9_e56151_d_n0, assign41920_body9_e56151_d_n2, assign41920_body9_e56151_d_n4, assign41920_body9_e56151_d_n5, assign41920_body9_e56151_d_n6, assign41920_body9_e56151_d_n7, assign41920_body9_e56151_d_n8, assign41920_body9_e56151_d_n9, assign41920_body9_e56151_d_n10, assign41920_body9_e56151_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1043 == 0.0)) {
        let assign41920_body9_e56139: f64 = (locals.var_beta * locals.var_t2);
        let assign41920_body9_e56141: f64 = (assign41920_body9_e56139 - locals.var_beta);
        let assign41920_body9_e56144: f64 = (-locals.var_beta);
        let assign41920_body9_e56146: f64 = (assign41920_body9_e56144 * locals.var_t3);
        let assign41920_body9_e56147: f64 = (locals.var_cnst1 * assign41920_body9_e56146);
        let assign41920_body9_e56148: f64 = (assign41920_body9_e56141 + assign41920_body9_e56147);
        let assign41920_body9_e56149: f64 = (locals.var_t5 * assign41920_body9_e56148);
        (assign41920_body9_e56149, ((locals.var_t5_dn0 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn10))))))), ((locals.var_t5_dn13 * assign41920_body9_e56148) + (locals.var_t5 * ((((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)) - locals.var_beta_dn13) + ((locals.var_cnst1_dn13 * assign41920_body9_e56146) + (locals.var_cnst1 * (((-locals.var_beta_dn13) * locals.var_t3) + (assign41920_body9_e56144 * locals.var_t3_dn13))))))),)
    } else {
        (locals.var_q_s0_dps__blk1029, locals.var_q_s0_dps__blk1029_dn0, locals.var_q_s0_dps__blk1029_dn2, locals.var_q_s0_dps__blk1029_dn4, locals.var_q_s0_dps__blk1029_dn5, locals.var_q_s0_dps__blk1029_dn6, locals.var_q_s0_dps__blk1029_dn7, locals.var_q_s0_dps__blk1029_dn8, locals.var_q_s0_dps__blk1029_dn9, locals.var_q_s0_dps__blk1029_dn10, locals.var_q_s0_dps__blk1029_dn13,)
    }
};
            locals.var_q_s0_dps__blk1029 = assign41920_body9_e56151;
            locals.var_q_s0_dps__blk1029_dn0 = assign41920_body9_e56151_d_n0;
            locals.var_q_s0_dps__blk1029_dn2 = assign41920_body9_e56151_d_n2;
            locals.var_q_s0_dps__blk1029_dn4 = assign41920_body9_e56151_d_n4;
            locals.var_q_s0_dps__blk1029_dn5 = assign41920_body9_e56151_d_n5;
            locals.var_q_s0_dps__blk1029_dn6 = assign41920_body9_e56151_d_n6;
            locals.var_q_s0_dps__blk1029_dn7 = assign41920_body9_e56151_d_n7;
            locals.var_q_s0_dps__blk1029_dn8 = assign41920_body9_e56151_d_n8;
            locals.var_q_s0_dps__blk1029_dn9 = assign41920_body9_e56151_d_n9;
            locals.var_q_s0_dps__blk1029_dn10 = assign41920_body9_e56151_d_n10;
            locals.var_q_s0_dps__blk1029_dn13 = assign41920_body9_e56151_d_n13;
            locals.var_q_s0_dps__blk1029_rv = 0.0;
            let (assign41920_body10_e56167,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign41920_body10_e56165: f64 = (150.0 + 1.0);
        (assign41920_body10_e56165,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign41920_body10_e56167;
            locals.var_lp_s0_rv = 0.0;
            let (assign41920_body11_e56188, assign41920_body11_e56188_d_n0, assign41920_body11_e56188_d_n2, assign41920_body11_e56188_d_n4, assign41920_body11_e56188_d_n5, assign41920_body11_e56188_d_n6, assign41920_body11_e56188_d_n7, assign41920_body11_e56188_d_n8, assign41920_body11_e56188_d_n9, assign41920_body11_e56188_d_n10, assign41920_body11_e56188_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41920_body11_e56183: f64 = (locals.var_vgp_res - locals.var_ps0dep);
        let assign41920_body11_e56184: f64 = (locals.var_cox * assign41920_body11_e56183);
        let assign41920_body11_e56186: f64 = (assign41920_body11_e56184 + locals.var_q_s0__blk1028);
        (assign41920_body11_e56186, (((locals.var_cox_dn0 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1028_dn0), (((locals.var_cox_dn2 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1028_dn2), (((locals.var_cox_dn4 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1028_dn4), (((locals.var_cox_dn5 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1028_dn5), (((locals.var_cox_dn6 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1028_dn6), (((locals.var_cox_dn7 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1028_dn7), (((locals.var_cox_dn8 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1028_dn8), (((locals.var_cox_dn9 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1028_dn9), (((locals.var_cox_dn10 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1028_dn10), (((locals.var_cox_dn13 * assign41920_body11_e56183) + (locals.var_cox * (locals.var_vgp_res_dn13 - locals.var_ps0dep_dn13))) + locals.var_q_s0__blk1028_dn13),)
    } else {
        (locals.var_pf1, locals.var_pf1_dn0, locals.var_pf1_dn2, locals.var_pf1_dn4, locals.var_pf1_dn5, locals.var_pf1_dn6, locals.var_pf1_dn7, locals.var_pf1_dn8, locals.var_pf1_dn9, locals.var_pf1_dn10, locals.var_pf1_dn13,)
    }
};
            locals.var_pf1 = assign41920_body11_e56188;
            locals.var_pf1_dn0 = assign41920_body11_e56188_d_n0;
            locals.var_pf1_dn2 = assign41920_body11_e56188_d_n2;
            locals.var_pf1_dn4 = assign41920_body11_e56188_d_n4;
            locals.var_pf1_dn5 = assign41920_body11_e56188_d_n5;
            locals.var_pf1_dn6 = assign41920_body11_e56188_d_n6;
            locals.var_pf1_dn7 = assign41920_body11_e56188_d_n7;
            locals.var_pf1_dn8 = assign41920_body11_e56188_d_n8;
            locals.var_pf1_dn9 = assign41920_body11_e56188_d_n9;
            locals.var_pf1_dn10 = assign41920_body11_e56188_d_n10;
            locals.var_pf1_dn13 = assign41920_body11_e56188_d_n13;
            locals.var_pf1_rv = 0.0;
            let (assign41920_body12_e56206, assign41920_body12_e56206_d_n0, assign41920_body12_e56206_d_n2, assign41920_body12_e56206_d_n4, assign41920_body12_e56206_d_n5, assign41920_body12_e56206_d_n6, assign41920_body12_e56206_d_n7, assign41920_body12_e56206_d_n8, assign41920_body12_e56206_d_n9, assign41920_body12_e56206_d_n10, assign41920_body12_e56206_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41920_body12_e56202: f64 = (-locals.var_cox);
        let assign41920_body12_e56204: f64 = (assign41920_body12_e56202 + locals.var_q_s0_dps__blk1029);
        (assign41920_body12_e56204, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1029_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1029_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1029_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1029_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1029_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1029_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1029_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1029_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1029_dn10), ((-locals.var_cox_dn13) + locals.var_q_s0_dps__blk1029_dn13),)
    } else {
        (locals.var_pf11, locals.var_pf11_dn0, locals.var_pf11_dn2, locals.var_pf11_dn4, locals.var_pf11_dn5, locals.var_pf11_dn6, locals.var_pf11_dn7, locals.var_pf11_dn8, locals.var_pf11_dn9, locals.var_pf11_dn10, locals.var_pf11_dn13,)
    }
};
            locals.var_pf11 = assign41920_body12_e56206;
            locals.var_pf11_dn0 = assign41920_body12_e56206_d_n0;
            locals.var_pf11_dn2 = assign41920_body12_e56206_d_n2;
            locals.var_pf11_dn4 = assign41920_body12_e56206_d_n4;
            locals.var_pf11_dn5 = assign41920_body12_e56206_d_n5;
            locals.var_pf11_dn6 = assign41920_body12_e56206_d_n6;
            locals.var_pf11_dn7 = assign41920_body12_e56206_d_n7;
            locals.var_pf11_dn8 = assign41920_body12_e56206_d_n8;
            locals.var_pf11_dn9 = assign41920_body12_e56206_d_n9;
            locals.var_pf11_dn10 = assign41920_body12_e56206_d_n10;
            locals.var_pf11_dn13 = assign41920_body12_e56206_d_n13;
            locals.var_pf11_rv = 0.0;
            let (assign41920_body13_e56224, assign41920_body13_e56224_d_n0, assign41920_body13_e56224_d_n2, assign41920_body13_e56224_d_n4, assign41920_body13_e56224_d_n5, assign41920_body13_e56224_d_n6, assign41920_body13_e56224_d_n7, assign41920_body13_e56224_d_n8, assign41920_body13_e56224_d_n9, assign41920_body13_e56224_d_n10, assign41920_body13_e56224_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41920_body13_e56220: f64 = (-locals.var_pf1);
        let assign41920_body13_e56222: f64 = (assign41920_body13_e56220 / locals.var_pf11);
        (assign41920_body13_e56222, ((((-locals.var_pf1_dn0) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn0)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn2) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn2)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn4) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn4)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn5) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn5)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn6) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn6)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn7) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn7)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn8) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn8)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn9) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn9)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn10) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn10)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn13) * locals.var_pf11) - (assign41920_body13_e56220 * locals.var_pf11_dn13)) / (locals.var_pf11 * locals.var_pf11)),)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn13,)
    }
};
            locals.var_dps = assign41920_body13_e56224;
            locals.var_dps_dn0 = assign41920_body13_e56224_d_n0;
            locals.var_dps_dn2 = assign41920_body13_e56224_d_n2;
            locals.var_dps_dn4 = assign41920_body13_e56224_d_n4;
            locals.var_dps_dn5 = assign41920_body13_e56224_d_n5;
            locals.var_dps_dn6 = assign41920_body13_e56224_d_n6;
            locals.var_dps_dn7 = assign41920_body13_e56224_d_n7;
            locals.var_dps_dn8 = assign41920_body13_e56224_d_n8;
            locals.var_dps_dn9 = assign41920_body13_e56224_d_n9;
            locals.var_dps_dn10 = assign41920_body13_e56224_d_n10;
            locals.var_dps_dn13 = assign41920_body13_e56224_d_n13;
            locals.var_dps_rv = 0.0;
            let assign41920_body14_e56226: f64 = (locals.var_dps).abs();
            let assign41920_body14_e56229: f64 = (1e-10 * 100.0);
            let assign41920_body14_e56230: f64 = if assign41920_body14_e56226 < assign41920_body14_e56229 { 1.0 } else { 0.0 };
            locals.var_guard1044 = assign41920_body14_e56230;
            locals.var_guard1044_rv = 0.0;
            let (assign41920_body15_e56247,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1044 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign41920_body15_e56247;
            locals.var_flg_conv_rv = 0.0;
            let assign41920_body16_e56250: f64 = if locals.var_dps > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1045 = assign41920_body16_e56250;
            locals.var_guard1045_rv = 0.0;
            let (assign41920_body17_e56270, assign41920_body17_e56270_d_n0, assign41920_body17_e56270_d_n2, assign41920_body17_e56270_d_n4, assign41920_body17_e56270_d_n5, assign41920_body17_e56270_d_n6, assign41920_body17_e56270_d_n7, assign41920_body17_e56270_d_n8, assign41920_body17_e56270_d_n9, assign41920_body17_e56270_d_n10, assign41920_body17_e56270_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1044 == 0.0)) && (locals.var_guard1045 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn13,)
    }
};
            locals.var_dps = assign41920_body17_e56270;
            locals.var_dps_dn0 = assign41920_body17_e56270_d_n0;
            locals.var_dps_dn2 = assign41920_body17_e56270_d_n2;
            locals.var_dps_dn4 = assign41920_body17_e56270_d_n4;
            locals.var_dps_dn5 = assign41920_body17_e56270_d_n5;
            locals.var_dps_dn6 = assign41920_body17_e56270_d_n6;
            locals.var_dps_dn7 = assign41920_body17_e56270_d_n7;
            locals.var_dps_dn8 = assign41920_body17_e56270_d_n8;
            locals.var_dps_dn9 = assign41920_body17_e56270_d_n9;
            locals.var_dps_dn10 = assign41920_body17_e56270_d_n10;
            locals.var_dps_dn13 = assign41920_body17_e56270_d_n13;
            locals.var_dps_rv = 0.0;
            let assign41920_body18_e56273: f64 = (-0.1);
            let assign41920_body18_e56274: f64 = if locals.var_dps < assign41920_body18_e56273 { 1.0 } else { 0.0 };
            locals.var_guard1046 = assign41920_body18_e56274;
            locals.var_guard1046_rv = 0.0;
            let (assign41920_body19_e56298, assign41920_body19_e56298_d_n0, assign41920_body19_e56298_d_n2, assign41920_body19_e56298_d_n4, assign41920_body19_e56298_d_n5, assign41920_body19_e56298_d_n6, assign41920_body19_e56298_d_n7, assign41920_body19_e56298_d_n8, assign41920_body19_e56298_d_n9, assign41920_body19_e56298_d_n10, assign41920_body19_e56298_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1044 == 0.0)) && (locals.var_guard1045 == 0.0)) && (locals.var_guard1046 != 0.0)) {
        let assign41920_body19_e56296: f64 = (-0.1);
        (assign41920_body19_e56296, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn13,)
    }
};
            locals.var_dps = assign41920_body19_e56298;
            locals.var_dps_dn0 = assign41920_body19_e56298_d_n0;
            locals.var_dps_dn2 = assign41920_body19_e56298_d_n2;
            locals.var_dps_dn4 = assign41920_body19_e56298_d_n4;
            locals.var_dps_dn5 = assign41920_body19_e56298_d_n5;
            locals.var_dps_dn6 = assign41920_body19_e56298_d_n6;
            locals.var_dps_dn7 = assign41920_body19_e56298_d_n7;
            locals.var_dps_dn8 = assign41920_body19_e56298_d_n8;
            locals.var_dps_dn9 = assign41920_body19_e56298_d_n9;
            locals.var_dps_dn10 = assign41920_body19_e56298_d_n10;
            locals.var_dps_dn13 = assign41920_body19_e56298_d_n13;
            locals.var_dps_rv = 0.0;
            let (assign41920_body20_e56315, assign41920_body20_e56315_d_n0, assign41920_body20_e56315_d_n2, assign41920_body20_e56315_d_n4, assign41920_body20_e56315_d_n5, assign41920_body20_e56315_d_n6, assign41920_body20_e56315_d_n7, assign41920_body20_e56315_d_n8, assign41920_body20_e56315_d_n9, assign41920_body20_e56315_d_n10, assign41920_body20_e56315_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41920_body20_e56313: f64 = (locals.var_ps0dep + locals.var_dps);
        (assign41920_body20_e56313, (locals.var_ps0dep_dn0 + locals.var_dps_dn0), (locals.var_ps0dep_dn2 + locals.var_dps_dn2), (locals.var_ps0dep_dn4 + locals.var_dps_dn4), (locals.var_ps0dep_dn5 + locals.var_dps_dn5), (locals.var_ps0dep_dn6 + locals.var_dps_dn6), (locals.var_ps0dep_dn7 + locals.var_dps_dn7), (locals.var_ps0dep_dn8 + locals.var_dps_dn8), (locals.var_ps0dep_dn9 + locals.var_dps_dn9), (locals.var_ps0dep_dn10 + locals.var_dps_dn10), (locals.var_ps0dep_dn13 + locals.var_dps_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
            locals.var_ps0dep = assign41920_body20_e56315;
            locals.var_ps0dep_dn0 = assign41920_body20_e56315_d_n0;
            locals.var_ps0dep_dn2 = assign41920_body20_e56315_d_n2;
            locals.var_ps0dep_dn4 = assign41920_body20_e56315_d_n4;
            locals.var_ps0dep_dn5 = assign41920_body20_e56315_d_n5;
            locals.var_ps0dep_dn6 = assign41920_body20_e56315_d_n6;
            locals.var_ps0dep_dn7 = assign41920_body20_e56315_d_n7;
            locals.var_ps0dep_dn8 = assign41920_body20_e56315_d_n8;
            locals.var_ps0dep_dn9 = assign41920_body20_e56315_d_n9;
            locals.var_ps0dep_dn10 = assign41920_body20_e56315_d_n10;
            locals.var_ps0dep_dn13 = assign41920_body20_e56315_d_n13;
            locals.var_ps0dep_rv = 0.0;
            let (assign41920_body21_e56329,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41920_body21_e56327: f64 = (locals.var_lp_s0 + 1.0);
        (assign41920_body21_e56327,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign41920_body21_e56329;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign41940_e56345, assign41940_e56345_d_n0, assign41940_e56345_d_n2, assign41940_e56345_d_n4, assign41940_e56345_d_n5, assign41940_e56345_d_n6, assign41940_e56345_d_n7, assign41940_e56345_d_n8, assign41940_e56345_d_n9, assign41940_e56345_d_n10, assign41940_e56345_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41940_e56343: f64 = (-locals.var_ps0dep);
        (assign41940_e56343, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign41940_e56345;
        locals.var_ps0dep_dn0 = assign41940_e56345_d_n0;
        locals.var_ps0dep_dn2 = assign41940_e56345_d_n2;
        locals.var_ps0dep_dn4 = assign41940_e56345_d_n4;
        locals.var_ps0dep_dn5 = assign41940_e56345_d_n5;
        locals.var_ps0dep_dn6 = assign41940_e56345_d_n6;
        locals.var_ps0dep_dn7 = assign41940_e56345_d_n7;
        locals.var_ps0dep_dn8 = assign41940_e56345_d_n8;
        locals.var_ps0dep_dn9 = assign41940_e56345_d_n9;
        locals.var_ps0dep_dn10 = assign41940_e56345_d_n10;
        locals.var_ps0dep_dn13 = assign41940_e56345_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign41950_e56365, assign41950_e56365_d_n0, assign41950_e56365_d_n2, assign41950_e56365_d_n4, assign41950_e56365_d_n5, assign41950_e56365_d_n6, assign41950_e56365_d_n7, assign41950_e56365_d_n8, assign41950_e56365_d_n9, assign41950_e56365_d_n10, assign41950_e56365_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41950_e56357: f64 = (locals.var_q_ndepm__blk905 * locals.var_tnp);
        let assign41950_e56359: f64 = (assign41950_e56357 * locals.var_tnp);
        let assign41950_e56361: f64 = (assign41950_e56359 / 2.0);
        let assign41950_e56363: f64 = (assign41950_e56361 / 1.034943e-10);
        (assign41950_e56363, ((((((locals.var_q_ndepm__blk905_dn0 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn0)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn2 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn2)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn4 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn4)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn5 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn5)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn6 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn6)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn7 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn7)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn8 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn8)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn9 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn9)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn10 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn10)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk905_dn13 * locals.var_tnp) + (locals.var_q_ndepm__blk905 * locals.var_tnp_dn13)) * locals.var_tnp) + (assign41950_e56357 * locals.var_tnp_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1026, locals.var_dphi_sb__blk1026_dn0, locals.var_dphi_sb__blk1026_dn2, locals.var_dphi_sb__blk1026_dn4, locals.var_dphi_sb__blk1026_dn5, locals.var_dphi_sb__blk1026_dn6, locals.var_dphi_sb__blk1026_dn7, locals.var_dphi_sb__blk1026_dn8, locals.var_dphi_sb__blk1026_dn9, locals.var_dphi_sb__blk1026_dn10, locals.var_dphi_sb__blk1026_dn13,)
    }
};
        locals.var_dphi_sb__blk1026 = assign41950_e56365;
        locals.var_dphi_sb__blk1026_dn0 = assign41950_e56365_d_n0;
        locals.var_dphi_sb__blk1026_dn2 = assign41950_e56365_d_n2;
        locals.var_dphi_sb__blk1026_dn4 = assign41950_e56365_d_n4;
        locals.var_dphi_sb__blk1026_dn5 = assign41950_e56365_d_n5;
        locals.var_dphi_sb__blk1026_dn6 = assign41950_e56365_d_n6;
        locals.var_dphi_sb__blk1026_dn7 = assign41950_e56365_d_n7;
        locals.var_dphi_sb__blk1026_dn8 = assign41950_e56365_d_n8;
        locals.var_dphi_sb__blk1026_dn9 = assign41950_e56365_d_n9;
        locals.var_dphi_sb__blk1026_dn10 = assign41950_e56365_d_n10;
        locals.var_dphi_sb__blk1026_dn13 = assign41950_e56365_d_n13;
        locals.var_dphi_sb__blk1026_rv = 0.0;

        let (assign41960_e56384, assign41960_e56384_d_n0, assign41960_e56384_d_n2, assign41960_e56384_d_n4, assign41960_e56384_d_n5, assign41960_e56384_d_n6, assign41960_e56384_d_n7, assign41960_e56384_d_n8, assign41960_e56384_d_n9, assign41960_e56384_d_n10, assign41960_e56384_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41960_e56378: f64 = (2.0 * locals.var_beta);
        let assign41960_e56380: f64 = (assign41960_e56378 * locals.var_dphi_sb__blk1026);
        let assign41960_e56381: f64 = (assign41960_e56380).sqrt();
        let assign41960_e56382: f64 = (p.p394 * assign41960_e56381);
        (assign41960_e56382, (p.p394 * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn0)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn2)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn4)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn5)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn6)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn7)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn8)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn9)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn10)) / (2.0 * assign41960_e56381))), (p.p394 * ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb__blk1026) + (assign41960_e56378 * locals.var_dphi_sb__blk1026_dn13)) / (2.0 * assign41960_e56381))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign41960_e56384;
        locals.var_t0_dn0 = assign41960_e56384_d_n0;
        locals.var_t0_dn2 = assign41960_e56384_d_n2;
        locals.var_t0_dn4 = assign41960_e56384_d_n4;
        locals.var_t0_dn5 = assign41960_e56384_d_n5;
        locals.var_t0_dn6 = assign41960_e56384_d_n6;
        locals.var_t0_dn7 = assign41960_e56384_d_n7;
        locals.var_t0_dn8 = assign41960_e56384_d_n8;
        locals.var_t0_dn9 = assign41960_e56384_d_n9;
        locals.var_t0_dn10 = assign41960_e56384_d_n10;
        locals.var_t0_dn13 = assign41960_e56384_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign41970_e56403, assign41970_e56403_d_n0, assign41970_e56403_d_n2, assign41970_e56403_d_n4, assign41970_e56403_d_n5, assign41970_e56403_d_n6, assign41970_e56403_d_n7, assign41970_e56403_d_n8, assign41970_e56403_d_n9, assign41970_e56403_d_n10, assign41970_e56403_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41970_e56395: f64 = (locals.var_t0).exp();
        let assign41970_e56397: f64 = (-locals.var_t0);
        let assign41970_e56398: f64 = (assign41970_e56397).exp();
        let assign41970_e56399: f64 = (assign41970_e56395 + assign41970_e56398);
        let assign41970_e56401: f64 = (assign41970_e56399 / 2.0);
        (assign41970_e56401, (((assign41970_e56395 * locals.var_t0_dn0) + (assign41970_e56398 * (-locals.var_t0_dn0))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn2) + (assign41970_e56398 * (-locals.var_t0_dn2))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn4) + (assign41970_e56398 * (-locals.var_t0_dn4))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn5) + (assign41970_e56398 * (-locals.var_t0_dn5))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn6) + (assign41970_e56398 * (-locals.var_t0_dn6))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn7) + (assign41970_e56398 * (-locals.var_t0_dn7))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn8) + (assign41970_e56398 * (-locals.var_t0_dn8))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn9) + (assign41970_e56398 * (-locals.var_t0_dn9))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn10) + (assign41970_e56398 * (-locals.var_t0_dn10))) / 2.0), (((assign41970_e56395 * locals.var_t0_dn13) + (assign41970_e56398 * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign41970_e56403;
        locals.var_t1_dn0 = assign41970_e56403_d_n0;
        locals.var_t1_dn2 = assign41970_e56403_d_n2;
        locals.var_t1_dn4 = assign41970_e56403_d_n4;
        locals.var_t1_dn5 = assign41970_e56403_d_n5;
        locals.var_t1_dn6 = assign41970_e56403_d_n6;
        locals.var_t1_dn7 = assign41970_e56403_d_n7;
        locals.var_t1_dn8 = assign41970_e56403_d_n8;
        locals.var_t1_dn9 = assign41970_e56403_d_n9;
        locals.var_t1_dn10 = assign41970_e56403_d_n10;
        locals.var_t1_dn13 = assign41970_e56403_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign41980_e56418, assign41980_e56418_d_n0, assign41980_e56418_d_n2, assign41980_e56418_d_n4, assign41980_e56418_d_n5, assign41980_e56418_d_n6, assign41980_e56418_d_n7, assign41980_e56418_d_n8, assign41980_e56418_d_n9, assign41980_e56418_d_n10, assign41980_e56418_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41980_e56414: f64 = (locals.var_t1).ln();
        let assign41980_e56416: f64 = (assign41980_e56414 / locals.var_dphi_sb__blk1026);
        (assign41980_e56416, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn0)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn2)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn4)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn5)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn6)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn7)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn8)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn9)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn10)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb__blk1026) - (assign41980_e56414 * locals.var_dphi_sb__blk1026_dn13)) / (locals.var_dphi_sb__blk1026 * locals.var_dphi_sb__blk1026)),)
    } else {
        (locals.var_c_sb__blk1027, locals.var_c_sb__blk1027_dn0, locals.var_c_sb__blk1027_dn2, locals.var_c_sb__blk1027_dn4, locals.var_c_sb__blk1027_dn5, locals.var_c_sb__blk1027_dn6, locals.var_c_sb__blk1027_dn7, locals.var_c_sb__blk1027_dn8, locals.var_c_sb__blk1027_dn9, locals.var_c_sb__blk1027_dn10, locals.var_c_sb__blk1027_dn13,)
    }
};
        locals.var_c_sb__blk1027 = assign41980_e56418;
        locals.var_c_sb__blk1027_dn0 = assign41980_e56418_d_n0;
        locals.var_c_sb__blk1027_dn2 = assign41980_e56418_d_n2;
        locals.var_c_sb__blk1027_dn4 = assign41980_e56418_d_n4;
        locals.var_c_sb__blk1027_dn5 = assign41980_e56418_d_n5;
        locals.var_c_sb__blk1027_dn6 = assign41980_e56418_d_n6;
        locals.var_c_sb__blk1027_dn7 = assign41980_e56418_d_n7;
        locals.var_c_sb__blk1027_dn8 = assign41980_e56418_d_n8;
        locals.var_c_sb__blk1027_dn9 = assign41980_e56418_d_n9;
        locals.var_c_sb__blk1027_dn10 = assign41980_e56418_d_n10;
        locals.var_c_sb__blk1027_dn13 = assign41980_e56418_d_n13;
        locals.var_c_sb__blk1027_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_140(
        locals: &mut StampLocals,
    ) {
        let (assign41990_e56432, assign41990_e56432_d_n0, assign41990_e56432_d_n2, assign41990_e56432_d_n4, assign41990_e56432_d_n5, assign41990_e56432_d_n6, assign41990_e56432_d_n7, assign41990_e56432_d_n8, assign41990_e56432_d_n9, assign41990_e56432_d_n10, assign41990_e56432_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign41990_e56430: f64 = (locals.var_c_sb__blk1027 * locals.var_ps0dep);
        (assign41990_e56430, ((locals.var_c_sb__blk1027_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1027_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1027_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1027_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1027_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1027_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1027_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1027_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1027_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1027_dn13 * locals.var_ps0dep) + (locals.var_c_sb__blk1027 * locals.var_ps0dep_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign41990_e56432;
        locals.var_tx_dn0 = assign41990_e56432_d_n0;
        locals.var_tx_dn2 = assign41990_e56432_d_n2;
        locals.var_tx_dn4 = assign41990_e56432_d_n4;
        locals.var_tx_dn5 = assign41990_e56432_d_n5;
        locals.var_tx_dn6 = assign41990_e56432_d_n6;
        locals.var_tx_dn7 = assign41990_e56432_d_n7;
        locals.var_tx_dn8 = assign41990_e56432_d_n8;
        locals.var_tx_dn9 = assign41990_e56432_d_n9;
        locals.var_tx_dn10 = assign41990_e56432_d_n10;
        locals.var_tx_dn13 = assign41990_e56432_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign42000_e56448, assign42000_e56448_d_n0, assign42000_e56448_d_n2, assign42000_e56448_d_n4, assign42000_e56448_d_n5, assign42000_e56448_d_n6, assign42000_e56448_d_n7, assign42000_e56448_d_n8, assign42000_e56448_d_n9, assign42000_e56448_d_n10, assign42000_e56448_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let assign42000_e56443: f64 = (-locals.var_c_sb__blk1027);
        let assign42000_e56445: f64 = (assign42000_e56443 * locals.var_dphi_sb__blk1026);
        let assign42000_e56446: f64 = (assign42000_e56445).exp();
        (assign42000_e56446, (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn0) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn0))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn2) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn2))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn4) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn4))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn5) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn5))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn6) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn6))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn7) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn7))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn8) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn8))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn9) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn9))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn10) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn10))), (assign42000_e56446 * (((-locals.var_c_sb__blk1027_dn13) * locals.var_dphi_sb__blk1026) + (assign42000_e56443 * locals.var_dphi_sb__blk1026_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign42000_e56448;
        locals.var_t0_dn0 = assign42000_e56448_d_n0;
        locals.var_t0_dn2 = assign42000_e56448_d_n2;
        locals.var_t0_dn4 = assign42000_e56448_d_n4;
        locals.var_t0_dn5 = assign42000_e56448_d_n5;
        locals.var_t0_dn6 = assign42000_e56448_d_n6;
        locals.var_t0_dn7 = assign42000_e56448_d_n7;
        locals.var_t0_dn8 = assign42000_e56448_d_n8;
        locals.var_t0_dn9 = assign42000_e56448_d_n9;
        locals.var_t0_dn10 = assign42000_e56448_d_n10;
        locals.var_t0_dn13 = assign42000_e56448_d_n13;
        locals.var_t0_rv = 0.0;

        let assign42010_e56450: f64 = (locals.var_tx).abs();
        let assign42010_e56452: f64 = if assign42010_e56450 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1048 = assign42010_e56452;
        locals.var_guard1048_rv = 0.0;

        let (assign42020_e56469, assign42020_e56469_d_n0, assign42020_e56469_d_n2, assign42020_e56469_d_n4, assign42020_e56469_d_n5, assign42020_e56469_d_n6, assign42020_e56469_d_n7, assign42020_e56469_d_n8, assign42020_e56469_d_n9, assign42020_e56469_d_n10, assign42020_e56469_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1048 != 0.0)) {
        let assign42020_e56465: f64 = (locals.var_tx).exp();
        let assign42020_e56467: f64 = (assign42020_e56465 * locals.var_t0);
        (assign42020_e56467, (((assign42020_e56465 * locals.var_tx_dn0) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn0)), (((assign42020_e56465 * locals.var_tx_dn2) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn2)), (((assign42020_e56465 * locals.var_tx_dn4) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn4)), (((assign42020_e56465 * locals.var_tx_dn5) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn5)), (((assign42020_e56465 * locals.var_tx_dn6) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn6)), (((assign42020_e56465 * locals.var_tx_dn7) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn7)), (((assign42020_e56465 * locals.var_tx_dn8) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn8)), (((assign42020_e56465 * locals.var_tx_dn9) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn9)), (((assign42020_e56465 * locals.var_tx_dn10) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn10)), (((assign42020_e56465 * locals.var_tx_dn13) * locals.var_t0) + (assign42020_e56465 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign42020_e56469;
        locals.var_t1_dn0 = assign42020_e56469_d_n0;
        locals.var_t1_dn2 = assign42020_e56469_d_n2;
        locals.var_t1_dn4 = assign42020_e56469_d_n4;
        locals.var_t1_dn5 = assign42020_e56469_d_n5;
        locals.var_t1_dn6 = assign42020_e56469_d_n6;
        locals.var_t1_dn7 = assign42020_e56469_d_n7;
        locals.var_t1_dn8 = assign42020_e56469_d_n8;
        locals.var_t1_dn9 = assign42020_e56469_d_n9;
        locals.var_t1_dn10 = assign42020_e56469_d_n10;
        locals.var_t1_dn13 = assign42020_e56469_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign42030_e56485, assign42030_e56485_d_n0, assign42030_e56485_d_n2, assign42030_e56485_d_n4, assign42030_e56485_d_n5, assign42030_e56485_d_n6, assign42030_e56485_d_n7, assign42030_e56485_d_n8, assign42030_e56485_d_n9, assign42030_e56485_d_n10, assign42030_e56485_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1048 != 0.0)) {
        let assign42030_e56483: f64 = (locals.var_t1 - locals.var_t0);
        (assign42030_e56483, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign42030_e56485;
        locals.var_t2_dn0 = assign42030_e56485_d_n0;
        locals.var_t2_dn2 = assign42030_e56485_d_n2;
        locals.var_t2_dn4 = assign42030_e56485_d_n4;
        locals.var_t2_dn5 = assign42030_e56485_d_n5;
        locals.var_t2_dn6 = assign42030_e56485_d_n6;
        locals.var_t2_dn7 = assign42030_e56485_d_n7;
        locals.var_t2_dn8 = assign42030_e56485_d_n8;
        locals.var_t2_dn9 = assign42030_e56485_d_n9;
        locals.var_t2_dn10 = assign42030_e56485_d_n10;
        locals.var_t2_dn13 = assign42030_e56485_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign42040_e56504, assign42040_e56504_d_n0, assign42040_e56504_d_n2, assign42040_e56504_d_n4, assign42040_e56504_d_n5, assign42040_e56504_d_n6, assign42040_e56504_d_n7, assign42040_e56504_d_n8, assign42040_e56504_d_n9, assign42040_e56504_d_n10, assign42040_e56504_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1048 == 0.0)) {
        let assign42040_e56500: f64 = (1.0 + locals.var_tx);
        let assign42040_e56502: f64 = (assign42040_e56500 * locals.var_t0);
        (assign42040_e56502, ((locals.var_tx_dn0 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn10)), ((locals.var_tx_dn13 * locals.var_t0) + (assign42040_e56500 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign42040_e56504;
        locals.var_t1_dn0 = assign42040_e56504_d_n0;
        locals.var_t1_dn2 = assign42040_e56504_d_n2;
        locals.var_t1_dn4 = assign42040_e56504_d_n4;
        locals.var_t1_dn5 = assign42040_e56504_d_n5;
        locals.var_t1_dn6 = assign42040_e56504_d_n6;
        locals.var_t1_dn7 = assign42040_e56504_d_n7;
        locals.var_t1_dn8 = assign42040_e56504_d_n8;
        locals.var_t1_dn9 = assign42040_e56504_d_n9;
        locals.var_t1_dn10 = assign42040_e56504_d_n10;
        locals.var_t1_dn13 = assign42040_e56504_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign42050_e56527, assign42050_e56527_d_n0, assign42050_e56527_d_n2, assign42050_e56527_d_n4, assign42050_e56527_d_n5, assign42050_e56527_d_n6, assign42050_e56527_d_n7, assign42050_e56527_d_n8, assign42050_e56527_d_n9, assign42050_e56527_d_n10, assign42050_e56527_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1048 == 0.0)) {
        let assign42050_e56521: f64 = (locals.var_tx / 2.0);
        let assign42050_e56522: f64 = (1.0 + assign42050_e56521);
        let assign42050_e56523: f64 = (locals.var_tx * assign42050_e56522);
        let assign42050_e56525: f64 = (assign42050_e56523 * locals.var_t0);
        (assign42050_e56525, ((((locals.var_tx_dn0 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn10)), ((((locals.var_tx_dn13 * assign42050_e56522) + (locals.var_tx * (locals.var_tx_dn13 / 2.0))) * locals.var_t0) + (assign42050_e56523 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign42050_e56527;
        locals.var_t2_dn0 = assign42050_e56527_d_n0;
        locals.var_t2_dn2 = assign42050_e56527_d_n2;
        locals.var_t2_dn4 = assign42050_e56527_d_n4;
        locals.var_t2_dn5 = assign42050_e56527_d_n5;
        locals.var_t2_dn6 = assign42050_e56527_d_n6;
        locals.var_t2_dn7 = assign42050_e56527_d_n7;
        locals.var_t2_dn8 = assign42050_e56527_d_n8;
        locals.var_t2_dn9 = assign42050_e56527_d_n9;
        locals.var_t2_dn10 = assign42050_e56527_d_n10;
        locals.var_t2_dn13 = assign42050_e56527_d_n13;
        locals.var_t2_rv = 0.0;

        let assign42060_e56529: f64 = (locals.var_t2).abs();
        let assign42060_e56531: f64 = if assign42060_e56529 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1049 = assign42060_e56531;
        locals.var_guard1049_rv = 0.0;

        let (assign42070_e56550, assign42070_e56550_d_n0, assign42070_e56550_d_n2, assign42070_e56550_d_n4, assign42070_e56550_d_n5, assign42070_e56550_d_n6, assign42070_e56550_d_n7, assign42070_e56550_d_n8, assign42070_e56550_d_n9, assign42070_e56550_d_n10, assign42070_e56550_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1049 != 0.0)) {
        let assign42070_e56545: f64 = (1.0 + locals.var_t2);
        let assign42070_e56546: f64 = (assign42070_e56545).ln();
        let assign42070_e56548: f64 = (assign42070_e56546 / locals.var_c_sb__blk1027);
        (assign42070_e56548, ((((locals.var_t2_dn0 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn0)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn2 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn2)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn4 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn4)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn5 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn5)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn6 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn6)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn7 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn7)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn8 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn8)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn9 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn9)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn10 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn10)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), ((((locals.var_t2_dn13 / assign42070_e56545) * locals.var_c_sb__blk1027) - (assign42070_e56546 * locals.var_c_sb__blk1027_dn13)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)),)
    } else {
        (locals.var_pb0dep, locals.var_pb0dep_dn0, locals.var_pb0dep_dn2, locals.var_pb0dep_dn4, locals.var_pb0dep_dn5, locals.var_pb0dep_dn6, locals.var_pb0dep_dn7, locals.var_pb0dep_dn8, locals.var_pb0dep_dn9, locals.var_pb0dep_dn10, locals.var_pb0dep_dn13,)
    }
};
        locals.var_pb0dep = assign42070_e56550;
        locals.var_pb0dep_dn0 = assign42070_e56550_d_n0;
        locals.var_pb0dep_dn2 = assign42070_e56550_d_n2;
        locals.var_pb0dep_dn4 = assign42070_e56550_d_n4;
        locals.var_pb0dep_dn5 = assign42070_e56550_d_n5;
        locals.var_pb0dep_dn6 = assign42070_e56550_d_n6;
        locals.var_pb0dep_dn7 = assign42070_e56550_d_n7;
        locals.var_pb0dep_dn8 = assign42070_e56550_d_n8;
        locals.var_pb0dep_dn9 = assign42070_e56550_d_n9;
        locals.var_pb0dep_dn10 = assign42070_e56550_d_n10;
        locals.var_pb0dep_dn13 = assign42070_e56550_d_n13;
        locals.var_pb0dep_rv = 0.0;

        let (assign42080_e56567, assign42080_e56567_d_n0, assign42080_e56567_d_n2, assign42080_e56567_d_n4, assign42080_e56567_d_n5, assign42080_e56567_d_n6, assign42080_e56567_d_n7, assign42080_e56567_d_n8, assign42080_e56567_d_n9, assign42080_e56567_d_n10, assign42080_e56567_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1049 == 0.0)) {
        let assign42080_e56565: f64 = (locals.var_t2 / locals.var_c_sb__blk1027);
        (assign42080_e56565, (((locals.var_t2_dn0 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn0)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn2)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn4)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn5)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn6)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn7)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn8)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn9)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn10)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)), (((locals.var_t2_dn13 * locals.var_c_sb__blk1027) - (locals.var_t2 * locals.var_c_sb__blk1027_dn13)) / (locals.var_c_sb__blk1027 * locals.var_c_sb__blk1027)),)
    } else {
        (locals.var_pb0dep, locals.var_pb0dep_dn0, locals.var_pb0dep_dn2, locals.var_pb0dep_dn4, locals.var_pb0dep_dn5, locals.var_pb0dep_dn6, locals.var_pb0dep_dn7, locals.var_pb0dep_dn8, locals.var_pb0dep_dn9, locals.var_pb0dep_dn10, locals.var_pb0dep_dn13,)
    }
};
        locals.var_pb0dep = assign42080_e56567;
        locals.var_pb0dep_dn0 = assign42080_e56567_d_n0;
        locals.var_pb0dep_dn2 = assign42080_e56567_d_n2;
        locals.var_pb0dep_dn4 = assign42080_e56567_d_n4;
        locals.var_pb0dep_dn5 = assign42080_e56567_d_n5;
        locals.var_pb0dep_dn6 = assign42080_e56567_d_n6;
        locals.var_pb0dep_dn7 = assign42080_e56567_d_n7;
        locals.var_pb0dep_dn8 = assign42080_e56567_d_n8;
        locals.var_pb0dep_dn9 = assign42080_e56567_d_n9;
        locals.var_pb0dep_dn10 = assign42080_e56567_d_n10;
        locals.var_pb0dep_dn13 = assign42080_e56567_d_n13;
        locals.var_pb0dep_rv = 0.0;

        let assign42090_e56570: f64 = (2.0 * 1.034943e-10);
        let assign42090_e56573: f64 = (locals.var_ps0dep - locals.var_pb0dep);
        let assign42090_e56574: f64 = (assign42090_e56570 * assign42090_e56573);
        let assign42090_e56576: f64 = (assign42090_e56574 / locals.var_q_ndepm__blk905);
        let assign42090_e56578: f64 = if assign42090_e56576 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1050 = assign42090_e56578;
        locals.var_guard1050_rv = 0.0;

        let (assign42100_e56592, assign42100_e56592_d_n0, assign42100_e56592_d_n2, assign42100_e56592_d_n4, assign42100_e56592_d_n5, assign42100_e56592_d_n6, assign42100_e56592_d_n7, assign42100_e56592_d_n8, assign42100_e56592_d_n9, assign42100_e56592_d_n10, assign42100_e56592_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1050 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn13,)
    }
};
        locals.var_ws = assign42100_e56592;
        locals.var_ws_dn0 = assign42100_e56592_d_n0;
        locals.var_ws_dn2 = assign42100_e56592_d_n2;
        locals.var_ws_dn4 = assign42100_e56592_d_n4;
        locals.var_ws_dn5 = assign42100_e56592_d_n5;
        locals.var_ws_dn6 = assign42100_e56592_d_n6;
        locals.var_ws_dn7 = assign42100_e56592_d_n7;
        locals.var_ws_dn8 = assign42100_e56592_d_n8;
        locals.var_ws_dn9 = assign42100_e56592_d_n9;
        locals.var_ws_dn10 = assign42100_e56592_d_n10;
        locals.var_ws_dn13 = assign42100_e56592_d_n13;
        locals.var_ws_rv = 0.0;

        let (assign42110_e56616, assign42110_e56616_d_n0, assign42110_e56616_d_n2, assign42110_e56616_d_n4, assign42110_e56616_d_n5, assign42110_e56616_d_n6, assign42110_e56616_d_n7, assign42110_e56616_d_n8, assign42110_e56616_d_n9, assign42110_e56616_d_n10, assign42110_e56616_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) && (locals.var_guard1050 == 0.0)) {
        let assign42110_e56607: f64 = (2.0 * 1.034943e-10);
        let assign42110_e56610: f64 = (locals.var_ps0dep - locals.var_pb0dep);
        let assign42110_e56611: f64 = (assign42110_e56607 * assign42110_e56610);
        let assign42110_e56613: f64 = (assign42110_e56611 / locals.var_q_ndepm__blk905);
        let assign42110_e56614: f64 = (assign42110_e56613).sqrt();
        (assign42110_e56614, (((((assign42110_e56607 * (locals.var_ps0dep_dn0 - locals.var_pb0dep_dn0)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn0)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn2 - locals.var_pb0dep_dn2)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn2)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn4 - locals.var_pb0dep_dn4)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn4)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn5 - locals.var_pb0dep_dn5)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn5)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn6 - locals.var_pb0dep_dn6)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn6)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn7 - locals.var_pb0dep_dn7)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn7)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn8 - locals.var_pb0dep_dn8)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn8)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn9 - locals.var_pb0dep_dn9)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn9)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn10 - locals.var_pb0dep_dn10)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn10)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)), (((((assign42110_e56607 * (locals.var_ps0dep_dn13 - locals.var_pb0dep_dn13)) * locals.var_q_ndepm__blk905) - (assign42110_e56611 * locals.var_q_ndepm__blk905_dn13)) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905)) / (2.0 * assign42110_e56614)),)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn13,)
    }
};
        locals.var_ws = assign42110_e56616;
        locals.var_ws_dn0 = assign42110_e56616_d_n0;
        locals.var_ws_dn2 = assign42110_e56616_d_n2;
        locals.var_ws_dn4 = assign42110_e56616_d_n4;
        locals.var_ws_dn5 = assign42110_e56616_d_n5;
        locals.var_ws_dn6 = assign42110_e56616_d_n6;
        locals.var_ws_dn7 = assign42110_e56616_d_n7;
        locals.var_ws_dn8 = assign42110_e56616_d_n8;
        locals.var_ws_dn9 = assign42110_e56616_d_n9;
        locals.var_ws_dn10 = assign42110_e56616_d_n10;
        locals.var_ws_dn13 = assign42110_e56616_d_n13;
        locals.var_ws_rv = 0.0;

        let (assign42120_e56633, assign42120_e56633_d_n0, assign42120_e56633_d_n2, assign42120_e56633_d_n4, assign42120_e56633_d_n5, assign42120_e56633_d_n6, assign42120_e56633_d_n7, assign42120_e56633_d_n8, assign42120_e56633_d_n9, assign42120_e56633_d_n10, assign42120_e56633_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1030 == 0.0)) {
        let (assign42120_e56631, assign42120_e56631_d_n0, assign42120_e56631_d_n2, assign42120_e56631_d_n4, assign42120_e56631_d_n5, assign42120_e56631_d_n6, assign42120_e56631_d_n7, assign42120_e56631_d_n8, assign42120_e56631_d_n9, assign42120_e56631_d_n10, assign42120_e56631_d_n13,) = {
            if (locals.var_ws > locals.var_tnp) {
                (locals.var_tnp, locals.var_tnp_dn0, locals.var_tnp_dn2, locals.var_tnp_dn4, locals.var_tnp_dn5, locals.var_tnp_dn6, locals.var_tnp_dn7, locals.var_tnp_dn8, locals.var_tnp_dn9, locals.var_tnp_dn10, locals.var_tnp_dn13,)
            } else {
                (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn13,)
            }
        };
        (assign42120_e56631, assign42120_e56631_d_n0, assign42120_e56631_d_n2, assign42120_e56631_d_n4, assign42120_e56631_d_n5, assign42120_e56631_d_n6, assign42120_e56631_d_n7, assign42120_e56631_d_n8, assign42120_e56631_d_n9, assign42120_e56631_d_n10, assign42120_e56631_d_n13,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn13,)
    }
};
        locals.var_ws = assign42120_e56633;
        locals.var_ws_dn0 = assign42120_e56633_d_n0;
        locals.var_ws_dn2 = assign42120_e56633_d_n2;
        locals.var_ws_dn4 = assign42120_e56633_d_n4;
        locals.var_ws_dn5 = assign42120_e56633_d_n5;
        locals.var_ws_dn6 = assign42120_e56633_d_n6;
        locals.var_ws_dn7 = assign42120_e56633_d_n7;
        locals.var_ws_dn8 = assign42120_e56633_d_n8;
        locals.var_ws_dn9 = assign42120_e56633_d_n9;
        locals.var_ws_dn10 = assign42120_e56633_d_n10;
        locals.var_ws_dn13 = assign42120_e56633_d_n13;
        locals.var_ws_rv = 0.0;

        let assign42130_e56636: f64 = if locals.var_ws < locals.var_tnp { 1.0 } else { 0.0 };
        locals.var_guard1051 = assign42130_e56636;
        locals.var_guard1051_rv = 0.0;

        let (assign42140_e56649, assign42140_e56649_d_n0, assign42140_e56649_d_n2, assign42140_e56649_d_n4, assign42140_e56649_d_n5, assign42140_e56649_d_n6, assign42140_e56649_d_n7, assign42140_e56649_d_n8, assign42140_e56649_d_n9, assign42140_e56649_d_n10, assign42140_e56649_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1051 != 0.0)) {
        let assign42140_e56647: f64 = (locals.var_tnp - locals.var_ws);
        (assign42140_e56647, (locals.var_tnp_dn0 - locals.var_ws_dn0), (locals.var_tnp_dn2 - locals.var_ws_dn2), (locals.var_tnp_dn4 - locals.var_ws_dn4), (locals.var_tnp_dn5 - locals.var_ws_dn5), (locals.var_tnp_dn6 - locals.var_ws_dn6), (locals.var_tnp_dn7 - locals.var_ws_dn7), (locals.var_tnp_dn8 - locals.var_ws_dn8), (locals.var_tnp_dn9 - locals.var_ws_dn9), (locals.var_tnp_dn10 - locals.var_ws_dn10), (locals.var_tnp_dn13 - locals.var_ws_dn13),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign42140_e56649;
        locals.var_w_res_dn0 = assign42140_e56649_d_n0;
        locals.var_w_res_dn2 = assign42140_e56649_d_n2;
        locals.var_w_res_dn4 = assign42140_e56649_d_n4;
        locals.var_w_res_dn5 = assign42140_e56649_d_n5;
        locals.var_w_res_dn6 = assign42140_e56649_d_n6;
        locals.var_w_res_dn7 = assign42140_e56649_d_n7;
        locals.var_w_res_dn8 = assign42140_e56649_d_n8;
        locals.var_w_res_dn9 = assign42140_e56649_d_n9;
        locals.var_w_res_dn10 = assign42140_e56649_d_n10;
        locals.var_w_res_dn13 = assign42140_e56649_d_n13;
        locals.var_w_res_rv = 0.0;

        let (assign42150_e56661, assign42150_e56661_d_n0, assign42150_e56661_d_n2, assign42150_e56661_d_n4, assign42150_e56661_d_n5, assign42150_e56661_d_n6, assign42150_e56661_d_n7, assign42150_e56661_d_n8, assign42150_e56661_d_n9, assign42150_e56661_d_n10, assign42150_e56661_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1051 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign42150_e56661;
        locals.var_w_res_dn0 = assign42150_e56661_d_n0;
        locals.var_w_res_dn2 = assign42150_e56661_d_n2;
        locals.var_w_res_dn4 = assign42150_e56661_d_n4;
        locals.var_w_res_dn5 = assign42150_e56661_d_n5;
        locals.var_w_res_dn6 = assign42150_e56661_d_n6;
        locals.var_w_res_dn7 = assign42150_e56661_d_n7;
        locals.var_w_res_dn8 = assign42150_e56661_d_n8;
        locals.var_w_res_dn9 = assign42150_e56661_d_n9;
        locals.var_w_res_dn10 = assign42150_e56661_d_n10;
        locals.var_w_res_dn13 = assign42150_e56661_d_n13;
        locals.var_w_res_rv = 0.0;

        let (assign42160_e56673, assign42160_e56673_d_n0, assign42160_e56673_d_n2, assign42160_e56673_d_n4, assign42160_e56673_d_n5, assign42160_e56673_d_n6, assign42160_e56673_d_n7, assign42160_e56673_d_n8, assign42160_e56673_d_n9, assign42160_e56673_d_n10, assign42160_e56673_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42160_e56670: f64 = (locals.var_q_n0_cur__blk889 + locals.var_q_nl_cur__blk890);
        let assign42160_e56671: f64 = (-assign42160_e56670);
        (assign42160_e56671, (-(locals.var_q_n0_cur__blk889_dn0 + locals.var_q_nl_cur__blk890_dn0)), (-(locals.var_q_n0_cur__blk889_dn2 + locals.var_q_nl_cur__blk890_dn2)), (-(locals.var_q_n0_cur__blk889_dn4 + locals.var_q_nl_cur__blk890_dn4)), (-(locals.var_q_n0_cur__blk889_dn5 + locals.var_q_nl_cur__blk890_dn5)), (-(locals.var_q_n0_cur__blk889_dn6 + locals.var_q_nl_cur__blk890_dn6)), (-(locals.var_q_n0_cur__blk889_dn7 + locals.var_q_nl_cur__blk890_dn7)), (-(locals.var_q_n0_cur__blk889_dn8 + locals.var_q_nl_cur__blk890_dn8)), (-(locals.var_q_n0_cur__blk889_dn9 + locals.var_q_nl_cur__blk890_dn9)), (-(locals.var_q_n0_cur__blk889_dn10 + locals.var_q_nl_cur__blk890_dn10)), (-(locals.var_q_n0_cur__blk889_dn13 + locals.var_q_nl_cur__blk890_dn13)),)
    } else {
        (locals.var_qn_drift__blk894, locals.var_qn_drift__blk894_dn0, locals.var_qn_drift__blk894_dn2, locals.var_qn_drift__blk894_dn4, locals.var_qn_drift__blk894_dn5, locals.var_qn_drift__blk894_dn6, locals.var_qn_drift__blk894_dn7, locals.var_qn_drift__blk894_dn8, locals.var_qn_drift__blk894_dn9, locals.var_qn_drift__blk894_dn10, locals.var_qn_drift__blk894_dn13,)
    }
};
        locals.var_qn_drift__blk894 = assign42160_e56673;
        locals.var_qn_drift__blk894_dn0 = assign42160_e56673_d_n0;
        locals.var_qn_drift__blk894_dn2 = assign42160_e56673_d_n2;
        locals.var_qn_drift__blk894_dn4 = assign42160_e56673_d_n4;
        locals.var_qn_drift__blk894_dn5 = assign42160_e56673_d_n5;
        locals.var_qn_drift__blk894_dn6 = assign42160_e56673_d_n6;
        locals.var_qn_drift__blk894_dn7 = assign42160_e56673_d_n7;
        locals.var_qn_drift__blk894_dn8 = assign42160_e56673_d_n8;
        locals.var_qn_drift__blk894_dn9 = assign42160_e56673_d_n9;
        locals.var_qn_drift__blk894_dn10 = assign42160_e56673_d_n10;
        locals.var_qn_drift__blk894_dn13 = assign42160_e56673_d_n13;
        locals.var_qn_drift__blk894_rv = 0.0;

        let assign42170_e56676: f64 = if locals.var_pds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1052 = assign42170_e56676;
        locals.var_guard1052_rv = 0.0;

        let (assign42180_e56687, assign42180_e56687_d_n0, assign42180_e56687_d_n2, assign42180_e56687_d_n4, assign42180_e56687_d_n5, assign42180_e56687_d_n6, assign42180_e56687_d_n7, assign42180_e56687_d_n8, assign42180_e56687_d_n9, assign42180_e56687_d_n10, assign42180_e56687_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1052 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn13,)
    }
};
        locals.var_pds = assign42180_e56687;
        locals.var_pds_dn0 = assign42180_e56687_d_n0;
        locals.var_pds_dn2 = assign42180_e56687_d_n2;
        locals.var_pds_dn4 = assign42180_e56687_d_n4;
        locals.var_pds_dn5 = assign42180_e56687_d_n5;
        locals.var_pds_dn6 = assign42180_e56687_d_n6;
        locals.var_pds_dn7 = assign42180_e56687_d_n7;
        locals.var_pds_dn8 = assign42180_e56687_d_n8;
        locals.var_pds_dn9 = assign42180_e56687_d_n9;
        locals.var_pds_dn10 = assign42180_e56687_d_n10;
        locals.var_pds_dn13 = assign42180_e56687_d_n13;
        locals.var_pds_rv = 0.0;

        let (assign42190_e56698, assign42190_e56698_d_n0, assign42190_e56698_d_n2, assign42190_e56698_d_n4, assign42190_e56698_d_n5, assign42190_e56698_d_n6, assign42190_e56698_d_n7, assign42190_e56698_d_n8, assign42190_e56698_d_n9, assign42190_e56698_d_n10, assign42190_e56698_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1052 != 0.0)) {
        (locals.var_phi_s0_dep__blk853, locals.var_phi_s0_dep__blk853_dn0, locals.var_phi_s0_dep__blk853_dn2, locals.var_phi_s0_dep__blk853_dn4, locals.var_phi_s0_dep__blk853_dn5, locals.var_phi_s0_dep__blk853_dn6, locals.var_phi_s0_dep__blk853_dn7, locals.var_phi_s0_dep__blk853_dn8, locals.var_phi_s0_dep__blk853_dn9, locals.var_phi_s0_dep__blk853_dn10, locals.var_phi_s0_dep__blk853_dn13,)
    } else {
        (locals.var_phi_sl_dep__blk854, locals.var_phi_sl_dep__blk854_dn0, locals.var_phi_sl_dep__blk854_dn2, locals.var_phi_sl_dep__blk854_dn4, locals.var_phi_sl_dep__blk854_dn5, locals.var_phi_sl_dep__blk854_dn6, locals.var_phi_sl_dep__blk854_dn7, locals.var_phi_sl_dep__blk854_dn8, locals.var_phi_sl_dep__blk854_dn9, locals.var_phi_sl_dep__blk854_dn10, locals.var_phi_sl_dep__blk854_dn13,)
    }
};
        locals.var_phi_sl_dep__blk854 = assign42190_e56698;
        locals.var_phi_sl_dep__blk854_dn0 = assign42190_e56698_d_n0;
        locals.var_phi_sl_dep__blk854_dn2 = assign42190_e56698_d_n2;
        locals.var_phi_sl_dep__blk854_dn4 = assign42190_e56698_d_n4;
        locals.var_phi_sl_dep__blk854_dn5 = assign42190_e56698_d_n5;
        locals.var_phi_sl_dep__blk854_dn6 = assign42190_e56698_d_n6;
        locals.var_phi_sl_dep__blk854_dn7 = assign42190_e56698_d_n7;
        locals.var_phi_sl_dep__blk854_dn8 = assign42190_e56698_d_n8;
        locals.var_phi_sl_dep__blk854_dn9 = assign42190_e56698_d_n9;
        locals.var_phi_sl_dep__blk854_dn10 = assign42190_e56698_d_n10;
        locals.var_phi_sl_dep__blk854_dn13 = assign42190_e56698_d_n13;
        locals.var_phi_sl_dep__blk854_rv = 0.0;

        let (assign42200_e56709, assign42200_e56709_d_n0, assign42200_e56709_d_n2, assign42200_e56709_d_n4, assign42200_e56709_d_n5, assign42200_e56709_d_n6, assign42200_e56709_d_n7, assign42200_e56709_d_n8, assign42200_e56709_d_n9, assign42200_e56709_d_n10, assign42200_e56709_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1052 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn13,)
    }
};
        locals.var_idd = assign42200_e56709;
        locals.var_idd_dn0 = assign42200_e56709_d_n0;
        locals.var_idd_dn2 = assign42200_e56709_d_n2;
        locals.var_idd_dn4 = assign42200_e56709_d_n4;
        locals.var_idd_dn5 = assign42200_e56709_d_n5;
        locals.var_idd_dn6 = assign42200_e56709_d_n6;
        locals.var_idd_dn7 = assign42200_e56709_d_n7;
        locals.var_idd_dn8 = assign42200_e56709_d_n8;
        locals.var_idd_dn9 = assign42200_e56709_d_n9;
        locals.var_idd_dn10 = assign42200_e56709_d_n10;
        locals.var_idd_dn13 = assign42200_e56709_d_n13;
        locals.var_idd_rv = 0.0;

        let (assign42210_e56727, assign42210_e56727_d_n0, assign42210_e56727_d_n2, assign42210_e56727_d_n4, assign42210_e56727_d_n5, assign42210_e56727_d_n6, assign42210_e56727_d_n7, assign42210_e56727_d_n8, assign42210_e56727_d_n9, assign42210_e56727_d_n10, assign42210_e56727_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1052 == 0.0)) {
        let assign42210_e56721: f64 = (locals.var_beta * locals.var_qn_drift__blk894);
        let assign42210_e56723: f64 = (assign42210_e56721 / 2.0);
        let assign42210_e56725: f64 = (assign42210_e56723 * locals.var_pds);
        (assign42210_e56725, (((((locals.var_beta_dn0 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn0)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn0)), (((((locals.var_beta_dn2 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn2)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn2)), (((((locals.var_beta_dn4 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn4)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn4)), (((((locals.var_beta_dn5 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn5)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn5)), (((((locals.var_beta_dn6 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn6)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn6)), (((((locals.var_beta_dn7 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn7)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn7)), (((((locals.var_beta_dn8 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn8)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn8)), (((((locals.var_beta_dn9 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn9)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn9)), (((((locals.var_beta_dn10 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn10)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn10)), (((((locals.var_beta_dn13 * locals.var_qn_drift__blk894) + (locals.var_beta * locals.var_qn_drift__blk894_dn13)) / 2.0) * locals.var_pds) + (assign42210_e56723 * locals.var_pds_dn13)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn13,)
    }
};
        locals.var_idd = assign42210_e56727;
        locals.var_idd_dn0 = assign42210_e56727_d_n0;
        locals.var_idd_dn2 = assign42210_e56727_d_n2;
        locals.var_idd_dn4 = assign42210_e56727_d_n4;
        locals.var_idd_dn5 = assign42210_e56727_d_n5;
        locals.var_idd_dn6 = assign42210_e56727_d_n6;
        locals.var_idd_dn7 = assign42210_e56727_d_n7;
        locals.var_idd_dn8 = assign42210_e56727_d_n8;
        locals.var_idd_dn9 = assign42210_e56727_d_n9;
        locals.var_idd_dn10 = assign42210_e56727_d_n10;
        locals.var_idd_dn13 = assign42210_e56727_d_n13;
        locals.var_idd_rv = 0.0;

        let (assign42220_e56744, assign42220_e56744_d_n0, assign42220_e56744_d_n2, assign42220_e56744_d_n4, assign42220_e56744_d_n5, assign42220_e56744_d_n6, assign42220_e56744_d_n7, assign42220_e56744_d_n8, assign42220_e56744_d_n9, assign42220_e56744_d_n10, assign42220_e56744_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1052 == 0.0)) {
        let (assign42220_e56742, assign42220_e56742_d_n0, assign42220_e56742_d_n2, assign42220_e56742_d_n4, assign42220_e56742_d_n5, assign42220_e56742_d_n6, assign42220_e56742_d_n7, assign42220_e56742_d_n8, assign42220_e56742_d_n9, assign42220_e56742_d_n10, assign42220_e56742_d_n13,) = {
            if (locals.var_idd < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn13,)
            }
        };
        (assign42220_e56742, assign42220_e56742_d_n0, assign42220_e56742_d_n2, assign42220_e56742_d_n4, assign42220_e56742_d_n5, assign42220_e56742_d_n6, assign42220_e56742_d_n7, assign42220_e56742_d_n8, assign42220_e56742_d_n9, assign42220_e56742_d_n10, assign42220_e56742_d_n13,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn13,)
    }
};
        locals.var_idd = assign42220_e56744;
        locals.var_idd_dn0 = assign42220_e56744_d_n0;
        locals.var_idd_dn2 = assign42220_e56744_d_n2;
        locals.var_idd_dn4 = assign42220_e56744_d_n4;
        locals.var_idd_dn5 = assign42220_e56744_d_n5;
        locals.var_idd_dn6 = assign42220_e56744_d_n6;
        locals.var_idd_dn7 = assign42220_e56744_d_n7;
        locals.var_idd_dn8 = assign42220_e56744_d_n8;
        locals.var_idd_dn9 = assign42220_e56744_d_n9;
        locals.var_idd_dn10 = assign42220_e56744_d_n10;
        locals.var_idd_dn13 = assign42220_e56744_d_n13;
        locals.var_idd_rv = 0.0;

        let (assign42230_e56754, assign42230_e56754_d_n0, assign42230_e56754_d_n2, assign42230_e56754_d_n4, assign42230_e56754_d_n5, assign42230_e56754_d_n6, assign42230_e56754_d_n7, assign42230_e56754_d_n8, assign42230_e56754_d_n9, assign42230_e56754_d_n10, assign42230_e56754_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42230_e56752: f64 = (-locals.var_q_n0_sym);
        (assign42230_e56752, (-locals.var_q_n0_sym_dn0), (-locals.var_q_n0_sym_dn2), (-locals.var_q_n0_sym_dn4), (-locals.var_q_n0_sym_dn5), (-locals.var_q_n0_sym_dn6), (-locals.var_q_n0_sym_dn7), (-locals.var_q_n0_sym_dn8), (-locals.var_q_n0_sym_dn9), (-locals.var_q_n0_sym_dn10), (-locals.var_q_n0_sym_dn13),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn13,)
    }
};
        locals.var_qn0 = assign42230_e56754;
        locals.var_qn0_dn0 = assign42230_e56754_d_n0;
        locals.var_qn0_dn2 = assign42230_e56754_d_n2;
        locals.var_qn0_dn4 = assign42230_e56754_d_n4;
        locals.var_qn0_dn5 = assign42230_e56754_d_n5;
        locals.var_qn0_dn6 = assign42230_e56754_d_n6;
        locals.var_qn0_dn7 = assign42230_e56754_d_n7;
        locals.var_qn0_dn8 = assign42230_e56754_d_n8;
        locals.var_qn0_dn9 = assign42230_e56754_d_n9;
        locals.var_qn0_dn10 = assign42230_e56754_d_n10;
        locals.var_qn0_dn13 = assign42230_e56754_d_n13;
        locals.var_qn0_rv = 0.0;

        let (assign42240_e56763, assign42240_e56763_d_n0, assign42240_e56763_d_n2, assign42240_e56763_d_n4, assign42240_e56763_d_n5, assign42240_e56763_d_n6, assign42240_e56763_d_n7, assign42240_e56763_d_n8, assign42240_e56763_d_n9, assign42240_e56763_d_n10, assign42240_e56763_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_leff, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    }
};
        locals.var_lch = assign42240_e56763;
        locals.var_lch_dn0 = assign42240_e56763_d_n0;
        locals.var_lch_dn2 = assign42240_e56763_d_n2;
        locals.var_lch_dn4 = assign42240_e56763_d_n4;
        locals.var_lch_dn5 = assign42240_e56763_d_n5;
        locals.var_lch_dn6 = assign42240_e56763_d_n6;
        locals.var_lch_dn7 = assign42240_e56763_d_n7;
        locals.var_lch_dn8 = assign42240_e56763_d_n8;
        locals.var_lch_dn9 = assign42240_e56763_d_n9;
        locals.var_lch_dn10 = assign42240_e56763_d_n10;
        locals.var_lch_dn13 = assign42240_e56763_d_n13;
        locals.var_lch_rv = 0.0;

        let (assign42250_e56774, assign42250_e56774_d_n0, assign42250_e56774_d_n2, assign42250_e56774_d_n4, assign42250_e56774_d_n5, assign42250_e56774_d_n6, assign42250_e56774_d_n7, assign42250_e56774_d_n8, assign42250_e56774_d_n9, assign42250_e56774_d_n10, assign42250_e56774_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42250_e56772: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign42250_e56772, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign42250_e56774;
        locals.var_t2_dn0 = assign42250_e56774_d_n0;
        locals.var_t2_dn2 = assign42250_e56774_d_n2;
        locals.var_t2_dn4 = assign42250_e56774_d_n4;
        locals.var_t2_dn5 = assign42250_e56774_d_n5;
        locals.var_t2_dn6 = assign42250_e56774_d_n6;
        locals.var_t2_dn7 = assign42250_e56774_d_n7;
        locals.var_t2_dn8 = assign42250_e56774_d_n8;
        locals.var_t2_dn9 = assign42250_e56774_d_n9;
        locals.var_t2_dn10 = assign42250_e56774_d_n10;
        locals.var_t2_dn13 = assign42250_e56774_d_n13;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_141(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42260_e56783, assign42260_e56783_d_n0, assign42260_e56783_d_n2, assign42260_e56783_d_n4, assign42260_e56783_d_n5, assign42260_e56783_d_n6, assign42260_e56783_d_n7, assign42260_e56783_d_n8, assign42260_e56783_d_n9, assign42260_e56783_d_n10, assign42260_e56783_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign42260_e56783;
        locals.var_t0_dn0 = assign42260_e56783_d_n0;
        locals.var_t0_dn2 = assign42260_e56783_d_n2;
        locals.var_t0_dn4 = assign42260_e56783_d_n4;
        locals.var_t0_dn5 = assign42260_e56783_d_n5;
        locals.var_t0_dn6 = assign42260_e56783_d_n6;
        locals.var_t0_dn7 = assign42260_e56783_d_n7;
        locals.var_t0_dn8 = assign42260_e56783_d_n8;
        locals.var_t0_dn9 = assign42260_e56783_d_n9;
        locals.var_t0_dn10 = assign42260_e56783_d_n10;
        locals.var_t0_dn13 = assign42260_e56783_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign42270_e56800, assign42270_e56800_d_n0, assign42270_e56800_d_n2, assign42270_e56800_d_n4, assign42270_e56800_d_n5, assign42270_e56800_d_n6, assign42270_e56800_d_n7, assign42270_e56800_d_n8, assign42270_e56800_d_n9, assign42270_e56800_d_n10, assign42270_e56800_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42270_e56792: f64 = (locals.var_pds * locals.var_pds);
        let assign42270_e56794: f64 = (assign42270_e56792 + p.p262);
        let assign42270_e56795: f64 = (assign42270_e56794).sqrt();
        let assign42270_e56797: f64 = (p.p262).sqrt();
        let assign42270_e56798: f64 = (assign42270_e56795 - assign42270_e56797);
        (assign42270_e56798, (((locals.var_pds_dn0 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn0)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn2 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn2)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn4 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn4)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn5 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn5)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn6 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn6)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn7 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn7)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn8 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn8)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn9 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn9)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn10 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn10)) / (2.0 * assign42270_e56795)), (((locals.var_pds_dn13 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn13)) / (2.0 * assign42270_e56795)),)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn7, locals.var_pdsz_dn8, locals.var_pdsz_dn9, locals.var_pdsz_dn10, locals.var_pdsz_dn13,)
    }
};
        locals.var_pdsz = assign42270_e56800;
        locals.var_pdsz_dn0 = assign42270_e56800_d_n0;
        locals.var_pdsz_dn2 = assign42270_e56800_d_n2;
        locals.var_pdsz_dn4 = assign42270_e56800_d_n4;
        locals.var_pdsz_dn5 = assign42270_e56800_d_n5;
        locals.var_pdsz_dn6 = assign42270_e56800_d_n6;
        locals.var_pdsz_dn7 = assign42270_e56800_d_n7;
        locals.var_pdsz_dn8 = assign42270_e56800_d_n8;
        locals.var_pdsz_dn9 = assign42270_e56800_d_n9;
        locals.var_pdsz_dn10 = assign42270_e56800_d_n10;
        locals.var_pdsz_dn13 = assign42270_e56800_d_n13;
        locals.var_pdsz_rv = 0.0;

        let (assign42280_e56813, assign42280_e56813_d_n0, assign42280_e56813_d_n2, assign42280_e56813_d_n4, assign42280_e56813_d_n5, assign42280_e56813_d_n6, assign42280_e56813_d_n7, assign42280_e56813_d_n8, assign42280_e56813_d_n9, assign42280_e56813_d_n10, assign42280_e56813_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42280_e56810: f64 = (locals.var_pdsz * locals.var_t0);
        let assign42280_e56811: f64 = (1.0 + assign42280_e56810);
        (assign42280_e56811, ((locals.var_pdsz_dn0 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn0)), ((locals.var_pdsz_dn2 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn2)), ((locals.var_pdsz_dn4 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn4)), ((locals.var_pdsz_dn5 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn5)), ((locals.var_pdsz_dn6 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn6)), ((locals.var_pdsz_dn7 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn7)), ((locals.var_pdsz_dn8 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn8)), ((locals.var_pdsz_dn9 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn9)), ((locals.var_pdsz_dn10 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn10)), ((locals.var_pdsz_dn13 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign42280_e56813;
        locals.var_t4_dn0 = assign42280_e56813_d_n0;
        locals.var_t4_dn2 = assign42280_e56813_d_n2;
        locals.var_t4_dn4 = assign42280_e56813_d_n4;
        locals.var_t4_dn5 = assign42280_e56813_d_n5;
        locals.var_t4_dn6 = assign42280_e56813_d_n6;
        locals.var_t4_dn7 = assign42280_e56813_d_n7;
        locals.var_t4_dn8 = assign42280_e56813_d_n8;
        locals.var_t4_dn9 = assign42280_e56813_d_n9;
        locals.var_t4_dn10 = assign42280_e56813_d_n10;
        locals.var_t4_dn13 = assign42280_e56813_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign42290_e56824, assign42290_e56824_d_n0, assign42290_e56824_d_n2, assign42290_e56824_d_n4, assign42290_e56824_d_n5, assign42290_e56824_d_n6, assign42290_e56824_d_n7, assign42290_e56824_d_n8, assign42290_e56824_d_n9, assign42290_e56824_d_n10, assign42290_e56824_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42290_e56822: f64 = (locals.var_t2 * locals.var_qn0);
        (assign42290_e56822, ((locals.var_t2_dn0 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn0)), ((locals.var_t2_dn2 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn2)), ((locals.var_t2_dn4 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn4)), ((locals.var_t2_dn5 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn5)), ((locals.var_t2_dn6 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn6)), ((locals.var_t2_dn7 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn7)), ((locals.var_t2_dn8 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn8)), ((locals.var_t2_dn9 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn9)), ((locals.var_t2_dn10 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn10)), ((locals.var_t2_dn13 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign42290_e56824;
        locals.var_t5_dn0 = assign42290_e56824_d_n0;
        locals.var_t5_dn2 = assign42290_e56824_d_n2;
        locals.var_t5_dn4 = assign42290_e56824_d_n4;
        locals.var_t5_dn5 = assign42290_e56824_d_n5;
        locals.var_t5_dn6 = assign42290_e56824_d_n6;
        locals.var_t5_dn7 = assign42290_e56824_d_n7;
        locals.var_t5_dn8 = assign42290_e56824_d_n8;
        locals.var_t5_dn9 = assign42290_e56824_d_n9;
        locals.var_t5_dn10 = assign42290_e56824_d_n10;
        locals.var_t5_dn13 = assign42290_e56824_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign42300_e56835, assign42300_e56835_d_n0, assign42300_e56835_d_n2, assign42300_e56835_d_n4, assign42300_e56835_d_n5, assign42300_e56835_d_n6, assign42300_e56835_d_n7, assign42300_e56835_d_n8, assign42300_e56835_d_n9, assign42300_e56835_d_n10, assign42300_e56835_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42300_e56833: f64 = (locals.var_t5 / locals.var_t4);
        (assign42300_e56833, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn13 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign42300_e56835;
        locals.var_t3_dn0 = assign42300_e56835_d_n0;
        locals.var_t3_dn2 = assign42300_e56835_d_n2;
        locals.var_t3_dn4 = assign42300_e56835_d_n4;
        locals.var_t3_dn5 = assign42300_e56835_d_n5;
        locals.var_t3_dn6 = assign42300_e56835_d_n6;
        locals.var_t3_dn7 = assign42300_e56835_d_n7;
        locals.var_t3_dn8 = assign42300_e56835_d_n8;
        locals.var_t3_dn9 = assign42300_e56835_d_n9;
        locals.var_t3_dn10 = assign42300_e56835_d_n10;
        locals.var_t3_dn13 = assign42300_e56835_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign42310_e56844, assign42310_e56844_d_n0, assign42310_e56844_d_n2, assign42310_e56844_d_n4, assign42310_e56844_d_n5, assign42310_e56844_d_n6, assign42310_e56844_d_n7, assign42310_e56844_d_n8, assign42310_e56844_d_n9, assign42310_e56844_d_n10, assign42310_e56844_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn13,)
    }
};
        locals.var_eeff = assign42310_e56844;
        locals.var_eeff_dn0 = assign42310_e56844_d_n0;
        locals.var_eeff_dn2 = assign42310_e56844_d_n2;
        locals.var_eeff_dn4 = assign42310_e56844_d_n4;
        locals.var_eeff_dn5 = assign42310_e56844_d_n5;
        locals.var_eeff_dn6 = assign42310_e56844_d_n6;
        locals.var_eeff_dn7 = assign42310_e56844_d_n7;
        locals.var_eeff_dn8 = assign42310_e56844_d_n8;
        locals.var_eeff_dn9 = assign42310_e56844_d_n9;
        locals.var_eeff_dn10 = assign42310_e56844_d_n10;
        locals.var_eeff_dn13 = assign42310_e56844_d_n13;
        locals.var_eeff_rv = 0.0;

        let (assign42320_e56862, assign42320_e56862_d_n0, assign42320_e56862_d_n2, assign42320_e56862_d_n4, assign42320_e56862_d_n5, assign42320_e56862_d_n6, assign42320_e56862_d_n7, assign42320_e56862_d_n8, assign42320_e56862_d_n9, assign42320_e56862_d_n10, assign42320_e56862_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let (assign42320_e56860, assign42320_e56860_d_n0, assign42320_e56860_d_n2, assign42320_e56860_d_n4, assign42320_e56860_d_n5, assign42320_e56860_d_n6, assign42320_e56860_d_n7, assign42320_e56860_d_n8, assign42320_e56860_d_n9, assign42320_e56860_d_n10, assign42320_e56860_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42320_e56858: f64 = (p.p160 - 1.0);
                let assign42320_e56859: f64 = (locals.var_eeff).powf(assign42320_e56858);
                (assign42320_e56859, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn0)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn2)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn4)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn5)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn6)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn7)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn8)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn9)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn10)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42320_e56858) as f64).is_finite() && ((assign42320_e56858) as f64).fract() == 0.0 { if assign42320_e56858 == 0.0 { 0.0 } else { (assign42320_e56858 * ((locals.var_eeff).powf(assign42320_e56858 - 1.0) * locals.var_eeff_dn13)) } } else { (assign42320_e56859 * (assign42320_e56858 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign42320_e56860, assign42320_e56860_d_n0, assign42320_e56860_d_n2, assign42320_e56860_d_n4, assign42320_e56860_d_n5, assign42320_e56860_d_n6, assign42320_e56860_d_n7, assign42320_e56860_d_n8, assign42320_e56860_d_n9, assign42320_e56860_d_n10, assign42320_e56860_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign42320_e56862;
        locals.var_t5_dn0 = assign42320_e56862_d_n0;
        locals.var_t5_dn2 = assign42320_e56862_d_n2;
        locals.var_t5_dn4 = assign42320_e56862_d_n4;
        locals.var_t5_dn5 = assign42320_e56862_d_n5;
        locals.var_t5_dn6 = assign42320_e56862_d_n6;
        locals.var_t5_dn7 = assign42320_e56862_d_n7;
        locals.var_t5_dn8 = assign42320_e56862_d_n8;
        locals.var_t5_dn9 = assign42320_e56862_d_n9;
        locals.var_t5_dn10 = assign42320_e56862_d_n10;
        locals.var_t5_dn13 = assign42320_e56862_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign42330_e56873, assign42330_e56873_d_n0, assign42330_e56873_d_n2, assign42330_e56873_d_n4, assign42330_e56873_d_n5, assign42330_e56873_d_n6, assign42330_e56873_d_n7, assign42330_e56873_d_n8, assign42330_e56873_d_n9, assign42330_e56873_d_n10, assign42330_e56873_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42330_e56871: f64 = (locals.var_t5 * locals.var_eeff);
        (assign42330_e56871, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn13 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign42330_e56873;
        locals.var_t8_dn0 = assign42330_e56873_d_n0;
        locals.var_t8_dn2 = assign42330_e56873_d_n2;
        locals.var_t8_dn4 = assign42330_e56873_d_n4;
        locals.var_t8_dn5 = assign42330_e56873_d_n5;
        locals.var_t8_dn6 = assign42330_e56873_d_n6;
        locals.var_t8_dn7 = assign42330_e56873_d_n7;
        locals.var_t8_dn8 = assign42330_e56873_d_n8;
        locals.var_t8_dn9 = assign42330_e56873_d_n9;
        locals.var_t8_dn10 = assign42330_e56873_d_n10;
        locals.var_t8_dn13 = assign42330_e56873_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign42340_e56891, assign42340_e56891_d_n0, assign42340_e56891_d_n2, assign42340_e56891_d_n4, assign42340_e56891_d_n5, assign42340_e56891_d_n6, assign42340_e56891_d_n7, assign42340_e56891_d_n8, assign42340_e56891_d_n9, assign42340_e56891_d_n10, assign42340_e56891_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let (assign42340_e56889, assign42340_e56889_d_n0, assign42340_e56889_d_n2, assign42340_e56889_d_n4, assign42340_e56889_d_n5, assign42340_e56889_d_n6, assign42340_e56889_d_n7, assign42340_e56889_d_n8, assign42340_e56889_d_n9, assign42340_e56889_d_n10, assign42340_e56889_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42340_e56887: f64 = (locals.var_muesr - 1.0);
                let assign42340_e56888: f64 = (locals.var_eeff).powf(assign42340_e56887);
                (assign42340_e56888, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn0)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn2)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn4)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn5)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn6)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn7)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn8)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn9)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn10)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42340_e56887) as f64).is_finite() && ((assign42340_e56887) as f64).fract() == 0.0 { if assign42340_e56887 == 0.0 { 0.0 } else { (assign42340_e56887 * ((locals.var_eeff).powf(assign42340_e56887 - 1.0) * locals.var_eeff_dn13)) } } else { (assign42340_e56888 * (assign42340_e56887 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign42340_e56889, assign42340_e56889_d_n0, assign42340_e56889_d_n2, assign42340_e56889_d_n4, assign42340_e56889_d_n5, assign42340_e56889_d_n6, assign42340_e56889_d_n7, assign42340_e56889_d_n8, assign42340_e56889_d_n9, assign42340_e56889_d_n10, assign42340_e56889_d_n13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign42340_e56891;
        locals.var_t7_dn0 = assign42340_e56891_d_n0;
        locals.var_t7_dn2 = assign42340_e56891_d_n2;
        locals.var_t7_dn4 = assign42340_e56891_d_n4;
        locals.var_t7_dn5 = assign42340_e56891_d_n5;
        locals.var_t7_dn6 = assign42340_e56891_d_n6;
        locals.var_t7_dn7 = assign42340_e56891_d_n7;
        locals.var_t7_dn8 = assign42340_e56891_d_n8;
        locals.var_t7_dn9 = assign42340_e56891_d_n9;
        locals.var_t7_dn10 = assign42340_e56891_d_n10;
        locals.var_t7_dn13 = assign42340_e56891_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign42350_e56902, assign42350_e56902_d_n0, assign42350_e56902_d_n2, assign42350_e56902_d_n4, assign42350_e56902_d_n5, assign42350_e56902_d_n6, assign42350_e56902_d_n7, assign42350_e56902_d_n8, assign42350_e56902_d_n9, assign42350_e56902_d_n10, assign42350_e56902_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42350_e56900: f64 = (locals.var_t7 * locals.var_eeff);
        (assign42350_e56900, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn13 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign42350_e56902;
        locals.var_t6_dn0 = assign42350_e56902_d_n0;
        locals.var_t6_dn2 = assign42350_e56902_d_n2;
        locals.var_t6_dn4 = assign42350_e56902_d_n4;
        locals.var_t6_dn5 = assign42350_e56902_d_n5;
        locals.var_t6_dn6 = assign42350_e56902_d_n6;
        locals.var_t6_dn7 = assign42350_e56902_d_n7;
        locals.var_t6_dn8 = assign42350_e56902_d_n8;
        locals.var_t6_dn9 = assign42350_e56902_d_n9;
        locals.var_t6_dn10 = assign42350_e56902_d_n10;
        locals.var_t6_dn13 = assign42350_e56902_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign42360_e56913, assign42360_e56913_d_n0, assign42360_e56913_d_n2, assign42360_e56913_d_n4, assign42360_e56913_d_n5, assign42360_e56913_d_n6, assign42360_e56913_d_n7, assign42360_e56913_d_n8, assign42360_e56913_d_n9, assign42360_e56913_d_n10, assign42360_e56913_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42360_e56911: f64 = (1.6021918e-19 * 10000.0);
        (assign42360_e56911, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign42360_e56913;
        locals.var_t9_dn0 = assign42360_e56913_d_n0;
        locals.var_t9_dn2 = assign42360_e56913_d_n2;
        locals.var_t9_dn4 = assign42360_e56913_d_n4;
        locals.var_t9_dn5 = assign42360_e56913_d_n5;
        locals.var_t9_dn6 = assign42360_e56913_d_n6;
        locals.var_t9_dn7 = assign42360_e56913_d_n7;
        locals.var_t9_dn8 = assign42360_e56913_d_n8;
        locals.var_t9_dn9 = assign42360_e56913_d_n9;
        locals.var_t9_dn10 = assign42360_e56913_d_n10;
        locals.var_t9_dn13 = assign42360_e56913_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign42370_e56924, assign42370_e56924_d_n0, assign42370_e56924_d_n2, assign42370_e56924_d_n4, assign42370_e56924_d_n5, assign42370_e56924_d_n6, assign42370_e56924_d_n7, assign42370_e56924_d_n8, assign42370_e56924_d_n9, assign42370_e56924_d_n10, assign42370_e56924_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42370_e56922: f64 = (locals.var_qn0 / locals.var_t9);
        (assign42370_e56922, (((locals.var_qn0_dn0 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn2 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn4 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn5 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn6 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn7 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn8 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn9 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn10 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn13 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn13,)
    }
};
        locals.var_rns = assign42370_e56924;
        locals.var_rns_dn0 = assign42370_e56924_d_n0;
        locals.var_rns_dn2 = assign42370_e56924_d_n2;
        locals.var_rns_dn4 = assign42370_e56924_d_n4;
        locals.var_rns_dn5 = assign42370_e56924_d_n5;
        locals.var_rns_dn6 = assign42370_e56924_d_n6;
        locals.var_rns_dn7 = assign42370_e56924_d_n7;
        locals.var_rns_dn8 = assign42370_e56924_d_n8;
        locals.var_rns_dn9 = assign42370_e56924_d_n9;
        locals.var_rns_dn10 = assign42370_e56924_d_n10;
        locals.var_rns_dn13 = assign42370_e56924_d_n13;
        locals.var_rns_rv = 0.0;

        let (assign42380_e56933, assign42380_e56933_d_n0, assign42380_e56933_d_n2, assign42380_e56933_d_n4, assign42380_e56933_d_n5, assign42380_e56933_d_n6, assign42380_e56933_d_n7, assign42380_e56933_d_n8, assign42380_e56933_d_n9, assign42380_e56933_d_n10, assign42380_e56933_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_uc_muecb0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign42380_e56933;
        locals.var_t2_dn0 = assign42380_e56933_d_n0;
        locals.var_t2_dn2 = assign42380_e56933_d_n2;
        locals.var_t2_dn4 = assign42380_e56933_d_n4;
        locals.var_t2_dn5 = assign42380_e56933_d_n5;
        locals.var_t2_dn6 = assign42380_e56933_d_n6;
        locals.var_t2_dn7 = assign42380_e56933_d_n7;
        locals.var_t2_dn8 = assign42380_e56933_d_n8;
        locals.var_t2_dn9 = assign42380_e56933_d_n9;
        locals.var_t2_dn10 = assign42380_e56933_d_n10;
        locals.var_t2_dn13 = assign42380_e56933_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign42390_e56958, assign42390_e56958_d_n0, assign42390_e56958_d_n2, assign42390_e56958_d_n4, assign42390_e56958_d_n5, assign42390_e56958_d_n6, assign42390_e56958_d_n7, assign42390_e56958_d_n8, assign42390_e56958_d_n9, assign42390_e56958_d_n10, assign42390_e56958_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42390_e56944: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign42390_e56946: f64 = (assign42390_e56944 / 100000000000.0);
        let assign42390_e56947: f64 = (locals.var_t2 + assign42390_e56946);
        let assign42390_e56948: f64 = (1.0 / assign42390_e56947);
        let assign42390_e56951: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign42390_e56952: f64 = (assign42390_e56948 + assign42390_e56951);
        let assign42390_e56955: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign42390_e56956: f64 = (assign42390_e56952 + assign42390_e56955);
        (assign42390_e56956, (((-((locals.var_t2_dn0 + ((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn2 + ((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn4 + ((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn5 + ((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn6 + ((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn7 + ((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn8 + ((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn9 + ((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn10 + ((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn13 + ((locals.var_uc_muecb1 * locals.var_rns_dn13) / 100000000000.0)) / (assign42390_e56947 * assign42390_e56947))) + ((locals.var_mphn0_dn13 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn13))) + (locals.var_t6_dn13 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign42390_e56958;
        locals.var_t1_dn0 = assign42390_e56958_d_n0;
        locals.var_t1_dn2 = assign42390_e56958_d_n2;
        locals.var_t1_dn4 = assign42390_e56958_d_n4;
        locals.var_t1_dn5 = assign42390_e56958_d_n5;
        locals.var_t1_dn6 = assign42390_e56958_d_n6;
        locals.var_t1_dn7 = assign42390_e56958_d_n7;
        locals.var_t1_dn8 = assign42390_e56958_d_n8;
        locals.var_t1_dn9 = assign42390_e56958_d_n9;
        locals.var_t1_dn10 = assign42390_e56958_d_n10;
        locals.var_t1_dn13 = assign42390_e56958_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign42400_e56969, assign42400_e56969_d_n0, assign42400_e56969_d_n2, assign42400_e56969_d_n4, assign42400_e56969_d_n5, assign42400_e56969_d_n6, assign42400_e56969_d_n7, assign42400_e56969_d_n8, assign42400_e56969_d_n9, assign42400_e56969_d_n10, assign42400_e56969_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42400_e56967: f64 = (1.0 / locals.var_t1);
        (assign42400_e56967, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign42400_e56969;
        locals.var_muun_dn0 = assign42400_e56969_d_n0;
        locals.var_muun_dn2 = assign42400_e56969_d_n2;
        locals.var_muun_dn4 = assign42400_e56969_d_n4;
        locals.var_muun_dn5 = assign42400_e56969_d_n5;
        locals.var_muun_dn6 = assign42400_e56969_d_n6;
        locals.var_muun_dn7 = assign42400_e56969_d_n7;
        locals.var_muun_dn8 = assign42400_e56969_d_n8;
        locals.var_muun_dn9 = assign42400_e56969_d_n9;
        locals.var_muun_dn10 = assign42400_e56969_d_n10;
        locals.var_muun_dn13 = assign42400_e56969_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign42410_e56980, assign42410_e56980_d_n0, assign42410_e56980_d_n2, assign42410_e56980_d_n4, assign42410_e56980_d_n5, assign42410_e56980_d_n6, assign42410_e56980_d_n7, assign42410_e56980_d_n8, assign42410_e56980_d_n9, assign42410_e56980_d_n10, assign42410_e56980_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42410_e56978: f64 = (locals.var_muun / 10000.0);
        (assign42410_e56978, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn13 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign42410_e56980;
        locals.var_muun_dn0 = assign42410_e56980_d_n0;
        locals.var_muun_dn2 = assign42410_e56980_d_n2;
        locals.var_muun_dn4 = assign42410_e56980_d_n4;
        locals.var_muun_dn5 = assign42410_e56980_d_n5;
        locals.var_muun_dn6 = assign42410_e56980_d_n6;
        locals.var_muun_dn7 = assign42410_e56980_d_n7;
        locals.var_muun_dn8 = assign42410_e56980_d_n8;
        locals.var_muun_dn9 = assign42410_e56980_d_n9;
        locals.var_muun_dn10 = assign42410_e56980_d_n10;
        locals.var_muun_dn13 = assign42410_e56980_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign42420_e56995, assign42420_e56995_d_n0, assign42420_e56995_d_n2, assign42420_e56995_d_n4, assign42420_e56995_d_n5, assign42420_e56995_d_n6, assign42420_e56995_d_n7, assign42420_e56995_d_n8, assign42420_e56995_d_n9, assign42420_e56995_d_n10, assign42420_e56995_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42420_e56990: f64 = (locals.var_qn0 + 1e-25);
        let assign42420_e56991: f64 = (locals.var_beta * assign42420_e56990);
        let assign42420_e56993: f64 = (assign42420_e56991 * locals.var_lch);
        (assign42420_e56993, ((((locals.var_beta_dn0 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn10)), ((((locals.var_beta_dn13 * assign42420_e56990) + (locals.var_beta * locals.var_qn0_dn13)) * locals.var_lch) + (assign42420_e56991 * locals.var_lch_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign42420_e56995;
        locals.var_t2_dn0 = assign42420_e56995_d_n0;
        locals.var_t2_dn2 = assign42420_e56995_d_n2;
        locals.var_t2_dn4 = assign42420_e56995_d_n4;
        locals.var_t2_dn5 = assign42420_e56995_d_n5;
        locals.var_t2_dn6 = assign42420_e56995_d_n6;
        locals.var_t2_dn7 = assign42420_e56995_d_n7;
        locals.var_t2_dn8 = assign42420_e56995_d_n8;
        locals.var_t2_dn9 = assign42420_e56995_d_n9;
        locals.var_t2_dn10 = assign42420_e56995_d_n10;
        locals.var_t2_dn13 = assign42420_e56995_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign42430_e57006, assign42430_e57006_d_n0, assign42430_e57006_d_n2, assign42430_e57006_d_n4, assign42430_e57006_d_n5, assign42430_e57006_d_n6, assign42430_e57006_d_n7, assign42430_e57006_d_n8, assign42430_e57006_d_n9, assign42430_e57006_d_n10, assign42430_e57006_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42430_e57004: f64 = (1.0 / locals.var_t2);
        (assign42430_e57004, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign42430_e57006;
        locals.var_t1_dn0 = assign42430_e57006_d_n0;
        locals.var_t1_dn2 = assign42430_e57006_d_n2;
        locals.var_t1_dn4 = assign42430_e57006_d_n4;
        locals.var_t1_dn5 = assign42430_e57006_d_n5;
        locals.var_t1_dn6 = assign42430_e57006_d_n6;
        locals.var_t1_dn7 = assign42430_e57006_d_n7;
        locals.var_t1_dn8 = assign42430_e57006_d_n8;
        locals.var_t1_dn9 = assign42430_e57006_d_n9;
        locals.var_t1_dn10 = assign42430_e57006_d_n10;
        locals.var_t1_dn13 = assign42430_e57006_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign42440_e57017, assign42440_e57017_d_n0, assign42440_e57017_d_n2, assign42440_e57017_d_n4, assign42440_e57017_d_n5, assign42440_e57017_d_n6, assign42440_e57017_d_n7, assign42440_e57017_d_n8, assign42440_e57017_d_n9, assign42440_e57017_d_n10, assign42440_e57017_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42440_e57015: f64 = (locals.var_idd * locals.var_t1);
        (assign42440_e57015, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn13 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign42440_e57017;
        locals.var_ty_dn0 = assign42440_e57017_d_n0;
        locals.var_ty_dn2 = assign42440_e57017_d_n2;
        locals.var_ty_dn4 = assign42440_e57017_d_n4;
        locals.var_ty_dn5 = assign42440_e57017_d_n5;
        locals.var_ty_dn6 = assign42440_e57017_d_n6;
        locals.var_ty_dn7 = assign42440_e57017_d_n7;
        locals.var_ty_dn8 = assign42440_e57017_d_n8;
        locals.var_ty_dn9 = assign42440_e57017_d_n9;
        locals.var_ty_dn10 = assign42440_e57017_d_n10;
        locals.var_ty_dn13 = assign42440_e57017_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign42450_e57030, assign42450_e57030_d_n0, assign42450_e57030_d_n2, assign42450_e57030_d_n4, assign42450_e57030_d_n5, assign42450_e57030_d_n6, assign42450_e57030_d_n7, assign42450_e57030_d_n8, assign42450_e57030_d_n9, assign42450_e57030_d_n10, assign42450_e57030_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42450_e57026: f64 = (0.2 * locals.var_vmaxe);
        let assign42450_e57028: f64 = (assign42450_e57026 / locals.var_muun);
        (assign42450_e57028, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn13) * locals.var_muun) - (assign42450_e57026 * locals.var_muun_dn13)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign42450_e57030;
        locals.var_t2_dn0 = assign42450_e57030_d_n0;
        locals.var_t2_dn2 = assign42450_e57030_d_n2;
        locals.var_t2_dn4 = assign42450_e57030_d_n4;
        locals.var_t2_dn5 = assign42450_e57030_d_n5;
        locals.var_t2_dn6 = assign42450_e57030_d_n6;
        locals.var_t2_dn7 = assign42450_e57030_d_n7;
        locals.var_t2_dn8 = assign42450_e57030_d_n8;
        locals.var_t2_dn9 = assign42450_e57030_d_n9;
        locals.var_t2_dn10 = assign42450_e57030_d_n10;
        locals.var_t2_dn13 = assign42450_e57030_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign42460_e57046, assign42460_e57046_d_n0, assign42460_e57046_d_n2, assign42460_e57046_d_n4, assign42460_e57046_d_n5, assign42460_e57046_d_n6, assign42460_e57046_d_n7, assign42460_e57046_d_n8, assign42460_e57046_d_n9, assign42460_e57046_d_n10, assign42460_e57046_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42460_e57039: f64 = (locals.var_ty * locals.var_ty);
        let assign42460_e57042: f64 = (locals.var_t2 * locals.var_t2);
        let assign42460_e57043: f64 = (assign42460_e57039 + assign42460_e57042);
        let assign42460_e57044: f64 = (assign42460_e57043).sqrt();
        (assign42460_e57044, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign42460_e57044)), ((((locals.var_ty_dn13 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn13)) + ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))) / (2.0 * assign42460_e57044)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    }
};
        locals.var_ey = assign42460_e57046;
        locals.var_ey_dn0 = assign42460_e57046_d_n0;
        locals.var_ey_dn2 = assign42460_e57046_d_n2;
        locals.var_ey_dn4 = assign42460_e57046_d_n4;
        locals.var_ey_dn5 = assign42460_e57046_d_n5;
        locals.var_ey_dn6 = assign42460_e57046_d_n6;
        locals.var_ey_dn7 = assign42460_e57046_d_n7;
        locals.var_ey_dn8 = assign42460_e57046_d_n8;
        locals.var_ey_dn9 = assign42460_e57046_d_n9;
        locals.var_ey_dn10 = assign42460_e57046_d_n10;
        locals.var_ey_dn13 = assign42460_e57046_d_n13;
        locals.var_ey_rv = 0.0;

        let (assign42470_e57057, assign42470_e57057_d_n0, assign42470_e57057_d_n2, assign42470_e57057_d_n4, assign42470_e57057_d_n5, assign42470_e57057_d_n6, assign42470_e57057_d_n7, assign42470_e57057_d_n8, assign42470_e57057_d_n9, assign42470_e57057_d_n10, assign42470_e57057_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42470_e57055: f64 = (1.0 / locals.var_ey);
        (assign42470_e57055, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn13 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign42470_e57057;
        locals.var_t4_dn0 = assign42470_e57057_d_n0;
        locals.var_t4_dn2 = assign42470_e57057_d_n2;
        locals.var_t4_dn4 = assign42470_e57057_d_n4;
        locals.var_t4_dn5 = assign42470_e57057_d_n5;
        locals.var_t4_dn6 = assign42470_e57057_d_n6;
        locals.var_t4_dn7 = assign42470_e57057_d_n7;
        locals.var_t4_dn8 = assign42470_e57057_d_n8;
        locals.var_t4_dn9 = assign42470_e57057_d_n9;
        locals.var_t4_dn10 = assign42470_e57057_d_n10;
        locals.var_t4_dn13 = assign42470_e57057_d_n13;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_142(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42480_e57068, assign42480_e57068_d_n0, assign42480_e57068_d_n2, assign42480_e57068_d_n4, assign42480_e57068_d_n5, assign42480_e57068_d_n6, assign42480_e57068_d_n7, assign42480_e57068_d_n8, assign42480_e57068_d_n9, assign42480_e57068_d_n10, assign42480_e57068_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42480_e57066: f64 = (locals.var_muun * locals.var_ey);
        (assign42480_e57066, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn13 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn13)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn13,)
    }
};
        locals.var_em = assign42480_e57068;
        locals.var_em_dn0 = assign42480_e57068_d_n0;
        locals.var_em_dn2 = assign42480_e57068_d_n2;
        locals.var_em_dn4 = assign42480_e57068_d_n4;
        locals.var_em_dn5 = assign42480_e57068_d_n5;
        locals.var_em_dn6 = assign42480_e57068_d_n6;
        locals.var_em_dn7 = assign42480_e57068_d_n7;
        locals.var_em_dn8 = assign42480_e57068_d_n8;
        locals.var_em_dn9 = assign42480_e57068_d_n9;
        locals.var_em_dn10 = assign42480_e57068_d_n10;
        locals.var_em_dn13 = assign42480_e57068_d_n13;
        locals.var_em_rv = 0.0;

        let (assign42490_e57079, assign42490_e57079_d_n0, assign42490_e57079_d_n2, assign42490_e57079_d_n4, assign42490_e57079_d_n5, assign42490_e57079_d_n6, assign42490_e57079_d_n7, assign42490_e57079_d_n8, assign42490_e57079_d_n9, assign42490_e57079_d_n10, assign42490_e57079_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42490_e57077: f64 = (locals.var_em / locals.var_vmaxe);
        (assign42490_e57077, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn13 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn13)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign42490_e57079;
        locals.var_t1_dn0 = assign42490_e57079_d_n0;
        locals.var_t1_dn2 = assign42490_e57079_d_n2;
        locals.var_t1_dn4 = assign42490_e57079_d_n4;
        locals.var_t1_dn5 = assign42490_e57079_d_n5;
        locals.var_t1_dn6 = assign42490_e57079_d_n6;
        locals.var_t1_dn7 = assign42490_e57079_d_n7;
        locals.var_t1_dn8 = assign42490_e57079_d_n8;
        locals.var_t1_dn9 = assign42490_e57079_d_n9;
        locals.var_t1_dn10 = assign42490_e57079_d_n10;
        locals.var_t1_dn13 = assign42490_e57079_d_n13;
        locals.var_t1_rv = 0.0;

        let assign42500_e57083: f64 = (10.0 * 2.220446049250313e-16);
        let assign42500_e57084: f64 = (1.0 - assign42500_e57083);
        let assign42500_e57091: f64 = (10.0 * 2.220446049250313e-16);
        let assign42500_e57092: f64 = (1.0 + assign42500_e57091);
        let assign42500_e57094: f64 = if ((assign42500_e57084 <= p.p178) && (p.p178 <= assign42500_e57092)) { 1.0 } else { 0.0 };
        locals.var_guard1053 = assign42500_e57094;
        locals.var_guard1053_rv = 0.0;

        let (assign42510_e57105, assign42510_e57105_d_n0, assign42510_e57105_d_n2, assign42510_e57105_d_n4, assign42510_e57105_d_n5, assign42510_e57105_d_n6, assign42510_e57105_d_n7, assign42510_e57105_d_n8, assign42510_e57105_d_n9, assign42510_e57105_d_n10, assign42510_e57105_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1053 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign42510_e57105;
        locals.var_t3_dn0 = assign42510_e57105_d_n0;
        locals.var_t3_dn2 = assign42510_e57105_d_n2;
        locals.var_t3_dn4 = assign42510_e57105_d_n4;
        locals.var_t3_dn5 = assign42510_e57105_d_n5;
        locals.var_t3_dn6 = assign42510_e57105_d_n6;
        locals.var_t3_dn7 = assign42510_e57105_d_n7;
        locals.var_t3_dn8 = assign42510_e57105_d_n8;
        locals.var_t3_dn9 = assign42510_e57105_d_n9;
        locals.var_t3_dn10 = assign42510_e57105_d_n10;
        locals.var_t3_dn13 = assign42510_e57105_d_n13;
        locals.var_t3_rv = 0.0;

        let assign42520_e57109: f64 = (10.0 * 2.220446049250313e-16);
        let assign42520_e57110: f64 = (2.0 - assign42520_e57109);
        let assign42520_e57117: f64 = (10.0 * 2.220446049250313e-16);
        let assign42520_e57118: f64 = (2.0 + assign42520_e57117);
        let assign42520_e57120: f64 = if ((assign42520_e57110 <= p.p178) && (p.p178 <= assign42520_e57118)) { 1.0 } else { 0.0 };
        locals.var_guard1054 = assign42520_e57120;
        locals.var_guard1054_rv = 0.0;

        let (assign42530_e57134, assign42530_e57134_d_n0, assign42530_e57134_d_n2, assign42530_e57134_d_n4, assign42530_e57134_d_n5, assign42530_e57134_d_n6, assign42530_e57134_d_n7, assign42530_e57134_d_n8, assign42530_e57134_d_n9, assign42530_e57134_d_n10, assign42530_e57134_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1053 == 0.0)) && (locals.var_guard1054 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign42530_e57134;
        locals.var_t3_dn0 = assign42530_e57134_d_n0;
        locals.var_t3_dn2 = assign42530_e57134_d_n2;
        locals.var_t3_dn4 = assign42530_e57134_d_n4;
        locals.var_t3_dn5 = assign42530_e57134_d_n5;
        locals.var_t3_dn6 = assign42530_e57134_d_n6;
        locals.var_t3_dn7 = assign42530_e57134_d_n7;
        locals.var_t3_dn8 = assign42530_e57134_d_n8;
        locals.var_t3_dn9 = assign42530_e57134_d_n9;
        locals.var_t3_dn10 = assign42530_e57134_d_n10;
        locals.var_t3_dn13 = assign42530_e57134_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign42540_e57158, assign42540_e57158_d_n0, assign42540_e57158_d_n2, assign42540_e57158_d_n4, assign42540_e57158_d_n5, assign42540_e57158_d_n6, assign42540_e57158_d_n7, assign42540_e57158_d_n8, assign42540_e57158_d_n9, assign42540_e57158_d_n10, assign42540_e57158_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1053 == 0.0)) && (locals.var_guard1054 == 0.0)) {
        let (assign42540_e57156, assign42540_e57156_d_n0, assign42540_e57156_d_n2, assign42540_e57156_d_n4, assign42540_e57156_d_n5, assign42540_e57156_d_n6, assign42540_e57156_d_n7, assign42540_e57156_d_n8, assign42540_e57156_d_n9, assign42540_e57156_d_n10, assign42540_e57156_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42540_e57154: f64 = (p.p178 - 1.0);
                let assign42540_e57155: f64 = (locals.var_t1).powf(assign42540_e57154);
                (assign42540_e57155, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn0)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn2)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn4)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn5)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn6)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn7)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn8)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn9)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn10)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42540_e57154) as f64).is_finite() && ((assign42540_e57154) as f64).fract() == 0.0 { if assign42540_e57154 == 0.0 { 0.0 } else { (assign42540_e57154 * ((locals.var_t1).powf(assign42540_e57154 - 1.0) * locals.var_t1_dn13)) } } else { (assign42540_e57155 * (assign42540_e57154 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign42540_e57156, assign42540_e57156_d_n0, assign42540_e57156_d_n2, assign42540_e57156_d_n4, assign42540_e57156_d_n5, assign42540_e57156_d_n6, assign42540_e57156_d_n7, assign42540_e57156_d_n8, assign42540_e57156_d_n9, assign42540_e57156_d_n10, assign42540_e57156_d_n13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign42540_e57158;
        locals.var_t3_dn0 = assign42540_e57158_d_n0;
        locals.var_t3_dn2 = assign42540_e57158_d_n2;
        locals.var_t3_dn4 = assign42540_e57158_d_n4;
        locals.var_t3_dn5 = assign42540_e57158_d_n5;
        locals.var_t3_dn6 = assign42540_e57158_d_n6;
        locals.var_t3_dn7 = assign42540_e57158_d_n7;
        locals.var_t3_dn8 = assign42540_e57158_d_n8;
        locals.var_t3_dn9 = assign42540_e57158_d_n9;
        locals.var_t3_dn10 = assign42540_e57158_d_n10;
        locals.var_t3_dn13 = assign42540_e57158_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign42550_e57169, assign42550_e57169_d_n0, assign42550_e57169_d_n2, assign42550_e57169_d_n4, assign42550_e57169_d_n5, assign42550_e57169_d_n6, assign42550_e57169_d_n7, assign42550_e57169_d_n8, assign42550_e57169_d_n9, assign42550_e57169_d_n10, assign42550_e57169_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42550_e57167: f64 = (locals.var_t1 * locals.var_t3);
        (assign42550_e57167, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign42550_e57169;
        locals.var_t2_dn0 = assign42550_e57169_d_n0;
        locals.var_t2_dn2 = assign42550_e57169_d_n2;
        locals.var_t2_dn4 = assign42550_e57169_d_n4;
        locals.var_t2_dn5 = assign42550_e57169_d_n5;
        locals.var_t2_dn6 = assign42550_e57169_d_n6;
        locals.var_t2_dn7 = assign42550_e57169_d_n7;
        locals.var_t2_dn8 = assign42550_e57169_d_n8;
        locals.var_t2_dn9 = assign42550_e57169_d_n9;
        locals.var_t2_dn10 = assign42550_e57169_d_n10;
        locals.var_t2_dn13 = assign42550_e57169_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign42560_e57180, assign42560_e57180_d_n0, assign42560_e57180_d_n2, assign42560_e57180_d_n4, assign42560_e57180_d_n5, assign42560_e57180_d_n6, assign42560_e57180_d_n7, assign42560_e57180_d_n8, assign42560_e57180_d_n9, assign42560_e57180_d_n10, assign42560_e57180_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42560_e57178: f64 = (1.0 + locals.var_t2);
        (assign42560_e57178, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign42560_e57180;
        locals.var_t4_dn0 = assign42560_e57180_d_n0;
        locals.var_t4_dn2 = assign42560_e57180_d_n2;
        locals.var_t4_dn4 = assign42560_e57180_d_n4;
        locals.var_t4_dn5 = assign42560_e57180_d_n5;
        locals.var_t4_dn6 = assign42560_e57180_d_n6;
        locals.var_t4_dn7 = assign42560_e57180_d_n7;
        locals.var_t4_dn8 = assign42560_e57180_d_n8;
        locals.var_t4_dn9 = assign42560_e57180_d_n9;
        locals.var_t4_dn10 = assign42560_e57180_d_n10;
        locals.var_t4_dn13 = assign42560_e57180_d_n13;
        locals.var_t4_rv = 0.0;

        let assign42570_e57184: f64 = (10.0 * 2.220446049250313e-16);
        let assign42570_e57185: f64 = (1.0 - assign42570_e57184);
        let assign42570_e57192: f64 = (10.0 * 2.220446049250313e-16);
        let assign42570_e57193: f64 = (1.0 + assign42570_e57192);
        let assign42570_e57195: f64 = if ((assign42570_e57185 <= p.p178) && (p.p178 <= assign42570_e57193)) { 1.0 } else { 0.0 };
        locals.var_guard1055 = assign42570_e57195;
        locals.var_guard1055_rv = 0.0;

        let (assign42580_e57208, assign42580_e57208_d_n0, assign42580_e57208_d_n2, assign42580_e57208_d_n4, assign42580_e57208_d_n5, assign42580_e57208_d_n6, assign42580_e57208_d_n7, assign42580_e57208_d_n8, assign42580_e57208_d_n9, assign42580_e57208_d_n10, assign42580_e57208_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1055 != 0.0)) {
        let assign42580_e57206: f64 = (1.0 / locals.var_t4);
        (assign42580_e57206, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign42580_e57208;
        locals.var_t5_dn0 = assign42580_e57208_d_n0;
        locals.var_t5_dn2 = assign42580_e57208_d_n2;
        locals.var_t5_dn4 = assign42580_e57208_d_n4;
        locals.var_t5_dn5 = assign42580_e57208_d_n5;
        locals.var_t5_dn6 = assign42580_e57208_d_n6;
        locals.var_t5_dn7 = assign42580_e57208_d_n7;
        locals.var_t5_dn8 = assign42580_e57208_d_n8;
        locals.var_t5_dn9 = assign42580_e57208_d_n9;
        locals.var_t5_dn10 = assign42580_e57208_d_n10;
        locals.var_t5_dn13 = assign42580_e57208_d_n13;
        locals.var_t5_rv = 0.0;

        let assign42590_e57212: f64 = (10.0 * 2.220446049250313e-16);
        let assign42590_e57213: f64 = (2.0 - assign42590_e57212);
        let assign42590_e57220: f64 = (10.0 * 2.220446049250313e-16);
        let assign42590_e57221: f64 = (2.0 + assign42590_e57220);
        let assign42590_e57223: f64 = if ((assign42590_e57213 <= p.p178) && (p.p178 <= assign42590_e57221)) { 1.0 } else { 0.0 };
        locals.var_guard1056 = assign42590_e57223;
        locals.var_guard1056_rv = 0.0;

        let (assign42600_e57240, assign42600_e57240_d_n0, assign42600_e57240_d_n2, assign42600_e57240_d_n4, assign42600_e57240_d_n5, assign42600_e57240_d_n6, assign42600_e57240_d_n7, assign42600_e57240_d_n8, assign42600_e57240_d_n9, assign42600_e57240_d_n10, assign42600_e57240_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1055 == 0.0)) && (locals.var_guard1056 != 0.0)) {
        let assign42600_e57237: f64 = (locals.var_t4).sqrt();
        let assign42600_e57238: f64 = (1.0 / assign42600_e57237);
        (assign42600_e57238, (-((locals.var_t4_dn0 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn2 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn4 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn5 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn6 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn7 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn8 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn9 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn10 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))), (-((locals.var_t4_dn13 / (2.0 * assign42600_e57237)) / (assign42600_e57237 * assign42600_e57237))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign42600_e57240;
        locals.var_t5_dn0 = assign42600_e57240_d_n0;
        locals.var_t5_dn2 = assign42600_e57240_d_n2;
        locals.var_t5_dn4 = assign42600_e57240_d_n4;
        locals.var_t5_dn5 = assign42600_e57240_d_n5;
        locals.var_t5_dn6 = assign42600_e57240_d_n6;
        locals.var_t5_dn7 = assign42600_e57240_d_n7;
        locals.var_t5_dn8 = assign42600_e57240_d_n8;
        locals.var_t5_dn9 = assign42600_e57240_d_n9;
        locals.var_t5_dn10 = assign42600_e57240_d_n10;
        locals.var_t5_dn13 = assign42600_e57240_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign42610_e57267, assign42610_e57267_d_n0, assign42610_e57267_d_n2, assign42610_e57267_d_n4, assign42610_e57267_d_n5, assign42610_e57267_d_n6, assign42610_e57267_d_n7, assign42610_e57267_d_n8, assign42610_e57267_d_n9, assign42610_e57267_d_n10, assign42610_e57267_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1055 == 0.0)) && (locals.var_guard1056 == 0.0)) {
        let (assign42610_e57265, assign42610_e57265_d_n0, assign42610_e57265_d_n2, assign42610_e57265_d_n4, assign42610_e57265_d_n5, assign42610_e57265_d_n6, assign42610_e57265_d_n7, assign42610_e57265_d_n8, assign42610_e57265_d_n9, assign42610_e57265_d_n10, assign42610_e57265_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42610_e57259: f64 = (-1.0);
                let assign42610_e57261: f64 = (assign42610_e57259 / p.p178);
                let assign42610_e57263: f64 = (assign42610_e57261 - 1.0);
                let assign42610_e57264: f64 = (locals.var_t4).powf(assign42610_e57263);
                (assign42610_e57264, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn0)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn2)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn4)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn5)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn6)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn7)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn8)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn9)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn10)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42610_e57263) as f64).is_finite() && ((assign42610_e57263) as f64).fract() == 0.0 { if assign42610_e57263 == 0.0 { 0.0 } else { (assign42610_e57263 * ((locals.var_t4).powf(assign42610_e57263 - 1.0) * locals.var_t4_dn13)) } } else { (assign42610_e57264 * (assign42610_e57263 * (locals.var_t4_dn13 / locals.var_t4))) },)
            }
        };
        (assign42610_e57265, assign42610_e57265_d_n0, assign42610_e57265_d_n2, assign42610_e57265_d_n4, assign42610_e57265_d_n5, assign42610_e57265_d_n6, assign42610_e57265_d_n7, assign42610_e57265_d_n8, assign42610_e57265_d_n9, assign42610_e57265_d_n10, assign42610_e57265_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign42610_e57267;
        locals.var_t6_dn0 = assign42610_e57267_d_n0;
        locals.var_t6_dn2 = assign42610_e57267_d_n2;
        locals.var_t6_dn4 = assign42610_e57267_d_n4;
        locals.var_t6_dn5 = assign42610_e57267_d_n5;
        locals.var_t6_dn6 = assign42610_e57267_d_n6;
        locals.var_t6_dn7 = assign42610_e57267_d_n7;
        locals.var_t6_dn8 = assign42610_e57267_d_n8;
        locals.var_t6_dn9 = assign42610_e57267_d_n9;
        locals.var_t6_dn10 = assign42610_e57267_d_n10;
        locals.var_t6_dn13 = assign42610_e57267_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign42620_e57284, assign42620_e57284_d_n0, assign42620_e57284_d_n2, assign42620_e57284_d_n4, assign42620_e57284_d_n5, assign42620_e57284_d_n6, assign42620_e57284_d_n7, assign42620_e57284_d_n8, assign42620_e57284_d_n9, assign42620_e57284_d_n10, assign42620_e57284_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1055 == 0.0)) && (locals.var_guard1056 == 0.0)) {
        let assign42620_e57282: f64 = (locals.var_t4 * locals.var_t6);
        (assign42620_e57282, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn13 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign42620_e57284;
        locals.var_t5_dn0 = assign42620_e57284_d_n0;
        locals.var_t5_dn2 = assign42620_e57284_d_n2;
        locals.var_t5_dn4 = assign42620_e57284_d_n4;
        locals.var_t5_dn5 = assign42620_e57284_d_n5;
        locals.var_t5_dn6 = assign42620_e57284_d_n6;
        locals.var_t5_dn7 = assign42620_e57284_d_n7;
        locals.var_t5_dn8 = assign42620_e57284_d_n8;
        locals.var_t5_dn9 = assign42620_e57284_d_n9;
        locals.var_t5_dn10 = assign42620_e57284_d_n10;
        locals.var_t5_dn13 = assign42620_e57284_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign42630_e57295, assign42630_e57295_d_n0, assign42630_e57295_d_n2, assign42630_e57295_d_n4, assign42630_e57295_d_n5, assign42630_e57295_d_n6, assign42630_e57295_d_n7, assign42630_e57295_d_n8, assign42630_e57295_d_n9, assign42630_e57295_d_n10, assign42630_e57295_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign42630_e57293: f64 = (locals.var_muun * locals.var_t5);
        (assign42630_e57293, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn13 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn13)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn13,)
    }
};
        locals.var_mu = assign42630_e57295;
        locals.var_mu_dn0 = assign42630_e57295_d_n0;
        locals.var_mu_dn2 = assign42630_e57295_d_n2;
        locals.var_mu_dn4 = assign42630_e57295_d_n4;
        locals.var_mu_dn5 = assign42630_e57295_d_n5;
        locals.var_mu_dn6 = assign42630_e57295_d_n6;
        locals.var_mu_dn7 = assign42630_e57295_d_n7;
        locals.var_mu_dn8 = assign42630_e57295_d_n8;
        locals.var_mu_dn9 = assign42630_e57295_d_n9;
        locals.var_mu_dn10 = assign42630_e57295_d_n10;
        locals.var_mu_dn13 = assign42630_e57295_d_n13;
        locals.var_mu_rv = 0.0;

        let (assign42640_e57304, assign42640_e57304_d_n0, assign42640_e57304_d_n2, assign42640_e57304_d_n4, assign42640_e57304_d_n5, assign42640_e57304_d_n6, assign42640_e57304_d_n7, assign42640_e57304_d_n8, assign42640_e57304_d_n9, assign42640_e57304_d_n10, assign42640_e57304_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn13,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn13,)
    }
};
        locals.var_mu_acc = assign42640_e57304;
        locals.var_mu_acc_dn0 = assign42640_e57304_d_n0;
        locals.var_mu_acc_dn2 = assign42640_e57304_d_n2;
        locals.var_mu_acc_dn4 = assign42640_e57304_d_n4;
        locals.var_mu_acc_dn5 = assign42640_e57304_d_n5;
        locals.var_mu_acc_dn6 = assign42640_e57304_d_n6;
        locals.var_mu_acc_dn7 = assign42640_e57304_d_n7;
        locals.var_mu_acc_dn8 = assign42640_e57304_d_n8;
        locals.var_mu_acc_dn9 = assign42640_e57304_d_n9;
        locals.var_mu_acc_dn10 = assign42640_e57304_d_n10;
        locals.var_mu_acc_dn13 = assign42640_e57304_d_n13;
        locals.var_mu_acc_rv = 0.0;

        let (assign42650_e57313, assign42650_e57313_d_n0, assign42650_e57313_d_n2, assign42650_e57313_d_n4, assign42650_e57313_d_n5, assign42650_e57313_d_n6, assign42650_e57313_d_n7, assign42650_e57313_d_n8, assign42650_e57313_d_n9, assign42650_e57313_d_n10, assign42650_e57313_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    } else {
        (locals.var_ey_acc, locals.var_ey_acc_dn0, locals.var_ey_acc_dn2, locals.var_ey_acc_dn4, locals.var_ey_acc_dn5, locals.var_ey_acc_dn6, locals.var_ey_acc_dn7, locals.var_ey_acc_dn8, locals.var_ey_acc_dn9, locals.var_ey_acc_dn10, locals.var_ey_acc_dn13,)
    }
};
        locals.var_ey_acc = assign42650_e57313;
        locals.var_ey_acc_dn0 = assign42650_e57313_d_n0;
        locals.var_ey_acc_dn2 = assign42650_e57313_d_n2;
        locals.var_ey_acc_dn4 = assign42650_e57313_d_n4;
        locals.var_ey_acc_dn5 = assign42650_e57313_d_n5;
        locals.var_ey_acc_dn6 = assign42650_e57313_d_n6;
        locals.var_ey_acc_dn7 = assign42650_e57313_d_n7;
        locals.var_ey_acc_dn8 = assign42650_e57313_d_n8;
        locals.var_ey_acc_dn9 = assign42650_e57313_d_n9;
        locals.var_ey_acc_dn10 = assign42650_e57313_d_n10;
        locals.var_ey_acc_dn13 = assign42650_e57313_d_n13;
        locals.var_ey_acc_rv = 0.0;

        let (assign42660_e57322, assign42660_e57322_d_n0, assign42660_e57322_d_n2, assign42660_e57322_d_n4, assign42660_e57322_d_n5, assign42660_e57322_d_n6, assign42660_e57322_d_n7, assign42660_e57322_d_n8, assign42660_e57322_d_n9, assign42660_e57322_d_n10, assign42660_e57322_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn13,)
    }
};
        locals.var_vds_res = assign42660_e57322;
        locals.var_vds_res_dn0 = assign42660_e57322_d_n0;
        locals.var_vds_res_dn2 = assign42660_e57322_d_n2;
        locals.var_vds_res_dn4 = assign42660_e57322_d_n4;
        locals.var_vds_res_dn5 = assign42660_e57322_d_n5;
        locals.var_vds_res_dn6 = assign42660_e57322_d_n6;
        locals.var_vds_res_dn7 = assign42660_e57322_d_n7;
        locals.var_vds_res_dn8 = assign42660_e57322_d_n8;
        locals.var_vds_res_dn9 = assign42660_e57322_d_n9;
        locals.var_vds_res_dn10 = assign42660_e57322_d_n10;
        locals.var_vds_res_dn13 = assign42660_e57322_d_n13;
        locals.var_vds_res_rv = 0.0;

        let assign42670_e57325: f64 = if locals.var_vdsorg > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1057 = assign42670_e57325;
        locals.var_guard1057_rv = 0.0;

        let (assign42680_e57340, assign42680_e57340_d_n0, assign42680_e57340_d_n2, assign42680_e57340_d_n4, assign42680_e57340_d_n5, assign42680_e57340_d_n6, assign42680_e57340_d_n7, assign42680_e57340_d_n8, assign42680_e57340_d_n9, assign42680_e57340_d_n10, assign42680_e57340_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign42680_e57336: f64 = (locals.var_vbsc + locals.var_beta_inv);
        let assign42680_e57338: f64 = (assign42680_e57336 * p.p396);
        (assign42680_e57338, ((locals.var_vbsc_dn0 + locals.var_beta_inv_dn0) * p.p396), ((locals.var_vbsc_dn2 + locals.var_beta_inv_dn2) * p.p396), ((locals.var_vbsc_dn4 + locals.var_beta_inv_dn4) * p.p396), ((locals.var_vbsc_dn5 + locals.var_beta_inv_dn5) * p.p396), ((locals.var_vbsc_dn6 + locals.var_beta_inv_dn6) * p.p396), ((locals.var_vbsc_dn7 + locals.var_beta_inv_dn7) * p.p396), ((locals.var_vbsc_dn8 + locals.var_beta_inv_dn8) * p.p396), ((locals.var_vbsc_dn9 + locals.var_beta_inv_dn9) * p.p396), ((locals.var_vbsc_dn10 + locals.var_beta_inv_dn10) * p.p396), ((locals.var_vbsc_dn13 + locals.var_beta_inv_dn13) * p.p396),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign42680_e57340;
        locals.var_t10_dn0 = assign42680_e57340_d_n0;
        locals.var_t10_dn2 = assign42680_e57340_d_n2;
        locals.var_t10_dn4 = assign42680_e57340_d_n4;
        locals.var_t10_dn5 = assign42680_e57340_d_n5;
        locals.var_t10_dn6 = assign42680_e57340_d_n6;
        locals.var_t10_dn7 = assign42680_e57340_d_n7;
        locals.var_t10_dn8 = assign42680_e57340_d_n8;
        locals.var_t10_dn9 = assign42680_e57340_d_n9;
        locals.var_t10_dn10 = assign42680_e57340_d_n10;
        locals.var_t10_dn13 = assign42680_e57340_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign42690_e57357, assign42690_e57357_d_n0, assign42690_e57357_d_n2, assign42690_e57357_d_n4, assign42690_e57357_d_n5, assign42690_e57357_d_n6, assign42690_e57357_d_n7, assign42690_e57357_d_n8, assign42690_e57357_d_n9, assign42690_e57357_d_n10, assign42690_e57357_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign42690_e57353: f64 = (locals.var_vgp - locals.var_t10);
        let assign42690_e57354: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2 * assign42690_e57353);
        let assign42690_e57355: f64 = (1.0 + assign42690_e57354);
        (assign42690_e57355, ((locals.var_c2_q_ndepm_esi_cox_inv2_dn0 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn2 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn4 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn4 - locals.var_t10_dn4))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn5 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn5 - locals.var_t10_dn5))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn6 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn7 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn8 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn8 - locals.var_t10_dn8))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn9 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn9 - locals.var_t10_dn9))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn10 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn13 * assign42690_e57353) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn13 - locals.var_t10_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign42690_e57357;
        locals.var_t4_dn0 = assign42690_e57357_d_n0;
        locals.var_t4_dn2 = assign42690_e57357_d_n2;
        locals.var_t4_dn4 = assign42690_e57357_d_n4;
        locals.var_t4_dn5 = assign42690_e57357_d_n5;
        locals.var_t4_dn6 = assign42690_e57357_d_n6;
        locals.var_t4_dn7 = assign42690_e57357_d_n7;
        locals.var_t4_dn8 = assign42690_e57357_d_n8;
        locals.var_t4_dn9 = assign42690_e57357_d_n9;
        locals.var_t4_dn10 = assign42690_e57357_d_n10;
        locals.var_t4_dn13 = assign42690_e57357_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign42700_e57370, assign42700_e57370_d_n0, assign42700_e57370_d_n2, assign42700_e57370_d_n4, assign42700_e57370_d_n5, assign42700_e57370_d_n6, assign42700_e57370_d_n7, assign42700_e57370_d_n8, assign42700_e57370_d_n9, assign42700_e57370_d_n10, assign42700_e57370_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign42700_e57368: f64 = (1.0 + locals.var_c2_q_ndepm_esi_cox_inv2);
        (assign42700_e57368, locals.var_c2_q_ndepm_esi_cox_inv2_dn0, locals.var_c2_q_ndepm_esi_cox_inv2_dn2, locals.var_c2_q_ndepm_esi_cox_inv2_dn4, locals.var_c2_q_ndepm_esi_cox_inv2_dn5, locals.var_c2_q_ndepm_esi_cox_inv2_dn6, locals.var_c2_q_ndepm_esi_cox_inv2_dn7, locals.var_c2_q_ndepm_esi_cox_inv2_dn8, locals.var_c2_q_ndepm_esi_cox_inv2_dn9, locals.var_c2_q_ndepm_esi_cox_inv2_dn10, locals.var_c2_q_ndepm_esi_cox_inv2_dn13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign42700_e57370;
        locals.var_t5_dn0 = assign42700_e57370_d_n0;
        locals.var_t5_dn2 = assign42700_e57370_d_n2;
        locals.var_t5_dn4 = assign42700_e57370_d_n4;
        locals.var_t5_dn5 = assign42700_e57370_d_n5;
        locals.var_t5_dn6 = assign42700_e57370_d_n6;
        locals.var_t5_dn7 = assign42700_e57370_d_n7;
        locals.var_t5_dn8 = assign42700_e57370_d_n8;
        locals.var_t5_dn9 = assign42700_e57370_d_n9;
        locals.var_t5_dn10 = assign42700_e57370_d_n10;
        locals.var_t5_dn13 = assign42700_e57370_d_n13;
        locals.var_t5_rv = 0.0;

        let assign42710_e57374: f64 = locals.var_t5;
        let assign42710_e57379: f64 = if ((locals.var_t4 < assign42710_e57374) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1058 = assign42710_e57379;
        locals.var_guard1058_rv = 0.0;

        let (assign42720_e57396, assign42720_e57396_d_n0, assign42720_e57396_d_n2, assign42720_e57396_d_n4, assign42720_e57396_d_n5, assign42720_e57396_d_n6, assign42720_e57396_d_n7, assign42720_e57396_d_n8, assign42720_e57396_d_n9, assign42720_e57396_d_n10, assign42720_e57396_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42720_e57392: f64 = locals.var_t5;
        let assign42720_e57394: f64 = (assign42720_e57392 - locals.var_t4);
        (assign42720_e57394, (locals.var_t5_dn0 - locals.var_t4_dn0), (locals.var_t5_dn2 - locals.var_t4_dn2), (locals.var_t5_dn4 - locals.var_t4_dn4), (locals.var_t5_dn5 - locals.var_t4_dn5), (locals.var_t5_dn6 - locals.var_t4_dn6), (locals.var_t5_dn7 - locals.var_t4_dn7), (locals.var_t5_dn8 - locals.var_t4_dn8), (locals.var_t5_dn9 - locals.var_t4_dn9), (locals.var_t5_dn10 - locals.var_t4_dn10), (locals.var_t5_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign42720_e57396;
        locals.var_tmf1_dn0 = assign42720_e57396_d_n0;
        locals.var_tmf1_dn2 = assign42720_e57396_d_n2;
        locals.var_tmf1_dn4 = assign42720_e57396_d_n4;
        locals.var_tmf1_dn5 = assign42720_e57396_d_n5;
        locals.var_tmf1_dn6 = assign42720_e57396_d_n6;
        locals.var_tmf1_dn7 = assign42720_e57396_d_n7;
        locals.var_tmf1_dn8 = assign42720_e57396_d_n8;
        locals.var_tmf1_dn9 = assign42720_e57396_d_n9;
        locals.var_tmf1_dn10 = assign42720_e57396_d_n10;
        locals.var_tmf1_dn13 = assign42720_e57396_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign42730_e57411, assign42730_e57411_d_n0, assign42730_e57411_d_n2, assign42730_e57411_d_n4, assign42730_e57411_d_n5, assign42730_e57411_d_n6, assign42730_e57411_d_n7, assign42730_e57411_d_n8, assign42730_e57411_d_n9, assign42730_e57411_d_n10, assign42730_e57411_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42730_e57409: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign42730_e57409, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign42730_e57411;
        locals.var_x2_dn0 = assign42730_e57411_d_n0;
        locals.var_x2_dn2 = assign42730_e57411_d_n2;
        locals.var_x2_dn4 = assign42730_e57411_d_n4;
        locals.var_x2_dn5 = assign42730_e57411_d_n5;
        locals.var_x2_dn6 = assign42730_e57411_d_n6;
        locals.var_x2_dn7 = assign42730_e57411_d_n7;
        locals.var_x2_dn8 = assign42730_e57411_d_n8;
        locals.var_x2_dn9 = assign42730_e57411_d_n9;
        locals.var_x2_dn10 = assign42730_e57411_d_n10;
        locals.var_x2_dn13 = assign42730_e57411_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign42740_e57426, assign42740_e57426_d_n0, assign42740_e57426_d_n2, assign42740_e57426_d_n4, assign42740_e57426_d_n5, assign42740_e57426_d_n6, assign42740_e57426_d_n7, assign42740_e57426_d_n8, assign42740_e57426_d_n9, assign42740_e57426_d_n10, assign42740_e57426_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42740_e57424: f64 = (locals.var_t5 * locals.var_t5);
        (assign42740_e57424, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign42740_e57426;
        locals.var_xmax2_dn0 = assign42740_e57426_d_n0;
        locals.var_xmax2_dn2 = assign42740_e57426_d_n2;
        locals.var_xmax2_dn4 = assign42740_e57426_d_n4;
        locals.var_xmax2_dn5 = assign42740_e57426_d_n5;
        locals.var_xmax2_dn6 = assign42740_e57426_d_n6;
        locals.var_xmax2_dn7 = assign42740_e57426_d_n7;
        locals.var_xmax2_dn8 = assign42740_e57426_d_n8;
        locals.var_xmax2_dn9 = assign42740_e57426_d_n9;
        locals.var_xmax2_dn10 = assign42740_e57426_d_n10;
        locals.var_xmax2_dn13 = assign42740_e57426_d_n13;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_143(
        locals: &mut StampLocals,
    ) {
        let (assign42750_e57439, assign42750_e57439_d_n0, assign42750_e57439_d_n2, assign42750_e57439_d_n4, assign42750_e57439_d_n5, assign42750_e57439_d_n6, assign42750_e57439_d_n7, assign42750_e57439_d_n8, assign42750_e57439_d_n9, assign42750_e57439_d_n10, assign42750_e57439_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign42750_e57439;
        locals.var_xp_dn0 = assign42750_e57439_d_n0;
        locals.var_xp_dn2 = assign42750_e57439_d_n2;
        locals.var_xp_dn4 = assign42750_e57439_d_n4;
        locals.var_xp_dn5 = assign42750_e57439_d_n5;
        locals.var_xp_dn6 = assign42750_e57439_d_n6;
        locals.var_xp_dn7 = assign42750_e57439_d_n7;
        locals.var_xp_dn8 = assign42750_e57439_d_n8;
        locals.var_xp_dn9 = assign42750_e57439_d_n9;
        locals.var_xp_dn10 = assign42750_e57439_d_n10;
        locals.var_xp_dn13 = assign42750_e57439_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign42760_e57452, assign42760_e57452_d_n0, assign42760_e57452_d_n2, assign42760_e57452_d_n4, assign42760_e57452_d_n5, assign42760_e57452_d_n6, assign42760_e57452_d_n7, assign42760_e57452_d_n8, assign42760_e57452_d_n9, assign42760_e57452_d_n10, assign42760_e57452_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign42760_e57452;
        locals.var_xmp_dn0 = assign42760_e57452_d_n0;
        locals.var_xmp_dn2 = assign42760_e57452_d_n2;
        locals.var_xmp_dn4 = assign42760_e57452_d_n4;
        locals.var_xmp_dn5 = assign42760_e57452_d_n5;
        locals.var_xmp_dn6 = assign42760_e57452_d_n6;
        locals.var_xmp_dn7 = assign42760_e57452_d_n7;
        locals.var_xmp_dn8 = assign42760_e57452_d_n8;
        locals.var_xmp_dn9 = assign42760_e57452_d_n9;
        locals.var_xmp_dn10 = assign42760_e57452_d_n10;
        locals.var_xmp_dn13 = assign42760_e57452_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign42770_e57465,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign42770_e57465;
        locals.var_m0_rv = 0.0;

        let (assign42780_e57478,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42780_e57478;
        locals.var_mm_rv = 0.0;

        let (assign42790_e57491, assign42790_e57491_d_n0, assign42790_e57491_d_n2, assign42790_e57491_d_n4, assign42790_e57491_d_n5, assign42790_e57491_d_n6, assign42790_e57491_d_n7, assign42790_e57491_d_n8, assign42790_e57491_d_n9, assign42790_e57491_d_n10, assign42790_e57491_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign42790_e57491;
        locals.var_arg_dn0 = assign42790_e57491_d_n0;
        locals.var_arg_dn2 = assign42790_e57491_d_n2;
        locals.var_arg_dn4 = assign42790_e57491_d_n4;
        locals.var_arg_dn5 = assign42790_e57491_d_n5;
        locals.var_arg_dn6 = assign42790_e57491_d_n6;
        locals.var_arg_dn7 = assign42790_e57491_d_n7;
        locals.var_arg_dn8 = assign42790_e57491_d_n8;
        locals.var_arg_dn9 = assign42790_e57491_d_n9;
        locals.var_arg_dn10 = assign42790_e57491_d_n10;
        locals.var_arg_dn13 = assign42790_e57491_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign42800_e57504, assign42800_e57504_d_n0, assign42800_e57504_d_n2, assign42800_e57504_d_n4, assign42800_e57504_d_n5, assign42800_e57504_d_n6, assign42800_e57504_d_n7, assign42800_e57504_d_n8, assign42800_e57504_d_n9, assign42800_e57504_d_n10, assign42800_e57504_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign42800_e57504;
        locals.var_dnm_dn0 = assign42800_e57504_d_n0;
        locals.var_dnm_dn2 = assign42800_e57504_d_n2;
        locals.var_dnm_dn4 = assign42800_e57504_d_n4;
        locals.var_dnm_dn5 = assign42800_e57504_d_n5;
        locals.var_dnm_dn6 = assign42800_e57504_d_n6;
        locals.var_dnm_dn7 = assign42800_e57504_d_n7;
        locals.var_dnm_dn8 = assign42800_e57504_d_n8;
        locals.var_dnm_dn9 = assign42800_e57504_d_n9;
        locals.var_dnm_dn10 = assign42800_e57504_d_n10;
        locals.var_dnm_dn13 = assign42800_e57504_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign42810_e57519, assign42810_e57519_d_n0, assign42810_e57519_d_n2, assign42810_e57519_d_n4, assign42810_e57519_d_n5, assign42810_e57519_d_n6, assign42810_e57519_d_n7, assign42810_e57519_d_n8, assign42810_e57519_d_n9, assign42810_e57519_d_n10, assign42810_e57519_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42810_e57517: f64 = (locals.var_xp * locals.var_x2);
        (assign42810_e57517, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign42810_e57519;
        locals.var_xp_dn0 = assign42810_e57519_d_n0;
        locals.var_xp_dn2 = assign42810_e57519_d_n2;
        locals.var_xp_dn4 = assign42810_e57519_d_n4;
        locals.var_xp_dn5 = assign42810_e57519_d_n5;
        locals.var_xp_dn6 = assign42810_e57519_d_n6;
        locals.var_xp_dn7 = assign42810_e57519_d_n7;
        locals.var_xp_dn8 = assign42810_e57519_d_n8;
        locals.var_xp_dn9 = assign42810_e57519_d_n9;
        locals.var_xp_dn10 = assign42810_e57519_d_n10;
        locals.var_xp_dn13 = assign42810_e57519_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign42820_e57534, assign42820_e57534_d_n0, assign42820_e57534_d_n2, assign42820_e57534_d_n4, assign42820_e57534_d_n5, assign42820_e57534_d_n6, assign42820_e57534_d_n7, assign42820_e57534_d_n8, assign42820_e57534_d_n9, assign42820_e57534_d_n10, assign42820_e57534_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42820_e57532: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign42820_e57532, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign42820_e57534;
        locals.var_xmp_dn0 = assign42820_e57534_d_n0;
        locals.var_xmp_dn2 = assign42820_e57534_d_n2;
        locals.var_xmp_dn4 = assign42820_e57534_d_n4;
        locals.var_xmp_dn5 = assign42820_e57534_d_n5;
        locals.var_xmp_dn6 = assign42820_e57534_d_n6;
        locals.var_xmp_dn7 = assign42820_e57534_d_n7;
        locals.var_xmp_dn8 = assign42820_e57534_d_n8;
        locals.var_xmp_dn9 = assign42820_e57534_d_n9;
        locals.var_xmp_dn10 = assign42820_e57534_d_n10;
        locals.var_xmp_dn13 = assign42820_e57534_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign42830_e57549, assign42830_e57549_d_n0, assign42830_e57549_d_n2, assign42830_e57549_d_n4, assign42830_e57549_d_n5, assign42830_e57549_d_n6, assign42830_e57549_d_n7, assign42830_e57549_d_n8, assign42830_e57549_d_n9, assign42830_e57549_d_n10, assign42830_e57549_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42830_e57547: f64 = (locals.var_xp * locals.var_x2);
        (assign42830_e57547, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign42830_e57549;
        locals.var_xp_dn0 = assign42830_e57549_d_n0;
        locals.var_xp_dn2 = assign42830_e57549_d_n2;
        locals.var_xp_dn4 = assign42830_e57549_d_n4;
        locals.var_xp_dn5 = assign42830_e57549_d_n5;
        locals.var_xp_dn6 = assign42830_e57549_d_n6;
        locals.var_xp_dn7 = assign42830_e57549_d_n7;
        locals.var_xp_dn8 = assign42830_e57549_d_n8;
        locals.var_xp_dn9 = assign42830_e57549_d_n9;
        locals.var_xp_dn10 = assign42830_e57549_d_n10;
        locals.var_xp_dn13 = assign42830_e57549_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign42840_e57564, assign42840_e57564_d_n0, assign42840_e57564_d_n2, assign42840_e57564_d_n4, assign42840_e57564_d_n5, assign42840_e57564_d_n6, assign42840_e57564_d_n7, assign42840_e57564_d_n8, assign42840_e57564_d_n9, assign42840_e57564_d_n10, assign42840_e57564_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42840_e57562: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign42840_e57562, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign42840_e57564;
        locals.var_xmp_dn0 = assign42840_e57564_d_n0;
        locals.var_xmp_dn2 = assign42840_e57564_d_n2;
        locals.var_xmp_dn4 = assign42840_e57564_d_n4;
        locals.var_xmp_dn5 = assign42840_e57564_d_n5;
        locals.var_xmp_dn6 = assign42840_e57564_d_n6;
        locals.var_xmp_dn7 = assign42840_e57564_d_n7;
        locals.var_xmp_dn8 = assign42840_e57564_d_n8;
        locals.var_xmp_dn9 = assign42840_e57564_d_n9;
        locals.var_xmp_dn10 = assign42840_e57564_d_n10;
        locals.var_xmp_dn13 = assign42840_e57564_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign42850_e57579, assign42850_e57579_d_n0, assign42850_e57579_d_n2, assign42850_e57579_d_n4, assign42850_e57579_d_n5, assign42850_e57579_d_n6, assign42850_e57579_d_n7, assign42850_e57579_d_n8, assign42850_e57579_d_n9, assign42850_e57579_d_n10, assign42850_e57579_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42850_e57577: f64 = (locals.var_xp + locals.var_xmp);
        (assign42850_e57577, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign42850_e57579;
        locals.var_arg_dn0 = assign42850_e57579_d_n0;
        locals.var_arg_dn2 = assign42850_e57579_d_n2;
        locals.var_arg_dn4 = assign42850_e57579_d_n4;
        locals.var_arg_dn5 = assign42850_e57579_d_n5;
        locals.var_arg_dn6 = assign42850_e57579_d_n6;
        locals.var_arg_dn7 = assign42850_e57579_d_n7;
        locals.var_arg_dn8 = assign42850_e57579_d_n8;
        locals.var_arg_dn9 = assign42850_e57579_d_n9;
        locals.var_arg_dn10 = assign42850_e57579_d_n10;
        locals.var_arg_dn13 = assign42850_e57579_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign42860_e57592, assign42860_e57592_d_n0, assign42860_e57592_d_n2, assign42860_e57592_d_n4, assign42860_e57592_d_n5, assign42860_e57592_d_n6, assign42860_e57592_d_n7, assign42860_e57592_d_n8, assign42860_e57592_d_n9, assign42860_e57592_d_n10, assign42860_e57592_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign42860_e57592;
        locals.var_dnm_dn0 = assign42860_e57592_d_n0;
        locals.var_dnm_dn2 = assign42860_e57592_d_n2;
        locals.var_dnm_dn4 = assign42860_e57592_d_n4;
        locals.var_dnm_dn5 = assign42860_e57592_d_n5;
        locals.var_dnm_dn6 = assign42860_e57592_d_n6;
        locals.var_dnm_dn7 = assign42860_e57592_d_n7;
        locals.var_dnm_dn8 = assign42860_e57592_d_n8;
        locals.var_dnm_dn9 = assign42860_e57592_d_n9;
        locals.var_dnm_dn10 = assign42860_e57592_d_n10;
        locals.var_dnm_dn13 = assign42860_e57592_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign42870_e57607: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign42870_e57607;
        locals.var_guard1059_rv = 0.0;

        let assign42880_e57610: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign42880_e57610;
        locals.var_guard1060_rv = 0.0;

        let (assign42890_e57627,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42890_e57627;
        locals.var_mm_rv = 0.0;

        let assign42900_e57630: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign42900_e57630;
        locals.var_guard1061_rv = 0.0;

        let (assign42910_e57650,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1061 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42910_e57650;
        locals.var_mm_rv = 0.0;

        let assign42920_e57653: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign42920_e57653;
        locals.var_guard1062_rv = 0.0;

        let (assign42930_e57676,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1061 == 0.0)) && (locals.var_guard1062 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42930_e57676;
        locals.var_mm_rv = 0.0;

        let assign42940_e57679: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign42940_e57679;
        locals.var_guard1063_rv = 0.0;

        let (assign42950_e57705,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1061 == 0.0)) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1063 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42950_e57705;
        locals.var_mm_rv = 0.0;

        let (assign42960_e57720,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign42960_e57720;
        locals.var_m0_rv = 0.0;

        let mut assign42970_loop_guard: usize = 0;
        while {
            let assign42970_cond_e57736: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign42970_cond_e57736 != 0.0
        } {
            assign42970_loop_guard += 1;
            assert!(assign42970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign42970_body0_e57752, assign42970_body0_e57752_d_n0, assign42970_body0_e57752_d_n2, assign42970_body0_e57752_d_n4, assign42970_body0_e57752_d_n5, assign42970_body0_e57752_d_n6, assign42970_body0_e57752_d_n7, assign42970_body0_e57752_d_n8, assign42970_body0_e57752_d_n9, assign42970_body0_e57752_d_n10, assign42970_body0_e57752_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) {
        let assign42970_body0_e57750: f64 = (locals.var_dnm).sqrt();
        (assign42970_body0_e57750, (locals.var_dnm_dn0 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn2 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn4 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn5 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn6 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn7 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn8 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn9 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn10 / (2.0 * assign42970_body0_e57750)), (locals.var_dnm_dn13 / (2.0 * assign42970_body0_e57750)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign42970_body0_e57752;
            locals.var_dnm_dn0 = assign42970_body0_e57752_d_n0;
            locals.var_dnm_dn2 = assign42970_body0_e57752_d_n2;
            locals.var_dnm_dn4 = assign42970_body0_e57752_d_n4;
            locals.var_dnm_dn5 = assign42970_body0_e57752_d_n5;
            locals.var_dnm_dn6 = assign42970_body0_e57752_d_n6;
            locals.var_dnm_dn7 = assign42970_body0_e57752_d_n7;
            locals.var_dnm_dn8 = assign42970_body0_e57752_d_n8;
            locals.var_dnm_dn9 = assign42970_body0_e57752_d_n9;
            locals.var_dnm_dn10 = assign42970_body0_e57752_d_n10;
            locals.var_dnm_dn13 = assign42970_body0_e57752_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign42970_body1_e57769,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 != 0.0)) {
        let assign42970_body1_e57767: f64 = (locals.var_m0 + 1.0);
        (assign42970_body1_e57767,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign42970_body1_e57769;
            locals.var_m0_rv = 0.0;
        }

        let (assign42980_e57796, assign42980_e57796_d_n0, assign42980_e57796_d_n2, assign42980_e57796_d_n4, assign42980_e57796_d_n5, assign42980_e57796_d_n6, assign42980_e57796_d_n7, assign42980_e57796_d_n8, assign42980_e57796_d_n9, assign42980_e57796_d_n10, assign42980_e57796_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) && (locals.var_guard1059 == 0.0)) {
        let (assign42980_e57794, assign42980_e57794_d_n0, assign42980_e57794_d_n2, assign42980_e57794_d_n4, assign42980_e57794_d_n5, assign42980_e57794_d_n6, assign42980_e57794_d_n7, assign42980_e57794_d_n8, assign42980_e57794_d_n9, assign42980_e57794_d_n10, assign42980_e57794_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42980_e57791: f64 = (2.0 * 2.0);
                let assign42980_e57792: f64 = (1.0 / assign42980_e57791);
                let assign42980_e57793: f64 = (locals.var_dnm).powf(assign42980_e57792);
                (assign42980_e57793, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn0)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn2)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn4)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn5)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn6)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn7)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn8)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn9)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn10)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign42980_e57792) as f64).is_finite() && ((assign42980_e57792) as f64).fract() == 0.0 { if assign42980_e57792 == 0.0 { 0.0 } else { (assign42980_e57792 * ((locals.var_dnm).powf(assign42980_e57792 - 1.0) * locals.var_dnm_dn13)) } } else { (assign42980_e57793 * (assign42980_e57792 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign42980_e57794, assign42980_e57794_d_n0, assign42980_e57794_d_n2, assign42980_e57794_d_n4, assign42980_e57794_d_n5, assign42980_e57794_d_n6, assign42980_e57794_d_n7, assign42980_e57794_d_n8, assign42980_e57794_d_n9, assign42980_e57794_d_n10, assign42980_e57794_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign42980_e57796;
        locals.var_dnm_dn0 = assign42980_e57796_d_n0;
        locals.var_dnm_dn2 = assign42980_e57796_d_n2;
        locals.var_dnm_dn4 = assign42980_e57796_d_n4;
        locals.var_dnm_dn5 = assign42980_e57796_d_n5;
        locals.var_dnm_dn6 = assign42980_e57796_d_n6;
        locals.var_dnm_dn7 = assign42980_e57796_d_n7;
        locals.var_dnm_dn8 = assign42980_e57796_d_n8;
        locals.var_dnm_dn9 = assign42980_e57796_d_n9;
        locals.var_dnm_dn10 = assign42980_e57796_d_n10;
        locals.var_dnm_dn13 = assign42980_e57796_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign42990_e57811, assign42990_e57811_d_n0, assign42990_e57811_d_n2, assign42990_e57811_d_n4, assign42990_e57811_d_n5, assign42990_e57811_d_n6, assign42990_e57811_d_n7, assign42990_e57811_d_n8, assign42990_e57811_d_n9, assign42990_e57811_d_n10, assign42990_e57811_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign42990_e57809: f64 = (1.0 / locals.var_dnm);
        (assign42990_e57809, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign42990_e57811;
        locals.var_dnm_dn0 = assign42990_e57811_d_n0;
        locals.var_dnm_dn2 = assign42990_e57811_d_n2;
        locals.var_dnm_dn4 = assign42990_e57811_d_n4;
        locals.var_dnm_dn5 = assign42990_e57811_d_n5;
        locals.var_dnm_dn6 = assign42990_e57811_d_n6;
        locals.var_dnm_dn7 = assign42990_e57811_d_n7;
        locals.var_dnm_dn8 = assign42990_e57811_d_n8;
        locals.var_dnm_dn9 = assign42990_e57811_d_n9;
        locals.var_dnm_dn10 = assign42990_e57811_d_n10;
        locals.var_dnm_dn13 = assign42990_e57811_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign43000_e57828, assign43000_e57828_d_n0, assign43000_e57828_d_n2, assign43000_e57828_d_n4, assign43000_e57828_d_n5, assign43000_e57828_d_n6, assign43000_e57828_d_n7, assign43000_e57828_d_n8, assign43000_e57828_d_n9, assign43000_e57828_d_n10, assign43000_e57828_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign43000_e57824: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign43000_e57826: f64 = (assign43000_e57824 * locals.var_dnm);
        (assign43000_e57826, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn4)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn5)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn8)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn9)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn13)) * locals.var_dnm) + (assign43000_e57824 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign43000_e57828;
        locals.var_tmf0_dn0 = assign43000_e57828_d_n0;
        locals.var_tmf0_dn2 = assign43000_e57828_d_n2;
        locals.var_tmf0_dn4 = assign43000_e57828_d_n4;
        locals.var_tmf0_dn5 = assign43000_e57828_d_n5;
        locals.var_tmf0_dn6 = assign43000_e57828_d_n6;
        locals.var_tmf0_dn7 = assign43000_e57828_d_n7;
        locals.var_tmf0_dn8 = assign43000_e57828_d_n8;
        locals.var_tmf0_dn9 = assign43000_e57828_d_n9;
        locals.var_tmf0_dn10 = assign43000_e57828_d_n10;
        locals.var_tmf0_dn13 = assign43000_e57828_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign43010_e57847, assign43010_e57847_d_n0, assign43010_e57847_d_n2, assign43010_e57847_d_n4, assign43010_e57847_d_n5, assign43010_e57847_d_n6, assign43010_e57847_d_n7, assign43010_e57847_d_n8, assign43010_e57847_d_n9, assign43010_e57847_d_n10, assign43010_e57847_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign43010_e57841: f64 = (locals.var_t5 * locals.var_xmp);
        let assign43010_e57843: f64 = (assign43010_e57841 * locals.var_dnm);
        let assign43010_e57845: f64 = (assign43010_e57843 / locals.var_arg);
        (assign43010_e57845, (((((((locals.var_t5_dn0 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn2 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn4 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn5 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn6 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn7 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn8 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn9 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn10 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn13 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign43010_e57841 * locals.var_dnm_dn13)) * locals.var_arg) - (assign43010_e57843 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43010_e57847;
        locals.var_t0_dn0 = assign43010_e57847_d_n0;
        locals.var_t0_dn2 = assign43010_e57847_d_n2;
        locals.var_t0_dn4 = assign43010_e57847_d_n4;
        locals.var_t0_dn5 = assign43010_e57847_d_n5;
        locals.var_t0_dn6 = assign43010_e57847_d_n6;
        locals.var_t0_dn7 = assign43010_e57847_d_n7;
        locals.var_t0_dn8 = assign43010_e57847_d_n8;
        locals.var_t0_dn9 = assign43010_e57847_d_n9;
        locals.var_t0_dn10 = assign43010_e57847_d_n10;
        locals.var_t0_dn13 = assign43010_e57847_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43020_e57864, assign43020_e57864_d_n0, assign43020_e57864_d_n2, assign43020_e57864_d_n4, assign43020_e57864_d_n5, assign43020_e57864_d_n6, assign43020_e57864_d_n7, assign43020_e57864_d_n8, assign43020_e57864_d_n9, assign43020_e57864_d_n10, assign43020_e57864_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        let assign43020_e57860: f64 = locals.var_t5;
        let assign43020_e57862: f64 = (assign43020_e57860 - locals.var_tmf0);
        (assign43020_e57862, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn4 - locals.var_tmf0_dn4), (locals.var_t5_dn5 - locals.var_tmf0_dn5), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn8 - locals.var_tmf0_dn8), (locals.var_t5_dn9 - locals.var_tmf0_dn9), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign43020_e57864;
        locals.var_t4_dn0 = assign43020_e57864_d_n0;
        locals.var_t4_dn2 = assign43020_e57864_d_n2;
        locals.var_t4_dn4 = assign43020_e57864_d_n4;
        locals.var_t4_dn5 = assign43020_e57864_d_n5;
        locals.var_t4_dn6 = assign43020_e57864_d_n6;
        locals.var_t4_dn7 = assign43020_e57864_d_n7;
        locals.var_t4_dn8 = assign43020_e57864_d_n8;
        locals.var_t4_dn9 = assign43020_e57864_d_n9;
        locals.var_t4_dn10 = assign43020_e57864_d_n10;
        locals.var_t4_dn13 = assign43020_e57864_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign43030_e57877, assign43030_e57877_d_n0, assign43030_e57877_d_n2, assign43030_e57877_d_n4, assign43030_e57877_d_n5, assign43030_e57877_d_n6, assign43030_e57877_d_n7, assign43030_e57877_d_n8, assign43030_e57877_d_n9, assign43030_e57877_d_n10, assign43030_e57877_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43030_e57877;
        locals.var_t0_dn0 = assign43030_e57877_d_n0;
        locals.var_t0_dn2 = assign43030_e57877_d_n2;
        locals.var_t0_dn4 = assign43030_e57877_d_n4;
        locals.var_t0_dn5 = assign43030_e57877_d_n5;
        locals.var_t0_dn6 = assign43030_e57877_d_n6;
        locals.var_t0_dn7 = assign43030_e57877_d_n7;
        locals.var_t0_dn8 = assign43030_e57877_d_n8;
        locals.var_t0_dn9 = assign43030_e57877_d_n9;
        locals.var_t0_dn10 = assign43030_e57877_d_n10;
        locals.var_t0_dn13 = assign43030_e57877_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43040_e57891, assign43040_e57891_d_n0, assign43040_e57891_d_n2, assign43040_e57891_d_n4, assign43040_e57891_d_n5, assign43040_e57891_d_n6, assign43040_e57891_d_n7, assign43040_e57891_d_n8, assign43040_e57891_d_n9, assign43040_e57891_d_n10, assign43040_e57891_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign43040_e57891;
        locals.var_t4_dn0 = assign43040_e57891_d_n0;
        locals.var_t4_dn2 = assign43040_e57891_d_n2;
        locals.var_t4_dn4 = assign43040_e57891_d_n4;
        locals.var_t4_dn5 = assign43040_e57891_d_n5;
        locals.var_t4_dn6 = assign43040_e57891_d_n6;
        locals.var_t4_dn7 = assign43040_e57891_d_n7;
        locals.var_t4_dn8 = assign43040_e57891_d_n8;
        locals.var_t4_dn9 = assign43040_e57891_d_n9;
        locals.var_t4_dn10 = assign43040_e57891_d_n10;
        locals.var_t4_dn13 = assign43040_e57891_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign43050_e57905, assign43050_e57905_d_n0, assign43050_e57905_d_n2, assign43050_e57905_d_n4, assign43050_e57905_d_n5, assign43050_e57905_d_n6, assign43050_e57905_d_n7, assign43050_e57905_d_n8, assign43050_e57905_d_n9, assign43050_e57905_d_n10, assign43050_e57905_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1058 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43050_e57905;
        locals.var_t0_dn0 = assign43050_e57905_d_n0;
        locals.var_t0_dn2 = assign43050_e57905_d_n2;
        locals.var_t0_dn4 = assign43050_e57905_d_n4;
        locals.var_t0_dn5 = assign43050_e57905_d_n5;
        locals.var_t0_dn6 = assign43050_e57905_d_n6;
        locals.var_t0_dn7 = assign43050_e57905_d_n7;
        locals.var_t0_dn8 = assign43050_e57905_d_n8;
        locals.var_t0_dn9 = assign43050_e57905_d_n9;
        locals.var_t0_dn10 = assign43050_e57905_d_n10;
        locals.var_t0_dn13 = assign43050_e57905_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_144(
        locals: &mut StampLocals,
    ) {
        let (assign43060_e57917, assign43060_e57917_d_n0, assign43060_e57917_d_n2, assign43060_e57917_d_n4, assign43060_e57917_d_n5, assign43060_e57917_d_n6, assign43060_e57917_d_n7, assign43060_e57917_d_n8, assign43060_e57917_d_n9, assign43060_e57917_d_n10, assign43060_e57917_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign43060_e57915: f64 = (locals.var_t4).sqrt();
        (assign43060_e57915, (locals.var_t4_dn0 / (2.0 * assign43060_e57915)), (locals.var_t4_dn2 / (2.0 * assign43060_e57915)), (locals.var_t4_dn4 / (2.0 * assign43060_e57915)), (locals.var_t4_dn5 / (2.0 * assign43060_e57915)), (locals.var_t4_dn6 / (2.0 * assign43060_e57915)), (locals.var_t4_dn7 / (2.0 * assign43060_e57915)), (locals.var_t4_dn8 / (2.0 * assign43060_e57915)), (locals.var_t4_dn9 / (2.0 * assign43060_e57915)), (locals.var_t4_dn10 / (2.0 * assign43060_e57915)), (locals.var_t4_dn13 / (2.0 * assign43060_e57915)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign43060_e57917;
        locals.var_t3_dn0 = assign43060_e57917_d_n0;
        locals.var_t3_dn2 = assign43060_e57917_d_n2;
        locals.var_t3_dn4 = assign43060_e57917_d_n4;
        locals.var_t3_dn5 = assign43060_e57917_d_n5;
        locals.var_t3_dn6 = assign43060_e57917_d_n6;
        locals.var_t3_dn7 = assign43060_e57917_d_n7;
        locals.var_t3_dn8 = assign43060_e57917_d_n8;
        locals.var_t3_dn9 = assign43060_e57917_d_n9;
        locals.var_t3_dn10 = assign43060_e57917_d_n10;
        locals.var_t3_dn13 = assign43060_e57917_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign43070_e57934, assign43070_e57934_d_n0, assign43070_e57934_d_n2, assign43070_e57934_d_n4, assign43070_e57934_d_n5, assign43070_e57934_d_n6, assign43070_e57934_d_n7, assign43070_e57934_d_n8, assign43070_e57934_d_n9, assign43070_e57934_d_n10, assign43070_e57934_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign43070_e57930: f64 = (1.0 - locals.var_t3);
        let assign43070_e57931: f64 = (locals.var_q_ndepm_esi_cox_inv2 * assign43070_e57930);
        let assign43070_e57932: f64 = (locals.var_vgp + assign43070_e57931);
        (assign43070_e57932, (locals.var_vgp_dn0 + ((locals.var_q_ndepm_esi_cox_inv2_dn0 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_q_ndepm_esi_cox_inv2_dn2 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_q_ndepm_esi_cox_inv2_dn4 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_q_ndepm_esi_cox_inv2_dn5 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_q_ndepm_esi_cox_inv2_dn6 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_q_ndepm_esi_cox_inv2_dn7 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_q_ndepm_esi_cox_inv2_dn8 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_q_ndepm_esi_cox_inv2_dn9 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_q_ndepm_esi_cox_inv2_dn10 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn13 + ((locals.var_q_ndepm_esi_cox_inv2_dn13 * assign43070_e57930) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn13)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign43070_e57934;
        locals.var_t10_dn0 = assign43070_e57934_d_n0;
        locals.var_t10_dn2 = assign43070_e57934_d_n2;
        locals.var_t10_dn4 = assign43070_e57934_d_n4;
        locals.var_t10_dn5 = assign43070_e57934_d_n5;
        locals.var_t10_dn6 = assign43070_e57934_d_n6;
        locals.var_t10_dn7 = assign43070_e57934_d_n7;
        locals.var_t10_dn8 = assign43070_e57934_d_n8;
        locals.var_t10_dn9 = assign43070_e57934_d_n9;
        locals.var_t10_dn10 = assign43070_e57934_d_n10;
        locals.var_t10_dn13 = assign43070_e57934_d_n13;
        locals.var_t10_rv = 0.0;

        let assign43080_e57938: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43080_e57943: f64 = if ((locals.var_t10 < assign43080_e57938) && (locals.var_depqfn_dlt >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign43080_e57943;
        locals.var_guard1064_rv = 0.0;

        let (assign43090_e57960, assign43090_e57960_d_n0, assign43090_e57960_d_n2, assign43090_e57960_d_n4, assign43090_e57960_d_n5, assign43090_e57960_d_n6, assign43090_e57960_d_n7, assign43090_e57960_d_n8, assign43090_e57960_d_n9, assign43090_e57960_d_n10, assign43090_e57960_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43090_e57956: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43090_e57958: f64 = (assign43090_e57956 - locals.var_t10);
        (assign43090_e57958, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn13 - locals.var_t10_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign43090_e57960;
        locals.var_tmf1_dn0 = assign43090_e57960_d_n0;
        locals.var_tmf1_dn2 = assign43090_e57960_d_n2;
        locals.var_tmf1_dn4 = assign43090_e57960_d_n4;
        locals.var_tmf1_dn5 = assign43090_e57960_d_n5;
        locals.var_tmf1_dn6 = assign43090_e57960_d_n6;
        locals.var_tmf1_dn7 = assign43090_e57960_d_n7;
        locals.var_tmf1_dn8 = assign43090_e57960_d_n8;
        locals.var_tmf1_dn9 = assign43090_e57960_d_n9;
        locals.var_tmf1_dn10 = assign43090_e57960_d_n10;
        locals.var_tmf1_dn13 = assign43090_e57960_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign43100_e57975, assign43100_e57975_d_n0, assign43100_e57975_d_n2, assign43100_e57975_d_n4, assign43100_e57975_d_n5, assign43100_e57975_d_n6, assign43100_e57975_d_n7, assign43100_e57975_d_n8, assign43100_e57975_d_n9, assign43100_e57975_d_n10, assign43100_e57975_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43100_e57973: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign43100_e57973, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign43100_e57975;
        locals.var_x2_dn0 = assign43100_e57975_d_n0;
        locals.var_x2_dn2 = assign43100_e57975_d_n2;
        locals.var_x2_dn4 = assign43100_e57975_d_n4;
        locals.var_x2_dn5 = assign43100_e57975_d_n5;
        locals.var_x2_dn6 = assign43100_e57975_d_n6;
        locals.var_x2_dn7 = assign43100_e57975_d_n7;
        locals.var_x2_dn8 = assign43100_e57975_d_n8;
        locals.var_x2_dn9 = assign43100_e57975_d_n9;
        locals.var_x2_dn10 = assign43100_e57975_d_n10;
        locals.var_x2_dn13 = assign43100_e57975_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign43110_e57990, assign43110_e57990_d_n0, assign43110_e57990_d_n2, assign43110_e57990_d_n4, assign43110_e57990_d_n5, assign43110_e57990_d_n6, assign43110_e57990_d_n7, assign43110_e57990_d_n8, assign43110_e57990_d_n9, assign43110_e57990_d_n10, assign43110_e57990_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43110_e57988: f64 = (locals.var_depqfn_dlt * locals.var_depqfn_dlt);
        (assign43110_e57988, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign43110_e57990;
        locals.var_xmax2_dn0 = assign43110_e57990_d_n0;
        locals.var_xmax2_dn2 = assign43110_e57990_d_n2;
        locals.var_xmax2_dn4 = assign43110_e57990_d_n4;
        locals.var_xmax2_dn5 = assign43110_e57990_d_n5;
        locals.var_xmax2_dn6 = assign43110_e57990_d_n6;
        locals.var_xmax2_dn7 = assign43110_e57990_d_n7;
        locals.var_xmax2_dn8 = assign43110_e57990_d_n8;
        locals.var_xmax2_dn9 = assign43110_e57990_d_n9;
        locals.var_xmax2_dn10 = assign43110_e57990_d_n10;
        locals.var_xmax2_dn13 = assign43110_e57990_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign43120_e58003, assign43120_e58003_d_n0, assign43120_e58003_d_n2, assign43120_e58003_d_n4, assign43120_e58003_d_n5, assign43120_e58003_d_n6, assign43120_e58003_d_n7, assign43120_e58003_d_n8, assign43120_e58003_d_n9, assign43120_e58003_d_n10, assign43120_e58003_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign43120_e58003;
        locals.var_xp_dn0 = assign43120_e58003_d_n0;
        locals.var_xp_dn2 = assign43120_e58003_d_n2;
        locals.var_xp_dn4 = assign43120_e58003_d_n4;
        locals.var_xp_dn5 = assign43120_e58003_d_n5;
        locals.var_xp_dn6 = assign43120_e58003_d_n6;
        locals.var_xp_dn7 = assign43120_e58003_d_n7;
        locals.var_xp_dn8 = assign43120_e58003_d_n8;
        locals.var_xp_dn9 = assign43120_e58003_d_n9;
        locals.var_xp_dn10 = assign43120_e58003_d_n10;
        locals.var_xp_dn13 = assign43120_e58003_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign43130_e58016, assign43130_e58016_d_n0, assign43130_e58016_d_n2, assign43130_e58016_d_n4, assign43130_e58016_d_n5, assign43130_e58016_d_n6, assign43130_e58016_d_n7, assign43130_e58016_d_n8, assign43130_e58016_d_n9, assign43130_e58016_d_n10, assign43130_e58016_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign43130_e58016;
        locals.var_xmp_dn0 = assign43130_e58016_d_n0;
        locals.var_xmp_dn2 = assign43130_e58016_d_n2;
        locals.var_xmp_dn4 = assign43130_e58016_d_n4;
        locals.var_xmp_dn5 = assign43130_e58016_d_n5;
        locals.var_xmp_dn6 = assign43130_e58016_d_n6;
        locals.var_xmp_dn7 = assign43130_e58016_d_n7;
        locals.var_xmp_dn8 = assign43130_e58016_d_n8;
        locals.var_xmp_dn9 = assign43130_e58016_d_n9;
        locals.var_xmp_dn10 = assign43130_e58016_d_n10;
        locals.var_xmp_dn13 = assign43130_e58016_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign43140_e58029,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43140_e58029;
        locals.var_m0_rv = 0.0;

        let (assign43150_e58042,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43150_e58042;
        locals.var_mm_rv = 0.0;

        let (assign43160_e58055, assign43160_e58055_d_n0, assign43160_e58055_d_n2, assign43160_e58055_d_n4, assign43160_e58055_d_n5, assign43160_e58055_d_n6, assign43160_e58055_d_n7, assign43160_e58055_d_n8, assign43160_e58055_d_n9, assign43160_e58055_d_n10, assign43160_e58055_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign43160_e58055;
        locals.var_arg_dn0 = assign43160_e58055_d_n0;
        locals.var_arg_dn2 = assign43160_e58055_d_n2;
        locals.var_arg_dn4 = assign43160_e58055_d_n4;
        locals.var_arg_dn5 = assign43160_e58055_d_n5;
        locals.var_arg_dn6 = assign43160_e58055_d_n6;
        locals.var_arg_dn7 = assign43160_e58055_d_n7;
        locals.var_arg_dn8 = assign43160_e58055_d_n8;
        locals.var_arg_dn9 = assign43160_e58055_d_n9;
        locals.var_arg_dn10 = assign43160_e58055_d_n10;
        locals.var_arg_dn13 = assign43160_e58055_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign43170_e58068, assign43170_e58068_d_n0, assign43170_e58068_d_n2, assign43170_e58068_d_n4, assign43170_e58068_d_n5, assign43170_e58068_d_n6, assign43170_e58068_d_n7, assign43170_e58068_d_n8, assign43170_e58068_d_n9, assign43170_e58068_d_n10, assign43170_e58068_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43170_e58068;
        locals.var_dnm_dn0 = assign43170_e58068_d_n0;
        locals.var_dnm_dn2 = assign43170_e58068_d_n2;
        locals.var_dnm_dn4 = assign43170_e58068_d_n4;
        locals.var_dnm_dn5 = assign43170_e58068_d_n5;
        locals.var_dnm_dn6 = assign43170_e58068_d_n6;
        locals.var_dnm_dn7 = assign43170_e58068_d_n7;
        locals.var_dnm_dn8 = assign43170_e58068_d_n8;
        locals.var_dnm_dn9 = assign43170_e58068_d_n9;
        locals.var_dnm_dn10 = assign43170_e58068_d_n10;
        locals.var_dnm_dn13 = assign43170_e58068_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign43180_e58083, assign43180_e58083_d_n0, assign43180_e58083_d_n2, assign43180_e58083_d_n4, assign43180_e58083_d_n5, assign43180_e58083_d_n6, assign43180_e58083_d_n7, assign43180_e58083_d_n8, assign43180_e58083_d_n9, assign43180_e58083_d_n10, assign43180_e58083_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43180_e58081: f64 = (locals.var_xp * locals.var_x2);
        (assign43180_e58081, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign43180_e58083;
        locals.var_xp_dn0 = assign43180_e58083_d_n0;
        locals.var_xp_dn2 = assign43180_e58083_d_n2;
        locals.var_xp_dn4 = assign43180_e58083_d_n4;
        locals.var_xp_dn5 = assign43180_e58083_d_n5;
        locals.var_xp_dn6 = assign43180_e58083_d_n6;
        locals.var_xp_dn7 = assign43180_e58083_d_n7;
        locals.var_xp_dn8 = assign43180_e58083_d_n8;
        locals.var_xp_dn9 = assign43180_e58083_d_n9;
        locals.var_xp_dn10 = assign43180_e58083_d_n10;
        locals.var_xp_dn13 = assign43180_e58083_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign43190_e58098, assign43190_e58098_d_n0, assign43190_e58098_d_n2, assign43190_e58098_d_n4, assign43190_e58098_d_n5, assign43190_e58098_d_n6, assign43190_e58098_d_n7, assign43190_e58098_d_n8, assign43190_e58098_d_n9, assign43190_e58098_d_n10, assign43190_e58098_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43190_e58096: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43190_e58096, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign43190_e58098;
        locals.var_xmp_dn0 = assign43190_e58098_d_n0;
        locals.var_xmp_dn2 = assign43190_e58098_d_n2;
        locals.var_xmp_dn4 = assign43190_e58098_d_n4;
        locals.var_xmp_dn5 = assign43190_e58098_d_n5;
        locals.var_xmp_dn6 = assign43190_e58098_d_n6;
        locals.var_xmp_dn7 = assign43190_e58098_d_n7;
        locals.var_xmp_dn8 = assign43190_e58098_d_n8;
        locals.var_xmp_dn9 = assign43190_e58098_d_n9;
        locals.var_xmp_dn10 = assign43190_e58098_d_n10;
        locals.var_xmp_dn13 = assign43190_e58098_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign43200_e58113, assign43200_e58113_d_n0, assign43200_e58113_d_n2, assign43200_e58113_d_n4, assign43200_e58113_d_n5, assign43200_e58113_d_n6, assign43200_e58113_d_n7, assign43200_e58113_d_n8, assign43200_e58113_d_n9, assign43200_e58113_d_n10, assign43200_e58113_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43200_e58111: f64 = (locals.var_xp * locals.var_x2);
        (assign43200_e58111, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign43200_e58113;
        locals.var_xp_dn0 = assign43200_e58113_d_n0;
        locals.var_xp_dn2 = assign43200_e58113_d_n2;
        locals.var_xp_dn4 = assign43200_e58113_d_n4;
        locals.var_xp_dn5 = assign43200_e58113_d_n5;
        locals.var_xp_dn6 = assign43200_e58113_d_n6;
        locals.var_xp_dn7 = assign43200_e58113_d_n7;
        locals.var_xp_dn8 = assign43200_e58113_d_n8;
        locals.var_xp_dn9 = assign43200_e58113_d_n9;
        locals.var_xp_dn10 = assign43200_e58113_d_n10;
        locals.var_xp_dn13 = assign43200_e58113_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign43210_e58128, assign43210_e58128_d_n0, assign43210_e58128_d_n2, assign43210_e58128_d_n4, assign43210_e58128_d_n5, assign43210_e58128_d_n6, assign43210_e58128_d_n7, assign43210_e58128_d_n8, assign43210_e58128_d_n9, assign43210_e58128_d_n10, assign43210_e58128_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43210_e58126: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43210_e58126, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign43210_e58128;
        locals.var_xmp_dn0 = assign43210_e58128_d_n0;
        locals.var_xmp_dn2 = assign43210_e58128_d_n2;
        locals.var_xmp_dn4 = assign43210_e58128_d_n4;
        locals.var_xmp_dn5 = assign43210_e58128_d_n5;
        locals.var_xmp_dn6 = assign43210_e58128_d_n6;
        locals.var_xmp_dn7 = assign43210_e58128_d_n7;
        locals.var_xmp_dn8 = assign43210_e58128_d_n8;
        locals.var_xmp_dn9 = assign43210_e58128_d_n9;
        locals.var_xmp_dn10 = assign43210_e58128_d_n10;
        locals.var_xmp_dn13 = assign43210_e58128_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign43220_e58143, assign43220_e58143_d_n0, assign43220_e58143_d_n2, assign43220_e58143_d_n4, assign43220_e58143_d_n5, assign43220_e58143_d_n6, assign43220_e58143_d_n7, assign43220_e58143_d_n8, assign43220_e58143_d_n9, assign43220_e58143_d_n10, assign43220_e58143_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43220_e58141: f64 = (locals.var_xp + locals.var_xmp);
        (assign43220_e58141, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign43220_e58143;
        locals.var_arg_dn0 = assign43220_e58143_d_n0;
        locals.var_arg_dn2 = assign43220_e58143_d_n2;
        locals.var_arg_dn4 = assign43220_e58143_d_n4;
        locals.var_arg_dn5 = assign43220_e58143_d_n5;
        locals.var_arg_dn6 = assign43220_e58143_d_n6;
        locals.var_arg_dn7 = assign43220_e58143_d_n7;
        locals.var_arg_dn8 = assign43220_e58143_d_n8;
        locals.var_arg_dn9 = assign43220_e58143_d_n9;
        locals.var_arg_dn10 = assign43220_e58143_d_n10;
        locals.var_arg_dn13 = assign43220_e58143_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign43230_e58156, assign43230_e58156_d_n0, assign43230_e58156_d_n2, assign43230_e58156_d_n4, assign43230_e58156_d_n5, assign43230_e58156_d_n6, assign43230_e58156_d_n7, assign43230_e58156_d_n8, assign43230_e58156_d_n9, assign43230_e58156_d_n10, assign43230_e58156_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43230_e58156;
        locals.var_dnm_dn0 = assign43230_e58156_d_n0;
        locals.var_dnm_dn2 = assign43230_e58156_d_n2;
        locals.var_dnm_dn4 = assign43230_e58156_d_n4;
        locals.var_dnm_dn5 = assign43230_e58156_d_n5;
        locals.var_dnm_dn6 = assign43230_e58156_d_n6;
        locals.var_dnm_dn7 = assign43230_e58156_d_n7;
        locals.var_dnm_dn8 = assign43230_e58156_d_n8;
        locals.var_dnm_dn9 = assign43230_e58156_d_n9;
        locals.var_dnm_dn10 = assign43230_e58156_d_n10;
        locals.var_dnm_dn13 = assign43230_e58156_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign43240_e58171: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign43240_e58171;
        locals.var_guard1065_rv = 0.0;

        let assign43250_e58174: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign43250_e58174;
        locals.var_guard1066_rv = 0.0;

        let (assign43260_e58191,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43260_e58191;
        locals.var_mm_rv = 0.0;

        let assign43270_e58194: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign43270_e58194;
        locals.var_guard1067_rv = 0.0;

        let (assign43280_e58214,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 == 0.0)) && (locals.var_guard1067 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43280_e58214;
        locals.var_mm_rv = 0.0;

        let assign43290_e58217: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign43290_e58217;
        locals.var_guard1068_rv = 0.0;

        let (assign43300_e58240,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 == 0.0)) && (locals.var_guard1067 == 0.0)) && (locals.var_guard1068 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43300_e58240;
        locals.var_mm_rv = 0.0;

        let assign43310_e58243: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign43310_e58243;
        locals.var_guard1069_rv = 0.0;

        let (assign43320_e58269,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 == 0.0)) && (locals.var_guard1067 == 0.0)) && (locals.var_guard1068 == 0.0)) && (locals.var_guard1069 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43320_e58269;
        locals.var_mm_rv = 0.0;

        let (assign43330_e58284,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43330_e58284;
        locals.var_m0_rv = 0.0;

        let mut assign43340_loop_guard: usize = 0;
        while {
            let assign43340_cond_e58300: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign43340_cond_e58300 != 0.0
        } {
            assign43340_loop_guard += 1;
            assert!(assign43340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign43340_body0_e58316, assign43340_body0_e58316_d_n0, assign43340_body0_e58316_d_n2, assign43340_body0_e58316_d_n4, assign43340_body0_e58316_d_n5, assign43340_body0_e58316_d_n6, assign43340_body0_e58316_d_n7, assign43340_body0_e58316_d_n8, assign43340_body0_e58316_d_n9, assign43340_body0_e58316_d_n10, assign43340_body0_e58316_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
        let assign43340_body0_e58314: f64 = (locals.var_dnm).sqrt();
        (assign43340_body0_e58314, (locals.var_dnm_dn0 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn2 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn4 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn5 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn6 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn7 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn8 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn9 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn10 / (2.0 * assign43340_body0_e58314)), (locals.var_dnm_dn13 / (2.0 * assign43340_body0_e58314)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign43340_body0_e58316;
            locals.var_dnm_dn0 = assign43340_body0_e58316_d_n0;
            locals.var_dnm_dn2 = assign43340_body0_e58316_d_n2;
            locals.var_dnm_dn4 = assign43340_body0_e58316_d_n4;
            locals.var_dnm_dn5 = assign43340_body0_e58316_d_n5;
            locals.var_dnm_dn6 = assign43340_body0_e58316_d_n6;
            locals.var_dnm_dn7 = assign43340_body0_e58316_d_n7;
            locals.var_dnm_dn8 = assign43340_body0_e58316_d_n8;
            locals.var_dnm_dn9 = assign43340_body0_e58316_d_n9;
            locals.var_dnm_dn10 = assign43340_body0_e58316_d_n10;
            locals.var_dnm_dn13 = assign43340_body0_e58316_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign43340_body1_e58333,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
        let assign43340_body1_e58331: f64 = (locals.var_m0 + 1.0);
        (assign43340_body1_e58331,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign43340_body1_e58333;
            locals.var_m0_rv = 0.0;
        }

        let (assign43350_e58360, assign43350_e58360_d_n0, assign43350_e58360_d_n2, assign43350_e58360_d_n4, assign43350_e58360_d_n5, assign43350_e58360_d_n6, assign43350_e58360_d_n7, assign43350_e58360_d_n8, assign43350_e58360_d_n9, assign43350_e58360_d_n10, assign43350_e58360_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
        let (assign43350_e58358, assign43350_e58358_d_n0, assign43350_e58358_d_n2, assign43350_e58358_d_n4, assign43350_e58358_d_n5, assign43350_e58358_d_n6, assign43350_e58358_d_n7, assign43350_e58358_d_n8, assign43350_e58358_d_n9, assign43350_e58358_d_n10, assign43350_e58358_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43350_e58355: f64 = (2.0 * 2.0);
                let assign43350_e58356: f64 = (1.0 / assign43350_e58355);
                let assign43350_e58357: f64 = (locals.var_dnm).powf(assign43350_e58356);
                (assign43350_e58357, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43350_e58356) as f64).is_finite() && ((assign43350_e58356) as f64).fract() == 0.0 { if assign43350_e58356 == 0.0 { 0.0 } else { (assign43350_e58356 * ((locals.var_dnm).powf(assign43350_e58356 - 1.0) * locals.var_dnm_dn13)) } } else { (assign43350_e58357 * (assign43350_e58356 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign43350_e58358, assign43350_e58358_d_n0, assign43350_e58358_d_n2, assign43350_e58358_d_n4, assign43350_e58358_d_n5, assign43350_e58358_d_n6, assign43350_e58358_d_n7, assign43350_e58358_d_n8, assign43350_e58358_d_n9, assign43350_e58358_d_n10, assign43350_e58358_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43350_e58360;
        locals.var_dnm_dn0 = assign43350_e58360_d_n0;
        locals.var_dnm_dn2 = assign43350_e58360_d_n2;
        locals.var_dnm_dn4 = assign43350_e58360_d_n4;
        locals.var_dnm_dn5 = assign43350_e58360_d_n5;
        locals.var_dnm_dn6 = assign43350_e58360_d_n6;
        locals.var_dnm_dn7 = assign43350_e58360_d_n7;
        locals.var_dnm_dn8 = assign43350_e58360_d_n8;
        locals.var_dnm_dn9 = assign43350_e58360_d_n9;
        locals.var_dnm_dn10 = assign43350_e58360_d_n10;
        locals.var_dnm_dn13 = assign43350_e58360_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign43360_e58375, assign43360_e58375_d_n0, assign43360_e58375_d_n2, assign43360_e58375_d_n4, assign43360_e58375_d_n5, assign43360_e58375_d_n6, assign43360_e58375_d_n7, assign43360_e58375_d_n8, assign43360_e58375_d_n9, assign43360_e58375_d_n10, assign43360_e58375_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43360_e58373: f64 = (1.0 / locals.var_dnm);
        (assign43360_e58373, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43360_e58375;
        locals.var_dnm_dn0 = assign43360_e58375_d_n0;
        locals.var_dnm_dn2 = assign43360_e58375_d_n2;
        locals.var_dnm_dn4 = assign43360_e58375_d_n4;
        locals.var_dnm_dn5 = assign43360_e58375_d_n5;
        locals.var_dnm_dn6 = assign43360_e58375_d_n6;
        locals.var_dnm_dn7 = assign43360_e58375_d_n7;
        locals.var_dnm_dn8 = assign43360_e58375_d_n8;
        locals.var_dnm_dn9 = assign43360_e58375_d_n9;
        locals.var_dnm_dn10 = assign43360_e58375_d_n10;
        locals.var_dnm_dn13 = assign43360_e58375_d_n13;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_145(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign43370_e58392, assign43370_e58392_d_n0, assign43370_e58392_d_n2, assign43370_e58392_d_n4, assign43370_e58392_d_n5, assign43370_e58392_d_n6, assign43370_e58392_d_n7, assign43370_e58392_d_n8, assign43370_e58392_d_n9, assign43370_e58392_d_n10, assign43370_e58392_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43370_e58388: f64 = (locals.var_tmf1 * locals.var_depqfn_dlt);
        let assign43370_e58390: f64 = (assign43370_e58388 * locals.var_dnm);
        (assign43370_e58390, (((locals.var_tmf1_dn0 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43370_e58388 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign43370_e58392;
        locals.var_tmf0_dn0 = assign43370_e58392_d_n0;
        locals.var_tmf0_dn2 = assign43370_e58392_d_n2;
        locals.var_tmf0_dn4 = assign43370_e58392_d_n4;
        locals.var_tmf0_dn5 = assign43370_e58392_d_n5;
        locals.var_tmf0_dn6 = assign43370_e58392_d_n6;
        locals.var_tmf0_dn7 = assign43370_e58392_d_n7;
        locals.var_tmf0_dn8 = assign43370_e58392_d_n8;
        locals.var_tmf0_dn9 = assign43370_e58392_d_n9;
        locals.var_tmf0_dn10 = assign43370_e58392_d_n10;
        locals.var_tmf0_dn13 = assign43370_e58392_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign43380_e58411, assign43380_e58411_d_n0, assign43380_e58411_d_n2, assign43380_e58411_d_n4, assign43380_e58411_d_n5, assign43380_e58411_d_n6, assign43380_e58411_d_n7, assign43380_e58411_d_n8, assign43380_e58411_d_n9, assign43380_e58411_d_n10, assign43380_e58411_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43380_e58405: f64 = (locals.var_depqfn_dlt * locals.var_xmp);
        let assign43380_e58407: f64 = (assign43380_e58405 * locals.var_dnm);
        let assign43380_e58409: f64 = (assign43380_e58407 / locals.var_arg);
        (assign43380_e58409, ((((((locals.var_depqfn_dlt * locals.var_xmp_dn0) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn2) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn4) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn5) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn6) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn7) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn8) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn9) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn10) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn13) * locals.var_dnm) + (assign43380_e58405 * locals.var_dnm_dn13)) * locals.var_arg) - (assign43380_e58407 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43380_e58411;
        locals.var_t0_dn0 = assign43380_e58411_d_n0;
        locals.var_t0_dn2 = assign43380_e58411_d_n2;
        locals.var_t0_dn4 = assign43380_e58411_d_n4;
        locals.var_t0_dn5 = assign43380_e58411_d_n5;
        locals.var_t0_dn6 = assign43380_e58411_d_n6;
        locals.var_t0_dn7 = assign43380_e58411_d_n7;
        locals.var_t0_dn8 = assign43380_e58411_d_n8;
        locals.var_t0_dn9 = assign43380_e58411_d_n9;
        locals.var_t0_dn10 = assign43380_e58411_d_n10;
        locals.var_t0_dn13 = assign43380_e58411_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43390_e58428, assign43390_e58428_d_n0, assign43390_e58428_d_n2, assign43390_e58428_d_n4, assign43390_e58428_d_n5, assign43390_e58428_d_n6, assign43390_e58428_d_n7, assign43390_e58428_d_n8, assign43390_e58428_d_n9, assign43390_e58428_d_n10, assign43390_e58428_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        let assign43390_e58424: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43390_e58426: f64 = (assign43390_e58424 - locals.var_tmf0);
        (assign43390_e58426, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign43390_e58428;
        locals.var_t10_dn0 = assign43390_e58428_d_n0;
        locals.var_t10_dn2 = assign43390_e58428_d_n2;
        locals.var_t10_dn4 = assign43390_e58428_d_n4;
        locals.var_t10_dn5 = assign43390_e58428_d_n5;
        locals.var_t10_dn6 = assign43390_e58428_d_n6;
        locals.var_t10_dn7 = assign43390_e58428_d_n7;
        locals.var_t10_dn8 = assign43390_e58428_d_n8;
        locals.var_t10_dn9 = assign43390_e58428_d_n9;
        locals.var_t10_dn10 = assign43390_e58428_d_n10;
        locals.var_t10_dn13 = assign43390_e58428_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign43400_e58441, assign43400_e58441_d_n0, assign43400_e58441_d_n2, assign43400_e58441_d_n4, assign43400_e58441_d_n5, assign43400_e58441_d_n6, assign43400_e58441_d_n7, assign43400_e58441_d_n8, assign43400_e58441_d_n9, assign43400_e58441_d_n10, assign43400_e58441_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43400_e58441;
        locals.var_t0_dn0 = assign43400_e58441_d_n0;
        locals.var_t0_dn2 = assign43400_e58441_d_n2;
        locals.var_t0_dn4 = assign43400_e58441_d_n4;
        locals.var_t0_dn5 = assign43400_e58441_d_n5;
        locals.var_t0_dn6 = assign43400_e58441_d_n6;
        locals.var_t0_dn7 = assign43400_e58441_d_n7;
        locals.var_t0_dn8 = assign43400_e58441_d_n8;
        locals.var_t0_dn9 = assign43400_e58441_d_n9;
        locals.var_t0_dn10 = assign43400_e58441_d_n10;
        locals.var_t0_dn13 = assign43400_e58441_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43410_e58455, assign43410_e58455_d_n0, assign43410_e58455_d_n2, assign43410_e58455_d_n4, assign43410_e58455_d_n5, assign43410_e58455_d_n6, assign43410_e58455_d_n7, assign43410_e58455_d_n8, assign43410_e58455_d_n9, assign43410_e58455_d_n10, assign43410_e58455_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign43410_e58455;
        locals.var_t10_dn0 = assign43410_e58455_d_n0;
        locals.var_t10_dn2 = assign43410_e58455_d_n2;
        locals.var_t10_dn4 = assign43410_e58455_d_n4;
        locals.var_t10_dn5 = assign43410_e58455_d_n5;
        locals.var_t10_dn6 = assign43410_e58455_d_n6;
        locals.var_t10_dn7 = assign43410_e58455_d_n7;
        locals.var_t10_dn8 = assign43410_e58455_d_n8;
        locals.var_t10_dn9 = assign43410_e58455_d_n9;
        locals.var_t10_dn10 = assign43410_e58455_d_n10;
        locals.var_t10_dn13 = assign43410_e58455_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign43420_e58469, assign43420_e58469_d_n0, assign43420_e58469_d_n2, assign43420_e58469_d_n4, assign43420_e58469_d_n5, assign43420_e58469_d_n6, assign43420_e58469_d_n7, assign43420_e58469_d_n8, assign43420_e58469_d_n9, assign43420_e58469_d_n10, assign43420_e58469_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) && (locals.var_guard1064 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43420_e58469;
        locals.var_t0_dn0 = assign43420_e58469_d_n0;
        locals.var_t0_dn2 = assign43420_e58469_d_n2;
        locals.var_t0_dn4 = assign43420_e58469_d_n4;
        locals.var_t0_dn5 = assign43420_e58469_d_n5;
        locals.var_t0_dn6 = assign43420_e58469_d_n6;
        locals.var_t0_dn7 = assign43420_e58469_d_n7;
        locals.var_t0_dn8 = assign43420_e58469_d_n8;
        locals.var_t0_dn9 = assign43420_e58469_d_n9;
        locals.var_t0_dn10 = assign43420_e58469_d_n10;
        locals.var_t0_dn13 = assign43420_e58469_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43430_e58482, assign43430_e58482_d_n0, assign43430_e58482_d_n2, assign43430_e58482_d_n4, assign43430_e58482_d_n5, assign43430_e58482_d_n6, assign43430_e58482_d_n7, assign43430_e58482_d_n8, assign43430_e58482_d_n9, assign43430_e58482_d_n10, assign43430_e58482_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign43430_e58480: f64 = (locals.var_vds_res / locals.var_t10);
        (assign43430_e58480, (((locals.var_vds_res_dn0 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn2 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn4 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn5 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn6 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn7 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn8 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn9 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn10 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn13 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn13)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43430_e58482;
        locals.var_t1_dn0 = assign43430_e58482_d_n0;
        locals.var_t1_dn2 = assign43430_e58482_d_n2;
        locals.var_t1_dn4 = assign43430_e58482_d_n4;
        locals.var_t1_dn5 = assign43430_e58482_d_n5;
        locals.var_t1_dn6 = assign43430_e58482_d_n6;
        locals.var_t1_dn7 = assign43430_e58482_d_n7;
        locals.var_t1_dn8 = assign43430_e58482_d_n8;
        locals.var_t1_dn9 = assign43430_e58482_d_n9;
        locals.var_t1_dn10 = assign43430_e58482_d_n10;
        locals.var_t1_dn13 = assign43430_e58482_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign43440_e58502, assign43440_e58502_d_n0, assign43440_e58502_d_n2, assign43440_e58502_d_n4, assign43440_e58502_d_n5, assign43440_e58502_d_n6, assign43440_e58502_d_n7, assign43440_e58502_d_n8, assign43440_e58502_d_n9, assign43440_e58502_d_n10, assign43440_e58502_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let (assign43440_e58500, assign43440_e58500_d_n0, assign43440_e58500_d_n2, assign43440_e58500_d_n4, assign43440_e58500_d_n5, assign43440_e58500_d_n6, assign43440_e58500_d_n7, assign43440_e58500_d_n8, assign43440_e58500_d_n9, assign43440_e58500_d_n10, assign43440_e58500_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43440_e58498: f64 = (p.p383 - 1.0);
                let assign43440_e58499: f64 = (locals.var_t1).powf(assign43440_e58498);
                (assign43440_e58499, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn0)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn2)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn4)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn5)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn6)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn7)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn8)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn9)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn10)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43440_e58498) as f64).is_finite() && ((assign43440_e58498) as f64).fract() == 0.0 { if assign43440_e58498 == 0.0 { 0.0 } else { (assign43440_e58498 * ((locals.var_t1).powf(assign43440_e58498 - 1.0) * locals.var_t1_dn13)) } } else { (assign43440_e58499 * (assign43440_e58498 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign43440_e58500, assign43440_e58500_d_n0, assign43440_e58500_d_n2, assign43440_e58500_d_n4, assign43440_e58500_d_n5, assign43440_e58500_d_n6, assign43440_e58500_d_n7, assign43440_e58500_d_n8, assign43440_e58500_d_n9, assign43440_e58500_d_n10, assign43440_e58500_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign43440_e58502;
        locals.var_t2_dn0 = assign43440_e58502_d_n0;
        locals.var_t2_dn2 = assign43440_e58502_d_n2;
        locals.var_t2_dn4 = assign43440_e58502_d_n4;
        locals.var_t2_dn5 = assign43440_e58502_d_n5;
        locals.var_t2_dn6 = assign43440_e58502_d_n6;
        locals.var_t2_dn7 = assign43440_e58502_d_n7;
        locals.var_t2_dn8 = assign43440_e58502_d_n8;
        locals.var_t2_dn9 = assign43440_e58502_d_n9;
        locals.var_t2_dn10 = assign43440_e58502_d_n10;
        locals.var_t2_dn13 = assign43440_e58502_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign43450_e58517, assign43450_e58517_d_n0, assign43450_e58517_d_n2, assign43450_e58517_d_n4, assign43450_e58517_d_n5, assign43450_e58517_d_n6, assign43450_e58517_d_n7, assign43450_e58517_d_n8, assign43450_e58517_d_n9, assign43450_e58517_d_n10, assign43450_e58517_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign43450_e58514: f64 = (locals.var_t2 * locals.var_t1);
        let assign43450_e58515: f64 = (1.0 + assign43450_e58514);
        (assign43450_e58515, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn13 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign43450_e58517;
        locals.var_t3_dn0 = assign43450_e58517_d_n0;
        locals.var_t3_dn2 = assign43450_e58517_d_n2;
        locals.var_t3_dn4 = assign43450_e58517_d_n4;
        locals.var_t3_dn5 = assign43450_e58517_d_n5;
        locals.var_t3_dn6 = assign43450_e58517_d_n6;
        locals.var_t3_dn7 = assign43450_e58517_d_n7;
        locals.var_t3_dn8 = assign43450_e58517_d_n8;
        locals.var_t3_dn9 = assign43450_e58517_d_n9;
        locals.var_t3_dn10 = assign43450_e58517_d_n10;
        locals.var_t3_dn13 = assign43450_e58517_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign43460_e58539, assign43460_e58539_d_n0, assign43460_e58539_d_n2, assign43460_e58539_d_n4, assign43460_e58539_d_n5, assign43460_e58539_d_n6, assign43460_e58539_d_n7, assign43460_e58539_d_n8, assign43460_e58539_d_n9, assign43460_e58539_d_n10, assign43460_e58539_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let (assign43460_e58537, assign43460_e58537_d_n0, assign43460_e58537_d_n2, assign43460_e58537_d_n4, assign43460_e58537_d_n5, assign43460_e58537_d_n6, assign43460_e58537_d_n7, assign43460_e58537_d_n8, assign43460_e58537_d_n9, assign43460_e58537_d_n10, assign43460_e58537_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43460_e58533: f64 = (1.0 / p.p383);
                let assign43460_e58535: f64 = (assign43460_e58533 - 1.0);
                let assign43460_e58536: f64 = (locals.var_t3).powf(assign43460_e58535);
                (assign43460_e58536, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn0)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn2)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn4)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn5)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn6)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn7)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn8)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn9)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn10)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43460_e58535) as f64).is_finite() && ((assign43460_e58535) as f64).fract() == 0.0 { if assign43460_e58535 == 0.0 { 0.0 } else { (assign43460_e58535 * ((locals.var_t3).powf(assign43460_e58535 - 1.0) * locals.var_t3_dn13)) } } else { (assign43460_e58536 * (assign43460_e58535 * (locals.var_t3_dn13 / locals.var_t3))) },)
            }
        };
        (assign43460_e58537, assign43460_e58537_d_n0, assign43460_e58537_d_n2, assign43460_e58537_d_n4, assign43460_e58537_d_n5, assign43460_e58537_d_n6, assign43460_e58537_d_n7, assign43460_e58537_d_n8, assign43460_e58537_d_n9, assign43460_e58537_d_n10, assign43460_e58537_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign43460_e58539;
        locals.var_t4_dn0 = assign43460_e58539_d_n0;
        locals.var_t4_dn2 = assign43460_e58539_d_n2;
        locals.var_t4_dn4 = assign43460_e58539_d_n4;
        locals.var_t4_dn5 = assign43460_e58539_d_n5;
        locals.var_t4_dn6 = assign43460_e58539_d_n6;
        locals.var_t4_dn7 = assign43460_e58539_d_n7;
        locals.var_t4_dn8 = assign43460_e58539_d_n8;
        locals.var_t4_dn9 = assign43460_e58539_d_n9;
        locals.var_t4_dn10 = assign43460_e58539_d_n10;
        locals.var_t4_dn13 = assign43460_e58539_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign43470_e58552, assign43470_e58552_d_n0, assign43470_e58552_d_n2, assign43470_e58552_d_n4, assign43470_e58552_d_n5, assign43470_e58552_d_n6, assign43470_e58552_d_n7, assign43470_e58552_d_n8, assign43470_e58552_d_n9, assign43470_e58552_d_n10, assign43470_e58552_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign43470_e58550: f64 = (locals.var_t4 * locals.var_t3);
        (assign43470_e58550, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn13 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign43470_e58552;
        locals.var_t6_dn0 = assign43470_e58552_d_n0;
        locals.var_t6_dn2 = assign43470_e58552_d_n2;
        locals.var_t6_dn4 = assign43470_e58552_d_n4;
        locals.var_t6_dn5 = assign43470_e58552_d_n5;
        locals.var_t6_dn6 = assign43470_e58552_d_n6;
        locals.var_t6_dn7 = assign43470_e58552_d_n7;
        locals.var_t6_dn8 = assign43470_e58552_d_n8;
        locals.var_t6_dn9 = assign43470_e58552_d_n9;
        locals.var_t6_dn10 = assign43470_e58552_d_n10;
        locals.var_t6_dn13 = assign43470_e58552_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign43480_e58565, assign43480_e58565_d_n0, assign43480_e58565_d_n2, assign43480_e58565_d_n4, assign43480_e58565_d_n5, assign43480_e58565_d_n6, assign43480_e58565_d_n7, assign43480_e58565_d_n8, assign43480_e58565_d_n9, assign43480_e58565_d_n10, assign43480_e58565_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        let assign43480_e58563: f64 = (locals.var_vds_res / locals.var_t6);
        (assign43480_e58563, (((locals.var_vds_res_dn0 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn2 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn4 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn5 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn6 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn7 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn8 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn9 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn10 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn13 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn13)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn13,)
    }
};
        locals.var_vds_res = assign43480_e58565;
        locals.var_vds_res_dn0 = assign43480_e58565_d_n0;
        locals.var_vds_res_dn2 = assign43480_e58565_d_n2;
        locals.var_vds_res_dn4 = assign43480_e58565_d_n4;
        locals.var_vds_res_dn5 = assign43480_e58565_d_n5;
        locals.var_vds_res_dn6 = assign43480_e58565_d_n6;
        locals.var_vds_res_dn7 = assign43480_e58565_d_n7;
        locals.var_vds_res_dn8 = assign43480_e58565_d_n8;
        locals.var_vds_res_dn9 = assign43480_e58565_d_n9;
        locals.var_vds_res_dn10 = assign43480_e58565_d_n10;
        locals.var_vds_res_dn13 = assign43480_e58565_d_n13;
        locals.var_vds_res_rv = 0.0;

        let (assign43490_e58576, assign43490_e58576_d_n0, assign43490_e58576_d_n2, assign43490_e58576_d_n4, assign43490_e58576_d_n5, assign43490_e58576_d_n6, assign43490_e58576_d_n7, assign43490_e58576_d_n8, assign43490_e58576_d_n9, assign43490_e58576_d_n10, assign43490_e58576_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43490_e58574: f64 = (locals.var_vgs - locals.var_vbsc);
        (assign43490_e58574, (-locals.var_vbsc_dn0), (-locals.var_vbsc_dn2), (-locals.var_vbsc_dn4), (locals.var_vgs_dn5 - locals.var_vbsc_dn5), (locals.var_vgs_dn6 - locals.var_vbsc_dn6), (locals.var_vgs_dn7 - locals.var_vbsc_dn7), (-locals.var_vbsc_dn8), (-locals.var_vbsc_dn9), (-locals.var_vbsc_dn10), (-locals.var_vbsc_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43490_e58576;
        locals.var_t1_dn0 = assign43490_e58576_d_n0;
        locals.var_t1_dn2 = assign43490_e58576_d_n2;
        locals.var_t1_dn4 = assign43490_e58576_d_n4;
        locals.var_t1_dn5 = assign43490_e58576_d_n5;
        locals.var_t1_dn6 = assign43490_e58576_d_n6;
        locals.var_t1_dn7 = assign43490_e58576_d_n7;
        locals.var_t1_dn8 = assign43490_e58576_d_n8;
        locals.var_t1_dn9 = assign43490_e58576_d_n9;
        locals.var_t1_dn10 = assign43490_e58576_d_n10;
        locals.var_t1_dn13 = assign43490_e58576_d_n13;
        locals.var_t1_rv = 0.0;

        let assign43500_e58580: f64 = 1.0;
        let assign43500_e58585: f64 = if ((locals.var_t1 < assign43500_e58580) && (1.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign43500_e58585;
        locals.var_guard1070_rv = 0.0;

        let (assign43510_e58600, assign43510_e58600_d_n0, assign43510_e58600_d_n2, assign43510_e58600_d_n4, assign43510_e58600_d_n5, assign43510_e58600_d_n6, assign43510_e58600_d_n7, assign43510_e58600_d_n8, assign43510_e58600_d_n9, assign43510_e58600_d_n10, assign43510_e58600_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43510_e58596: f64 = 1.0;
        let assign43510_e58598: f64 = (assign43510_e58596 - locals.var_t1);
        (assign43510_e58598, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign43510_e58600;
        locals.var_tmf1_dn0 = assign43510_e58600_d_n0;
        locals.var_tmf1_dn2 = assign43510_e58600_d_n2;
        locals.var_tmf1_dn4 = assign43510_e58600_d_n4;
        locals.var_tmf1_dn5 = assign43510_e58600_d_n5;
        locals.var_tmf1_dn6 = assign43510_e58600_d_n6;
        locals.var_tmf1_dn7 = assign43510_e58600_d_n7;
        locals.var_tmf1_dn8 = assign43510_e58600_d_n8;
        locals.var_tmf1_dn9 = assign43510_e58600_d_n9;
        locals.var_tmf1_dn10 = assign43510_e58600_d_n10;
        locals.var_tmf1_dn13 = assign43510_e58600_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign43520_e58613, assign43520_e58613_d_n0, assign43520_e58613_d_n2, assign43520_e58613_d_n4, assign43520_e58613_d_n5, assign43520_e58613_d_n6, assign43520_e58613_d_n7, assign43520_e58613_d_n8, assign43520_e58613_d_n9, assign43520_e58613_d_n10, assign43520_e58613_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43520_e58611: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign43520_e58611, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign43520_e58613;
        locals.var_x2_dn0 = assign43520_e58613_d_n0;
        locals.var_x2_dn2 = assign43520_e58613_d_n2;
        locals.var_x2_dn4 = assign43520_e58613_d_n4;
        locals.var_x2_dn5 = assign43520_e58613_d_n5;
        locals.var_x2_dn6 = assign43520_e58613_d_n6;
        locals.var_x2_dn7 = assign43520_e58613_d_n7;
        locals.var_x2_dn8 = assign43520_e58613_d_n8;
        locals.var_x2_dn9 = assign43520_e58613_d_n9;
        locals.var_x2_dn10 = assign43520_e58613_d_n10;
        locals.var_x2_dn13 = assign43520_e58613_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign43530_e58626, assign43530_e58626_d_n0, assign43530_e58626_d_n2, assign43530_e58626_d_n4, assign43530_e58626_d_n5, assign43530_e58626_d_n6, assign43530_e58626_d_n7, assign43530_e58626_d_n8, assign43530_e58626_d_n9, assign43530_e58626_d_n10, assign43530_e58626_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43530_e58624: f64 = 1.0;
        (assign43530_e58624, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign43530_e58626;
        locals.var_xmax2_dn0 = assign43530_e58626_d_n0;
        locals.var_xmax2_dn2 = assign43530_e58626_d_n2;
        locals.var_xmax2_dn4 = assign43530_e58626_d_n4;
        locals.var_xmax2_dn5 = assign43530_e58626_d_n5;
        locals.var_xmax2_dn6 = assign43530_e58626_d_n6;
        locals.var_xmax2_dn7 = assign43530_e58626_d_n7;
        locals.var_xmax2_dn8 = assign43530_e58626_d_n8;
        locals.var_xmax2_dn9 = assign43530_e58626_d_n9;
        locals.var_xmax2_dn10 = assign43530_e58626_d_n10;
        locals.var_xmax2_dn13 = assign43530_e58626_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign43540_e58637, assign43540_e58637_d_n0, assign43540_e58637_d_n2, assign43540_e58637_d_n4, assign43540_e58637_d_n5, assign43540_e58637_d_n6, assign43540_e58637_d_n7, assign43540_e58637_d_n8, assign43540_e58637_d_n9, assign43540_e58637_d_n10, assign43540_e58637_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign43540_e58637;
        locals.var_xp_dn0 = assign43540_e58637_d_n0;
        locals.var_xp_dn2 = assign43540_e58637_d_n2;
        locals.var_xp_dn4 = assign43540_e58637_d_n4;
        locals.var_xp_dn5 = assign43540_e58637_d_n5;
        locals.var_xp_dn6 = assign43540_e58637_d_n6;
        locals.var_xp_dn7 = assign43540_e58637_d_n7;
        locals.var_xp_dn8 = assign43540_e58637_d_n8;
        locals.var_xp_dn9 = assign43540_e58637_d_n9;
        locals.var_xp_dn10 = assign43540_e58637_d_n10;
        locals.var_xp_dn13 = assign43540_e58637_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign43550_e58648, assign43550_e58648_d_n0, assign43550_e58648_d_n2, assign43550_e58648_d_n4, assign43550_e58648_d_n5, assign43550_e58648_d_n6, assign43550_e58648_d_n7, assign43550_e58648_d_n8, assign43550_e58648_d_n9, assign43550_e58648_d_n10, assign43550_e58648_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign43550_e58648;
        locals.var_xmp_dn0 = assign43550_e58648_d_n0;
        locals.var_xmp_dn2 = assign43550_e58648_d_n2;
        locals.var_xmp_dn4 = assign43550_e58648_d_n4;
        locals.var_xmp_dn5 = assign43550_e58648_d_n5;
        locals.var_xmp_dn6 = assign43550_e58648_d_n6;
        locals.var_xmp_dn7 = assign43550_e58648_d_n7;
        locals.var_xmp_dn8 = assign43550_e58648_d_n8;
        locals.var_xmp_dn9 = assign43550_e58648_d_n9;
        locals.var_xmp_dn10 = assign43550_e58648_d_n10;
        locals.var_xmp_dn13 = assign43550_e58648_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign43560_e58659,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43560_e58659;
        locals.var_m0_rv = 0.0;

        let (assign43570_e58670,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43570_e58670;
        locals.var_mm_rv = 0.0;

        let (assign43580_e58681, assign43580_e58681_d_n0, assign43580_e58681_d_n2, assign43580_e58681_d_n4, assign43580_e58681_d_n5, assign43580_e58681_d_n6, assign43580_e58681_d_n7, assign43580_e58681_d_n8, assign43580_e58681_d_n9, assign43580_e58681_d_n10, assign43580_e58681_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign43580_e58681;
        locals.var_arg_dn0 = assign43580_e58681_d_n0;
        locals.var_arg_dn2 = assign43580_e58681_d_n2;
        locals.var_arg_dn4 = assign43580_e58681_d_n4;
        locals.var_arg_dn5 = assign43580_e58681_d_n5;
        locals.var_arg_dn6 = assign43580_e58681_d_n6;
        locals.var_arg_dn7 = assign43580_e58681_d_n7;
        locals.var_arg_dn8 = assign43580_e58681_d_n8;
        locals.var_arg_dn9 = assign43580_e58681_d_n9;
        locals.var_arg_dn10 = assign43580_e58681_d_n10;
        locals.var_arg_dn13 = assign43580_e58681_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign43590_e58692, assign43590_e58692_d_n0, assign43590_e58692_d_n2, assign43590_e58692_d_n4, assign43590_e58692_d_n5, assign43590_e58692_d_n6, assign43590_e58692_d_n7, assign43590_e58692_d_n8, assign43590_e58692_d_n9, assign43590_e58692_d_n10, assign43590_e58692_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43590_e58692;
        locals.var_dnm_dn0 = assign43590_e58692_d_n0;
        locals.var_dnm_dn2 = assign43590_e58692_d_n2;
        locals.var_dnm_dn4 = assign43590_e58692_d_n4;
        locals.var_dnm_dn5 = assign43590_e58692_d_n5;
        locals.var_dnm_dn6 = assign43590_e58692_d_n6;
        locals.var_dnm_dn7 = assign43590_e58692_d_n7;
        locals.var_dnm_dn8 = assign43590_e58692_d_n8;
        locals.var_dnm_dn9 = assign43590_e58692_d_n9;
        locals.var_dnm_dn10 = assign43590_e58692_d_n10;
        locals.var_dnm_dn13 = assign43590_e58692_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign43600_e58705, assign43600_e58705_d_n0, assign43600_e58705_d_n2, assign43600_e58705_d_n4, assign43600_e58705_d_n5, assign43600_e58705_d_n6, assign43600_e58705_d_n7, assign43600_e58705_d_n8, assign43600_e58705_d_n9, assign43600_e58705_d_n10, assign43600_e58705_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43600_e58703: f64 = (locals.var_xp * locals.var_x2);
        (assign43600_e58703, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign43600_e58705;
        locals.var_xp_dn0 = assign43600_e58705_d_n0;
        locals.var_xp_dn2 = assign43600_e58705_d_n2;
        locals.var_xp_dn4 = assign43600_e58705_d_n4;
        locals.var_xp_dn5 = assign43600_e58705_d_n5;
        locals.var_xp_dn6 = assign43600_e58705_d_n6;
        locals.var_xp_dn7 = assign43600_e58705_d_n7;
        locals.var_xp_dn8 = assign43600_e58705_d_n8;
        locals.var_xp_dn9 = assign43600_e58705_d_n9;
        locals.var_xp_dn10 = assign43600_e58705_d_n10;
        locals.var_xp_dn13 = assign43600_e58705_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign43610_e58718, assign43610_e58718_d_n0, assign43610_e58718_d_n2, assign43610_e58718_d_n4, assign43610_e58718_d_n5, assign43610_e58718_d_n6, assign43610_e58718_d_n7, assign43610_e58718_d_n8, assign43610_e58718_d_n9, assign43610_e58718_d_n10, assign43610_e58718_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43610_e58716: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43610_e58716, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign43610_e58718;
        locals.var_xmp_dn0 = assign43610_e58718_d_n0;
        locals.var_xmp_dn2 = assign43610_e58718_d_n2;
        locals.var_xmp_dn4 = assign43610_e58718_d_n4;
        locals.var_xmp_dn5 = assign43610_e58718_d_n5;
        locals.var_xmp_dn6 = assign43610_e58718_d_n6;
        locals.var_xmp_dn7 = assign43610_e58718_d_n7;
        locals.var_xmp_dn8 = assign43610_e58718_d_n8;
        locals.var_xmp_dn9 = assign43610_e58718_d_n9;
        locals.var_xmp_dn10 = assign43610_e58718_d_n10;
        locals.var_xmp_dn13 = assign43610_e58718_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_146(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign43620_e58731, assign43620_e58731_d_n0, assign43620_e58731_d_n2, assign43620_e58731_d_n4, assign43620_e58731_d_n5, assign43620_e58731_d_n6, assign43620_e58731_d_n7, assign43620_e58731_d_n8, assign43620_e58731_d_n9, assign43620_e58731_d_n10, assign43620_e58731_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43620_e58729: f64 = (locals.var_xp * locals.var_x2);
        (assign43620_e58729, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign43620_e58731;
        locals.var_xp_dn0 = assign43620_e58731_d_n0;
        locals.var_xp_dn2 = assign43620_e58731_d_n2;
        locals.var_xp_dn4 = assign43620_e58731_d_n4;
        locals.var_xp_dn5 = assign43620_e58731_d_n5;
        locals.var_xp_dn6 = assign43620_e58731_d_n6;
        locals.var_xp_dn7 = assign43620_e58731_d_n7;
        locals.var_xp_dn8 = assign43620_e58731_d_n8;
        locals.var_xp_dn9 = assign43620_e58731_d_n9;
        locals.var_xp_dn10 = assign43620_e58731_d_n10;
        locals.var_xp_dn13 = assign43620_e58731_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign43630_e58744, assign43630_e58744_d_n0, assign43630_e58744_d_n2, assign43630_e58744_d_n4, assign43630_e58744_d_n5, assign43630_e58744_d_n6, assign43630_e58744_d_n7, assign43630_e58744_d_n8, assign43630_e58744_d_n9, assign43630_e58744_d_n10, assign43630_e58744_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43630_e58742: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43630_e58742, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign43630_e58744;
        locals.var_xmp_dn0 = assign43630_e58744_d_n0;
        locals.var_xmp_dn2 = assign43630_e58744_d_n2;
        locals.var_xmp_dn4 = assign43630_e58744_d_n4;
        locals.var_xmp_dn5 = assign43630_e58744_d_n5;
        locals.var_xmp_dn6 = assign43630_e58744_d_n6;
        locals.var_xmp_dn7 = assign43630_e58744_d_n7;
        locals.var_xmp_dn8 = assign43630_e58744_d_n8;
        locals.var_xmp_dn9 = assign43630_e58744_d_n9;
        locals.var_xmp_dn10 = assign43630_e58744_d_n10;
        locals.var_xmp_dn13 = assign43630_e58744_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign43640_e58757, assign43640_e58757_d_n0, assign43640_e58757_d_n2, assign43640_e58757_d_n4, assign43640_e58757_d_n5, assign43640_e58757_d_n6, assign43640_e58757_d_n7, assign43640_e58757_d_n8, assign43640_e58757_d_n9, assign43640_e58757_d_n10, assign43640_e58757_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43640_e58755: f64 = (locals.var_xp + locals.var_xmp);
        (assign43640_e58755, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign43640_e58757;
        locals.var_arg_dn0 = assign43640_e58757_d_n0;
        locals.var_arg_dn2 = assign43640_e58757_d_n2;
        locals.var_arg_dn4 = assign43640_e58757_d_n4;
        locals.var_arg_dn5 = assign43640_e58757_d_n5;
        locals.var_arg_dn6 = assign43640_e58757_d_n6;
        locals.var_arg_dn7 = assign43640_e58757_d_n7;
        locals.var_arg_dn8 = assign43640_e58757_d_n8;
        locals.var_arg_dn9 = assign43640_e58757_d_n9;
        locals.var_arg_dn10 = assign43640_e58757_d_n10;
        locals.var_arg_dn13 = assign43640_e58757_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign43650_e58768, assign43650_e58768_d_n0, assign43650_e58768_d_n2, assign43650_e58768_d_n4, assign43650_e58768_d_n5, assign43650_e58768_d_n6, assign43650_e58768_d_n7, assign43650_e58768_d_n8, assign43650_e58768_d_n9, assign43650_e58768_d_n10, assign43650_e58768_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43650_e58768;
        locals.var_dnm_dn0 = assign43650_e58768_d_n0;
        locals.var_dnm_dn2 = assign43650_e58768_d_n2;
        locals.var_dnm_dn4 = assign43650_e58768_d_n4;
        locals.var_dnm_dn5 = assign43650_e58768_d_n5;
        locals.var_dnm_dn6 = assign43650_e58768_d_n6;
        locals.var_dnm_dn7 = assign43650_e58768_d_n7;
        locals.var_dnm_dn8 = assign43650_e58768_d_n8;
        locals.var_dnm_dn9 = assign43650_e58768_d_n9;
        locals.var_dnm_dn10 = assign43650_e58768_d_n10;
        locals.var_dnm_dn13 = assign43650_e58768_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign43660_e58783: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign43660_e58783;
        locals.var_guard1071_rv = 0.0;

        let assign43670_e58786: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign43670_e58786;
        locals.var_guard1072_rv = 0.0;

        let (assign43680_e58801,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43680_e58801;
        locals.var_mm_rv = 0.0;

        let assign43690_e58804: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign43690_e58804;
        locals.var_guard1073_rv = 0.0;

        let (assign43700_e58822,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) && (locals.var_guard1073 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43700_e58822;
        locals.var_mm_rv = 0.0;

        let assign43710_e58825: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign43710_e58825;
        locals.var_guard1074_rv = 0.0;

        let (assign43720_e58846,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1074 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43720_e58846;
        locals.var_mm_rv = 0.0;

        let assign43730_e58849: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign43730_e58849;
        locals.var_guard1075_rv = 0.0;

        let (assign43740_e58873,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1074 == 0.0)) && (locals.var_guard1075 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43740_e58873;
        locals.var_mm_rv = 0.0;

        let (assign43750_e58886,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43750_e58886;
        locals.var_m0_rv = 0.0;

        let mut assign43760_loop_guard: usize = 0;
        while {
            let assign43760_cond_e58900: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign43760_cond_e58900 != 0.0
        } {
            assign43760_loop_guard += 1;
            assert!(assign43760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign43760_body0_e58914, assign43760_body0_e58914_d_n0, assign43760_body0_e58914_d_n2, assign43760_body0_e58914_d_n4, assign43760_body0_e58914_d_n5, assign43760_body0_e58914_d_n6, assign43760_body0_e58914_d_n7, assign43760_body0_e58914_d_n8, assign43760_body0_e58914_d_n9, assign43760_body0_e58914_d_n10, assign43760_body0_e58914_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign43760_body0_e58912: f64 = (locals.var_dnm).sqrt();
        (assign43760_body0_e58912, (locals.var_dnm_dn0 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn2 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn4 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn5 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn6 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn7 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn8 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn9 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn10 / (2.0 * assign43760_body0_e58912)), (locals.var_dnm_dn13 / (2.0 * assign43760_body0_e58912)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign43760_body0_e58914;
            locals.var_dnm_dn0 = assign43760_body0_e58914_d_n0;
            locals.var_dnm_dn2 = assign43760_body0_e58914_d_n2;
            locals.var_dnm_dn4 = assign43760_body0_e58914_d_n4;
            locals.var_dnm_dn5 = assign43760_body0_e58914_d_n5;
            locals.var_dnm_dn6 = assign43760_body0_e58914_d_n6;
            locals.var_dnm_dn7 = assign43760_body0_e58914_d_n7;
            locals.var_dnm_dn8 = assign43760_body0_e58914_d_n8;
            locals.var_dnm_dn9 = assign43760_body0_e58914_d_n9;
            locals.var_dnm_dn10 = assign43760_body0_e58914_d_n10;
            locals.var_dnm_dn13 = assign43760_body0_e58914_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign43760_body1_e58929,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
        let assign43760_body1_e58927: f64 = (locals.var_m0 + 1.0);
        (assign43760_body1_e58927,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign43760_body1_e58929;
            locals.var_m0_rv = 0.0;
        }

        let (assign43770_e58954, assign43770_e58954_d_n0, assign43770_e58954_d_n2, assign43770_e58954_d_n4, assign43770_e58954_d_n5, assign43770_e58954_d_n6, assign43770_e58954_d_n7, assign43770_e58954_d_n8, assign43770_e58954_d_n9, assign43770_e58954_d_n10, assign43770_e58954_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 == 0.0)) {
        let (assign43770_e58952, assign43770_e58952_d_n0, assign43770_e58952_d_n2, assign43770_e58952_d_n4, assign43770_e58952_d_n5, assign43770_e58952_d_n6, assign43770_e58952_d_n7, assign43770_e58952_d_n8, assign43770_e58952_d_n9, assign43770_e58952_d_n10, assign43770_e58952_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43770_e58949: f64 = (2.0 * 2.0);
                let assign43770_e58950: f64 = (1.0 / assign43770_e58949);
                let assign43770_e58951: f64 = (locals.var_dnm).powf(assign43770_e58950);
                (assign43770_e58951, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43770_e58950) as f64).is_finite() && ((assign43770_e58950) as f64).fract() == 0.0 { if assign43770_e58950 == 0.0 { 0.0 } else { (assign43770_e58950 * ((locals.var_dnm).powf(assign43770_e58950 - 1.0) * locals.var_dnm_dn13)) } } else { (assign43770_e58951 * (assign43770_e58950 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign43770_e58952, assign43770_e58952_d_n0, assign43770_e58952_d_n2, assign43770_e58952_d_n4, assign43770_e58952_d_n5, assign43770_e58952_d_n6, assign43770_e58952_d_n7, assign43770_e58952_d_n8, assign43770_e58952_d_n9, assign43770_e58952_d_n10, assign43770_e58952_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43770_e58954;
        locals.var_dnm_dn0 = assign43770_e58954_d_n0;
        locals.var_dnm_dn2 = assign43770_e58954_d_n2;
        locals.var_dnm_dn4 = assign43770_e58954_d_n4;
        locals.var_dnm_dn5 = assign43770_e58954_d_n5;
        locals.var_dnm_dn6 = assign43770_e58954_d_n6;
        locals.var_dnm_dn7 = assign43770_e58954_d_n7;
        locals.var_dnm_dn8 = assign43770_e58954_d_n8;
        locals.var_dnm_dn9 = assign43770_e58954_d_n9;
        locals.var_dnm_dn10 = assign43770_e58954_d_n10;
        locals.var_dnm_dn13 = assign43770_e58954_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign43780_e58967, assign43780_e58967_d_n0, assign43780_e58967_d_n2, assign43780_e58967_d_n4, assign43780_e58967_d_n5, assign43780_e58967_d_n6, assign43780_e58967_d_n7, assign43780_e58967_d_n8, assign43780_e58967_d_n9, assign43780_e58967_d_n10, assign43780_e58967_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43780_e58965: f64 = (1.0 / locals.var_dnm);
        (assign43780_e58965, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign43780_e58967;
        locals.var_dnm_dn0 = assign43780_e58967_d_n0;
        locals.var_dnm_dn2 = assign43780_e58967_d_n2;
        locals.var_dnm_dn4 = assign43780_e58967_d_n4;
        locals.var_dnm_dn5 = assign43780_e58967_d_n5;
        locals.var_dnm_dn6 = assign43780_e58967_d_n6;
        locals.var_dnm_dn7 = assign43780_e58967_d_n7;
        locals.var_dnm_dn8 = assign43780_e58967_d_n8;
        locals.var_dnm_dn9 = assign43780_e58967_d_n9;
        locals.var_dnm_dn10 = assign43780_e58967_d_n10;
        locals.var_dnm_dn13 = assign43780_e58967_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign43790_e58982, assign43790_e58982_d_n0, assign43790_e58982_d_n2, assign43790_e58982_d_n4, assign43790_e58982_d_n5, assign43790_e58982_d_n6, assign43790_e58982_d_n7, assign43790_e58982_d_n8, assign43790_e58982_d_n9, assign43790_e58982_d_n10, assign43790_e58982_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43790_e58978: f64 = locals.var_tmf1;
        let assign43790_e58980: f64 = (assign43790_e58978 * locals.var_dnm);
        (assign43790_e58980, ((locals.var_tmf1_dn0 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn0)), ((locals.var_tmf1_dn2 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn2)), ((locals.var_tmf1_dn4 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn4)), ((locals.var_tmf1_dn5 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn5)), ((locals.var_tmf1_dn6 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn6)), ((locals.var_tmf1_dn7 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn7)), ((locals.var_tmf1_dn8 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn8)), ((locals.var_tmf1_dn9 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn9)), ((locals.var_tmf1_dn10 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn10)), ((locals.var_tmf1_dn13 * locals.var_dnm) + (assign43790_e58978 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign43790_e58982;
        locals.var_tmf0_dn0 = assign43790_e58982_d_n0;
        locals.var_tmf0_dn2 = assign43790_e58982_d_n2;
        locals.var_tmf0_dn4 = assign43790_e58982_d_n4;
        locals.var_tmf0_dn5 = assign43790_e58982_d_n5;
        locals.var_tmf0_dn6 = assign43790_e58982_d_n6;
        locals.var_tmf0_dn7 = assign43790_e58982_d_n7;
        locals.var_tmf0_dn8 = assign43790_e58982_d_n8;
        locals.var_tmf0_dn9 = assign43790_e58982_d_n9;
        locals.var_tmf0_dn10 = assign43790_e58982_d_n10;
        locals.var_tmf0_dn13 = assign43790_e58982_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign43800_e58999, assign43800_e58999_d_n0, assign43800_e58999_d_n2, assign43800_e58999_d_n4, assign43800_e58999_d_n5, assign43800_e58999_d_n6, assign43800_e58999_d_n7, assign43800_e58999_d_n8, assign43800_e58999_d_n9, assign43800_e58999_d_n10, assign43800_e58999_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43800_e58993: f64 = locals.var_xmp;
        let assign43800_e58995: f64 = (assign43800_e58993 * locals.var_dnm);
        let assign43800_e58997: f64 = (assign43800_e58995 / locals.var_arg);
        (assign43800_e58997, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn13 * locals.var_dnm) + (assign43800_e58993 * locals.var_dnm_dn13)) * locals.var_arg) - (assign43800_e58995 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43800_e58999;
        locals.var_t0_dn0 = assign43800_e58999_d_n0;
        locals.var_t0_dn2 = assign43800_e58999_d_n2;
        locals.var_t0_dn4 = assign43800_e58999_d_n4;
        locals.var_t0_dn5 = assign43800_e58999_d_n5;
        locals.var_t0_dn6 = assign43800_e58999_d_n6;
        locals.var_t0_dn7 = assign43800_e58999_d_n7;
        locals.var_t0_dn8 = assign43800_e58999_d_n8;
        locals.var_t0_dn9 = assign43800_e58999_d_n9;
        locals.var_t0_dn10 = assign43800_e58999_d_n10;
        locals.var_t0_dn13 = assign43800_e58999_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43810_e59014, assign43810_e59014_d_n0, assign43810_e59014_d_n2, assign43810_e59014_d_n4, assign43810_e59014_d_n5, assign43810_e59014_d_n6, assign43810_e59014_d_n7, assign43810_e59014_d_n8, assign43810_e59014_d_n9, assign43810_e59014_d_n10, assign43810_e59014_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        let assign43810_e59010: f64 = 1.0;
        let assign43810_e59012: f64 = (assign43810_e59010 - locals.var_tmf0);
        (assign43810_e59012, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43810_e59014;
        locals.var_t1_dn0 = assign43810_e59014_d_n0;
        locals.var_t1_dn2 = assign43810_e59014_d_n2;
        locals.var_t1_dn4 = assign43810_e59014_d_n4;
        locals.var_t1_dn5 = assign43810_e59014_d_n5;
        locals.var_t1_dn6 = assign43810_e59014_d_n6;
        locals.var_t1_dn7 = assign43810_e59014_d_n7;
        locals.var_t1_dn8 = assign43810_e59014_d_n8;
        locals.var_t1_dn9 = assign43810_e59014_d_n9;
        locals.var_t1_dn10 = assign43810_e59014_d_n10;
        locals.var_t1_dn13 = assign43810_e59014_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign43820_e59025, assign43820_e59025_d_n0, assign43820_e59025_d_n2, assign43820_e59025_d_n4, assign43820_e59025_d_n5, assign43820_e59025_d_n6, assign43820_e59025_d_n7, assign43820_e59025_d_n8, assign43820_e59025_d_n9, assign43820_e59025_d_n10, assign43820_e59025_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43820_e59025;
        locals.var_t0_dn0 = assign43820_e59025_d_n0;
        locals.var_t0_dn2 = assign43820_e59025_d_n2;
        locals.var_t0_dn4 = assign43820_e59025_d_n4;
        locals.var_t0_dn5 = assign43820_e59025_d_n5;
        locals.var_t0_dn6 = assign43820_e59025_d_n6;
        locals.var_t0_dn7 = assign43820_e59025_d_n7;
        locals.var_t0_dn8 = assign43820_e59025_d_n8;
        locals.var_t0_dn9 = assign43820_e59025_d_n9;
        locals.var_t0_dn10 = assign43820_e59025_d_n10;
        locals.var_t0_dn13 = assign43820_e59025_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43830_e59037, assign43830_e59037_d_n0, assign43830_e59037_d_n2, assign43830_e59037_d_n4, assign43830_e59037_d_n5, assign43830_e59037_d_n6, assign43830_e59037_d_n7, assign43830_e59037_d_n8, assign43830_e59037_d_n9, assign43830_e59037_d_n10, assign43830_e59037_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43830_e59037;
        locals.var_t1_dn0 = assign43830_e59037_d_n0;
        locals.var_t1_dn2 = assign43830_e59037_d_n2;
        locals.var_t1_dn4 = assign43830_e59037_d_n4;
        locals.var_t1_dn5 = assign43830_e59037_d_n5;
        locals.var_t1_dn6 = assign43830_e59037_d_n6;
        locals.var_t1_dn7 = assign43830_e59037_d_n7;
        locals.var_t1_dn8 = assign43830_e59037_d_n8;
        locals.var_t1_dn9 = assign43830_e59037_d_n9;
        locals.var_t1_dn10 = assign43830_e59037_d_n10;
        locals.var_t1_dn13 = assign43830_e59037_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign43840_e59049, assign43840_e59049_d_n0, assign43840_e59049_d_n2, assign43840_e59049_d_n4, assign43840_e59049_d_n5, assign43840_e59049_d_n6, assign43840_e59049_d_n7, assign43840_e59049_d_n8, assign43840_e59049_d_n9, assign43840_e59049_d_n10, assign43840_e59049_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1070 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign43840_e59049;
        locals.var_t0_dn0 = assign43840_e59049_d_n0;
        locals.var_t0_dn2 = assign43840_e59049_d_n2;
        locals.var_t0_dn4 = assign43840_e59049_d_n4;
        locals.var_t0_dn5 = assign43840_e59049_d_n5;
        locals.var_t0_dn6 = assign43840_e59049_d_n6;
        locals.var_t0_dn7 = assign43840_e59049_d_n7;
        locals.var_t0_dn8 = assign43840_e59049_d_n8;
        locals.var_t0_dn9 = assign43840_e59049_d_n9;
        locals.var_t0_dn10 = assign43840_e59049_d_n10;
        locals.var_t0_dn13 = assign43840_e59049_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign43850_e59060, assign43850_e59060_d_n0, assign43850_e59060_d_n2, assign43850_e59060_d_n4, assign43850_e59060_d_n5, assign43850_e59060_d_n6, assign43850_e59060_d_n7, assign43850_e59060_d_n8, assign43850_e59060_d_n9, assign43850_e59060_d_n10, assign43850_e59060_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43850_e59058: f64 = (locals.var_t1 / locals.var_uc_depthn);
        (assign43850_e59058, (((locals.var_t1_dn0 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn0)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn2 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn2)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn4 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn4)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn5 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn5)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn6 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn6)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn7 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn7)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn8 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn8)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn9 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn9)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn10 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn10)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn13 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn13)) / (locals.var_uc_depthn * locals.var_uc_depthn)),)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn13,)
    }
};
        locals.var_eeff = assign43850_e59060;
        locals.var_eeff_dn0 = assign43850_e59060_d_n0;
        locals.var_eeff_dn2 = assign43850_e59060_d_n2;
        locals.var_eeff_dn4 = assign43850_e59060_d_n4;
        locals.var_eeff_dn5 = assign43850_e59060_d_n5;
        locals.var_eeff_dn6 = assign43850_e59060_d_n6;
        locals.var_eeff_dn7 = assign43850_e59060_d_n7;
        locals.var_eeff_dn8 = assign43850_e59060_d_n8;
        locals.var_eeff_dn9 = assign43850_e59060_d_n9;
        locals.var_eeff_dn10 = assign43850_e59060_d_n10;
        locals.var_eeff_dn13 = assign43850_e59060_d_n13;
        locals.var_eeff_rv = 0.0;

        let (assign43860_e59078, assign43860_e59078_d_n0, assign43860_e59078_d_n2, assign43860_e59078_d_n4, assign43860_e59078_d_n5, assign43860_e59078_d_n6, assign43860_e59078_d_n7, assign43860_e59078_d_n8, assign43860_e59078_d_n9, assign43860_e59078_d_n10, assign43860_e59078_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let (assign43860_e59076, assign43860_e59076_d_n0, assign43860_e59076_d_n2, assign43860_e59076_d_n4, assign43860_e59076_d_n5, assign43860_e59076_d_n6, assign43860_e59076_d_n7, assign43860_e59076_d_n8, assign43860_e59076_d_n9, assign43860_e59076_d_n10, assign43860_e59076_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43860_e59074: f64 = (p.p353 - 1.0);
                let assign43860_e59075: f64 = (locals.var_eeff).powf(assign43860_e59074);
                (assign43860_e59075, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn0)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn2)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn4)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn5)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn6)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn7)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn8)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn9)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn10)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign43860_e59074) as f64).is_finite() && ((assign43860_e59074) as f64).fract() == 0.0 { if assign43860_e59074 == 0.0 { 0.0 } else { (assign43860_e59074 * ((locals.var_eeff).powf(assign43860_e59074 - 1.0) * locals.var_eeff_dn13)) } } else { (assign43860_e59075 * (assign43860_e59074 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign43860_e59076, assign43860_e59076_d_n0, assign43860_e59076_d_n2, assign43860_e59076_d_n4, assign43860_e59076_d_n5, assign43860_e59076_d_n6, assign43860_e59076_d_n7, assign43860_e59076_d_n8, assign43860_e59076_d_n9, assign43860_e59076_d_n10, assign43860_e59076_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign43860_e59078;
        locals.var_t5_dn0 = assign43860_e59078_d_n0;
        locals.var_t5_dn2 = assign43860_e59078_d_n2;
        locals.var_t5_dn4 = assign43860_e59078_d_n4;
        locals.var_t5_dn5 = assign43860_e59078_d_n5;
        locals.var_t5_dn6 = assign43860_e59078_d_n6;
        locals.var_t5_dn7 = assign43860_e59078_d_n7;
        locals.var_t5_dn8 = assign43860_e59078_d_n8;
        locals.var_t5_dn9 = assign43860_e59078_d_n9;
        locals.var_t5_dn10 = assign43860_e59078_d_n10;
        locals.var_t5_dn13 = assign43860_e59078_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign43870_e59089, assign43870_e59089_d_n0, assign43870_e59089_d_n2, assign43870_e59089_d_n4, assign43870_e59089_d_n5, assign43870_e59089_d_n6, assign43870_e59089_d_n7, assign43870_e59089_d_n8, assign43870_e59089_d_n9, assign43870_e59089_d_n10, assign43870_e59089_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43870_e59087: f64 = (locals.var_t5 * locals.var_eeff);
        (assign43870_e59087, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn13 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign43870_e59089;
        locals.var_t8_dn0 = assign43870_e59089_d_n0;
        locals.var_t8_dn2 = assign43870_e59089_d_n2;
        locals.var_t8_dn4 = assign43870_e59089_d_n4;
        locals.var_t8_dn5 = assign43870_e59089_d_n5;
        locals.var_t8_dn6 = assign43870_e59089_d_n6;
        locals.var_t8_dn7 = assign43870_e59089_d_n7;
        locals.var_t8_dn8 = assign43870_e59089_d_n8;
        locals.var_t8_dn9 = assign43870_e59089_d_n9;
        locals.var_t8_dn10 = assign43870_e59089_d_n10;
        locals.var_t8_dn13 = assign43870_e59089_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign43880_e59100, assign43880_e59100_d_n0, assign43880_e59100_d_n2, assign43880_e59100_d_n4, assign43880_e59100_d_n5, assign43880_e59100_d_n6, assign43880_e59100_d_n7, assign43880_e59100_d_n8, assign43880_e59100_d_n9, assign43880_e59100_d_n10, assign43880_e59100_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43880_e59098: f64 = (locals.var_uc_depmue0 + 1e-25);
        (assign43880_e59098, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign43880_e59100;
        locals.var_t2_dn0 = assign43880_e59100_d_n0;
        locals.var_t2_dn2 = assign43880_e59100_d_n2;
        locals.var_t2_dn4 = assign43880_e59100_d_n4;
        locals.var_t2_dn5 = assign43880_e59100_d_n5;
        locals.var_t2_dn6 = assign43880_e59100_d_n6;
        locals.var_t2_dn7 = assign43880_e59100_d_n7;
        locals.var_t2_dn8 = assign43880_e59100_d_n8;
        locals.var_t2_dn9 = assign43880_e59100_d_n9;
        locals.var_t2_dn10 = assign43880_e59100_d_n10;
        locals.var_t2_dn13 = assign43880_e59100_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign43890_e59115, assign43890_e59115_d_n0, assign43890_e59115_d_n2, assign43890_e59115_d_n4, assign43890_e59115_d_n5, assign43890_e59115_d_n6, assign43890_e59115_d_n7, assign43890_e59115_d_n8, assign43890_e59115_d_n9, assign43890_e59115_d_n10, assign43890_e59115_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43890_e59109: f64 = (1.0 / locals.var_t2);
        let assign43890_e59112: f64 = (locals.var_t8 / locals.var_uc_depmue2);
        let assign43890_e59113: f64 = (assign43890_e59109 + assign43890_e59112);
        (assign43890_e59113, ((-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn0 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn0)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn2 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn2)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn4 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn4)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn5 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn5)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn6 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn6)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn7 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn7)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn8 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn8)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn9 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn9)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn10 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn10)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))), ((-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))) + (((locals.var_t8_dn13 * locals.var_uc_depmue2) - (locals.var_t8 * locals.var_uc_depmue2_dn13)) / (locals.var_uc_depmue2 * locals.var_uc_depmue2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43890_e59115;
        locals.var_t1_dn0 = assign43890_e59115_d_n0;
        locals.var_t1_dn2 = assign43890_e59115_d_n2;
        locals.var_t1_dn4 = assign43890_e59115_d_n4;
        locals.var_t1_dn5 = assign43890_e59115_d_n5;
        locals.var_t1_dn6 = assign43890_e59115_d_n6;
        locals.var_t1_dn7 = assign43890_e59115_d_n7;
        locals.var_t1_dn8 = assign43890_e59115_d_n8;
        locals.var_t1_dn9 = assign43890_e59115_d_n9;
        locals.var_t1_dn10 = assign43890_e59115_d_n10;
        locals.var_t1_dn13 = assign43890_e59115_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign43900_e59126, assign43900_e59126_d_n0, assign43900_e59126_d_n2, assign43900_e59126_d_n4, assign43900_e59126_d_n5, assign43900_e59126_d_n6, assign43900_e59126_d_n7, assign43900_e59126_d_n8, assign43900_e59126_d_n9, assign43900_e59126_d_n10, assign43900_e59126_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43900_e59124: f64 = (1.0 / locals.var_t1);
        (assign43900_e59124, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign43900_e59126;
        locals.var_muun_dn0 = assign43900_e59126_d_n0;
        locals.var_muun_dn2 = assign43900_e59126_d_n2;
        locals.var_muun_dn4 = assign43900_e59126_d_n4;
        locals.var_muun_dn5 = assign43900_e59126_d_n5;
        locals.var_muun_dn6 = assign43900_e59126_d_n6;
        locals.var_muun_dn7 = assign43900_e59126_d_n7;
        locals.var_muun_dn8 = assign43900_e59126_d_n8;
        locals.var_muun_dn9 = assign43900_e59126_d_n9;
        locals.var_muun_dn10 = assign43900_e59126_d_n10;
        locals.var_muun_dn13 = assign43900_e59126_d_n13;
        locals.var_muun_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_147(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign43910_e59137, assign43910_e59137_d_n0, assign43910_e59137_d_n2, assign43910_e59137_d_n4, assign43910_e59137_d_n5, assign43910_e59137_d_n6, assign43910_e59137_d_n7, assign43910_e59137_d_n8, assign43910_e59137_d_n9, assign43910_e59137_d_n10, assign43910_e59137_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43910_e59135: f64 = (locals.var_muun / 10000.0);
        (assign43910_e59135, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn13 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign43910_e59137;
        locals.var_muun_dn0 = assign43910_e59137_d_n0;
        locals.var_muun_dn2 = assign43910_e59137_d_n2;
        locals.var_muun_dn4 = assign43910_e59137_d_n4;
        locals.var_muun_dn5 = assign43910_e59137_d_n5;
        locals.var_muun_dn6 = assign43910_e59137_d_n6;
        locals.var_muun_dn7 = assign43910_e59137_d_n7;
        locals.var_muun_dn8 = assign43910_e59137_d_n8;
        locals.var_muun_dn9 = assign43910_e59137_d_n9;
        locals.var_muun_dn10 = assign43910_e59137_d_n10;
        locals.var_muun_dn13 = assign43910_e59137_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign43920_e59148, assign43920_e59148_d_n0, assign43920_e59148_d_n2, assign43920_e59148_d_n4, assign43920_e59148_d_n5, assign43920_e59148_d_n6, assign43920_e59148_d_n7, assign43920_e59148_d_n8, assign43920_e59148_d_n9, assign43920_e59148_d_n10, assign43920_e59148_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43920_e59146: f64 = (locals.var_vds_res / locals.var_lch);
        (assign43920_e59146, (((locals.var_vds_res_dn0 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn2 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn4 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn5 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn6 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn7 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn8 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn9 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn10 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vds_res_dn13 * locals.var_lch) - (locals.var_vds_res * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk885, locals.var_edri__blk885_dn0, locals.var_edri__blk885_dn2, locals.var_edri__blk885_dn4, locals.var_edri__blk885_dn5, locals.var_edri__blk885_dn6, locals.var_edri__blk885_dn7, locals.var_edri__blk885_dn8, locals.var_edri__blk885_dn9, locals.var_edri__blk885_dn10, locals.var_edri__blk885_dn13,)
    }
};
        locals.var_edri__blk885 = assign43920_e59148;
        locals.var_edri__blk885_dn0 = assign43920_e59148_d_n0;
        locals.var_edri__blk885_dn2 = assign43920_e59148_d_n2;
        locals.var_edri__blk885_dn4 = assign43920_e59148_d_n4;
        locals.var_edri__blk885_dn5 = assign43920_e59148_d_n5;
        locals.var_edri__blk885_dn6 = assign43920_e59148_d_n6;
        locals.var_edri__blk885_dn7 = assign43920_e59148_d_n7;
        locals.var_edri__blk885_dn8 = assign43920_e59148_d_n8;
        locals.var_edri__blk885_dn9 = assign43920_e59148_d_n9;
        locals.var_edri__blk885_dn10 = assign43920_e59148_d_n10;
        locals.var_edri__blk885_dn13 = assign43920_e59148_d_n13;
        locals.var_edri__blk885_rv = 0.0;

        let (assign43930_e59159, assign43930_e59159_d_n0, assign43930_e59159_d_n2, assign43930_e59159_d_n4, assign43930_e59159_d_n5, assign43930_e59159_d_n6, assign43930_e59159_d_n7, assign43930_e59159_d_n8, assign43930_e59159_d_n9, assign43930_e59159_d_n10, assign43930_e59159_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43930_e59157: f64 = (locals.var_vds_res).powf(2.0);
        (assign43930_e59157, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn0)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn0 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn2)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn2 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn4)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn4 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn5)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn5 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn6)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn6 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn7)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn7 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn8)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn8 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn9)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn9 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn10)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn10 / locals.var_vds_res))) }, if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vds_res).powf(2.0 - 1.0) * locals.var_vds_res_dn13)) } } else { (assign43930_e59157 * (2.0 * (locals.var_vds_res_dn13 / locals.var_vds_res))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign43930_e59159;
        locals.var_tmf1_dn0 = assign43930_e59159_d_n0;
        locals.var_tmf1_dn2 = assign43930_e59159_d_n2;
        locals.var_tmf1_dn4 = assign43930_e59159_d_n4;
        locals.var_tmf1_dn5 = assign43930_e59159_d_n5;
        locals.var_tmf1_dn6 = assign43930_e59159_d_n6;
        locals.var_tmf1_dn7 = assign43930_e59159_d_n7;
        locals.var_tmf1_dn8 = assign43930_e59159_d_n8;
        locals.var_tmf1_dn9 = assign43930_e59159_d_n9;
        locals.var_tmf1_dn10 = assign43930_e59159_d_n10;
        locals.var_tmf1_dn13 = assign43930_e59159_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign43940_e59170, assign43940_e59170_d_n0, assign43940_e59170_d_n2, assign43940_e59170_d_n4, assign43940_e59170_d_n5, assign43940_e59170_d_n6, assign43940_e59170_d_n7, assign43940_e59170_d_n8, assign43940_e59170_d_n9, assign43940_e59170_d_n10, assign43940_e59170_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43940_e59168: f64 = (0.1_f64).powf(2.0);
        (assign43940_e59168, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign43940_e59170;
        locals.var_tmf2_dn0 = assign43940_e59170_d_n0;
        locals.var_tmf2_dn2 = assign43940_e59170_d_n2;
        locals.var_tmf2_dn4 = assign43940_e59170_d_n4;
        locals.var_tmf2_dn5 = assign43940_e59170_d_n5;
        locals.var_tmf2_dn6 = assign43940_e59170_d_n6;
        locals.var_tmf2_dn7 = assign43940_e59170_d_n7;
        locals.var_tmf2_dn8 = assign43940_e59170_d_n8;
        locals.var_tmf2_dn9 = assign43940_e59170_d_n9;
        locals.var_tmf2_dn10 = assign43940_e59170_d_n10;
        locals.var_tmf2_dn13 = assign43940_e59170_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign43950_e59191, assign43950_e59191_d_n0, assign43950_e59191_d_n2, assign43950_e59191_d_n4, assign43950_e59191_d_n5, assign43950_e59191_d_n6, assign43950_e59191_d_n7, assign43950_e59191_d_n8, assign43950_e59191_d_n9, assign43950_e59191_d_n10, assign43950_e59191_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43950_e59179: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign43950_e59182: f64 = (1.0 / 2.0);
        let assign43950_e59183: f64 = (assign43950_e59179).powf(assign43950_e59182);
        let assign43950_e59187: f64 = (1.0 / 2.0);
        let assign43950_e59188: f64 = (locals.var_tmf2).powf(assign43950_e59187);
        let assign43950_e59189: f64 = (assign43950_e59183 - assign43950_e59188);
        (assign43950_e59189, (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn0)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn0 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn2)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn2 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn4)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn4 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn5)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn5 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn6)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn6 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn7)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn7 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn8)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn8 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn9)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn9 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn10)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn10 / locals.var_tmf2))) }), (if 0.0 == 0.0 && ((assign43950_e59182) as f64).is_finite() && ((assign43950_e59182) as f64).fract() == 0.0 { if assign43950_e59182 == 0.0 { 0.0 } else { (assign43950_e59182 * ((assign43950_e59179).powf(assign43950_e59182 - 1.0) * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))) } } else { (assign43950_e59183 * (assign43950_e59182 * ((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) / assign43950_e59179))) } - if 0.0 == 0.0 && ((assign43950_e59187) as f64).is_finite() && ((assign43950_e59187) as f64).fract() == 0.0 { if assign43950_e59187 == 0.0 { 0.0 } else { (assign43950_e59187 * ((locals.var_tmf2).powf(assign43950_e59187 - 1.0) * locals.var_tmf2_dn13)) } } else { (assign43950_e59188 * (assign43950_e59187 * (locals.var_tmf2_dn13 / locals.var_tmf2))) }),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43950_e59191;
        locals.var_t1_dn0 = assign43950_e59191_d_n0;
        locals.var_t1_dn2 = assign43950_e59191_d_n2;
        locals.var_t1_dn4 = assign43950_e59191_d_n4;
        locals.var_t1_dn5 = assign43950_e59191_d_n5;
        locals.var_t1_dn6 = assign43950_e59191_d_n6;
        locals.var_t1_dn7 = assign43950_e59191_d_n7;
        locals.var_t1_dn8 = assign43950_e59191_d_n8;
        locals.var_t1_dn9 = assign43950_e59191_d_n9;
        locals.var_t1_dn10 = assign43950_e59191_d_n10;
        locals.var_t1_dn13 = assign43950_e59191_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign43960_e59202, assign43960_e59202_d_n0, assign43960_e59202_d_n2, assign43960_e59202_d_n4, assign43960_e59202_d_n5, assign43960_e59202_d_n6, assign43960_e59202_d_n7, assign43960_e59202_d_n8, assign43960_e59202_d_n9, assign43960_e59202_d_n10, assign43960_e59202_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43960_e59200: f64 = (locals.var_t1 / locals.var_lch);
        (assign43960_e59200, (((locals.var_t1_dn0 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn2 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn4 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn5 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn6 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn7 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn8 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn9 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn10 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_t1_dn13 * locals.var_lch) - (locals.var_t1 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43960_e59202;
        locals.var_t1_dn0 = assign43960_e59202_d_n0;
        locals.var_t1_dn2 = assign43960_e59202_d_n2;
        locals.var_t1_dn4 = assign43960_e59202_d_n4;
        locals.var_t1_dn5 = assign43960_e59202_d_n5;
        locals.var_t1_dn6 = assign43960_e59202_d_n6;
        locals.var_t1_dn7 = assign43960_e59202_d_n7;
        locals.var_t1_dn8 = assign43960_e59202_d_n8;
        locals.var_t1_dn9 = assign43960_e59202_d_n9;
        locals.var_t1_dn10 = assign43960_e59202_d_n10;
        locals.var_t1_dn13 = assign43960_e59202_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign43970_e59215, assign43970_e59215_d_n0, assign43970_e59215_d_n2, assign43970_e59215_d_n4, assign43970_e59215_d_n5, assign43970_e59215_d_n6, assign43970_e59215_d_n7, assign43970_e59215_d_n8, assign43970_e59215_d_n9, assign43970_e59215_d_n10, assign43970_e59215_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43970_e59211: f64 = (locals.var_muun * locals.var_t1);
        let assign43970_e59213: f64 = (assign43970_e59211 / locals.var_uc_depvmax);
        (assign43970_e59213, (((((locals.var_muun_dn0 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn0)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn2)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn4)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn5)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn6)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn7)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn8)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn9)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn10)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn13 * locals.var_t1) + (locals.var_muun * locals.var_t1_dn13)) * locals.var_uc_depvmax) - (assign43970_e59211 * locals.var_uc_depvmax_dn13)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign43970_e59215;
        locals.var_t1_dn0 = assign43970_e59215_d_n0;
        locals.var_t1_dn2 = assign43970_e59215_d_n2;
        locals.var_t1_dn4 = assign43970_e59215_d_n4;
        locals.var_t1_dn5 = assign43970_e59215_d_n5;
        locals.var_t1_dn6 = assign43970_e59215_d_n6;
        locals.var_t1_dn7 = assign43970_e59215_d_n7;
        locals.var_t1_dn8 = assign43970_e59215_d_n8;
        locals.var_t1_dn9 = assign43970_e59215_d_n9;
        locals.var_t1_dn10 = assign43970_e59215_d_n10;
        locals.var_t1_dn13 = assign43970_e59215_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign43980_e59231, assign43980_e59231_d_n0, assign43980_e59231_d_n2, assign43980_e59231_d_n4, assign43980_e59231_d_n5, assign43980_e59231_d_n6, assign43980_e59231_d_n7, assign43980_e59231_d_n8, assign43980_e59231_d_n9, assign43980_e59231_d_n10, assign43980_e59231_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let (assign43980_e59229, assign43980_e59229_d_n0, assign43980_e59229_d_n2, assign43980_e59229_d_n4, assign43980_e59229_d_n5, assign43980_e59229_d_n6, assign43980_e59229_d_n7, assign43980_e59229_d_n8, assign43980_e59229_d_n9, assign43980_e59229_d_n10, assign43980_e59229_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43980_e59228: f64 = (locals.var_t1).powf(p.p378);
                (assign43980_e59228, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn13)) } } else { (assign43980_e59228 * (p.p378 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign43980_e59229, assign43980_e59229_d_n0, assign43980_e59229_d_n2, assign43980_e59229_d_n4, assign43980_e59229_d_n5, assign43980_e59229_d_n6, assign43980_e59229_d_n7, assign43980_e59229_d_n8, assign43980_e59229_d_n9, assign43980_e59229_d_n10, assign43980_e59229_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign43980_e59231;
        locals.var_t2_dn0 = assign43980_e59231_d_n0;
        locals.var_t2_dn2 = assign43980_e59231_d_n2;
        locals.var_t2_dn4 = assign43980_e59231_d_n4;
        locals.var_t2_dn5 = assign43980_e59231_d_n5;
        locals.var_t2_dn6 = assign43980_e59231_d_n6;
        locals.var_t2_dn7 = assign43980_e59231_d_n7;
        locals.var_t2_dn8 = assign43980_e59231_d_n8;
        locals.var_t2_dn9 = assign43980_e59231_d_n9;
        locals.var_t2_dn10 = assign43980_e59231_d_n10;
        locals.var_t2_dn13 = assign43980_e59231_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign43990_e59242, assign43990_e59242_d_n0, assign43990_e59242_d_n2, assign43990_e59242_d_n4, assign43990_e59242_d_n5, assign43990_e59242_d_n6, assign43990_e59242_d_n7, assign43990_e59242_d_n8, assign43990_e59242_d_n9, assign43990_e59242_d_n10, assign43990_e59242_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign43990_e59240: f64 = (1.0 + locals.var_t2);
        (assign43990_e59240, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign43990_e59242;
        locals.var_t3_dn0 = assign43990_e59242_d_n0;
        locals.var_t3_dn2 = assign43990_e59242_d_n2;
        locals.var_t3_dn4 = assign43990_e59242_d_n4;
        locals.var_t3_dn5 = assign43990_e59242_d_n5;
        locals.var_t3_dn6 = assign43990_e59242_d_n6;
        locals.var_t3_dn7 = assign43990_e59242_d_n7;
        locals.var_t3_dn8 = assign43990_e59242_d_n8;
        locals.var_t3_dn9 = assign43990_e59242_d_n9;
        locals.var_t3_dn10 = assign43990_e59242_d_n10;
        locals.var_t3_dn13 = assign43990_e59242_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign44000_e59260, assign44000_e59260_d_n0, assign44000_e59260_d_n2, assign44000_e59260_d_n4, assign44000_e59260_d_n5, assign44000_e59260_d_n6, assign44000_e59260_d_n7, assign44000_e59260_d_n8, assign44000_e59260_d_n9, assign44000_e59260_d_n10, assign44000_e59260_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let (assign44000_e59258, assign44000_e59258_d_n0, assign44000_e59258_d_n2, assign44000_e59258_d_n4, assign44000_e59258_d_n5, assign44000_e59258_d_n6, assign44000_e59258_d_n7, assign44000_e59258_d_n8, assign44000_e59258_d_n9, assign44000_e59258_d_n10, assign44000_e59258_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44000_e59256: f64 = (1.0 / p.p378);
                let assign44000_e59257: f64 = (locals.var_t3).powf(assign44000_e59256);
                (assign44000_e59257, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn0)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn2)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn4)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn5)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn6)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn7)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn8)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn9)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn10)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign44000_e59256) as f64).is_finite() && ((assign44000_e59256) as f64).fract() == 0.0 { if assign44000_e59256 == 0.0 { 0.0 } else { (assign44000_e59256 * ((locals.var_t3).powf(assign44000_e59256 - 1.0) * locals.var_t3_dn13)) } } else { (assign44000_e59257 * (assign44000_e59256 * (locals.var_t3_dn13 / locals.var_t3))) },)
            }
        };
        (assign44000_e59258, assign44000_e59258_d_n0, assign44000_e59258_d_n2, assign44000_e59258_d_n4, assign44000_e59258_d_n5, assign44000_e59258_d_n6, assign44000_e59258_d_n7, assign44000_e59258_d_n8, assign44000_e59258_d_n9, assign44000_e59258_d_n10, assign44000_e59258_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign44000_e59260;
        locals.var_t4_dn0 = assign44000_e59260_d_n0;
        locals.var_t4_dn2 = assign44000_e59260_d_n2;
        locals.var_t4_dn4 = assign44000_e59260_d_n4;
        locals.var_t4_dn5 = assign44000_e59260_d_n5;
        locals.var_t4_dn6 = assign44000_e59260_d_n6;
        locals.var_t4_dn7 = assign44000_e59260_d_n7;
        locals.var_t4_dn8 = assign44000_e59260_d_n8;
        locals.var_t4_dn9 = assign44000_e59260_d_n9;
        locals.var_t4_dn10 = assign44000_e59260_d_n10;
        locals.var_t4_dn13 = assign44000_e59260_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign44010_e59271, assign44010_e59271_d_n0, assign44010_e59271_d_n2, assign44010_e59271_d_n4, assign44010_e59271_d_n5, assign44010_e59271_d_n6, assign44010_e59271_d_n7, assign44010_e59271_d_n8, assign44010_e59271_d_n9, assign44010_e59271_d_n10, assign44010_e59271_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44010_e59269: f64 = (locals.var_muun / locals.var_t4);
        (assign44010_e59269, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn13 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res, locals.var_mu_res_dn0, locals.var_mu_res_dn2, locals.var_mu_res_dn4, locals.var_mu_res_dn5, locals.var_mu_res_dn6, locals.var_mu_res_dn7, locals.var_mu_res_dn8, locals.var_mu_res_dn9, locals.var_mu_res_dn10, locals.var_mu_res_dn13,)
    }
};
        locals.var_mu_res = assign44010_e59271;
        locals.var_mu_res_dn0 = assign44010_e59271_d_n0;
        locals.var_mu_res_dn2 = assign44010_e59271_d_n2;
        locals.var_mu_res_dn4 = assign44010_e59271_d_n4;
        locals.var_mu_res_dn5 = assign44010_e59271_d_n5;
        locals.var_mu_res_dn6 = assign44010_e59271_d_n6;
        locals.var_mu_res_dn7 = assign44010_e59271_d_n7;
        locals.var_mu_res_dn8 = assign44010_e59271_d_n8;
        locals.var_mu_res_dn9 = assign44010_e59271_d_n9;
        locals.var_mu_res_dn10 = assign44010_e59271_d_n10;
        locals.var_mu_res_dn13 = assign44010_e59271_d_n13;
        locals.var_mu_res_rv = 0.0;

        let (assign44020_e59285, assign44020_e59285_d_n0, assign44020_e59285_d_n2, assign44020_e59285_d_n4, assign44020_e59285_d_n5, assign44020_e59285_d_n6, assign44020_e59285_d_n7, assign44020_e59285_d_n8, assign44020_e59285_d_n9, assign44020_e59285_d_n10, assign44020_e59285_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44020_e59279: f64 = (-locals.var_w_res);
        let assign44020_e59281: f64 = (assign44020_e59279 * 1.6021918e-19);
        let assign44020_e59283: f64 = (assign44020_e59281 * locals.var_uc_ndepm);
        (assign44020_e59283, ((((-locals.var_w_res_dn0) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn0)), ((((-locals.var_w_res_dn2) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn2)), ((((-locals.var_w_res_dn4) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn4)), ((((-locals.var_w_res_dn5) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn5)), ((((-locals.var_w_res_dn6) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn6)), ((((-locals.var_w_res_dn7) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn7)), ((((-locals.var_w_res_dn8) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn8)), ((((-locals.var_w_res_dn9) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn9)), ((((-locals.var_w_res_dn10) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn10)), ((((-locals.var_w_res_dn13) * 1.6021918e-19) * locals.var_uc_ndepm) + (assign44020_e59281 * locals.var_uc_ndepm_dn13)),)
    } else {
        (locals.var_qn_res, locals.var_qn_res_dn0, locals.var_qn_res_dn2, locals.var_qn_res_dn4, locals.var_qn_res_dn5, locals.var_qn_res_dn6, locals.var_qn_res_dn7, locals.var_qn_res_dn8, locals.var_qn_res_dn9, locals.var_qn_res_dn10, locals.var_qn_res_dn13,)
    }
};
        locals.var_qn_res = assign44020_e59285;
        locals.var_qn_res_dn0 = assign44020_e59285_d_n0;
        locals.var_qn_res_dn2 = assign44020_e59285_d_n2;
        locals.var_qn_res_dn4 = assign44020_e59285_d_n4;
        locals.var_qn_res_dn5 = assign44020_e59285_d_n5;
        locals.var_qn_res_dn6 = assign44020_e59285_d_n6;
        locals.var_qn_res_dn7 = assign44020_e59285_d_n7;
        locals.var_qn_res_dn8 = assign44020_e59285_d_n8;
        locals.var_qn_res_dn9 = assign44020_e59285_d_n9;
        locals.var_qn_res_dn10 = assign44020_e59285_d_n10;
        locals.var_qn_res_dn13 = assign44020_e59285_d_n13;
        locals.var_qn_res_rv = 0.0;

        let (assign44030_e59301, assign44030_e59301_d_n0, assign44030_e59301_d_n2, assign44030_e59301_d_n4, assign44030_e59301_d_n5, assign44030_e59301_d_n6, assign44030_e59301_d_n7, assign44030_e59301_d_n8, assign44030_e59301_d_n9, assign44030_e59301_d_n10, assign44030_e59301_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44030_e59294: f64 = (-locals.var_qn_res);
        let assign44030_e59295: f64 = (locals.var_weff_nf * assign44030_e59294);
        let assign44030_e59297: f64 = (assign44030_e59295 * locals.var_mu_res);
        let assign44030_e59299: f64 = (assign44030_e59297 * locals.var_edri__blk885);
        (assign44030_e59299, (((((locals.var_weff_nf * (-locals.var_qn_res_dn0)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn0)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn2)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn2)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn4)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn4)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn5)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn5)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn6)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn6)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn7)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn7)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn8)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn8)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn9)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn9)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn10)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn10)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_res_dn13)) * locals.var_mu_res) + (assign44030_e59295 * locals.var_mu_res_dn13)) * locals.var_edri__blk885) + (assign44030_e59297 * locals.var_edri__blk885_dn13)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn13,)
    }
};
        locals.var_ids_res = assign44030_e59301;
        locals.var_ids_res_dn0 = assign44030_e59301_d_n0;
        locals.var_ids_res_dn2 = assign44030_e59301_d_n2;
        locals.var_ids_res_dn4 = assign44030_e59301_d_n4;
        locals.var_ids_res_dn5 = assign44030_e59301_d_n5;
        locals.var_ids_res_dn6 = assign44030_e59301_d_n6;
        locals.var_ids_res_dn7 = assign44030_e59301_d_n7;
        locals.var_ids_res_dn8 = assign44030_e59301_d_n8;
        locals.var_ids_res_dn9 = assign44030_e59301_d_n9;
        locals.var_ids_res_dn10 = assign44030_e59301_d_n10;
        locals.var_ids_res_dn13 = assign44030_e59301_d_n13;
        locals.var_ids_res_rv = 0.0;

        let (assign44040_e59314, assign44040_e59314_d_n0, assign44040_e59314_d_n2, assign44040_e59314_d_n4, assign44040_e59314_d_n5, assign44040_e59314_d_n6, assign44040_e59314_d_n7, assign44040_e59314_d_n8, assign44040_e59314_d_n9, assign44040_e59314_d_n10, assign44040_e59314_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44040_e59310: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign44040_e59312: f64 = (assign44040_e59310 / locals.var_lch);
        (assign44040_e59312, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn13) * locals.var_lch) - (assign44040_e59310 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn13,)
    }
};
        locals.var_betawl = assign44040_e59314;
        locals.var_betawl_dn0 = assign44040_e59314_d_n0;
        locals.var_betawl_dn2 = assign44040_e59314_d_n2;
        locals.var_betawl_dn4 = assign44040_e59314_d_n4;
        locals.var_betawl_dn5 = assign44040_e59314_d_n5;
        locals.var_betawl_dn6 = assign44040_e59314_d_n6;
        locals.var_betawl_dn7 = assign44040_e59314_d_n7;
        locals.var_betawl_dn8 = assign44040_e59314_d_n8;
        locals.var_betawl_dn9 = assign44040_e59314_d_n9;
        locals.var_betawl_dn10 = assign44040_e59314_d_n10;
        locals.var_betawl_dn13 = assign44040_e59314_d_n13;
        locals.var_betawl_rv = 0.0;

        let (assign44050_e59327, assign44050_e59327_d_n0, assign44050_e59327_d_n2, assign44050_e59327_d_n4, assign44050_e59327_d_n5, assign44050_e59327_d_n6, assign44050_e59327_d_n7, assign44050_e59327_d_n8, assign44050_e59327_d_n9, assign44050_e59327_d_n10, assign44050_e59327_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44050_e59323: f64 = (locals.var_betawl * locals.var_idd);
        let assign44050_e59325: f64 = (assign44050_e59323 * locals.var_mu_acc);
        (assign44050_e59325, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn10)), ((((locals.var_betawl_dn13 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn13)) * locals.var_mu_acc) + (assign44050_e59323 * locals.var_mu_acc_dn13)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn13,)
    }
};
        locals.var_ids_acc = assign44050_e59327;
        locals.var_ids_acc_dn0 = assign44050_e59327_d_n0;
        locals.var_ids_acc_dn2 = assign44050_e59327_d_n2;
        locals.var_ids_acc_dn4 = assign44050_e59327_d_n4;
        locals.var_ids_acc_dn5 = assign44050_e59327_d_n5;
        locals.var_ids_acc_dn6 = assign44050_e59327_d_n6;
        locals.var_ids_acc_dn7 = assign44050_e59327_d_n7;
        locals.var_ids_acc_dn8 = assign44050_e59327_d_n8;
        locals.var_ids_acc_dn9 = assign44050_e59327_d_n9;
        locals.var_ids_acc_dn10 = assign44050_e59327_d_n10;
        locals.var_ids_acc_dn13 = assign44050_e59327_d_n13;
        locals.var_ids_acc_rv = 0.0;

        let (assign44060_e59338, assign44060_e59338_d_n0, assign44060_e59338_d_n2, assign44060_e59338_d_n4, assign44060_e59338_d_n5, assign44060_e59338_d_n6, assign44060_e59338_d_n7, assign44060_e59338_d_n8, assign44060_e59338_d_n9, assign44060_e59338_d_n10, assign44060_e59338_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44060_e59336: f64 = (locals.var_ids_acc + locals.var_ids_res);
        (assign44060_e59336, (locals.var_ids_acc_dn0 + locals.var_ids_res_dn0), (locals.var_ids_acc_dn2 + locals.var_ids_res_dn2), (locals.var_ids_acc_dn4 + locals.var_ids_res_dn4), (locals.var_ids_acc_dn5 + locals.var_ids_res_dn5), (locals.var_ids_acc_dn6 + locals.var_ids_res_dn6), (locals.var_ids_acc_dn7 + locals.var_ids_res_dn7), (locals.var_ids_acc_dn8 + locals.var_ids_res_dn8), (locals.var_ids_acc_dn9 + locals.var_ids_res_dn9), (locals.var_ids_acc_dn10 + locals.var_ids_res_dn10), (locals.var_ids_acc_dn13 + locals.var_ids_res_dn13),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign44060_e59338;
        locals.var_ids0_dn0 = assign44060_e59338_d_n0;
        locals.var_ids0_dn2 = assign44060_e59338_d_n2;
        locals.var_ids0_dn4 = assign44060_e59338_d_n4;
        locals.var_ids0_dn5 = assign44060_e59338_d_n5;
        locals.var_ids0_dn6 = assign44060_e59338_d_n6;
        locals.var_ids0_dn7 = assign44060_e59338_d_n7;
        locals.var_ids0_dn8 = assign44060_e59338_d_n8;
        locals.var_ids0_dn9 = assign44060_e59338_d_n9;
        locals.var_ids0_dn10 = assign44060_e59338_d_n10;
        locals.var_ids0_dn13 = assign44060_e59338_d_n13;
        locals.var_ids0_rv = 0.0;

        let (assign44070_e59347, assign44070_e59347_d_n0, assign44070_e59347_d_n2, assign44070_e59347_d_n4, assign44070_e59347_d_n5, assign44070_e59347_d_n6, assign44070_e59347_d_n7, assign44070_e59347_d_n8, assign44070_e59347_d_n9, assign44070_e59347_d_n10, assign44070_e59347_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign44070_e59347;
        locals.var_vds_dn0 = assign44070_e59347_d_n0;
        locals.var_vds_dn2 = assign44070_e59347_d_n2;
        locals.var_vds_dn4 = assign44070_e59347_d_n4;
        locals.var_vds_dn5 = assign44070_e59347_d_n5;
        locals.var_vds_dn6 = assign44070_e59347_d_n6;
        locals.var_vds_dn7 = assign44070_e59347_d_n7;
        locals.var_vds_dn8 = assign44070_e59347_d_n8;
        locals.var_vds_dn9 = assign44070_e59347_d_n9;
        locals.var_vds_dn10 = assign44070_e59347_d_n10;
        locals.var_vds_dn13 = assign44070_e59347_d_n13;
        locals.var_vds_rv = 0.0;

        let assign44080_e59350: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign44080_e59350;
        locals.var_guard1076_rv = 0.0;

        let (assign44090_e59365, assign44090_e59365_d_n0, assign44090_e59365_d_n2, assign44090_e59365_d_n4, assign44090_e59365_d_n5, assign44090_e59365_d_n6, assign44090_e59365_d_n7, assign44090_e59365_d_n8, assign44090_e59365_d_n9, assign44090_e59365_d_n10, assign44090_e59365_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44090_e59362: f64 = (locals.var_vds - locals.var_pds);
        let assign44090_e59363: f64 = (0.5 * assign44090_e59362);
        (assign44090_e59363, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign44090_e59365;
        locals.var_t1_dn0 = assign44090_e59365_d_n0;
        locals.var_t1_dn2 = assign44090_e59365_d_n2;
        locals.var_t1_dn4 = assign44090_e59365_d_n4;
        locals.var_t1_dn5 = assign44090_e59365_d_n5;
        locals.var_t1_dn6 = assign44090_e59365_d_n6;
        locals.var_t1_dn7 = assign44090_e59365_d_n7;
        locals.var_t1_dn8 = assign44090_e59365_d_n8;
        locals.var_t1_dn9 = assign44090_e59365_d_n9;
        locals.var_t1_dn10 = assign44090_e59365_d_n10;
        locals.var_t1_dn13 = assign44090_e59365_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign44100_e59380, assign44100_e59380_d_n0, assign44100_e59380_d_n2, assign44100_e59380_d_n4, assign44100_e59380_d_n5, assign44100_e59380_d_n6, assign44100_e59380_d_n7, assign44100_e59380_d_n8, assign44100_e59380_d_n9, assign44100_e59380_d_n10, assign44100_e59380_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44100_e59376: f64 = (2.0 * locals.var_t1);
        let assign44100_e59378: f64 = (assign44100_e59376 / 0.01);
        (assign44100_e59378, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn13) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign44100_e59380;
        locals.var_tmf1_dn0 = assign44100_e59380_d_n0;
        locals.var_tmf1_dn2 = assign44100_e59380_d_n2;
        locals.var_tmf1_dn4 = assign44100_e59380_d_n4;
        locals.var_tmf1_dn5 = assign44100_e59380_d_n5;
        locals.var_tmf1_dn6 = assign44100_e59380_d_n6;
        locals.var_tmf1_dn7 = assign44100_e59380_d_n7;
        locals.var_tmf1_dn8 = assign44100_e59380_d_n8;
        locals.var_tmf1_dn9 = assign44100_e59380_d_n9;
        locals.var_tmf1_dn10 = assign44100_e59380_d_n10;
        locals.var_tmf1_dn13 = assign44100_e59380_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign44110_e59427, assign44110_e59427_d_n0, assign44110_e59427_d_n2, assign44110_e59427_d_n4, assign44110_e59427_d_n5, assign44110_e59427_d_n6, assign44110_e59427_d_n7, assign44110_e59427_d_n8, assign44110_e59427_d_n9, assign44110_e59427_d_n10, assign44110_e59427_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44110_e59393: f64 = (1.0 / 2.0);
        let assign44110_e59397: f64 = (1.0 / 6.0);
        let assign44110_e59401: f64 = (1.0 / 24.0);
        let assign44110_e59405: f64 = (1.0 / 120.0);
        let assign44110_e59409: f64 = (1.0 / 720.0);
        let assign44110_e59413: f64 = (1.0 / 5040.0);
        let assign44110_e59414: f64 = (locals.var_tmf1 * assign44110_e59413);
        let assign44110_e59415: f64 = (assign44110_e59409 + assign44110_e59414);
        let assign44110_e59416: f64 = (locals.var_tmf1 * assign44110_e59415);
        let assign44110_e59417: f64 = (assign44110_e59405 + assign44110_e59416);
        let assign44110_e59418: f64 = (locals.var_tmf1 * assign44110_e59417);
        let assign44110_e59419: f64 = (assign44110_e59401 + assign44110_e59418);
        let assign44110_e59420: f64 = (locals.var_tmf1 * assign44110_e59419);
        let assign44110_e59421: f64 = (assign44110_e59397 + assign44110_e59420);
        let assign44110_e59422: f64 = (locals.var_tmf1 * assign44110_e59421);
        let assign44110_e59423: f64 = (assign44110_e59393 + assign44110_e59422);
        let assign44110_e59424: f64 = (locals.var_tmf1 * assign44110_e59423);
        let assign44110_e59425: f64 = (1.0 + assign44110_e59424);
        (assign44110_e59425, ((locals.var_tmf1_dn0 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn2 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn4 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn5 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn6 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn7 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn8 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn9 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn10 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign44110_e59413))))))))))), ((locals.var_tmf1_dn13 * assign44110_e59423) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign44110_e59421) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign44110_e59419) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign44110_e59417) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign44110_e59415) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign44110_e59413))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44110_e59427;
        locals.var_tmf2_dn0 = assign44110_e59427_d_n0;
        locals.var_tmf2_dn2 = assign44110_e59427_d_n2;
        locals.var_tmf2_dn4 = assign44110_e59427_d_n4;
        locals.var_tmf2_dn5 = assign44110_e59427_d_n5;
        locals.var_tmf2_dn6 = assign44110_e59427_d_n6;
        locals.var_tmf2_dn7 = assign44110_e59427_d_n7;
        locals.var_tmf2_dn8 = assign44110_e59427_d_n8;
        locals.var_tmf2_dn9 = assign44110_e59427_d_n9;
        locals.var_tmf2_dn10 = assign44110_e59427_d_n10;
        locals.var_tmf2_dn13 = assign44110_e59427_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44120_e59470, assign44120_e59470_d_n0, assign44120_e59470_d_n2, assign44120_e59470_d_n4, assign44120_e59470_d_n5, assign44120_e59470_d_n6, assign44120_e59470_d_n7, assign44120_e59470_d_n8, assign44120_e59470_d_n9, assign44120_e59470_d_n10, assign44120_e59470_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44120_e59438: f64 = (1.0 / 2.0);
        let assign44120_e59442: f64 = (1.0 / 3.0);
        let assign44120_e59446: f64 = (1.0 / 8.0);
        let assign44120_e59450: f64 = (1.0 / 30.0);
        let assign44120_e59454: f64 = (1.0 / 144.0);
        let assign44120_e59458: f64 = (1.0 / 840.0);
        let assign44120_e59459: f64 = (locals.var_tmf1 * assign44120_e59458);
        let assign44120_e59460: f64 = (assign44120_e59454 + assign44120_e59459);
        let assign44120_e59461: f64 = (locals.var_tmf1 * assign44120_e59460);
        let assign44120_e59462: f64 = (assign44120_e59450 + assign44120_e59461);
        let assign44120_e59463: f64 = (locals.var_tmf1 * assign44120_e59462);
        let assign44120_e59464: f64 = (assign44120_e59446 + assign44120_e59463);
        let assign44120_e59465: f64 = (locals.var_tmf1 * assign44120_e59464);
        let assign44120_e59466: f64 = (assign44120_e59442 + assign44120_e59465);
        let assign44120_e59467: f64 = (locals.var_tmf1 * assign44120_e59466);
        let assign44120_e59468: f64 = (assign44120_e59438 + assign44120_e59467);
        (assign44120_e59468, ((locals.var_tmf1_dn0 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign44120_e59458))))))))), ((locals.var_tmf1_dn2 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign44120_e59458))))))))), ((locals.var_tmf1_dn4 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign44120_e59458))))))))), ((locals.var_tmf1_dn5 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign44120_e59458))))))))), ((locals.var_tmf1_dn6 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign44120_e59458))))))))), ((locals.var_tmf1_dn7 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign44120_e59458))))))))), ((locals.var_tmf1_dn8 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign44120_e59458))))))))), ((locals.var_tmf1_dn9 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign44120_e59458))))))))), ((locals.var_tmf1_dn10 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign44120_e59458))))))))), ((locals.var_tmf1_dn13 * assign44120_e59466) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign44120_e59464) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign44120_e59462) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign44120_e59460) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign44120_e59458))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign44120_e59470;
        locals.var_tmf3_dn0 = assign44120_e59470_d_n0;
        locals.var_tmf3_dn2 = assign44120_e59470_d_n2;
        locals.var_tmf3_dn4 = assign44120_e59470_d_n4;
        locals.var_tmf3_dn5 = assign44120_e59470_d_n5;
        locals.var_tmf3_dn6 = assign44120_e59470_d_n6;
        locals.var_tmf3_dn7 = assign44120_e59470_d_n7;
        locals.var_tmf3_dn8 = assign44120_e59470_d_n8;
        locals.var_tmf3_dn9 = assign44120_e59470_d_n9;
        locals.var_tmf3_dn10 = assign44120_e59470_d_n10;
        locals.var_tmf3_dn13 = assign44120_e59470_d_n13;
        locals.var_tmf3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_148(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44130_e59483, assign44130_e59483_d_n0, assign44130_e59483_d_n2, assign44130_e59483_d_n4, assign44130_e59483_d_n5, assign44130_e59483_d_n6, assign44130_e59483_d_n7, assign44130_e59483_d_n8, assign44130_e59483_d_n9, assign44130_e59483_d_n10, assign44130_e59483_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44130_e59481: f64 = (0.01 / locals.var_tmf2);
        (assign44130_e59481, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign44130_e59483;
        locals.var_t6_dn0 = assign44130_e59483_d_n0;
        locals.var_t6_dn2 = assign44130_e59483_d_n2;
        locals.var_t6_dn4 = assign44130_e59483_d_n4;
        locals.var_t6_dn5 = assign44130_e59483_d_n5;
        locals.var_t6_dn6 = assign44130_e59483_d_n6;
        locals.var_t6_dn7 = assign44130_e59483_d_n7;
        locals.var_t6_dn8 = assign44130_e59483_d_n8;
        locals.var_t6_dn9 = assign44130_e59483_d_n9;
        locals.var_t6_dn10 = assign44130_e59483_d_n10;
        locals.var_t6_dn13 = assign44130_e59483_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign44140_e59501, assign44140_e59501_d_n0, assign44140_e59501_d_n2, assign44140_e59501_d_n4, assign44140_e59501_d_n5, assign44140_e59501_d_n6, assign44140_e59501_d_n7, assign44140_e59501_d_n8, assign44140_e59501_d_n9, assign44140_e59501_d_n10, assign44140_e59501_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44140_e59493: f64 = (-2.0);
        let assign44140_e59495: f64 = (assign44140_e59493 * locals.var_tmf3);
        let assign44140_e59498: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign44140_e59499: f64 = (assign44140_e59495 / assign44140_e59498);
        (assign44140_e59499, ((((assign44140_e59493 * locals.var_tmf3_dn0) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn2) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn4) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn5) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn6) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn7) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn8) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn9) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn10) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign44140_e59498 * assign44140_e59498)), ((((assign44140_e59493 * locals.var_tmf3_dn13) * assign44140_e59498) - (assign44140_e59495 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign44140_e59498 * assign44140_e59498)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign44140_e59501;
        locals.var_t2_dn0 = assign44140_e59501_d_n0;
        locals.var_t2_dn2 = assign44140_e59501_d_n2;
        locals.var_t2_dn4 = assign44140_e59501_d_n4;
        locals.var_t2_dn5 = assign44140_e59501_d_n5;
        locals.var_t2_dn6 = assign44140_e59501_d_n6;
        locals.var_t2_dn7 = assign44140_e59501_d_n7;
        locals.var_t2_dn8 = assign44140_e59501_d_n8;
        locals.var_t2_dn9 = assign44140_e59501_d_n9;
        locals.var_t2_dn10 = assign44140_e59501_d_n10;
        locals.var_t2_dn13 = assign44140_e59501_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign44150_e59516, assign44150_e59516_d_n0, assign44150_e59516_d_n2, assign44150_e59516_d_n4, assign44150_e59516_d_n5, assign44150_e59516_d_n6, assign44150_e59516_d_n7, assign44150_e59516_d_n8, assign44150_e59516_d_n9, assign44150_e59516_d_n10, assign44150_e59516_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44150_e59513: f64 = (locals.var_phi_s0_dep__blk853 + locals.var_t6);
        let assign44150_e59514: f64 = (1.1 - assign44150_e59513);
        (assign44150_e59514, (-(locals.var_phi_s0_dep__blk853_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep__blk853_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep__blk853_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep__blk853_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep__blk853_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep__blk853_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep__blk853_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep__blk853_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep__blk853_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep__blk853_dn13 + locals.var_t6_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign44150_e59516;
        locals.var_t1_dn0 = assign44150_e59516_d_n0;
        locals.var_t1_dn2 = assign44150_e59516_d_n2;
        locals.var_t1_dn4 = assign44150_e59516_d_n4;
        locals.var_t1_dn5 = assign44150_e59516_d_n5;
        locals.var_t1_dn6 = assign44150_e59516_d_n6;
        locals.var_t1_dn7 = assign44150_e59516_d_n7;
        locals.var_t1_dn8 = assign44150_e59516_d_n8;
        locals.var_t1_dn9 = assign44150_e59516_d_n9;
        locals.var_t1_dn10 = assign44150_e59516_d_n10;
        locals.var_t1_dn13 = assign44150_e59516_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign44160_e59536, assign44160_e59536_d_n0, assign44160_e59536_d_n2, assign44160_e59536_d_n4, assign44160_e59536_d_n5, assign44160_e59536_d_n6, assign44160_e59536_d_n7, assign44160_e59536_d_n8, assign44160_e59536_d_n9, assign44160_e59536_d_n10, assign44160_e59536_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44160_e59527: f64 = (locals.var_t1 * locals.var_t1);
        let assign44160_e59530: f64 = (4.0 * 0.05);
        let assign44160_e59532: f64 = (assign44160_e59530 * 0.05);
        let assign44160_e59533: f64 = (assign44160_e59527 + assign44160_e59532);
        let assign44160_e59534: f64 = (assign44160_e59533).sqrt();
        (assign44160_e59534, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign44160_e59534)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign44160_e59534)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44160_e59536;
        locals.var_tmf2_dn0 = assign44160_e59536_d_n0;
        locals.var_tmf2_dn2 = assign44160_e59536_d_n2;
        locals.var_tmf2_dn4 = assign44160_e59536_d_n4;
        locals.var_tmf2_dn5 = assign44160_e59536_d_n5;
        locals.var_tmf2_dn6 = assign44160_e59536_d_n6;
        locals.var_tmf2_dn7 = assign44160_e59536_d_n7;
        locals.var_tmf2_dn8 = assign44160_e59536_d_n8;
        locals.var_tmf2_dn9 = assign44160_e59536_d_n9;
        locals.var_tmf2_dn10 = assign44160_e59536_d_n10;
        locals.var_tmf2_dn13 = assign44160_e59536_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44170_e59553, assign44170_e59553_d_n0, assign44170_e59553_d_n2, assign44170_e59553_d_n4, assign44170_e59553_d_n5, assign44170_e59553_d_n6, assign44170_e59553_d_n7, assign44170_e59553_d_n8, assign44170_e59553_d_n9, assign44170_e59553_d_n10, assign44170_e59553_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44170_e59549: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign44170_e59550: f64 = (1.0 + assign44170_e59549);
        let assign44170_e59551: f64 = (0.5 * assign44170_e59550);
        (assign44170_e59551, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44170_e59553;
        locals.var_t0_dn0 = assign44170_e59553_d_n0;
        locals.var_t0_dn2 = assign44170_e59553_d_n2;
        locals.var_t0_dn4 = assign44170_e59553_d_n4;
        locals.var_t0_dn5 = assign44170_e59553_d_n5;
        locals.var_t0_dn6 = assign44170_e59553_d_n6;
        locals.var_t0_dn7 = assign44170_e59553_d_n7;
        locals.var_t0_dn8 = assign44170_e59553_d_n8;
        locals.var_t0_dn9 = assign44170_e59553_d_n9;
        locals.var_t0_dn10 = assign44170_e59553_d_n10;
        locals.var_t0_dn13 = assign44170_e59553_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44180_e59568, assign44180_e59568_d_n0, assign44180_e59568_d_n2, assign44180_e59568_d_n4, assign44180_e59568_d_n5, assign44180_e59568_d_n6, assign44180_e59568_d_n7, assign44180_e59568_d_n8, assign44180_e59568_d_n9, assign44180_e59568_d_n10, assign44180_e59568_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44180_e59565: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign44180_e59566: f64 = (0.5 * assign44180_e59565);
        (assign44180_e59566, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign44180_e59568;
        locals.var_t2_dn0 = assign44180_e59568_d_n0;
        locals.var_t2_dn2 = assign44180_e59568_d_n2;
        locals.var_t2_dn4 = assign44180_e59568_d_n4;
        locals.var_t2_dn5 = assign44180_e59568_d_n5;
        locals.var_t2_dn6 = assign44180_e59568_d_n6;
        locals.var_t2_dn7 = assign44180_e59568_d_n7;
        locals.var_t2_dn8 = assign44180_e59568_d_n8;
        locals.var_t2_dn9 = assign44180_e59568_d_n9;
        locals.var_t2_dn10 = assign44180_e59568_d_n10;
        locals.var_t2_dn13 = assign44180_e59568_d_n13;
        locals.var_t2_rv = 0.0;

        let assign44190_e59571: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign44190_e59571;
        locals.var_guard1077_rv = 0.0;

        let (assign44200_e59584, assign44200_e59584_d_n0, assign44200_e59584_d_n2, assign44200_e59584_d_n4, assign44200_e59584_d_n5, assign44200_e59584_d_n6, assign44200_e59584_d_n7, assign44200_e59584_d_n8, assign44200_e59584_d_n9, assign44200_e59584_d_n10, assign44200_e59584_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign44200_e59584;
        locals.var_t2_dn0 = assign44200_e59584_d_n0;
        locals.var_t2_dn2 = assign44200_e59584_d_n2;
        locals.var_t2_dn4 = assign44200_e59584_d_n4;
        locals.var_t2_dn5 = assign44200_e59584_d_n5;
        locals.var_t2_dn6 = assign44200_e59584_d_n6;
        locals.var_t2_dn7 = assign44200_e59584_d_n7;
        locals.var_t2_dn8 = assign44200_e59584_d_n8;
        locals.var_t2_dn9 = assign44200_e59584_d_n9;
        locals.var_t2_dn10 = assign44200_e59584_d_n10;
        locals.var_t2_dn13 = assign44200_e59584_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign44210_e59597, assign44210_e59597_d_n0, assign44210_e59597_d_n2, assign44210_e59597_d_n4, assign44210_e59597_d_n5, assign44210_e59597_d_n6, assign44210_e59597_d_n7, assign44210_e59597_d_n8, assign44210_e59597_d_n9, assign44210_e59597_d_n10, assign44210_e59597_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44210_e59597;
        locals.var_t0_dn0 = assign44210_e59597_d_n0;
        locals.var_t0_dn2 = assign44210_e59597_d_n2;
        locals.var_t0_dn4 = assign44210_e59597_d_n4;
        locals.var_t0_dn5 = assign44210_e59597_d_n5;
        locals.var_t0_dn6 = assign44210_e59597_d_n6;
        locals.var_t0_dn7 = assign44210_e59597_d_n7;
        locals.var_t0_dn8 = assign44210_e59597_d_n8;
        locals.var_t0_dn9 = assign44210_e59597_d_n9;
        locals.var_t0_dn10 = assign44210_e59597_d_n10;
        locals.var_t0_dn13 = assign44210_e59597_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44220_e59610, assign44220_e59610_d_n0, assign44220_e59610_d_n2, assign44220_e59610_d_n4, assign44220_e59610_d_n5, assign44220_e59610_d_n6, assign44220_e59610_d_n7, assign44220_e59610_d_n8, assign44220_e59610_d_n9, assign44220_e59610_d_n10, assign44220_e59610_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44220_e59608: f64 = (locals.var_t2 + 1e-25);
        (assign44220_e59608, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign44220_e59610;
        locals.var_t2_dn0 = assign44220_e59610_d_n0;
        locals.var_t2_dn2 = assign44220_e59610_d_n2;
        locals.var_t2_dn4 = assign44220_e59610_d_n4;
        locals.var_t2_dn5 = assign44220_e59610_d_n5;
        locals.var_t2_dn6 = assign44220_e59610_d_n6;
        locals.var_t2_dn7 = assign44220_e59610_d_n7;
        locals.var_t2_dn8 = assign44220_e59610_d_n8;
        locals.var_t2_dn9 = assign44220_e59610_d_n9;
        locals.var_t2_dn10 = assign44220_e59610_d_n10;
        locals.var_t2_dn13 = assign44220_e59610_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign44230_e59623, assign44230_e59623_d_n0, assign44230_e59623_d_n2, assign44230_e59623_d_n4, assign44230_e59623_d_n5, assign44230_e59623_d_n6, assign44230_e59623_d_n7, assign44230_e59623_d_n8, assign44230_e59623_d_n9, assign44230_e59623_d_n10, assign44230_e59623_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44230_e59621: f64 = (locals.var_beta * locals.var_ptl0);
        (assign44230_e59621, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn13 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44230_e59623;
        locals.var_t0_dn0 = assign44230_e59623_d_n0;
        locals.var_t0_dn2 = assign44230_e59623_d_n2;
        locals.var_t0_dn4 = assign44230_e59623_d_n4;
        locals.var_t0_dn5 = assign44230_e59623_d_n5;
        locals.var_t0_dn6 = assign44230_e59623_d_n6;
        locals.var_t0_dn7 = assign44230_e59623_d_n7;
        locals.var_t0_dn8 = assign44230_e59623_d_n8;
        locals.var_t0_dn9 = assign44230_e59623_d_n9;
        locals.var_t0_dn10 = assign44230_e59623_d_n10;
        locals.var_t0_dn13 = assign44230_e59623_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44240_e59636, assign44240_e59636_d_n0, assign44240_e59636_d_n2, assign44240_e59636_d_n4, assign44240_e59636_d_n5, assign44240_e59636_d_n6, assign44240_e59636_d_n7, assign44240_e59636_d_n8, assign44240_e59636_d_n9, assign44240_e59636_d_n10, assign44240_e59636_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44240_e59634: f64 = (locals.var_cox * locals.var_t0);
        (assign44240_e59634, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn13 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign44240_e59636;
        locals.var_t3_dn0 = assign44240_e59636_d_n0;
        locals.var_t3_dn2 = assign44240_e59636_d_n2;
        locals.var_t3_dn4 = assign44240_e59636_d_n4;
        locals.var_t3_dn5 = assign44240_e59636_d_n5;
        locals.var_t3_dn6 = assign44240_e59636_d_n6;
        locals.var_t3_dn7 = assign44240_e59636_d_n7;
        locals.var_t3_dn8 = assign44240_e59636_d_n8;
        locals.var_t3_dn9 = assign44240_e59636_d_n9;
        locals.var_t3_dn10 = assign44240_e59636_d_n10;
        locals.var_t3_dn13 = assign44240_e59636_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign44250_e59649, assign44250_e59649_d_n0, assign44250_e59649_d_n2, assign44250_e59649_d_n4, assign44250_e59649_d_n5, assign44250_e59649_d_n6, assign44250_e59649_d_n7, assign44250_e59649_d_n8, assign44250_e59649_d_n9, assign44250_e59649_d_n10, assign44250_e59649_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44250_e59647: f64 = (locals.var_t2).powf(p.p284);
        (assign44250_e59647, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn13)) } } else { (assign44250_e59647 * (p.p284 * (locals.var_t2_dn13 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44250_e59649;
        locals.var_t0_dn0 = assign44250_e59649_d_n0;
        locals.var_t0_dn2 = assign44250_e59649_d_n2;
        locals.var_t0_dn4 = assign44250_e59649_d_n4;
        locals.var_t0_dn5 = assign44250_e59649_d_n5;
        locals.var_t0_dn6 = assign44250_e59649_d_n6;
        locals.var_t0_dn7 = assign44250_e59649_d_n7;
        locals.var_t0_dn8 = assign44250_e59649_d_n8;
        locals.var_t0_dn9 = assign44250_e59649_d_n9;
        locals.var_t0_dn10 = assign44250_e59649_d_n10;
        locals.var_t0_dn13 = assign44250_e59649_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44260_e59662, assign44260_e59662_d_n0, assign44260_e59662_d_n2, assign44260_e59662_d_n4, assign44260_e59662_d_n5, assign44260_e59662_d_n6, assign44260_e59662_d_n7, assign44260_e59662_d_n8, assign44260_e59662_d_n9, assign44260_e59662_d_n10, assign44260_e59662_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44260_e59660: f64 = (locals.var_t3 * locals.var_t0);
        (assign44260_e59660, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn13 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign44260_e59662;
        locals.var_t9_dn0 = assign44260_e59662_d_n0;
        locals.var_t9_dn2 = assign44260_e59662_d_n2;
        locals.var_t9_dn4 = assign44260_e59662_d_n4;
        locals.var_t9_dn5 = assign44260_e59662_d_n5;
        locals.var_t9_dn6 = assign44260_e59662_d_n6;
        locals.var_t9_dn7 = assign44260_e59662_d_n7;
        locals.var_t9_dn8 = assign44260_e59662_d_n8;
        locals.var_t9_dn9 = assign44260_e59662_d_n9;
        locals.var_t9_dn10 = assign44260_e59662_d_n10;
        locals.var_t9_dn13 = assign44260_e59662_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign44270_e59677, assign44270_e59677_d_n0, assign44270_e59677_d_n2, assign44270_e59677_d_n4, assign44270_e59677_d_n5, assign44270_e59677_d_n6, assign44270_e59677_d_n7, assign44270_e59677_d_n8, assign44270_e59677_d_n9, assign44270_e59677_d_n10, assign44270_e59677_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44270_e59674: f64 = (locals.var_vdsz__blk439 * p.p285);
        let assign44270_e59675: f64 = (1.0 + assign44270_e59674);
        (assign44270_e59675, (locals.var_vdsz__blk439_dn0 * p.p285), (locals.var_vdsz__blk439_dn2 * p.p285), (locals.var_vdsz__blk439_dn4 * p.p285), (locals.var_vdsz__blk439_dn5 * p.p285), (locals.var_vdsz__blk439_dn6 * p.p285), (locals.var_vdsz__blk439_dn7 * p.p285), (locals.var_vdsz__blk439_dn8 * p.p285), (locals.var_vdsz__blk439_dn9 * p.p285), (locals.var_vdsz__blk439_dn10 * p.p285), (locals.var_vdsz__blk439_dn13 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign44270_e59677;
        locals.var_t4_dn0 = assign44270_e59677_d_n0;
        locals.var_t4_dn2 = assign44270_e59677_d_n2;
        locals.var_t4_dn4 = assign44270_e59677_d_n4;
        locals.var_t4_dn5 = assign44270_e59677_d_n5;
        locals.var_t4_dn6 = assign44270_e59677_d_n6;
        locals.var_t4_dn7 = assign44270_e59677_d_n7;
        locals.var_t4_dn8 = assign44270_e59677_d_n8;
        locals.var_t4_dn9 = assign44270_e59677_d_n9;
        locals.var_t4_dn10 = assign44270_e59677_d_n10;
        locals.var_t4_dn13 = assign44270_e59677_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign44280_e59688, assign44280_e59688_d_n0, assign44280_e59688_d_n2, assign44280_e59688_d_n4, assign44280_e59688_d_n5, assign44280_e59688_d_n6, assign44280_e59688_d_n7, assign44280_e59688_d_n8, assign44280_e59688_d_n9, assign44280_e59688_d_n10, assign44280_e59688_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44280_e59688;
        locals.var_t0_dn0 = assign44280_e59688_d_n0;
        locals.var_t0_dn2 = assign44280_e59688_d_n2;
        locals.var_t0_dn4 = assign44280_e59688_d_n4;
        locals.var_t0_dn5 = assign44280_e59688_d_n5;
        locals.var_t0_dn6 = assign44280_e59688_d_n6;
        locals.var_t0_dn7 = assign44280_e59688_d_n7;
        locals.var_t0_dn8 = assign44280_e59688_d_n8;
        locals.var_t0_dn9 = assign44280_e59688_d_n9;
        locals.var_t0_dn10 = assign44280_e59688_d_n10;
        locals.var_t0_dn13 = assign44280_e59688_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44290_e59703, assign44290_e59703_d_n0, assign44290_e59703_d_n2, assign44290_e59703_d_n4, assign44290_e59703_d_n5, assign44290_e59703_d_n6, assign44290_e59703_d_n7, assign44290_e59703_d_n8, assign44290_e59703_d_n9, assign44290_e59703_d_n10, assign44290_e59703_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44290_e59699: f64 = (locals.var_phi_s0_dep__blk853 + locals.var_t6);
        let assign44290_e59701: f64 = (assign44290_e59699 - locals.var_vbsz__blk438);
        (assign44290_e59701, ((locals.var_phi_s0_dep__blk853_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk438_dn0), ((locals.var_phi_s0_dep__blk853_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk438_dn2), ((locals.var_phi_s0_dep__blk853_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk438_dn4), ((locals.var_phi_s0_dep__blk853_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk438_dn5), ((locals.var_phi_s0_dep__blk853_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk438_dn6), ((locals.var_phi_s0_dep__blk853_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk438_dn7), ((locals.var_phi_s0_dep__blk853_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk438_dn8), ((locals.var_phi_s0_dep__blk853_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk438_dn9), ((locals.var_phi_s0_dep__blk853_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk438_dn10), ((locals.var_phi_s0_dep__blk853_dn13 + locals.var_t6_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign44290_e59703;
        locals.var_t5_dn0 = assign44290_e59703_d_n0;
        locals.var_t5_dn2 = assign44290_e59703_d_n2;
        locals.var_t5_dn4 = assign44290_e59703_d_n4;
        locals.var_t5_dn5 = assign44290_e59703_d_n5;
        locals.var_t5_dn6 = assign44290_e59703_d_n6;
        locals.var_t5_dn7 = assign44290_e59703_d_n7;
        locals.var_t5_dn8 = assign44290_e59703_d_n8;
        locals.var_t5_dn9 = assign44290_e59703_d_n9;
        locals.var_t5_dn10 = assign44290_e59703_d_n10;
        locals.var_t5_dn13 = assign44290_e59703_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign44300_e59720, assign44300_e59720_d_n0, assign44300_e59720_d_n2, assign44300_e59720_d_n4, assign44300_e59720_d_n5, assign44300_e59720_d_n6, assign44300_e59720_d_n7, assign44300_e59720_d_n8, assign44300_e59720_d_n9, assign44300_e59720_d_n10, assign44300_e59720_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44300_e59715: f64 = (locals.var_vdsz__blk439 * locals.var_t0);
        let assign44300_e59717: f64 = (assign44300_e59715 * locals.var_t5);
        let assign44300_e59718: f64 = (locals.var_t4 + assign44300_e59717);
        (assign44300_e59718, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk439_dn0 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn0)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk439_dn2 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn2)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk439_dn4 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn4)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk439_dn5 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn5)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk439_dn6 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn6)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk439_dn7 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn7)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk439_dn8 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn8)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk439_dn9 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn9)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk439_dn10 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn10)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn10))), (locals.var_t4_dn13 + ((((locals.var_vdsz__blk439_dn13 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn13)) * locals.var_t5) + (assign44300_e59715 * locals.var_t5_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign44300_e59720;
        locals.var_t4_dn0 = assign44300_e59720_d_n0;
        locals.var_t4_dn2 = assign44300_e59720_d_n2;
        locals.var_t4_dn4 = assign44300_e59720_d_n4;
        locals.var_t4_dn5 = assign44300_e59720_d_n5;
        locals.var_t4_dn6 = assign44300_e59720_d_n6;
        locals.var_t4_dn7 = assign44300_e59720_d_n7;
        locals.var_t4_dn8 = assign44300_e59720_d_n8;
        locals.var_t4_dn9 = assign44300_e59720_d_n9;
        locals.var_t4_dn10 = assign44300_e59720_d_n10;
        locals.var_t4_dn13 = assign44300_e59720_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign44310_e59733, assign44310_e59733_d_n0, assign44310_e59733_d_n2, assign44310_e59733_d_n4, assign44310_e59733_d_n5, assign44310_e59733_d_n6, assign44310_e59733_d_n7, assign44310_e59733_d_n8, assign44310_e59733_d_n9, assign44310_e59733_d_n10, assign44310_e59733_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        let assign44310_e59731: f64 = (locals.var_t9 * locals.var_t4);
        (assign44310_e59731, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn13 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign44310_e59733;
        locals.var_t6_dn0 = assign44310_e59733_d_n0;
        locals.var_t6_dn2 = assign44310_e59733_d_n2;
        locals.var_t6_dn4 = assign44310_e59733_d_n4;
        locals.var_t6_dn5 = assign44310_e59733_d_n5;
        locals.var_t6_dn6 = assign44310_e59733_d_n6;
        locals.var_t6_dn7 = assign44310_e59733_d_n7;
        locals.var_t6_dn8 = assign44310_e59733_d_n8;
        locals.var_t6_dn9 = assign44310_e59733_d_n9;
        locals.var_t6_dn10 = assign44310_e59733_d_n10;
        locals.var_t6_dn13 = assign44310_e59733_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign44320_e59744, assign44320_e59744_d_n0, assign44320_e59744_d_n2, assign44320_e59744_d_n4, assign44320_e59744_d_n5, assign44320_e59744_d_n6, assign44320_e59744_d_n7, assign44320_e59744_d_n8, assign44320_e59744_d_n9, assign44320_e59744_d_n10, assign44320_e59744_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign44320_e59744;
        locals.var_t9_dn0 = assign44320_e59744_d_n0;
        locals.var_t9_dn2 = assign44320_e59744_d_n2;
        locals.var_t9_dn4 = assign44320_e59744_d_n4;
        locals.var_t9_dn5 = assign44320_e59744_d_n5;
        locals.var_t9_dn6 = assign44320_e59744_d_n6;
        locals.var_t9_dn7 = assign44320_e59744_d_n7;
        locals.var_t9_dn8 = assign44320_e59744_d_n8;
        locals.var_t9_dn9 = assign44320_e59744_d_n9;
        locals.var_t9_dn10 = assign44320_e59744_d_n10;
        locals.var_t9_dn13 = assign44320_e59744_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign44330_e59756, assign44330_e59756_d_n0, assign44330_e59756_d_n2, assign44330_e59756_d_n4, assign44330_e59756_d_n5, assign44330_e59756_d_n6, assign44330_e59756_d_n7, assign44330_e59756_d_n8, assign44330_e59756_d_n9, assign44330_e59756_d_n10, assign44330_e59756_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1076 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign44330_e59756;
        locals.var_t9_dn0 = assign44330_e59756_d_n0;
        locals.var_t9_dn2 = assign44330_e59756_d_n2;
        locals.var_t9_dn4 = assign44330_e59756_d_n4;
        locals.var_t9_dn5 = assign44330_e59756_d_n5;
        locals.var_t9_dn6 = assign44330_e59756_d_n6;
        locals.var_t9_dn7 = assign44330_e59756_d_n7;
        locals.var_t9_dn8 = assign44330_e59756_d_n8;
        locals.var_t9_dn9 = assign44330_e59756_d_n9;
        locals.var_t9_dn10 = assign44330_e59756_d_n10;
        locals.var_t9_dn13 = assign44330_e59756_d_n13;
        locals.var_t9_rv = 0.0;

        let assign44340_e59759: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign44340_e59759;
        locals.var_guard1078_rv = 0.0;

        let (assign44350_e59772, assign44350_e59772_d_n0, assign44350_e59772_d_n2, assign44350_e59772_d_n4, assign44350_e59772_d_n5, assign44350_e59772_d_n6, assign44350_e59772_d_n7, assign44350_e59772_d_n8, assign44350_e59772_d_n9, assign44350_e59772_d_n10, assign44350_e59772_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44350_e59770: f64 = (locals.var_beta * locals.var_gdl0);
        (assign44350_e59770, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn13 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign44350_e59772;
        locals.var_t1_dn0 = assign44350_e59772_d_n0;
        locals.var_t1_dn2 = assign44350_e59772_d_n2;
        locals.var_t1_dn4 = assign44350_e59772_d_n4;
        locals.var_t1_dn5 = assign44350_e59772_d_n5;
        locals.var_t1_dn6 = assign44350_e59772_d_n6;
        locals.var_t1_dn7 = assign44350_e59772_d_n7;
        locals.var_t1_dn8 = assign44350_e59772_d_n8;
        locals.var_t1_dn9 = assign44350_e59772_d_n9;
        locals.var_t1_dn10 = assign44350_e59772_d_n10;
        locals.var_t1_dn13 = assign44350_e59772_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign44360_e59785, assign44360_e59785_d_n0, assign44360_e59785_d_n2, assign44360_e59785_d_n4, assign44360_e59785_d_n5, assign44360_e59785_d_n6, assign44360_e59785_d_n7, assign44360_e59785_d_n8, assign44360_e59785_d_n9, assign44360_e59785_d_n10, assign44360_e59785_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44360_e59783: f64 = (locals.var_cox * locals.var_t1);
        (assign44360_e59783, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn13 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign44360_e59785;
        locals.var_t2_dn0 = assign44360_e59785_d_n0;
        locals.var_t2_dn2 = assign44360_e59785_d_n2;
        locals.var_t2_dn4 = assign44360_e59785_d_n4;
        locals.var_t2_dn5 = assign44360_e59785_d_n5;
        locals.var_t2_dn6 = assign44360_e59785_d_n6;
        locals.var_t2_dn7 = assign44360_e59785_d_n7;
        locals.var_t2_dn8 = assign44360_e59785_d_n8;
        locals.var_t2_dn9 = assign44360_e59785_d_n9;
        locals.var_t2_dn10 = assign44360_e59785_d_n10;
        locals.var_t2_dn13 = assign44360_e59785_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign44370_e59798, assign44370_e59798_d_n0, assign44370_e59798_d_n2, assign44370_e59798_d_n4, assign44370_e59798_d_n5, assign44370_e59798_d_n6, assign44370_e59798_d_n7, assign44370_e59798_d_n8, assign44370_e59798_d_n9, assign44370_e59798_d_n10, assign44370_e59798_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1078 != 0.0)) {
        let assign44370_e59796: f64 = (locals.var_t2 * locals.var_vdsz__blk439);
        (assign44370_e59796, ((locals.var_t2_dn0 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn10)), ((locals.var_t2_dn13 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign44370_e59798;
        locals.var_t8_dn0 = assign44370_e59798_d_n0;
        locals.var_t8_dn2 = assign44370_e59798_d_n2;
        locals.var_t8_dn4 = assign44370_e59798_d_n4;
        locals.var_t8_dn5 = assign44370_e59798_d_n5;
        locals.var_t8_dn6 = assign44370_e59798_d_n6;
        locals.var_t8_dn7 = assign44370_e59798_d_n7;
        locals.var_t8_dn8 = assign44370_e59798_d_n8;
        locals.var_t8_dn9 = assign44370_e59798_d_n9;
        locals.var_t8_dn10 = assign44370_e59798_d_n10;
        locals.var_t8_dn13 = assign44370_e59798_d_n13;
        locals.var_t8_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_149(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44380_e59810, assign44380_e59810_d_n0, assign44380_e59810_d_n2, assign44380_e59810_d_n4, assign44380_e59810_d_n5, assign44380_e59810_d_n6, assign44380_e59810_d_n7, assign44380_e59810_d_n8, assign44380_e59810_d_n9, assign44380_e59810_d_n10, assign44380_e59810_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1078 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign44380_e59810;
        locals.var_t8_dn0 = assign44380_e59810_d_n0;
        locals.var_t8_dn2 = assign44380_e59810_d_n2;
        locals.var_t8_dn4 = assign44380_e59810_d_n4;
        locals.var_t8_dn5 = assign44380_e59810_d_n5;
        locals.var_t8_dn6 = assign44380_e59810_d_n6;
        locals.var_t8_dn7 = assign44380_e59810_d_n7;
        locals.var_t8_dn8 = assign44380_e59810_d_n8;
        locals.var_t8_dn9 = assign44380_e59810_d_n9;
        locals.var_t8_dn10 = assign44380_e59810_d_n10;
        locals.var_t8_dn13 = assign44380_e59810_d_n13;
        locals.var_t8_rv = 0.0;

        let assign44390_e59813: f64 = (locals.var_t9 + locals.var_t8);
        let assign44390_e59815: f64 = if assign44390_e59813 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign44390_e59815;
        locals.var_guard1079_rv = 0.0;

        let (assign44400_e59830, assign44400_e59830_d_n0, assign44400_e59830_d_n2, assign44400_e59830_d_n4, assign44400_e59830_d_n5, assign44400_e59830_d_n6, assign44400_e59830_d_n7, assign44400_e59830_d_n8, assign44400_e59830_d_n9, assign44400_e59830_d_n10, assign44400_e59830_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1079 != 0.0)) {
        let assign44400_e59827: f64 = (locals.var_t9 + locals.var_t8);
        let assign44400_e59828: f64 = (locals.var_pds * assign44400_e59827);
        (assign44400_e59828, ((locals.var_pds_dn0 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn13 * assign44400_e59827) + (locals.var_pds * (locals.var_t9_dn13 + locals.var_t8_dn13))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn13,)
    }
};
        locals.var_idd1 = assign44400_e59830;
        locals.var_idd1_dn0 = assign44400_e59830_d_n0;
        locals.var_idd1_dn2 = assign44400_e59830_d_n2;
        locals.var_idd1_dn4 = assign44400_e59830_d_n4;
        locals.var_idd1_dn5 = assign44400_e59830_d_n5;
        locals.var_idd1_dn6 = assign44400_e59830_d_n6;
        locals.var_idd1_dn7 = assign44400_e59830_d_n7;
        locals.var_idd1_dn8 = assign44400_e59830_d_n8;
        locals.var_idd1_dn9 = assign44400_e59830_d_n9;
        locals.var_idd1_dn10 = assign44400_e59830_d_n10;
        locals.var_idd1_dn13 = assign44400_e59830_d_n13;
        locals.var_idd1_rv = 0.0;

        let (assign44410_e59847, assign44410_e59847_d_n0, assign44410_e59847_d_n2, assign44410_e59847_d_n4, assign44410_e59847_d_n5, assign44410_e59847_d_n6, assign44410_e59847_d_n7, assign44410_e59847_d_n8, assign44410_e59847_d_n9, assign44410_e59847_d_n10, assign44410_e59847_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1079 != 0.0)) {
        let assign44410_e59842: f64 = (locals.var_betawl * locals.var_idd1);
        let assign44410_e59844: f64 = (assign44410_e59842 * locals.var_mu);
        let assign44410_e59845: f64 = (locals.var_ids0 + assign44410_e59844);
        (assign44410_e59845, (locals.var_ids0_dn0 + ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn0))), (locals.var_ids0_dn2 + ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn2))), (locals.var_ids0_dn4 + ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn4))), (locals.var_ids0_dn5 + ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn5))), (locals.var_ids0_dn6 + ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn6))), (locals.var_ids0_dn7 + ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn7))), (locals.var_ids0_dn8 + ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn8))), (locals.var_ids0_dn9 + ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn9))), (locals.var_ids0_dn10 + ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn10))), (locals.var_ids0_dn13 + ((((locals.var_betawl_dn13 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn13)) * locals.var_mu) + (assign44410_e59842 * locals.var_mu_dn13))),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign44410_e59847;
        locals.var_ids0_dn0 = assign44410_e59847_d_n0;
        locals.var_ids0_dn2 = assign44410_e59847_d_n2;
        locals.var_ids0_dn4 = assign44410_e59847_d_n4;
        locals.var_ids0_dn5 = assign44410_e59847_d_n5;
        locals.var_ids0_dn6 = assign44410_e59847_d_n6;
        locals.var_ids0_dn7 = assign44410_e59847_d_n7;
        locals.var_ids0_dn8 = assign44410_e59847_d_n8;
        locals.var_ids0_dn9 = assign44410_e59847_d_n9;
        locals.var_ids0_dn10 = assign44410_e59847_d_n10;
        locals.var_ids0_dn13 = assign44410_e59847_d_n13;
        locals.var_ids0_rv = 0.0;

        let assign44420_e59854: f64 = if ((locals.var_flg_rsrd == 2.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign44420_e59854;
        locals.var_guard1080_rv = 0.0;

        let assign44430_e59857: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign44430_e59857;
        locals.var_guard1081_rv = 0.0;

        let (assign44440_e59870, assign44440_e59870_d_n0, assign44440_e59870_d_n2, assign44440_e59870_d_n4, assign44440_e59870_d_n5, assign44440_e59870_d_n6, assign44440_e59870_d_n7, assign44440_e59870_d_n8, assign44440_e59870_d_n9, assign44440_e59870_d_n10, assign44440_e59870_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign44440_e59870;
        locals.var_t4_dn0 = assign44440_e59870_d_n0;
        locals.var_t4_dn2 = assign44440_e59870_d_n2;
        locals.var_t4_dn4 = assign44440_e59870_d_n4;
        locals.var_t4_dn5 = assign44440_e59870_d_n5;
        locals.var_t4_dn6 = assign44440_e59870_d_n6;
        locals.var_t4_dn7 = assign44440_e59870_d_n7;
        locals.var_t4_dn8 = assign44440_e59870_d_n8;
        locals.var_t4_dn9 = assign44440_e59870_d_n9;
        locals.var_t4_dn10 = assign44440_e59870_d_n10;
        locals.var_t4_dn13 = assign44440_e59870_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign44450_e59887, assign44450_e59887_d_n0, assign44450_e59887_d_n2, assign44450_e59887_d_n4, assign44450_e59887_d_n5, assign44450_e59887_d_n6, assign44450_e59887_d_n7, assign44450_e59887_d_n8, assign44450_e59887_d_n9, assign44450_e59887_d_n10, assign44450_e59887_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44450_e59884: f64 = (locals.var_vgse - p.p300);
        let assign44450_e59885: f64 = (locals.var_uc_rd24 * assign44450_e59884);
        (assign44450_e59885, (locals.var_uc_rd24 * locals.var_vgse_dn0), (locals.var_uc_rd24 * locals.var_vgse_dn2), 0.0, 0.0, (locals.var_uc_rd24 * locals.var_vgse_dn6), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign44450_e59887;
        locals.var_t1_dn0 = assign44450_e59887_d_n0;
        locals.var_t1_dn2 = assign44450_e59887_d_n2;
        locals.var_t1_dn4 = assign44450_e59887_d_n4;
        locals.var_t1_dn5 = assign44450_e59887_d_n5;
        locals.var_t1_dn6 = assign44450_e59887_d_n6;
        locals.var_t1_dn7 = assign44450_e59887_d_n7;
        locals.var_t1_dn8 = assign44450_e59887_d_n8;
        locals.var_t1_dn9 = assign44450_e59887_d_n9;
        locals.var_t1_dn10 = assign44450_e59887_d_n10;
        locals.var_t1_dn13 = assign44450_e59887_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign44460_e59906, assign44460_e59906_d_n0, assign44460_e59906_d_n2, assign44460_e59906_d_n4, assign44460_e59906_d_n5, assign44460_e59906_d_n6, assign44460_e59906_d_n7, assign44460_e59906_d_n8, assign44460_e59906_d_n9, assign44460_e59906_d_n10, assign44460_e59906_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44460_e59900: f64 = (locals.var_t1 - locals.var_t4);
        let assign44460_e59903: f64 = (0.01 * 0.01);
        let assign44460_e59904: f64 = (assign44460_e59900 - assign44460_e59903);
        (assign44460_e59904, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign44460_e59906;
        locals.var_tmf1_dn0 = assign44460_e59906_d_n0;
        locals.var_tmf1_dn2 = assign44460_e59906_d_n2;
        locals.var_tmf1_dn4 = assign44460_e59906_d_n4;
        locals.var_tmf1_dn5 = assign44460_e59906_d_n5;
        locals.var_tmf1_dn6 = assign44460_e59906_d_n6;
        locals.var_tmf1_dn7 = assign44460_e59906_d_n7;
        locals.var_tmf1_dn8 = assign44460_e59906_d_n8;
        locals.var_tmf1_dn9 = assign44460_e59906_d_n9;
        locals.var_tmf1_dn10 = assign44460_e59906_d_n10;
        locals.var_tmf1_dn13 = assign44460_e59906_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign44470_e59925, assign44470_e59925_d_n0, assign44470_e59925_d_n2, assign44470_e59925_d_n4, assign44470_e59925_d_n5, assign44470_e59925_d_n6, assign44470_e59925_d_n7, assign44470_e59925_d_n8, assign44470_e59925_d_n9, assign44470_e59925_d_n10, assign44470_e59925_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44470_e59919: f64 = (4.0 * locals.var_t4);
        let assign44470_e59922: f64 = (0.01 * 0.01);
        let assign44470_e59923: f64 = (assign44470_e59919 * assign44470_e59922);
        (assign44470_e59923, ((4.0 * locals.var_t4_dn0) * assign44470_e59922), ((4.0 * locals.var_t4_dn2) * assign44470_e59922), ((4.0 * locals.var_t4_dn4) * assign44470_e59922), ((4.0 * locals.var_t4_dn5) * assign44470_e59922), ((4.0 * locals.var_t4_dn6) * assign44470_e59922), ((4.0 * locals.var_t4_dn7) * assign44470_e59922), ((4.0 * locals.var_t4_dn8) * assign44470_e59922), ((4.0 * locals.var_t4_dn9) * assign44470_e59922), ((4.0 * locals.var_t4_dn10) * assign44470_e59922), ((4.0 * locals.var_t4_dn13) * assign44470_e59922),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44470_e59925;
        locals.var_tmf2_dn0 = assign44470_e59925_d_n0;
        locals.var_tmf2_dn2 = assign44470_e59925_d_n2;
        locals.var_tmf2_dn4 = assign44470_e59925_d_n4;
        locals.var_tmf2_dn5 = assign44470_e59925_d_n5;
        locals.var_tmf2_dn6 = assign44470_e59925_d_n6;
        locals.var_tmf2_dn7 = assign44470_e59925_d_n7;
        locals.var_tmf2_dn8 = assign44470_e59925_d_n8;
        locals.var_tmf2_dn9 = assign44470_e59925_d_n9;
        locals.var_tmf2_dn10 = assign44470_e59925_d_n10;
        locals.var_tmf2_dn13 = assign44470_e59925_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44480_e59944, assign44480_e59944_d_n0, assign44480_e59944_d_n2, assign44480_e59944_d_n4, assign44480_e59944_d_n5, assign44480_e59944_d_n6, assign44480_e59944_d_n7, assign44480_e59944_d_n8, assign44480_e59944_d_n9, assign44480_e59944_d_n10, assign44480_e59944_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let (assign44480_e59942, assign44480_e59942_d_n0, assign44480_e59942_d_n2, assign44480_e59942_d_n4, assign44480_e59942_d_n5, assign44480_e59942_d_n6, assign44480_e59942_d_n7, assign44480_e59942_d_n8, assign44480_e59942_d_n9, assign44480_e59942_d_n10, assign44480_e59942_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign44480_e59941: f64 = (-locals.var_tmf2);
                (assign44480_e59941, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign44480_e59942, assign44480_e59942_d_n0, assign44480_e59942_d_n2, assign44480_e59942_d_n4, assign44480_e59942_d_n5, assign44480_e59942_d_n6, assign44480_e59942_d_n7, assign44480_e59942_d_n8, assign44480_e59942_d_n9, assign44480_e59942_d_n10, assign44480_e59942_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44480_e59944;
        locals.var_tmf2_dn0 = assign44480_e59944_d_n0;
        locals.var_tmf2_dn2 = assign44480_e59944_d_n2;
        locals.var_tmf2_dn4 = assign44480_e59944_d_n4;
        locals.var_tmf2_dn5 = assign44480_e59944_d_n5;
        locals.var_tmf2_dn6 = assign44480_e59944_d_n6;
        locals.var_tmf2_dn7 = assign44480_e59944_d_n7;
        locals.var_tmf2_dn8 = assign44480_e59944_d_n8;
        locals.var_tmf2_dn9 = assign44480_e59944_d_n9;
        locals.var_tmf2_dn10 = assign44480_e59944_d_n10;
        locals.var_tmf2_dn13 = assign44480_e59944_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44490_e59962, assign44490_e59962_d_n0, assign44490_e59962_d_n2, assign44490_e59962_d_n4, assign44490_e59962_d_n5, assign44490_e59962_d_n6, assign44490_e59962_d_n7, assign44490_e59962_d_n8, assign44490_e59962_d_n9, assign44490_e59962_d_n10, assign44490_e59962_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44490_e59957: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign44490_e59959: f64 = (assign44490_e59957 + locals.var_tmf2);
        let assign44490_e59960: f64 = (assign44490_e59959).sqrt();
        (assign44490_e59960, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign44490_e59960)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign44490_e59960)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44490_e59962;
        locals.var_tmf2_dn0 = assign44490_e59962_d_n0;
        locals.var_tmf2_dn2 = assign44490_e59962_d_n2;
        locals.var_tmf2_dn4 = assign44490_e59962_d_n4;
        locals.var_tmf2_dn5 = assign44490_e59962_d_n5;
        locals.var_tmf2_dn6 = assign44490_e59962_d_n6;
        locals.var_tmf2_dn7 = assign44490_e59962_d_n7;
        locals.var_tmf2_dn8 = assign44490_e59962_d_n8;
        locals.var_tmf2_dn9 = assign44490_e59962_d_n9;
        locals.var_tmf2_dn10 = assign44490_e59962_d_n10;
        locals.var_tmf2_dn13 = assign44490_e59962_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44500_e59981, assign44500_e59981_d_n0, assign44500_e59981_d_n2, assign44500_e59981_d_n4, assign44500_e59981_d_n5, assign44500_e59981_d_n6, assign44500_e59981_d_n7, assign44500_e59981_d_n8, assign44500_e59981_d_n9, assign44500_e59981_d_n10, assign44500_e59981_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44500_e59977: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign44500_e59978: f64 = (1.0 + assign44500_e59977);
        let assign44500_e59979: f64 = (0.5 * assign44500_e59978);
        (assign44500_e59979, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44500_e59981;
        locals.var_t0_dn0 = assign44500_e59981_d_n0;
        locals.var_t0_dn2 = assign44500_e59981_d_n2;
        locals.var_t0_dn4 = assign44500_e59981_d_n4;
        locals.var_t0_dn5 = assign44500_e59981_d_n5;
        locals.var_t0_dn6 = assign44500_e59981_d_n6;
        locals.var_t0_dn7 = assign44500_e59981_d_n7;
        locals.var_t0_dn8 = assign44500_e59981_d_n8;
        locals.var_t0_dn9 = assign44500_e59981_d_n9;
        locals.var_t0_dn10 = assign44500_e59981_d_n10;
        locals.var_t0_dn13 = assign44500_e59981_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44510_e60000, assign44510_e60000_d_n0, assign44510_e60000_d_n2, assign44510_e60000_d_n4, assign44510_e60000_d_n5, assign44510_e60000_d_n6, assign44510_e60000_d_n7, assign44510_e60000_d_n8, assign44510_e60000_d_n9, assign44510_e60000_d_n10, assign44510_e60000_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44510_e59996: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign44510_e59997: f64 = (0.5 * assign44510_e59996);
        let assign44510_e59998: f64 = (locals.var_t4 + assign44510_e59997);
        (assign44510_e59998, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign44510_e60000;
        locals.var_t2_dn0 = assign44510_e60000_d_n0;
        locals.var_t2_dn2 = assign44510_e60000_d_n2;
        locals.var_t2_dn4 = assign44510_e60000_d_n4;
        locals.var_t2_dn5 = assign44510_e60000_d_n5;
        locals.var_t2_dn6 = assign44510_e60000_d_n6;
        locals.var_t2_dn7 = assign44510_e60000_d_n7;
        locals.var_t2_dn8 = assign44510_e60000_d_n8;
        locals.var_t2_dn9 = assign44510_e60000_d_n9;
        locals.var_t2_dn10 = assign44510_e60000_d_n10;
        locals.var_t2_dn13 = assign44510_e60000_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign44520_e60017, assign44520_e60017_d_n0, assign44520_e60017_d_n2, assign44520_e60017_d_n4, assign44520_e60017_d_n5, assign44520_e60017_d_n6, assign44520_e60017_d_n7, assign44520_e60017_d_n8, assign44520_e60017_d_n9, assign44520_e60017_d_n10, assign44520_e60017_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44520_e60014: f64 = (p.p296 + 1.0);
        let assign44520_e60015: f64 = (locals.var_t4 * assign44520_e60014);
        (assign44520_e60015, (locals.var_t4_dn0 * assign44520_e60014), (locals.var_t4_dn2 * assign44520_e60014), (locals.var_t4_dn4 * assign44520_e60014), (locals.var_t4_dn5 * assign44520_e60014), (locals.var_t4_dn6 * assign44520_e60014), (locals.var_t4_dn7 * assign44520_e60014), (locals.var_t4_dn8 * assign44520_e60014), (locals.var_t4_dn9 * assign44520_e60014), (locals.var_t4_dn10 * assign44520_e60014), (locals.var_t4_dn13 * assign44520_e60014),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign44520_e60017;
        locals.var_t3_dn0 = assign44520_e60017_d_n0;
        locals.var_t3_dn2 = assign44520_e60017_d_n2;
        locals.var_t3_dn4 = assign44520_e60017_d_n4;
        locals.var_t3_dn5 = assign44520_e60017_d_n5;
        locals.var_t3_dn6 = assign44520_e60017_d_n6;
        locals.var_t3_dn7 = assign44520_e60017_d_n7;
        locals.var_t3_dn8 = assign44520_e60017_d_n8;
        locals.var_t3_dn9 = assign44520_e60017_d_n9;
        locals.var_t3_dn10 = assign44520_e60017_d_n10;
        locals.var_t3_dn13 = assign44520_e60017_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign44530_e60036, assign44530_e60036_d_n0, assign44530_e60036_d_n2, assign44530_e60036_d_n4, assign44530_e60036_d_n5, assign44530_e60036_d_n6, assign44530_e60036_d_n7, assign44530_e60036_d_n8, assign44530_e60036_d_n9, assign44530_e60036_d_n10, assign44530_e60036_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44530_e60030: f64 = (locals.var_t3 - locals.var_t2);
        let assign44530_e60033: f64 = (0.01 * 0.01);
        let assign44530_e60034: f64 = (assign44530_e60030 - assign44530_e60033);
        (assign44530_e60034, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign44530_e60036;
        locals.var_tmf1_dn0 = assign44530_e60036_d_n0;
        locals.var_tmf1_dn2 = assign44530_e60036_d_n2;
        locals.var_tmf1_dn4 = assign44530_e60036_d_n4;
        locals.var_tmf1_dn5 = assign44530_e60036_d_n5;
        locals.var_tmf1_dn6 = assign44530_e60036_d_n6;
        locals.var_tmf1_dn7 = assign44530_e60036_d_n7;
        locals.var_tmf1_dn8 = assign44530_e60036_d_n8;
        locals.var_tmf1_dn9 = assign44530_e60036_d_n9;
        locals.var_tmf1_dn10 = assign44530_e60036_d_n10;
        locals.var_tmf1_dn13 = assign44530_e60036_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign44540_e60055, assign44540_e60055_d_n0, assign44540_e60055_d_n2, assign44540_e60055_d_n4, assign44540_e60055_d_n5, assign44540_e60055_d_n6, assign44540_e60055_d_n7, assign44540_e60055_d_n8, assign44540_e60055_d_n9, assign44540_e60055_d_n10, assign44540_e60055_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44540_e60049: f64 = (4.0 * locals.var_t3);
        let assign44540_e60052: f64 = (0.01 * 0.01);
        let assign44540_e60053: f64 = (assign44540_e60049 * assign44540_e60052);
        (assign44540_e60053, ((4.0 * locals.var_t3_dn0) * assign44540_e60052), ((4.0 * locals.var_t3_dn2) * assign44540_e60052), ((4.0 * locals.var_t3_dn4) * assign44540_e60052), ((4.0 * locals.var_t3_dn5) * assign44540_e60052), ((4.0 * locals.var_t3_dn6) * assign44540_e60052), ((4.0 * locals.var_t3_dn7) * assign44540_e60052), ((4.0 * locals.var_t3_dn8) * assign44540_e60052), ((4.0 * locals.var_t3_dn9) * assign44540_e60052), ((4.0 * locals.var_t3_dn10) * assign44540_e60052), ((4.0 * locals.var_t3_dn13) * assign44540_e60052),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44540_e60055;
        locals.var_tmf2_dn0 = assign44540_e60055_d_n0;
        locals.var_tmf2_dn2 = assign44540_e60055_d_n2;
        locals.var_tmf2_dn4 = assign44540_e60055_d_n4;
        locals.var_tmf2_dn5 = assign44540_e60055_d_n5;
        locals.var_tmf2_dn6 = assign44540_e60055_d_n6;
        locals.var_tmf2_dn7 = assign44540_e60055_d_n7;
        locals.var_tmf2_dn8 = assign44540_e60055_d_n8;
        locals.var_tmf2_dn9 = assign44540_e60055_d_n9;
        locals.var_tmf2_dn10 = assign44540_e60055_d_n10;
        locals.var_tmf2_dn13 = assign44540_e60055_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44550_e60074, assign44550_e60074_d_n0, assign44550_e60074_d_n2, assign44550_e60074_d_n4, assign44550_e60074_d_n5, assign44550_e60074_d_n6, assign44550_e60074_d_n7, assign44550_e60074_d_n8, assign44550_e60074_d_n9, assign44550_e60074_d_n10, assign44550_e60074_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let (assign44550_e60072, assign44550_e60072_d_n0, assign44550_e60072_d_n2, assign44550_e60072_d_n4, assign44550_e60072_d_n5, assign44550_e60072_d_n6, assign44550_e60072_d_n7, assign44550_e60072_d_n8, assign44550_e60072_d_n9, assign44550_e60072_d_n10, assign44550_e60072_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign44550_e60071: f64 = (-locals.var_tmf2);
                (assign44550_e60071, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign44550_e60072, assign44550_e60072_d_n0, assign44550_e60072_d_n2, assign44550_e60072_d_n4, assign44550_e60072_d_n5, assign44550_e60072_d_n6, assign44550_e60072_d_n7, assign44550_e60072_d_n8, assign44550_e60072_d_n9, assign44550_e60072_d_n10, assign44550_e60072_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44550_e60074;
        locals.var_tmf2_dn0 = assign44550_e60074_d_n0;
        locals.var_tmf2_dn2 = assign44550_e60074_d_n2;
        locals.var_tmf2_dn4 = assign44550_e60074_d_n4;
        locals.var_tmf2_dn5 = assign44550_e60074_d_n5;
        locals.var_tmf2_dn6 = assign44550_e60074_d_n6;
        locals.var_tmf2_dn7 = assign44550_e60074_d_n7;
        locals.var_tmf2_dn8 = assign44550_e60074_d_n8;
        locals.var_tmf2_dn9 = assign44550_e60074_d_n9;
        locals.var_tmf2_dn10 = assign44550_e60074_d_n10;
        locals.var_tmf2_dn13 = assign44550_e60074_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44560_e60092, assign44560_e60092_d_n0, assign44560_e60092_d_n2, assign44560_e60092_d_n4, assign44560_e60092_d_n5, assign44560_e60092_d_n6, assign44560_e60092_d_n7, assign44560_e60092_d_n8, assign44560_e60092_d_n9, assign44560_e60092_d_n10, assign44560_e60092_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44560_e60087: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign44560_e60089: f64 = (assign44560_e60087 + locals.var_tmf2);
        let assign44560_e60090: f64 = (assign44560_e60089).sqrt();
        (assign44560_e60090, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign44560_e60090)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign44560_e60090)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign44560_e60092;
        locals.var_tmf2_dn0 = assign44560_e60092_d_n0;
        locals.var_tmf2_dn2 = assign44560_e60092_d_n2;
        locals.var_tmf2_dn4 = assign44560_e60092_d_n4;
        locals.var_tmf2_dn5 = assign44560_e60092_d_n5;
        locals.var_tmf2_dn6 = assign44560_e60092_d_n6;
        locals.var_tmf2_dn7 = assign44560_e60092_d_n7;
        locals.var_tmf2_dn8 = assign44560_e60092_d_n8;
        locals.var_tmf2_dn9 = assign44560_e60092_d_n9;
        locals.var_tmf2_dn10 = assign44560_e60092_d_n10;
        locals.var_tmf2_dn13 = assign44560_e60092_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign44570_e60111, assign44570_e60111_d_n0, assign44570_e60111_d_n2, assign44570_e60111_d_n4, assign44570_e60111_d_n5, assign44570_e60111_d_n6, assign44570_e60111_d_n7, assign44570_e60111_d_n8, assign44570_e60111_d_n9, assign44570_e60111_d_n10, assign44570_e60111_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44570_e60107: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign44570_e60108: f64 = (1.0 + assign44570_e60107);
        let assign44570_e60109: f64 = (0.5 * assign44570_e60108);
        (assign44570_e60109, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44570_e60111;
        locals.var_t0_dn0 = assign44570_e60111_d_n0;
        locals.var_t0_dn2 = assign44570_e60111_d_n2;
        locals.var_t0_dn4 = assign44570_e60111_d_n4;
        locals.var_t0_dn5 = assign44570_e60111_d_n5;
        locals.var_t0_dn6 = assign44570_e60111_d_n6;
        locals.var_t0_dn7 = assign44570_e60111_d_n7;
        locals.var_t0_dn8 = assign44570_e60111_d_n8;
        locals.var_t0_dn9 = assign44570_e60111_d_n9;
        locals.var_t0_dn10 = assign44570_e60111_d_n10;
        locals.var_t0_dn13 = assign44570_e60111_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44580_e60130, assign44580_e60130_d_n0, assign44580_e60130_d_n2, assign44580_e60130_d_n4, assign44580_e60130_d_n5, assign44580_e60130_d_n6, assign44580_e60130_d_n7, assign44580_e60130_d_n8, assign44580_e60130_d_n9, assign44580_e60130_d_n10, assign44580_e60130_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 != 0.0)) {
        let assign44580_e60126: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign44580_e60127: f64 = (0.5 * assign44580_e60126);
        let assign44580_e60128: f64 = (locals.var_t3 - assign44580_e60127);
        (assign44580_e60128, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign44580_e60130;
        locals.var_t7_dn0 = assign44580_e60130_d_n0;
        locals.var_t7_dn2 = assign44580_e60130_d_n2;
        locals.var_t7_dn4 = assign44580_e60130_d_n4;
        locals.var_t7_dn5 = assign44580_e60130_d_n5;
        locals.var_t7_dn6 = assign44580_e60130_d_n6;
        locals.var_t7_dn7 = assign44580_e60130_d_n7;
        locals.var_t7_dn8 = assign44580_e60130_d_n8;
        locals.var_t7_dn9 = assign44580_e60130_d_n9;
        locals.var_t7_dn10 = assign44580_e60130_d_n10;
        locals.var_t7_dn13 = assign44580_e60130_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign44590_e60144, assign44590_e60144_d_n0, assign44590_e60144_d_n2, assign44590_e60144_d_n4, assign44590_e60144_d_n5, assign44590_e60144_d_n6, assign44590_e60144_d_n7, assign44590_e60144_d_n8, assign44590_e60144_d_n9, assign44590_e60144_d_n10, assign44590_e60144_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1081 == 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign44590_e60144;
        locals.var_t7_dn0 = assign44590_e60144_d_n0;
        locals.var_t7_dn2 = assign44590_e60144_d_n2;
        locals.var_t7_dn4 = assign44590_e60144_d_n4;
        locals.var_t7_dn5 = assign44590_e60144_d_n5;
        locals.var_t7_dn6 = assign44590_e60144_d_n6;
        locals.var_t7_dn7 = assign44590_e60144_d_n7;
        locals.var_t7_dn8 = assign44590_e60144_d_n8;
        locals.var_t7_dn9 = assign44590_e60144_d_n9;
        locals.var_t7_dn10 = assign44590_e60144_d_n10;
        locals.var_t7_dn13 = assign44590_e60144_d_n13;
        locals.var_t7_rv = 0.0;

        let assign44600_e60147: f64 = if locals.var_vdse >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign44600_e60147;
        locals.var_guard1082_rv = 0.0;

        let (assign44610_e60160, assign44610_e60160_d_n0, assign44610_e60160_d_n2,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign44610_e60160;
        locals.var_vdse_eff_dn0 = assign44610_e60160_d_n0;
        locals.var_vdse_eff_dn2 = assign44610_e60160_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let (assign44620_e60174, assign44620_e60174_d_n0, assign44620_e60174_d_n2,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1082 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign44620_e60174;
        locals.var_vdse_eff_dn0 = assign44620_e60174_d_n0;
        locals.var_vdse_eff_dn2 = assign44620_e60174_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let assign44630_e60178: f64 = (20.0 * 1e-12);
        let assign44630_e60179: f64 = if locals.var_vdse_eff < assign44630_e60178 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign44630_e60179;
        locals.var_guard1083_rv = 0.0;

        let (assign44640_e60212,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44640_e60192: f64 = (20.0 + 1.0);
        let assign44640_e60195: f64 = (p.p297 - 1.0);
        let assign44640_e60196: f64 = (assign44640_e60192).powf(assign44640_e60195);
        let assign44640_e60199: f64 = (20.0 + 1.0);
        let assign44640_e60202: f64 = (0.5 * p.p297);
        let assign44640_e60204: f64 = (assign44640_e60202 * 20.0);
        let assign44640_e60205: f64 = (assign44640_e60199 - assign44640_e60204);
        let assign44640_e60206: f64 = (assign44640_e60196 * assign44640_e60205);
        let assign44640_e60209: f64 = (1e-12_f64).powf(p.p297);
        let assign44640_e60210: f64 = (assign44640_e60206 * assign44640_e60209);
        (assign44640_e60210,)
    } else {
        (locals.var_ra_alpha,)
    }
};
        locals.var_ra_alpha = assign44640_e60212;
        locals.var_ra_alpha_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_150(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44650_e60243,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44650_e60225: f64 = (0.5 * p.p297);
        let assign44650_e60228: f64 = (20.0 + 1.0);
        let assign44650_e60231: f64 = (p.p297 - 1.0);
        let assign44650_e60232: f64 = (assign44650_e60228).powf(assign44650_e60231);
        let assign44650_e60233: f64 = (assign44650_e60225 * assign44650_e60232);
        let assign44650_e60235: f64 = (assign44650_e60233 / 20.0);
        let assign44650_e60239: f64 = (p.p297 - 2.0);
        let assign44650_e60240: f64 = (1e-12_f64).powf(assign44650_e60239);
        let assign44650_e60241: f64 = (assign44650_e60235 * assign44650_e60240);
        (assign44650_e60241,)
    } else {
        (locals.var_ra_beta,)
    }
};
        locals.var_ra_beta = assign44650_e60243;
        locals.var_ra_beta_rv = 0.0;

        let (assign44660_e60262, assign44660_e60262_d_n0, assign44660_e60262_d_n2, assign44660_e60262_d_n4, assign44660_e60262_d_n5, assign44660_e60262_d_n6, assign44660_e60262_d_n7, assign44660_e60262_d_n8, assign44660_e60262_d_n9, assign44660_e60262_d_n10, assign44660_e60262_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign44660_e60257: f64 = (locals.var_ra_beta * locals.var_vdse_eff);
        let assign44660_e60259: f64 = (assign44660_e60257 * locals.var_vdse_eff);
        let assign44660_e60260: f64 = (locals.var_ra_alpha + assign44660_e60259);
        (assign44660_e60260, (((locals.var_ra_beta * locals.var_vdse_eff_dn0) * locals.var_vdse_eff) + (assign44660_e60257 * locals.var_vdse_eff_dn0)), (((locals.var_ra_beta * locals.var_vdse_eff_dn2) * locals.var_vdse_eff) + (assign44660_e60257 * locals.var_vdse_eff_dn2)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign44660_e60262;
        locals.var_t1_dn0 = assign44660_e60262_d_n0;
        locals.var_t1_dn2 = assign44660_e60262_d_n2;
        locals.var_t1_dn4 = assign44660_e60262_d_n4;
        locals.var_t1_dn5 = assign44660_e60262_d_n5;
        locals.var_t1_dn6 = assign44660_e60262_d_n6;
        locals.var_t1_dn7 = assign44660_e60262_d_n7;
        locals.var_t1_dn8 = assign44660_e60262_d_n8;
        locals.var_t1_dn9 = assign44660_e60262_d_n9;
        locals.var_t1_dn10 = assign44660_e60262_d_n10;
        locals.var_t1_dn13 = assign44660_e60262_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign44670_e60280, assign44670_e60280_d_n0, assign44670_e60280_d_n2, assign44670_e60280_d_n4, assign44670_e60280_d_n5, assign44670_e60280_d_n6, assign44670_e60280_d_n7, assign44670_e60280_d_n8, assign44670_e60280_d_n9, assign44670_e60280_d_n10, assign44670_e60280_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign44670_e60276: f64 = (locals.var_vdse_eff + 1e-12);
        let assign44670_e60278: f64 = (assign44670_e60276).powf(p.p297);
        (assign44670_e60278, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign44670_e60276).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign44670_e60278 * (p.p297 * (locals.var_vdse_eff_dn0 / assign44670_e60276))) }, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign44670_e60276).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign44670_e60278 * (p.p297 * (locals.var_vdse_eff_dn2 / assign44670_e60276))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign44670_e60280;
        locals.var_t1_dn0 = assign44670_e60280_d_n0;
        locals.var_t1_dn2 = assign44670_e60280_d_n2;
        locals.var_t1_dn4 = assign44670_e60280_d_n4;
        locals.var_t1_dn5 = assign44670_e60280_d_n5;
        locals.var_t1_dn6 = assign44670_e60280_d_n6;
        locals.var_t1_dn7 = assign44670_e60280_d_n7;
        locals.var_t1_dn8 = assign44670_e60280_d_n8;
        locals.var_t1_dn9 = assign44670_e60280_d_n9;
        locals.var_t1_dn10 = assign44670_e60280_d_n10;
        locals.var_t1_dn13 = assign44670_e60280_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign44680_e60295, assign44680_e60295_d_n0, assign44680_e60295_d_n2, assign44680_e60295_d_n4, assign44680_e60295_d_n5, assign44680_e60295_d_n6, assign44680_e60295_d_n7, assign44680_e60295_d_n8, assign44680_e60295_d_n9, assign44680_e60295_d_n10, assign44680_e60295_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44680_e60291: f64 = (locals.var_vdse_eff + 1e-12);
        let assign44680_e60293: f64 = (assign44680_e60291).powf(p.p299);
        (assign44680_e60293, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign44680_e60291).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign44680_e60293 * (p.p299 * (locals.var_vdse_eff_dn0 / assign44680_e60291))) }, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign44680_e60291).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign44680_e60293 * (p.p299 * (locals.var_vdse_eff_dn2 / assign44680_e60291))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign44680_e60295;
        locals.var_t9_dn0 = assign44680_e60295_d_n0;
        locals.var_t9_dn2 = assign44680_e60295_d_n2;
        locals.var_t9_dn4 = assign44680_e60295_d_n4;
        locals.var_t9_dn5 = assign44680_e60295_d_n5;
        locals.var_t9_dn6 = assign44680_e60295_d_n6;
        locals.var_t9_dn7 = assign44680_e60295_d_n7;
        locals.var_t9_dn8 = assign44680_e60295_d_n8;
        locals.var_t9_dn9 = assign44680_e60295_d_n9;
        locals.var_t9_dn10 = assign44680_e60295_d_n10;
        locals.var_t9_dn13 = assign44680_e60295_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign44690_e60316, assign44690_e60316_d_n0, assign44690_e60316_d_n2, assign44690_e60316_d_n4, assign44690_e60316_d_n5, assign44690_e60316_d_n6, assign44690_e60316_d_n7, assign44690_e60316_d_n8, assign44690_e60316_d_n9, assign44690_e60316_d_n10, assign44690_e60316_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44690_e60306: f64 = (locals.var_t7 * locals.var_t1);
        let assign44690_e60309: f64 = (locals.var_vbse * locals.var_uc_rd22);
        let assign44690_e60311: f64 = (assign44690_e60309 * locals.var_t9);
        let assign44690_e60312: f64 = (assign44690_e60306 + assign44690_e60311);
        let assign44690_e60314: f64 = (assign44690_e60312 / locals.var_weff_nf);
        (assign44690_e60314, ((((locals.var_t7_dn0 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn0)) + (((locals.var_vbse_dn0 * locals.var_uc_rd22) * locals.var_t9) + (assign44690_e60309 * locals.var_t9_dn0))) / locals.var_weff_nf), ((((locals.var_t7_dn2 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn2)) + (((locals.var_vbse_dn2 * locals.var_uc_rd22) * locals.var_t9) + (assign44690_e60309 * locals.var_t9_dn2))) / locals.var_weff_nf), ((((locals.var_t7_dn4 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn4)) + (assign44690_e60309 * locals.var_t9_dn4)) / locals.var_weff_nf), ((((locals.var_t7_dn5 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn5)) + (assign44690_e60309 * locals.var_t9_dn5)) / locals.var_weff_nf), ((((locals.var_t7_dn6 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn6)) + (assign44690_e60309 * locals.var_t9_dn6)) / locals.var_weff_nf), ((((locals.var_t7_dn7 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn7)) + (assign44690_e60309 * locals.var_t9_dn7)) / locals.var_weff_nf), ((((locals.var_t7_dn8 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn8)) + (((locals.var_vbse_dn8 * locals.var_uc_rd22) * locals.var_t9) + (assign44690_e60309 * locals.var_t9_dn8))) / locals.var_weff_nf), ((((locals.var_t7_dn9 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn9)) + (assign44690_e60309 * locals.var_t9_dn9)) / locals.var_weff_nf), ((((locals.var_t7_dn10 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn10)) + (assign44690_e60309 * locals.var_t9_dn10)) / locals.var_weff_nf), ((((locals.var_t7_dn13 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn13)) + (assign44690_e60309 * locals.var_t9_dn13)) / locals.var_weff_nf),)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn13,)
    }
};
        locals.var_ra = assign44690_e60316;
        locals.var_ra_dn0 = assign44690_e60316_d_n0;
        locals.var_ra_dn2 = assign44690_e60316_d_n2;
        locals.var_ra_dn4 = assign44690_e60316_d_n4;
        locals.var_ra_dn5 = assign44690_e60316_d_n5;
        locals.var_ra_dn6 = assign44690_e60316_d_n6;
        locals.var_ra_dn7 = assign44690_e60316_d_n7;
        locals.var_ra_dn8 = assign44690_e60316_d_n8;
        locals.var_ra_dn9 = assign44690_e60316_d_n9;
        locals.var_ra_dn10 = assign44690_e60316_d_n10;
        locals.var_ra_dn13 = assign44690_e60316_d_n13;
        locals.var_ra_rv = 0.0;

        let (assign44700_e60329, assign44700_e60329_d_n0, assign44700_e60329_d_n2, assign44700_e60329_d_n4, assign44700_e60329_d_n5, assign44700_e60329_d_n6, assign44700_e60329_d_n7, assign44700_e60329_d_n8, assign44700_e60329_d_n9, assign44700_e60329_d_n10, assign44700_e60329_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44700_e60327: f64 = (locals.var_ra * locals.var_ids0);
        (assign44700_e60327, ((locals.var_ra_dn0 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn0)), ((locals.var_ra_dn2 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn2)), ((locals.var_ra_dn4 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn4)), ((locals.var_ra_dn5 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn5)), ((locals.var_ra_dn6 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn6)), ((locals.var_ra_dn7 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn7)), ((locals.var_ra_dn8 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn8)), ((locals.var_ra_dn9 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn9)), ((locals.var_ra_dn10 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn10)), ((locals.var_ra_dn13 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign44700_e60329;
        locals.var_t0_dn0 = assign44700_e60329_d_n0;
        locals.var_t0_dn2 = assign44700_e60329_d_n2;
        locals.var_t0_dn4 = assign44700_e60329_d_n4;
        locals.var_t0_dn5 = assign44700_e60329_d_n5;
        locals.var_t0_dn6 = assign44700_e60329_d_n6;
        locals.var_t0_dn7 = assign44700_e60329_d_n7;
        locals.var_t0_dn8 = assign44700_e60329_d_n8;
        locals.var_t0_dn9 = assign44700_e60329_d_n9;
        locals.var_t0_dn10 = assign44700_e60329_d_n10;
        locals.var_t0_dn13 = assign44700_e60329_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign44710_e60342, assign44710_e60342_d_n0, assign44710_e60342_d_n2, assign44710_e60342_d_n4, assign44710_e60342_d_n5, assign44710_e60342_d_n6, assign44710_e60342_d_n7, assign44710_e60342_d_n8, assign44710_e60342_d_n9, assign44710_e60342_d_n10, assign44710_e60342_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44710_e60340: f64 = (locals.var_vds + 1e-12);
        (assign44710_e60340, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign44710_e60342;
        locals.var_t1_dn0 = assign44710_e60342_d_n0;
        locals.var_t1_dn2 = assign44710_e60342_d_n2;
        locals.var_t1_dn4 = assign44710_e60342_d_n4;
        locals.var_t1_dn5 = assign44710_e60342_d_n5;
        locals.var_t1_dn6 = assign44710_e60342_d_n6;
        locals.var_t1_dn7 = assign44710_e60342_d_n7;
        locals.var_t1_dn8 = assign44710_e60342_d_n8;
        locals.var_t1_dn9 = assign44710_e60342_d_n9;
        locals.var_t1_dn10 = assign44710_e60342_d_n10;
        locals.var_t1_dn13 = assign44710_e60342_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign44720_e60355, assign44720_e60355_d_n0, assign44720_e60355_d_n2, assign44720_e60355_d_n4, assign44720_e60355_d_n5, assign44720_e60355_d_n6, assign44720_e60355_d_n7, assign44720_e60355_d_n8, assign44720_e60355_d_n9, assign44720_e60355_d_n10, assign44720_e60355_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44720_e60353: f64 = (1.0 / locals.var_t1);
        (assign44720_e60353, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign44720_e60355;
        locals.var_t2_dn0 = assign44720_e60355_d_n0;
        locals.var_t2_dn2 = assign44720_e60355_d_n2;
        locals.var_t2_dn4 = assign44720_e60355_d_n4;
        locals.var_t2_dn5 = assign44720_e60355_d_n5;
        locals.var_t2_dn6 = assign44720_e60355_d_n6;
        locals.var_t2_dn7 = assign44720_e60355_d_n7;
        locals.var_t2_dn8 = assign44720_e60355_d_n8;
        locals.var_t2_dn9 = assign44720_e60355_d_n9;
        locals.var_t2_dn10 = assign44720_e60355_d_n10;
        locals.var_t2_dn13 = assign44720_e60355_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign44730_e60370, assign44730_e60370_d_n0, assign44730_e60370_d_n2, assign44730_e60370_d_n4, assign44730_e60370_d_n5, assign44730_e60370_d_n6, assign44730_e60370_d_n7, assign44730_e60370_d_n8, assign44730_e60370_d_n9, assign44730_e60370_d_n10, assign44730_e60370_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44730_e60367: f64 = (locals.var_t0 * locals.var_t2);
        let assign44730_e60368: f64 = (1.0 + assign44730_e60367);
        (assign44730_e60368, ((locals.var_t0_dn0 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn0)), ((locals.var_t0_dn2 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn2)), ((locals.var_t0_dn4 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn4)), ((locals.var_t0_dn5 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn5)), ((locals.var_t0_dn6 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn6)), ((locals.var_t0_dn7 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn7)), ((locals.var_t0_dn8 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn8)), ((locals.var_t0_dn9 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn9)), ((locals.var_t0_dn10 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn10)), ((locals.var_t0_dn13 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign44730_e60370;
        locals.var_t3_dn0 = assign44730_e60370_d_n0;
        locals.var_t3_dn2 = assign44730_e60370_d_n2;
        locals.var_t3_dn4 = assign44730_e60370_d_n4;
        locals.var_t3_dn5 = assign44730_e60370_d_n5;
        locals.var_t3_dn6 = assign44730_e60370_d_n6;
        locals.var_t3_dn7 = assign44730_e60370_d_n7;
        locals.var_t3_dn8 = assign44730_e60370_d_n8;
        locals.var_t3_dn9 = assign44730_e60370_d_n9;
        locals.var_t3_dn10 = assign44730_e60370_d_n10;
        locals.var_t3_dn13 = assign44730_e60370_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign44740_e60383, assign44740_e60383_d_n0, assign44740_e60383_d_n2, assign44740_e60383_d_n4, assign44740_e60383_d_n5, assign44740_e60383_d_n6, assign44740_e60383_d_n7, assign44740_e60383_d_n8, assign44740_e60383_d_n9, assign44740_e60383_d_n10, assign44740_e60383_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44740_e60381: f64 = (1.0 / locals.var_t3);
        (assign44740_e60381, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign44740_e60383;
        locals.var_t4_dn0 = assign44740_e60383_d_n0;
        locals.var_t4_dn2 = assign44740_e60383_d_n2;
        locals.var_t4_dn4 = assign44740_e60383_d_n4;
        locals.var_t4_dn5 = assign44740_e60383_d_n5;
        locals.var_t4_dn6 = assign44740_e60383_d_n6;
        locals.var_t4_dn7 = assign44740_e60383_d_n7;
        locals.var_t4_dn8 = assign44740_e60383_d_n8;
        locals.var_t4_dn9 = assign44740_e60383_d_n9;
        locals.var_t4_dn10 = assign44740_e60383_d_n10;
        locals.var_t4_dn13 = assign44740_e60383_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign44750_e60396, assign44750_e60396_d_n0, assign44750_e60396_d_n2, assign44750_e60396_d_n4, assign44750_e60396_d_n5, assign44750_e60396_d_n6, assign44750_e60396_d_n7, assign44750_e60396_d_n8, assign44750_e60396_d_n9, assign44750_e60396_d_n10, assign44750_e60396_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 != 0.0)) {
        let assign44750_e60394: f64 = (locals.var_ids0 * locals.var_t4);
        (assign44750_e60394, ((locals.var_ids0_dn0 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn0)), ((locals.var_ids0_dn2 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn2)), ((locals.var_ids0_dn4 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn4)), ((locals.var_ids0_dn5 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn5)), ((locals.var_ids0_dn6 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn6)), ((locals.var_ids0_dn7 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn7)), ((locals.var_ids0_dn8 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn8)), ((locals.var_ids0_dn9 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn9)), ((locals.var_ids0_dn10 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn10)), ((locals.var_ids0_dn13 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn13)),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign44750_e60396;
        locals.var_ids_dn0 = assign44750_e60396_d_n0;
        locals.var_ids_dn2 = assign44750_e60396_d_n2;
        locals.var_ids_dn4 = assign44750_e60396_d_n4;
        locals.var_ids_dn5 = assign44750_e60396_d_n5;
        locals.var_ids_dn6 = assign44750_e60396_d_n6;
        locals.var_ids_dn7 = assign44750_e60396_d_n7;
        locals.var_ids_dn8 = assign44750_e60396_d_n8;
        locals.var_ids_dn9 = assign44750_e60396_d_n9;
        locals.var_ids_dn10 = assign44750_e60396_d_n10;
        locals.var_ids_dn13 = assign44750_e60396_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign44760_e60408, assign44760_e60408_d_n0, assign44760_e60408_d_n2, assign44760_e60408_d_n4, assign44760_e60408_d_n5, assign44760_e60408_d_n6, assign44760_e60408_d_n7, assign44760_e60408_d_n8, assign44760_e60408_d_n9, assign44760_e60408_d_n10, assign44760_e60408_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 == 0.0)) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign44760_e60408;
        locals.var_ids_dn0 = assign44760_e60408_d_n0;
        locals.var_ids_dn2 = assign44760_e60408_d_n2;
        locals.var_ids_dn4 = assign44760_e60408_d_n4;
        locals.var_ids_dn5 = assign44760_e60408_d_n5;
        locals.var_ids_dn6 = assign44760_e60408_d_n6;
        locals.var_ids_dn7 = assign44760_e60408_d_n7;
        locals.var_ids_dn8 = assign44760_e60408_d_n8;
        locals.var_ids_dn9 = assign44760_e60408_d_n9;
        locals.var_ids_dn10 = assign44760_e60408_d_n10;
        locals.var_ids_dn13 = assign44760_e60408_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign44770_e60420, assign44770_e60420_d_n0, assign44770_e60420_d_n2, assign44770_e60420_d_n4, assign44770_e60420_d_n5, assign44770_e60420_d_n6, assign44770_e60420_d_n7, assign44770_e60420_d_n8, assign44770_e60420_d_n9, assign44770_e60420_d_n10, assign44770_e60420_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1080 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn13,)
    }
};
        locals.var_ra = assign44770_e60420;
        locals.var_ra_dn0 = assign44770_e60420_d_n0;
        locals.var_ra_dn2 = assign44770_e60420_d_n2;
        locals.var_ra_dn4 = assign44770_e60420_d_n4;
        locals.var_ra_dn5 = assign44770_e60420_d_n5;
        locals.var_ra_dn6 = assign44770_e60420_d_n6;
        locals.var_ra_dn7 = assign44770_e60420_d_n7;
        locals.var_ra_dn8 = assign44770_e60420_d_n8;
        locals.var_ra_dn9 = assign44770_e60420_d_n9;
        locals.var_ra_dn10 = assign44770_e60420_d_n10;
        locals.var_ra_dn13 = assign44770_e60420_d_n13;
        locals.var_ra_rv = 0.0;

        let (assign44780_e60438, assign44780_e60438_d_n0, assign44780_e60438_d_n2, assign44780_e60438_d_n4, assign44780_e60438_d_n5, assign44780_e60438_d_n6, assign44780_e60438_d_n7, assign44780_e60438_d_n8, assign44780_e60438_d_n9, assign44780_e60438_d_n10, assign44780_e60438_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44780_e60428: f64 = (-0.5);
        let assign44780_e60431: f64 = (locals.var_q_sub0__blk864 + locals.var_q_subl__blk865);
        let assign44780_e60433: f64 = (assign44780_e60431 + locals.var_q_sub0_dep__blk867);
        let assign44780_e60435: f64 = (assign44780_e60433 + locals.var_q_subl_dep__blk869);
        let assign44780_e60436: f64 = (assign44780_e60428 * assign44780_e60435);
        (assign44780_e60436, (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn0 + locals.var_q_subl__blk865_dn0) + locals.var_q_sub0_dep__blk867_dn0) + locals.var_q_subl_dep__blk869_dn0)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn2 + locals.var_q_subl__blk865_dn2) + locals.var_q_sub0_dep__blk867_dn2) + locals.var_q_subl_dep__blk869_dn2)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn4 + locals.var_q_subl__blk865_dn4) + locals.var_q_sub0_dep__blk867_dn4) + locals.var_q_subl_dep__blk869_dn4)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn5 + locals.var_q_subl__blk865_dn5) + locals.var_q_sub0_dep__blk867_dn5) + locals.var_q_subl_dep__blk869_dn5)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn6 + locals.var_q_subl__blk865_dn6) + locals.var_q_sub0_dep__blk867_dn6) + locals.var_q_subl_dep__blk869_dn6)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn7 + locals.var_q_subl__blk865_dn7) + locals.var_q_sub0_dep__blk867_dn7) + locals.var_q_subl_dep__blk869_dn7)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn8 + locals.var_q_subl__blk865_dn8) + locals.var_q_sub0_dep__blk867_dn8) + locals.var_q_subl_dep__blk869_dn8)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn9 + locals.var_q_subl__blk865_dn9) + locals.var_q_sub0_dep__blk867_dn9) + locals.var_q_subl_dep__blk869_dn9)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn10 + locals.var_q_subl__blk865_dn10) + locals.var_q_sub0_dep__blk867_dn10) + locals.var_q_subl_dep__blk869_dn10)), (assign44780_e60428 * (((locals.var_q_sub0__blk864_dn13 + locals.var_q_subl__blk865_dn13) + locals.var_q_sub0_dep__blk867_dn13) + locals.var_q_subl_dep__blk869_dn13)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    }
};
        locals.var_qbu = assign44780_e60438;
        locals.var_qbu_dn0 = assign44780_e60438_d_n0;
        locals.var_qbu_dn2 = assign44780_e60438_d_n2;
        locals.var_qbu_dn4 = assign44780_e60438_d_n4;
        locals.var_qbu_dn5 = assign44780_e60438_d_n5;
        locals.var_qbu_dn6 = assign44780_e60438_d_n6;
        locals.var_qbu_dn7 = assign44780_e60438_d_n7;
        locals.var_qbu_dn8 = assign44780_e60438_d_n8;
        locals.var_qbu_dn9 = assign44780_e60438_d_n9;
        locals.var_qbu_dn10 = assign44780_e60438_d_n10;
        locals.var_qbu_dn13 = assign44780_e60438_d_n13;
        locals.var_qbu_rv = 0.0;

        let (assign44790_e60460, assign44790_e60460_d_n0, assign44790_e60460_d_n2, assign44790_e60460_d_n4, assign44790_e60460_d_n5, assign44790_e60460_d_n6, assign44790_e60460_d_n7, assign44790_e60460_d_n8, assign44790_e60460_d_n9, assign44790_e60460_d_n10, assign44790_e60460_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44790_e60446: f64 = (-0.5);
        let assign44790_e60449: f64 = (locals.var_q_n0__blk892 + locals.var_q_nl__blk893);
        let assign44790_e60451: f64 = (assign44790_e60449 + locals.var_q_s0_dep__blk895);
        let assign44790_e60453: f64 = (assign44790_e60451 + locals.var_q_sl_dep__blk896);
        let assign44790_e60455: f64 = (assign44790_e60453 + locals.var_q_b0_dep__blk866);
        let assign44790_e60457: f64 = (assign44790_e60455 + locals.var_q_bl_dep__blk868);
        let assign44790_e60458: f64 = (assign44790_e60446 * assign44790_e60457);
        (assign44790_e60458, (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn0 + locals.var_q_nl__blk893_dn0) + locals.var_q_s0_dep__blk895_dn0) + locals.var_q_sl_dep__blk896_dn0) + locals.var_q_b0_dep__blk866_dn0) + locals.var_q_bl_dep__blk868_dn0)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn2 + locals.var_q_nl__blk893_dn2) + locals.var_q_s0_dep__blk895_dn2) + locals.var_q_sl_dep__blk896_dn2) + locals.var_q_b0_dep__blk866_dn2) + locals.var_q_bl_dep__blk868_dn2)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn4 + locals.var_q_nl__blk893_dn4) + locals.var_q_s0_dep__blk895_dn4) + locals.var_q_sl_dep__blk896_dn4) + locals.var_q_b0_dep__blk866_dn4) + locals.var_q_bl_dep__blk868_dn4)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn5 + locals.var_q_nl__blk893_dn5) + locals.var_q_s0_dep__blk895_dn5) + locals.var_q_sl_dep__blk896_dn5) + locals.var_q_b0_dep__blk866_dn5) + locals.var_q_bl_dep__blk868_dn5)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn6 + locals.var_q_nl__blk893_dn6) + locals.var_q_s0_dep__blk895_dn6) + locals.var_q_sl_dep__blk896_dn6) + locals.var_q_b0_dep__blk866_dn6) + locals.var_q_bl_dep__blk868_dn6)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn7 + locals.var_q_nl__blk893_dn7) + locals.var_q_s0_dep__blk895_dn7) + locals.var_q_sl_dep__blk896_dn7) + locals.var_q_b0_dep__blk866_dn7) + locals.var_q_bl_dep__blk868_dn7)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn8 + locals.var_q_nl__blk893_dn8) + locals.var_q_s0_dep__blk895_dn8) + locals.var_q_sl_dep__blk896_dn8) + locals.var_q_b0_dep__blk866_dn8) + locals.var_q_bl_dep__blk868_dn8)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn9 + locals.var_q_nl__blk893_dn9) + locals.var_q_s0_dep__blk895_dn9) + locals.var_q_sl_dep__blk896_dn9) + locals.var_q_b0_dep__blk866_dn9) + locals.var_q_bl_dep__blk868_dn9)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn10 + locals.var_q_nl__blk893_dn10) + locals.var_q_s0_dep__blk895_dn10) + locals.var_q_sl_dep__blk896_dn10) + locals.var_q_b0_dep__blk866_dn10) + locals.var_q_bl_dep__blk868_dn10)), (assign44790_e60446 * (((((locals.var_q_n0__blk892_dn13 + locals.var_q_nl__blk893_dn13) + locals.var_q_s0_dep__blk895_dn13) + locals.var_q_sl_dep__blk896_dn13) + locals.var_q_b0_dep__blk866_dn13) + locals.var_q_bl_dep__blk868_dn13)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign44790_e60460;
        locals.var_qiu_dn0 = assign44790_e60460_d_n0;
        locals.var_qiu_dn2 = assign44790_e60460_d_n2;
        locals.var_qiu_dn4 = assign44790_e60460_d_n4;
        locals.var_qiu_dn5 = assign44790_e60460_d_n5;
        locals.var_qiu_dn6 = assign44790_e60460_d_n6;
        locals.var_qiu_dn7 = assign44790_e60460_d_n7;
        locals.var_qiu_dn8 = assign44790_e60460_d_n8;
        locals.var_qiu_dn9 = assign44790_e60460_d_n9;
        locals.var_qiu_dn10 = assign44790_e60460_d_n10;
        locals.var_qiu_dn13 = assign44790_e60460_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign44800_e60469, assign44800_e60469_d_n0, assign44800_e60469_d_n2, assign44800_e60469_d_n4, assign44800_e60469_d_n5, assign44800_e60469_d_n6, assign44800_e60469_d_n7, assign44800_e60469_d_n8, assign44800_e60469_d_n9, assign44800_e60469_d_n10, assign44800_e60469_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign44800_e60469;
        locals.var_qdrat_dn0 = assign44800_e60469_d_n0;
        locals.var_qdrat_dn2 = assign44800_e60469_d_n2;
        locals.var_qdrat_dn4 = assign44800_e60469_d_n4;
        locals.var_qdrat_dn5 = assign44800_e60469_d_n5;
        locals.var_qdrat_dn6 = assign44800_e60469_d_n6;
        locals.var_qdrat_dn7 = assign44800_e60469_d_n7;
        locals.var_qdrat_dn8 = assign44800_e60469_d_n8;
        locals.var_qdrat_dn9 = assign44800_e60469_d_n9;
        locals.var_qdrat_dn10 = assign44800_e60469_d_n10;
        locals.var_qdrat_dn13 = assign44800_e60469_d_n13;
        locals.var_qdrat_rv = 0.0;

        let (assign44810_e60483, assign44810_e60483_d_n0, assign44810_e60483_d_n2, assign44810_e60483_d_n4, assign44810_e60483_d_n5, assign44810_e60483_d_n6, assign44810_e60483_d_n7, assign44810_e60483_d_n8, assign44810_e60483_d_n9, assign44810_e60483_d_n10, assign44810_e60483_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44810_e60477: f64 = (-0.5);
        let assign44810_e60480: f64 = (locals.var_q_n0__blk892 + locals.var_q_nl__blk893);
        let assign44810_e60481: f64 = (assign44810_e60477 * assign44810_e60480);
        (assign44810_e60481, (assign44810_e60477 * (locals.var_q_n0__blk892_dn0 + locals.var_q_nl__blk893_dn0)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn2 + locals.var_q_nl__blk893_dn2)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn4 + locals.var_q_nl__blk893_dn4)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn5 + locals.var_q_nl__blk893_dn5)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn6 + locals.var_q_nl__blk893_dn6)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn7 + locals.var_q_nl__blk893_dn7)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn8 + locals.var_q_nl__blk893_dn8)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn9 + locals.var_q_nl__blk893_dn9)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn10 + locals.var_q_nl__blk893_dn10)), (assign44810_e60477 * (locals.var_q_n0__blk892_dn13 + locals.var_q_nl__blk893_dn13)),)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn13,)
    }
};
        locals.var_qiu_noi = assign44810_e60483;
        locals.var_qiu_noi_dn0 = assign44810_e60483_d_n0;
        locals.var_qiu_noi_dn2 = assign44810_e60483_d_n2;
        locals.var_qiu_noi_dn4 = assign44810_e60483_d_n4;
        locals.var_qiu_noi_dn5 = assign44810_e60483_d_n5;
        locals.var_qiu_noi_dn6 = assign44810_e60483_d_n6;
        locals.var_qiu_noi_dn7 = assign44810_e60483_d_n7;
        locals.var_qiu_noi_dn8 = assign44810_e60483_d_n8;
        locals.var_qiu_noi_dn9 = assign44810_e60483_d_n9;
        locals.var_qiu_noi_dn10 = assign44810_e60483_d_n10;
        locals.var_qiu_noi_dn13 = assign44810_e60483_d_n13;
        locals.var_qiu_noi_rv = 0.0;

        let (assign44820_e60493, assign44820_e60493_d_n0, assign44820_e60493_d_n2, assign44820_e60493_d_n4, assign44820_e60493_d_n5, assign44820_e60493_d_n6, assign44820_e60493_d_n7, assign44820_e60493_d_n8, assign44820_e60493_d_n9, assign44820_e60493_d_n10, assign44820_e60493_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign44820_e60491: f64 = (-locals.var_q_n0__blk892);
        (assign44820_e60491, (-locals.var_q_n0__blk892_dn0), (-locals.var_q_n0__blk892_dn2), (-locals.var_q_n0__blk892_dn4), (-locals.var_q_n0__blk892_dn5), (-locals.var_q_n0__blk892_dn6), (-locals.var_q_n0__blk892_dn7), (-locals.var_q_n0__blk892_dn8), (-locals.var_q_n0__blk892_dn9), (-locals.var_q_n0__blk892_dn10), (-locals.var_q_n0__blk892_dn13),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn13,)
    }
};
        locals.var_qn0 = assign44820_e60493;
        locals.var_qn0_dn0 = assign44820_e60493_d_n0;
        locals.var_qn0_dn2 = assign44820_e60493_d_n2;
        locals.var_qn0_dn4 = assign44820_e60493_d_n4;
        locals.var_qn0_dn5 = assign44820_e60493_d_n5;
        locals.var_qn0_dn6 = assign44820_e60493_d_n6;
        locals.var_qn0_dn7 = assign44820_e60493_d_n7;
        locals.var_qn0_dn8 = assign44820_e60493_d_n8;
        locals.var_qn0_dn9 = assign44820_e60493_d_n9;
        locals.var_qn0_dn10 = assign44820_e60493_d_n10;
        locals.var_qn0_dn13 = assign44820_e60493_d_n13;
        locals.var_qn0_rv = 0.0;

        let (assign44830_e60502, assign44830_e60502_d_n0, assign44830_e60502_d_n2, assign44830_e60502_d_n4, assign44830_e60502_d_n5, assign44830_e60502_d_n6, assign44830_e60502_d_n7, assign44830_e60502_d_n8, assign44830_e60502_d_n9, assign44830_e60502_d_n10, assign44830_e60502_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_ey_acc, locals.var_ey_acc_dn0, locals.var_ey_acc_dn2, locals.var_ey_acc_dn4, locals.var_ey_acc_dn5, locals.var_ey_acc_dn6, locals.var_ey_acc_dn7, locals.var_ey_acc_dn8, locals.var_ey_acc_dn9, locals.var_ey_acc_dn10, locals.var_ey_acc_dn13,)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    }
};
        locals.var_ey = assign44830_e60502;
        locals.var_ey_dn0 = assign44830_e60502_d_n0;
        locals.var_ey_dn2 = assign44830_e60502_d_n2;
        locals.var_ey_dn4 = assign44830_e60502_d_n4;
        locals.var_ey_dn5 = assign44830_e60502_d_n5;
        locals.var_ey_dn6 = assign44830_e60502_d_n6;
        locals.var_ey_dn7 = assign44830_e60502_d_n7;
        locals.var_ey_dn8 = assign44830_e60502_d_n8;
        locals.var_ey_dn9 = assign44830_e60502_d_n9;
        locals.var_ey_dn10 = assign44830_e60502_d_n10;
        locals.var_ey_dn13 = assign44830_e60502_d_n13;
        locals.var_ey_rv = 0.0;

        let assign44840_e60509: f64 = if ((locals.var_qn0 < 1e-25) || (locals.var_qiu < 1e-25)) { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign44840_e60509;
        locals.var_guard1084_rv = 0.0;

        let (assign44850_e60520,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) && (locals.var_guard1084 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign44850_e60520;
        locals.var_flg_noqi_rv = 0.0;

        let (assign44860_e60531, assign44860_e60531_d_n0, assign44860_e60531_d_n2, assign44860_e60531_d_n4, assign44860_e60531_d_n5, assign44860_e60531_d_n6, assign44860_e60531_d_n7, assign44860_e60531_d_n8, assign44860_e60531_d_n9, assign44860_e60531_d_n10, assign44860_e60531_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    } else {
        (locals.var_vbi_dep__blk1091, locals.var_vbi_dep__blk1091_dn0, locals.var_vbi_dep__blk1091_dn2, locals.var_vbi_dep__blk1091_dn4, locals.var_vbi_dep__blk1091_dn5, locals.var_vbi_dep__blk1091_dn6, locals.var_vbi_dep__blk1091_dn7, locals.var_vbi_dep__blk1091_dn8, locals.var_vbi_dep__blk1091_dn9, locals.var_vbi_dep__blk1091_dn10, locals.var_vbi_dep__blk1091_dn13,)
    }
};
        locals.var_vbi_dep__blk1091 = assign44860_e60531;
        locals.var_vbi_dep__blk1091_dn0 = assign44860_e60531_d_n0;
        locals.var_vbi_dep__blk1091_dn2 = assign44860_e60531_d_n2;
        locals.var_vbi_dep__blk1091_dn4 = assign44860_e60531_d_n4;
        locals.var_vbi_dep__blk1091_dn5 = assign44860_e60531_d_n5;
        locals.var_vbi_dep__blk1091_dn6 = assign44860_e60531_d_n6;
        locals.var_vbi_dep__blk1091_dn7 = assign44860_e60531_d_n7;
        locals.var_vbi_dep__blk1091_dn8 = assign44860_e60531_d_n8;
        locals.var_vbi_dep__blk1091_dn9 = assign44860_e60531_d_n9;
        locals.var_vbi_dep__blk1091_dn10 = assign44860_e60531_d_n10;
        locals.var_vbi_dep__blk1091_dn13 = assign44860_e60531_d_n13;
        locals.var_vbi_dep__blk1091_rv = 0.0;

        let (assign44870_e60544, assign44870_e60544_d_n0, assign44870_e60544_d_n2, assign44870_e60544_d_n4, assign44870_e60544_d_n5, assign44870_e60544_d_n6, assign44870_e60544_d_n7, assign44870_e60544_d_n8, assign44870_e60544_d_n9, assign44870_e60544_d_n10, assign44870_e60544_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign44870_e60542: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        (assign44870_e60542, (1.6021918e-19 * locals.var_uc_ndepm_dn0), (1.6021918e-19 * locals.var_uc_ndepm_dn2), (1.6021918e-19 * locals.var_uc_ndepm_dn4), (1.6021918e-19 * locals.var_uc_ndepm_dn5), (1.6021918e-19 * locals.var_uc_ndepm_dn6), (1.6021918e-19 * locals.var_uc_ndepm_dn7), (1.6021918e-19 * locals.var_uc_ndepm_dn8), (1.6021918e-19 * locals.var_uc_ndepm_dn9), (1.6021918e-19 * locals.var_uc_ndepm_dn10), (1.6021918e-19 * locals.var_uc_ndepm_dn13),)
    } else {
        (locals.var_q_ndepm__blk1133, locals.var_q_ndepm__blk1133_dn0, locals.var_q_ndepm__blk1133_dn2, locals.var_q_ndepm__blk1133_dn4, locals.var_q_ndepm__blk1133_dn5, locals.var_q_ndepm__blk1133_dn6, locals.var_q_ndepm__blk1133_dn7, locals.var_q_ndepm__blk1133_dn8, locals.var_q_ndepm__blk1133_dn9, locals.var_q_ndepm__blk1133_dn10, locals.var_q_ndepm__blk1133_dn13,)
    }
};
        locals.var_q_ndepm__blk1133 = assign44870_e60544;
        locals.var_q_ndepm__blk1133_dn0 = assign44870_e60544_d_n0;
        locals.var_q_ndepm__blk1133_dn2 = assign44870_e60544_d_n2;
        locals.var_q_ndepm__blk1133_dn4 = assign44870_e60544_d_n4;
        locals.var_q_ndepm__blk1133_dn5 = assign44870_e60544_d_n5;
        locals.var_q_ndepm__blk1133_dn6 = assign44870_e60544_d_n6;
        locals.var_q_ndepm__blk1133_dn7 = assign44870_e60544_d_n7;
        locals.var_q_ndepm__blk1133_dn8 = assign44870_e60544_d_n8;
        locals.var_q_ndepm__blk1133_dn9 = assign44870_e60544_d_n9;
        locals.var_q_ndepm__blk1133_dn10 = assign44870_e60544_d_n10;
        locals.var_q_ndepm__blk1133_dn13 = assign44870_e60544_d_n13;
        locals.var_q_ndepm__blk1133_rv = 0.0;

        let (assign44880_e60559, assign44880_e60559_d_n0, assign44880_e60559_d_n2, assign44880_e60559_d_n4, assign44880_e60559_d_n5, assign44880_e60559_d_n6, assign44880_e60559_d_n7, assign44880_e60559_d_n8, assign44880_e60559_d_n9, assign44880_e60559_d_n10, assign44880_e60559_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign44880_e60555: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        let assign44880_e60557: f64 = (assign44880_e60555 * 1.034943e-10);
        (assign44880_e60557, ((1.6021918e-19 * locals.var_uc_ndepm_dn0) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn2) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn4) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn5) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn6) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn7) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn8) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn9) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn10) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn13) * 1.034943e-10),)
    } else {
        (locals.var_q_ndepm_esi__blk1114, locals.var_q_ndepm_esi__blk1114_dn0, locals.var_q_ndepm_esi__blk1114_dn2, locals.var_q_ndepm_esi__blk1114_dn4, locals.var_q_ndepm_esi__blk1114_dn5, locals.var_q_ndepm_esi__blk1114_dn6, locals.var_q_ndepm_esi__blk1114_dn7, locals.var_q_ndepm_esi__blk1114_dn8, locals.var_q_ndepm_esi__blk1114_dn9, locals.var_q_ndepm_esi__blk1114_dn10, locals.var_q_ndepm_esi__blk1114_dn13,)
    }
};
        locals.var_q_ndepm_esi__blk1114 = assign44880_e60559;
        locals.var_q_ndepm_esi__blk1114_dn0 = assign44880_e60559_d_n0;
        locals.var_q_ndepm_esi__blk1114_dn2 = assign44880_e60559_d_n2;
        locals.var_q_ndepm_esi__blk1114_dn4 = assign44880_e60559_d_n4;
        locals.var_q_ndepm_esi__blk1114_dn5 = assign44880_e60559_d_n5;
        locals.var_q_ndepm_esi__blk1114_dn6 = assign44880_e60559_d_n6;
        locals.var_q_ndepm_esi__blk1114_dn7 = assign44880_e60559_d_n7;
        locals.var_q_ndepm_esi__blk1114_dn8 = assign44880_e60559_d_n8;
        locals.var_q_ndepm_esi__blk1114_dn9 = assign44880_e60559_d_n9;
        locals.var_q_ndepm_esi__blk1114_dn10 = assign44880_e60559_d_n10;
        locals.var_q_ndepm_esi__blk1114_dn13 = assign44880_e60559_d_n13;
        locals.var_q_ndepm_esi__blk1114_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_151(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign44890_e60574, assign44890_e60574_d_n0, assign44890_e60574_d_n2, assign44890_e60574_d_n4, assign44890_e60574_d_n5, assign44890_e60574_d_n6, assign44890_e60574_d_n7, assign44890_e60574_d_n8, assign44890_e60574_d_n9, assign44890_e60574_d_n10, assign44890_e60574_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign44890_e60570: f64 = (2.0 * 1.034943e-10);
        let assign44890_e60572: f64 = (assign44890_e60570 / locals.var_q_ndepm__blk1133);
        (assign44890_e60572, (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn0) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn2) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn4) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn5) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn6) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn7) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn8) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn9) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn10) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))), (-((assign44890_e60570 * locals.var_q_ndepm__blk1133_dn13) / (locals.var_q_ndepm__blk1133 * locals.var_q_ndepm__blk1133))),)
    } else {
        (locals.var_c_2esipq_ndepm__blk1136, locals.var_c_2esipq_ndepm__blk1136_dn0, locals.var_c_2esipq_ndepm__blk1136_dn2, locals.var_c_2esipq_ndepm__blk1136_dn4, locals.var_c_2esipq_ndepm__blk1136_dn5, locals.var_c_2esipq_ndepm__blk1136_dn6, locals.var_c_2esipq_ndepm__blk1136_dn7, locals.var_c_2esipq_ndepm__blk1136_dn8, locals.var_c_2esipq_ndepm__blk1136_dn9, locals.var_c_2esipq_ndepm__blk1136_dn10, locals.var_c_2esipq_ndepm__blk1136_dn13,)
    }
};
        locals.var_c_2esipq_ndepm__blk1136 = assign44890_e60574;
        locals.var_c_2esipq_ndepm__blk1136_dn0 = assign44890_e60574_d_n0;
        locals.var_c_2esipq_ndepm__blk1136_dn2 = assign44890_e60574_d_n2;
        locals.var_c_2esipq_ndepm__blk1136_dn4 = assign44890_e60574_d_n4;
        locals.var_c_2esipq_ndepm__blk1136_dn5 = assign44890_e60574_d_n5;
        locals.var_c_2esipq_ndepm__blk1136_dn6 = assign44890_e60574_d_n6;
        locals.var_c_2esipq_ndepm__blk1136_dn7 = assign44890_e60574_d_n7;
        locals.var_c_2esipq_ndepm__blk1136_dn8 = assign44890_e60574_d_n8;
        locals.var_c_2esipq_ndepm__blk1136_dn9 = assign44890_e60574_d_n9;
        locals.var_c_2esipq_ndepm__blk1136_dn10 = assign44890_e60574_d_n10;
        locals.var_c_2esipq_ndepm__blk1136_dn13 = assign44890_e60574_d_n13;
        locals.var_c_2esipq_ndepm__blk1136_rv = 0.0;

        let (assign44900_e60587, assign44900_e60587_d_n0, assign44900_e60587_d_n2, assign44900_e60587_d_n4, assign44900_e60587_d_n5, assign44900_e60587_d_n6, assign44900_e60587_d_n7, assign44900_e60587_d_n8, assign44900_e60587_d_n9, assign44900_e60587_d_n10, assign44900_e60587_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign44900_e60585: f64 = (locals.var_uc_ndepm / locals.var_ef_nsubc);
        (assign44900_e60585, (((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)),)
    } else {
        (locals.var_ndepmpnsub__blk1130, locals.var_ndepmpnsub__blk1130_dn0, locals.var_ndepmpnsub__blk1130_dn2, locals.var_ndepmpnsub__blk1130_dn4, locals.var_ndepmpnsub__blk1130_dn5, locals.var_ndepmpnsub__blk1130_dn6, locals.var_ndepmpnsub__blk1130_dn7, locals.var_ndepmpnsub__blk1130_dn8, locals.var_ndepmpnsub__blk1130_dn9, locals.var_ndepmpnsub__blk1130_dn10, locals.var_ndepmpnsub__blk1130_dn13,)
    }
};
        locals.var_ndepmpnsub__blk1130 = assign44900_e60587;
        locals.var_ndepmpnsub__blk1130_dn0 = assign44900_e60587_d_n0;
        locals.var_ndepmpnsub__blk1130_dn2 = assign44900_e60587_d_n2;
        locals.var_ndepmpnsub__blk1130_dn4 = assign44900_e60587_d_n4;
        locals.var_ndepmpnsub__blk1130_dn5 = assign44900_e60587_d_n5;
        locals.var_ndepmpnsub__blk1130_dn6 = assign44900_e60587_d_n6;
        locals.var_ndepmpnsub__blk1130_dn7 = assign44900_e60587_d_n7;
        locals.var_ndepmpnsub__blk1130_dn8 = assign44900_e60587_d_n8;
        locals.var_ndepmpnsub__blk1130_dn9 = assign44900_e60587_d_n9;
        locals.var_ndepmpnsub__blk1130_dn10 = assign44900_e60587_d_n10;
        locals.var_ndepmpnsub__blk1130_dn13 = assign44900_e60587_d_n13;
        locals.var_ndepmpnsub__blk1130_rv = 0.0;

        let (assign44910_e60602, assign44910_e60602_d_n0, assign44910_e60602_d_n2, assign44910_e60602_d_n4, assign44910_e60602_d_n5, assign44910_e60602_d_n6, assign44910_e60602_d_n7, assign44910_e60602_d_n8, assign44910_e60602_d_n9, assign44910_e60602_d_n10, assign44910_e60602_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign44910_e60599: f64 = (1.0 + locals.var_ndepmpnsub__blk1130);
        let assign44910_e60600: f64 = (1.0 / assign44910_e60599);
        (assign44910_e60600, (-(locals.var_ndepmpnsub__blk1130_dn0 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn2 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn4 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn5 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn6 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn7 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn8 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn9 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn10 / (assign44910_e60599 * assign44910_e60599))), (-(locals.var_ndepmpnsub__blk1130_dn13 / (assign44910_e60599 * assign44910_e60599))),)
    } else {
        (locals.var_ndepmpnsub_inv1__blk1129, locals.var_ndepmpnsub_inv1__blk1129_dn0, locals.var_ndepmpnsub_inv1__blk1129_dn2, locals.var_ndepmpnsub_inv1__blk1129_dn4, locals.var_ndepmpnsub_inv1__blk1129_dn5, locals.var_ndepmpnsub_inv1__blk1129_dn6, locals.var_ndepmpnsub_inv1__blk1129_dn7, locals.var_ndepmpnsub_inv1__blk1129_dn8, locals.var_ndepmpnsub_inv1__blk1129_dn9, locals.var_ndepmpnsub_inv1__blk1129_dn10, locals.var_ndepmpnsub_inv1__blk1129_dn13,)
    }
};
        locals.var_ndepmpnsub_inv1__blk1129 = assign44910_e60602;
        locals.var_ndepmpnsub_inv1__blk1129_dn0 = assign44910_e60602_d_n0;
        locals.var_ndepmpnsub_inv1__blk1129_dn2 = assign44910_e60602_d_n2;
        locals.var_ndepmpnsub_inv1__blk1129_dn4 = assign44910_e60602_d_n4;
        locals.var_ndepmpnsub_inv1__blk1129_dn5 = assign44910_e60602_d_n5;
        locals.var_ndepmpnsub_inv1__blk1129_dn6 = assign44910_e60602_d_n6;
        locals.var_ndepmpnsub_inv1__blk1129_dn7 = assign44910_e60602_d_n7;
        locals.var_ndepmpnsub_inv1__blk1129_dn8 = assign44910_e60602_d_n8;
        locals.var_ndepmpnsub_inv1__blk1129_dn9 = assign44910_e60602_d_n9;
        locals.var_ndepmpnsub_inv1__blk1129_dn10 = assign44910_e60602_d_n10;
        locals.var_ndepmpnsub_inv1__blk1129_dn13 = assign44910_e60602_d_n13;
        locals.var_ndepmpnsub_inv1__blk1129_rv = 0.0;

        let (assign44920_e60617, assign44920_e60617_d_n0, assign44920_e60617_d_n2, assign44920_e60617_d_n4, assign44920_e60617_d_n5, assign44920_e60617_d_n6, assign44920_e60617_d_n7, assign44920_e60617_d_n8, assign44920_e60617_d_n9, assign44920_e60617_d_n10, assign44920_e60617_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign44920_e60614: f64 = (locals.var_cox * locals.var_cox);
        let assign44920_e60615: f64 = (locals.var_q_ndepm_esi__blk1114 / assign44920_e60614);
        (assign44920_e60615, (((locals.var_q_ndepm_esi__blk1114_dn0 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn2 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn4 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn5 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn6 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn7 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn8 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn9 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn10 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign44920_e60614 * assign44920_e60614)), (((locals.var_q_ndepm_esi__blk1114_dn13 * assign44920_e60614) - (locals.var_q_ndepm_esi__blk1114 * ((locals.var_cox_dn13 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn13)))) / (assign44920_e60614 * assign44920_e60614)),)
    } else {
        (locals.var_q_ndepm_esi_cox_inv2__blk1134, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn0, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn2, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn4, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn5, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn6, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn7, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn8, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn9, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn10, locals.var_q_ndepm_esi_cox_inv2__blk1134_dn13,)
    }
};
        locals.var_q_ndepm_esi_cox_inv2__blk1134 = assign44920_e60617;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn0 = assign44920_e60617_d_n0;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn2 = assign44920_e60617_d_n2;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn4 = assign44920_e60617_d_n4;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn5 = assign44920_e60617_d_n5;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn6 = assign44920_e60617_d_n6;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn7 = assign44920_e60617_d_n7;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn8 = assign44920_e60617_d_n8;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn9 = assign44920_e60617_d_n9;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn10 = assign44920_e60617_d_n10;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_dn13 = assign44920_e60617_d_n13;
        locals.var_q_ndepm_esi_cox_inv2__blk1134_rv = 0.0;

        let (assign44930_e60630, assign44930_e60630_d_n0, assign44930_e60630_d_n2, assign44930_e60630_d_n4, assign44930_e60630_d_n5, assign44930_e60630_d_n6, assign44930_e60630_d_n7, assign44930_e60630_d_n8, assign44930_e60630_d_n9, assign44930_e60630_d_n10, assign44930_e60630_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign44930_e60628: f64 = (2.0 / locals.var_q_ndepm_esi_cox_inv2__blk1134);
        (assign44930_e60628, (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn0) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn2) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn4) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn5) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn6) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn7) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn8) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn9) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn10) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2__blk1134_dn13) / (locals.var_q_ndepm_esi_cox_inv2__blk1134 * locals.var_q_ndepm_esi_cox_inv2__blk1134))),)
    } else {
        (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn0, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn2, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn4, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn5, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn6, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn7, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn8, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn9, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn10, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn13,)
    }
};
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 = assign44930_e60630;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn0 = assign44930_e60630_d_n0;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn2 = assign44930_e60630_d_n2;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn4 = assign44930_e60630_d_n4;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn5 = assign44930_e60630_d_n5;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn6 = assign44930_e60630_d_n6;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn7 = assign44930_e60630_d_n7;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn8 = assign44930_e60630_d_n8;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn9 = assign44930_e60630_d_n9;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn10 = assign44930_e60630_d_n10;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn13 = assign44930_e60630_d_n13;
        locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_rv = 0.0;

        let (assign44940_e60641,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (2.0,)
    } else {
        (locals.var_vgpdep_pw__blk1143,)
    }
};
        locals.var_vgpdep_pw__blk1143 = assign44940_e60641;
        locals.var_vgpdep_pw__blk1143_rv = 0.0;

        let (assign44950_e60660, assign44950_e60660_d_n0, assign44950_e60660_d_n2, assign44950_e60660_d_n4, assign44950_e60660_d_n5, assign44950_e60660_d_n6, assign44950_e60660_d_n7, assign44950_e60660_d_n8, assign44950_e60660_d_n9, assign44950_e60660_d_n10, assign44950_e60660_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign44950_e60658, assign44950_e60658_d_n0, assign44950_e60658_d_n2, assign44950_e60658_d_n4, assign44950_e60658_d_n5, assign44950_e60658_d_n6, assign44950_e60658_d_n7, assign44950_e60658_d_n8, assign44950_e60658_d_n9, assign44950_e60658_d_n10, assign44950_e60658_d_n13,) = {
            if param_given[227] {
                (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn4, locals.var_uc_clm2_dn5, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn8, locals.var_uc_clm2_dn9, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn13,)
            } else {
                let assign44950_e60656: f64 = (p.p343 * p.p340);
                let assign44950_e60657: f64 = (5000000000.0 / assign44950_e60656);
                (assign44950_e60657, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign44950_e60658, assign44950_e60658_d_n0, assign44950_e60658_d_n2, assign44950_e60658_d_n4, assign44950_e60658_d_n5, assign44950_e60658_d_n6, assign44950_e60658_d_n7, assign44950_e60658_d_n8, assign44950_e60658_d_n9, assign44950_e60658_d_n10, assign44950_e60658_d_n13,)
    } else {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn4, locals.var_uc_clm2_dn5, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn8, locals.var_uc_clm2_dn9, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn13,)
    }
};
        locals.var_uc_clm2 = assign44950_e60660;
        locals.var_uc_clm2_dn0 = assign44950_e60660_d_n0;
        locals.var_uc_clm2_dn2 = assign44950_e60660_d_n2;
        locals.var_uc_clm2_dn4 = assign44950_e60660_d_n4;
        locals.var_uc_clm2_dn5 = assign44950_e60660_d_n5;
        locals.var_uc_clm2_dn6 = assign44950_e60660_d_n6;
        locals.var_uc_clm2_dn7 = assign44950_e60660_d_n7;
        locals.var_uc_clm2_dn8 = assign44950_e60660_d_n8;
        locals.var_uc_clm2_dn9 = assign44950_e60660_d_n9;
        locals.var_uc_clm2_dn10 = assign44950_e60660_d_n10;
        locals.var_uc_clm2_dn13 = assign44950_e60660_d_n13;
        locals.var_uc_clm2_rv = 0.0;

        let assign44960_e60664: f64 = (2.0 + 0.1);
        let assign44960_e60669: f64 = if ((locals.var_uc_clm2 < assign44960_e60664) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign44960_e60669;
        locals.var_guard1172_rv = 0.0;

        let (assign44970_e60686, assign44970_e60686_d_n0, assign44970_e60686_d_n2, assign44970_e60686_d_n4, assign44970_e60686_d_n5, assign44970_e60686_d_n6, assign44970_e60686_d_n7, assign44970_e60686_d_n8, assign44970_e60686_d_n9, assign44970_e60686_d_n10, assign44970_e60686_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign44970_e60682: f64 = (2.0 + 0.1);
        let assign44970_e60684: f64 = (assign44970_e60682 - locals.var_uc_clm2);
        (assign44970_e60684, (-locals.var_uc_clm2_dn0), (-locals.var_uc_clm2_dn2), (-locals.var_uc_clm2_dn4), (-locals.var_uc_clm2_dn5), (-locals.var_uc_clm2_dn6), (-locals.var_uc_clm2_dn7), (-locals.var_uc_clm2_dn8), (-locals.var_uc_clm2_dn9), (-locals.var_uc_clm2_dn10), (-locals.var_uc_clm2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign44970_e60686;
        locals.var_tmf1_dn0 = assign44970_e60686_d_n0;
        locals.var_tmf1_dn2 = assign44970_e60686_d_n2;
        locals.var_tmf1_dn4 = assign44970_e60686_d_n4;
        locals.var_tmf1_dn5 = assign44970_e60686_d_n5;
        locals.var_tmf1_dn6 = assign44970_e60686_d_n6;
        locals.var_tmf1_dn7 = assign44970_e60686_d_n7;
        locals.var_tmf1_dn8 = assign44970_e60686_d_n8;
        locals.var_tmf1_dn9 = assign44970_e60686_d_n9;
        locals.var_tmf1_dn10 = assign44970_e60686_d_n10;
        locals.var_tmf1_dn13 = assign44970_e60686_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign44980_e60701, assign44980_e60701_d_n0, assign44980_e60701_d_n2, assign44980_e60701_d_n4, assign44980_e60701_d_n5, assign44980_e60701_d_n6, assign44980_e60701_d_n7, assign44980_e60701_d_n8, assign44980_e60701_d_n9, assign44980_e60701_d_n10, assign44980_e60701_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign44980_e60699: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign44980_e60699, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign44980_e60701;
        locals.var_x2_dn0 = assign44980_e60701_d_n0;
        locals.var_x2_dn2 = assign44980_e60701_d_n2;
        locals.var_x2_dn4 = assign44980_e60701_d_n4;
        locals.var_x2_dn5 = assign44980_e60701_d_n5;
        locals.var_x2_dn6 = assign44980_e60701_d_n6;
        locals.var_x2_dn7 = assign44980_e60701_d_n7;
        locals.var_x2_dn8 = assign44980_e60701_d_n8;
        locals.var_x2_dn9 = assign44980_e60701_d_n9;
        locals.var_x2_dn10 = assign44980_e60701_d_n10;
        locals.var_x2_dn13 = assign44980_e60701_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign44990_e60716, assign44990_e60716_d_n0, assign44990_e60716_d_n2, assign44990_e60716_d_n4, assign44990_e60716_d_n5, assign44990_e60716_d_n6, assign44990_e60716_d_n7, assign44990_e60716_d_n8, assign44990_e60716_d_n9, assign44990_e60716_d_n10, assign44990_e60716_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign44990_e60714: f64 = (0.1 * 0.1);
        (assign44990_e60714, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign44990_e60716;
        locals.var_xmax2_dn0 = assign44990_e60716_d_n0;
        locals.var_xmax2_dn2 = assign44990_e60716_d_n2;
        locals.var_xmax2_dn4 = assign44990_e60716_d_n4;
        locals.var_xmax2_dn5 = assign44990_e60716_d_n5;
        locals.var_xmax2_dn6 = assign44990_e60716_d_n6;
        locals.var_xmax2_dn7 = assign44990_e60716_d_n7;
        locals.var_xmax2_dn8 = assign44990_e60716_d_n8;
        locals.var_xmax2_dn9 = assign44990_e60716_d_n9;
        locals.var_xmax2_dn10 = assign44990_e60716_d_n10;
        locals.var_xmax2_dn13 = assign44990_e60716_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign45000_e60729, assign45000_e60729_d_n0, assign45000_e60729_d_n2, assign45000_e60729_d_n4, assign45000_e60729_d_n5, assign45000_e60729_d_n6, assign45000_e60729_d_n7, assign45000_e60729_d_n8, assign45000_e60729_d_n9, assign45000_e60729_d_n10, assign45000_e60729_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign45000_e60729;
        locals.var_xp_dn0 = assign45000_e60729_d_n0;
        locals.var_xp_dn2 = assign45000_e60729_d_n2;
        locals.var_xp_dn4 = assign45000_e60729_d_n4;
        locals.var_xp_dn5 = assign45000_e60729_d_n5;
        locals.var_xp_dn6 = assign45000_e60729_d_n6;
        locals.var_xp_dn7 = assign45000_e60729_d_n7;
        locals.var_xp_dn8 = assign45000_e60729_d_n8;
        locals.var_xp_dn9 = assign45000_e60729_d_n9;
        locals.var_xp_dn10 = assign45000_e60729_d_n10;
        locals.var_xp_dn13 = assign45000_e60729_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign45010_e60742, assign45010_e60742_d_n0, assign45010_e60742_d_n2, assign45010_e60742_d_n4, assign45010_e60742_d_n5, assign45010_e60742_d_n6, assign45010_e60742_d_n7, assign45010_e60742_d_n8, assign45010_e60742_d_n9, assign45010_e60742_d_n10, assign45010_e60742_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign45010_e60742;
        locals.var_xmp_dn0 = assign45010_e60742_d_n0;
        locals.var_xmp_dn2 = assign45010_e60742_d_n2;
        locals.var_xmp_dn4 = assign45010_e60742_d_n4;
        locals.var_xmp_dn5 = assign45010_e60742_d_n5;
        locals.var_xmp_dn6 = assign45010_e60742_d_n6;
        locals.var_xmp_dn7 = assign45010_e60742_d_n7;
        locals.var_xmp_dn8 = assign45010_e60742_d_n8;
        locals.var_xmp_dn9 = assign45010_e60742_d_n9;
        locals.var_xmp_dn10 = assign45010_e60742_d_n10;
        locals.var_xmp_dn13 = assign45010_e60742_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign45020_e60755,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign45020_e60755;
        locals.var_m0_rv = 0.0;

        let (assign45030_e60768,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign45030_e60768;
        locals.var_mm_rv = 0.0;

        let (assign45040_e60781, assign45040_e60781_d_n0, assign45040_e60781_d_n2, assign45040_e60781_d_n4, assign45040_e60781_d_n5, assign45040_e60781_d_n6, assign45040_e60781_d_n7, assign45040_e60781_d_n8, assign45040_e60781_d_n9, assign45040_e60781_d_n10, assign45040_e60781_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign45040_e60781;
        locals.var_arg_dn0 = assign45040_e60781_d_n0;
        locals.var_arg_dn2 = assign45040_e60781_d_n2;
        locals.var_arg_dn4 = assign45040_e60781_d_n4;
        locals.var_arg_dn5 = assign45040_e60781_d_n5;
        locals.var_arg_dn6 = assign45040_e60781_d_n6;
        locals.var_arg_dn7 = assign45040_e60781_d_n7;
        locals.var_arg_dn8 = assign45040_e60781_d_n8;
        locals.var_arg_dn9 = assign45040_e60781_d_n9;
        locals.var_arg_dn10 = assign45040_e60781_d_n10;
        locals.var_arg_dn13 = assign45040_e60781_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign45050_e60794, assign45050_e60794_d_n0, assign45050_e60794_d_n2, assign45050_e60794_d_n4, assign45050_e60794_d_n5, assign45050_e60794_d_n6, assign45050_e60794_d_n7, assign45050_e60794_d_n8, assign45050_e60794_d_n9, assign45050_e60794_d_n10, assign45050_e60794_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign45050_e60794;
        locals.var_dnm_dn0 = assign45050_e60794_d_n0;
        locals.var_dnm_dn2 = assign45050_e60794_d_n2;
        locals.var_dnm_dn4 = assign45050_e60794_d_n4;
        locals.var_dnm_dn5 = assign45050_e60794_d_n5;
        locals.var_dnm_dn6 = assign45050_e60794_d_n6;
        locals.var_dnm_dn7 = assign45050_e60794_d_n7;
        locals.var_dnm_dn8 = assign45050_e60794_d_n8;
        locals.var_dnm_dn9 = assign45050_e60794_d_n9;
        locals.var_dnm_dn10 = assign45050_e60794_d_n10;
        locals.var_dnm_dn13 = assign45050_e60794_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign45060_e60809, assign45060_e60809_d_n0, assign45060_e60809_d_n2, assign45060_e60809_d_n4, assign45060_e60809_d_n5, assign45060_e60809_d_n6, assign45060_e60809_d_n7, assign45060_e60809_d_n8, assign45060_e60809_d_n9, assign45060_e60809_d_n10, assign45060_e60809_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45060_e60807: f64 = (locals.var_xp * locals.var_x2);
        (assign45060_e60807, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign45060_e60809;
        locals.var_xp_dn0 = assign45060_e60809_d_n0;
        locals.var_xp_dn2 = assign45060_e60809_d_n2;
        locals.var_xp_dn4 = assign45060_e60809_d_n4;
        locals.var_xp_dn5 = assign45060_e60809_d_n5;
        locals.var_xp_dn6 = assign45060_e60809_d_n6;
        locals.var_xp_dn7 = assign45060_e60809_d_n7;
        locals.var_xp_dn8 = assign45060_e60809_d_n8;
        locals.var_xp_dn9 = assign45060_e60809_d_n9;
        locals.var_xp_dn10 = assign45060_e60809_d_n10;
        locals.var_xp_dn13 = assign45060_e60809_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign45070_e60824, assign45070_e60824_d_n0, assign45070_e60824_d_n2, assign45070_e60824_d_n4, assign45070_e60824_d_n5, assign45070_e60824_d_n6, assign45070_e60824_d_n7, assign45070_e60824_d_n8, assign45070_e60824_d_n9, assign45070_e60824_d_n10, assign45070_e60824_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45070_e60822: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign45070_e60822, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign45070_e60824;
        locals.var_xmp_dn0 = assign45070_e60824_d_n0;
        locals.var_xmp_dn2 = assign45070_e60824_d_n2;
        locals.var_xmp_dn4 = assign45070_e60824_d_n4;
        locals.var_xmp_dn5 = assign45070_e60824_d_n5;
        locals.var_xmp_dn6 = assign45070_e60824_d_n6;
        locals.var_xmp_dn7 = assign45070_e60824_d_n7;
        locals.var_xmp_dn8 = assign45070_e60824_d_n8;
        locals.var_xmp_dn9 = assign45070_e60824_d_n9;
        locals.var_xmp_dn10 = assign45070_e60824_d_n10;
        locals.var_xmp_dn13 = assign45070_e60824_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign45080_e60839, assign45080_e60839_d_n0, assign45080_e60839_d_n2, assign45080_e60839_d_n4, assign45080_e60839_d_n5, assign45080_e60839_d_n6, assign45080_e60839_d_n7, assign45080_e60839_d_n8, assign45080_e60839_d_n9, assign45080_e60839_d_n10, assign45080_e60839_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45080_e60837: f64 = (locals.var_xp * locals.var_x2);
        (assign45080_e60837, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign45080_e60839;
        locals.var_xp_dn0 = assign45080_e60839_d_n0;
        locals.var_xp_dn2 = assign45080_e60839_d_n2;
        locals.var_xp_dn4 = assign45080_e60839_d_n4;
        locals.var_xp_dn5 = assign45080_e60839_d_n5;
        locals.var_xp_dn6 = assign45080_e60839_d_n6;
        locals.var_xp_dn7 = assign45080_e60839_d_n7;
        locals.var_xp_dn8 = assign45080_e60839_d_n8;
        locals.var_xp_dn9 = assign45080_e60839_d_n9;
        locals.var_xp_dn10 = assign45080_e60839_d_n10;
        locals.var_xp_dn13 = assign45080_e60839_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign45090_e60854, assign45090_e60854_d_n0, assign45090_e60854_d_n2, assign45090_e60854_d_n4, assign45090_e60854_d_n5, assign45090_e60854_d_n6, assign45090_e60854_d_n7, assign45090_e60854_d_n8, assign45090_e60854_d_n9, assign45090_e60854_d_n10, assign45090_e60854_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45090_e60852: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign45090_e60852, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign45090_e60854;
        locals.var_xmp_dn0 = assign45090_e60854_d_n0;
        locals.var_xmp_dn2 = assign45090_e60854_d_n2;
        locals.var_xmp_dn4 = assign45090_e60854_d_n4;
        locals.var_xmp_dn5 = assign45090_e60854_d_n5;
        locals.var_xmp_dn6 = assign45090_e60854_d_n6;
        locals.var_xmp_dn7 = assign45090_e60854_d_n7;
        locals.var_xmp_dn8 = assign45090_e60854_d_n8;
        locals.var_xmp_dn9 = assign45090_e60854_d_n9;
        locals.var_xmp_dn10 = assign45090_e60854_d_n10;
        locals.var_xmp_dn13 = assign45090_e60854_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign45100_e60869, assign45100_e60869_d_n0, assign45100_e60869_d_n2, assign45100_e60869_d_n4, assign45100_e60869_d_n5, assign45100_e60869_d_n6, assign45100_e60869_d_n7, assign45100_e60869_d_n8, assign45100_e60869_d_n9, assign45100_e60869_d_n10, assign45100_e60869_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45100_e60867: f64 = (locals.var_xp + locals.var_xmp);
        (assign45100_e60867, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign45100_e60869;
        locals.var_arg_dn0 = assign45100_e60869_d_n0;
        locals.var_arg_dn2 = assign45100_e60869_d_n2;
        locals.var_arg_dn4 = assign45100_e60869_d_n4;
        locals.var_arg_dn5 = assign45100_e60869_d_n5;
        locals.var_arg_dn6 = assign45100_e60869_d_n6;
        locals.var_arg_dn7 = assign45100_e60869_d_n7;
        locals.var_arg_dn8 = assign45100_e60869_d_n8;
        locals.var_arg_dn9 = assign45100_e60869_d_n9;
        locals.var_arg_dn10 = assign45100_e60869_d_n10;
        locals.var_arg_dn13 = assign45100_e60869_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign45110_e60882, assign45110_e60882_d_n0, assign45110_e60882_d_n2, assign45110_e60882_d_n4, assign45110_e60882_d_n5, assign45110_e60882_d_n6, assign45110_e60882_d_n7, assign45110_e60882_d_n8, assign45110_e60882_d_n9, assign45110_e60882_d_n10, assign45110_e60882_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign45110_e60882;
        locals.var_dnm_dn0 = assign45110_e60882_d_n0;
        locals.var_dnm_dn2 = assign45110_e60882_d_n2;
        locals.var_dnm_dn4 = assign45110_e60882_d_n4;
        locals.var_dnm_dn5 = assign45110_e60882_d_n5;
        locals.var_dnm_dn6 = assign45110_e60882_d_n6;
        locals.var_dnm_dn7 = assign45110_e60882_d_n7;
        locals.var_dnm_dn8 = assign45110_e60882_d_n8;
        locals.var_dnm_dn9 = assign45110_e60882_d_n9;
        locals.var_dnm_dn10 = assign45110_e60882_d_n10;
        locals.var_dnm_dn13 = assign45110_e60882_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign45120_e60897: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign45120_e60897;
        locals.var_guard1173_rv = 0.0;

        let assign45130_e60900: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign45130_e60900;
        locals.var_guard1174_rv = 0.0;

        let (assign45140_e60917,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) && (locals.var_guard1174 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign45140_e60917;
        locals.var_mm_rv = 0.0;

        let assign45150_e60920: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign45150_e60920;
        locals.var_guard1175_rv = 0.0;

        let (assign45160_e60940,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) && (locals.var_guard1174 == 0.0)) && (locals.var_guard1175 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign45160_e60940;
        locals.var_mm_rv = 0.0;

        let assign45170_e60943: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign45170_e60943;
        locals.var_guard1176_rv = 0.0;

        let (assign45180_e60966,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) && (locals.var_guard1174 == 0.0)) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign45180_e60966;
        locals.var_mm_rv = 0.0;

        let assign45190_e60969: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign45190_e60969;
        locals.var_guard1177_rv = 0.0;

        let (assign45200_e60995,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) && (locals.var_guard1174 == 0.0)) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 == 0.0)) && (locals.var_guard1177 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign45200_e60995;
        locals.var_mm_rv = 0.0;

        let (assign45210_e61010,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign45210_e61010;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_152(
        locals: &mut StampLocals,
    ) {
        let mut assign45220_loop_guard: usize = 0;
        while {
            let assign45220_cond_e61026: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign45220_cond_e61026 != 0.0
        } {
            assign45220_loop_guard += 1;
            assert!(assign45220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign45220_body0_e61042, assign45220_body0_e61042_d_n0, assign45220_body0_e61042_d_n2, assign45220_body0_e61042_d_n4, assign45220_body0_e61042_d_n5, assign45220_body0_e61042_d_n6, assign45220_body0_e61042_d_n7, assign45220_body0_e61042_d_n8, assign45220_body0_e61042_d_n9, assign45220_body0_e61042_d_n10, assign45220_body0_e61042_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign45220_body0_e61040: f64 = (locals.var_dnm).sqrt();
        (assign45220_body0_e61040, (locals.var_dnm_dn0 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn2 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn4 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn5 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn6 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn7 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn8 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn9 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn10 / (2.0 * assign45220_body0_e61040)), (locals.var_dnm_dn13 / (2.0 * assign45220_body0_e61040)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign45220_body0_e61042;
            locals.var_dnm_dn0 = assign45220_body0_e61042_d_n0;
            locals.var_dnm_dn2 = assign45220_body0_e61042_d_n2;
            locals.var_dnm_dn4 = assign45220_body0_e61042_d_n4;
            locals.var_dnm_dn5 = assign45220_body0_e61042_d_n5;
            locals.var_dnm_dn6 = assign45220_body0_e61042_d_n6;
            locals.var_dnm_dn7 = assign45220_body0_e61042_d_n7;
            locals.var_dnm_dn8 = assign45220_body0_e61042_d_n8;
            locals.var_dnm_dn9 = assign45220_body0_e61042_d_n9;
            locals.var_dnm_dn10 = assign45220_body0_e61042_d_n10;
            locals.var_dnm_dn13 = assign45220_body0_e61042_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign45220_body1_e61059,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign45220_body1_e61057: f64 = (locals.var_m0 + 1.0);
        (assign45220_body1_e61057,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign45220_body1_e61059;
            locals.var_m0_rv = 0.0;
        }

        let (assign45230_e61086, assign45230_e61086_d_n0, assign45230_e61086_d_n2, assign45230_e61086_d_n4, assign45230_e61086_d_n5, assign45230_e61086_d_n6, assign45230_e61086_d_n7, assign45230_e61086_d_n8, assign45230_e61086_d_n9, assign45230_e61086_d_n10, assign45230_e61086_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) && (locals.var_guard1173 == 0.0)) {
        let (assign45230_e61084, assign45230_e61084_d_n0, assign45230_e61084_d_n2, assign45230_e61084_d_n4, assign45230_e61084_d_n5, assign45230_e61084_d_n6, assign45230_e61084_d_n7, assign45230_e61084_d_n8, assign45230_e61084_d_n9, assign45230_e61084_d_n10, assign45230_e61084_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign45230_e61081: f64 = (2.0 * 2.0);
                let assign45230_e61082: f64 = (1.0 / assign45230_e61081);
                let assign45230_e61083: f64 = (locals.var_dnm).powf(assign45230_e61082);
                (assign45230_e61083, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn0)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn2)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn4)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn5)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn6)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn7)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn8)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn9)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn10)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign45230_e61082) as f64).is_finite() && ((assign45230_e61082) as f64).fract() == 0.0 { if assign45230_e61082 == 0.0 { 0.0 } else { (assign45230_e61082 * ((locals.var_dnm).powf(assign45230_e61082 - 1.0) * locals.var_dnm_dn13)) } } else { (assign45230_e61083 * (assign45230_e61082 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign45230_e61084, assign45230_e61084_d_n0, assign45230_e61084_d_n2, assign45230_e61084_d_n4, assign45230_e61084_d_n5, assign45230_e61084_d_n6, assign45230_e61084_d_n7, assign45230_e61084_d_n8, assign45230_e61084_d_n9, assign45230_e61084_d_n10, assign45230_e61084_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign45230_e61086;
        locals.var_dnm_dn0 = assign45230_e61086_d_n0;
        locals.var_dnm_dn2 = assign45230_e61086_d_n2;
        locals.var_dnm_dn4 = assign45230_e61086_d_n4;
        locals.var_dnm_dn5 = assign45230_e61086_d_n5;
        locals.var_dnm_dn6 = assign45230_e61086_d_n6;
        locals.var_dnm_dn7 = assign45230_e61086_d_n7;
        locals.var_dnm_dn8 = assign45230_e61086_d_n8;
        locals.var_dnm_dn9 = assign45230_e61086_d_n9;
        locals.var_dnm_dn10 = assign45230_e61086_d_n10;
        locals.var_dnm_dn13 = assign45230_e61086_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign45240_e61101, assign45240_e61101_d_n0, assign45240_e61101_d_n2, assign45240_e61101_d_n4, assign45240_e61101_d_n5, assign45240_e61101_d_n6, assign45240_e61101_d_n7, assign45240_e61101_d_n8, assign45240_e61101_d_n9, assign45240_e61101_d_n10, assign45240_e61101_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45240_e61099: f64 = (1.0 / locals.var_dnm);
        (assign45240_e61099, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign45240_e61101;
        locals.var_dnm_dn0 = assign45240_e61101_d_n0;
        locals.var_dnm_dn2 = assign45240_e61101_d_n2;
        locals.var_dnm_dn4 = assign45240_e61101_d_n4;
        locals.var_dnm_dn5 = assign45240_e61101_d_n5;
        locals.var_dnm_dn6 = assign45240_e61101_d_n6;
        locals.var_dnm_dn7 = assign45240_e61101_d_n7;
        locals.var_dnm_dn8 = assign45240_e61101_d_n8;
        locals.var_dnm_dn9 = assign45240_e61101_d_n9;
        locals.var_dnm_dn10 = assign45240_e61101_d_n10;
        locals.var_dnm_dn13 = assign45240_e61101_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign45250_e61118, assign45250_e61118_d_n0, assign45250_e61118_d_n2, assign45250_e61118_d_n4, assign45250_e61118_d_n5, assign45250_e61118_d_n6, assign45250_e61118_d_n7, assign45250_e61118_d_n8, assign45250_e61118_d_n9, assign45250_e61118_d_n10, assign45250_e61118_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45250_e61114: f64 = (locals.var_tmf1 * 0.1);
        let assign45250_e61116: f64 = (assign45250_e61114 * locals.var_dnm);
        (assign45250_e61116, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign45250_e61114 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign45250_e61118;
        locals.var_tmf0_dn0 = assign45250_e61118_d_n0;
        locals.var_tmf0_dn2 = assign45250_e61118_d_n2;
        locals.var_tmf0_dn4 = assign45250_e61118_d_n4;
        locals.var_tmf0_dn5 = assign45250_e61118_d_n5;
        locals.var_tmf0_dn6 = assign45250_e61118_d_n6;
        locals.var_tmf0_dn7 = assign45250_e61118_d_n7;
        locals.var_tmf0_dn8 = assign45250_e61118_d_n8;
        locals.var_tmf0_dn9 = assign45250_e61118_d_n9;
        locals.var_tmf0_dn10 = assign45250_e61118_d_n10;
        locals.var_tmf0_dn13 = assign45250_e61118_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign45260_e61137, assign45260_e61137_d_n0, assign45260_e61137_d_n2, assign45260_e61137_d_n4, assign45260_e61137_d_n5, assign45260_e61137_d_n6, assign45260_e61137_d_n7, assign45260_e61137_d_n8, assign45260_e61137_d_n9, assign45260_e61137_d_n10, assign45260_e61137_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45260_e61131: f64 = (0.1 * locals.var_xmp);
        let assign45260_e61133: f64 = (assign45260_e61131 * locals.var_dnm);
        let assign45260_e61135: f64 = (assign45260_e61133 / locals.var_arg);
        (assign45260_e61135, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn0)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn2)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn4)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn5)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn6)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn7)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn8)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn9)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn10)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign45260_e61131 * locals.var_dnm_dn13)) * locals.var_arg) - (assign45260_e61133 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign45260_e61137;
        locals.var_t0_dn0 = assign45260_e61137_d_n0;
        locals.var_t0_dn2 = assign45260_e61137_d_n2;
        locals.var_t0_dn4 = assign45260_e61137_d_n4;
        locals.var_t0_dn5 = assign45260_e61137_d_n5;
        locals.var_t0_dn6 = assign45260_e61137_d_n6;
        locals.var_t0_dn7 = assign45260_e61137_d_n7;
        locals.var_t0_dn8 = assign45260_e61137_d_n8;
        locals.var_t0_dn9 = assign45260_e61137_d_n9;
        locals.var_t0_dn10 = assign45260_e61137_d_n10;
        locals.var_t0_dn13 = assign45260_e61137_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign45270_e61154, assign45270_e61154_d_n0, assign45270_e61154_d_n2, assign45270_e61154_d_n4, assign45270_e61154_d_n5, assign45270_e61154_d_n6, assign45270_e61154_d_n7, assign45270_e61154_d_n8, assign45270_e61154_d_n9, assign45270_e61154_d_n10, assign45270_e61154_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        let assign45270_e61150: f64 = (2.0 + 0.1);
        let assign45270_e61152: f64 = (assign45270_e61150 - locals.var_tmf0);
        (assign45270_e61152, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn4, locals.var_uc_clm2_dn5, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn8, locals.var_uc_clm2_dn9, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn13,)
    }
};
        locals.var_uc_clm2 = assign45270_e61154;
        locals.var_uc_clm2_dn0 = assign45270_e61154_d_n0;
        locals.var_uc_clm2_dn2 = assign45270_e61154_d_n2;
        locals.var_uc_clm2_dn4 = assign45270_e61154_d_n4;
        locals.var_uc_clm2_dn5 = assign45270_e61154_d_n5;
        locals.var_uc_clm2_dn6 = assign45270_e61154_d_n6;
        locals.var_uc_clm2_dn7 = assign45270_e61154_d_n7;
        locals.var_uc_clm2_dn8 = assign45270_e61154_d_n8;
        locals.var_uc_clm2_dn9 = assign45270_e61154_d_n9;
        locals.var_uc_clm2_dn10 = assign45270_e61154_d_n10;
        locals.var_uc_clm2_dn13 = assign45270_e61154_d_n13;
        locals.var_uc_clm2_rv = 0.0;

        let (assign45280_e61167, assign45280_e61167_d_n0, assign45280_e61167_d_n2, assign45280_e61167_d_n4, assign45280_e61167_d_n5, assign45280_e61167_d_n6, assign45280_e61167_d_n7, assign45280_e61167_d_n8, assign45280_e61167_d_n9, assign45280_e61167_d_n10, assign45280_e61167_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign45280_e61167;
        locals.var_t0_dn0 = assign45280_e61167_d_n0;
        locals.var_t0_dn2 = assign45280_e61167_d_n2;
        locals.var_t0_dn4 = assign45280_e61167_d_n4;
        locals.var_t0_dn5 = assign45280_e61167_d_n5;
        locals.var_t0_dn6 = assign45280_e61167_d_n6;
        locals.var_t0_dn7 = assign45280_e61167_d_n7;
        locals.var_t0_dn8 = assign45280_e61167_d_n8;
        locals.var_t0_dn9 = assign45280_e61167_d_n9;
        locals.var_t0_dn10 = assign45280_e61167_d_n10;
        locals.var_t0_dn13 = assign45280_e61167_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign45290_e61181, assign45290_e61181_d_n0, assign45290_e61181_d_n2, assign45290_e61181_d_n4, assign45290_e61181_d_n5, assign45290_e61181_d_n6, assign45290_e61181_d_n7, assign45290_e61181_d_n8, assign45290_e61181_d_n9, assign45290_e61181_d_n10, assign45290_e61181_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 == 0.0)) {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn4, locals.var_uc_clm2_dn5, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn8, locals.var_uc_clm2_dn9, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn13,)
    } else {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn4, locals.var_uc_clm2_dn5, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn8, locals.var_uc_clm2_dn9, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn13,)
    }
};
        locals.var_uc_clm2 = assign45290_e61181;
        locals.var_uc_clm2_dn0 = assign45290_e61181_d_n0;
        locals.var_uc_clm2_dn2 = assign45290_e61181_d_n2;
        locals.var_uc_clm2_dn4 = assign45290_e61181_d_n4;
        locals.var_uc_clm2_dn5 = assign45290_e61181_d_n5;
        locals.var_uc_clm2_dn6 = assign45290_e61181_d_n6;
        locals.var_uc_clm2_dn7 = assign45290_e61181_d_n7;
        locals.var_uc_clm2_dn8 = assign45290_e61181_d_n8;
        locals.var_uc_clm2_dn9 = assign45290_e61181_d_n9;
        locals.var_uc_clm2_dn10 = assign45290_e61181_d_n10;
        locals.var_uc_clm2_dn13 = assign45290_e61181_d_n13;
        locals.var_uc_clm2_rv = 0.0;

        let (assign45300_e61195, assign45300_e61195_d_n0, assign45300_e61195_d_n2, assign45300_e61195_d_n4, assign45300_e61195_d_n5, assign45300_e61195_d_n6, assign45300_e61195_d_n7, assign45300_e61195_d_n8, assign45300_e61195_d_n9, assign45300_e61195_d_n10, assign45300_e61195_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1172 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign45300_e61195;
        locals.var_t0_dn0 = assign45300_e61195_d_n0;
        locals.var_t0_dn2 = assign45300_e61195_d_n2;
        locals.var_t0_dn4 = assign45300_e61195_d_n4;
        locals.var_t0_dn5 = assign45300_e61195_d_n5;
        locals.var_t0_dn6 = assign45300_e61195_d_n6;
        locals.var_t0_dn7 = assign45300_e61195_d_n7;
        locals.var_t0_dn8 = assign45300_e61195_d_n8;
        locals.var_t0_dn9 = assign45300_e61195_d_n9;
        locals.var_t0_dn10 = assign45300_e61195_d_n10;
        locals.var_t0_dn13 = assign45300_e61195_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign45310_e61206, assign45310_e61206_d_n0, assign45310_e61206_d_n2, assign45310_e61206_d_n4, assign45310_e61206_d_n5, assign45310_e61206_d_n6, assign45310_e61206_d_n7, assign45310_e61206_d_n8, assign45310_e61206_d_n9, assign45310_e61206_d_n10, assign45310_e61206_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_dep__blk1089, locals.var_phi_s0_dep__blk1089_dn0, locals.var_phi_s0_dep__blk1089_dn2, locals.var_phi_s0_dep__blk1089_dn4, locals.var_phi_s0_dep__blk1089_dn5, locals.var_phi_s0_dep__blk1089_dn6, locals.var_phi_s0_dep__blk1089_dn7, locals.var_phi_s0_dep__blk1089_dn8, locals.var_phi_s0_dep__blk1089_dn9, locals.var_phi_s0_dep__blk1089_dn10, locals.var_phi_s0_dep__blk1089_dn13,)
    }
};
        locals.var_phi_s0_dep__blk1089 = assign45310_e61206;
        locals.var_phi_s0_dep__blk1089_dn0 = assign45310_e61206_d_n0;
        locals.var_phi_s0_dep__blk1089_dn2 = assign45310_e61206_d_n2;
        locals.var_phi_s0_dep__blk1089_dn4 = assign45310_e61206_d_n4;
        locals.var_phi_s0_dep__blk1089_dn5 = assign45310_e61206_d_n5;
        locals.var_phi_s0_dep__blk1089_dn6 = assign45310_e61206_d_n6;
        locals.var_phi_s0_dep__blk1089_dn7 = assign45310_e61206_d_n7;
        locals.var_phi_s0_dep__blk1089_dn8 = assign45310_e61206_d_n8;
        locals.var_phi_s0_dep__blk1089_dn9 = assign45310_e61206_d_n9;
        locals.var_phi_s0_dep__blk1089_dn10 = assign45310_e61206_d_n10;
        locals.var_phi_s0_dep__blk1089_dn13 = assign45310_e61206_d_n13;
        locals.var_phi_s0_dep__blk1089_rv = 0.0;

        let (assign45320_e61217, assign45320_e61217_d_n0, assign45320_e61217_d_n2, assign45320_e61217_d_n4, assign45320_e61217_d_n5, assign45320_e61217_d_n6, assign45320_e61217_d_n7, assign45320_e61217_d_n8, assign45320_e61217_d_n9, assign45320_e61217_d_n10, assign45320_e61217_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_dep__blk1090, locals.var_phi_sl_dep__blk1090_dn0, locals.var_phi_sl_dep__blk1090_dn2, locals.var_phi_sl_dep__blk1090_dn4, locals.var_phi_sl_dep__blk1090_dn5, locals.var_phi_sl_dep__blk1090_dn6, locals.var_phi_sl_dep__blk1090_dn7, locals.var_phi_sl_dep__blk1090_dn8, locals.var_phi_sl_dep__blk1090_dn9, locals.var_phi_sl_dep__blk1090_dn10, locals.var_phi_sl_dep__blk1090_dn13,)
    }
};
        locals.var_phi_sl_dep__blk1090 = assign45320_e61217;
        locals.var_phi_sl_dep__blk1090_dn0 = assign45320_e61217_d_n0;
        locals.var_phi_sl_dep__blk1090_dn2 = assign45320_e61217_d_n2;
        locals.var_phi_sl_dep__blk1090_dn4 = assign45320_e61217_d_n4;
        locals.var_phi_sl_dep__blk1090_dn5 = assign45320_e61217_d_n5;
        locals.var_phi_sl_dep__blk1090_dn6 = assign45320_e61217_d_n6;
        locals.var_phi_sl_dep__blk1090_dn7 = assign45320_e61217_d_n7;
        locals.var_phi_sl_dep__blk1090_dn8 = assign45320_e61217_d_n8;
        locals.var_phi_sl_dep__blk1090_dn9 = assign45320_e61217_d_n9;
        locals.var_phi_sl_dep__blk1090_dn10 = assign45320_e61217_d_n10;
        locals.var_phi_sl_dep__blk1090_dn13 = assign45320_e61217_d_n13;
        locals.var_phi_sl_dep__blk1090_rv = 0.0;

        let (assign45330_e61228, assign45330_e61228_d_n0, assign45330_e61228_d_n2, assign45330_e61228_d_n4, assign45330_e61228_d_n5, assign45330_e61228_d_n6, assign45330_e61228_d_n7, assign45330_e61228_d_n8, assign45330_e61228_d_n9, assign45330_e61228_d_n10, assign45330_e61228_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0__blk1098, locals.var_q_s0__blk1098_dn0, locals.var_q_s0__blk1098_dn2, locals.var_q_s0__blk1098_dn4, locals.var_q_s0__blk1098_dn5, locals.var_q_s0__blk1098_dn6, locals.var_q_s0__blk1098_dn7, locals.var_q_s0__blk1098_dn8, locals.var_q_s0__blk1098_dn9, locals.var_q_s0__blk1098_dn10, locals.var_q_s0__blk1098_dn13,)
    }
};
        locals.var_q_s0__blk1098 = assign45330_e61228;
        locals.var_q_s0__blk1098_dn0 = assign45330_e61228_d_n0;
        locals.var_q_s0__blk1098_dn2 = assign45330_e61228_d_n2;
        locals.var_q_s0__blk1098_dn4 = assign45330_e61228_d_n4;
        locals.var_q_s0__blk1098_dn5 = assign45330_e61228_d_n5;
        locals.var_q_s0__blk1098_dn6 = assign45330_e61228_d_n6;
        locals.var_q_s0__blk1098_dn7 = assign45330_e61228_d_n7;
        locals.var_q_s0__blk1098_dn8 = assign45330_e61228_d_n8;
        locals.var_q_s0__blk1098_dn9 = assign45330_e61228_d_n9;
        locals.var_q_s0__blk1098_dn10 = assign45330_e61228_d_n10;
        locals.var_q_s0__blk1098_dn13 = assign45330_e61228_d_n13;
        locals.var_q_s0__blk1098_rv = 0.0;

        let (assign45340_e61239, assign45340_e61239_d_n0, assign45340_e61239_d_n2, assign45340_e61239_d_n4, assign45340_e61239_d_n5, assign45340_e61239_d_n6, assign45340_e61239_d_n7, assign45340_e61239_d_n8, assign45340_e61239_d_n9, assign45340_e61239_d_n10, assign45340_e61239_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl__blk1099, locals.var_q_sl__blk1099_dn0, locals.var_q_sl__blk1099_dn2, locals.var_q_sl__blk1099_dn4, locals.var_q_sl__blk1099_dn5, locals.var_q_sl__blk1099_dn6, locals.var_q_sl__blk1099_dn7, locals.var_q_sl__blk1099_dn8, locals.var_q_sl__blk1099_dn9, locals.var_q_sl__blk1099_dn10, locals.var_q_sl__blk1099_dn13,)
    }
};
        locals.var_q_sl__blk1099 = assign45340_e61239;
        locals.var_q_sl__blk1099_dn0 = assign45340_e61239_d_n0;
        locals.var_q_sl__blk1099_dn2 = assign45340_e61239_d_n2;
        locals.var_q_sl__blk1099_dn4 = assign45340_e61239_d_n4;
        locals.var_q_sl__blk1099_dn5 = assign45340_e61239_d_n5;
        locals.var_q_sl__blk1099_dn6 = assign45340_e61239_d_n6;
        locals.var_q_sl__blk1099_dn7 = assign45340_e61239_d_n7;
        locals.var_q_sl__blk1099_dn8 = assign45340_e61239_d_n8;
        locals.var_q_sl__blk1099_dn9 = assign45340_e61239_d_n9;
        locals.var_q_sl__blk1099_dn10 = assign45340_e61239_d_n10;
        locals.var_q_sl__blk1099_dn13 = assign45340_e61239_d_n13;
        locals.var_q_sl__blk1099_rv = 0.0;

        let (assign45350_e61250, assign45350_e61250_d_n0, assign45350_e61250_d_n2, assign45350_e61250_d_n4, assign45350_e61250_d_n5, assign45350_e61250_d_n6, assign45350_e61250_d_n7, assign45350_e61250_d_n8, assign45350_e61250_d_n9, assign45350_e61250_d_n10, assign45350_e61250_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0dep0, locals.var_ps0dep0_dn0, locals.var_ps0dep0_dn2, locals.var_ps0dep0_dn4, locals.var_ps0dep0_dn5, locals.var_ps0dep0_dn6, locals.var_ps0dep0_dn7, locals.var_ps0dep0_dn8, locals.var_ps0dep0_dn9, locals.var_ps0dep0_dn10, locals.var_ps0dep0_dn13,)
    }
};
        locals.var_ps0dep0 = assign45350_e61250;
        locals.var_ps0dep0_dn0 = assign45350_e61250_d_n0;
        locals.var_ps0dep0_dn2 = assign45350_e61250_d_n2;
        locals.var_ps0dep0_dn4 = assign45350_e61250_d_n4;
        locals.var_ps0dep0_dn5 = assign45350_e61250_d_n5;
        locals.var_ps0dep0_dn6 = assign45350_e61250_d_n6;
        locals.var_ps0dep0_dn7 = assign45350_e61250_d_n7;
        locals.var_ps0dep0_dn8 = assign45350_e61250_d_n8;
        locals.var_ps0dep0_dn9 = assign45350_e61250_d_n9;
        locals.var_ps0dep0_dn10 = assign45350_e61250_d_n10;
        locals.var_ps0dep0_dn13 = assign45350_e61250_d_n13;
        locals.var_ps0dep0_rv = 0.0;

        let (assign45360_e61261, assign45360_e61261_d_n0, assign45360_e61261_d_n2, assign45360_e61261_d_n4, assign45360_e61261_d_n5, assign45360_e61261_d_n6, assign45360_e61261_d_n7, assign45360_e61261_d_n8, assign45360_e61261_d_n9, assign45360_e61261_d_n10, assign45360_e61261_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn13,)
    }
};
        locals.var_vgp_res_raw = assign45360_e61261;
        locals.var_vgp_res_raw_dn0 = assign45360_e61261_d_n0;
        locals.var_vgp_res_raw_dn2 = assign45360_e61261_d_n2;
        locals.var_vgp_res_raw_dn4 = assign45360_e61261_d_n4;
        locals.var_vgp_res_raw_dn5 = assign45360_e61261_d_n5;
        locals.var_vgp_res_raw_dn6 = assign45360_e61261_d_n6;
        locals.var_vgp_res_raw_dn7 = assign45360_e61261_d_n7;
        locals.var_vgp_res_raw_dn8 = assign45360_e61261_d_n8;
        locals.var_vgp_res_raw_dn9 = assign45360_e61261_d_n9;
        locals.var_vgp_res_raw_dn10 = assign45360_e61261_d_n10;
        locals.var_vgp_res_raw_dn13 = assign45360_e61261_d_n13;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign45370_e61272, assign45370_e61272_d_n0, assign45370_e61272_d_n2, assign45370_e61272_d_n4, assign45370_e61272_d_n5, assign45370_e61272_d_n6, assign45370_e61272_d_n7, assign45370_e61272_d_n8, assign45370_e61272_d_n9, assign45370_e61272_d_n10, assign45370_e61272_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        (locals.var_vbscl__blk435, locals.var_vbscl__blk435_dn0, locals.var_vbscl__blk435_dn2, locals.var_vbscl__blk435_dn4, locals.var_vbscl__blk435_dn5, locals.var_vbscl__blk435_dn6, locals.var_vbscl__blk435_dn7, locals.var_vbscl__blk435_dn8, locals.var_vbscl__blk435_dn9, locals.var_vbscl__blk435_dn10, locals.var_vbscl__blk435_dn13,)
    } else {
        (locals.var_vbsc__blk1117, locals.var_vbsc__blk1117_dn0, locals.var_vbsc__blk1117_dn2, locals.var_vbsc__blk1117_dn4, locals.var_vbsc__blk1117_dn5, locals.var_vbsc__blk1117_dn6, locals.var_vbsc__blk1117_dn7, locals.var_vbsc__blk1117_dn8, locals.var_vbsc__blk1117_dn9, locals.var_vbsc__blk1117_dn10, locals.var_vbsc__blk1117_dn13,)
    }
};
        locals.var_vbsc__blk1117 = assign45370_e61272;
        locals.var_vbsc__blk1117_dn0 = assign45370_e61272_d_n0;
        locals.var_vbsc__blk1117_dn2 = assign45370_e61272_d_n2;
        locals.var_vbsc__blk1117_dn4 = assign45370_e61272_d_n4;
        locals.var_vbsc__blk1117_dn5 = assign45370_e61272_d_n5;
        locals.var_vbsc__blk1117_dn6 = assign45370_e61272_d_n6;
        locals.var_vbsc__blk1117_dn7 = assign45370_e61272_d_n7;
        locals.var_vbsc__blk1117_dn8 = assign45370_e61272_d_n8;
        locals.var_vbsc__blk1117_dn9 = assign45370_e61272_d_n9;
        locals.var_vbsc__blk1117_dn10 = assign45370_e61272_d_n10;
        locals.var_vbsc__blk1117_dn13 = assign45370_e61272_d_n13;
        locals.var_vbsc__blk1117_rv = 0.0;

        let (assign45380_e61291, assign45380_e61291_d_n0, assign45380_e61291_d_n2, assign45380_e61291_d_n4, assign45380_e61291_d_n5, assign45380_e61291_d_n6, assign45380_e61291_d_n7, assign45380_e61291_d_n8, assign45380_e61291_d_n9, assign45380_e61291_d_n10, assign45380_e61291_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign45380_e61283: f64 = (locals.var_vgs - locals.var_vfb);
        let assign45380_e61287: f64 = (locals.var_dvth - locals.var_dppg);
        let assign45380_e61288: f64 = assign45380_e61287;
        let assign45380_e61289: f64 = (assign45380_e61283 + assign45380_e61288);
        (assign45380_e61289, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), (locals.var_dvth_dn4 - locals.var_dppg_dn4), (locals.var_vgs_dn5 + (locals.var_dvth_dn5 - locals.var_dppg_dn5)), (locals.var_vgs_dn6 + (locals.var_dvth_dn6 - locals.var_dppg_dn6)), (locals.var_vgs_dn7 + (locals.var_dvth_dn7 - locals.var_dppg_dn7)), (locals.var_dvth_dn8 - locals.var_dppg_dn8), (locals.var_dvth_dn9 - locals.var_dppg_dn9), (locals.var_dvth_dn10 - locals.var_dppg_dn10), (locals.var_dvth_dn13 - locals.var_dppg_dn13),)
    } else {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn13,)
    }
};
        locals.var_vgp = assign45380_e61291;
        locals.var_vgp_dn0 = assign45380_e61291_d_n0;
        locals.var_vgp_dn2 = assign45380_e61291_d_n2;
        locals.var_vgp_dn4 = assign45380_e61291_d_n4;
        locals.var_vgp_dn5 = assign45380_e61291_d_n5;
        locals.var_vgp_dn6 = assign45380_e61291_d_n6;
        locals.var_vgp_dn7 = assign45380_e61291_d_n7;
        locals.var_vgp_dn8 = assign45380_e61291_d_n8;
        locals.var_vgp_dn9 = assign45380_e61291_d_n9;
        locals.var_vgp_dn10 = assign45380_e61291_d_n10;
        locals.var_vgp_dn13 = assign45380_e61291_d_n13;
        locals.var_vgp_rv = 0.0;

        let (assign45390_e61306, assign45390_e61306_d_n0, assign45390_e61306_d_n2, assign45390_e61306_d_n4, assign45390_e61306_d_n5, assign45390_e61306_d_n6, assign45390_e61306_d_n7, assign45390_e61306_d_n8, assign45390_e61306_d_n9, assign45390_e61306_d_n10, assign45390_e61306_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign45390_e61302: f64 = (0.3 - locals.var_vgp);
        let assign45390_e61304: f64 = (assign45390_e61302 - 0.01);
        (assign45390_e61304, (-locals.var_vgp_dn0), (-locals.var_vgp_dn2), (-locals.var_vgp_dn4), (-locals.var_vgp_dn5), (-locals.var_vgp_dn6), (-locals.var_vgp_dn7), (-locals.var_vgp_dn8), (-locals.var_vgp_dn9), (-locals.var_vgp_dn10), (-locals.var_vgp_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign45390_e61306;
        locals.var_tmf1_dn0 = assign45390_e61306_d_n0;
        locals.var_tmf1_dn2 = assign45390_e61306_d_n2;
        locals.var_tmf1_dn4 = assign45390_e61306_d_n4;
        locals.var_tmf1_dn5 = assign45390_e61306_d_n5;
        locals.var_tmf1_dn6 = assign45390_e61306_d_n6;
        locals.var_tmf1_dn7 = assign45390_e61306_d_n7;
        locals.var_tmf1_dn8 = assign45390_e61306_d_n8;
        locals.var_tmf1_dn9 = assign45390_e61306_d_n9;
        locals.var_tmf1_dn10 = assign45390_e61306_d_n10;
        locals.var_tmf1_dn13 = assign45390_e61306_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign45400_e61321, assign45400_e61321_d_n0, assign45400_e61321_d_n2, assign45400_e61321_d_n4, assign45400_e61321_d_n5, assign45400_e61321_d_n6, assign45400_e61321_d_n7, assign45400_e61321_d_n8, assign45400_e61321_d_n9, assign45400_e61321_d_n10, assign45400_e61321_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign45400_e61317: f64 = (4.0 * 0.3);
        let assign45400_e61319: f64 = (assign45400_e61317 * 0.01);
        (assign45400_e61319, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign45400_e61321;
        locals.var_tmf2_dn0 = assign45400_e61321_d_n0;
        locals.var_tmf2_dn2 = assign45400_e61321_d_n2;
        locals.var_tmf2_dn4 = assign45400_e61321_d_n4;
        locals.var_tmf2_dn5 = assign45400_e61321_d_n5;
        locals.var_tmf2_dn6 = assign45400_e61321_d_n6;
        locals.var_tmf2_dn7 = assign45400_e61321_d_n7;
        locals.var_tmf2_dn8 = assign45400_e61321_d_n8;
        locals.var_tmf2_dn9 = assign45400_e61321_d_n9;
        locals.var_tmf2_dn10 = assign45400_e61321_d_n10;
        locals.var_tmf2_dn13 = assign45400_e61321_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign45410_e61338, assign45410_e61338_d_n0, assign45410_e61338_d_n2, assign45410_e61338_d_n4, assign45410_e61338_d_n5, assign45410_e61338_d_n6, assign45410_e61338_d_n7, assign45410_e61338_d_n8, assign45410_e61338_d_n9, assign45410_e61338_d_n10, assign45410_e61338_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign45410_e61336, assign45410_e61336_d_n0, assign45410_e61336_d_n2, assign45410_e61336_d_n4, assign45410_e61336_d_n5, assign45410_e61336_d_n6, assign45410_e61336_d_n7, assign45410_e61336_d_n8, assign45410_e61336_d_n9, assign45410_e61336_d_n10, assign45410_e61336_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign45410_e61335: f64 = (-locals.var_tmf2);
                (assign45410_e61335, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign45410_e61336, assign45410_e61336_d_n0, assign45410_e61336_d_n2, assign45410_e61336_d_n4, assign45410_e61336_d_n5, assign45410_e61336_d_n6, assign45410_e61336_d_n7, assign45410_e61336_d_n8, assign45410_e61336_d_n9, assign45410_e61336_d_n10, assign45410_e61336_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign45410_e61338;
        locals.var_tmf2_dn0 = assign45410_e61338_d_n0;
        locals.var_tmf2_dn2 = assign45410_e61338_d_n2;
        locals.var_tmf2_dn4 = assign45410_e61338_d_n4;
        locals.var_tmf2_dn5 = assign45410_e61338_d_n5;
        locals.var_tmf2_dn6 = assign45410_e61338_d_n6;
        locals.var_tmf2_dn7 = assign45410_e61338_d_n7;
        locals.var_tmf2_dn8 = assign45410_e61338_d_n8;
        locals.var_tmf2_dn9 = assign45410_e61338_d_n9;
        locals.var_tmf2_dn10 = assign45410_e61338_d_n10;
        locals.var_tmf2_dn13 = assign45410_e61338_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign45420_e61354, assign45420_e61354_d_n0, assign45420_e61354_d_n2, assign45420_e61354_d_n4, assign45420_e61354_d_n5, assign45420_e61354_d_n6, assign45420_e61354_d_n7, assign45420_e61354_d_n8, assign45420_e61354_d_n9, assign45420_e61354_d_n10, assign45420_e61354_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign45420_e61349: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign45420_e61351: f64 = (assign45420_e61349 + locals.var_tmf2);
        let assign45420_e61352: f64 = (assign45420_e61351).sqrt();
        (assign45420_e61352, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign45420_e61352)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign45420_e61352)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign45420_e61354;
        locals.var_tmf2_dn0 = assign45420_e61354_d_n0;
        locals.var_tmf2_dn2 = assign45420_e61354_d_n2;
        locals.var_tmf2_dn4 = assign45420_e61354_d_n4;
        locals.var_tmf2_dn5 = assign45420_e61354_d_n5;
        locals.var_tmf2_dn6 = assign45420_e61354_d_n6;
        locals.var_tmf2_dn7 = assign45420_e61354_d_n7;
        locals.var_tmf2_dn8 = assign45420_e61354_d_n8;
        locals.var_tmf2_dn9 = assign45420_e61354_d_n9;
        locals.var_tmf2_dn10 = assign45420_e61354_d_n10;
        locals.var_tmf2_dn13 = assign45420_e61354_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign45430_e61371, assign45430_e61371_d_n0, assign45430_e61371_d_n2, assign45430_e61371_d_n4, assign45430_e61371_d_n5, assign45430_e61371_d_n6, assign45430_e61371_d_n7, assign45430_e61371_d_n8, assign45430_e61371_d_n9, assign45430_e61371_d_n10, assign45430_e61371_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign45430_e61367: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign45430_e61368: f64 = (1.0 + assign45430_e61367);
        let assign45430_e61369: f64 = (0.5 * assign45430_e61368);
        (assign45430_e61369, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign45430_e61371;
        locals.var_t0_dn0 = assign45430_e61371_d_n0;
        locals.var_t0_dn2 = assign45430_e61371_d_n2;
        locals.var_t0_dn4 = assign45430_e61371_d_n4;
        locals.var_t0_dn5 = assign45430_e61371_d_n5;
        locals.var_t0_dn6 = assign45430_e61371_d_n6;
        locals.var_t0_dn7 = assign45430_e61371_d_n7;
        locals.var_t0_dn8 = assign45430_e61371_d_n8;
        locals.var_t0_dn9 = assign45430_e61371_d_n9;
        locals.var_t0_dn10 = assign45430_e61371_d_n10;
        locals.var_t0_dn13 = assign45430_e61371_d_n13;
        locals.var_t0_rv = 0.0;

    }
}
