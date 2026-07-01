#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_208(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign60420_e98145, assign60420_e98145_d_n3, assign60420_e98145_d_n4, assign60420_e98145_d_n5, assign60420_e98145_d_n6, assign60420_e98145_d_n7, assign60420_e98145_d_n8, assign60420_e98145_d_n9, assign60420_e98145_d_n10, assign60420_e98145_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60420_e98139: f64 = (10000000000.0 * locals.var_leffnoisq_edge);
        let assign60420_e98141: f64 = (assign60420_e98139 * p.p1147);
        let assign60420_e98143: f64 = (assign60420_e98141 * p.p2);
        (assign60420_e98143, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign60420_e98145;
        locals.var_t4_dn3 = assign60420_e98145_d_n3;
        locals.var_t4_dn4 = assign60420_e98145_d_n4;
        locals.var_t4_dn5 = assign60420_e98145_d_n5;
        locals.var_t4_dn6 = assign60420_e98145_d_n6;
        locals.var_t4_dn7 = assign60420_e98145_d_n7;
        locals.var_t4_dn8 = assign60420_e98145_d_n8;
        locals.var_t4_dn9 = assign60420_e98145_d_n9;
        locals.var_t4_dn10 = assign60420_e98145_d_n10;
        locals.var_t4_dn11 = assign60420_e98145_d_n11;

        let (assign60430_e98170, assign60430_e98170_d_n3, assign60430_e98170_d_n4, assign60430_e98170_d_n5, assign60430_e98170_d_n6, assign60430_e98170_d_n7, assign60430_e98170_d_n8, assign60430_e98170_d_n9, assign60430_e98170_d_n10, assign60430_e98170_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60430_e98152: f64 = (locals.var_t0a / locals.var_t0);
        let assign60430_e98155: f64 = (locals.var_t1 + locals.var_t2);
        let assign60430_e98157: f64 = (assign60430_e98155 + locals.var_t3);
        let assign60430_e98158: f64 = (assign60430_e98152 * assign60430_e98157);
        let assign60430_e98161: f64 = (locals.var_t0b / locals.var_t4);
        let assign60430_e98163: f64 = (assign60430_e98161 * locals.var_delclm);
        let assign60430_e98165: f64 = (assign60430_e98163 * locals.var_t0c);
        let assign60430_e98167: f64 = (assign60430_e98165 / locals.var_t0d);
        let assign60430_e98168: f64 = (assign60430_e98158 + assign60430_e98167);
        (assign60430_e98168, ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11,)
    }
};
        locals.var_ssi = assign60430_e98170;
        locals.var_ssi_dn3 = assign60430_e98170_d_n3;
        locals.var_ssi_dn4 = assign60430_e98170_d_n4;
        locals.var_ssi_dn5 = assign60430_e98170_d_n5;
        locals.var_ssi_dn6 = assign60430_e98170_d_n6;
        locals.var_ssi_dn7 = assign60430_e98170_d_n7;
        locals.var_ssi_dn8 = assign60430_e98170_d_n8;
        locals.var_ssi_dn9 = assign60430_e98170_d_n9;
        locals.var_ssi_dn10 = assign60430_e98170_d_n10;
        locals.var_ssi_dn11 = assign60430_e98170_d_n11;

        let (assign60440_e98187, assign60440_e98187_d_n3, assign60440_e98187_d_n4, assign60440_e98187_d_n5, assign60440_e98187_d_n6, assign60440_e98187_d_n7, assign60440_e98187_d_n8, assign60440_e98187_d_n9, assign60440_e98187_d_n10, assign60440_e98187_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60440_e98177: f64 = (p.p1147 * p.p2);
        let assign60440_e98179: f64 = (assign60440_e98177 * locals.var_leffnoi_edge);
        let assign60440_e98181: f64 = (assign60440_e98179 * 10000000000.0);
        let assign60440_e98183: f64 = (assign60440_e98181 * locals.var_nstar);
        let assign60440_e98185: f64 = (assign60440_e98183 * locals.var_nstar);
        (assign60440_e98185, (((assign60440_e98181 * locals.var_nstar_dn3) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn3)), (((assign60440_e98181 * locals.var_nstar_dn4) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn4)), (((assign60440_e98181 * locals.var_nstar_dn5) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn5)), (((assign60440_e98181 * locals.var_nstar_dn6) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn6)), (((assign60440_e98181 * locals.var_nstar_dn7) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn7)), (((assign60440_e98181 * locals.var_nstar_dn8) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn8)), (((assign60440_e98181 * locals.var_nstar_dn9) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn9)), (((assign60440_e98181 * locals.var_nstar_dn10) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn10)), (((assign60440_e98181 * locals.var_nstar_dn11) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign60440_e98187;
        locals.var_t5_dn3 = assign60440_e98187_d_n3;
        locals.var_t5_dn4 = assign60440_e98187_d_n4;
        locals.var_t5_dn5 = assign60440_e98187_d_n5;
        locals.var_t5_dn6 = assign60440_e98187_d_n6;
        locals.var_t5_dn7 = assign60440_e98187_d_n7;
        locals.var_t5_dn8 = assign60440_e98187_d_n8;
        locals.var_t5_dn9 = assign60440_e98187_d_n9;
        locals.var_t5_dn10 = assign60440_e98187_d_n10;
        locals.var_t5_dn11 = assign60440_e98187_d_n11;

        let (assign60450_e98200, assign60450_e98200_d_n3, assign60450_e98200_d_n4, assign60450_e98200_d_n5, assign60450_e98200_d_n6, assign60450_e98200_d_n7, assign60450_e98200_d_n8, assign60450_e98200_d_n9, assign60450_e98200_d_n10, assign60450_e98200_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60450_e98194: f64 = (locals.var_t0e / locals.var_t5);
        let assign60450_e98196: f64 = (assign60450_e98194 * locals.var_ids_edge);
        let assign60450_e98198: f64 = (assign60450_e98196 * locals.var_ids_edge);
        (assign60450_e98198, (((((-((locals.var_t0e * locals.var_t5_dn3) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn3)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn5)), (((((-((locals.var_t0e * locals.var_t5_dn6) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn6)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn6)), (((((-((locals.var_t0e * locals.var_t5_dn7) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn7)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn7)), (((((-((locals.var_t0e * locals.var_t5_dn8) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn8)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn8)), (((((-((locals.var_t0e * locals.var_t5_dn9) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn9)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn9)), (((((-((locals.var_t0e * locals.var_t5_dn10) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn10)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn10)), (((((-((locals.var_t0e * locals.var_t5_dn11) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn11)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn11)),)
    } else {
        (locals.var_swi, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11,)
    }
};
        locals.var_swi = assign60450_e98200;
        locals.var_swi_dn3 = assign60450_e98200_d_n3;
        locals.var_swi_dn4 = assign60450_e98200_d_n4;
        locals.var_swi_dn5 = assign60450_e98200_d_n5;
        locals.var_swi_dn6 = assign60450_e98200_d_n6;
        locals.var_swi_dn7 = assign60450_e98200_d_n7;
        locals.var_swi_dn8 = assign60450_e98200_d_n8;
        locals.var_swi_dn9 = assign60450_e98200_d_n9;
        locals.var_swi_dn10 = assign60450_e98200_d_n10;
        locals.var_swi_dn11 = assign60450_e98200_d_n11;

        let (assign60460_e98209, assign60460_e98209_d_n3, assign60460_e98209_d_n4, assign60460_e98209_d_n5, assign60460_e98209_d_n6, assign60460_e98209_d_n7, assign60460_e98209_d_n8, assign60460_e98209_d_n9, assign60460_e98209_d_n10, assign60460_e98209_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60460_e98207: f64 = (locals.var_swi + locals.var_ssi);
        (assign60460_e98207, (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign60460_e98209;
        locals.var_t6_dn3 = assign60460_e98209_d_n3;
        locals.var_t6_dn4 = assign60460_e98209_d_n4;
        locals.var_t6_dn5 = assign60460_e98209_d_n5;
        locals.var_t6_dn6 = assign60460_e98209_d_n6;
        locals.var_t6_dn7 = assign60460_e98209_d_n7;
        locals.var_t6_dn8 = assign60460_e98209_d_n8;
        locals.var_t6_dn9 = assign60460_e98209_d_n9;
        locals.var_t6_dn10 = assign60460_e98209_d_n10;
        locals.var_t6_dn11 = assign60460_e98209_d_n11;

        let assign60470_e98212: f64 = if locals.var_t6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign60470_e98212;

        let (assign60480_e98225, assign60480_e98225_d_n3, assign60480_e98225_d_n4, assign60480_e98225_d_n5, assign60480_e98225_d_n6, assign60480_e98225_d_n7, assign60480_e98225_d_n8, assign60480_e98225_d_n9, assign60480_e98225_d_n10, assign60480_e98225_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign60480_e98221: f64 = (locals.var_ssi * locals.var_swi);
        let assign60480_e98223: f64 = (assign60480_e98221 / locals.var_t6);
        (assign60480_e98223, (((((locals.var_ssi_dn3 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn3)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn4 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn4)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn5 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn5)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn6 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn6)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn7 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn7)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn8 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn8)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn9 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn9)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn10 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn10)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn11 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn11)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign60480_e98225;
        locals.var_t7_dn3 = assign60480_e98225_d_n3;
        locals.var_t7_dn4 = assign60480_e98225_d_n4;
        locals.var_t7_dn5 = assign60480_e98225_d_n5;
        locals.var_t7_dn6 = assign60480_e98225_d_n6;
        locals.var_t7_dn7 = assign60480_e98225_d_n7;
        locals.var_t7_dn8 = assign60480_e98225_d_n8;
        locals.var_t7_dn9 = assign60480_e98225_d_n9;
        locals.var_t7_dn10 = assign60480_e98225_d_n10;
        locals.var_t7_dn11 = assign60480_e98225_d_n11;

        let (assign60490_e98242, assign60490_e98242_d_n3, assign60490_e98242_d_n4, assign60490_e98242_d_n5, assign60490_e98242_d_n6, assign60490_e98242_d_n7, assign60490_e98242_d_n8, assign60490_e98242_d_n9, assign60490_e98242_d_n10, assign60490_e98242_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign60490_e98236: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign60490_e98238: f64 = (assign60490_e98236).powf(p.p1318);
        let assign60490_e98239: f64 = (p.p1317 * assign60490_e98238);
        let assign60490_e98240: f64 = (1.0 + assign60490_e98239);
        (assign60490_e98240, (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) / assign60490_e98236))) }),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign60490_e98242;
        locals.var_t8_dn3 = assign60490_e98242_d_n3;
        locals.var_t8_dn4 = assign60490_e98242_d_n4;
        locals.var_t8_dn5 = assign60490_e98242_d_n5;
        locals.var_t8_dn6 = assign60490_e98242_d_n6;
        locals.var_t8_dn7 = assign60490_e98242_d_n7;
        locals.var_t8_dn8 = assign60490_e98242_d_n8;
        locals.var_t8_dn9 = assign60490_e98242_d_n9;
        locals.var_t8_dn10 = assign60490_e98242_d_n10;
        locals.var_t8_dn11 = assign60490_e98242_d_n11;

        let (assign60520_e98276, assign60520_e98276_d_n3, assign60520_e98276_d_n4, assign60520_e98276_d_n5, assign60520_e98276_d_n6, assign60520_e98276_d_n7, assign60520_e98276_d_n8, assign60520_e98276_d_n9, assign60520_e98276_d_n10, assign60520_e98276_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60520_e98269: f64 = (locals.var_qbi + locals.var_qovb);
        let assign60520_e98271: f64 = (assign60520_e98269 + locals.var_qbsj);
        let assign60520_e98273: f64 = (assign60520_e98271 + locals.var_qbdj);
        let assign60520_e98274: f64 = (locals.var_devsign * assign60520_e98273);
        (assign60520_e98274, (locals.var_devsign * ((locals.var_qbi_dn3 + locals.var_qbsj_dn3) + locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qbi_dn4 + locals.var_qbsj_dn4) + locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qbi_dn5 + locals.var_qbsj_dn5) + locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qbi_dn6 + locals.var_qbsj_dn6) + locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qbi_dn7 + locals.var_qbsj_dn7) + locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qbi_dn8 + locals.var_qbsj_dn8) + locals.var_qbdj_dn8)), (locals.var_devsign * (((locals.var_qbi_dn9 + locals.var_qovb_dn9) + locals.var_qbsj_dn9) + locals.var_qbdj_dn9)), (locals.var_devsign * (((locals.var_qbi_dn10 + locals.var_qovb_dn10) + locals.var_qbsj_dn10) + locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qbi_dn11 + locals.var_qbsj_dn11) + locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qb_2, locals.var_qb_2_dn3, locals.var_qb_2_dn4, locals.var_qb_2_dn5, locals.var_qb_2_dn6, locals.var_qb_2_dn7, locals.var_qb_2_dn8, locals.var_qb_2_dn9, locals.var_qb_2_dn10, locals.var_qb_2_dn11,)
    }
};
        locals.var_qb_2 = assign60520_e98276;
        locals.var_qb_2_dn3 = assign60520_e98276_d_n3;
        locals.var_qb_2_dn4 = assign60520_e98276_d_n4;
        locals.var_qb_2_dn5 = assign60520_e98276_d_n5;
        locals.var_qb_2_dn6 = assign60520_e98276_d_n6;
        locals.var_qb_2_dn7 = assign60520_e98276_d_n7;
        locals.var_qb_2_dn8 = assign60520_e98276_d_n8;
        locals.var_qb_2_dn9 = assign60520_e98276_d_n9;
        locals.var_qb_2_dn10 = assign60520_e98276_d_n10;
        locals.var_qb_2_dn11 = assign60520_e98276_d_n11;

        let assign60530_e98279: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard878 = assign60530_e98279;

        let (assign60540_e98288, assign60540_e98288_d_n3, assign60540_e98288_d_n4, assign60540_e98288_d_n5, assign60540_e98288_d_n6, assign60540_e98288_d_n7, assign60540_e98288_d_n8, assign60540_e98288_d_n9, assign60540_e98288_d_n10, assign60540_e98288_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60540_e98286: f64 = (locals.var_devsign * locals.var_qsi);
        (assign60540_e98286, (locals.var_devsign * locals.var_qsi_dn3), (locals.var_devsign * locals.var_qsi_dn4), (locals.var_devsign * locals.var_qsi_dn5), (locals.var_devsign * locals.var_qsi_dn6), (locals.var_devsign * locals.var_qsi_dn7), (locals.var_devsign * locals.var_qsi_dn8), (locals.var_devsign * locals.var_qsi_dn9), (locals.var_devsign * locals.var_qsi_dn10), (locals.var_devsign * locals.var_qsi_dn11),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11,)
    }
};
        locals.var_qsi_1 = assign60540_e98288;
        locals.var_qsi_1_dn3 = assign60540_e98288_d_n3;
        locals.var_qsi_1_dn4 = assign60540_e98288_d_n4;
        locals.var_qsi_1_dn5 = assign60540_e98288_d_n5;
        locals.var_qsi_1_dn6 = assign60540_e98288_d_n6;
        locals.var_qsi_1_dn7 = assign60540_e98288_d_n7;
        locals.var_qsi_1_dn8 = assign60540_e98288_d_n8;
        locals.var_qsi_1_dn9 = assign60540_e98288_d_n9;
        locals.var_qsi_1_dn10 = assign60540_e98288_d_n10;
        locals.var_qsi_1_dn11 = assign60540_e98288_d_n11;

        let (assign60550_e98297, assign60550_e98297_d_n3, assign60550_e98297_d_n4, assign60550_e98297_d_n5, assign60550_e98297_d_n6, assign60550_e98297_d_n7, assign60550_e98297_d_n8, assign60550_e98297_d_n9, assign60550_e98297_d_n10, assign60550_e98297_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60550_e98295: f64 = (locals.var_devsign * locals.var_qsim);
        (assign60550_e98295, (locals.var_devsign * locals.var_qsim_dn3), (locals.var_devsign * locals.var_qsim_dn4), (locals.var_devsign * locals.var_qsim_dn5), (locals.var_devsign * locals.var_qsim_dn6), (locals.var_devsign * locals.var_qsim_dn7), (locals.var_devsign * locals.var_qsim_dn8), (locals.var_devsign * locals.var_qsim_dn9), (locals.var_devsign * locals.var_qsim_dn10), (locals.var_devsign * locals.var_qsim_dn11),)
    } else {
        (locals.var_qsim_1, locals.var_qsim_1_dn3, locals.var_qsim_1_dn4, locals.var_qsim_1_dn5, locals.var_qsim_1_dn6, locals.var_qsim_1_dn7, locals.var_qsim_1_dn8, locals.var_qsim_1_dn9, locals.var_qsim_1_dn10, locals.var_qsim_1_dn11,)
    }
};
        locals.var_qsim_1 = assign60550_e98297;
        locals.var_qsim_1_dn3 = assign60550_e98297_d_n3;
        locals.var_qsim_1_dn4 = assign60550_e98297_d_n4;
        locals.var_qsim_1_dn5 = assign60550_e98297_d_n5;
        locals.var_qsim_1_dn6 = assign60550_e98297_d_n6;
        locals.var_qsim_1_dn7 = assign60550_e98297_d_n7;
        locals.var_qsim_1_dn8 = assign60550_e98297_d_n8;
        locals.var_qsim_1_dn9 = assign60550_e98297_d_n9;
        locals.var_qsim_1_dn10 = assign60550_e98297_d_n10;
        locals.var_qsim_1_dn11 = assign60550_e98297_d_n11;

        let (assign60560_e98306, assign60560_e98306_d_n3, assign60560_e98306_d_n4, assign60560_e98306_d_n5, assign60560_e98306_d_n6, assign60560_e98306_d_n7, assign60560_e98306_d_n8, assign60560_e98306_d_n9, assign60560_e98306_d_n10, assign60560_e98306_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60560_e98304: f64 = (locals.var_devsign * locals.var_qsiagbcp2);
        (assign60560_e98304, (locals.var_devsign * locals.var_qsiagbcp2_dn3), (locals.var_devsign * locals.var_qsiagbcp2_dn4), (locals.var_devsign * locals.var_qsiagbcp2_dn5), (locals.var_devsign * locals.var_qsiagbcp2_dn6), (locals.var_devsign * locals.var_qsiagbcp2_dn7), (locals.var_devsign * locals.var_qsiagbcp2_dn8), (locals.var_devsign * locals.var_qsiagbcp2_dn9), (locals.var_devsign * locals.var_qsiagbcp2_dn10), (locals.var_devsign * locals.var_qsiagbcp2_dn11),)
    } else {
        (locals.var_qsiagbcp2_1, locals.var_qsiagbcp2_1_dn3, locals.var_qsiagbcp2_1_dn4, locals.var_qsiagbcp2_1_dn5, locals.var_qsiagbcp2_1_dn6, locals.var_qsiagbcp2_1_dn7, locals.var_qsiagbcp2_1_dn8, locals.var_qsiagbcp2_1_dn9, locals.var_qsiagbcp2_1_dn10, locals.var_qsiagbcp2_1_dn11,)
    }
};
        locals.var_qsiagbcp2_1 = assign60560_e98306;
        locals.var_qsiagbcp2_1_dn3 = assign60560_e98306_d_n3;
        locals.var_qsiagbcp2_1_dn4 = assign60560_e98306_d_n4;
        locals.var_qsiagbcp2_1_dn5 = assign60560_e98306_d_n5;
        locals.var_qsiagbcp2_1_dn6 = assign60560_e98306_d_n6;
        locals.var_qsiagbcp2_1_dn7 = assign60560_e98306_d_n7;
        locals.var_qsiagbcp2_1_dn8 = assign60560_e98306_d_n8;
        locals.var_qsiagbcp2_1_dn9 = assign60560_e98306_d_n9;
        locals.var_qsiagbcp2_1_dn10 = assign60560_e98306_d_n10;
        locals.var_qsiagbcp2_1_dn11 = assign60560_e98306_d_n11;

        let (assign60570_e98315, assign60570_e98315_d_n3, assign60570_e98315_d_n4, assign60570_e98315_d_n5, assign60570_e98315_d_n6, assign60570_e98315_d_n7, assign60570_e98315_d_n8, assign60570_e98315_d_n9, assign60570_e98315_d_n10, assign60570_e98315_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60570_e98313: f64 = (locals.var_devsign * locals.var_qdi);
        (assign60570_e98313, (locals.var_devsign * locals.var_qdi_dn3), (locals.var_devsign * locals.var_qdi_dn4), (locals.var_devsign * locals.var_qdi_dn5), (locals.var_devsign * locals.var_qdi_dn6), (locals.var_devsign * locals.var_qdi_dn7), (locals.var_devsign * locals.var_qdi_dn8), (locals.var_devsign * locals.var_qdi_dn9), (locals.var_devsign * locals.var_qdi_dn10), (locals.var_devsign * locals.var_qdi_dn11),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11,)
    }
};
        locals.var_qdi_1 = assign60570_e98315;
        locals.var_qdi_1_dn3 = assign60570_e98315_d_n3;
        locals.var_qdi_1_dn4 = assign60570_e98315_d_n4;
        locals.var_qdi_1_dn5 = assign60570_e98315_d_n5;
        locals.var_qdi_1_dn6 = assign60570_e98315_d_n6;
        locals.var_qdi_1_dn7 = assign60570_e98315_d_n7;
        locals.var_qdi_1_dn8 = assign60570_e98315_d_n8;
        locals.var_qdi_1_dn9 = assign60570_e98315_d_n9;
        locals.var_qdi_1_dn10 = assign60570_e98315_d_n10;
        locals.var_qdi_1_dn11 = assign60570_e98315_d_n11;

        let (assign60580_e98324, assign60580_e98324_d_n3, assign60580_e98324_d_n4, assign60580_e98324_d_n5, assign60580_e98324_d_n6, assign60580_e98324_d_n7, assign60580_e98324_d_n8, assign60580_e98324_d_n9, assign60580_e98324_d_n10, assign60580_e98324_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60580_e98322: f64 = (locals.var_devsign * locals.var_qdim);
        (assign60580_e98322, (locals.var_devsign * locals.var_qdim_dn3), (locals.var_devsign * locals.var_qdim_dn4), (locals.var_devsign * locals.var_qdim_dn5), (locals.var_devsign * locals.var_qdim_dn6), (locals.var_devsign * locals.var_qdim_dn7), (locals.var_devsign * locals.var_qdim_dn8), (locals.var_devsign * locals.var_qdim_dn9), (locals.var_devsign * locals.var_qdim_dn10), (locals.var_devsign * locals.var_qdim_dn11),)
    } else {
        (locals.var_qdim_1, locals.var_qdim_1_dn3, locals.var_qdim_1_dn4, locals.var_qdim_1_dn5, locals.var_qdim_1_dn6, locals.var_qdim_1_dn7, locals.var_qdim_1_dn8, locals.var_qdim_1_dn9, locals.var_qdim_1_dn10, locals.var_qdim_1_dn11,)
    }
};
        locals.var_qdim_1 = assign60580_e98324;
        locals.var_qdim_1_dn3 = assign60580_e98324_d_n3;
        locals.var_qdim_1_dn4 = assign60580_e98324_d_n4;
        locals.var_qdim_1_dn5 = assign60580_e98324_d_n5;
        locals.var_qdim_1_dn6 = assign60580_e98324_d_n6;
        locals.var_qdim_1_dn7 = assign60580_e98324_d_n7;
        locals.var_qdim_1_dn8 = assign60580_e98324_d_n8;
        locals.var_qdim_1_dn9 = assign60580_e98324_d_n9;
        locals.var_qdim_1_dn10 = assign60580_e98324_d_n10;
        locals.var_qdim_1_dn11 = assign60580_e98324_d_n11;

        let (assign60590_e98333, assign60590_e98333_d_n3, assign60590_e98333_d_n4, assign60590_e98333_d_n5, assign60590_e98333_d_n6, assign60590_e98333_d_n7, assign60590_e98333_d_n8, assign60590_e98333_d_n9, assign60590_e98333_d_n10, assign60590_e98333_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60590_e98331: f64 = (locals.var_devsign * locals.var_qdiagbcp2);
        (assign60590_e98331, (locals.var_devsign * locals.var_qdiagbcp2_dn3), (locals.var_devsign * locals.var_qdiagbcp2_dn4), (locals.var_devsign * locals.var_qdiagbcp2_dn5), (locals.var_devsign * locals.var_qdiagbcp2_dn6), (locals.var_devsign * locals.var_qdiagbcp2_dn7), (locals.var_devsign * locals.var_qdiagbcp2_dn8), (locals.var_devsign * locals.var_qdiagbcp2_dn9), (locals.var_devsign * locals.var_qdiagbcp2_dn10), (locals.var_devsign * locals.var_qdiagbcp2_dn11),)
    } else {
        (locals.var_qdiagbcp2_1, locals.var_qdiagbcp2_1_dn3, locals.var_qdiagbcp2_1_dn4, locals.var_qdiagbcp2_1_dn5, locals.var_qdiagbcp2_1_dn6, locals.var_qdiagbcp2_1_dn7, locals.var_qdiagbcp2_1_dn8, locals.var_qdiagbcp2_1_dn9, locals.var_qdiagbcp2_1_dn10, locals.var_qdiagbcp2_1_dn11,)
    }
};
        locals.var_qdiagbcp2_1 = assign60590_e98333;
        locals.var_qdiagbcp2_1_dn3 = assign60590_e98333_d_n3;
        locals.var_qdiagbcp2_1_dn4 = assign60590_e98333_d_n4;
        locals.var_qdiagbcp2_1_dn5 = assign60590_e98333_d_n5;
        locals.var_qdiagbcp2_1_dn6 = assign60590_e98333_d_n6;
        locals.var_qdiagbcp2_1_dn7 = assign60590_e98333_d_n7;
        locals.var_qdiagbcp2_1_dn8 = assign60590_e98333_d_n8;
        locals.var_qdiagbcp2_1_dn9 = assign60590_e98333_d_n9;
        locals.var_qdiagbcp2_1_dn10 = assign60590_e98333_d_n10;
        locals.var_qdiagbcp2_1_dn11 = assign60590_e98333_d_n11;

        let (assign60600_e98346, assign60600_e98346_d_n3, assign60600_e98346_d_n4, assign60600_e98346_d_n5, assign60600_e98346_d_n6, assign60600_e98346_d_n7, assign60600_e98346_d_n8, assign60600_e98346_d_n9, assign60600_e98346_d_n10, assign60600_e98346_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60600_e98341: f64 = (locals.var_qsi + locals.var_qovs);
        let assign60600_e98343: f64 = (assign60600_e98341 - locals.var_qbsj);
        let assign60600_e98344: f64 = (locals.var_devsign * assign60600_e98343);
        (assign60600_e98344, (locals.var_devsign * ((locals.var_qsi_dn3 + locals.var_qovs_dn3) - locals.var_qbsj_dn3)), (locals.var_devsign * ((locals.var_qsi_dn4 + locals.var_qovs_dn4) - locals.var_qbsj_dn4)), (locals.var_devsign * ((locals.var_qsi_dn5 + locals.var_qovs_dn5) - locals.var_qbsj_dn5)), (locals.var_devsign * ((locals.var_qsi_dn6 + locals.var_qovs_dn6) - locals.var_qbsj_dn6)), (locals.var_devsign * ((locals.var_qsi_dn7 + locals.var_qovs_dn7) - locals.var_qbsj_dn7)), (locals.var_devsign * ((locals.var_qsi_dn8 + locals.var_qovs_dn8) - locals.var_qbsj_dn8)), (locals.var_devsign * ((locals.var_qsi_dn9 + locals.var_qovs_dn9) - locals.var_qbsj_dn9)), (locals.var_devsign * ((locals.var_qsi_dn10 + locals.var_qovs_dn10) - locals.var_qbsj_dn10)), (locals.var_devsign * ((locals.var_qsi_dn11 + locals.var_qovs_dn11) - locals.var_qbsj_dn11)),)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign60600_e98346;
        locals.var_qs_2_dn3 = assign60600_e98346_d_n3;
        locals.var_qs_2_dn4 = assign60600_e98346_d_n4;
        locals.var_qs_2_dn5 = assign60600_e98346_d_n5;
        locals.var_qs_2_dn6 = assign60600_e98346_d_n6;
        locals.var_qs_2_dn7 = assign60600_e98346_d_n7;
        locals.var_qs_2_dn8 = assign60600_e98346_d_n8;
        locals.var_qs_2_dn9 = assign60600_e98346_d_n9;
        locals.var_qs_2_dn10 = assign60600_e98346_d_n10;
        locals.var_qs_2_dn11 = assign60600_e98346_d_n11;

        let (assign60610_e98359, assign60610_e98359_d_n3, assign60610_e98359_d_n4, assign60610_e98359_d_n5, assign60610_e98359_d_n6, assign60610_e98359_d_n7, assign60610_e98359_d_n8, assign60610_e98359_d_n9, assign60610_e98359_d_n10, assign60610_e98359_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60610_e98354: f64 = (locals.var_qdi + locals.var_qovd);
        let assign60610_e98356: f64 = (assign60610_e98354 - locals.var_qbdj);
        let assign60610_e98357: f64 = (locals.var_devsign * assign60610_e98356);
        (assign60610_e98357, (locals.var_devsign * ((locals.var_qdi_dn3 + locals.var_qovd_dn3) - locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qdi_dn4 + locals.var_qovd_dn4) - locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qdi_dn5 + locals.var_qovd_dn5) - locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qdi_dn6 + locals.var_qovd_dn6) - locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qdi_dn7 + locals.var_qovd_dn7) - locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qdi_dn8 + locals.var_qovd_dn8) - locals.var_qbdj_dn8)), (locals.var_devsign * ((locals.var_qdi_dn9 + locals.var_qovd_dn9) - locals.var_qbdj_dn9)), (locals.var_devsign * ((locals.var_qdi_dn10 + locals.var_qovd_dn10) - locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qdi_dn11 + locals.var_qovd_dn11) - locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign60610_e98359;
        locals.var_qd_1_dn3 = assign60610_e98359_d_n3;
        locals.var_qd_1_dn4 = assign60610_e98359_d_n4;
        locals.var_qd_1_dn5 = assign60610_e98359_d_n5;
        locals.var_qd_1_dn6 = assign60610_e98359_d_n6;
        locals.var_qd_1_dn7 = assign60610_e98359_d_n7;
        locals.var_qd_1_dn8 = assign60610_e98359_d_n8;
        locals.var_qd_1_dn9 = assign60610_e98359_d_n9;
        locals.var_qd_1_dn10 = assign60610_e98359_d_n10;
        locals.var_qd_1_dn11 = assign60610_e98359_d_n11;

        let (assign60620_e98369, assign60620_e98369_d_n3, assign60620_e98369_d_n4, assign60620_e98369_d_n5, assign60620_e98369_d_n6, assign60620_e98369_d_n7, assign60620_e98369_d_n8, assign60620_e98369_d_n9, assign60620_e98369_d_n10, assign60620_e98369_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60620_e98367: f64 = (locals.var_devsign * locals.var_qdi);
        (assign60620_e98367, (locals.var_devsign * locals.var_qdi_dn3), (locals.var_devsign * locals.var_qdi_dn4), (locals.var_devsign * locals.var_qdi_dn5), (locals.var_devsign * locals.var_qdi_dn6), (locals.var_devsign * locals.var_qdi_dn7), (locals.var_devsign * locals.var_qdi_dn8), (locals.var_devsign * locals.var_qdi_dn9), (locals.var_devsign * locals.var_qdi_dn10), (locals.var_devsign * locals.var_qdi_dn11),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11,)
    }
};
        locals.var_qsi_1 = assign60620_e98369;
        locals.var_qsi_1_dn3 = assign60620_e98369_d_n3;
        locals.var_qsi_1_dn4 = assign60620_e98369_d_n4;
        locals.var_qsi_1_dn5 = assign60620_e98369_d_n5;
        locals.var_qsi_1_dn6 = assign60620_e98369_d_n6;
        locals.var_qsi_1_dn7 = assign60620_e98369_d_n7;
        locals.var_qsi_1_dn8 = assign60620_e98369_d_n8;
        locals.var_qsi_1_dn9 = assign60620_e98369_d_n9;
        locals.var_qsi_1_dn10 = assign60620_e98369_d_n10;
        locals.var_qsi_1_dn11 = assign60620_e98369_d_n11;

        let (assign60630_e98379, assign60630_e98379_d_n3, assign60630_e98379_d_n4, assign60630_e98379_d_n5, assign60630_e98379_d_n6, assign60630_e98379_d_n7, assign60630_e98379_d_n8, assign60630_e98379_d_n9, assign60630_e98379_d_n10, assign60630_e98379_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60630_e98377: f64 = (locals.var_devsign * locals.var_qdim);
        (assign60630_e98377, (locals.var_devsign * locals.var_qdim_dn3), (locals.var_devsign * locals.var_qdim_dn4), (locals.var_devsign * locals.var_qdim_dn5), (locals.var_devsign * locals.var_qdim_dn6), (locals.var_devsign * locals.var_qdim_dn7), (locals.var_devsign * locals.var_qdim_dn8), (locals.var_devsign * locals.var_qdim_dn9), (locals.var_devsign * locals.var_qdim_dn10), (locals.var_devsign * locals.var_qdim_dn11),)
    } else {
        (locals.var_qsim_1, locals.var_qsim_1_dn3, locals.var_qsim_1_dn4, locals.var_qsim_1_dn5, locals.var_qsim_1_dn6, locals.var_qsim_1_dn7, locals.var_qsim_1_dn8, locals.var_qsim_1_dn9, locals.var_qsim_1_dn10, locals.var_qsim_1_dn11,)
    }
};
        locals.var_qsim_1 = assign60630_e98379;
        locals.var_qsim_1_dn3 = assign60630_e98379_d_n3;
        locals.var_qsim_1_dn4 = assign60630_e98379_d_n4;
        locals.var_qsim_1_dn5 = assign60630_e98379_d_n5;
        locals.var_qsim_1_dn6 = assign60630_e98379_d_n6;
        locals.var_qsim_1_dn7 = assign60630_e98379_d_n7;
        locals.var_qsim_1_dn8 = assign60630_e98379_d_n8;
        locals.var_qsim_1_dn9 = assign60630_e98379_d_n9;
        locals.var_qsim_1_dn10 = assign60630_e98379_d_n10;
        locals.var_qsim_1_dn11 = assign60630_e98379_d_n11;

        let (assign60640_e98389, assign60640_e98389_d_n3, assign60640_e98389_d_n4, assign60640_e98389_d_n5, assign60640_e98389_d_n6, assign60640_e98389_d_n7, assign60640_e98389_d_n8, assign60640_e98389_d_n9, assign60640_e98389_d_n10, assign60640_e98389_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60640_e98387: f64 = (locals.var_devsign * locals.var_qdiagbcp2);
        (assign60640_e98387, (locals.var_devsign * locals.var_qdiagbcp2_dn3), (locals.var_devsign * locals.var_qdiagbcp2_dn4), (locals.var_devsign * locals.var_qdiagbcp2_dn5), (locals.var_devsign * locals.var_qdiagbcp2_dn6), (locals.var_devsign * locals.var_qdiagbcp2_dn7), (locals.var_devsign * locals.var_qdiagbcp2_dn8), (locals.var_devsign * locals.var_qdiagbcp2_dn9), (locals.var_devsign * locals.var_qdiagbcp2_dn10), (locals.var_devsign * locals.var_qdiagbcp2_dn11),)
    } else {
        (locals.var_qsiagbcp2_1, locals.var_qsiagbcp2_1_dn3, locals.var_qsiagbcp2_1_dn4, locals.var_qsiagbcp2_1_dn5, locals.var_qsiagbcp2_1_dn6, locals.var_qsiagbcp2_1_dn7, locals.var_qsiagbcp2_1_dn8, locals.var_qsiagbcp2_1_dn9, locals.var_qsiagbcp2_1_dn10, locals.var_qsiagbcp2_1_dn11,)
    }
};
        locals.var_qsiagbcp2_1 = assign60640_e98389;
        locals.var_qsiagbcp2_1_dn3 = assign60640_e98389_d_n3;
        locals.var_qsiagbcp2_1_dn4 = assign60640_e98389_d_n4;
        locals.var_qsiagbcp2_1_dn5 = assign60640_e98389_d_n5;
        locals.var_qsiagbcp2_1_dn6 = assign60640_e98389_d_n6;
        locals.var_qsiagbcp2_1_dn7 = assign60640_e98389_d_n7;
        locals.var_qsiagbcp2_1_dn8 = assign60640_e98389_d_n8;
        locals.var_qsiagbcp2_1_dn9 = assign60640_e98389_d_n9;
        locals.var_qsiagbcp2_1_dn10 = assign60640_e98389_d_n10;
        locals.var_qsiagbcp2_1_dn11 = assign60640_e98389_d_n11;

        let (assign60650_e98399, assign60650_e98399_d_n3, assign60650_e98399_d_n4, assign60650_e98399_d_n5, assign60650_e98399_d_n6, assign60650_e98399_d_n7, assign60650_e98399_d_n8, assign60650_e98399_d_n9, assign60650_e98399_d_n10, assign60650_e98399_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60650_e98397: f64 = (locals.var_devsign * locals.var_qsi);
        (assign60650_e98397, (locals.var_devsign * locals.var_qsi_dn3), (locals.var_devsign * locals.var_qsi_dn4), (locals.var_devsign * locals.var_qsi_dn5), (locals.var_devsign * locals.var_qsi_dn6), (locals.var_devsign * locals.var_qsi_dn7), (locals.var_devsign * locals.var_qsi_dn8), (locals.var_devsign * locals.var_qsi_dn9), (locals.var_devsign * locals.var_qsi_dn10), (locals.var_devsign * locals.var_qsi_dn11),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11,)
    }
};
        locals.var_qdi_1 = assign60650_e98399;
        locals.var_qdi_1_dn3 = assign60650_e98399_d_n3;
        locals.var_qdi_1_dn4 = assign60650_e98399_d_n4;
        locals.var_qdi_1_dn5 = assign60650_e98399_d_n5;
        locals.var_qdi_1_dn6 = assign60650_e98399_d_n6;
        locals.var_qdi_1_dn7 = assign60650_e98399_d_n7;
        locals.var_qdi_1_dn8 = assign60650_e98399_d_n8;
        locals.var_qdi_1_dn9 = assign60650_e98399_d_n9;
        locals.var_qdi_1_dn10 = assign60650_e98399_d_n10;
        locals.var_qdi_1_dn11 = assign60650_e98399_d_n11;

        let (assign60660_e98409, assign60660_e98409_d_n3, assign60660_e98409_d_n4, assign60660_e98409_d_n5, assign60660_e98409_d_n6, assign60660_e98409_d_n7, assign60660_e98409_d_n8, assign60660_e98409_d_n9, assign60660_e98409_d_n10, assign60660_e98409_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60660_e98407: f64 = (locals.var_devsign * locals.var_qsim);
        (assign60660_e98407, (locals.var_devsign * locals.var_qsim_dn3), (locals.var_devsign * locals.var_qsim_dn4), (locals.var_devsign * locals.var_qsim_dn5), (locals.var_devsign * locals.var_qsim_dn6), (locals.var_devsign * locals.var_qsim_dn7), (locals.var_devsign * locals.var_qsim_dn8), (locals.var_devsign * locals.var_qsim_dn9), (locals.var_devsign * locals.var_qsim_dn10), (locals.var_devsign * locals.var_qsim_dn11),)
    } else {
        (locals.var_qdim_1, locals.var_qdim_1_dn3, locals.var_qdim_1_dn4, locals.var_qdim_1_dn5, locals.var_qdim_1_dn6, locals.var_qdim_1_dn7, locals.var_qdim_1_dn8, locals.var_qdim_1_dn9, locals.var_qdim_1_dn10, locals.var_qdim_1_dn11,)
    }
};
        locals.var_qdim_1 = assign60660_e98409;
        locals.var_qdim_1_dn3 = assign60660_e98409_d_n3;
        locals.var_qdim_1_dn4 = assign60660_e98409_d_n4;
        locals.var_qdim_1_dn5 = assign60660_e98409_d_n5;
        locals.var_qdim_1_dn6 = assign60660_e98409_d_n6;
        locals.var_qdim_1_dn7 = assign60660_e98409_d_n7;
        locals.var_qdim_1_dn8 = assign60660_e98409_d_n8;
        locals.var_qdim_1_dn9 = assign60660_e98409_d_n9;
        locals.var_qdim_1_dn10 = assign60660_e98409_d_n10;
        locals.var_qdim_1_dn11 = assign60660_e98409_d_n11;

        let (assign60670_e98419, assign60670_e98419_d_n3, assign60670_e98419_d_n4, assign60670_e98419_d_n5, assign60670_e98419_d_n6, assign60670_e98419_d_n7, assign60670_e98419_d_n8, assign60670_e98419_d_n9, assign60670_e98419_d_n10, assign60670_e98419_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60670_e98417: f64 = (locals.var_devsign * locals.var_qsiagbcp2);
        (assign60670_e98417, (locals.var_devsign * locals.var_qsiagbcp2_dn3), (locals.var_devsign * locals.var_qsiagbcp2_dn4), (locals.var_devsign * locals.var_qsiagbcp2_dn5), (locals.var_devsign * locals.var_qsiagbcp2_dn6), (locals.var_devsign * locals.var_qsiagbcp2_dn7), (locals.var_devsign * locals.var_qsiagbcp2_dn8), (locals.var_devsign * locals.var_qsiagbcp2_dn9), (locals.var_devsign * locals.var_qsiagbcp2_dn10), (locals.var_devsign * locals.var_qsiagbcp2_dn11),)
    } else {
        (locals.var_qdiagbcp2_1, locals.var_qdiagbcp2_1_dn3, locals.var_qdiagbcp2_1_dn4, locals.var_qdiagbcp2_1_dn5, locals.var_qdiagbcp2_1_dn6, locals.var_qdiagbcp2_1_dn7, locals.var_qdiagbcp2_1_dn8, locals.var_qdiagbcp2_1_dn9, locals.var_qdiagbcp2_1_dn10, locals.var_qdiagbcp2_1_dn11,)
    }
};
        locals.var_qdiagbcp2_1 = assign60670_e98419;
        locals.var_qdiagbcp2_1_dn3 = assign60670_e98419_d_n3;
        locals.var_qdiagbcp2_1_dn4 = assign60670_e98419_d_n4;
        locals.var_qdiagbcp2_1_dn5 = assign60670_e98419_d_n5;
        locals.var_qdiagbcp2_1_dn6 = assign60670_e98419_d_n6;
        locals.var_qdiagbcp2_1_dn7 = assign60670_e98419_d_n7;
        locals.var_qdiagbcp2_1_dn8 = assign60670_e98419_d_n8;
        locals.var_qdiagbcp2_1_dn9 = assign60670_e98419_d_n9;
        locals.var_qdiagbcp2_1_dn10 = assign60670_e98419_d_n10;
        locals.var_qdiagbcp2_1_dn11 = assign60670_e98419_d_n11;

        let (assign60680_e98433, assign60680_e98433_d_n3, assign60680_e98433_d_n4, assign60680_e98433_d_n5, assign60680_e98433_d_n6, assign60680_e98433_d_n7, assign60680_e98433_d_n8, assign60680_e98433_d_n9, assign60680_e98433_d_n10, assign60680_e98433_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60680_e98428: f64 = (locals.var_qdi + locals.var_qovs);
        let assign60680_e98430: f64 = (assign60680_e98428 - locals.var_qbsj);
        let assign60680_e98431: f64 = (locals.var_devsign * assign60680_e98430);
        (assign60680_e98431, (locals.var_devsign * ((locals.var_qdi_dn3 + locals.var_qovs_dn3) - locals.var_qbsj_dn3)), (locals.var_devsign * ((locals.var_qdi_dn4 + locals.var_qovs_dn4) - locals.var_qbsj_dn4)), (locals.var_devsign * ((locals.var_qdi_dn5 + locals.var_qovs_dn5) - locals.var_qbsj_dn5)), (locals.var_devsign * ((locals.var_qdi_dn6 + locals.var_qovs_dn6) - locals.var_qbsj_dn6)), (locals.var_devsign * ((locals.var_qdi_dn7 + locals.var_qovs_dn7) - locals.var_qbsj_dn7)), (locals.var_devsign * ((locals.var_qdi_dn8 + locals.var_qovs_dn8) - locals.var_qbsj_dn8)), (locals.var_devsign * ((locals.var_qdi_dn9 + locals.var_qovs_dn9) - locals.var_qbsj_dn9)), (locals.var_devsign * ((locals.var_qdi_dn10 + locals.var_qovs_dn10) - locals.var_qbsj_dn10)), (locals.var_devsign * ((locals.var_qdi_dn11 + locals.var_qovs_dn11) - locals.var_qbsj_dn11)),)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign60680_e98433;
        locals.var_qs_2_dn3 = assign60680_e98433_d_n3;
        locals.var_qs_2_dn4 = assign60680_e98433_d_n4;
        locals.var_qs_2_dn5 = assign60680_e98433_d_n5;
        locals.var_qs_2_dn6 = assign60680_e98433_d_n6;
        locals.var_qs_2_dn7 = assign60680_e98433_d_n7;
        locals.var_qs_2_dn8 = assign60680_e98433_d_n8;
        locals.var_qs_2_dn9 = assign60680_e98433_d_n9;
        locals.var_qs_2_dn10 = assign60680_e98433_d_n10;
        locals.var_qs_2_dn11 = assign60680_e98433_d_n11;

        let (assign60690_e98447, assign60690_e98447_d_n3, assign60690_e98447_d_n4, assign60690_e98447_d_n5, assign60690_e98447_d_n6, assign60690_e98447_d_n7, assign60690_e98447_d_n8, assign60690_e98447_d_n9, assign60690_e98447_d_n10, assign60690_e98447_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60690_e98442: f64 = (locals.var_qsi + locals.var_qovd);
        let assign60690_e98444: f64 = (assign60690_e98442 - locals.var_qbdj);
        let assign60690_e98445: f64 = (locals.var_devsign * assign60690_e98444);
        (assign60690_e98445, (locals.var_devsign * ((locals.var_qsi_dn3 + locals.var_qovd_dn3) - locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qsi_dn4 + locals.var_qovd_dn4) - locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qsi_dn5 + locals.var_qovd_dn5) - locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qsi_dn6 + locals.var_qovd_dn6) - locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qsi_dn7 + locals.var_qovd_dn7) - locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qsi_dn8 + locals.var_qovd_dn8) - locals.var_qbdj_dn8)), (locals.var_devsign * ((locals.var_qsi_dn9 + locals.var_qovd_dn9) - locals.var_qbdj_dn9)), (locals.var_devsign * ((locals.var_qsi_dn10 + locals.var_qovd_dn10) - locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qsi_dn11 + locals.var_qovd_dn11) - locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign60690_e98447;
        locals.var_qd_1_dn3 = assign60690_e98447_d_n3;
        locals.var_qd_1_dn4 = assign60690_e98447_d_n4;
        locals.var_qd_1_dn5 = assign60690_e98447_d_n5;
        locals.var_qd_1_dn6 = assign60690_e98447_d_n6;
        locals.var_qd_1_dn7 = assign60690_e98447_d_n7;
        locals.var_qd_1_dn8 = assign60690_e98447_d_n8;
        locals.var_qd_1_dn9 = assign60690_e98447_d_n9;
        locals.var_qd_1_dn10 = assign60690_e98447_d_n10;
        locals.var_qd_1_dn11 = assign60690_e98447_d_n11;

    }

    pub(super) fn stamp_transient_block_209(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign60700_e98456, assign60700_e98456_d_n3, assign60700_e98456_d_n4, assign60700_e98456_d_n5, assign60700_e98456_d_n6, assign60700_e98456_d_n7, assign60700_e98456_d_n8, assign60700_e98456_d_n9, assign60700_e98456_d_n10, assign60700_e98456_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60700_e98453: f64 = (locals.var_qgi + locals.var_qovg);
        let assign60700_e98454: f64 = (locals.var_devsign * assign60700_e98453);
        (assign60700_e98454, (locals.var_devsign * (locals.var_qgi_dn3 + locals.var_qovg_dn3)), (locals.var_devsign * (locals.var_qgi_dn4 + locals.var_qovg_dn4)), (locals.var_devsign * (locals.var_qgi_dn5 + locals.var_qovg_dn5)), (locals.var_devsign * (locals.var_qgi_dn6 + locals.var_qovg_dn6)), (locals.var_devsign * (locals.var_qgi_dn7 + locals.var_qovg_dn7)), (locals.var_devsign * (locals.var_qgi_dn8 + locals.var_qovg_dn8)), (locals.var_devsign * (locals.var_qgi_dn9 + locals.var_qovg_dn9)), (locals.var_devsign * (locals.var_qgi_dn10 + locals.var_qovg_dn10)), (locals.var_devsign * (locals.var_qgi_dn11 + locals.var_qovg_dn11)),)
    } else {
        (locals.var_qg, locals.var_qg_dn3, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11,)
    }
};
        locals.var_qg = assign60700_e98456;
        locals.var_qg_dn3 = assign60700_e98456_d_n3;
        locals.var_qg_dn4 = assign60700_e98456_d_n4;
        locals.var_qg_dn5 = assign60700_e98456_d_n5;
        locals.var_qg_dn6 = assign60700_e98456_d_n6;
        locals.var_qg_dn7 = assign60700_e98456_d_n7;
        locals.var_qg_dn8 = assign60700_e98456_d_n8;
        locals.var_qg_dn9 = assign60700_e98456_d_n9;
        locals.var_qg_dn10 = assign60700_e98456_d_n10;
        locals.var_qg_dn11 = assign60700_e98456_d_n11;

        let (assign60730_e98477, assign60730_e98477_d_n3, assign60730_e98477_d_n4, assign60730_e98477_d_n5, assign60730_e98477_d_n6, assign60730_e98477_d_n7, assign60730_e98477_d_n8, assign60730_e98477_d_n9, assign60730_e98477_d_n10, assign60730_e98477_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60730_e98475: f64 = (locals.var_devsign * locals.var_qgim);
        (assign60730_e98475, (locals.var_devsign * locals.var_qgim_dn3), (locals.var_devsign * locals.var_qgim_dn4), (locals.var_devsign * locals.var_qgim_dn5), (locals.var_devsign * locals.var_qgim_dn6), (locals.var_devsign * locals.var_qgim_dn7), (locals.var_devsign * locals.var_qgim_dn8), (locals.var_devsign * locals.var_qgim_dn9), (locals.var_devsign * locals.var_qgim_dn10), (locals.var_devsign * locals.var_qgim_dn11),)
    } else {
        (locals.var_qgim_1, locals.var_qgim_1_dn3, locals.var_qgim_1_dn4, locals.var_qgim_1_dn5, locals.var_qgim_1_dn6, locals.var_qgim_1_dn7, locals.var_qgim_1_dn8, locals.var_qgim_1_dn9, locals.var_qgim_1_dn10, locals.var_qgim_1_dn11,)
    }
};
        locals.var_qgim_1 = assign60730_e98477;
        locals.var_qgim_1_dn3 = assign60730_e98477_d_n3;
        locals.var_qgim_1_dn4 = assign60730_e98477_d_n4;
        locals.var_qgim_1_dn5 = assign60730_e98477_d_n5;
        locals.var_qgim_1_dn6 = assign60730_e98477_d_n6;
        locals.var_qgim_1_dn7 = assign60730_e98477_d_n7;
        locals.var_qgim_1_dn8 = assign60730_e98477_d_n8;
        locals.var_qgim_1_dn9 = assign60730_e98477_d_n9;
        locals.var_qgim_1_dn10 = assign60730_e98477_d_n10;
        locals.var_qgim_1_dn11 = assign60730_e98477_d_n11;

        let (assign60740_e98484, assign60740_e98484_d_n3, assign60740_e98484_d_n4, assign60740_e98484_d_n5, assign60740_e98484_d_n6, assign60740_e98484_d_n7, assign60740_e98484_d_n8, assign60740_e98484_d_n9, assign60740_e98484_d_n10, assign60740_e98484_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60740_e98482: f64 = (locals.var_devsign * locals.var_qgiagbcp2);
        (assign60740_e98482, (locals.var_devsign * locals.var_qgiagbcp2_dn3), (locals.var_devsign * locals.var_qgiagbcp2_dn4), (locals.var_devsign * locals.var_qgiagbcp2_dn5), (locals.var_devsign * locals.var_qgiagbcp2_dn6), (locals.var_devsign * locals.var_qgiagbcp2_dn7), (locals.var_devsign * locals.var_qgiagbcp2_dn8), (locals.var_devsign * locals.var_qgiagbcp2_dn9), (locals.var_devsign * locals.var_qgiagbcp2_dn10), (locals.var_devsign * locals.var_qgiagbcp2_dn11),)
    } else {
        (locals.var_qgiagbcp2_1, locals.var_qgiagbcp2_1_dn3, locals.var_qgiagbcp2_1_dn4, locals.var_qgiagbcp2_1_dn5, locals.var_qgiagbcp2_1_dn6, locals.var_qgiagbcp2_1_dn7, locals.var_qgiagbcp2_1_dn8, locals.var_qgiagbcp2_1_dn9, locals.var_qgiagbcp2_1_dn10, locals.var_qgiagbcp2_1_dn11,)
    }
};
        locals.var_qgiagbcp2_1 = assign60740_e98484;
        locals.var_qgiagbcp2_1_dn3 = assign60740_e98484_d_n3;
        locals.var_qgiagbcp2_1_dn4 = assign60740_e98484_d_n4;
        locals.var_qgiagbcp2_1_dn5 = assign60740_e98484_d_n5;
        locals.var_qgiagbcp2_1_dn6 = assign60740_e98484_d_n6;
        locals.var_qgiagbcp2_1_dn7 = assign60740_e98484_d_n7;
        locals.var_qgiagbcp2_1_dn8 = assign60740_e98484_d_n8;
        locals.var_qgiagbcp2_1_dn9 = assign60740_e98484_d_n9;
        locals.var_qgiagbcp2_1_dn10 = assign60740_e98484_d_n10;
        locals.var_qgiagbcp2_1_dn11 = assign60740_e98484_d_n11;

        locals.var_weff_1 = locals.var_weff;

        locals.var_leff_1 = locals.var_leff;

        let assign61350_e98789: f64 = if p.p38 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard881 = assign61350_e98789;

        let assign61360_e98792: f64 = if p.p37 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard882 = assign61360_e98792;

        let assign61370_e98795: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard883 = assign61370_e98795;

        let assign61380_e98798: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard884 = assign61380_e98798;

        let assign61390_e98801: f64 = if p.p7 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard887 = assign61390_e98801;

        let (assign61400_e98808, assign61400_e98808_d_n3, assign61400_e98808_d_n4, assign61400_e98808_d_n5, assign61400_e98808_d_n6, assign61400_e98808_d_n7, assign61400_e98808_d_n8, assign61400_e98808_d_n9, assign61400_e98808_d_n10, assign61400_e98808_d_n11,) = {
    if ((locals.var_guard884 == 0.0) && (locals.var_guard887 != 0.0)) {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    } else {
        (locals.var_ggate, locals.var_ggate_dn3, locals.var_ggate_dn4, locals.var_ggate_dn5, locals.var_ggate_dn6, locals.var_ggate_dn7, locals.var_ggate_dn8, locals.var_ggate_dn9, locals.var_ggate_dn10, locals.var_ggate_dn11,)
    }
};
        locals.var_ggate = assign61400_e98808;
        locals.var_ggate_dn3 = assign61400_e98808_d_n3;
        locals.var_ggate_dn4 = assign61400_e98808_d_n4;
        locals.var_ggate_dn5 = assign61400_e98808_d_n5;
        locals.var_ggate_dn6 = assign61400_e98808_d_n6;
        locals.var_ggate_dn7 = assign61400_e98808_d_n7;
        locals.var_ggate_dn8 = assign61400_e98808_d_n8;
        locals.var_ggate_dn9 = assign61400_e98808_d_n9;
        locals.var_ggate_dn10 = assign61400_e98808_d_n10;
        locals.var_ggate_dn11 = assign61400_e98808_d_n11;

        let (assign61420_e98827, assign61420_e98827_d_n3, assign61420_e98827_d_n4, assign61420_e98827_d_n5, assign61420_e98827_d_n6, assign61420_e98827_d_n7, assign61420_e98827_d_n8, assign61420_e98827_d_n9, assign61420_e98827_d_n10, assign61420_e98827_d_n11,) = {
    if ((locals.var_guard884 == 0.0) && (locals.var_guard887 == 0.0)) {
        (locals.var_grgeltd, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ggate, locals.var_ggate_dn3, locals.var_ggate_dn4, locals.var_ggate_dn5, locals.var_ggate_dn6, locals.var_ggate_dn7, locals.var_ggate_dn8, locals.var_ggate_dn9, locals.var_ggate_dn10, locals.var_ggate_dn11,)
    }
};
        locals.var_ggate = assign61420_e98827;
        locals.var_ggate_dn3 = assign61420_e98827_d_n3;
        locals.var_ggate_dn4 = assign61420_e98827_d_n4;
        locals.var_ggate_dn5 = assign61420_e98827_d_n5;
        locals.var_ggate_dn6 = assign61420_e98827_d_n6;
        locals.var_ggate_dn7 = assign61420_e98827_d_n7;
        locals.var_ggate_dn8 = assign61420_e98827_d_n8;
        locals.var_ggate_dn9 = assign61420_e98827_d_n9;
        locals.var_ggate_dn10 = assign61420_e98827_d_n10;
        locals.var_ggate_dn11 = assign61420_e98827_d_n11;

        let assign61440_e98842: f64 = if ((p.p33 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard888 = assign61440_e98842;

        let (assign61450_e98848, assign61450_e98848_d_n3, assign61450_e98848_d_n4, assign61450_e98848_d_n5, assign61450_e98848_d_n6, assign61450_e98848_d_n7, assign61450_e98848_d_n8, assign61450_e98848_d_n9, assign61450_e98848_d_n10, assign61450_e98848_d_n11,) = {
    if (locals.var_guard888 != 0.0) {
        let assign61450_e98846: f64 = (1.0 / locals.var_rdrain);
        (assign61450_e98846, (-(locals.var_rdrain_dn3 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn4 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn5 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn6 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn7 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn8 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn9 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn10 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn11 / (locals.var_rdrain * locals.var_rdrain))),)
    } else {
        (locals.var_gdpr, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11,)
    }
};
        locals.var_gdpr = assign61450_e98848;
        locals.var_gdpr_dn3 = assign61450_e98848_d_n3;
        locals.var_gdpr_dn4 = assign61450_e98848_d_n4;
        locals.var_gdpr_dn5 = assign61450_e98848_d_n5;
        locals.var_gdpr_dn6 = assign61450_e98848_d_n6;
        locals.var_gdpr_dn7 = assign61450_e98848_d_n7;
        locals.var_gdpr_dn8 = assign61450_e98848_d_n8;
        locals.var_gdpr_dn9 = assign61450_e98848_d_n9;
        locals.var_gdpr_dn10 = assign61450_e98848_d_n10;
        locals.var_gdpr_dn11 = assign61450_e98848_d_n11;

        let assign61470_e98862: f64 = if ((p.p33 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard890 = assign61470_e98862;

        let (assign61480_e98868, assign61480_e98868_d_n3, assign61480_e98868_d_n4, assign61480_e98868_d_n5, assign61480_e98868_d_n6, assign61480_e98868_d_n7, assign61480_e98868_d_n8, assign61480_e98868_d_n9, assign61480_e98868_d_n10, assign61480_e98868_d_n11,) = {
    if (locals.var_guard890 != 0.0) {
        let assign61480_e98866: f64 = (1.0 / locals.var_rsource);
        (assign61480_e98866, (-(locals.var_rsource_dn3 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn4 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn5 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn6 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn7 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn8 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn9 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn10 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn11 / (locals.var_rsource * locals.var_rsource))),)
    } else {
        (locals.var_gspr, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11,)
    }
};
        locals.var_gspr = assign61480_e98868;
        locals.var_gspr_dn3 = assign61480_e98868_d_n3;
        locals.var_gspr_dn4 = assign61480_e98868_d_n4;
        locals.var_gspr_dn5 = assign61480_e98868_d_n5;
        locals.var_gspr_dn6 = assign61480_e98868_d_n6;
        locals.var_gspr_dn7 = assign61480_e98868_d_n7;
        locals.var_gspr_dn8 = assign61480_e98868_d_n8;
        locals.var_gspr_dn9 = assign61480_e98868_d_n9;
        locals.var_gspr_dn10 = assign61480_e98868_d_n10;
        locals.var_gspr_dn11 = assign61480_e98868_d_n11;

        let assign61500_e98878: f64 = if p.p7 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard892 = assign61500_e98878;

        let assign61510_e98885: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard893 = assign61510_e98885;

        let (assign61520_e98895, assign61520_e98895_d_n0, assign61520_e98895_d_n2, assign61520_e98895_d_n3, assign61520_e98895_d_n4, assign61520_e98895_d_n5, assign61520_e98895_d_n6, assign61520_e98895_d_n7, assign61520_e98895_d_n8, assign61520_e98895_d_n9, assign61520_e98895_d_n10, assign61520_e98895_d_n11,) = {
    if (locals.var_guard893 != 0.0) {
        let assign61520_e98889: f64 = (locals.var_devsign * locals.var_sigvds);
        let assign61520_e98891: f64 = (assign61520_e98889 * locals.var_ids);
        let assign61520_e98893: f64 = (assign61520_e98891 * (nv6 - nv7));
        (assign61520_e98893, 0.0, 0.0, ((assign61520_e98889 * locals.var_ids_dn3) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn4) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn5) * (nv6 - nv7)), (((assign61520_e98889 * locals.var_ids_dn6) * (nv6 - nv7)) + assign61520_e98891), (((assign61520_e98889 * locals.var_ids_dn7) * (nv6 - nv7)) + (-assign61520_e98891)), ((assign61520_e98889 * locals.var_ids_dn8) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn9) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn10) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn11) * (nv6 - nv7)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61520_e98895;
        locals.var_pdiss_dn0 = assign61520_e98895_d_n0;
        locals.var_pdiss_dn2 = assign61520_e98895_d_n2;
        locals.var_pdiss_dn3 = assign61520_e98895_d_n3;
        locals.var_pdiss_dn4 = assign61520_e98895_d_n4;
        locals.var_pdiss_dn5 = assign61520_e98895_d_n5;
        locals.var_pdiss_dn6 = assign61520_e98895_d_n6;
        locals.var_pdiss_dn7 = assign61520_e98895_d_n7;
        locals.var_pdiss_dn8 = assign61520_e98895_d_n8;
        locals.var_pdiss_dn9 = assign61520_e98895_d_n9;
        locals.var_pdiss_dn10 = assign61520_e98895_d_n10;
        locals.var_pdiss_dn11 = assign61520_e98895_d_n11;

        let assign61530_e98902: f64 = if ((p.p33 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard894 = assign61530_e98902;

        let (assign61540_e98914, assign61540_e98914_d_n0, assign61540_e98914_d_n2, assign61540_e98914_d_n3, assign61540_e98914_d_n4, assign61540_e98914_d_n5, assign61540_e98914_d_n6, assign61540_e98914_d_n7, assign61540_e98914_d_n8, assign61540_e98914_d_n9, assign61540_e98914_d_n10, assign61540_e98914_d_n11,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard894 != 0.0)) {
        let assign61540_e98909: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign61540_e98911: f64 = (assign61540_e98909 / locals.var_rdrain);
        let assign61540_e98912: f64 = (locals.var_pdiss + assign61540_e98911);
        (assign61540_e98912, (locals.var_pdiss_dn0 + (((nv0 - nv6) + (nv0 - nv6)) / locals.var_rdrain)), locals.var_pdiss_dn2, (locals.var_pdiss_dn3 + (-((assign61540_e98909 * locals.var_rdrain_dn3) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn4 + (-((assign61540_e98909 * locals.var_rdrain_dn4) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn5 + (-((assign61540_e98909 * locals.var_rdrain_dn5) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn6 + (((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_rdrain) - (assign61540_e98909 * locals.var_rdrain_dn6)) / (locals.var_rdrain * locals.var_rdrain))), (locals.var_pdiss_dn7 + (-((assign61540_e98909 * locals.var_rdrain_dn7) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn8 + (-((assign61540_e98909 * locals.var_rdrain_dn8) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn9 + (-((assign61540_e98909 * locals.var_rdrain_dn9) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn10 + (-((assign61540_e98909 * locals.var_rdrain_dn10) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn11 + (-((assign61540_e98909 * locals.var_rdrain_dn11) / (locals.var_rdrain * locals.var_rdrain)))),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61540_e98914;
        locals.var_pdiss_dn0 = assign61540_e98914_d_n0;
        locals.var_pdiss_dn2 = assign61540_e98914_d_n2;
        locals.var_pdiss_dn3 = assign61540_e98914_d_n3;
        locals.var_pdiss_dn4 = assign61540_e98914_d_n4;
        locals.var_pdiss_dn5 = assign61540_e98914_d_n5;
        locals.var_pdiss_dn6 = assign61540_e98914_d_n6;
        locals.var_pdiss_dn7 = assign61540_e98914_d_n7;
        locals.var_pdiss_dn8 = assign61540_e98914_d_n8;
        locals.var_pdiss_dn9 = assign61540_e98914_d_n9;
        locals.var_pdiss_dn10 = assign61540_e98914_d_n10;
        locals.var_pdiss_dn11 = assign61540_e98914_d_n11;

        let assign61550_e98921: f64 = if ((p.p33 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard895 = assign61550_e98921;

        let (assign61560_e98933, assign61560_e98933_d_n0, assign61560_e98933_d_n2, assign61560_e98933_d_n3, assign61560_e98933_d_n4, assign61560_e98933_d_n5, assign61560_e98933_d_n6, assign61560_e98933_d_n7, assign61560_e98933_d_n8, assign61560_e98933_d_n9, assign61560_e98933_d_n10, assign61560_e98933_d_n11,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard895 != 0.0)) {
        let assign61560_e98928: f64 = ((nv2 - nv7) * (nv2 - nv7));
        let assign61560_e98930: f64 = (assign61560_e98928 / locals.var_rsource);
        let assign61560_e98931: f64 = (locals.var_pdiss + assign61560_e98930);
        (assign61560_e98931, locals.var_pdiss_dn0, (locals.var_pdiss_dn2 + (((nv2 - nv7) + (nv2 - nv7)) / locals.var_rsource)), (locals.var_pdiss_dn3 + (-((assign61560_e98928 * locals.var_rsource_dn3) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn4 + (-((assign61560_e98928 * locals.var_rsource_dn4) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn5 + (-((assign61560_e98928 * locals.var_rsource_dn5) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn6 + (-((assign61560_e98928 * locals.var_rsource_dn6) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn7 + (((((-(nv2 - nv7)) + (-(nv2 - nv7))) * locals.var_rsource) - (assign61560_e98928 * locals.var_rsource_dn7)) / (locals.var_rsource * locals.var_rsource))), (locals.var_pdiss_dn8 + (-((assign61560_e98928 * locals.var_rsource_dn8) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn9 + (-((assign61560_e98928 * locals.var_rsource_dn9) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn10 + (-((assign61560_e98928 * locals.var_rsource_dn10) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn11 + (-((assign61560_e98928 * locals.var_rsource_dn11) / (locals.var_rsource * locals.var_rsource)))),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61560_e98933;
        locals.var_pdiss_dn0 = assign61560_e98933_d_n0;
        locals.var_pdiss_dn2 = assign61560_e98933_d_n2;
        locals.var_pdiss_dn3 = assign61560_e98933_d_n3;
        locals.var_pdiss_dn4 = assign61560_e98933_d_n4;
        locals.var_pdiss_dn5 = assign61560_e98933_d_n5;
        locals.var_pdiss_dn6 = assign61560_e98933_d_n6;
        locals.var_pdiss_dn7 = assign61560_e98933_d_n7;
        locals.var_pdiss_dn8 = assign61560_e98933_d_n8;
        locals.var_pdiss_dn9 = assign61560_e98933_d_n9;
        locals.var_pdiss_dn10 = assign61560_e98933_d_n10;
        locals.var_pdiss_dn11 = assign61560_e98933_d_n11;

        let assign61570_e98938: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard896 = assign61570_e98938;

        let assign61580_e98940: f64 = 1.0;
        locals.var_guard897 = assign61580_e98940;

        let assign61610_e98950: f64 = (p.p1359 * p.p1358);
        locals.var_rbodyext = assign61610_e98950;

        let assign61620_e98958: f64 = if ((p.p43 == 0.0) || (!true)) { 1.0 } else { 0.0 };
        locals.var_guard900 = assign61620_e98958;

        let assign61630_e98963: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard901 = assign61630_e98963;

        let assign61640_e98966: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard902 = assign61640_e98966;

        let (assign61650_e98994, assign61650_e98994_d_n3, assign61650_e98994_d_n4, assign61650_e98994_d_n5, assign61650_e98994_d_n6, assign61650_e98994_d_n7, assign61650_e98994_d_n8, assign61650_e98994_d_n9, assign61650_e98994_d_n10, assign61650_e98994_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) {
        let assign61650_e98976: f64 = (p.p1357 * p.p1356);
        let assign61650_e98978: f64 = (assign61650_e98976 * p.p1360);
        let assign61650_e98981: f64 = (2.0 * p.p1356);
        let assign61650_e98984: f64 = (p.p1360 * locals.var_leff_1);
        let assign61650_e98985: f64 = (assign61650_e98981 + assign61650_e98984);
        let assign61650_e98986: f64 = (assign61650_e98978 / assign61650_e98985);
        let assign61650_e98988: f64 = (assign61650_e98986 * locals.var_weff_1);
        let assign61650_e98990: f64 = (assign61650_e98988 / p.p1373);
        let assign61650_e98992: f64 = (assign61650_e98990 / p.p2);
        (assign61650_e98992, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbodyint, locals.var_rbodyint_dn3, locals.var_rbodyint_dn4, locals.var_rbodyint_dn5, locals.var_rbodyint_dn6, locals.var_rbodyint_dn7, locals.var_rbodyint_dn8, locals.var_rbodyint_dn9, locals.var_rbodyint_dn10, locals.var_rbodyint_dn11,)
    }
};
        locals.var_rbodyint = assign61650_e98994;
        locals.var_rbodyint_dn3 = assign61650_e98994_d_n3;
        locals.var_rbodyint_dn4 = assign61650_e98994_d_n4;
        locals.var_rbodyint_dn5 = assign61650_e98994_d_n5;
        locals.var_rbodyint_dn6 = assign61650_e98994_d_n6;
        locals.var_rbodyint_dn7 = assign61650_e98994_d_n7;
        locals.var_rbodyint_dn8 = assign61650_e98994_d_n8;
        locals.var_rbodyint_dn9 = assign61650_e98994_d_n9;
        locals.var_rbodyint_dn10 = assign61650_e98994_d_n10;
        locals.var_rbodyint_dn11 = assign61650_e98994_d_n11;

        let assign61660_e98997: f64 = if locals.var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard903 = assign61660_e98997;

        let assign61670_e99000: f64 = if locals.var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard904 = assign61670_e99000;

        let (assign61680_e99016, assign61680_e99016_d_n3, assign61680_e99016_d_n4, assign61680_e99016_d_n5, assign61680_e99016_d_n6, assign61680_e99016_d_n7, assign61680_e99016_d_n8, assign61680_e99016_d_n9, assign61680_e99016_d_n10, assign61680_e99016_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 != 0.0)) && (locals.var_guard904 != 0.0)) {
        let assign61680_e99014: f64 = (1.0 / 0.001);
        (assign61680_e99014, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61680_e99016;
        locals.var_t0_dn3 = assign61680_e99016_d_n3;
        locals.var_t0_dn4 = assign61680_e99016_d_n4;
        locals.var_t0_dn5 = assign61680_e99016_d_n5;
        locals.var_t0_dn6 = assign61680_e99016_d_n6;
        locals.var_t0_dn7 = assign61680_e99016_d_n7;
        locals.var_t0_dn8 = assign61680_e99016_d_n8;
        locals.var_t0_dn9 = assign61680_e99016_d_n9;
        locals.var_t0_dn10 = assign61680_e99016_d_n10;
        locals.var_t0_dn11 = assign61680_e99016_d_n11;

        let (assign61690_e99033, assign61690_e99033_d_n3, assign61690_e99033_d_n4, assign61690_e99033_d_n5, assign61690_e99033_d_n6, assign61690_e99033_d_n7, assign61690_e99033_d_n8, assign61690_e99033_d_n9, assign61690_e99033_d_n10, assign61690_e99033_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 != 0.0)) && (locals.var_guard904 == 0.0)) {
        let assign61690_e99031: f64 = (1.0 / locals.var_rbodyext);
        (assign61690_e99031, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61690_e99033;
        locals.var_t0_dn3 = assign61690_e99033_d_n3;
        locals.var_t0_dn4 = assign61690_e99033_d_n4;
        locals.var_t0_dn5 = assign61690_e99033_d_n5;
        locals.var_t0_dn6 = assign61690_e99033_d_n6;
        locals.var_t0_dn7 = assign61690_e99033_d_n7;
        locals.var_t0_dn8 = assign61690_e99033_d_n8;
        locals.var_t0_dn9 = assign61690_e99033_d_n9;
        locals.var_t0_dn10 = assign61690_e99033_d_n10;
        locals.var_t0_dn11 = assign61690_e99033_d_n11;

        let (assign61700_e99045, assign61700_e99045_d_n3, assign61700_e99045_d_n4, assign61700_e99045_d_n5, assign61700_e99045_d_n6, assign61700_e99045_d_n7, assign61700_e99045_d_n8, assign61700_e99045_d_n9, assign61700_e99045_d_n10, assign61700_e99045_d_n11,) = {
    if ((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_gbody, locals.var_gbody_dn3, locals.var_gbody_dn4, locals.var_gbody_dn5, locals.var_gbody_dn6, locals.var_gbody_dn7, locals.var_gbody_dn8, locals.var_gbody_dn9, locals.var_gbody_dn10, locals.var_gbody_dn11,)
    }
};
        locals.var_gbody = assign61700_e99045;
        locals.var_gbody_dn3 = assign61700_e99045_d_n3;
        locals.var_gbody_dn4 = assign61700_e99045_d_n4;
        locals.var_gbody_dn5 = assign61700_e99045_d_n5;
        locals.var_gbody_dn6 = assign61700_e99045_d_n6;
        locals.var_gbody_dn7 = assign61700_e99045_d_n7;
        locals.var_gbody_dn8 = assign61700_e99045_d_n8;
        locals.var_gbody_dn9 = assign61700_e99045_d_n9;
        locals.var_gbody_dn10 = assign61700_e99045_d_n10;
        locals.var_gbody_dn11 = assign61700_e99045_d_n11;

        let (assign61710_e99062, assign61710_e99062_d_n3, assign61710_e99062_d_n4, assign61710_e99062_d_n5, assign61710_e99062_d_n6, assign61710_e99062_d_n7, assign61710_e99062_d_n8, assign61710_e99062_d_n9, assign61710_e99062_d_n10, assign61710_e99062_d_n11,) = {
    if ((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 == 0.0)) {
        let assign61710_e99059: f64 = (locals.var_rbodyint + locals.var_rbodyext);
        let assign61710_e99060: f64 = (1.0 / assign61710_e99059);
        (assign61710_e99060, (-(locals.var_rbodyint_dn3 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn4 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn5 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn6 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn7 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn8 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn9 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn10 / (assign61710_e99059 * assign61710_e99059))), (-(locals.var_rbodyint_dn11 / (assign61710_e99059 * assign61710_e99059))),)
    } else {
        (locals.var_gbody, locals.var_gbody_dn3, locals.var_gbody_dn4, locals.var_gbody_dn5, locals.var_gbody_dn6, locals.var_gbody_dn7, locals.var_gbody_dn8, locals.var_gbody_dn9, locals.var_gbody_dn10, locals.var_gbody_dn11,)
    }
};
        locals.var_gbody = assign61710_e99062;
        locals.var_gbody_dn3 = assign61710_e99062_d_n3;
        locals.var_gbody_dn4 = assign61710_e99062_d_n4;
        locals.var_gbody_dn5 = assign61710_e99062_d_n5;
        locals.var_gbody_dn6 = assign61710_e99062_d_n6;
        locals.var_gbody_dn7 = assign61710_e99062_d_n7;
        locals.var_gbody_dn8 = assign61710_e99062_d_n8;
        locals.var_gbody_dn9 = assign61710_e99062_d_n9;
        locals.var_gbody_dn10 = assign61710_e99062_d_n10;
        locals.var_gbody_dn11 = assign61710_e99062_d_n11;

        let (assign61720_e99077, assign61720_e99077_d_n4, assign61720_e99077_d_n5,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61720_e99074: f64 = (locals.var_tratio).powf(locals.var_ubte_i);
        let assign61720_e99075: f64 = (locals.var_ub_i * assign61720_e99074);
        (assign61720_e99075, (locals.var_ub_i * if 0.0 == 0.0 && ((locals.var_ubte_i) as f64).is_finite() && ((locals.var_ubte_i) as f64).fract() == 0.0 { if locals.var_ubte_i == 0.0 { 0.0 } else { (locals.var_ubte_i * ((locals.var_tratio).powf(locals.var_ubte_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign61720_e99074 * (locals.var_ubte_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), (locals.var_ub_i * if 0.0 == 0.0 && ((locals.var_ubte_i) as f64).is_finite() && ((locals.var_ubte_i) as f64).fract() == 0.0 { if locals.var_ubte_i == 0.0 { 0.0 } else { (locals.var_ubte_i * ((locals.var_tratio).powf(locals.var_ubte_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign61720_e99074 * (locals.var_ubte_i * (locals.var_tratio_dn5 / locals.var_tratio))) }),)
    } else {
        (locals.var_ub_t, locals.var_ub_t_dn4, locals.var_ub_t_dn5,)
    }
};
        locals.var_ub_t = assign61720_e99077;
        locals.var_ub_t_dn4 = assign61720_e99077_d_n4;
        locals.var_ub_t_dn5 = assign61720_e99077_d_n5;

        let (assign61730_e99095, assign61730_e99095_d_n3, assign61730_e99095_d_n4, assign61730_e99095_d_n5, assign61730_e99095_d_n6, assign61730_e99095_d_n7, assign61730_e99095_d_n8, assign61730_e99095_d_n9, assign61730_e99095_d_n10, assign61730_e99095_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61730_e99088: f64 = (locals.var_qbi + locals.var_qbsj);
        let assign61730_e99090: f64 = (assign61730_e99088 + locals.var_qbdj);
        let assign61730_e99091: f64 = (-assign61730_e99090);
        let assign61730_e99093: f64 = (assign61730_e99091 + locals.var_qsub);
        (assign61730_e99093, ((-((locals.var_qbi_dn3 + locals.var_qbsj_dn3) + locals.var_qbdj_dn3)) + locals.var_qsub_dn3), ((-((locals.var_qbi_dn4 + locals.var_qbsj_dn4) + locals.var_qbdj_dn4)) + locals.var_qsub_dn4), ((-((locals.var_qbi_dn5 + locals.var_qbsj_dn5) + locals.var_qbdj_dn5)) + locals.var_qsub_dn5), ((-((locals.var_qbi_dn6 + locals.var_qbsj_dn6) + locals.var_qbdj_dn6)) + locals.var_qsub_dn6), ((-((locals.var_qbi_dn7 + locals.var_qbsj_dn7) + locals.var_qbdj_dn7)) + locals.var_qsub_dn7), ((-((locals.var_qbi_dn8 + locals.var_qbsj_dn8) + locals.var_qbdj_dn8)) + locals.var_qsub_dn8), ((-((locals.var_qbi_dn9 + locals.var_qbsj_dn9) + locals.var_qbdj_dn9)) + locals.var_qsub_dn9), ((-((locals.var_qbi_dn10 + locals.var_qbsj_dn10) + locals.var_qbdj_dn10)) + locals.var_qsub_dn10), ((-((locals.var_qbi_dn11 + locals.var_qbsj_dn11) + locals.var_qbdj_dn11)) + locals.var_qsub_dn11),)
    } else {
        (locals.var_qb1, locals.var_qb1_dn3, locals.var_qb1_dn4, locals.var_qb1_dn5, locals.var_qb1_dn6, locals.var_qb1_dn7, locals.var_qb1_dn8, locals.var_qb1_dn9, locals.var_qb1_dn10, locals.var_qb1_dn11,)
    }
};
        locals.var_qb1 = assign61730_e99095;
        locals.var_qb1_dn3 = assign61730_e99095_d_n3;
        locals.var_qb1_dn4 = assign61730_e99095_d_n4;
        locals.var_qb1_dn5 = assign61730_e99095_d_n5;
        locals.var_qb1_dn6 = assign61730_e99095_d_n6;
        locals.var_qb1_dn7 = assign61730_e99095_d_n7;
        locals.var_qb1_dn8 = assign61730_e99095_d_n8;
        locals.var_qb1_dn9 = assign61730_e99095_d_n9;
        locals.var_qb1_dn10 = assign61730_e99095_d_n10;
        locals.var_qb1_dn11 = assign61730_e99095_d_n11;

        let (assign61740_e99116, assign61740_e99116_d_n3, assign61740_e99116_d_n4, assign61740_e99116_d_n5, assign61740_e99116_d_n6, assign61740_e99116_d_n7, assign61740_e99116_d_n8, assign61740_e99116_d_n9, assign61740_e99116_d_n10, assign61740_e99116_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61740_e99106: f64 = (1.602176462e-19 * locals.var_neff_i);
        let assign61740_e99108: f64 = (assign61740_e99106 * p.p74);
        let assign61740_e99110: f64 = (assign61740_e99108 * locals.var_weff_1);
        let assign61740_e99112: f64 = (assign61740_e99110 * locals.var_leff_1);
        let assign61740_e99114: f64 = (assign61740_e99112 - locals.var_qb1);
        (assign61740_e99114, (-locals.var_qb1_dn3), (-locals.var_qb1_dn4), (-locals.var_qb1_dn5), (-locals.var_qb1_dn6), (-locals.var_qb1_dn7), (-locals.var_qb1_dn8), (-locals.var_qb1_dn9), (-locals.var_qb1_dn10), (-locals.var_qb1_dn11),)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11,)
    }
};
        locals.var_qbody = assign61740_e99116;
        locals.var_qbody_dn3 = assign61740_e99116_d_n3;
        locals.var_qbody_dn4 = assign61740_e99116_d_n4;
        locals.var_qbody_dn5 = assign61740_e99116_d_n5;
        locals.var_qbody_dn6 = assign61740_e99116_d_n6;
        locals.var_qbody_dn7 = assign61740_e99116_d_n7;
        locals.var_qbody_dn8 = assign61740_e99116_d_n8;
        locals.var_qbody_dn9 = assign61740_e99116_d_n9;
        locals.var_qbody_dn10 = assign61740_e99116_d_n10;
        locals.var_qbody_dn11 = assign61740_e99116_d_n11;

        let (assign61750_e99129, assign61750_e99129_d_n3, assign61750_e99129_d_n4, assign61750_e99129_d_n5, assign61750_e99129_d_n6, assign61750_e99129_d_n7, assign61750_e99129_d_n8, assign61750_e99129_d_n9, assign61750_e99129_d_n10, assign61750_e99129_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61750_e99127: f64 = (locals.var_ub_t * locals.var_qbody);
        (assign61750_e99127, (locals.var_ub_t * locals.var_qbody_dn3), ((locals.var_ub_t_dn4 * locals.var_qbody) + (locals.var_ub_t * locals.var_qbody_dn4)), ((locals.var_ub_t_dn5 * locals.var_qbody) + (locals.var_ub_t * locals.var_qbody_dn5)), (locals.var_ub_t * locals.var_qbody_dn6), (locals.var_ub_t * locals.var_qbody_dn7), (locals.var_ub_t * locals.var_qbody_dn8), (locals.var_ub_t * locals.var_qbody_dn9), (locals.var_ub_t * locals.var_qbody_dn10), (locals.var_ub_t * locals.var_qbody_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61750_e99129;
        locals.var_t0_dn3 = assign61750_e99129_d_n3;
        locals.var_t0_dn4 = assign61750_e99129_d_n4;
        locals.var_t0_dn5 = assign61750_e99129_d_n5;
        locals.var_t0_dn6 = assign61750_e99129_d_n6;
        locals.var_t0_dn7 = assign61750_e99129_d_n7;
        locals.var_t0_dn8 = assign61750_e99129_d_n8;
        locals.var_t0_dn9 = assign61750_e99129_d_n9;
        locals.var_t0_dn10 = assign61750_e99129_d_n10;
        locals.var_t0_dn11 = assign61750_e99129_d_n11;

        let (assign61760_e99142, assign61760_e99142_d_n3, assign61760_e99142_d_n4, assign61760_e99142_d_n5, assign61760_e99142_d_n6, assign61760_e99142_d_n7, assign61760_e99142_d_n8, assign61760_e99142_d_n9, assign61760_e99142_d_n10, assign61760_e99142_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61760_e99140: f64 = (locals.var_weff_1 * locals.var_weff_1);
        (assign61760_e99140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign61760_e99142;
        locals.var_t1_dn3 = assign61760_e99142_d_n3;
        locals.var_t1_dn4 = assign61760_e99142_d_n4;
        locals.var_t1_dn5 = assign61760_e99142_d_n5;
        locals.var_t1_dn6 = assign61760_e99142_d_n6;
        locals.var_t1_dn7 = assign61760_e99142_d_n7;
        locals.var_t1_dn8 = assign61760_e99142_d_n8;
        locals.var_t1_dn9 = assign61760_e99142_d_n9;
        locals.var_t1_dn10 = assign61760_e99142_d_n10;
        locals.var_t1_dn11 = assign61760_e99142_d_n11;

        let (assign61770_e99157, assign61770_e99157_d_n3, assign61770_e99157_d_n4, assign61770_e99157_d_n5, assign61770_e99157_d_n6, assign61770_e99157_d_n7, assign61770_e99157_d_n8, assign61770_e99157_d_n9, assign61770_e99157_d_n10, assign61770_e99157_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61770_e99153: f64 = (p.p2 * locals.var_t0);
        let assign61770_e99155: f64 = (assign61770_e99153 / locals.var_t1);
        (assign61770_e99155, ((((p.p2 * locals.var_t0_dn3) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn4) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn5) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn6) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn7) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn8) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn9) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn10) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn11) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gbodyint, locals.var_gbodyint_dn3, locals.var_gbodyint_dn4, locals.var_gbodyint_dn5, locals.var_gbodyint_dn6, locals.var_gbodyint_dn7, locals.var_gbodyint_dn8, locals.var_gbodyint_dn9, locals.var_gbodyint_dn10, locals.var_gbodyint_dn11,)
    }
};
        locals.var_gbodyint = assign61770_e99157;
        locals.var_gbodyint_dn3 = assign61770_e99157_d_n3;
        locals.var_gbodyint_dn4 = assign61770_e99157_d_n4;
        locals.var_gbodyint_dn5 = assign61770_e99157_d_n5;
        locals.var_gbodyint_dn6 = assign61770_e99157_d_n6;
        locals.var_gbodyint_dn7 = assign61770_e99157_d_n7;
        locals.var_gbodyint_dn8 = assign61770_e99157_d_n8;
        locals.var_gbodyint_dn9 = assign61770_e99157_d_n9;
        locals.var_gbodyint_dn10 = assign61770_e99157_d_n10;
        locals.var_gbodyint_dn11 = assign61770_e99157_d_n11;

        let (assign61780_e99170, assign61780_e99170_d_n3, assign61780_e99170_d_n4, assign61780_e99170_d_n5, assign61780_e99170_d_n6, assign61780_e99170_d_n7, assign61780_e99170_d_n8, assign61780_e99170_d_n9, assign61780_e99170_d_n10, assign61780_e99170_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61780_e99168: f64 = (1.0 / locals.var_gbodyint);
        (assign61780_e99168, (-(locals.var_gbodyint_dn3 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn4 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn5 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn6 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn7 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn8 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn9 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn10 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn11 / (locals.var_gbodyint * locals.var_gbodyint))),)
    } else {
        (locals.var_rbodyint, locals.var_rbodyint_dn3, locals.var_rbodyint_dn4, locals.var_rbodyint_dn5, locals.var_rbodyint_dn6, locals.var_rbodyint_dn7, locals.var_rbodyint_dn8, locals.var_rbodyint_dn9, locals.var_rbodyint_dn10, locals.var_rbodyint_dn11,)
    }
};
        locals.var_rbodyint = assign61780_e99170;
        locals.var_rbodyint_dn3 = assign61780_e99170_d_n3;
        locals.var_rbodyint_dn4 = assign61780_e99170_d_n4;
        locals.var_rbodyint_dn5 = assign61780_e99170_d_n5;
        locals.var_rbodyint_dn6 = assign61780_e99170_d_n6;
        locals.var_rbodyint_dn7 = assign61780_e99170_d_n7;
        locals.var_rbodyint_dn8 = assign61780_e99170_d_n8;
        locals.var_rbodyint_dn9 = assign61780_e99170_d_n9;
        locals.var_rbodyint_dn10 = assign61780_e99170_d_n10;
        locals.var_rbodyint_dn11 = assign61780_e99170_d_n11;

        let assign61790_e99173: f64 = if locals.var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard905 = assign61790_e99173;

        let assign61800_e99176: f64 = if locals.var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard906 = assign61800_e99176;

    }

    pub(super) fn stamp_transient_block_210(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign61810_e99193, assign61810_e99193_d_n3, assign61810_e99193_d_n4, assign61810_e99193_d_n5, assign61810_e99193_d_n6, assign61810_e99193_d_n7, assign61810_e99193_d_n8, assign61810_e99193_d_n9, assign61810_e99193_d_n10, assign61810_e99193_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 != 0.0)) && (locals.var_guard906 != 0.0)) {
        let assign61810_e99191: f64 = (1.0 / 0.001);
        (assign61810_e99191, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61810_e99193;
        locals.var_t0_dn3 = assign61810_e99193_d_n3;
        locals.var_t0_dn4 = assign61810_e99193_d_n4;
        locals.var_t0_dn5 = assign61810_e99193_d_n5;
        locals.var_t0_dn6 = assign61810_e99193_d_n6;
        locals.var_t0_dn7 = assign61810_e99193_d_n7;
        locals.var_t0_dn8 = assign61810_e99193_d_n8;
        locals.var_t0_dn9 = assign61810_e99193_d_n9;
        locals.var_t0_dn10 = assign61810_e99193_d_n10;
        locals.var_t0_dn11 = assign61810_e99193_d_n11;

        let (assign61820_e99211, assign61820_e99211_d_n3, assign61820_e99211_d_n4, assign61820_e99211_d_n5, assign61820_e99211_d_n6, assign61820_e99211_d_n7, assign61820_e99211_d_n8, assign61820_e99211_d_n9, assign61820_e99211_d_n10, assign61820_e99211_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 != 0.0)) && (locals.var_guard906 == 0.0)) {
        let assign61820_e99209: f64 = (1.0 / locals.var_rbodyext);
        (assign61820_e99209, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61820_e99211;
        locals.var_t0_dn3 = assign61820_e99211_d_n3;
        locals.var_t0_dn4 = assign61820_e99211_d_n4;
        locals.var_t0_dn5 = assign61820_e99211_d_n5;
        locals.var_t0_dn6 = assign61820_e99211_d_n6;
        locals.var_t0_dn7 = assign61820_e99211_d_n7;
        locals.var_t0_dn8 = assign61820_e99211_d_n8;
        locals.var_t0_dn9 = assign61820_e99211_d_n9;
        locals.var_t0_dn10 = assign61820_e99211_d_n10;
        locals.var_t0_dn11 = assign61820_e99211_d_n11;

        let (assign61830_e99224, assign61830_e99224_d_n3, assign61830_e99224_d_n4, assign61830_e99224_d_n5, assign61830_e99224_d_n6, assign61830_e99224_d_n7, assign61830_e99224_d_n8, assign61830_e99224_d_n9, assign61830_e99224_d_n10, assign61830_e99224_d_n11,) = {
    if ((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_gbody, locals.var_gbody_dn3, locals.var_gbody_dn4, locals.var_gbody_dn5, locals.var_gbody_dn6, locals.var_gbody_dn7, locals.var_gbody_dn8, locals.var_gbody_dn9, locals.var_gbody_dn10, locals.var_gbody_dn11,)
    }
};
        locals.var_gbody = assign61830_e99224;
        locals.var_gbody_dn3 = assign61830_e99224_d_n3;
        locals.var_gbody_dn4 = assign61830_e99224_d_n4;
        locals.var_gbody_dn5 = assign61830_e99224_d_n5;
        locals.var_gbody_dn6 = assign61830_e99224_d_n6;
        locals.var_gbody_dn7 = assign61830_e99224_d_n7;
        locals.var_gbody_dn8 = assign61830_e99224_d_n8;
        locals.var_gbody_dn9 = assign61830_e99224_d_n9;
        locals.var_gbody_dn10 = assign61830_e99224_d_n10;
        locals.var_gbody_dn11 = assign61830_e99224_d_n11;

        let (assign61840_e99242, assign61840_e99242_d_n3, assign61840_e99242_d_n4, assign61840_e99242_d_n5, assign61840_e99242_d_n6, assign61840_e99242_d_n7, assign61840_e99242_d_n8, assign61840_e99242_d_n9, assign61840_e99242_d_n10, assign61840_e99242_d_n11,) = {
    if ((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 == 0.0)) {
        let assign61840_e99239: f64 = (locals.var_rbodyint + locals.var_rbodyext);
        let assign61840_e99240: f64 = (1.0 / assign61840_e99239);
        (assign61840_e99240, (-(locals.var_rbodyint_dn3 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn4 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn5 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn6 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn7 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn8 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn9 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn10 / (assign61840_e99239 * assign61840_e99239))), (-(locals.var_rbodyint_dn11 / (assign61840_e99239 * assign61840_e99239))),)
    } else {
        (locals.var_gbody, locals.var_gbody_dn3, locals.var_gbody_dn4, locals.var_gbody_dn5, locals.var_gbody_dn6, locals.var_gbody_dn7, locals.var_gbody_dn8, locals.var_gbody_dn9, locals.var_gbody_dn10, locals.var_gbody_dn11,)
    }
};
        locals.var_gbody = assign61840_e99242;
        locals.var_gbody_dn3 = assign61840_e99242_d_n3;
        locals.var_gbody_dn4 = assign61840_e99242_d_n4;
        locals.var_gbody_dn5 = assign61840_e99242_d_n5;
        locals.var_gbody_dn6 = assign61840_e99242_d_n6;
        locals.var_gbody_dn7 = assign61840_e99242_d_n7;
        locals.var_gbody_dn8 = assign61840_e99242_d_n8;
        locals.var_gbody_dn9 = assign61840_e99242_d_n9;
        locals.var_gbody_dn10 = assign61840_e99242_d_n10;
        locals.var_gbody_dn11 = assign61840_e99242_d_n11;

        let assign61850_e99245: f64 = if p.p1375 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard907 = assign61850_e99245;

        let (assign61860_e99251, assign61860_e99251_d_n3, assign61860_e99251_d_n4, assign61860_e99251_d_n5, assign61860_e99251_d_n6, assign61860_e99251_d_n7, assign61860_e99251_d_n8, assign61860_e99251_d_n9, assign61860_e99251_d_n10, assign61860_e99251_d_n11,) = {
    if (locals.var_guard907 != 0.0) {
        let assign61860_e99249: f64 = (2.0 * locals.var_gbody);
        (assign61860_e99249, (2.0 * locals.var_gbody_dn3), (2.0 * locals.var_gbody_dn4), (2.0 * locals.var_gbody_dn5), (2.0 * locals.var_gbody_dn6), (2.0 * locals.var_gbody_dn7), (2.0 * locals.var_gbody_dn8), (2.0 * locals.var_gbody_dn9), (2.0 * locals.var_gbody_dn10), (2.0 * locals.var_gbody_dn11),)
    } else {
        (locals.var_gbody, locals.var_gbody_dn3, locals.var_gbody_dn4, locals.var_gbody_dn5, locals.var_gbody_dn6, locals.var_gbody_dn7, locals.var_gbody_dn8, locals.var_gbody_dn9, locals.var_gbody_dn10, locals.var_gbody_dn11,)
    }
};
        locals.var_gbody = assign61860_e99251;
        locals.var_gbody_dn3 = assign61860_e99251_d_n3;
        locals.var_gbody_dn4 = assign61860_e99251_d_n4;
        locals.var_gbody_dn5 = assign61860_e99251_d_n5;
        locals.var_gbody_dn6 = assign61860_e99251_d_n6;
        locals.var_gbody_dn7 = assign61860_e99251_d_n7;
        locals.var_gbody_dn8 = assign61860_e99251_d_n8;
        locals.var_gbody_dn9 = assign61860_e99251_d_n9;
        locals.var_gbody_dn10 = assign61860_e99251_d_n10;
        locals.var_gbody_dn11 = assign61860_e99251_d_n11;

        let assign61870_e99254: f64 = if p.p1374 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard908 = assign61870_e99254;

        let (assign61880_e99260, assign61880_e99260_d_n3, assign61880_e99260_d_n4, assign61880_e99260_d_n5, assign61880_e99260_d_n6, assign61880_e99260_d_n7, assign61880_e99260_d_n8, assign61880_e99260_d_n9, assign61880_e99260_d_n10, assign61880_e99260_d_n11,) = {
    if (locals.var_guard908 != 0.0) {
        let assign61880_e99258: f64 = (1.0 / 0.001);
        (assign61880_e99258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61880_e99260;
        locals.var_t0_dn3 = assign61880_e99260_d_n3;
        locals.var_t0_dn4 = assign61880_e99260_d_n4;
        locals.var_t0_dn5 = assign61880_e99260_d_n5;
        locals.var_t0_dn6 = assign61880_e99260_d_n6;
        locals.var_t0_dn7 = assign61880_e99260_d_n7;
        locals.var_t0_dn8 = assign61880_e99260_d_n8;
        locals.var_t0_dn9 = assign61880_e99260_d_n9;
        locals.var_t0_dn10 = assign61880_e99260_d_n10;
        locals.var_t0_dn11 = assign61880_e99260_d_n11;

        let assign61910_e99273: f64 = 1.0;
        locals.var_guard909 = assign61910_e99273;

        let assign61920_e99279: f64 = if ((p.p40 == 0.0) || true) { 1.0 } else { 0.0 };
        locals.var_guard910 = assign61920_e99279;

    }

    pub(super) fn stamp_reactive_block_0(
        locals: &mut StampLocals,
    ) {
        locals.var_sp_vfb = 0.0;
        locals.var_sp_vfb_dn3 = 0.0;
        locals.var_sp_vfb_dn4 = 0.0;
        locals.var_sp_vfb_dn5 = 0.0;
        locals.var_sp_vfb_dn6 = 0.0;
        locals.var_sp_vfb_dn7 = 0.0;
        locals.var_sp_vfb_dn8 = 0.0;
        locals.var_sp_vfb_dn9 = 0.0;
        locals.var_sp_vfb_dn10 = 0.0;
        locals.var_sp_vfb_dn11 = 0.0;
        locals.var_sp_vfb_rv = 0.0;

        locals.var_sp_s_xbar = 0.0;
        locals.var_sp_s_xbar_dn3 = 0.0;
        locals.var_sp_s_xbar_dn4 = 0.0;
        locals.var_sp_s_xbar_dn5 = 0.0;
        locals.var_sp_s_xbar_dn6 = 0.0;
        locals.var_sp_s_xbar_dn7 = 0.0;
        locals.var_sp_s_xbar_dn8 = 0.0;
        locals.var_sp_s_xbar_dn9 = 0.0;
        locals.var_sp_s_xbar_dn10 = 0.0;
        locals.var_sp_s_xbar_dn11 = 0.0;
        locals.var_sp_s_xbar_rv = 0.0;

        locals.var_psip = 0.0;
        locals.var_psip_dn3 = 0.0;
        locals.var_psip_dn4 = 0.0;
        locals.var_psip_dn5 = 0.0;
        locals.var_psip_dn6 = 0.0;
        locals.var_psip_dn7 = 0.0;
        locals.var_psip_dn8 = 0.0;
        locals.var_psip_dn9 = 0.0;
        locals.var_psip_dn10 = 0.0;
        locals.var_psip_dn11 = 0.0;
        locals.var_psip_rv = 0.0;

        locals.var_inv_gam = 0.0;
        locals.var_inv_gam_dn3 = 0.0;
        locals.var_inv_gam_dn4 = 0.0;
        locals.var_inv_gam_dn5 = 0.0;
        locals.var_inv_gam_dn6 = 0.0;
        locals.var_inv_gam_dn7 = 0.0;
        locals.var_inv_gam_dn8 = 0.0;
        locals.var_inv_gam_dn9 = 0.0;
        locals.var_inv_gam_dn10 = 0.0;
        locals.var_inv_gam_dn11 = 0.0;
        locals.var_inv_gam_rv = 0.0;

        locals.var_vdssat = 0.0;
        locals.var_vdssat_dn3 = 0.0;
        locals.var_vdssat_dn4 = 0.0;
        locals.var_vdssat_dn5 = 0.0;
        locals.var_vdssat_dn6 = 0.0;
        locals.var_vdssat_dn7 = 0.0;
        locals.var_vdssat_dn8 = 0.0;
        locals.var_vdssat_dn9 = 0.0;
        locals.var_vdssat_dn10 = 0.0;
        locals.var_vdssat_dn11 = 0.0;
        locals.var_vdssat_rv = 0.0;

        locals.var_vfb2 = 0.0;
        locals.var_vfb2_dn3 = 0.0;
        locals.var_vfb2_dn4 = 0.0;
        locals.var_vfb2_dn5 = 0.0;
        locals.var_vfb2_dn6 = 0.0;
        locals.var_vfb2_dn7 = 0.0;
        locals.var_vfb2_dn8 = 0.0;
        locals.var_vfb2_dn9 = 0.0;
        locals.var_vfb2_dn10 = 0.0;
        locals.var_vfb2_dn11 = 0.0;
        locals.var_vfb2_rv = 0.0;

        locals.var_cdscdedger_i = 0.0;
        locals.var_cdscdedger_i_dn3 = 0.0;
        locals.var_cdscdedger_i_dn4 = 0.0;
        locals.var_cdscdedger_i_dn5 = 0.0;
        locals.var_cdscdedger_i_dn6 = 0.0;
        locals.var_cdscdedger_i_dn7 = 0.0;
        locals.var_cdscdedger_i_dn8 = 0.0;
        locals.var_cdscdedger_i_dn9 = 0.0;
        locals.var_cdscdedger_i_dn10 = 0.0;
        locals.var_cdscdedger_i_dn11 = 0.0;
        locals.var_cdscdedger_i_rv = 0.0;

        locals.var_cdscdr_i = 0.0;
        locals.var_cdscdr_i_dn3 = 0.0;
        locals.var_cdscdr_i_dn4 = 0.0;
        locals.var_cdscdr_i_dn5 = 0.0;
        locals.var_cdscdr_i_dn6 = 0.0;
        locals.var_cdscdr_i_dn7 = 0.0;
        locals.var_cdscdr_i_dn8 = 0.0;
        locals.var_cdscdr_i_dn9 = 0.0;
        locals.var_cdscdr_i_dn10 = 0.0;
        locals.var_cdscdr_i_dn11 = 0.0;
        locals.var_cdscdr_i_rv = 0.0;

        locals.var_l_wln1 = 0.0;
        locals.var_l_wln1_rv = 0.0;

        locals.var_ptwgr_i = 0.0;
        locals.var_ptwgr_i_dn3 = 0.0;
        locals.var_ptwgr_i_dn4 = 0.0;
        locals.var_ptwgr_i_dn5 = 0.0;
        locals.var_ptwgr_i_dn6 = 0.0;
        locals.var_ptwgr_i_dn7 = 0.0;
        locals.var_ptwgr_i_dn8 = 0.0;
        locals.var_ptwgr_i_dn9 = 0.0;
        locals.var_ptwgr_i_dn10 = 0.0;
        locals.var_ptwgr_i_dn11 = 0.0;
        locals.var_ptwgr_i_rv = 0.0;

        locals.var_uar_i = 0.0;
        locals.var_uar_i_dn3 = 0.0;
        locals.var_uar_i_dn4 = 0.0;
        locals.var_uar_i_dn5 = 0.0;
        locals.var_uar_i_dn6 = 0.0;
        locals.var_uar_i_dn7 = 0.0;
        locals.var_uar_i_dn8 = 0.0;
        locals.var_uar_i_dn9 = 0.0;
        locals.var_uar_i_dn10 = 0.0;
        locals.var_uar_i_dn11 = 0.0;
        locals.var_uar_i_rv = 0.0;

        locals.var_ucsr_i = 0.0;
        locals.var_ucsr_i_rv = 0.0;

        locals.var_ud_a = 0.0;
        locals.var_ud_a_dn3 = 0.0;
        locals.var_ud_a_dn4 = 0.0;
        locals.var_ud_a_dn5 = 0.0;
        locals.var_ud_a_dn6 = 0.0;
        locals.var_ud_a_dn7 = 0.0;
        locals.var_ud_a_dn8 = 0.0;
        locals.var_ud_a_dn9 = 0.0;
        locals.var_ud_a_dn10 = 0.0;
        locals.var_ud_a_dn11 = 0.0;
        locals.var_ud_a_rv = 0.0;

        locals.var_w_wwn1 = 0.0;
        locals.var_w_wwn1_rv = 0.0;

        locals.var_inv_sa = 0.0;
        locals.var_inv_sa_dn3 = 0.0;
        locals.var_inv_sa_dn4 = 0.0;
        locals.var_inv_sa_dn5 = 0.0;
        locals.var_inv_sa_dn6 = 0.0;
        locals.var_inv_sa_dn7 = 0.0;
        locals.var_inv_sa_dn8 = 0.0;
        locals.var_inv_sa_dn9 = 0.0;
        locals.var_inv_sa_dn10 = 0.0;
        locals.var_inv_sa_dn11 = 0.0;
        locals.var_inv_sa_rv = 0.0;

        locals.var_eta_stress = 0.0;
        locals.var_eta_stress_dn3 = 0.0;
        locals.var_eta_stress_dn4 = 0.0;
        locals.var_eta_stress_dn5 = 0.0;
        locals.var_eta_stress_dn6 = 0.0;
        locals.var_eta_stress_dn7 = 0.0;
        locals.var_eta_stress_dn8 = 0.0;
        locals.var_eta_stress_dn9 = 0.0;
        locals.var_eta_stress_dn10 = 0.0;
        locals.var_eta_stress_dn11 = 0.0;
        locals.var_eta_stress_rv = 0.0;

        locals.var_m0_i = 0.0;
        locals.var_m0_i_rv = 0.0;

        locals.var_m0_t = 0.0;
        locals.var_m0_t_dn4 = 0.0;
        locals.var_m0_t_dn5 = 0.0;
        locals.var_m0_t_rv = 0.0;

        locals.var_eta0edge_i = 0.0;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_rv = 0.0;

        locals.var_kt2edge_i = 0.0;
        locals.var_kt2edge_i_rv = 0.0;

        locals.var_k2edge_i = 0.0;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_rv = 0.0;

        locals.var_mnud1 = 0.0;
        locals.var_mnud1_dn3 = 0.0;
        locals.var_mnud1_dn4 = 0.0;
        locals.var_mnud1_dn5 = 0.0;
        locals.var_mnud1_dn6 = 0.0;
        locals.var_mnud1_dn7 = 0.0;
        locals.var_mnud1_dn8 = 0.0;
        locals.var_mnud1_dn9 = 0.0;
        locals.var_mnud1_dn10 = 0.0;
        locals.var_mnud1_dn11 = 0.0;
        locals.var_mnud1_rv = 0.0;

        locals.var_c0si_i = 0.0;
        locals.var_c0si_i_rv = 0.0;

        locals.var_c0sisat1_i = 0.0;
        locals.var_c0sisat1_i_rv = 0.0;

        locals.var_eta0r_i = 0.0;
        locals.var_eta0r_i_dn3 = 0.0;
        locals.var_eta0r_i_dn4 = 0.0;
        locals.var_eta0r_i_dn5 = 0.0;
        locals.var_eta0r_i_dn6 = 0.0;
        locals.var_eta0r_i_dn7 = 0.0;
        locals.var_eta0r_i_dn8 = 0.0;
        locals.var_eta0r_i_dn9 = 0.0;
        locals.var_eta0r_i_dn10 = 0.0;
        locals.var_eta0r_i_dn11 = 0.0;
        locals.var_eta0r_i_rv = 0.0;

        locals.var_pclmr_i = 0.0;
        locals.var_pclmr_i_dn3 = 0.0;
        locals.var_pclmr_i_dn4 = 0.0;
        locals.var_pclmr_i_dn5 = 0.0;
        locals.var_pclmr_i_dn6 = 0.0;
        locals.var_pclmr_i_dn7 = 0.0;
        locals.var_pclmr_i_dn8 = 0.0;
        locals.var_pclmr_i_dn9 = 0.0;
        locals.var_pclmr_i_dn10 = 0.0;
        locals.var_pclmr_i_dn11 = 0.0;
        locals.var_pclmr_i_rv = 0.0;

        locals.var_ptwgr_t = 0.0;
        locals.var_ptwgr_t_dn3 = 0.0;
        locals.var_ptwgr_t_dn4 = 0.0;
        locals.var_ptwgr_t_dn5 = 0.0;
        locals.var_ptwgr_t_dn6 = 0.0;
        locals.var_ptwgr_t_dn7 = 0.0;
        locals.var_ptwgr_t_dn8 = 0.0;
        locals.var_ptwgr_t_dn9 = 0.0;
        locals.var_ptwgr_t_dn10 = 0.0;
        locals.var_ptwgr_t_dn11 = 0.0;
        locals.var_ptwgr_t_rv = 0.0;

        locals.var_uar_t = 0.0;
        locals.var_uar_t_dn3 = 0.0;
        locals.var_uar_t_dn4 = 0.0;
        locals.var_uar_t_dn5 = 0.0;
        locals.var_uar_t_dn6 = 0.0;
        locals.var_uar_t_dn7 = 0.0;
        locals.var_uar_t_dn8 = 0.0;
        locals.var_uar_t_dn9 = 0.0;
        locals.var_uar_t_dn10 = 0.0;
        locals.var_uar_t_dn11 = 0.0;
        locals.var_uar_t_rv = 0.0;

        locals.var_ucsr_t = 0.0;
        locals.var_ucsr_t_dn4 = 0.0;
        locals.var_ucsr_t_dn5 = 0.0;
        locals.var_ucsr_t_rv = 0.0;

        locals.var_vsatr_i = 0.0;
        locals.var_vsatr_i_dn3 = 0.0;
        locals.var_vsatr_i_dn4 = 0.0;
        locals.var_vsatr_i_dn5 = 0.0;
        locals.var_vsatr_i_dn6 = 0.0;
        locals.var_vsatr_i_dn7 = 0.0;
        locals.var_vsatr_i_dn8 = 0.0;
        locals.var_vsatr_i_dn9 = 0.0;
        locals.var_vsatr_i_dn10 = 0.0;
        locals.var_vsatr_i_dn11 = 0.0;
        locals.var_vsatr_i_rv = 0.0;

        locals.var_local_sca = 0.0;
        locals.var_local_sca_dn3 = 0.0;
        locals.var_local_sca_dn4 = 0.0;
        locals.var_local_sca_dn5 = 0.0;
        locals.var_local_sca_dn6 = 0.0;
        locals.var_local_sca_dn7 = 0.0;
        locals.var_local_sca_dn8 = 0.0;
        locals.var_local_sca_dn9 = 0.0;
        locals.var_local_sca_dn10 = 0.0;
        locals.var_local_sca_dn11 = 0.0;
        locals.var_local_sca_rv = 0.0;

        locals.var_inv_sb = 0.0;
        locals.var_inv_sb_dn3 = 0.0;
        locals.var_inv_sb_dn4 = 0.0;
        locals.var_inv_sb_dn5 = 0.0;
        locals.var_inv_sb_dn6 = 0.0;
        locals.var_inv_sb_dn7 = 0.0;
        locals.var_inv_sb_dn8 = 0.0;
        locals.var_inv_sb_dn9 = 0.0;
        locals.var_inv_sb_dn10 = 0.0;
        locals.var_inv_sb_dn11 = 0.0;
        locals.var_inv_sb_rv = 0.0;

        locals.var_k01_i = 0.0;
        locals.var_k01_i_rv = 0.0;

        locals.var_citedge_i = 0.0;
        locals.var_citedge_i_rv = 0.0;

        locals.var_etabedge_i = 0.0;
        locals.var_etabedge_i_rv = 0.0;

        locals.var_kt1expedge_i = 0.0;
        locals.var_kt1expedge_i_rv = 0.0;

        locals.var_kvth0edge_i = 0.0;
        locals.var_kvth0edge_i_rv = 0.0;

        locals.var_c0_i = 0.0;
        locals.var_c0_i_rv = 0.0;

        locals.var_c0si1_i = 0.0;
        locals.var_c0si1_i_rv = 0.0;

        locals.var_c0sisat_t = 0.0;
        locals.var_c0sisat_t_dn4 = 0.0;
        locals.var_c0sisat_t_dn5 = 0.0;
        locals.var_c0sisat_t_rv = 0.0;

        locals.var_eta0r_t = 0.0;
        locals.var_eta0r_t_dn3 = 0.0;
        locals.var_eta0r_t_dn4 = 0.0;
        locals.var_eta0r_t_dn5 = 0.0;
        locals.var_eta0r_t_dn6 = 0.0;
        locals.var_eta0r_t_dn7 = 0.0;
        locals.var_eta0r_t_dn8 = 0.0;
        locals.var_eta0r_t_dn9 = 0.0;
        locals.var_eta0r_t_dn10 = 0.0;
        locals.var_eta0r_t_dn11 = 0.0;
        locals.var_eta0r_t_rv = 0.0;

        locals.var_pdiblcr_i = 0.0;
        locals.var_pdiblcr_i_dn3 = 0.0;
        locals.var_pdiblcr_i_dn4 = 0.0;
        locals.var_pdiblcr_i_dn5 = 0.0;
        locals.var_pdiblcr_i_dn6 = 0.0;
        locals.var_pdiblcr_i_dn7 = 0.0;
        locals.var_pdiblcr_i_dn8 = 0.0;
        locals.var_pdiblcr_i_dn9 = 0.0;
        locals.var_pdiblcr_i_dn10 = 0.0;
        locals.var_pdiblcr_i_dn11 = 0.0;
        locals.var_pdiblcr_i_rv = 0.0;

        locals.var_u0r_i = 0.0;
        locals.var_u0r_i_rv = 0.0;

        locals.var_ucr_i = 0.0;
        locals.var_ucr_i_dn3 = 0.0;
        locals.var_ucr_i_dn4 = 0.0;
        locals.var_ucr_i_dn5 = 0.0;
        locals.var_ucr_i_dn6 = 0.0;
        locals.var_ucr_i_dn7 = 0.0;
        locals.var_ucr_i_dn8 = 0.0;
        locals.var_ucr_i_dn9 = 0.0;
        locals.var_ucr_i_dn10 = 0.0;
        locals.var_ucr_i_dn11 = 0.0;
        locals.var_ucr_i_rv = 0.0;

        locals.var_udr_i = 0.0;
        locals.var_udr_i_dn3 = 0.0;
        locals.var_udr_i_dn4 = 0.0;
        locals.var_udr_i_dn5 = 0.0;
        locals.var_udr_i_dn6 = 0.0;
        locals.var_udr_i_dn7 = 0.0;
        locals.var_udr_i_dn8 = 0.0;
        locals.var_udr_i_dn9 = 0.0;
        locals.var_udr_i_dn10 = 0.0;
        locals.var_udr_i_dn11 = 0.0;
        locals.var_udr_i_rv = 0.0;

        locals.var_vsatr_t = 0.0;
        locals.var_vsatr_t_dn3 = 0.0;
        locals.var_vsatr_t_dn4 = 0.0;
        locals.var_vsatr_t_dn5 = 0.0;
        locals.var_vsatr_t_dn6 = 0.0;
        locals.var_vsatr_t_dn7 = 0.0;
        locals.var_vsatr_t_dn8 = 0.0;
        locals.var_vsatr_t_dn9 = 0.0;
        locals.var_vsatr_t_dn10 = 0.0;
        locals.var_vsatr_t_dn11 = 0.0;
        locals.var_vsatr_t_rv = 0.0;

        locals.var_local_scb = 0.0;
        locals.var_local_scb_dn3 = 0.0;
        locals.var_local_scb_dn4 = 0.0;
        locals.var_local_scb_dn5 = 0.0;
        locals.var_local_scb_dn6 = 0.0;
        locals.var_local_scb_dn7 = 0.0;
        locals.var_local_scb_dn8 = 0.0;
        locals.var_local_scb_dn9 = 0.0;
        locals.var_local_scb_dn10 = 0.0;
        locals.var_local_scb_dn11 = 0.0;
        locals.var_local_scb_rv = 0.0;

        locals.var_vth0_stress_edge = 0.0;
        locals.var_vth0_stress_edge_dn3 = 0.0;
        locals.var_vth0_stress_edge_dn4 = 0.0;
        locals.var_vth0_stress_edge_dn5 = 0.0;
        locals.var_vth0_stress_edge_dn6 = 0.0;
        locals.var_vth0_stress_edge_dn7 = 0.0;
        locals.var_vth0_stress_edge_dn8 = 0.0;
        locals.var_vth0_stress_edge_dn9 = 0.0;
        locals.var_vth0_stress_edge_dn10 = 0.0;
        locals.var_vth0_stress_edge_dn11 = 0.0;
        locals.var_vth0_stress_edge_rv = 0.0;

        locals.var_eta_stress_edge = 0.0;
        locals.var_eta_stress_edge_dn3 = 0.0;
        locals.var_eta_stress_edge_dn4 = 0.0;
        locals.var_eta_stress_edge_dn5 = 0.0;
        locals.var_eta_stress_edge_dn6 = 0.0;
        locals.var_eta_stress_edge_dn7 = 0.0;
        locals.var_eta_stress_edge_dn8 = 0.0;
        locals.var_eta_stress_edge_dn9 = 0.0;
        locals.var_eta_stress_edge_dn10 = 0.0;
        locals.var_eta_stress_edge_dn11 = 0.0;
        locals.var_eta_stress_edge_rv = 0.0;

        locals.var_m01_i = 0.0;
        locals.var_m01_i_rv = 0.0;

        locals.var_cdscdedge_i = 0.0;
        locals.var_cdscdedge_i_rv = 0.0;

        locals.var_kt1edge_i = 0.0;
        locals.var_kt1edge_i_rv = 0.0;

        locals.var_tnfactoredge_i = 0.0;
        locals.var_tnfactoredge_i_rv = 0.0;

        locals.var_stk2edge_i = 0.0;
        locals.var_stk2edge_i_rv = 0.0;

        locals.var_c01_i = 0.0;
        locals.var_c01_i_rv = 0.0;

        locals.var_c0si_t = 0.0;
        locals.var_c0si_t_dn4 = 0.0;
        locals.var_c0si_t_dn5 = 0.0;
        locals.var_c0si_t_rv = 0.0;

        locals.var_l_lln1 = 0.0;
        locals.var_l_lln1_rv = 0.0;

        locals.var_psatr_i = 0.0;
        locals.var_psatr_i_rv = 0.0;

        locals.var_u0r_t = 0.0;
        locals.var_u0r_t_dn4 = 0.0;
        locals.var_u0r_t_dn5 = 0.0;
        locals.var_u0r_t_rv = 0.0;

        locals.var_ucr_t = 0.0;
        locals.var_ucr_t_dn3 = 0.0;
        locals.var_ucr_t_dn4 = 0.0;
        locals.var_ucr_t_dn5 = 0.0;
        locals.var_ucr_t_dn6 = 0.0;
        locals.var_ucr_t_dn7 = 0.0;
        locals.var_ucr_t_dn8 = 0.0;
        locals.var_ucr_t_dn9 = 0.0;
        locals.var_ucr_t_dn10 = 0.0;
        locals.var_ucr_t_dn11 = 0.0;
        locals.var_ucr_t_rv = 0.0;

        locals.var_udr_t = 0.0;
        locals.var_udr_t_dn3 = 0.0;
        locals.var_udr_t_dn4 = 0.0;
        locals.var_udr_t_dn5 = 0.0;
        locals.var_udr_t_dn6 = 0.0;
        locals.var_udr_t_dn7 = 0.0;
        locals.var_udr_t_dn8 = 0.0;
        locals.var_udr_t_dn9 = 0.0;
        locals.var_udr_t_dn10 = 0.0;
        locals.var_udr_t_dn11 = 0.0;
        locals.var_udr_t_rv = 0.0;

        locals.var_w_lwn1 = 0.0;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_local_scc = 0.0;
        locals.var_local_scc_dn3 = 0.0;
        locals.var_local_scc_dn4 = 0.0;
        locals.var_local_scc_dn5 = 0.0;
        locals.var_local_scc_dn6 = 0.0;
        locals.var_local_scc_dn7 = 0.0;
        locals.var_local_scc_dn8 = 0.0;
        locals.var_local_scc_dn9 = 0.0;
        locals.var_local_scc_dn10 = 0.0;
        locals.var_local_scc_dn11 = 0.0;
        locals.var_local_scc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_k2_stress_edge = 0.0;
        locals.var_k2_stress_edge_dn3 = 0.0;
        locals.var_k2_stress_edge_dn4 = 0.0;
        locals.var_k2_stress_edge_dn5 = 0.0;
        locals.var_k2_stress_edge_dn6 = 0.0;
        locals.var_k2_stress_edge_dn7 = 0.0;
        locals.var_k2_stress_edge_dn8 = 0.0;
        locals.var_k2_stress_edge_dn9 = 0.0;
        locals.var_k2_stress_edge_dn10 = 0.0;
        locals.var_k2_stress_edge_dn11 = 0.0;
        locals.var_k2_stress_edge_rv = 0.0;

        locals.var_k0_i = 0.0;
        locals.var_k0_i_rv = 0.0;

        locals.var_k0_t = 0.0;
        locals.var_k0_t_dn4 = 0.0;
        locals.var_k0_t_dn5 = 0.0;
        locals.var_k0_t_rv = 0.0;

        locals.var_cdscbedge_i = 0.0;
        locals.var_cdscbedge_i_rv = 0.0;

        locals.var_kt1ledge_i = 0.0;
        locals.var_kt1ledge_i_rv = 0.0;

        locals.var_teta0edge_i = 0.0;
        locals.var_teta0edge_i_rv = 0.0;

        locals.var_steta0edge_i = 0.0;
        locals.var_steta0edge_i_rv = 0.0;

        locals.var_c0_t = 0.0;
        locals.var_c0_t_dn4 = 0.0;
        locals.var_c0_t_dn5 = 0.0;
        locals.var_c0_t_rv = 0.0;

        locals.var_c0sisat_i = 0.0;
        locals.var_c0sisat_i_rv = 0.0;

        locals.var_k2edgewe_i = 0.0;
        locals.var_k2edgewe_i_rv = 0.0;

        locals.var_kvth0edgewe_i = 0.0;
        locals.var_kvth0edgewe_i_rv = 0.0;

        locals.var_temp_adeff = 0.0;
        locals.var_temp_adeff_dn3 = 0.0;
        locals.var_temp_adeff_dn4 = 0.0;
        locals.var_temp_adeff_dn5 = 0.0;
        locals.var_temp_adeff_dn6 = 0.0;
        locals.var_temp_adeff_dn7 = 0.0;
        locals.var_temp_adeff_dn8 = 0.0;
        locals.var_temp_adeff_dn9 = 0.0;
        locals.var_temp_adeff_dn10 = 0.0;
        locals.var_temp_adeff_dn11 = 0.0;
        locals.var_temp_adeff_rv = 0.0;

        locals.var_temp_aseff = 0.0;
        locals.var_temp_aseff_dn3 = 0.0;
        locals.var_temp_aseff_dn4 = 0.0;
        locals.var_temp_aseff_dn5 = 0.0;
        locals.var_temp_aseff_dn6 = 0.0;
        locals.var_temp_aseff_dn7 = 0.0;
        locals.var_temp_aseff_dn8 = 0.0;
        locals.var_temp_aseff_dn9 = 0.0;
        locals.var_temp_aseff_dn10 = 0.0;
        locals.var_temp_aseff_dn11 = 0.0;
        locals.var_temp_aseff_rv = 0.0;

        locals.var_temp_pdeff = 0.0;
        locals.var_temp_pdeff_dn3 = 0.0;
        locals.var_temp_pdeff_dn4 = 0.0;
        locals.var_temp_pdeff_dn5 = 0.0;
        locals.var_temp_pdeff_dn6 = 0.0;
        locals.var_temp_pdeff_dn7 = 0.0;
        locals.var_temp_pdeff_dn8 = 0.0;
        locals.var_temp_pdeff_dn9 = 0.0;
        locals.var_temp_pdeff_dn10 = 0.0;
        locals.var_temp_pdeff_dn11 = 0.0;
        locals.var_temp_pdeff_rv = 0.0;

        locals.var_temp_pseff = 0.0;
        locals.var_temp_pseff_dn3 = 0.0;
        locals.var_temp_pseff_dn4 = 0.0;
        locals.var_temp_pseff_dn5 = 0.0;
        locals.var_temp_pseff_dn6 = 0.0;
        locals.var_temp_pseff_dn7 = 0.0;
        locals.var_temp_pseff_dn8 = 0.0;
        locals.var_temp_pseff_dn9 = 0.0;
        locals.var_temp_pseff_dn10 = 0.0;
        locals.var_temp_pseff_dn11 = 0.0;
        locals.var_temp_pseff_rv = 0.0;

        locals.var_abulkiv = 1.0;
        locals.var_abulkiv_dn3 = 0.0;
        locals.var_abulkiv_dn4 = 0.0;
        locals.var_abulkiv_dn5 = 0.0;
        locals.var_abulkiv_dn6 = 0.0;
        locals.var_abulkiv_dn7 = 0.0;
        locals.var_abulkiv_dn8 = 0.0;
        locals.var_abulkiv_dn9 = 0.0;
        locals.var_abulkiv_dn10 = 0.0;
        locals.var_abulkiv_dn11 = 0.0;
        locals.var_abulkiv_rv = 0.0;

        locals.var_abulkcv = 1.0;
        locals.var_abulkcv_dn3 = 0.0;
        locals.var_abulkcv_dn4 = 0.0;
        locals.var_abulkcv_dn5 = 0.0;
        locals.var_abulkcv_dn6 = 0.0;
        locals.var_abulkcv_dn7 = 0.0;
        locals.var_abulkcv_dn8 = 0.0;
        locals.var_abulkcv_dn9 = 0.0;
        locals.var_abulkcv_dn10 = 0.0;
        locals.var_abulkcv_dn11 = 0.0;
        locals.var_abulkcv_rv = 0.0;

        locals.var_eta_p = 1.0;
        locals.var_eta_p_dn3 = 0.0;
        locals.var_eta_p_dn4 = 0.0;
        locals.var_eta_p_dn5 = 0.0;
        locals.var_eta_p_dn6 = 0.0;
        locals.var_eta_p_dn7 = 0.0;
        locals.var_eta_p_dn8 = 0.0;
        locals.var_eta_p_dn9 = 0.0;
        locals.var_eta_p_dn10 = 0.0;
        locals.var_eta_p_dn11 = 0.0;
        locals.var_eta_p_rv = 0.0;

        locals.var_ddl = 0.0;
        locals.var_ddl_dn3 = 0.0;
        locals.var_ddl_dn4 = 0.0;
        locals.var_ddl_dn5 = 0.0;
        locals.var_ddl_dn6 = 0.0;
        locals.var_ddl_dn7 = 0.0;
        locals.var_ddl_dn8 = 0.0;
        locals.var_ddl_dn9 = 0.0;
        locals.var_ddl_dn10 = 0.0;
        locals.var_ddl_dn11 = 0.0;
        locals.var_ddl_rv = 0.0;

        locals.var_dmob = 0.0;
        locals.var_dmob_dn3 = 0.0;
        locals.var_dmob_dn4 = 0.0;
        locals.var_dmob_dn5 = 0.0;
        locals.var_dmob_dn6 = 0.0;
        locals.var_dmob_dn7 = 0.0;
        locals.var_dmob_dn8 = 0.0;
        locals.var_dmob_dn9 = 0.0;
        locals.var_dmob_dn10 = 0.0;
        locals.var_dmob_dn11 = 0.0;
        locals.var_dmob_rv = 0.0;

        locals.var_dr = 0.0;
        locals.var_dr_dn3 = 0.0;
        locals.var_dr_dn4 = 0.0;
        locals.var_dr_dn5 = 0.0;
        locals.var_dr_dn6 = 0.0;
        locals.var_dr_dn7 = 0.0;
        locals.var_dr_dn8 = 0.0;
        locals.var_dr_dn9 = 0.0;
        locals.var_dr_dn10 = 0.0;
        locals.var_dr_dn11 = 0.0;
        locals.var_dr_rv = 0.0;

        locals.var_dvsat = 0.0;
        locals.var_dvsat_dn3 = 0.0;
        locals.var_dvsat_dn4 = 0.0;
        locals.var_dvsat_dn5 = 0.0;
        locals.var_dvsat_dn6 = 0.0;
        locals.var_dvsat_dn7 = 0.0;
        locals.var_dvsat_dn8 = 0.0;
        locals.var_dvsat_dn9 = 0.0;
        locals.var_dvsat_dn10 = 0.0;
        locals.var_dvsat_dn11 = 0.0;
        locals.var_dvsat_rv = 0.0;

        locals.var_dvsatinv = 0.0;
        locals.var_dvsatinv_dn3 = 0.0;
        locals.var_dvsatinv_dn4 = 0.0;
        locals.var_dvsatinv_dn5 = 0.0;
        locals.var_dvsatinv_dn6 = 0.0;
        locals.var_dvsatinv_dn7 = 0.0;
        locals.var_dvsatinv_dn8 = 0.0;
        locals.var_dvsatinv_dn9 = 0.0;
        locals.var_dvsatinv_dn10 = 0.0;
        locals.var_dvsatinv_dn11 = 0.0;
        locals.var_dvsatinv_rv = 0.0;

        locals.var_ibddif = 0.0;
        locals.var_ibddif_dn3 = 0.0;
        locals.var_ibddif_dn4 = 0.0;
        locals.var_ibddif_dn5 = 0.0;
        locals.var_ibddif_dn6 = 0.0;
        locals.var_ibddif_dn7 = 0.0;
        locals.var_ibddif_dn8 = 0.0;
        locals.var_ibddif_dn9 = 0.0;
        locals.var_ibddif_dn10 = 0.0;
        locals.var_ibddif_dn11 = 0.0;
        locals.var_ibddif_rv = 0.0;

        locals.var_ibsdif = 0.0;
        locals.var_ibsdif_dn3 = 0.0;
        locals.var_ibsdif_dn4 = 0.0;
        locals.var_ibsdif_dn5 = 0.0;
        locals.var_ibsdif_dn6 = 0.0;
        locals.var_ibsdif_dn7 = 0.0;
        locals.var_ibsdif_dn8 = 0.0;
        locals.var_ibsdif_dn9 = 0.0;
        locals.var_ibsdif_dn10 = 0.0;
        locals.var_ibsdif_dn11 = 0.0;
        locals.var_ibsdif_rv = 0.0;

        locals.var_mnud = 0.0;
        locals.var_mnud_dn3 = 0.0;
        locals.var_mnud_dn4 = 0.0;
        locals.var_mnud_dn5 = 0.0;
        locals.var_mnud_dn6 = 0.0;
        locals.var_mnud_dn7 = 0.0;
        locals.var_mnud_dn8 = 0.0;
        locals.var_mnud_dn9 = 0.0;
        locals.var_mnud_dn10 = 0.0;
        locals.var_mnud_dn11 = 0.0;
        locals.var_mnud_rv = 0.0;

        locals.var_moc = 0.0;
        locals.var_moc_dn3 = 0.0;
        locals.var_moc_dn4 = 0.0;
        locals.var_moc_dn5 = 0.0;
        locals.var_moc_dn6 = 0.0;
        locals.var_moc_dn7 = 0.0;
        locals.var_moc_dn8 = 0.0;
        locals.var_moc_dn9 = 0.0;
        locals.var_moc_dn10 = 0.0;
        locals.var_moc_dn11 = 0.0;
        locals.var_moc_rv = 0.0;

        locals.var_mscbe = 0.0;
        locals.var_mscbe_dn3 = 0.0;
        locals.var_mscbe_dn4 = 0.0;
        locals.var_mscbe_dn5 = 0.0;
        locals.var_mscbe_dn6 = 0.0;
        locals.var_mscbe_dn7 = 0.0;
        locals.var_mscbe_dn8 = 0.0;
        locals.var_mscbe_dn9 = 0.0;
        locals.var_mscbe_dn10 = 0.0;
        locals.var_mscbe_dn11 = 0.0;
        locals.var_mscbe_rv = 0.0;

        locals.var_nsat = 0.0;
        locals.var_nsat_dn3 = 0.0;
        locals.var_nsat_dn4 = 0.0;
        locals.var_nsat_dn5 = 0.0;
        locals.var_nsat_dn6 = 0.0;
        locals.var_nsat_dn7 = 0.0;
        locals.var_nsat_dn8 = 0.0;
        locals.var_nsat_dn9 = 0.0;
        locals.var_nsat_dn10 = 0.0;
        locals.var_nsat_dn11 = 0.0;
        locals.var_nsat_rv = 0.0;

        locals.var_rdrain = 0.0;
        locals.var_rdrain_dn3 = 0.0;
        locals.var_rdrain_dn4 = 0.0;
        locals.var_rdrain_dn5 = 0.0;
        locals.var_rdrain_dn6 = 0.0;
        locals.var_rdrain_dn7 = 0.0;
        locals.var_rdrain_dn8 = 0.0;
        locals.var_rdrain_dn9 = 0.0;
        locals.var_rdrain_dn10 = 0.0;
        locals.var_rdrain_dn11 = 0.0;
        locals.var_rdrain_rv = 0.0;

        locals.var_rsource = 0.0;
        locals.var_rsource_dn3 = 0.0;
        locals.var_rsource_dn4 = 0.0;
        locals.var_rsource_dn5 = 0.0;
        locals.var_rsource_dn6 = 0.0;
        locals.var_rsource_dn7 = 0.0;
        locals.var_rsource_dn8 = 0.0;
        locals.var_rsource_dn9 = 0.0;
        locals.var_rsource_dn10 = 0.0;
        locals.var_rsource_dn11 = 0.0;
        locals.var_rsource_rv = 0.0;

        locals.var_vdseff = 0.0;
        locals.var_vdseff_dn3 = 0.0;
        locals.var_vdseff_dn4 = 0.0;
        locals.var_vdseff_dn5 = 0.0;
        locals.var_vdseff_dn6 = 0.0;
        locals.var_vdseff_dn7 = 0.0;
        locals.var_vdseff_dn8 = 0.0;
        locals.var_vdseff_dn9 = 0.0;
        locals.var_vdseff_dn10 = 0.0;
        locals.var_vdseff_dn11 = 0.0;
        locals.var_vdseff_rv = 0.0;

        locals.var_diffvds = 0.0;
        locals.var_diffvds_dn3 = 0.0;
        locals.var_diffvds_dn4 = 0.0;
        locals.var_diffvds_dn5 = 0.0;
        locals.var_diffvds_dn6 = 0.0;
        locals.var_diffvds_dn7 = 0.0;
        locals.var_diffvds_dn8 = 0.0;
        locals.var_diffvds_dn9 = 0.0;
        locals.var_diffvds_dn10 = 0.0;
        locals.var_diffvds_dn11 = 0.0;
        locals.var_diffvds_rv = 0.0;

        locals.var_dps = 0.0;
        locals.var_dps_dn3 = 0.0;
        locals.var_dps_dn4 = 0.0;
        locals.var_dps_dn5 = 0.0;
        locals.var_dps_dn6 = 0.0;
        locals.var_dps_dn7 = 0.0;
        locals.var_dps_dn8 = 0.0;
        locals.var_dps_dn9 = 0.0;
        locals.var_dps_dn10 = 0.0;
        locals.var_dps_dn11 = 0.0;
        locals.var_dps_rv = 0.0;

        locals.var_qia = 0.0;
        locals.var_qia_dn3 = 0.0;
        locals.var_qia_dn4 = 0.0;
        locals.var_qia_dn5 = 0.0;
        locals.var_qia_dn6 = 0.0;
        locals.var_qia_dn7 = 0.0;
        locals.var_qia_dn8 = 0.0;
        locals.var_qia_dn9 = 0.0;
        locals.var_qia_dn10 = 0.0;
        locals.var_qia_dn11 = 0.0;
        locals.var_qia_rv = 0.0;

        locals.var_qid = 0.0;
        locals.var_qid_dn3 = 0.0;
        locals.var_qid_dn4 = 0.0;
        locals.var_qid_dn5 = 0.0;
        locals.var_qid_dn6 = 0.0;
        locals.var_qid_dn7 = 0.0;
        locals.var_qid_dn8 = 0.0;
        locals.var_qid_dn9 = 0.0;
        locals.var_qid_dn10 = 0.0;
        locals.var_qid_dn11 = 0.0;
        locals.var_qid_rv = 0.0;

        locals.var_qim1 = 0.0;
        locals.var_qim1_dn3 = 0.0;
        locals.var_qim1_dn4 = 0.0;
        locals.var_qim1_dn5 = 0.0;
        locals.var_qim1_dn6 = 0.0;
        locals.var_qim1_dn7 = 0.0;
        locals.var_qim1_dn8 = 0.0;
        locals.var_qim1_dn9 = 0.0;
        locals.var_qim1_dn10 = 0.0;
        locals.var_qim1_dn11 = 0.0;
        locals.var_qim1_rv = 0.0;

        locals.var_rdsi = 0.0;
        locals.var_rdsi_dn3 = 0.0;
        locals.var_rdsi_dn4 = 0.0;
        locals.var_rdsi_dn5 = 0.0;
        locals.var_rdsi_dn6 = 0.0;
        locals.var_rdsi_dn7 = 0.0;
        locals.var_rdsi_dn8 = 0.0;
        locals.var_rdsi_dn9 = 0.0;
        locals.var_rdsi_dn10 = 0.0;
        locals.var_rdsi_dn11 = 0.0;
        locals.var_rdsi_rv = 0.0;

        let assign1090_e2343: f64 = (1.3806503e-23 / 1.602176462e-19);
        locals.var_kboq = assign1090_e2343;
        locals.var_kboq_rv = 0.0;

        locals.var_qdeff = 0.0;
        locals.var_qdeff_dn3 = 0.0;
        locals.var_qdeff_dn4 = 0.0;
        locals.var_qdeff_dn5 = 0.0;
        locals.var_qdeff_dn6 = 0.0;
        locals.var_qdeff_dn7 = 0.0;
        locals.var_qdeff_dn8 = 0.0;
        locals.var_qdeff_dn9 = 0.0;
        locals.var_qdeff_dn10 = 0.0;
        locals.var_qdeff_dn11 = 0.0;
        locals.var_qdeff_rv = 0.0;

        locals.var_qs_1 = 0.0;
        locals.var_qs_1_dn3 = 0.0;
        locals.var_qs_1_dn4 = 0.0;
        locals.var_qs_1_dn5 = 0.0;
        locals.var_qs_1_dn6 = 0.0;
        locals.var_qs_1_dn7 = 0.0;
        locals.var_qs_1_dn8 = 0.0;
        locals.var_qs_1_dn9 = 0.0;
        locals.var_qs_1_dn10 = 0.0;
        locals.var_qs_1_dn11 = 0.0;
        locals.var_qs_1_rv = 0.0;

        locals.var_x7_d = 0.0;
        locals.var_x7_d_dn3 = 0.0;
        locals.var_x7_d_dn4 = 0.0;
        locals.var_x7_d_dn5 = 0.0;
        locals.var_x7_d_dn6 = 0.0;
        locals.var_x7_d_dn7 = 0.0;
        locals.var_x7_d_dn8 = 0.0;
        locals.var_x7_d_dn9 = 0.0;
        locals.var_x7_d_dn10 = 0.0;
        locals.var_x7_d_dn11 = 0.0;
        locals.var_x7_d_rv = 0.0;

        locals.var_x7_s = 0.0;
        locals.var_x7_s_dn3 = 0.0;
        locals.var_x7_s_dn4 = 0.0;
        locals.var_x7_s_dn5 = 0.0;
        locals.var_x7_s_dn6 = 0.0;
        locals.var_x7_s_dn7 = 0.0;
        locals.var_x7_s_dn8 = 0.0;
        locals.var_x7_s_dn9 = 0.0;
        locals.var_x7_s_dn10 = 0.0;
        locals.var_x7_s_dn11 = 0.0;
        locals.var_x7_s_rv = 0.0;

        locals.var_ln_t1_t2 = 0.0;
        locals.var_ln_t1_t2_dn3 = 0.0;
        locals.var_ln_t1_t2_dn4 = 0.0;
        locals.var_ln_t1_t2_dn5 = 0.0;
        locals.var_ln_t1_t2_dn6 = 0.0;
        locals.var_ln_t1_t2_dn7 = 0.0;
        locals.var_ln_t1_t2_dn8 = 0.0;
        locals.var_ln_t1_t2_dn9 = 0.0;
        locals.var_ln_t1_t2_dn10 = 0.0;
        locals.var_ln_t1_t2_dn11 = 0.0;
        locals.var_ln_t1_t2_rv = 0.0;

        locals.var_alpha_dd = 0.0;
        locals.var_alpha_dd_dn3 = 0.0;
        locals.var_alpha_dd_dn4 = 0.0;
        locals.var_alpha_dd_dn5 = 0.0;
        locals.var_alpha_dd_dn6 = 0.0;
        locals.var_alpha_dd_dn7 = 0.0;
        locals.var_alpha_dd_dn8 = 0.0;
        locals.var_alpha_dd_dn9 = 0.0;
        locals.var_alpha_dd_dn10 = 0.0;
        locals.var_alpha_dd_dn11 = 0.0;
        locals.var_alpha_dd_rv = 0.0;

        locals.var_qim = 0.0;
        locals.var_qim_dn3 = 0.0;
        locals.var_qim_dn4 = 0.0;
        locals.var_qim_dn5 = 0.0;
        locals.var_qim_dn6 = 0.0;
        locals.var_qim_dn7 = 0.0;
        locals.var_qim_dn8 = 0.0;
        locals.var_qim_dn9 = 0.0;
        locals.var_qim_dn10 = 0.0;
        locals.var_qim_dn11 = 0.0;
        locals.var_qim_rv = 0.0;

        locals.var_h_fact = 0.0;
        locals.var_h_fact_dn3 = 0.0;
        locals.var_h_fact_dn4 = 0.0;
        locals.var_h_fact_dn5 = 0.0;
        locals.var_h_fact_dn6 = 0.0;
        locals.var_h_fact_dn7 = 0.0;
        locals.var_h_fact_dn8 = 0.0;
        locals.var_h_fact_dn9 = 0.0;
        locals.var_h_fact_dn10 = 0.0;
        locals.var_h_fact_dn11 = 0.0;
        locals.var_h_fact_rv = 0.0;

        locals.var_nq_edge = 0.0;
        locals.var_nq_edge_dn3 = 0.0;
        locals.var_nq_edge_dn4 = 0.0;
        locals.var_nq_edge_dn5 = 0.0;
        locals.var_nq_edge_dn6 = 0.0;
        locals.var_nq_edge_dn7 = 0.0;
        locals.var_nq_edge_dn8 = 0.0;
        locals.var_nq_edge_dn9 = 0.0;
        locals.var_nq_edge_dn10 = 0.0;
        locals.var_nq_edge_dn11 = 0.0;
        locals.var_nq_edge_rv = 0.0;

        locals.var_qdeff_edge = 0.0;
        locals.var_qdeff_edge_dn3 = 0.0;
        locals.var_qdeff_edge_dn4 = 0.0;
        locals.var_qdeff_edge_dn5 = 0.0;
        locals.var_qdeff_edge_dn6 = 0.0;
        locals.var_qdeff_edge_dn7 = 0.0;
        locals.var_qdeff_edge_dn8 = 0.0;
        locals.var_qdeff_edge_dn9 = 0.0;
        locals.var_qdeff_edge_dn10 = 0.0;
        locals.var_qdeff_edge_dn11 = 0.0;
        locals.var_qdeff_edge_rv = 0.0;

        locals.var_qs_edge = 0.0;
        locals.var_qs_edge_dn3 = 0.0;
        locals.var_qs_edge_dn4 = 0.0;
        locals.var_qs_edge_dn5 = 0.0;
        locals.var_qs_edge_dn6 = 0.0;
        locals.var_qs_edge_dn7 = 0.0;
        locals.var_qs_edge_dn8 = 0.0;
        locals.var_qs_edge_dn9 = 0.0;
        locals.var_qs_edge_dn10 = 0.0;
        locals.var_qs_edge_dn11 = 0.0;
        locals.var_qs_edge_rv = 0.0;

        let assign1210_e2357: f64 = if p.p30 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign1210_e2357;
        locals.var_guard1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign1220_e2361,) = {
    if (locals.var_guard1 != 0.0) {
        (1.0,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign1220_e2361;
        locals.var_devsign_rv = 0.0;

        let (assign1230_e2367,) = {
    if (locals.var_guard1 == 0.0) {
        let assign1230_e2365: f64 = (-1.0);
        (assign1230_e2365,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign1230_e2367;
        locals.var_devsign_rv = 0.0;

        let assign1240_e2370: f64 = (p.p109 * 8.8541878128e-12);
        locals.var_epssi = assign1240_e2370;
        locals.var_epssi_rv = 0.0;

        let assign1250_e2373: f64 = (p.p110 * 8.8541878128e-12);
        locals.var_epsox = assign1250_e2373;
        locals.var_epsox_rv = 0.0;

        let assign1260_e2376: f64 = (p.p110 * 8.8541878128e-12);
        let assign1260_e2378: f64 = (assign1260_e2376 / p.p76);
        locals.var_cox = assign1260_e2378;
        locals.var_cox_rv = 0.0;

        let assign1270_e2381: f64 = (p.p109 / p.p110);
        locals.var_epsratio = assign1270_e2381;
        locals.var_epsratio_rv = 0.0;

        let assign1280_e2384: f64 = if (!param_given[77]) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign1280_e2384;
        locals.var_guard2_rv = 0.0;

        let (assign1290_e2394,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1290_e2388: f64 = (p.p76 * p.p110);
        let assign1290_e2390: f64 = (assign1290_e2388 / 3.9);
        let assign1290_e2392: f64 = (assign1290_e2390 - p.p78);
        (assign1290_e2392,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1290_e2394;
        locals.var_bsimbulktoxp_rv = 0.0;

        let (assign1300_e2399,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p77,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1300_e2399;
        locals.var_bsimbulktoxp_rv = 0.0;

        let assign1310_e2402: f64 = (p.p0 * p.p49);
        locals.var_l_mult = assign1310_e2402;
        locals.var_l_mult_rv = 0.0;

        let assign1320_e2405: f64 = (p.p1 * p.p50);
        locals.var_w_mult = assign1320_e2405;
        locals.var_w_mult_rv = 0.0;

        let assign1330_e2408: f64 = (locals.var_l_mult + p.p51);
        locals.var_lnew = assign1330_e2408;
        locals.var_lnew_rv = 0.0;

        let assign1350_e2414: f64 = (locals.var_w_mult / p.p2);
        locals.var_w_by_nf = assign1350_e2414;
        locals.var_w_by_nf_rv = 0.0;

        let assign1360_e2417: f64 = (locals.var_w_by_nf + p.p53);
        locals.var_wnew = assign1360_e2417;
        locals.var_wnew_rv = 0.0;

        let assign1380_e2423: f64 = (-p.p58);
        let assign1380_e2424: f64 = (locals.var_lnew).powf(assign1380_e2423);
        locals.var_l_lln = assign1380_e2424;
        locals.var_l_lln_rv = 0.0;

        let assign1390_e2427: f64 = (-p.p59);
        let assign1390_e2428: f64 = (locals.var_wnew).powf(assign1390_e2427);
        locals.var_w_lwn = assign1390_e2428;
        locals.var_w_lwn_rv = 0.0;

        let assign1400_e2431: f64 = (locals.var_l_lln * locals.var_w_lwn);
        locals.var_lw_lln_lwn = assign1400_e2431;
        locals.var_lw_lln_lwn_rv = 0.0;

        let assign1410_e2435: f64 = (p.p55 * locals.var_l_lln);
        let assign1410_e2436: f64 = (p.p54 + assign1410_e2435);
        let assign1410_e2439: f64 = (p.p56 * locals.var_w_lwn);
        let assign1410_e2440: f64 = (assign1410_e2436 + assign1410_e2439);
        let assign1410_e2443: f64 = (p.p57 * locals.var_lw_lln_lwn);
        let assign1410_e2444: f64 = (assign1410_e2440 + assign1410_e2443);
        locals.var_dliv = assign1410_e2444;
        locals.var_dliv_rv = 0.0;

        let assign1420_e2447: f64 = (-p.p64);
        let assign1420_e2448: f64 = (locals.var_lnew).powf(assign1420_e2447);
        locals.var_l_wln = assign1420_e2448;
        locals.var_l_wln_rv = 0.0;

        let assign1430_e2451: f64 = (-p.p65);
        let assign1430_e2452: f64 = (locals.var_wnew).powf(assign1430_e2451);
        locals.var_w_wwn = assign1430_e2452;
        locals.var_w_wwn_rv = 0.0;

        let assign1440_e2455: f64 = (locals.var_l_wln * locals.var_w_wwn);
        locals.var_lw_wln_wwn = assign1440_e2455;
        locals.var_lw_wln_wwn_rv = 0.0;

        let assign1450_e2459: f64 = (p.p61 * locals.var_l_wln);
        let assign1450_e2460: f64 = (p.p60 + assign1450_e2459);
        let assign1450_e2463: f64 = (p.p62 * locals.var_w_wwn);
        let assign1450_e2464: f64 = (assign1450_e2460 + assign1450_e2463);
        let assign1450_e2467: f64 = (p.p63 * locals.var_lw_wln_wwn);
        let assign1450_e2468: f64 = (assign1450_e2464 + assign1450_e2467);
        locals.var_dwiv = assign1450_e2468;
        locals.var_dwiv_rv = 0.0;

        let assign1460_e2472: f64 = (2.0 * locals.var_dliv);
        let assign1460_e2473: f64 = (locals.var_lnew - assign1460_e2472);
        locals.var_leff = assign1460_e2473;
        locals.var_leff_rv = 0.0;

        let assign1490_e2483: f64 = (p.p1375 * p.p1376);
        let assign1490_e2484: f64 = (locals.var_wnew - assign1490_e2483);
        let assign1490_e2487: f64 = (2.0 - p.p1375);
        let assign1490_e2489: f64 = (assign1490_e2487 * locals.var_dwiv);
        let assign1490_e2490: f64 = (assign1490_e2484 - assign1490_e2489);
        locals.var_weff = assign1490_e2490;
        locals.var_weff_rv = 0.0;

        let assign1520_e2500: f64 = (p.p67 * locals.var_l_lln);
        let assign1520_e2501: f64 = (p.p66 + assign1520_e2500);
        let assign1520_e2504: f64 = (p.p68 * locals.var_w_lwn);
        let assign1520_e2505: f64 = (assign1520_e2501 + assign1520_e2504);
        let assign1520_e2508: f64 = (p.p69 * locals.var_lw_lln_lwn);
        let assign1520_e2509: f64 = (assign1520_e2505 + assign1520_e2508);
        locals.var_dlcv = assign1520_e2509;
        locals.var_dlcv_rv = 0.0;

        let assign1530_e2513: f64 = (p.p71 * locals.var_l_wln);
        let assign1530_e2514: f64 = (p.p70 + assign1530_e2513);
        let assign1530_e2517: f64 = (p.p72 * locals.var_w_wwn);
        let assign1530_e2518: f64 = (assign1530_e2514 + assign1530_e2517);
        let assign1530_e2521: f64 = (p.p73 * locals.var_lw_wln_wwn);
        let assign1530_e2522: f64 = (assign1530_e2518 + assign1530_e2521);
        locals.var_dwcv = assign1530_e2522;
        locals.var_dwcv_rv = 0.0;

        let assign1540_e2526: f64 = (2.0 * locals.var_dlcv);
        let assign1540_e2527: f64 = (locals.var_lnew - assign1540_e2526);
        locals.var_lact = assign1540_e2527;
        locals.var_lact_rv = 0.0;

        let assign1570_e2537: f64 = (p.p1375 * p.p1376);
        let assign1570_e2538: f64 = (locals.var_wnew - assign1570_e2537);
        let assign1570_e2541: f64 = (2.0 - p.p1375);
        let assign1570_e2543: f64 = (assign1570_e2541 * locals.var_dwcv);
        let assign1570_e2544: f64 = (assign1570_e2538 - assign1570_e2543);
        locals.var_wact = assign1570_e2544;
        locals.var_wact_rv = 0.0;

        let assign1600_e2555: f64 = (locals.var_lnew).powf(p.p64);
        let assign1600_e2556: f64 = (p.p71 / assign1600_e2555);
        let assign1600_e2557: f64 = (p.p927 + assign1600_e2556);
        let assign1600_e2561: f64 = (locals.var_wnew).powf(p.p65);
        let assign1600_e2562: f64 = (p.p72 / assign1600_e2561);
        let assign1600_e2563: f64 = (assign1600_e2557 + assign1600_e2562);
        let assign1600_e2567: f64 = (locals.var_lnew).powf(p.p64);
        let assign1600_e2568: f64 = (p.p73 / assign1600_e2567);
        let assign1600_e2571: f64 = (locals.var_wnew).powf(p.p65);
        let assign1600_e2572: f64 = (assign1600_e2568 / assign1600_e2571);
        let assign1600_e2573: f64 = (assign1600_e2563 + assign1600_e2572);
        locals.var_dwj = assign1600_e2573;
        locals.var_dwj_rv = 0.0;

        let assign1610_e2577: f64 = (2.0 * locals.var_dwj);
        let assign1610_e2578: f64 = (locals.var_wnew - assign1610_e2577);
        locals.var_weffcj = assign1610_e2578;
        locals.var_weffcj_rv = 0.0;

        let assign1630_e2584: f64 = (1e-6 / locals.var_leff);
        locals.var_inv_l = assign1630_e2584;
        locals.var_inv_l_rv = 0.0;

        let assign1640_e2587: f64 = (1e-6 / locals.var_weff);
        locals.var_inv_w = assign1640_e2587;
        locals.var_inv_w_rv = 0.0;

        let assign1650_e2590: f64 = (1e-6 / locals.var_lact);
        locals.var_inv_lact = assign1650_e2590;
        locals.var_inv_lact_rv = 0.0;

        let assign1660_e2593: f64 = (1e-6 / locals.var_wact);
        locals.var_inv_wact = assign1660_e2593;
        locals.var_inv_wact_rv = 0.0;

        let assign1670_e2596: f64 = (1e-6 / p.p48);
        locals.var_inv_llong = assign1670_e2596;
        locals.var_inv_llong_rv = 0.0;

        let assign1680_e2599: f64 = (1e-6 / p.p52);
        locals.var_inv_wwide = assign1680_e2599;
        locals.var_inv_wwide_rv = 0.0;

        let assign1690_e2602: f64 = (locals.var_inv_l * locals.var_inv_w);
        locals.var_inv_wl = assign1690_e2602;
        locals.var_inv_wl_rv = 0.0;

        locals.var_l_lln1 = locals.var_l_lln;
        locals.var_l_lln1_rv = 0.0;

        locals.var_l_wln1 = locals.var_l_wln;
        locals.var_l_wln1_rv = 0.0;

        let assign1720_e2607: f64 = if p.p1026 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1720_e2607;
        locals.var_guard14_rv = 0.0;

        let assign1730_e2610: f64 = (-locals.var_lnew);
        let assign1730_e2611: f64 = if p.p1026 <= assign1730_e2610 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1730_e2611;
        locals.var_guard15_rv = 0.0;

        let (assign1740_e2623,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1740_e2618: f64 = (locals.var_lnew + p.p1026);
        let assign1740_e2620: f64 = (-p.p58);
        let assign1740_e2621: f64 = (assign1740_e2618).powf(assign1740_e2620);
        (assign1740_e2621,)
    } else {
        (locals.var_l_lln1,)
    }
};
        locals.var_l_lln1 = assign1740_e2623;
        locals.var_l_lln1_rv = 0.0;

        let (assign1750_e2635,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1750_e2630: f64 = (locals.var_lnew + p.p1026);
        let assign1750_e2632: f64 = (-p.p64);
        let assign1750_e2633: f64 = (assign1750_e2630).powf(assign1750_e2632);
        (assign1750_e2633,)
    } else {
        (locals.var_l_wln1,)
    }
};
        locals.var_l_wln1 = assign1750_e2635;
        locals.var_l_wln1_rv = 0.0;

        locals.var_w_lwn1 = locals.var_w_lwn;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_w_wwn1 = locals.var_w_wwn;
        locals.var_w_wwn1_rv = 0.0;

        let assign1780_e2640: f64 = if p.p1027 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1780_e2640;
        locals.var_guard16_rv = 0.0;

        let assign1790_e2643: f64 = (-locals.var_wnew);
        let assign1790_e2644: f64 = if p.p1027 <= assign1790_e2643 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1790_e2644;
        locals.var_guard17_rv = 0.0;

        let (assign1800_e2656,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1800_e2651: f64 = (locals.var_wnew + p.p1027);
        let assign1800_e2653: f64 = (-p.p59);
        let assign1800_e2654: f64 = (assign1800_e2651).powf(assign1800_e2653);
        (assign1800_e2654,)
    } else {
        (locals.var_w_lwn1,)
    }
};
        locals.var_w_lwn1 = assign1800_e2656;
        locals.var_w_lwn1_rv = 0.0;

        let (assign1810_e2668,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1810_e2663: f64 = (locals.var_wnew + p.p1027);
        let assign1810_e2665: f64 = (-p.p65);
        let assign1810_e2666: f64 = (assign1810_e2663).powf(assign1810_e2665);
        (assign1810_e2666,)
    } else {
        (locals.var_w_wwn1,)
    }
};
        locals.var_w_wwn1 = assign1810_e2668;
        locals.var_w_wwn1_rv = 0.0;

        let assign1820_e2671: f64 = (locals.var_l_lln1 * locals.var_w_lwn1);
        locals.var_lw_lln_lwn1 = assign1820_e2671;
        locals.var_lw_lln_lwn1_rv = 0.0;

        let assign1830_e2675: f64 = (p.p55 * locals.var_l_lln1);
        let assign1830_e2676: f64 = (p.p54 + assign1830_e2675);
        let assign1830_e2679: f64 = (p.p56 * locals.var_w_lwn1);
        let assign1830_e2680: f64 = (assign1830_e2676 + assign1830_e2679);
        let assign1830_e2683: f64 = (p.p57 * locals.var_lw_lln_lwn1);
        let assign1830_e2684: f64 = (assign1830_e2680 + assign1830_e2683);
        locals.var_dlb = assign1830_e2684;
        locals.var_dlb_rv = 0.0;

        let assign1840_e2687: f64 = (locals.var_l_wln1 * locals.var_w_wwn1);
        locals.var_lw_wln_wwn1 = assign1840_e2687;
        locals.var_lw_wln_wwn1_rv = 0.0;

        let assign1850_e2691: f64 = (p.p61 * locals.var_l_wln1);
        let assign1850_e2692: f64 = (p.p60 + assign1850_e2691);
        let assign1850_e2695: f64 = (p.p62 * locals.var_w_wwn1);
        let assign1850_e2696: f64 = (assign1850_e2692 + assign1850_e2695);
        let assign1850_e2699: f64 = (p.p63 * locals.var_lw_wln_wwn1);
        let assign1850_e2700: f64 = (assign1850_e2696 + assign1850_e2699);
        locals.var_dwb = assign1850_e2700;
        locals.var_dwb_rv = 0.0;

        let assign1860_e2704: f64 = (2.0 * locals.var_dlb);
        let assign1860_e2705: f64 = (locals.var_lnew - assign1860_e2704);
        let assign1860_e2707: f64 = (assign1860_e2705 + p.p1026);
        locals.var_leff1 = assign1860_e2707;
        locals.var_leff1_rv = 0.0;

        let assign1880_e2714: f64 = (2.0 * locals.var_dwb);
        let assign1880_e2715: f64 = (locals.var_wnew - assign1880_e2714);
        let assign1880_e2717: f64 = (assign1880_e2715 + p.p1027);
        locals.var_weff1 = assign1880_e2717;
        locals.var_weff1_rv = 0.0;

        let assign1900_e2723: f64 = if p.p1025 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1900_e2723;
        locals.var_guard20_rv = 0.0;

        let (assign1910_e2729,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1910_e2727: f64 = (1e-6 / locals.var_leff1);
        (assign1910_e2727,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1910_e2729;
        locals.var_bin_l_rv = 0.0;

        let (assign1920_e2735,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1920_e2733: f64 = (1e-6 / locals.var_weff1);
        (assign1920_e2733,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1920_e2735;
        locals.var_bin_w_rv = 0.0;

        let (assign1930_e2742,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1930_e2740: f64 = (1.0 / locals.var_leff1);
        (assign1930_e2740,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1930_e2742;
        locals.var_bin_l_rv = 0.0;

        let (assign1940_e2749,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1940_e2747: f64 = (1.0 / locals.var_weff1);
        (assign1940_e2747,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1940_e2749;
        locals.var_bin_w_rv = 0.0;

        let assign1950_e2752: f64 = (locals.var_bin_l * locals.var_bin_w);
        locals.var_bin_wl = assign1950_e2752;
        locals.var_bin_wl_rv = 0.0;

        let assign1960_e2756: f64 = (locals.var_bin_l * p.p116);
        let assign1960_e2757: f64 = (p.p115 + assign1960_e2756);
        let assign1960_e2760: f64 = (locals.var_bin_w * p.p117);
        let assign1960_e2761: f64 = (assign1960_e2757 + assign1960_e2760);
        let assign1960_e2764: f64 = (locals.var_bin_wl * p.p118);
        let assign1960_e2765: f64 = (assign1960_e2761 + assign1960_e2764);
        locals.var_vfb_i = assign1960_e2765;
        locals.var_vfb_i_dn3 = 0.0;
        locals.var_vfb_i_dn4 = 0.0;
        locals.var_vfb_i_dn5 = 0.0;
        locals.var_vfb_i_dn6 = 0.0;
        locals.var_vfb_i_dn7 = 0.0;
        locals.var_vfb_i_dn8 = 0.0;
        locals.var_vfb_i_dn9 = 0.0;
        locals.var_vfb_i_dn10 = 0.0;
        locals.var_vfb_i_dn11 = 0.0;
        locals.var_vfb_i_rv = 0.0;

        let assign1970_e2769: f64 = (locals.var_bin_l * p.p120);
        let assign1970_e2770: f64 = (p.p119 + assign1970_e2769);
        let assign1970_e2773: f64 = (locals.var_bin_w * p.p121);
        let assign1970_e2774: f64 = (assign1970_e2770 + assign1970_e2773);
        let assign1970_e2777: f64 = (locals.var_bin_wl * p.p122);
        let assign1970_e2778: f64 = (assign1970_e2774 + assign1970_e2777);
        locals.var_vfbb_i = assign1970_e2778;
        locals.var_vfbb_i_rv = 0.0;

        let assign1980_e2782: f64 = (locals.var_bin_l * p.p130);
        let assign1980_e2783: f64 = (p.p129 + assign1980_e2782);
        let assign1980_e2786: f64 = (locals.var_bin_w * p.p131);
        let assign1980_e2787: f64 = (assign1980_e2783 + assign1980_e2786);
        let assign1980_e2790: f64 = (locals.var_bin_wl * p.p132);
        let assign1980_e2791: f64 = (assign1980_e2787 + assign1980_e2790);
        locals.var_vfbcv_i = assign1980_e2791;
        locals.var_vfbcv_i_dn3 = 0.0;
        locals.var_vfbcv_i_dn4 = 0.0;
        locals.var_vfbcv_i_dn5 = 0.0;
        locals.var_vfbcv_i_dn6 = 0.0;
        locals.var_vfbcv_i_dn7 = 0.0;
        locals.var_vfbcv_i_dn8 = 0.0;
        locals.var_vfbcv_i_dn9 = 0.0;
        locals.var_vfbcv_i_dn10 = 0.0;
        locals.var_vfbcv_i_dn11 = 0.0;
        locals.var_vfbcv_i_rv = 0.0;

        let assign1990_e2795: f64 = (locals.var_bin_l * p.p143);
        let assign1990_e2796: f64 = (p.p142 + assign1990_e2795);
        let assign1990_e2799: f64 = (locals.var_bin_w * p.p144);
        let assign1990_e2800: f64 = (assign1990_e2796 + assign1990_e2799);
        let assign1990_e2803: f64 = (locals.var_bin_wl * p.p145);
        let assign1990_e2804: f64 = (assign1990_e2800 + assign1990_e2803);
        locals.var_nsd_i = assign1990_e2804;
        locals.var_nsd_i_rv = 0.0;

        let assign2000_e2808: f64 = (locals.var_bin_l * p.p88);
        let assign2000_e2809: f64 = (p.p79 + assign2000_e2808);
        let assign2000_e2812: f64 = (locals.var_bin_w * p.p89);
        let assign2000_e2813: f64 = (assign2000_e2809 + assign2000_e2812);
        let assign2000_e2816: f64 = (locals.var_bin_wl * p.p90);
        let assign2000_e2817: f64 = (assign2000_e2813 + assign2000_e2816);
        locals.var_ndep_i = assign2000_e2817;
        locals.var_ndep_i_dn3 = 0.0;
        locals.var_ndep_i_dn4 = 0.0;
        locals.var_ndep_i_dn5 = 0.0;
        locals.var_ndep_i_dn6 = 0.0;
        locals.var_ndep_i_dn7 = 0.0;
        locals.var_ndep_i_dn8 = 0.0;
        locals.var_ndep_i_dn9 = 0.0;
        locals.var_ndep_i_dn10 = 0.0;
        locals.var_ndep_i_dn11 = 0.0;
        locals.var_ndep_i_rv = 0.0;

        let assign2010_e2821: f64 = (locals.var_bin_l * p.p100);
        let assign2010_e2822: f64 = (p.p91 + assign2010_e2821);
        let assign2010_e2825: f64 = (locals.var_bin_w * p.p101);
        let assign2010_e2826: f64 = (assign2010_e2822 + assign2010_e2825);
        let assign2010_e2829: f64 = (locals.var_bin_wl * p.p102);
        let assign2010_e2830: f64 = (assign2010_e2826 + assign2010_e2829);
        locals.var_ndepcv_i = assign2010_e2830;
        locals.var_ndepcv_i_dn3 = 0.0;
        locals.var_ndepcv_i_dn4 = 0.0;
        locals.var_ndepcv_i_dn5 = 0.0;
        locals.var_ndepcv_i_dn6 = 0.0;
        locals.var_ndepcv_i_dn7 = 0.0;
        locals.var_ndepcv_i_dn8 = 0.0;
        locals.var_ndepcv_i_dn9 = 0.0;
        locals.var_ndepcv_i_dn10 = 0.0;
        locals.var_ndepcv_i_dn11 = 0.0;
        locals.var_ndepcv_i_rv = 0.0;

        let assign2020_e2834: f64 = (locals.var_bin_l * p.p104);
        let assign2020_e2835: f64 = (p.p103 + assign2020_e2834);
        let assign2020_e2838: f64 = (locals.var_bin_w * p.p105);
        let assign2020_e2839: f64 = (assign2020_e2835 + assign2020_e2838);
        let assign2020_e2842: f64 = (locals.var_bin_wl * p.p106);
        let assign2020_e2843: f64 = (assign2020_e2839 + assign2020_e2842);
        locals.var_ngate_i = assign2020_e2843;
        locals.var_ngate_i_rv = 0.0;

        let assign2030_e2847: f64 = (locals.var_bin_l * p.p233);
        let assign2030_e2848: f64 = (p.p232 + assign2030_e2847);
        let assign2030_e2851: f64 = (locals.var_bin_w * p.p234);
        let assign2030_e2852: f64 = (assign2030_e2848 + assign2030_e2851);
        let assign2030_e2855: f64 = (locals.var_bin_wl * p.p235);
        let assign2030_e2856: f64 = (assign2030_e2852 + assign2030_e2855);
        locals.var_cit_i = assign2030_e2856;
        locals.var_cit_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2040_e2860: f64 = (locals.var_bin_l * p.p243);
        let assign2040_e2861: f64 = (p.p236 + assign2040_e2860);
        let assign2040_e2864: f64 = (locals.var_bin_w * p.p244);
        let assign2040_e2865: f64 = (assign2040_e2861 + assign2040_e2864);
        let assign2040_e2868: f64 = (locals.var_bin_wl * p.p245);
        let assign2040_e2869: f64 = (assign2040_e2865 + assign2040_e2868);
        locals.var_nfactor_i = assign2040_e2869;
        locals.var_nfactor_i_dn3 = 0.0;
        locals.var_nfactor_i_dn4 = 0.0;
        locals.var_nfactor_i_dn5 = 0.0;
        locals.var_nfactor_i_dn6 = 0.0;
        locals.var_nfactor_i_dn7 = 0.0;
        locals.var_nfactor_i_dn8 = 0.0;
        locals.var_nfactor_i_dn9 = 0.0;
        locals.var_nfactor_i_dn10 = 0.0;
        locals.var_nfactor_i_dn11 = 0.0;
        locals.var_nfactor_i_rv = 0.0;

        let assign2050_e2873: f64 = (p.p247 * locals.var_bin_l);
        let assign2050_e2874: f64 = (p.p246 + assign2050_e2873);
        let assign2050_e2877: f64 = (p.p248 * locals.var_bin_w);
        let assign2050_e2878: f64 = (assign2050_e2874 + assign2050_e2877);
        let assign2050_e2881: f64 = (p.p249 * locals.var_bin_wl);
        let assign2050_e2882: f64 = (assign2050_e2878 + assign2050_e2881);
        locals.var_ascl_i = assign2050_e2882;
        locals.var_ascl_i_rv = 0.0;

        let assign2060_e2886: f64 = (p.p251 * locals.var_bin_l);
        let assign2060_e2887: f64 = (p.p250 + assign2060_e2886);
        let assign2060_e2890: f64 = (p.p252 * locals.var_bin_w);
        let assign2060_e2891: f64 = (assign2060_e2887 + assign2060_e2890);
        let assign2060_e2894: f64 = (p.p253 * locals.var_bin_wl);
        let assign2060_e2895: f64 = (assign2060_e2891 + assign2060_e2894);
        locals.var_bscl_i = assign2060_e2895;
        locals.var_bscl_i_rv = 0.0;

        let assign2070_e2899: f64 = (p.p171 * locals.var_bin_l);
        let assign2070_e2900: f64 = (p.p170 + assign2070_e2899);
        let assign2070_e2903: f64 = (p.p172 * locals.var_bin_w);
        let assign2070_e2904: f64 = (assign2070_e2900 + assign2070_e2903);
        let assign2070_e2907: f64 = (p.p173 * locals.var_bin_wl);
        let assign2070_e2908: f64 = (assign2070_e2904 + assign2070_e2907);
        locals.var_dvbd0_i = assign2070_e2908;
        locals.var_dvbd0_i_rv = 0.0;

        let assign2080_e2912: f64 = (p.p175 * locals.var_bin_l);
        let assign2080_e2913: f64 = (p.p174 + assign2080_e2912);
        let assign2080_e2916: f64 = (p.p176 * locals.var_bin_w);
        let assign2080_e2917: f64 = (assign2080_e2913 + assign2080_e2916);
        let assign2080_e2920: f64 = (p.p177 * locals.var_bin_wl);
        let assign2080_e2921: f64 = (assign2080_e2917 + assign2080_e2920);
        locals.var_dvbd1_i = assign2080_e2921;
        locals.var_dvbd1_i_rv = 0.0;

        let assign2090_e2925: f64 = (p.p179 * locals.var_bin_l);
        let assign2090_e2926: f64 = (p.p178 + assign2090_e2925);
        let assign2090_e2929: f64 = (p.p180 * locals.var_bin_w);
        let assign2090_e2930: f64 = (assign2090_e2926 + assign2090_e2929);
        let assign2090_e2933: f64 = (p.p181 * locals.var_bin_wl);
        let assign2090_e2934: f64 = (assign2090_e2930 + assign2090_e2933);
        locals.var_vsce_i = assign2090_e2934;
        locals.var_vsce_i_rv = 0.0;

        let assign2100_e2938: f64 = (p.p187 * locals.var_bin_l);
        let assign2100_e2939: f64 = (p.p186 + assign2100_e2938);
        let assign2100_e2942: f64 = (p.p188 * locals.var_bin_w);
        let assign2100_e2943: f64 = (assign2100_e2939 + assign2100_e2942);
        let assign2100_e2946: f64 = (p.p189 * locals.var_bin_wl);
        let assign2100_e2947: f64 = (assign2100_e2943 + assign2100_e2946);
        locals.var_cdsbs_i = assign2100_e2947;
        locals.var_cdsbs_i_rv = 0.0;

        let assign2110_e2951: f64 = (p.p183 * locals.var_bin_l);
        let assign2110_e2952: f64 = (p.p182 + assign2110_e2951);
        let assign2110_e2955: f64 = (p.p184 * locals.var_bin_w);
        let assign2110_e2956: f64 = (assign2110_e2952 + assign2110_e2955);
        let assign2110_e2959: f64 = (p.p185 * locals.var_bin_wl);
        let assign2110_e2960: f64 = (assign2110_e2956 + assign2110_e2959);
        locals.var_cdsbs1_i = assign2110_e2960;
        locals.var_cdsbs1_i_rv = 0.0;

        let assign2120_e2964: f64 = (p.p255 * locals.var_bin_l);
        let assign2120_e2965: f64 = (p.p254 + assign2120_e2964);
        let assign2120_e2968: f64 = (p.p256 * locals.var_bin_w);
        let assign2120_e2969: f64 = (assign2120_e2965 + assign2120_e2968);
        let assign2120_e2972: f64 = (p.p257 * locals.var_bin_wl);
        let assign2120_e2973: f64 = (assign2120_e2969 + assign2120_e2972);
        locals.var_dvt1_i = assign2120_e2973;
        locals.var_dvt1_i_rv = 0.0;

        let assign2130_e2977: f64 = (locals.var_bin_l * p.p259);
        let assign2130_e2978: f64 = (p.p258 + assign2130_e2977);
        let assign2130_e2981: f64 = (locals.var_bin_w * p.p260);
        let assign2130_e2982: f64 = (assign2130_e2978 + assign2130_e2981);
        let assign2130_e2985: f64 = (locals.var_bin_wl * p.p261);
        let assign2130_e2986: f64 = (assign2130_e2982 + assign2130_e2985);
        locals.var_cdscd_i = assign2130_e2986;
        locals.var_cdscd_i_dn3 = 0.0;
        locals.var_cdscd_i_dn4 = 0.0;
        locals.var_cdscd_i_dn5 = 0.0;
        locals.var_cdscd_i_dn6 = 0.0;
        locals.var_cdscd_i_dn7 = 0.0;
        locals.var_cdscd_i_dn8 = 0.0;
        locals.var_cdscd_i_dn9 = 0.0;
        locals.var_cdscd_i_dn10 = 0.0;
        locals.var_cdscd_i_dn11 = 0.0;
        locals.var_cdscd_i_rv = 0.0;

        let assign2140_e2990: f64 = (locals.var_bin_l * p.p263);
        let assign2140_e2991: f64 = (p.p262 + assign2140_e2990);
        let assign2140_e2994: f64 = (locals.var_bin_w * p.p264);
        let assign2140_e2995: f64 = (assign2140_e2991 + assign2140_e2994);
        let assign2140_e2998: f64 = (locals.var_bin_wl * p.p265);
        let assign2140_e2999: f64 = (assign2140_e2995 + assign2140_e2998);
        locals.var_cdsc_i = assign2140_e2999;
        locals.var_cdsc_i_rv = 0.0;

        let assign2150_e3003: f64 = (locals.var_bin_l * p.p1165);
        let assign2150_e3004: f64 = (p.p1164 + assign2150_e3003);
        let assign2150_e3007: f64 = (locals.var_bin_w * p.p1166);
        let assign2150_e3008: f64 = (assign2150_e3004 + assign2150_e3007);
        let assign2150_e3011: f64 = (locals.var_bin_wl * p.p1167);
        let assign2150_e3012: f64 = (assign2150_e3008 + assign2150_e3011);
        locals.var_cdscedge_i = assign2150_e3012;
        locals.var_cdscedge_i_rv = 0.0;

        let assign2160_e3016: f64 = (locals.var_bin_l * p.p1192);
        let assign2160_e3017: f64 = (p.p1191 + assign2160_e3016);
        let assign2160_e3020: f64 = (locals.var_bin_w * p.p1193);
        let assign2160_e3021: f64 = (assign2160_e3017 + assign2160_e3020);
        let assign2160_e3024: f64 = (locals.var_bin_wl * p.p1194);
        let assign2160_e3025: f64 = (assign2160_e3021 + assign2160_e3024);
        locals.var_cbcbedge_i = assign2160_e3025;
        locals.var_cbcbedge_i_rv = 0.0;

        let assign2170_e3029: f64 = (locals.var_bin_l * p.p291);
        let assign2170_e3030: f64 = (p.p288 + assign2170_e3029);
        let assign2170_e3033: f64 = (locals.var_bin_w * p.p292);
        let assign2170_e3034: f64 = (assign2170_e3030 + assign2170_e3033);
        let assign2170_e3037: f64 = (locals.var_bin_wl * p.p293);
        let assign2170_e3038: f64 = (assign2170_e3034 + assign2170_e3037);
        locals.var_cdscb_i = assign2170_e3038;
        locals.var_cdscb_i_rv = 0.0;

        let assign2180_e3042: f64 = (locals.var_bin_l * p.p271);
        let assign2180_e3043: f64 = (p.p270 + assign2180_e3042);
        let assign2180_e3046: f64 = (locals.var_bin_w * p.p272);
        let assign2180_e3047: f64 = (assign2180_e3043 + assign2180_e3046);
        let assign2180_e3050: f64 = (locals.var_bin_wl * p.p273);
        let assign2180_e3051: f64 = (assign2180_e3047 + assign2180_e3050);
        locals.var_csecse_i = assign2180_e3051;
        locals.var_csecse_i_rv = 0.0;

        let assign2190_e3055: f64 = (locals.var_bin_l * p.p1177);
        let assign2190_e3056: f64 = (p.p1176 + assign2190_e3055);
        let assign2190_e3059: f64 = (locals.var_bin_w * p.p1178);
        let assign2190_e3060: f64 = (assign2190_e3056 + assign2190_e3059);
        let assign2190_e3063: f64 = (locals.var_bin_wl * p.p1179);
        let assign2190_e3064: f64 = (assign2190_e3060 + assign2190_e3063);
        locals.var_csecseedge_i = assign2190_e3064;
        locals.var_csecseedge_i_rv = 0.0;

        let assign2200_e3068: f64 = (locals.var_bin_l * p.p276);
        let assign2200_e3069: f64 = (p.p275 + assign2200_e3068);
        let assign2200_e3072: f64 = (locals.var_bin_w * p.p277);
        let assign2200_e3073: f64 = (assign2200_e3069 + assign2200_e3072);
        let assign2200_e3076: f64 = (locals.var_bin_wl * p.p278);
        let assign2200_e3077: f64 = (assign2200_e3073 + assign2200_e3076);
        locals.var_cbcb_i = assign2200_e3077;
        locals.var_cbcb_i_rv = 0.0;

        let assign2210_e3081: f64 = (locals.var_bin_l * p.p147);
        let assign2210_e3082: f64 = (p.p146 + assign2210_e3081);
        let assign2210_e3085: f64 = (locals.var_bin_w * p.p148);
        let assign2210_e3086: f64 = (assign2210_e3082 + assign2210_e3085);
        let assign2210_e3089: f64 = (locals.var_bin_wl * p.p149);
        let assign2210_e3090: f64 = (assign2210_e3086 + assign2210_e3089);
        locals.var_dvtp0_i = assign2210_e3090;
        locals.var_dvtp0_i_rv = 0.0;

        let assign2220_e3094: f64 = (locals.var_bin_l * p.p1239);
        let assign2220_e3095: f64 = (p.p1238 + assign2220_e3094);
        let assign2220_e3098: f64 = (locals.var_bin_w * p.p1240);
        let assign2220_e3099: f64 = (assign2220_e3095 + assign2220_e3098);
        let assign2220_e3102: f64 = (locals.var_bin_wl * p.p1241);
        let assign2220_e3103: f64 = (assign2220_e3099 + assign2220_e3102);
        locals.var_dvtp0edge_i = assign2220_e3103;
        locals.var_dvtp0edge_i_rv = 0.0;

        let assign2230_e3107: f64 = (locals.var_bin_l * p.p151);
        let assign2230_e3108: f64 = (p.p150 + assign2230_e3107);
        let assign2230_e3111: f64 = (locals.var_bin_w * p.p152);
        let assign2230_e3112: f64 = (assign2230_e3108 + assign2230_e3111);
        let assign2230_e3115: f64 = (locals.var_bin_wl * p.p153);
        let assign2230_e3116: f64 = (assign2230_e3112 + assign2230_e3115);
        locals.var_dvtp1_i = assign2230_e3116;
        locals.var_dvtp1_i_rv = 0.0;

        let assign2240_e3120: f64 = (locals.var_bin_l * p.p1243);
        let assign2240_e3121: f64 = (p.p1242 + assign2240_e3120);
        let assign2240_e3124: f64 = (locals.var_bin_w * p.p1244);
        let assign2240_e3125: f64 = (assign2240_e3121 + assign2240_e3124);
        let assign2240_e3128: f64 = (locals.var_bin_wl * p.p1245);
        let assign2240_e3129: f64 = (assign2240_e3125 + assign2240_e3128);
        locals.var_dvtp1edge_i = assign2240_e3129;
        locals.var_dvtp1edge_i_rv = 0.0;

        let assign2250_e3133: f64 = (locals.var_bin_l * p.p155);
        let assign2250_e3134: f64 = (p.p154 + assign2250_e3133);
        let assign2250_e3137: f64 = (locals.var_bin_w * p.p156);
        let assign2250_e3138: f64 = (assign2250_e3134 + assign2250_e3137);
        let assign2250_e3141: f64 = (locals.var_bin_wl * p.p157);
        let assign2250_e3142: f64 = (assign2250_e3138 + assign2250_e3141);
        locals.var_dvtp2_i = assign2250_e3142;
        locals.var_dvtp2_i_rv = 0.0;

        let assign2260_e3146: f64 = (locals.var_bin_l * p.p159);
        let assign2260_e3147: f64 = (p.p158 + assign2260_e3146);
        let assign2260_e3150: f64 = (locals.var_bin_w * p.p160);
        let assign2260_e3151: f64 = (assign2260_e3147 + assign2260_e3150);
        let assign2260_e3154: f64 = (locals.var_bin_wl * p.p161);
        let assign2260_e3155: f64 = (assign2260_e3151 + assign2260_e3154);
        locals.var_dvtp3_i = assign2260_e3155;
        locals.var_dvtp3_i_rv = 0.0;

        let assign2270_e3159: f64 = (locals.var_bin_l * p.p163);
        let assign2270_e3160: f64 = (p.p162 + assign2270_e3159);
        let assign2270_e3163: f64 = (locals.var_bin_w * p.p164);
        let assign2270_e3164: f64 = (assign2270_e3160 + assign2270_e3163);
        let assign2270_e3167: f64 = (locals.var_bin_wl * p.p165);
        let assign2270_e3168: f64 = (assign2270_e3164 + assign2270_e3167);
        locals.var_dvtp4_i = assign2270_e3168;
        locals.var_dvtp4_i_rv = 0.0;

        let assign2280_e3172: f64 = (locals.var_bin_l * p.p167);
        let assign2280_e3173: f64 = (p.p166 + assign2280_e3172);
        let assign2280_e3176: f64 = (locals.var_bin_w * p.p168);
        let assign2280_e3177: f64 = (assign2280_e3173 + assign2280_e3176);
        let assign2280_e3180: f64 = (locals.var_bin_wl * p.p169);
        let assign2280_e3181: f64 = (assign2280_e3177 + assign2280_e3180);
        locals.var_dvtp5_i = assign2280_e3181;
        locals.var_dvtp5_i_rv = 0.0;

        let assign2290_e3185: f64 = (locals.var_bin_l * p.p1247);
        let assign2290_e3186: f64 = (p.p1246 + assign2290_e3185);
        let assign2290_e3189: f64 = (locals.var_bin_w * p.p1248);
        let assign2290_e3190: f64 = (assign2290_e3186 + assign2290_e3189);
        let assign2290_e3193: f64 = (locals.var_bin_wl * p.p1249);
        let assign2290_e3194: f64 = (assign2290_e3190 + assign2290_e3193);
        locals.var_dvtp2edge_i = assign2290_e3194;
        locals.var_dvtp2edge_i_rv = 0.0;

        let assign2300_e3198: f64 = (locals.var_bin_l * p.p1251);
        let assign2300_e3199: f64 = (p.p1250 + assign2300_e3198);
        let assign2300_e3202: f64 = (locals.var_bin_w * p.p1252);
        let assign2300_e3203: f64 = (assign2300_e3199 + assign2300_e3202);
        let assign2300_e3206: f64 = (locals.var_bin_wl * p.p1253);
        let assign2300_e3207: f64 = (assign2300_e3203 + assign2300_e3206);
        locals.var_dvtp3edge_i = assign2300_e3207;
        locals.var_dvtp3edge_i_rv = 0.0;

        let assign2310_e3211: f64 = (locals.var_bin_l * p.p1255);
        let assign2310_e3212: f64 = (p.p1254 + assign2310_e3211);
        let assign2310_e3215: f64 = (locals.var_bin_w * p.p1256);
        let assign2310_e3216: f64 = (assign2310_e3212 + assign2310_e3215);
        let assign2310_e3219: f64 = (locals.var_bin_wl * p.p1257);
        let assign2310_e3220: f64 = (assign2310_e3216 + assign2310_e3219);
        locals.var_dvtp4edge_i = assign2310_e3220;
        locals.var_dvtp4edge_i_rv = 0.0;

        let assign2320_e3224: f64 = (locals.var_bin_l * p.p1259);
        let assign2320_e3225: f64 = (p.p1258 + assign2320_e3224);
        let assign2320_e3228: f64 = (locals.var_bin_w * p.p1260);
        let assign2320_e3229: f64 = (assign2320_e3225 + assign2320_e3228);
        let assign2320_e3232: f64 = (locals.var_bin_wl * p.p1261);
        let assign2320_e3233: f64 = (assign2320_e3229 + assign2320_e3232);
        locals.var_dvtp5edge_i = assign2320_e3233;
        locals.var_dvtp5edge_i_rv = 0.0;

        let assign2330_e3237: f64 = (locals.var_bin_l * p.p225);
        let assign2330_e3238: f64 = (p.p218 + assign2330_e3237);
        let assign2330_e3241: f64 = (locals.var_bin_w * p.p226);
        let assign2330_e3242: f64 = (assign2330_e3238 + assign2330_e3241);
        let assign2330_e3245: f64 = (locals.var_bin_wl * p.p227);
        let assign2330_e3246: f64 = (assign2330_e3242 + assign2330_e3245);
        locals.var_k2_i = assign2330_e3246;
        locals.var_k2_i_dn3 = 0.0;
        locals.var_k2_i_dn4 = 0.0;
        locals.var_k2_i_dn5 = 0.0;
        locals.var_k2_i_dn6 = 0.0;
        locals.var_k2_i_dn7 = 0.0;
        locals.var_k2_i_dn8 = 0.0;
        locals.var_k2_i_dn9 = 0.0;
        locals.var_k2_i_dn10 = 0.0;
        locals.var_k2_i_dn11 = 0.0;
        locals.var_k2_i_rv = 0.0;

        let assign2340_e3250: f64 = (locals.var_bin_l * p.p215);
        let assign2340_e3251: f64 = (p.p208 + assign2340_e3250);
        let assign2340_e3254: f64 = (locals.var_bin_w * p.p216);
        let assign2340_e3255: f64 = (assign2340_e3251 + assign2340_e3254);
        let assign2340_e3258: f64 = (locals.var_bin_wl * p.p217);
        let assign2340_e3259: f64 = (assign2340_e3255 + assign2340_e3258);
        locals.var_k1_i = assign2340_e3259;
        locals.var_k1_i_dn3 = 0.0;
        locals.var_k1_i_dn4 = 0.0;
        locals.var_k1_i_dn5 = 0.0;
        locals.var_k1_i_dn6 = 0.0;
        locals.var_k1_i_dn7 = 0.0;
        locals.var_k1_i_dn8 = 0.0;
        locals.var_k1_i_dn9 = 0.0;
        locals.var_k1_i_dn10 = 0.0;
        locals.var_k1_i_dn11 = 0.0;
        locals.var_k1_i_rv = 0.0;

        let assign2350_e3263: f64 = (locals.var_bin_l * p.p1203);
        let assign2350_e3264: f64 = (p.p1196 + assign2350_e3263);
        let assign2350_e3267: f64 = (locals.var_bin_w * p.p1204);
        let assign2350_e3268: f64 = (assign2350_e3264 + assign2350_e3267);
        let assign2350_e3271: f64 = (locals.var_bin_wl * p.p1205);
        let assign2350_e3272: f64 = (assign2350_e3268 + assign2350_e3271);
        locals.var_k1edge_i = assign2350_e3272;
        locals.var_k1edge_i_dn3 = 0.0;
        locals.var_k1edge_i_dn4 = 0.0;
        locals.var_k1edge_i_dn5 = 0.0;
        locals.var_k1edge_i_dn6 = 0.0;
        locals.var_k1edge_i_dn7 = 0.0;
        locals.var_k1edge_i_dn8 = 0.0;
        locals.var_k1edge_i_dn9 = 0.0;
        locals.var_k1edge_i_dn10 = 0.0;
        locals.var_k1edge_i_dn11 = 0.0;
        locals.var_k1edge_i_rv = 0.0;

        let assign2360_e3276: f64 = (locals.var_bin_l * p.p112);
        let assign2360_e3277: f64 = (p.p111 + assign2360_e3276);
        let assign2360_e3280: f64 = (locals.var_bin_w * p.p113);
        let assign2360_e3281: f64 = (assign2360_e3277 + assign2360_e3280);
        let assign2360_e3284: f64 = (locals.var_bin_wl * p.p114);
        let assign2360_e3285: f64 = (assign2360_e3281 + assign2360_e3284);
        locals.var_xj_i = assign2360_e3285;
        locals.var_xj_i_rv = 0.0;

        let assign2370_e3289: f64 = (locals.var_bin_l * p.p191);
        let assign2370_e3290: f64 = (p.p190 + assign2370_e3289);
        let assign2370_e3293: f64 = (locals.var_bin_w * p.p192);
        let assign2370_e3294: f64 = (assign2370_e3290 + assign2370_e3293);
        let assign2370_e3297: f64 = (locals.var_bin_wl * p.p193);
        let assign2370_e3298: f64 = (assign2370_e3294 + assign2370_e3297);
        locals.var_phin_i = assign2370_e3298;
        locals.var_phin_i_rv = 0.0;

        let assign2380_e3302: f64 = (locals.var_bin_l * p.p195);
        let assign2380_e3303: f64 = (p.p194 + assign2380_e3302);
        let assign2380_e3306: f64 = (locals.var_bin_w * p.p196);
        let assign2380_e3307: f64 = (assign2380_e3303 + assign2380_e3306);
        let assign2380_e3310: f64 = (locals.var_bin_wl * p.p197);
        let assign2380_e3311: f64 = (assign2380_e3307 + assign2380_e3310);
        locals.var_eta0_i = assign2380_e3311;
        locals.var_eta0_i_dn3 = 0.0;
        locals.var_eta0_i_dn4 = 0.0;
        locals.var_eta0_i_dn5 = 0.0;
        locals.var_eta0_i_dn6 = 0.0;
        locals.var_eta0_i_dn7 = 0.0;
        locals.var_eta0_i_dn8 = 0.0;
        locals.var_eta0_i_dn9 = 0.0;
        locals.var_eta0_i_dn10 = 0.0;
        locals.var_eta0_i_dn11 = 0.0;
        locals.var_eta0_i_rv = 0.0;

        let assign2390_e3315: f64 = (locals.var_bin_l * p.p205);
        let assign2390_e3316: f64 = (p.p203 + assign2390_e3315);
        let assign2390_e3319: f64 = (locals.var_bin_w * p.p206);
        let assign2390_e3320: f64 = (assign2390_e3316 + assign2390_e3319);
        let assign2390_e3323: f64 = (locals.var_bin_wl * p.p207);
        let assign2390_e3324: f64 = (assign2390_e3320 + assign2390_e3323);
        locals.var_etab_i = assign2390_e3324;
        locals.var_etab_i_rv = 0.0;

        let assign2400_e3328: f64 = (locals.var_bin_l * p.p310);
        let assign2400_e3329: f64 = (p.p309 + assign2400_e3328);
        let assign2400_e3332: f64 = (locals.var_bin_w * p.p311);
        let assign2400_e3333: f64 = (assign2400_e3329 + assign2400_e3332);
        let assign2400_e3336: f64 = (locals.var_bin_wl * p.p312);
        let assign2400_e3337: f64 = (assign2400_e3333 + assign2400_e3336);
        locals.var_delta_i = assign2400_e3337;
        locals.var_delta_i_dn3 = 0.0;
        locals.var_delta_i_dn4 = 0.0;
        locals.var_delta_i_dn5 = 0.0;
        locals.var_delta_i_dn6 = 0.0;
        locals.var_delta_i_dn7 = 0.0;
        locals.var_delta_i_dn8 = 0.0;
        locals.var_delta_i_dn9 = 0.0;
        locals.var_delta_i_dn10 = 0.0;
        locals.var_delta_i_dn11 = 0.0;
        locals.var_delta_i_rv = 0.0;

        let assign2410_e3341: f64 = (locals.var_bin_l * p.p340);
        let assign2410_e3342: f64 = (p.p337 + assign2410_e3341);
        let assign2410_e3345: f64 = (locals.var_bin_w * p.p341);
        let assign2410_e3346: f64 = (assign2410_e3342 + assign2410_e3345);
        let assign2410_e3349: f64 = (locals.var_bin_wl * p.p342);
        let assign2410_e3350: f64 = (assign2410_e3346 + assign2410_e3349);
        locals.var_u0_i = assign2410_e3350;
        locals.var_u0_i_rv = 0.0;

        let assign2420_e3354: f64 = (locals.var_bin_l * p.p355);
        let assign2420_e3355: f64 = (p.p348 + assign2420_e3354);
        let assign2420_e3358: f64 = (locals.var_bin_w * p.p356);
        let assign2420_e3359: f64 = (assign2420_e3355 + assign2420_e3358);
        let assign2420_e3362: f64 = (locals.var_bin_wl * p.p357);
        let assign2420_e3363: f64 = (assign2420_e3359 + assign2420_e3362);
        locals.var_ua_i = assign2420_e3363;
        locals.var_ua_i_dn3 = 0.0;
        locals.var_ua_i_dn4 = 0.0;
        locals.var_ua_i_dn5 = 0.0;
        locals.var_ua_i_dn6 = 0.0;
        locals.var_ua_i_dn7 = 0.0;
        locals.var_ua_i_dn8 = 0.0;
        locals.var_ua_i_dn9 = 0.0;
        locals.var_ua_i_dn10 = 0.0;
        locals.var_ua_i_dn11 = 0.0;
        locals.var_ua_i_rv = 0.0;

        let assign2430_e3367: f64 = (locals.var_bin_l * p.p375);
        let assign2430_e3368: f64 = (p.p372 + assign2430_e3367);
        let assign2430_e3371: f64 = (locals.var_bin_w * p.p376);
        let assign2430_e3372: f64 = (assign2430_e3368 + assign2430_e3371);
        let assign2430_e3375: f64 = (locals.var_bin_wl * p.p377);
        let assign2430_e3376: f64 = (assign2430_e3372 + assign2430_e3375);
        locals.var_ud_i = assign2430_e3376;
        locals.var_ud_i_dn3 = 0.0;
        locals.var_ud_i_dn4 = 0.0;
        locals.var_ud_i_dn5 = 0.0;
        locals.var_ud_i_dn6 = 0.0;
        locals.var_ud_i_dn7 = 0.0;
        locals.var_ud_i_dn8 = 0.0;
        locals.var_ud_i_dn9 = 0.0;
        locals.var_ud_i_dn10 = 0.0;
        locals.var_ud_i_dn11 = 0.0;
        locals.var_ud_i_rv = 0.0;

        let assign2440_e3380: f64 = (locals.var_bin_l * p.p363);
        let assign2440_e3381: f64 = (p.p362 + assign2440_e3380);
        let assign2440_e3384: f64 = (locals.var_bin_w * p.p364);
        let assign2440_e3385: f64 = (assign2440_e3381 + assign2440_e3384);
        let assign2440_e3388: f64 = (locals.var_bin_wl * p.p365);
        let assign2440_e3389: f64 = (assign2440_e3385 + assign2440_e3388);
        locals.var_eu_i = assign2440_e3389;
        locals.var_eu_i_dn3 = 0.0;
        locals.var_eu_i_dn4 = 0.0;
        locals.var_eu_i_dn5 = 0.0;
        locals.var_eu_i_dn6 = 0.0;
        locals.var_eu_i_dn7 = 0.0;
        locals.var_eu_i_dn8 = 0.0;
        locals.var_eu_i_dn9 = 0.0;
        locals.var_eu_i_dn10 = 0.0;
        locals.var_eu_i_dn11 = 0.0;
        locals.var_eu_i_rv = 0.0;

        let assign2450_e3393: f64 = (locals.var_bin_l * p.p383);
        let assign2450_e3394: f64 = (p.p382 + assign2450_e3393);
        let assign2450_e3397: f64 = (locals.var_bin_w * p.p384);
        let assign2450_e3398: f64 = (assign2450_e3394 + assign2450_e3397);
        let assign2450_e3401: f64 = (locals.var_bin_wl * p.p385);
        let assign2450_e3402: f64 = (assign2450_e3398 + assign2450_e3401);
        locals.var_ucs_i = assign2450_e3402;
        locals.var_ucs_i_rv = 0.0;

        let assign2460_e3406: f64 = (locals.var_bin_l * p.p397);
        let assign2460_e3407: f64 = (p.p390 + assign2460_e3406);
        let assign2460_e3410: f64 = (locals.var_bin_w * p.p398);
        let assign2460_e3411: f64 = (assign2460_e3407 + assign2460_e3410);
        let assign2460_e3414: f64 = (locals.var_bin_wl * p.p399);
        let assign2460_e3415: f64 = (assign2460_e3411 + assign2460_e3414);
        locals.var_uc_i = assign2460_e3415;
        locals.var_uc_i_dn3 = 0.0;
        locals.var_uc_i_dn4 = 0.0;
        locals.var_uc_i_dn5 = 0.0;
        locals.var_uc_i_dn6 = 0.0;
        locals.var_uc_i_dn7 = 0.0;
        locals.var_uc_i_dn8 = 0.0;
        locals.var_uc_i_dn9 = 0.0;
        locals.var_uc_i_dn10 = 0.0;
        locals.var_uc_i_dn11 = 0.0;
        locals.var_uc_i_rv = 0.0;

        let assign2470_e3419: f64 = (locals.var_bin_l * p.p407);
        let assign2470_e3420: f64 = (p.p404 + assign2470_e3419);
        let assign2470_e3423: f64 = (locals.var_bin_w * p.p408);
        let assign2470_e3424: f64 = (assign2470_e3420 + assign2470_e3423);
        let assign2470_e3427: f64 = (locals.var_bin_wl * p.p409);
        let assign2470_e3428: f64 = (assign2470_e3424 + assign2470_e3427);
        locals.var_pclm_i = assign2470_e3428;
        locals.var_pclm_i_dn3 = 0.0;
        locals.var_pclm_i_dn4 = 0.0;
        locals.var_pclm_i_dn5 = 0.0;
        locals.var_pclm_i_dn6 = 0.0;
        locals.var_pclm_i_dn7 = 0.0;
        locals.var_pclm_i_dn8 = 0.0;
        locals.var_pclm_i_dn9 = 0.0;
        locals.var_pclm_i_dn10 = 0.0;
        locals.var_pclm_i_dn11 = 0.0;
        locals.var_pclm_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2480_e3432: f64 = (locals.var_bin_l * p.p418);
        let assign2480_e3433: f64 = (p.p415 + assign2480_e3432);
        let assign2480_e3436: f64 = (locals.var_bin_w * p.p419);
        let assign2480_e3437: f64 = (assign2480_e3433 + assign2480_e3436);
        let assign2480_e3440: f64 = (locals.var_bin_wl * p.p420);
        let assign2480_e3441: f64 = (assign2480_e3437 + assign2480_e3440);
        locals.var_pclmcv_i = assign2480_e3441;
        locals.var_pclmcv_i_rv = 0.0;

        let assign2490_e3445: f64 = (locals.var_bin_l * p.p458);
        let assign2490_e3446: f64 = (p.p457 + assign2490_e3445);
        let assign2490_e3449: f64 = (locals.var_bin_w * p.p459);
        let assign2490_e3450: f64 = (assign2490_e3446 + assign2490_e3449);
        let assign2490_e3453: f64 = (locals.var_bin_wl * p.p460);
        let assign2490_e3454: f64 = (assign2490_e3450 + assign2490_e3453);
        locals.var_rsw_i = assign2490_e3454;
        locals.var_rsw_i_rv = 0.0;

        let assign2500_e3458: f64 = (locals.var_bin_l * p.p468);
        let assign2500_e3459: f64 = (p.p467 + assign2500_e3458);
        let assign2500_e3462: f64 = (locals.var_bin_w * p.p469);
        let assign2500_e3463: f64 = (assign2500_e3459 + assign2500_e3462);
        let assign2500_e3466: f64 = (locals.var_bin_wl * p.p470);
        let assign2500_e3467: f64 = (assign2500_e3463 + assign2500_e3466);
        locals.var_rdw_i = assign2500_e3467;
        locals.var_rdw_i_rv = 0.0;

        let assign2510_e3471: f64 = (locals.var_bin_l * p.p440);
        let assign2510_e3472: f64 = (p.p439 + assign2510_e3471);
        let assign2510_e3475: f64 = (locals.var_bin_w * p.p441);
        let assign2510_e3476: f64 = (assign2510_e3472 + assign2510_e3475);
        let assign2510_e3479: f64 = (locals.var_bin_wl * p.p442);
        let assign2510_e3480: f64 = (assign2510_e3476 + assign2510_e3479);
        locals.var_prwg_i = assign2510_e3480;
        locals.var_prwg_i_rv = 0.0;

        let assign2520_e3484: f64 = (locals.var_bin_l * p.p444);
        let assign2520_e3485: f64 = (p.p443 + assign2520_e3484);
        let assign2520_e3488: f64 = (locals.var_bin_w * p.p445);
        let assign2520_e3489: f64 = (assign2520_e3485 + assign2520_e3488);
        let assign2520_e3492: f64 = (locals.var_bin_wl * p.p446);
        let assign2520_e3493: f64 = (assign2520_e3489 + assign2520_e3492);
        locals.var_prwb_i = assign2520_e3493;
        locals.var_prwb_i_rv = 0.0;

        let assign2530_e3497: f64 = (locals.var_bin_l * p.p450);
        let assign2530_e3498: f64 = (p.p449 + assign2530_e3497);
        let assign2530_e3501: f64 = (locals.var_bin_w * p.p451);
        let assign2530_e3502: f64 = (assign2530_e3498 + assign2530_e3501);
        let assign2530_e3505: f64 = (locals.var_bin_wl * p.p452);
        let assign2530_e3506: f64 = (assign2530_e3502 + assign2530_e3505);
        locals.var_wr_i = assign2530_e3506;
        locals.var_wr_i_rv = 0.0;

        let assign2540_e3510: f64 = (locals.var_bin_l * p.p454);
        let assign2540_e3511: f64 = (p.p453 + assign2540_e3510);
        let assign2540_e3514: f64 = (locals.var_bin_w * p.p455);
        let assign2540_e3515: f64 = (assign2540_e3511 + assign2540_e3514);
        let assign2540_e3518: f64 = (locals.var_bin_wl * p.p456);
        let assign2540_e3519: f64 = (assign2540_e3515 + assign2540_e3518);
        locals.var_rswmin_i = assign2540_e3519;
        locals.var_rswmin_i_rv = 0.0;

        let assign2550_e3523: f64 = (locals.var_bin_l * p.p464);
        let assign2550_e3524: f64 = (p.p463 + assign2550_e3523);
        let assign2550_e3527: f64 = (locals.var_bin_w * p.p465);
        let assign2550_e3528: f64 = (assign2550_e3524 + assign2550_e3527);
        let assign2550_e3531: f64 = (locals.var_bin_wl * p.p466);
        let assign2550_e3532: f64 = (assign2550_e3528 + assign2550_e3531);
        locals.var_rdwmin_i = assign2550_e3532;
        locals.var_rdwmin_i_rv = 0.0;

        let assign2560_e3536: f64 = (locals.var_bin_l * p.p480);
        let assign2560_e3537: f64 = (p.p477 + assign2560_e3536);
        let assign2560_e3540: f64 = (locals.var_bin_w * p.p481);
        let assign2560_e3541: f64 = (assign2560_e3537 + assign2560_e3540);
        let assign2560_e3544: f64 = (locals.var_bin_wl * p.p482);
        let assign2560_e3545: f64 = (assign2560_e3541 + assign2560_e3544);
        locals.var_rdsw_i = assign2560_e3545;
        locals.var_rdsw_i_rv = 0.0;

        let assign2570_e3549: f64 = (locals.var_bin_l * p.p474);
        let assign2570_e3550: f64 = (p.p473 + assign2570_e3549);
        let assign2570_e3553: f64 = (locals.var_bin_w * p.p475);
        let assign2570_e3554: f64 = (assign2570_e3550 + assign2570_e3553);
        let assign2570_e3557: f64 = (locals.var_bin_wl * p.p476);
        let assign2570_e3558: f64 = (assign2570_e3554 + assign2570_e3557);
        locals.var_rdswmin_i = assign2570_e3558;
        locals.var_rdswmin_i_rv = 0.0;

        let assign2580_e3562: f64 = (locals.var_bin_l * p.p499);
        let assign2580_e3563: f64 = (p.p498 + assign2580_e3562);
        let assign2580_e3566: f64 = (locals.var_bin_w * p.p500);
        let assign2580_e3567: f64 = (assign2580_e3563 + assign2580_e3566);
        let assign2580_e3570: f64 = (locals.var_bin_wl * p.p501);
        let assign2580_e3571: f64 = (assign2580_e3567 + assign2580_e3570);
        locals.var_ptwg_i = assign2580_e3571;
        locals.var_ptwg_i_dn3 = 0.0;
        locals.var_ptwg_i_dn4 = 0.0;
        locals.var_ptwg_i_dn5 = 0.0;
        locals.var_ptwg_i_dn6 = 0.0;
        locals.var_ptwg_i_dn7 = 0.0;
        locals.var_ptwg_i_dn8 = 0.0;
        locals.var_ptwg_i_dn9 = 0.0;
        locals.var_ptwg_i_dn10 = 0.0;
        locals.var_ptwg_i_dn11 = 0.0;
        locals.var_ptwg_i_rv = 0.0;

        let assign2590_e3575: f64 = (locals.var_bin_l * p.p533);
        let assign2590_e3576: f64 = (p.p530 + assign2590_e3575);
        let assign2590_e3579: f64 = (locals.var_bin_w * p.p534);
        let assign2590_e3580: f64 = (assign2590_e3576 + assign2590_e3579);
        let assign2590_e3583: f64 = (locals.var_bin_wl * p.p535);
        let assign2590_e3584: f64 = (assign2590_e3580 + assign2590_e3583);
        locals.var_pdiblc_i = assign2590_e3584;
        locals.var_pdiblc_i_dn3 = 0.0;
        locals.var_pdiblc_i_dn4 = 0.0;
        locals.var_pdiblc_i_dn5 = 0.0;
        locals.var_pdiblc_i_dn6 = 0.0;
        locals.var_pdiblc_i_dn7 = 0.0;
        locals.var_pdiblc_i_dn8 = 0.0;
        locals.var_pdiblc_i_dn9 = 0.0;
        locals.var_pdiblc_i_dn10 = 0.0;
        locals.var_pdiblc_i_dn11 = 0.0;
        locals.var_pdiblc_i_rv = 0.0;

        let assign2600_e3588: f64 = (locals.var_bin_l * p.p541);
        let assign2600_e3589: f64 = (p.p540 + assign2600_e3588);
        let assign2600_e3592: f64 = (locals.var_bin_w * p.p542);
        let assign2600_e3593: f64 = (assign2600_e3589 + assign2600_e3592);
        let assign2600_e3596: f64 = (locals.var_bin_wl * p.p543);
        let assign2600_e3597: f64 = (assign2600_e3593 + assign2600_e3596);
        locals.var_pdiblcb_i = assign2600_e3597;
        locals.var_pdiblcb_i_rv = 0.0;

        let assign2610_e3601: f64 = (locals.var_bin_l * p.p422);
        let assign2610_e3602: f64 = (p.p421 + assign2610_e3601);
        let assign2610_e3605: f64 = (locals.var_bin_w * p.p423);
        let assign2610_e3606: f64 = (assign2610_e3602 + assign2610_e3605);
        let assign2610_e3609: f64 = (locals.var_bin_wl * p.p424);
        let assign2610_e3610: f64 = (assign2610_e3606 + assign2610_e3609);
        locals.var_pscbe1_i = assign2610_e3610;
        locals.var_pscbe1_i_rv = 0.0;

        let assign2620_e3614: f64 = (locals.var_bin_l * p.p426);
        let assign2620_e3615: f64 = (p.p425 + assign2620_e3614);
        let assign2620_e3618: f64 = (locals.var_bin_w * p.p427);
        let assign2620_e3619: f64 = (assign2620_e3615 + assign2620_e3618);
        let assign2620_e3622: f64 = (locals.var_bin_wl * p.p428);
        let assign2620_e3623: f64 = (assign2620_e3619 + assign2620_e3622);
        locals.var_pscbe2_i = assign2620_e3623;
        locals.var_pscbe2_i_rv = 0.0;

        let assign2630_e3627: f64 = (locals.var_bin_l * p.p430);
        let assign2630_e3628: f64 = (p.p429 + assign2630_e3627);
        let assign2630_e3631: f64 = (locals.var_bin_w * p.p431);
        let assign2630_e3632: f64 = (assign2630_e3628 + assign2630_e3631);
        let assign2630_e3635: f64 = (locals.var_bin_wl * p.p432);
        let assign2630_e3636: f64 = (assign2630_e3632 + assign2630_e3635);
        locals.var_pdits_i = assign2630_e3636;
        locals.var_pdits_i_rv = 0.0;

        let assign2640_e3640: f64 = (locals.var_bin_l * p.p435);
        let assign2640_e3641: f64 = (p.p434 + assign2640_e3640);
        let assign2640_e3644: f64 = (locals.var_bin_w * p.p436);
        let assign2640_e3645: f64 = (assign2640_e3641 + assign2640_e3644);
        let assign2640_e3648: f64 = (locals.var_bin_wl * p.p437);
        let assign2640_e3649: f64 = (assign2640_e3645 + assign2640_e3648);
        locals.var_pditsd_i = assign2640_e3649;
        locals.var_pditsd_i_rv = 0.0;

        let assign2650_e3653: f64 = (locals.var_bin_l * p.p551);
        let assign2650_e3654: f64 = (p.p548 + assign2650_e3653);
        let assign2650_e3657: f64 = (locals.var_bin_w * p.p552);
        let assign2650_e3658: f64 = (assign2650_e3654 + assign2650_e3657);
        let assign2650_e3661: f64 = (locals.var_bin_wl * p.p553);
        let assign2650_e3662: f64 = (assign2650_e3658 + assign2650_e3661);
        locals.var_fprout_i = assign2650_e3662;
        locals.var_fprout_i_rv = 0.0;

        let assign2660_e3666: f64 = (locals.var_bin_l * p.p545);
        let assign2660_e3667: f64 = (p.p544 + assign2660_e3666);
        let assign2660_e3670: f64 = (locals.var_bin_w * p.p546);
        let assign2660_e3671: f64 = (assign2660_e3667 + assign2660_e3670);
        let assign2660_e3674: f64 = (locals.var_bin_wl * p.p547);
        let assign2660_e3675: f64 = (assign2660_e3671 + assign2660_e3674);
        locals.var_pvag_i = assign2660_e3675;
        locals.var_pvag_i_rv = 0.0;

        let assign2670_e3679: f64 = (locals.var_bin_l * p.p296);
        let assign2670_e3680: f64 = (p.p295 + assign2670_e3679);
        let assign2670_e3683: f64 = (locals.var_bin_w * p.p297);
        let assign2670_e3684: f64 = (assign2670_e3680 + assign2670_e3683);
        let assign2670_e3687: f64 = (locals.var_bin_wl * p.p298);
        let assign2670_e3688: f64 = (assign2670_e3684 + assign2670_e3687);
        locals.var_vsat_i = assign2670_e3688;
        locals.var_vsat_i_dn3 = 0.0;
        locals.var_vsat_i_dn4 = 0.0;
        locals.var_vsat_i_dn5 = 0.0;
        locals.var_vsat_i_dn6 = 0.0;
        locals.var_vsat_i_dn7 = 0.0;
        locals.var_vsat_i_dn8 = 0.0;
        locals.var_vsat_i_dn9 = 0.0;
        locals.var_vsat_i_dn10 = 0.0;
        locals.var_vsat_i_dn11 = 0.0;
        locals.var_vsat_i_rv = 0.0;

        let assign2680_e3692: f64 = (locals.var_bin_l * p.p511);
        let assign2680_e3693: f64 = (p.p510 + assign2680_e3692);
        let assign2680_e3696: f64 = (locals.var_bin_w * p.p512);
        let assign2680_e3697: f64 = (assign2680_e3693 + assign2680_e3696);
        let assign2680_e3700: f64 = (locals.var_bin_wl * p.p513);
        let assign2680_e3701: f64 = (assign2680_e3697 + assign2680_e3700);
        locals.var_ksativ_i = assign2680_e3701;
        locals.var_ksativ_i_rv = 0.0;

        let assign2690_e3705: f64 = (locals.var_bin_l * p.p326);
        let assign2690_e3706: f64 = (p.p325 + assign2690_e3705);
        let assign2690_e3709: f64 = (locals.var_bin_w * p.p327);
        let assign2690_e3710: f64 = (assign2690_e3706 + assign2690_e3709);
        let assign2690_e3713: f64 = (locals.var_bin_wl * p.p328);
        let assign2690_e3714: f64 = (assign2690_e3710 + assign2690_e3713);
        locals.var_thesat_i = assign2690_e3714;
        locals.var_thesat_i_rv = 0.0;

        let assign2700_e3718: f64 = (p.p330 * locals.var_bin_l);
        let assign2700_e3719: f64 = (p.p329 + assign2700_e3718);
        let assign2700_e3722: f64 = (p.p331 * locals.var_bin_w);
        let assign2700_e3723: f64 = (assign2700_e3719 + assign2700_e3722);
        let assign2700_e3726: f64 = (p.p332 * locals.var_bin_wl);
        let assign2700_e3727: f64 = (assign2700_e3723 + assign2700_e3726);
        locals.var_lpe1_i = assign2700_e3727;
        locals.var_lpe1_i_rv = 0.0;

        let assign2710_e3731: f64 = (locals.var_bin_l * p.p484);
        let assign2710_e3732: f64 = (p.p483 + assign2710_e3731);
        let assign2710_e3735: f64 = (locals.var_bin_w * p.p485);
        let assign2710_e3736: f64 = (assign2710_e3732 + assign2710_e3735);
        let assign2710_e3739: f64 = (locals.var_bin_wl * p.p486);
        let assign2710_e3740: f64 = (assign2710_e3736 + assign2710_e3739);
        locals.var_psat_i = assign2710_e3740;
        locals.var_psat_i_rv = 0.0;

        let assign2720_e3744: f64 = (locals.var_bin_l * p.p316);
        let assign2720_e3745: f64 = (p.p315 + assign2720_e3744);
        let assign2720_e3748: f64 = (locals.var_bin_w * p.p317);
        let assign2720_e3749: f64 = (assign2720_e3745 + assign2720_e3748);
        let assign2720_e3752: f64 = (locals.var_bin_wl * p.p318);
        let assign2720_e3753: f64 = (assign2720_e3749 + assign2720_e3752);
        locals.var_vsatcv_i = assign2720_e3753;
        locals.var_vsatcv_i_dn3 = 0.0;
        locals.var_vsatcv_i_dn4 = 0.0;
        locals.var_vsatcv_i_dn5 = 0.0;
        locals.var_vsatcv_i_dn6 = 0.0;
        locals.var_vsatcv_i_dn7 = 0.0;
        locals.var_vsatcv_i_dn8 = 0.0;
        locals.var_vsatcv_i_dn9 = 0.0;
        locals.var_vsatcv_i_dn10 = 0.0;
        locals.var_vsatcv_i_dn11 = 0.0;
        locals.var_vsatcv_i_rv = 0.0;

        let assign2730_e3757: f64 = (locals.var_bin_l * p.p868);
        let assign2730_e3758: f64 = (p.p867 + assign2730_e3757);
        let assign2730_e3761: f64 = (locals.var_bin_w * p.p869);
        let assign2730_e3762: f64 = (assign2730_e3758 + assign2730_e3761);
        let assign2730_e3765: f64 = (locals.var_bin_wl * p.p870);
        let assign2730_e3766: f64 = (assign2730_e3762 + assign2730_e3765);
        locals.var_cf_i = assign2730_e3766;
        locals.var_cf_i_rv = 0.0;

        let assign2740_e3770: f64 = (locals.var_bin_l * p.p876);
        let assign2740_e3771: f64 = (p.p875 + assign2740_e3770);
        let assign2740_e3774: f64 = (locals.var_bin_w * p.p877);
        let assign2740_e3775: f64 = (assign2740_e3771 + assign2740_e3774);
        let assign2740_e3778: f64 = (locals.var_bin_wl * p.p878);
        let assign2740_e3779: f64 = (assign2740_e3775 + assign2740_e3778);
        locals.var_cgsl_i = assign2740_e3779;
        locals.var_cgsl_i_rv = 0.0;

        let assign2750_e3783: f64 = (locals.var_bin_l * p.p880);
        let assign2750_e3784: f64 = (p.p879 + assign2750_e3783);
        let assign2750_e3787: f64 = (locals.var_bin_w * p.p881);
        let assign2750_e3788: f64 = (assign2750_e3784 + assign2750_e3787);
        let assign2750_e3791: f64 = (locals.var_bin_wl * p.p882);
        let assign2750_e3792: f64 = (assign2750_e3788 + assign2750_e3791);
        locals.var_cgdl_i = assign2750_e3792;
        locals.var_cgdl_i_rv = 0.0;

        let assign2760_e3796: f64 = (locals.var_bin_l * p.p884);
        let assign2760_e3797: f64 = (p.p883 + assign2760_e3796);
        let assign2760_e3800: f64 = (locals.var_bin_w * p.p885);
        let assign2760_e3801: f64 = (assign2760_e3797 + assign2760_e3800);
        let assign2760_e3804: f64 = (locals.var_bin_wl * p.p886);
        let assign2760_e3805: f64 = (assign2760_e3801 + assign2760_e3804);
        locals.var_ckappas_i = assign2760_e3805;
        locals.var_ckappas_i_rv = 0.0;

        let assign2770_e3809: f64 = (locals.var_bin_l * p.p888);
        let assign2770_e3810: f64 = (p.p887 + assign2770_e3809);
        let assign2770_e3813: f64 = (locals.var_bin_w * p.p889);
        let assign2770_e3814: f64 = (assign2770_e3810 + assign2770_e3813);
        let assign2770_e3817: f64 = (locals.var_bin_wl * p.p890);
        let assign2770_e3818: f64 = (assign2770_e3814 + assign2770_e3817);
        locals.var_ckappad_i = assign2770_e3818;
        locals.var_ckappad_i_rv = 0.0;

        let assign2780_e3822: f64 = (locals.var_bin_l * p.p604);
        let assign2780_e3823: f64 = (p.p601 + assign2780_e3822);
        let assign2780_e3826: f64 = (locals.var_bin_w * p.p605);
        let assign2780_e3827: f64 = (assign2780_e3823 + assign2780_e3826);
        let assign2780_e3830: f64 = (locals.var_bin_wl * p.p606);
        let assign2780_e3831: f64 = (assign2780_e3827 + assign2780_e3830);
        locals.var_alpha0_i = assign2780_e3831;
        locals.var_alpha0_i_rv = 0.0;

        let assign2790_e3835: f64 = (locals.var_bin_l * p.p608);
        let assign2790_e3836: f64 = (p.p607 + assign2790_e3835);
        let assign2790_e3839: f64 = (locals.var_bin_w * p.p609);
        let assign2790_e3840: f64 = (assign2790_e3836 + assign2790_e3839);
        let assign2790_e3843: f64 = (locals.var_bin_wl * p.p610);
        let assign2790_e3844: f64 = (assign2790_e3840 + assign2790_e3843);
        locals.var_beta0_i = assign2790_e3844;
        locals.var_beta0_i_rv = 0.0;

        let assign2800_e3848: f64 = (locals.var_bin_l * p.p612);
        let assign2800_e3849: f64 = (p.p611 + assign2800_e3848);
        let assign2800_e3852: f64 = (locals.var_bin_w * p.p613);
        let assign2800_e3853: f64 = (assign2800_e3849 + assign2800_e3852);
        let assign2800_e3856: f64 = (locals.var_bin_wl * p.p614);
        let assign2800_e3857: f64 = (assign2800_e3853 + assign2800_e3856);
        locals.var_beta1_i = assign2800_e3857;
        locals.var_beta1_i_rv = 0.0;

        let assign2810_e3861: f64 = (locals.var_bin_l * p.p616);
        let assign2810_e3862: f64 = (p.p615 + assign2810_e3861);
        let assign2810_e3865: f64 = (locals.var_bin_w * p.p617);
        let assign2810_e3866: f64 = (assign2810_e3862 + assign2810_e3865);
        let assign2810_e3869: f64 = (locals.var_bin_wl * p.p618);
        let assign2810_e3870: f64 = (assign2810_e3866 + assign2810_e3869);
        locals.var_beta2_i = assign2810_e3870;
        locals.var_beta2_i_rv = 0.0;

        let assign2820_e3874: f64 = (locals.var_bin_l * p.p620);
        let assign2820_e3875: f64 = (p.p619 + assign2820_e3874);
        let assign2820_e3878: f64 = (locals.var_bin_w * p.p621);
        let assign2820_e3879: f64 = (assign2820_e3875 + assign2820_e3878);
        let assign2820_e3882: f64 = (locals.var_bin_wl * p.p622);
        let assign2820_e3883: f64 = (assign2820_e3879 + assign2820_e3882);
        locals.var_lii_i = assign2820_e3883;
        locals.var_lii_i_rv = 0.0;

        let assign2830_e3887: f64 = (locals.var_bin_l * p.p624);
        let assign2830_e3888: f64 = (p.p623 + assign2830_e3887);
        let assign2830_e3891: f64 = (locals.var_bin_w * p.p625);
        let assign2830_e3892: f64 = (assign2830_e3888 + assign2830_e3891);
        let assign2830_e3895: f64 = (locals.var_bin_wl * p.p626);
        let assign2830_e3896: f64 = (assign2830_e3892 + assign2830_e3895);
        locals.var_sii0_i = assign2830_e3896;
        locals.var_sii0_i_rv = 0.0;

        let assign2840_e3900: f64 = (locals.var_bin_l * p.p628);
        let assign2840_e3901: f64 = (p.p627 + assign2840_e3900);
        let assign2840_e3904: f64 = (locals.var_bin_w * p.p629);
        let assign2840_e3905: f64 = (assign2840_e3901 + assign2840_e3904);
        let assign2840_e3908: f64 = (locals.var_bin_wl * p.p630);
        let assign2840_e3909: f64 = (assign2840_e3905 + assign2840_e3908);
        locals.var_sii1_i = assign2840_e3909;
        locals.var_sii1_i_rv = 0.0;

        let assign2850_e3913: f64 = (locals.var_bin_l * p.p632);
        let assign2850_e3914: f64 = (p.p631 + assign2850_e3913);
        let assign2850_e3917: f64 = (locals.var_bin_w * p.p633);
        let assign2850_e3918: f64 = (assign2850_e3914 + assign2850_e3917);
        let assign2850_e3921: f64 = (locals.var_bin_wl * p.p634);
        let assign2850_e3922: f64 = (assign2850_e3918 + assign2850_e3921);
        locals.var_sii2_i = assign2850_e3922;
        locals.var_sii2_i_rv = 0.0;

        let assign2860_e3926: f64 = (locals.var_bin_l * p.p636);
        let assign2860_e3927: f64 = (p.p635 + assign2860_e3926);
        let assign2860_e3930: f64 = (locals.var_bin_w * p.p637);
        let assign2860_e3931: f64 = (assign2860_e3927 + assign2860_e3930);
        let assign2860_e3934: f64 = (locals.var_bin_wl * p.p638);
        let assign2860_e3935: f64 = (assign2860_e3931 + assign2860_e3934);
        locals.var_siid_i = assign2860_e3935;
        locals.var_siid_i_rv = 0.0;

        let assign2870_e3939: f64 = (p.p597 * locals.var_bin_l);
        let assign2870_e3940: f64 = (p.p596 + assign2870_e3939);
        let assign2870_e3943: f64 = (p.p598 * locals.var_bin_w);
        let assign2870_e3944: f64 = (assign2870_e3940 + assign2870_e3943);
        let assign2870_e3947: f64 = (p.p599 * locals.var_bin_wl);
        let assign2870_e3948: f64 = (assign2870_e3944 + assign2870_e3947);
        locals.var_vdsatii0_i = assign2870_e3948;
        locals.var_vdsatii0_i_rv = 0.0;

        let assign2880_e3952: f64 = (locals.var_bin_l * p.p640);
        let assign2880_e3953: f64 = (p.p639 + assign2880_e3952);
        let assign2880_e3956: f64 = (locals.var_bin_w * p.p641);
        let assign2880_e3957: f64 = (assign2880_e3953 + assign2880_e3956);
        let assign2880_e3960: f64 = (locals.var_bin_wl * p.p642);
        let assign2880_e3961: f64 = (assign2880_e3957 + assign2880_e3960);
        locals.var_esatii_i = assign2880_e3961;
        locals.var_esatii_i_rv = 0.0;

        let assign2900_e3978: f64 = (locals.var_bin_l * p.p655);
        let assign2900_e3979: f64 = (p.p650 + assign2900_e3978);
        let assign2900_e3982: f64 = (locals.var_bin_w * p.p658);
        let assign2900_e3983: f64 = (assign2900_e3979 + assign2900_e3982);
        let assign2900_e3986: f64 = (locals.var_bin_wl * p.p661);
        let assign2900_e3987: f64 = (assign2900_e3983 + assign2900_e3986);
        locals.var_ebjtii_i = assign2900_e3987;
        locals.var_ebjtii_i_rv = 0.0;

        let assign2910_e3991: f64 = (locals.var_bin_l * p.p654);
        let assign2910_e3992: f64 = (p.p651 + assign2910_e3991);
        let assign2910_e3995: f64 = (locals.var_bin_w * p.p657);
        let assign2910_e3996: f64 = (assign2910_e3992 + assign2910_e3995);
        let assign2910_e3999: f64 = (locals.var_bin_wl * p.p660);
        let assign2910_e4000: f64 = (assign2910_e3996 + assign2910_e3999);
        locals.var_cbjtii_i = assign2910_e4000;
        locals.var_cbjtii_i_rv = 0.0;

        let assign2920_e4004: f64 = (locals.var_bin_l * p.p653);
        let assign2920_e4005: f64 = (p.p652 + assign2920_e4004);
        let assign2920_e4008: f64 = (locals.var_bin_w * p.p656);
        let assign2920_e4009: f64 = (assign2920_e4005 + assign2920_e4008);
        let assign2920_e4012: f64 = (locals.var_bin_wl * p.p659);
        let assign2920_e4013: f64 = (assign2920_e4009 + assign2920_e4012);
        locals.var_abjtii_i = assign2920_e4013;
        locals.var_abjtii_i_rv = 0.0;

        let assign2930_e4017: f64 = (locals.var_bin_l * p.p663);
        let assign2930_e4018: f64 = (p.p662 + assign2930_e4017);
        let assign2930_e4021: f64 = (locals.var_bin_w * p.p664);
        let assign2930_e4022: f64 = (assign2930_e4018 + assign2930_e4021);
        let assign2930_e4025: f64 = (locals.var_bin_wl * p.p665);
        let assign2930_e4026: f64 = (assign2930_e4022 + assign2930_e4025);
        locals.var_vbci_i = assign2930_e4026;
        locals.var_vbci_i_rv = 0.0;

        let assign2940_e4030: f64 = (locals.var_bin_l * p.p668);
        let assign2940_e4031: f64 = (p.p667 + assign2940_e4030);
        let assign2940_e4034: f64 = (locals.var_bin_w * p.p669);
        let assign2940_e4035: f64 = (assign2940_e4031 + assign2940_e4034);
        let assign2940_e4038: f64 = (locals.var_bin_wl * p.p670);
        let assign2940_e4039: f64 = (assign2940_e4035 + assign2940_e4038);
        locals.var_mbjtii_i = assign2940_e4039;
        locals.var_mbjtii_i_rv = 0.0;

        let assign2950_e4043: f64 = (locals.var_bin_l * p.p1362);
        let assign2950_e4044: f64 = (p.p1361 + assign2950_e4043);
        let assign2950_e4047: f64 = (locals.var_bin_w * p.p1363);
        let assign2950_e4048: f64 = (assign2950_e4044 + assign2950_e4047);
        let assign2950_e4051: f64 = (locals.var_bin_wl * p.p1364);
        let assign2950_e4052: f64 = (assign2950_e4048 + assign2950_e4051);
        locals.var_ub_i = assign2950_e4052;
        locals.var_ub_i_rv = 0.0;

        let assign2960_e4056: f64 = (locals.var_bin_l * p.p1366);
        let assign2960_e4057: f64 = (p.p1365 + assign2960_e4056);
        let assign2960_e4060: f64 = (locals.var_bin_w * p.p1367);
        let assign2960_e4061: f64 = (assign2960_e4057 + assign2960_e4060);
        let assign2960_e4064: f64 = (locals.var_bin_wl * p.p1368);
        let assign2960_e4065: f64 = (assign2960_e4061 + assign2960_e4064);
        locals.var_ubte_i = assign2960_e4065;
        locals.var_ubte_i_rv = 0.0;

        let assign2970_e4069: f64 = (locals.var_bin_l * p.p1370);
        let assign2970_e4070: f64 = (p.p1369 + assign2970_e4069);
        let assign2970_e4073: f64 = (locals.var_bin_w * p.p1371);
        let assign2970_e4074: f64 = (assign2970_e4070 + assign2970_e4073);
        let assign2970_e4077: f64 = (locals.var_bin_wl * p.p1372);
        let assign2970_e4078: f64 = (assign2970_e4074 + assign2970_e4077);
        locals.var_neff_i = assign2970_e4078;
        locals.var_neff_i_rv = 0.0;

        let assign2980_e4082: f64 = (p.p929 * locals.var_bin_l);
        let assign2980_e4083: f64 = (p.p928 + assign2980_e4082);
        let assign2980_e4086: f64 = (p.p930 * locals.var_bin_w);
        let assign2980_e4087: f64 = (assign2980_e4083 + assign2980_e4086);
        let assign2980_e4090: f64 = (p.p931 * locals.var_bin_wl);
        let assign2980_e4091: f64 = (assign2980_e4087 + assign2980_e4090);
        locals.var_xdif_i = assign2980_e4091;
        locals.var_xdif_i_rv = 0.0;

        let assign2990_e4095: f64 = (p.p934 * locals.var_bin_l);
        let assign2990_e4096: f64 = (p.p932 + assign2990_e4095);
        let assign2990_e4099: f64 = (p.p936 * locals.var_bin_w);
        let assign2990_e4100: f64 = (assign2990_e4096 + assign2990_e4099);
        let assign2990_e4103: f64 = (p.p938 * locals.var_bin_wl);
        let assign2990_e4104: f64 = (assign2990_e4100 + assign2990_e4103);
        locals.var_isdif_i = assign2990_e4104;
        locals.var_isdif_i_rv = 0.0;

        let assign3000_e4108: f64 = (p.p935 * locals.var_bin_l);
        let assign3000_e4109: f64 = (p.p933 + assign3000_e4108);
        let assign3000_e4112: f64 = (p.p937 * locals.var_bin_w);
        let assign3000_e4113: f64 = (assign3000_e4109 + assign3000_e4112);
        let assign3000_e4116: f64 = (p.p939 * locals.var_bin_wl);
        let assign3000_e4117: f64 = (assign3000_e4113 + assign3000_e4116);
        locals.var_iddif_i = assign3000_e4117;
        locals.var_iddif_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3010_e4121: f64 = (p.p941 * locals.var_bin_l);
        let assign3010_e4122: f64 = (p.p940 + assign3010_e4121);
        let assign3010_e4125: f64 = (p.p942 * locals.var_bin_w);
        let assign3010_e4126: f64 = (assign3010_e4122 + assign3010_e4125);
        let assign3010_e4129: f64 = (p.p943 * locals.var_bin_wl);
        let assign3010_e4130: f64 = (assign3010_e4126 + assign3010_e4129);
        locals.var_nrecf0_i = assign3010_e4130;
        locals.var_nrecf0_i_rv = 0.0;

        let assign3020_e4134: f64 = (p.p945 * locals.var_bin_l);
        let assign3020_e4135: f64 = (p.p944 + assign3020_e4134);
        let assign3020_e4138: f64 = (p.p946 * locals.var_bin_w);
        let assign3020_e4139: f64 = (assign3020_e4135 + assign3020_e4138);
        let assign3020_e4142: f64 = (p.p947 * locals.var_bin_wl);
        let assign3020_e4143: f64 = (assign3020_e4139 + assign3020_e4142);
        locals.var_nrecr0_i = assign3020_e4143;
        locals.var_nrecr0_i_rv = 0.0;

        let assign3030_e4147: f64 = (p.p949 * locals.var_bin_l);
        let assign3030_e4148: f64 = (p.p948 + assign3030_e4147);
        let assign3030_e4151: f64 = (p.p950 * locals.var_bin_w);
        let assign3030_e4152: f64 = (assign3030_e4148 + assign3030_e4151);
        let assign3030_e4155: f64 = (p.p951 * locals.var_bin_wl);
        let assign3030_e4156: f64 = (assign3030_e4152 + assign3030_e4155);
        locals.var_xrec_i = assign3030_e4156;
        locals.var_xrec_i_rv = 0.0;

        let assign3040_e4160: f64 = (p.p954 * locals.var_bin_l);
        let assign3040_e4161: f64 = (p.p952 + assign3040_e4160);
        let assign3040_e4164: f64 = (p.p956 * locals.var_bin_w);
        let assign3040_e4165: f64 = (assign3040_e4161 + assign3040_e4164);
        let assign3040_e4168: f64 = (p.p958 * locals.var_bin_wl);
        let assign3040_e4169: f64 = (assign3040_e4165 + assign3040_e4168);
        locals.var_isrec_i = assign3040_e4169;
        locals.var_isrec_i_rv = 0.0;

        let assign3050_e4173: f64 = (p.p955 * locals.var_bin_l);
        let assign3050_e4174: f64 = (p.p953 + assign3050_e4173);
        let assign3050_e4177: f64 = (p.p957 * locals.var_bin_w);
        let assign3050_e4178: f64 = (assign3050_e4174 + assign3050_e4177);
        let assign3050_e4181: f64 = (p.p959 * locals.var_bin_wl);
        let assign3050_e4182: f64 = (assign3050_e4178 + assign3050_e4181);
        locals.var_idrec_i = assign3050_e4182;
        locals.var_idrec_i_rv = 0.0;

        let assign3060_e4186: f64 = (p.p962 * locals.var_bin_l);
        let assign3060_e4187: f64 = (p.p960 + assign3060_e4186);
        let assign3060_e4190: f64 = (p.p964 * locals.var_bin_w);
        let assign3060_e4191: f64 = (assign3060_e4187 + assign3060_e4190);
        let assign3060_e4194: f64 = (p.p966 * locals.var_bin_wl);
        let assign3060_e4195: f64 = (assign3060_e4191 + assign3060_e4194);
        locals.var_ntrecf_i = assign3060_e4195;
        locals.var_ntrecf_i_rv = 0.0;

        let assign3070_e4199: f64 = (p.p963 * locals.var_bin_l);
        let assign3070_e4200: f64 = (p.p961 + assign3070_e4199);
        let assign3070_e4203: f64 = (p.p965 * locals.var_bin_w);
        let assign3070_e4204: f64 = (assign3070_e4200 + assign3070_e4203);
        let assign3070_e4207: f64 = (p.p967 * locals.var_bin_wl);
        let assign3070_e4208: f64 = (assign3070_e4204 + assign3070_e4207);
        locals.var_ntrecr_i = assign3070_e4208;
        locals.var_ntrecr_i_rv = 0.0;

        let assign3080_e4212: f64 = (p.p970 * locals.var_bin_l);
        let assign3080_e4213: f64 = (p.p968 + assign3080_e4212);
        let assign3080_e4216: f64 = (p.p972 * locals.var_bin_w);
        let assign3080_e4217: f64 = (assign3080_e4213 + assign3080_e4216);
        let assign3080_e4220: f64 = (p.p974 * locals.var_bin_wl);
        let assign3080_e4221: f64 = (assign3080_e4217 + assign3080_e4220);
        locals.var_istun_i = assign3080_e4221;
        locals.var_istun_i_rv = 0.0;

        let assign3090_e4225: f64 = (p.p971 * locals.var_bin_l);
        let assign3090_e4226: f64 = (p.p969 + assign3090_e4225);
        let assign3090_e4229: f64 = (p.p973 * locals.var_bin_w);
        let assign3090_e4230: f64 = (assign3090_e4226 + assign3090_e4229);
        let assign3090_e4233: f64 = (p.p975 * locals.var_bin_wl);
        let assign3090_e4234: f64 = (assign3090_e4230 + assign3090_e4233);
        locals.var_idtun_i = assign3090_e4234;
        locals.var_idtun_i_rv = 0.0;

        let assign3100_e4238: f64 = (p.p978 * locals.var_bin_l);
        let assign3100_e4239: f64 = (p.p976 + assign3100_e4238);
        let assign3100_e4242: f64 = (p.p980 * locals.var_bin_w);
        let assign3100_e4243: f64 = (assign3100_e4239 + assign3100_e4242);
        let assign3100_e4246: f64 = (p.p982 * locals.var_bin_wl);
        let assign3100_e4247: f64 = (assign3100_e4243 + assign3100_e4246);
        locals.var_xtun_i = assign3100_e4247;
        locals.var_xtun_i_rv = 0.0;

        let assign3110_e4251: f64 = (p.p979 * locals.var_bin_l);
        let assign3110_e4252: f64 = (p.p977 + assign3110_e4251);
        let assign3110_e4255: f64 = (p.p981 * locals.var_bin_w);
        let assign3110_e4256: f64 = (assign3110_e4252 + assign3110_e4255);
        let assign3110_e4259: f64 = (p.p983 * locals.var_bin_wl);
        let assign3110_e4260: f64 = (assign3110_e4256 + assign3110_e4259);
        locals.var_xtund_i = assign3110_e4260;
        locals.var_xtund_i_rv = 0.0;

        let assign3120_e4264: f64 = (p.p986 * locals.var_bin_l);
        let assign3120_e4265: f64 = (p.p984 + assign3120_e4264);
        let assign3120_e4268: f64 = (p.p988 * locals.var_bin_w);
        let assign3120_e4269: f64 = (assign3120_e4265 + assign3120_e4268);
        let assign3120_e4272: f64 = (p.p990 * locals.var_bin_wl);
        let assign3120_e4273: f64 = (assign3120_e4269 + assign3120_e4272);
        locals.var_ntun_i = assign3120_e4273;
        locals.var_ntun_i_rv = 0.0;

        let assign3130_e4277: f64 = (p.p987 * locals.var_bin_l);
        let assign3130_e4278: f64 = (p.p985 + assign3130_e4277);
        let assign3130_e4281: f64 = (p.p989 * locals.var_bin_w);
        let assign3130_e4282: f64 = (assign3130_e4278 + assign3130_e4281);
        let assign3130_e4285: f64 = (p.p991 * locals.var_bin_wl);
        let assign3130_e4286: f64 = (assign3130_e4282 + assign3130_e4285);
        locals.var_ntund_i = assign3130_e4286;
        locals.var_ntund_i_rv = 0.0;

        let assign3140_e4290: f64 = (p.p994 * locals.var_bin_l);
        let assign3140_e4291: f64 = (p.p992 + assign3140_e4290);
        let assign3140_e4294: f64 = (p.p996 * locals.var_bin_w);
        let assign3140_e4295: f64 = (assign3140_e4291 + assign3140_e4294);
        let assign3140_e4298: f64 = (p.p998 * locals.var_bin_wl);
        let assign3140_e4299: f64 = (assign3140_e4295 + assign3140_e4298);
        locals.var_vtun0_i = assign3140_e4299;
        locals.var_vtun0_i_rv = 0.0;

        let assign3150_e4303: f64 = (p.p995 * locals.var_bin_l);
        let assign3150_e4304: f64 = (p.p993 + assign3150_e4303);
        let assign3150_e4307: f64 = (p.p997 * locals.var_bin_w);
        let assign3150_e4308: f64 = (assign3150_e4304 + assign3150_e4307);
        let assign3150_e4311: f64 = (p.p999 * locals.var_bin_wl);
        let assign3150_e4312: f64 = (assign3150_e4308 + assign3150_e4311);
        locals.var_vtun0d_i = assign3150_e4312;
        locals.var_vtun0d_i_rv = 0.0;

        let assign3160_e4316: f64 = (p.p1002 * locals.var_bin_l);
        let assign3160_e4317: f64 = (p.p1000 + assign3160_e4316);
        let assign3160_e4320: f64 = (p.p1004 * locals.var_bin_w);
        let assign3160_e4321: f64 = (assign3160_e4317 + assign3160_e4320);
        let assign3160_e4324: f64 = (p.p1006 * locals.var_bin_wl);
        let assign3160_e4325: f64 = (assign3160_e4321 + assign3160_e4324);
        locals.var_vrec0_i = assign3160_e4325;
        locals.var_vrec0_i_rv = 0.0;

        let assign3170_e4329: f64 = (p.p1003 * locals.var_bin_l);
        let assign3170_e4330: f64 = (p.p1001 + assign3170_e4329);
        let assign3170_e4333: f64 = (p.p1005 * locals.var_bin_w);
        let assign3170_e4334: f64 = (assign3170_e4330 + assign3170_e4333);
        let assign3170_e4337: f64 = (p.p1007 * locals.var_bin_wl);
        let assign3170_e4338: f64 = (assign3170_e4334 + assign3170_e4337);
        locals.var_vrec0d_i = assign3170_e4338;
        locals.var_vrec0d_i_rv = 0.0;

        let assign3180_e4342: f64 = (p.p556 * locals.var_bin_l);
        let assign3180_e4343: f64 = (p.p555 + assign3180_e4342);
        let assign3180_e4346: f64 = (p.p557 * locals.var_bin_w);
        let assign3180_e4347: f64 = (assign3180_e4343 + assign3180_e4346);
        let assign3180_e4350: f64 = (p.p558 * locals.var_bin_wl);
        let assign3180_e4351: f64 = (assign3180_e4347 + assign3180_e4350);
        locals.var_vabjt_i = assign3180_e4351;
        locals.var_vabjt_i_rv = 0.0;

        let assign3190_e4355: f64 = (p.p560 * locals.var_bin_l);
        let assign3190_e4356: f64 = (p.p559 + assign3190_e4355);
        let assign3190_e4359: f64 = (p.p561 * locals.var_bin_w);
        let assign3190_e4360: f64 = (assign3190_e4356 + assign3190_e4359);
        let assign3190_e4363: f64 = (p.p562 * locals.var_bin_wl);
        let assign3190_e4364: f64 = (assign3190_e4360 + assign3190_e4363);
        locals.var_aely_i = assign3190_e4364;
        locals.var_aely_i_rv = 0.0;

        let assign3200_e4368: f64 = (locals.var_bin_l * p.p565);
        let assign3200_e4369: f64 = (p.p563 + assign3200_e4368);
        let assign3200_e4372: f64 = (locals.var_bin_w * p.p567);
        let assign3200_e4373: f64 = (assign3200_e4369 + assign3200_e4372);
        let assign3200_e4376: f64 = (p.p569 * locals.var_bin_wl);
        let assign3200_e4377: f64 = (assign3200_e4373 + assign3200_e4376);
        locals.var_ahli_i = assign3200_e4377;
        locals.var_ahli_i_rv = 0.0;

        let assign3210_e4381: f64 = (locals.var_bin_l * p.p566);
        let assign3210_e4382: f64 = (p.p564 + assign3210_e4381);
        let assign3210_e4385: f64 = (locals.var_bin_w * p.p568);
        let assign3210_e4386: f64 = (assign3210_e4382 + assign3210_e4385);
        let assign3210_e4389: f64 = (p.p570 * locals.var_bin_wl);
        let assign3210_e4390: f64 = (assign3210_e4386 + assign3210_e4389);
        locals.var_ahlid_i = assign3210_e4390;
        locals.var_ahlid_i_rv = 0.0;

        let assign3220_e4394: f64 = (p.p572 * locals.var_bin_l);
        let assign3220_e4395: f64 = (p.p571 + assign3220_e4394);
        let assign3220_e4398: f64 = (p.p573 * locals.var_bin_w);
        let assign3220_e4399: f64 = (assign3220_e4395 + assign3220_e4398);
        let assign3220_e4402: f64 = (p.p574 * locals.var_bin_wl);
        let assign3220_e4403: f64 = (assign3220_e4399 + assign3220_e4402);
        locals.var_xbjt_i = assign3220_e4403;
        locals.var_xbjt_i_rv = 0.0;

        let assign3230_e4407: f64 = (p.p576 * locals.var_bin_l);
        let assign3230_e4408: f64 = (p.p575 + assign3230_e4407);
        let assign3230_e4411: f64 = (p.p577 * locals.var_bin_w);
        let assign3230_e4412: f64 = (assign3230_e4408 + assign3230_e4411);
        let assign3230_e4415: f64 = (p.p578 * locals.var_bin_wl);
        let assign3230_e4416: f64 = (assign3230_e4412 + assign3230_e4415);
        locals.var_ndiode_i = assign3230_e4416;
        locals.var_ndiode_i_rv = 0.0;

        let assign3240_e4420: f64 = (p.p582 * locals.var_bin_l);
        let assign3240_e4421: f64 = (p.p579 + assign3240_e4420);
        let assign3240_e4424: f64 = (p.p581 * locals.var_bin_w);
        let assign3240_e4425: f64 = (assign3240_e4421 + assign3240_e4424);
        let assign3240_e4428: f64 = (p.p580 * locals.var_bin_wl);
        let assign3240_e4429: f64 = (assign3240_e4425 + assign3240_e4428);
        locals.var_isbjt_i = assign3240_e4429;
        locals.var_isbjt_i_rv = 0.0;

        let assign3250_e4433: f64 = (p.p584 * locals.var_bin_l);
        let assign3250_e4434: f64 = (p.p583 + assign3250_e4433);
        let assign3250_e4437: f64 = (p.p585 * locals.var_bin_w);
        let assign3250_e4438: f64 = (assign3250_e4434 + assign3250_e4437);
        let assign3250_e4441: f64 = (p.p586 * locals.var_bin_wl);
        let assign3250_e4442: f64 = (assign3250_e4438 + assign3250_e4441);
        locals.var_idbjt_i = assign3250_e4442;
        locals.var_idbjt_i_rv = 0.0;

        let assign3260_e4446: f64 = (p.p588 * locals.var_bin_l);
        let assign3260_e4447: f64 = (p.p587 + assign3260_e4446);
        let assign3260_e4450: f64 = (p.p590 * locals.var_bin_w);
        let assign3260_e4451: f64 = (assign3260_e4447 + assign3260_e4450);
        let assign3260_e4454: f64 = (p.p592 * locals.var_bin_wl);
        let assign3260_e4455: f64 = (assign3260_e4451 + assign3260_e4454);
        locals.var_nbjt_i = assign3260_e4455;
        locals.var_nbjt_i_rv = 0.0;

        let assign3270_e4459: f64 = (p.p589 * locals.var_bin_l);
        let assign3270_e4460: f64 = (p.p594 + assign3270_e4459);
        let assign3270_e4463: f64 = (p.p591 * locals.var_bin_w);
        let assign3270_e4464: f64 = (assign3270_e4460 + assign3270_e4463);
        let assign3270_e4467: f64 = (p.p593 * locals.var_bin_wl);
        let assign3270_e4468: f64 = (assign3270_e4464 + assign3270_e4467);
        locals.var_lbjt0_i = assign3270_e4468;
        locals.var_lbjt0_i_rv = 0.0;

        let assign3280_e4472: f64 = (p.p922 * locals.var_bin_l);
        let assign3280_e4473: f64 = (p.p921 + assign3280_e4472);
        let assign3280_e4476: f64 = (p.p923 * locals.var_bin_w);
        let assign3280_e4477: f64 = (assign3280_e4473 + assign3280_e4476);
        let assign3280_e4480: f64 = (p.p924 * locals.var_bin_wl);
        let assign3280_e4481: f64 = (assign3280_e4477 + assign3280_e4480);
        locals.var_ndif_i = assign3280_e4481;
        locals.var_ndif_i_rv = 0.0;

        let assign3290_e4485: f64 = (locals.var_bin_l * p.p1126);
        let assign3290_e4486: f64 = (p.p1125 + assign3290_e4485);
        let assign3290_e4489: f64 = (locals.var_bin_w * p.p1127);
        let assign3290_e4490: f64 = (assign3290_e4486 + assign3290_e4489);
        let assign3290_e4493: f64 = (locals.var_bin_wl * p.p1128);
        let assign3290_e4494: f64 = (assign3290_e4490 + assign3290_e4493);
        locals.var_kvth0we_i = assign3290_e4494;
        locals.var_kvth0we_i_rv = 0.0;

        let assign3300_e4498: f64 = (locals.var_bin_l * p.p1130);
        let assign3300_e4499: f64 = (p.p1129 + assign3300_e4498);
        let assign3300_e4502: f64 = (locals.var_bin_w * p.p1131);
        let assign3300_e4503: f64 = (assign3300_e4499 + assign3300_e4502);
        let assign3300_e4506: f64 = (locals.var_bin_wl * p.p1132);
        let assign3300_e4507: f64 = (assign3300_e4503 + assign3300_e4506);
        locals.var_k2we_i = assign3300_e4507;
        locals.var_k2we_i_rv = 0.0;

        let assign3310_e4511: f64 = (locals.var_bin_l * p.p1134);
        let assign3310_e4512: f64 = (p.p1133 + assign3310_e4511);
        let assign3310_e4515: f64 = (locals.var_bin_w * p.p1135);
        let assign3310_e4516: f64 = (assign3310_e4512 + assign3310_e4515);
        let assign3310_e4519: f64 = (locals.var_bin_wl * p.p1136);
        let assign3310_e4520: f64 = (assign3310_e4516 + assign3310_e4519);
        locals.var_ku0we_i = assign3310_e4520;
        locals.var_ku0we_i_rv = 0.0;

        let assign3320_e4524: f64 = (locals.var_bin_l * p.p802);
        let assign3320_e4525: f64 = (p.p799 + assign3320_e4524);
        let assign3320_e4528: f64 = (locals.var_bin_w * p.p803);
        let assign3320_e4529: f64 = (assign3320_e4525 + assign3320_e4528);
        let assign3320_e4532: f64 = (locals.var_bin_wl * p.p804);
        let assign3320_e4533: f64 = (assign3320_e4529 + assign3320_e4532);
        locals.var_agidl_i = assign3320_e4533;
        locals.var_agidl_i_rv = 0.0;

        let assign3330_e4537: f64 = (locals.var_bin_l * p.p807);
        let assign3330_e4538: f64 = (p.p805 + assign3330_e4537);
        let assign3330_e4541: f64 = (locals.var_bin_w * p.p808);
        let assign3330_e4542: f64 = (assign3330_e4538 + assign3330_e4541);
        let assign3330_e4545: f64 = (locals.var_bin_wl * p.p809);
        let assign3330_e4546: f64 = (assign3330_e4542 + assign3330_e4545);
        locals.var_bgidl_i = assign3330_e4546;
        locals.var_bgidl_i_rv = 0.0;

        let assign3340_e4550: f64 = (p.p810 * locals.var_bin_l);
        let assign3340_e4551: f64 = (p.p806 + assign3340_e4550);
        let assign3340_e4554: f64 = (p.p811 * locals.var_bin_w);
        let assign3340_e4555: f64 = (assign3340_e4551 + assign3340_e4554);
        let assign3340_e4558: f64 = (p.p812 * locals.var_bin_wl);
        let assign3340_e4559: f64 = (assign3340_e4555 + assign3340_e4558);
        locals.var_bgidl1_i = assign3340_e4559;
        locals.var_bgidl1_i_rv = 0.0;

        let assign3350_e4563: f64 = (locals.var_bin_l * p.p814);
        let assign3350_e4564: f64 = (p.p813 + assign3350_e4563);
        let assign3350_e4567: f64 = (locals.var_bin_w * p.p815);
        let assign3350_e4568: f64 = (assign3350_e4564 + assign3350_e4567);
        let assign3350_e4571: f64 = (locals.var_bin_wl * p.p816);
        let assign3350_e4572: f64 = (assign3350_e4568 + assign3350_e4571);
        locals.var_cgidl_i = assign3350_e4572;
        locals.var_cgidl_i_rv = 0.0;

        let assign3360_e4576: f64 = (locals.var_bin_l * p.p818);
        let assign3360_e4577: f64 = (p.p817 + assign3360_e4576);
        let assign3360_e4580: f64 = (locals.var_bin_w * p.p819);
        let assign3360_e4581: f64 = (assign3360_e4577 + assign3360_e4580);
        let assign3360_e4584: f64 = (locals.var_bin_wl * p.p820);
        let assign3360_e4585: f64 = (assign3360_e4581 + assign3360_e4584);
        locals.var_egidl_i = assign3360_e4585;
        locals.var_egidl_i_rv = 0.0;

        let assign3370_e4589: f64 = (locals.var_bin_l * p.p824);
        let assign3370_e4590: f64 = (p.p821 + assign3370_e4589);
        let assign3370_e4593: f64 = (locals.var_bin_w * p.p825);
        let assign3370_e4594: f64 = (assign3370_e4590 + assign3370_e4593);
        let assign3370_e4597: f64 = (locals.var_bin_wl * p.p826);
        let assign3370_e4598: f64 = (assign3370_e4594 + assign3370_e4597);
        locals.var_agisl_i = assign3370_e4598;
        locals.var_agisl_i_rv = 0.0;

        let assign3380_e4602: f64 = (locals.var_bin_l * p.p829);
        let assign3380_e4603: f64 = (p.p827 + assign3380_e4602);
        let assign3380_e4606: f64 = (locals.var_bin_w * p.p830);
        let assign3380_e4607: f64 = (assign3380_e4603 + assign3380_e4606);
        let assign3380_e4610: f64 = (locals.var_bin_wl * p.p831);
        let assign3380_e4611: f64 = (assign3380_e4607 + assign3380_e4610);
        locals.var_bgisl_i = assign3380_e4611;
        locals.var_bgisl_i_rv = 0.0;

        let assign3390_e4615: f64 = (p.p832 * locals.var_bin_l);
        let assign3390_e4616: f64 = (p.p828 + assign3390_e4615);
        let assign3390_e4619: f64 = (p.p833 * locals.var_bin_w);
        let assign3390_e4620: f64 = (assign3390_e4616 + assign3390_e4619);
        let assign3390_e4623: f64 = (p.p834 * locals.var_bin_wl);
        let assign3390_e4624: f64 = (assign3390_e4620 + assign3390_e4623);
        locals.var_bgisl1_i = assign3390_e4624;
        locals.var_bgisl1_i_rv = 0.0;

        let assign3400_e4628: f64 = (locals.var_bin_l * p.p836);
        let assign3400_e4629: f64 = (p.p835 + assign3400_e4628);
        let assign3400_e4632: f64 = (locals.var_bin_w * p.p837);
        let assign3400_e4633: f64 = (assign3400_e4629 + assign3400_e4632);
        let assign3400_e4636: f64 = (locals.var_bin_wl * p.p838);
        let assign3400_e4637: f64 = (assign3400_e4633 + assign3400_e4636);
        locals.var_cgisl_i = assign3400_e4637;
        locals.var_cgisl_i_rv = 0.0;

        let assign3410_e4641: f64 = (locals.var_bin_l * p.p840);
        let assign3410_e4642: f64 = (p.p839 + assign3410_e4641);
        let assign3410_e4645: f64 = (locals.var_bin_w * p.p841);
        let assign3410_e4646: f64 = (assign3410_e4642 + assign3410_e4645);
        let assign3410_e4649: f64 = (locals.var_bin_wl * p.p842);
        let assign3410_e4650: f64 = (assign3410_e4646 + assign3410_e4649);
        locals.var_egisl_i = assign3410_e4650;
        locals.var_egisl_i_rv = 0.0;

        let assign3420_e4654: f64 = (locals.var_bin_l * p.p856);
        let assign3420_e4655: f64 = (p.p855 + assign3420_e4654);
        let assign3420_e4658: f64 = (locals.var_bin_w * p.p857);
        let assign3420_e4659: f64 = (assign3420_e4655 + assign3420_e4658);
        let assign3420_e4662: f64 = (locals.var_bin_wl * p.p858);
        let assign3420_e4663: f64 = (assign3420_e4659 + assign3420_e4662);
        locals.var_rgisl_i = assign3420_e4663;
        locals.var_rgisl_i_rv = 0.0;

        let assign3430_e4667: f64 = (locals.var_bin_l * p.p844);
        let assign3430_e4668: f64 = (p.p843 + assign3430_e4667);
        let assign3430_e4671: f64 = (locals.var_bin_w * p.p845);
        let assign3430_e4672: f64 = (assign3430_e4668 + assign3430_e4671);
        let assign3430_e4675: f64 = (locals.var_bin_wl * p.p846);
        let assign3430_e4676: f64 = (assign3430_e4672 + assign3430_e4675);
        locals.var_rgidl_i = assign3430_e4676;
        locals.var_rgidl_i_rv = 0.0;

        let assign3440_e4680: f64 = (locals.var_bin_l * p.p860);
        let assign3440_e4681: f64 = (p.p859 + assign3440_e4680);
        let assign3440_e4684: f64 = (locals.var_bin_w * p.p861);
        let assign3440_e4685: f64 = (assign3440_e4681 + assign3440_e4684);
        let assign3440_e4688: f64 = (locals.var_bin_wl * p.p862);
        let assign3440_e4689: f64 = (assign3440_e4685 + assign3440_e4688);
        locals.var_kgisl_i = assign3440_e4689;
        locals.var_kgisl_i_rv = 0.0;

        let assign3450_e4693: f64 = (locals.var_bin_l * p.p848);
        let assign3450_e4694: f64 = (p.p847 + assign3450_e4693);
        let assign3450_e4697: f64 = (locals.var_bin_w * p.p849);
        let assign3450_e4698: f64 = (assign3450_e4694 + assign3450_e4697);
        let assign3450_e4701: f64 = (locals.var_bin_wl * p.p850);
        let assign3450_e4702: f64 = (assign3450_e4698 + assign3450_e4701);
        locals.var_kgidl_i = assign3450_e4702;
        locals.var_kgidl_i_rv = 0.0;

        let assign3460_e4706: f64 = (locals.var_bin_l * p.p864);
        let assign3460_e4707: f64 = (p.p863 + assign3460_e4706);
        let assign3460_e4710: f64 = (locals.var_bin_w * p.p865);
        let assign3460_e4711: f64 = (assign3460_e4707 + assign3460_e4710);
        let assign3460_e4714: f64 = (locals.var_bin_wl * p.p866);
        let assign3460_e4715: f64 = (assign3460_e4711 + assign3460_e4714);
        locals.var_fgisl_i = assign3460_e4715;
        locals.var_fgisl_i_rv = 0.0;

        let assign3470_e4719: f64 = (locals.var_bin_l * p.p852);
        let assign3470_e4720: f64 = (p.p851 + assign3470_e4719);
        let assign3470_e4723: f64 = (locals.var_bin_w * p.p853);
        let assign3470_e4724: f64 = (assign3470_e4720 + assign3470_e4723);
        let assign3470_e4727: f64 = (locals.var_bin_wl * p.p854);
        let assign3470_e4728: f64 = (assign3470_e4724 + assign3470_e4727);
        locals.var_fgidl_i = assign3470_e4728;
        locals.var_fgidl_i_rv = 0.0;

        let assign3480_e4732: f64 = (locals.var_bin_l * p.p1033);
        let assign3480_e4733: f64 = (p.p1032 + assign3480_e4732);
        let assign3480_e4736: f64 = (locals.var_bin_w * p.p1034);
        let assign3480_e4737: f64 = (assign3480_e4733 + assign3480_e4736);
        let assign3480_e4740: f64 = (locals.var_bin_wl * p.p1035);
        let assign3480_e4741: f64 = (assign3480_e4737 + assign3480_e4740);
        locals.var_ute_i = assign3480_e4741;
        locals.var_ute_i_rv = 0.0;

        let assign3490_e4745: f64 = (locals.var_bin_l * p.p1038);
        let assign3490_e4746: f64 = (p.p1037 + assign3490_e4745);
        let assign3490_e4749: f64 = (locals.var_bin_w * p.p1039);
        let assign3490_e4750: f64 = (assign3490_e4746 + assign3490_e4749);
        let assign3490_e4753: f64 = (locals.var_bin_wl * p.p1040);
        let assign3490_e4754: f64 = (assign3490_e4750 + assign3490_e4753);
        locals.var_ua1_i = assign3490_e4754;
        locals.var_ua1_i_rv = 0.0;

        let assign3500_e4758: f64 = (locals.var_bin_l * p.p1043);
        let assign3500_e4759: f64 = (p.p1042 + assign3500_e4758);
        let assign3500_e4762: f64 = (locals.var_bin_w * p.p1044);
        let assign3500_e4763: f64 = (assign3500_e4759 + assign3500_e4762);
        let assign3500_e4766: f64 = (locals.var_bin_wl * p.p1045);
        let assign3500_e4767: f64 = (assign3500_e4763 + assign3500_e4766);
        locals.var_uc1_i = assign3500_e4767;
        locals.var_uc1_i_rv = 0.0;

        let assign3510_e4771: f64 = (locals.var_bin_l * p.p1047);
        let assign3510_e4772: f64 = (p.p1046 + assign3510_e4771);
        let assign3510_e4775: f64 = (locals.var_bin_w * p.p1048);
        let assign3510_e4776: f64 = (assign3510_e4772 + assign3510_e4775);
        let assign3510_e4779: f64 = (locals.var_bin_wl * p.p1049);
        let assign3510_e4780: f64 = (assign3510_e4776 + assign3510_e4779);
        locals.var_ud1_i = assign3510_e4780;
        locals.var_ud1_i_rv = 0.0;

        let assign3520_e4784: f64 = (locals.var_bin_l * p.p1052);
        let assign3520_e4785: f64 = (p.p1051 + assign3520_e4784);
        let assign3520_e4788: f64 = (locals.var_bin_w * p.p1053);
        let assign3520_e4789: f64 = (assign3520_e4785 + assign3520_e4788);
        let assign3520_e4792: f64 = (locals.var_bin_wl * p.p1054);
        let assign3520_e4793: f64 = (assign3520_e4789 + assign3520_e4792);
        locals.var_eu1_i = assign3520_e4793;
        locals.var_eu1_i_rv = 0.0;

        let assign3530_e4797: f64 = (locals.var_bin_l * p.p1056);
        let assign3530_e4798: f64 = (p.p1055 + assign3530_e4797);
        let assign3530_e4801: f64 = (locals.var_bin_w * p.p1057);
        let assign3530_e4802: f64 = (assign3530_e4798 + assign3530_e4801);
        let assign3530_e4805: f64 = (locals.var_bin_wl * p.p1058);
        let assign3530_e4806: f64 = (assign3530_e4802 + assign3530_e4805);
        locals.var_ucste_i = assign3530_e4806;
        locals.var_ucste_i_rv = 0.0;

        let assign3540_e4810: f64 = (locals.var_bin_l * p.p1061);
        let assign3540_e4811: f64 = (p.p1060 + assign3540_e4810);
        let assign3540_e4814: f64 = (locals.var_bin_w * p.p1062);
        let assign3540_e4815: f64 = (assign3540_e4811 + assign3540_e4814);
        let assign3540_e4818: f64 = (locals.var_bin_wl * p.p1063);
        let assign3540_e4819: f64 = (assign3540_e4815 + assign3540_e4818);
        locals.var_prt_i = assign3540_e4819;
        locals.var_prt_i_rv = 0.0;

        let assign3550_e4823: f64 = (locals.var_bin_l * p.p1065);
        let assign3550_e4824: f64 = (p.p1064 + assign3550_e4823);
        let assign3550_e4827: f64 = (locals.var_bin_w * p.p1066);
        let assign3550_e4828: f64 = (assign3550_e4824 + assign3550_e4827);
        let assign3550_e4831: f64 = (locals.var_bin_wl * p.p1067);
        let assign3550_e4832: f64 = (assign3550_e4828 + assign3550_e4831);
        locals.var_at_i = assign3550_e4832;
        locals.var_at_i_rv = 0.0;

        let assign3560_e4836: f64 = (locals.var_bin_l * p.p1071);
        let assign3560_e4837: f64 = (p.p1070 + assign3560_e4836);
        let assign3560_e4840: f64 = (locals.var_bin_w * p.p1072);
        let assign3560_e4841: f64 = (assign3560_e4837 + assign3560_e4840);
        let assign3560_e4844: f64 = (locals.var_bin_wl * p.p1073);
        let assign3560_e4845: f64 = (assign3560_e4841 + assign3560_e4844);
        locals.var_ptwgt_i = assign3560_e4845;
        locals.var_ptwgt_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3570_e4849: f64 = (locals.var_bin_l * p.p1086);
        let assign3570_e4850: f64 = (p.p1085 + assign3570_e4849);
        let assign3570_e4853: f64 = (locals.var_bin_w * p.p1087);
        let assign3570_e4854: f64 = (assign3570_e4850 + assign3570_e4853);
        let assign3570_e4857: f64 = (locals.var_bin_wl * p.p1088);
        let assign3570_e4858: f64 = (assign3570_e4854 + assign3570_e4857);
        locals.var_iit_i = assign3570_e4858;
        locals.var_iit_i_rv = 0.0;

        let assign3590_e4875: f64 = (locals.var_bin_l * p.p732);
        let assign3590_e4876: f64 = (p.p706 + assign3590_e4875);
        let assign3590_e4879: f64 = (locals.var_bin_w * p.p733);
        let assign3590_e4880: f64 = (assign3590_e4876 + assign3590_e4879);
        let assign3590_e4883: f64 = (locals.var_bin_wl * p.p734);
        let assign3590_e4884: f64 = (assign3590_e4880 + assign3590_e4883);
        locals.var_eigbinv_i = assign3590_e4884;
        locals.var_eigbinv_i_rv = 0.0;

        let assign3600_e4888: f64 = (locals.var_bin_l * p.p685);
        let assign3600_e4889: f64 = (p.p684 + assign3600_e4888);
        let assign3600_e4892: f64 = (locals.var_bin_w * p.p686);
        let assign3600_e4893: f64 = (assign3600_e4889 + assign3600_e4892);
        let assign3600_e4896: f64 = (locals.var_bin_wl * p.p687);
        let assign3600_e4897: f64 = (assign3600_e4893 + assign3600_e4896);
        locals.var_alphagb2_i = assign3600_e4897;
        locals.var_alphagb2_i_dn4 = 0.0;
        locals.var_alphagb2_i_dn5 = 0.0;
        locals.var_alphagb2_i_rv = 0.0;

        let assign3610_e4901: f64 = (p.p689 * locals.var_bin_l);
        let assign3610_e4902: f64 = (p.p688 + assign3610_e4901);
        let assign3610_e4905: f64 = (p.p690 * locals.var_bin_w);
        let assign3610_e4906: f64 = (assign3610_e4902 + assign3610_e4905);
        let assign3610_e4909: f64 = (p.p691 * locals.var_bin_wl);
        let assign3610_e4910: f64 = (assign3610_e4906 + assign3610_e4909);
        locals.var_alphagb2_t_i = assign3610_e4910;
        locals.var_alphagb2_t_i_rv = 0.0;

        let assign3620_e4914: f64 = (locals.var_bin_l * p.p693);
        let assign3620_e4915: f64 = (p.p692 + assign3620_e4914);
        let assign3620_e4918: f64 = (locals.var_bin_w * p.p694);
        let assign3620_e4919: f64 = (assign3620_e4915 + assign3620_e4918);
        let assign3620_e4922: f64 = (locals.var_bin_wl * p.p695);
        let assign3620_e4923: f64 = (assign3620_e4919 + assign3620_e4922);
        locals.var_betagb2_i = assign3620_e4923;
        locals.var_betagb2_i_rv = 0.0;

        let assign3630_e4927: f64 = (locals.var_bin_l * p.p673);
        let assign3630_e4928: f64 = (p.p672 + assign3630_e4927);
        let assign3630_e4931: f64 = (locals.var_bin_w * p.p674);
        let assign3630_e4932: f64 = (assign3630_e4928 + assign3630_e4931);
        let assign3630_e4935: f64 = (locals.var_bin_wl * p.p675);
        let assign3630_e4936: f64 = (assign3630_e4932 + assign3630_e4935);
        locals.var_alphagb1_i = assign3630_e4936;
        locals.var_alphagb1_i_dn4 = 0.0;
        locals.var_alphagb1_i_dn5 = 0.0;
        locals.var_alphagb1_i_rv = 0.0;

        let assign3640_e4940: f64 = (p.p677 * locals.var_bin_l);
        let assign3640_e4941: f64 = (p.p676 + assign3640_e4940);
        let assign3640_e4944: f64 = (p.p678 * locals.var_bin_w);
        let assign3640_e4945: f64 = (assign3640_e4941 + assign3640_e4944);
        let assign3640_e4948: f64 = (p.p679 * locals.var_bin_wl);
        let assign3640_e4949: f64 = (assign3640_e4945 + assign3640_e4948);
        locals.var_alphagb1_t_i = assign3640_e4949;
        locals.var_alphagb1_t_i_rv = 0.0;

        let assign3650_e4953: f64 = (locals.var_bin_l * p.p681);
        let assign3650_e4954: f64 = (p.p680 + assign3650_e4953);
        let assign3650_e4957: f64 = (locals.var_bin_w * p.p682);
        let assign3650_e4958: f64 = (assign3650_e4954 + assign3650_e4957);
        let assign3650_e4961: f64 = (locals.var_bin_wl * p.p683);
        let assign3650_e4962: f64 = (assign3650_e4958 + assign3650_e4961);
        locals.var_betagb1_i = assign3650_e4962;
        locals.var_betagb1_i_rv = 0.0;

        let assign3660_e4966: f64 = (locals.var_bin_l * p.p735);
        let assign3660_e4967: f64 = (p.p707 + assign3660_e4966);
        let assign3660_e4970: f64 = (locals.var_bin_w * p.p737);
        let assign3660_e4971: f64 = (assign3660_e4967 + assign3660_e4970);
        let assign3660_e4974: f64 = (locals.var_bin_wl * p.p739);
        let assign3660_e4975: f64 = (assign3660_e4971 + assign3660_e4974);
        locals.var_aigc_i = assign3660_e4975;
        locals.var_aigc_i_dn4 = 0.0;
        locals.var_aigc_i_dn5 = 0.0;
        locals.var_aigc_i_rv = 0.0;

        let assign3670_e4979: f64 = (p.p736 * locals.var_bin_l);
        let assign3670_e4980: f64 = (p.p726 + assign3670_e4979);
        let assign3670_e4983: f64 = (p.p738 * locals.var_bin_w);
        let assign3670_e4984: f64 = (assign3670_e4980 + assign3670_e4983);
        let assign3670_e4987: f64 = (p.p740 * locals.var_bin_wl);
        let assign3670_e4988: f64 = (assign3670_e4984 + assign3670_e4987);
        locals.var_aigc1_i = assign3670_e4988;
        locals.var_aigc1_i_rv = 0.0;

        let assign3680_e4992: f64 = (locals.var_bin_l * p.p741);
        let assign3680_e4993: f64 = (p.p708 + assign3680_e4992);
        let assign3680_e4996: f64 = (locals.var_bin_w * p.p742);
        let assign3680_e4997: f64 = (assign3680_e4993 + assign3680_e4996);
        let assign3680_e5000: f64 = (locals.var_bin_wl * p.p743);
        let assign3680_e5001: f64 = (assign3680_e4997 + assign3680_e5000);
        locals.var_bigc_i = assign3680_e5001;
        locals.var_bigc_i_rv = 0.0;

        let assign3690_e5005: f64 = (locals.var_bin_l * p.p744);
        let assign3690_e5006: f64 = (p.p709 + assign3690_e5005);
        let assign3690_e5009: f64 = (locals.var_bin_w * p.p745);
        let assign3690_e5010: f64 = (assign3690_e5006 + assign3690_e5009);
        let assign3690_e5013: f64 = (locals.var_bin_wl * p.p746);
        let assign3690_e5014: f64 = (assign3690_e5010 + assign3690_e5013);
        locals.var_cigc_i = assign3690_e5014;
        locals.var_cigc_i_rv = 0.0;

        let assign3700_e5018: f64 = (locals.var_bin_l * p.p747);
        let assign3700_e5019: f64 = (p.p710 + assign3700_e5018);
        let assign3700_e5022: f64 = (locals.var_bin_w * p.p749);
        let assign3700_e5023: f64 = (assign3700_e5019 + assign3700_e5022);
        let assign3700_e5026: f64 = (locals.var_bin_wl * p.p751);
        let assign3700_e5027: f64 = (assign3700_e5023 + assign3700_e5026);
        locals.var_aigs_i = assign3700_e5027;
        locals.var_aigs_i_dn4 = 0.0;
        locals.var_aigs_i_dn5 = 0.0;
        locals.var_aigs_i_rv = 0.0;

        let assign3710_e5031: f64 = (p.p748 * locals.var_bin_l);
        let assign3710_e5032: f64 = (p.p711 + assign3710_e5031);
        let assign3710_e5035: f64 = (p.p750 * locals.var_bin_w);
        let assign3710_e5036: f64 = (assign3710_e5032 + assign3710_e5035);
        let assign3710_e5039: f64 = (p.p752 * locals.var_bin_wl);
        let assign3710_e5040: f64 = (assign3710_e5036 + assign3710_e5039);
        locals.var_aigs1_i = assign3710_e5040;
        locals.var_aigs1_i_rv = 0.0;

        let assign3720_e5044: f64 = (locals.var_bin_l * p.p753);
        let assign3720_e5045: f64 = (p.p712 + assign3720_e5044);
        let assign3720_e5048: f64 = (locals.var_bin_w * p.p754);
        let assign3720_e5049: f64 = (assign3720_e5045 + assign3720_e5048);
        let assign3720_e5052: f64 = (locals.var_bin_wl * p.p755);
        let assign3720_e5053: f64 = (assign3720_e5049 + assign3720_e5052);
        locals.var_bigs_i = assign3720_e5053;
        locals.var_bigs_i_rv = 0.0;

        let assign3730_e5057: f64 = (locals.var_bin_l * p.p756);
        let assign3730_e5058: f64 = (p.p713 + assign3730_e5057);
        let assign3730_e5061: f64 = (locals.var_bin_w * p.p757);
        let assign3730_e5062: f64 = (assign3730_e5058 + assign3730_e5061);
        let assign3730_e5065: f64 = (locals.var_bin_wl * p.p758);
        let assign3730_e5066: f64 = (assign3730_e5062 + assign3730_e5065);
        locals.var_cigs_i = assign3730_e5066;
        locals.var_cigs_i_rv = 0.0;

        let assign3740_e5070: f64 = (locals.var_bin_l * p.p759);
        let assign3740_e5071: f64 = (p.p714 + assign3740_e5070);
        let assign3740_e5074: f64 = (locals.var_bin_w * p.p761);
        let assign3740_e5075: f64 = (assign3740_e5071 + assign3740_e5074);
        let assign3740_e5078: f64 = (locals.var_bin_wl * p.p763);
        let assign3740_e5079: f64 = (assign3740_e5075 + assign3740_e5078);
        locals.var_aigd_i = assign3740_e5079;
        locals.var_aigd_i_dn4 = 0.0;
        locals.var_aigd_i_dn5 = 0.0;
        locals.var_aigd_i_rv = 0.0;

        let assign3750_e5083: f64 = (p.p760 * locals.var_bin_l);
        let assign3750_e5084: f64 = (p.p715 + assign3750_e5083);
        let assign3750_e5087: f64 = (p.p762 * locals.var_bin_w);
        let assign3750_e5088: f64 = (assign3750_e5084 + assign3750_e5087);
        let assign3750_e5091: f64 = (p.p764 * locals.var_bin_wl);
        let assign3750_e5092: f64 = (assign3750_e5088 + assign3750_e5091);
        locals.var_aigd1_i = assign3750_e5092;
        locals.var_aigd1_i_rv = 0.0;

        let assign3760_e5096: f64 = (locals.var_bin_l * p.p765);
        let assign3760_e5097: f64 = (p.p716 + assign3760_e5096);
        let assign3760_e5100: f64 = (locals.var_bin_w * p.p766);
        let assign3760_e5101: f64 = (assign3760_e5097 + assign3760_e5100);
        let assign3760_e5104: f64 = (locals.var_bin_wl * p.p767);
        let assign3760_e5105: f64 = (assign3760_e5101 + assign3760_e5104);
        locals.var_bigd_i = assign3760_e5105;
        locals.var_bigd_i_rv = 0.0;

        let assign3770_e5109: f64 = (locals.var_bin_l * p.p768);
        let assign3770_e5110: f64 = (p.p717 + assign3770_e5109);
        let assign3770_e5113: f64 = (locals.var_bin_w * p.p769);
        let assign3770_e5114: f64 = (assign3770_e5110 + assign3770_e5113);
        let assign3770_e5117: f64 = (locals.var_bin_wl * p.p770);
        let assign3770_e5118: f64 = (assign3770_e5114 + assign3770_e5117);
        locals.var_cigd_i = assign3770_e5118;
        locals.var_cigd_i_rv = 0.0;

        let assign3780_e5122: f64 = (locals.var_bin_l * p.p771);
        let assign3780_e5123: f64 = (p.p720 + assign3780_e5122);
        let assign3780_e5126: f64 = (locals.var_bin_w * p.p772);
        let assign3780_e5127: f64 = (assign3780_e5123 + assign3780_e5126);
        let assign3780_e5130: f64 = (locals.var_bin_wl * p.p773);
        let assign3780_e5131: f64 = (assign3780_e5127 + assign3780_e5130);
        locals.var_poxedge_i = assign3780_e5131;
        locals.var_poxedge_i_rv = 0.0;

        let assign3810_e5161: f64 = (locals.var_bin_l * p.p780);
        let assign3810_e5162: f64 = (p.p721 + assign3810_e5161);
        let assign3810_e5165: f64 = (locals.var_bin_w * p.p781);
        let assign3810_e5166: f64 = (assign3810_e5162 + assign3810_e5165);
        let assign3810_e5169: f64 = (locals.var_bin_wl * p.p782);
        let assign3810_e5170: f64 = (assign3810_e5166 + assign3810_e5169);
        locals.var_ntox_i = assign3810_e5170;
        locals.var_ntox_i_rv = 0.0;

        let assign3820_e5174: f64 = (locals.var_bin_l * p.p1078);
        let assign3820_e5175: f64 = (p.p1075 + assign3820_e5174);
        let assign3820_e5178: f64 = (locals.var_bin_w * p.p1079);
        let assign3820_e5179: f64 = (assign3820_e5175 + assign3820_e5178);
        let assign3820_e5182: f64 = (locals.var_bin_wl * p.p1080);
        let assign3820_e5183: f64 = (assign3820_e5179 + assign3820_e5182);
        locals.var_kt1_i = assign3820_e5183;
        locals.var_kt1_i_rv = 0.0;

        let assign3830_e5187: f64 = (locals.var_bin_l * p.p1082);
        let assign3830_e5188: f64 = (p.p1081 + assign3830_e5187);
        let assign3830_e5191: f64 = (locals.var_bin_w * p.p1083);
        let assign3830_e5192: f64 = (assign3830_e5188 + assign3830_e5191);
        let assign3830_e5195: f64 = (locals.var_bin_wl * p.p1084);
        let assign3830_e5196: f64 = (assign3830_e5192 + assign3830_e5195);
        locals.var_kt2_i = assign3830_e5196;
        locals.var_kt2_i_rv = 0.0;

        let assign3840_e5200: f64 = (locals.var_bin_l * p.p494);
        let assign3840_e5201: f64 = (p.p489 + assign3840_e5200);
        let assign3840_e5204: f64 = (locals.var_bin_w * p.p495);
        let assign3840_e5205: f64 = (assign3840_e5201 + assign3840_e5204);
        let assign3840_e5208: f64 = (locals.var_bin_wl * p.p496);
        let assign3840_e5209: f64 = (assign3840_e5205 + assign3840_e5208);
        locals.var_psatb_i = assign3840_e5209;
        locals.var_psatb_i_rv = 0.0;

        let assign3850_e5213: f64 = (locals.var_bin_l * p.p515);
        let assign3850_e5214: f64 = (p.p514 + assign3850_e5213);
        let assign3850_e5217: f64 = (locals.var_bin_w * p.p516);
        let assign3850_e5218: f64 = (assign3850_e5214 + assign3850_e5217);
        let assign3850_e5221: f64 = (locals.var_bin_wl * p.p517);
        let assign3850_e5222: f64 = (assign3850_e5218 + assign3850_e5221);
        locals.var_a1_i = assign3850_e5222;
        locals.var_a1_i_rv = 0.0;

        let assign3860_e5226: f64 = (locals.var_bin_l * p.p519);
        let assign3860_e5227: f64 = (p.p518 + assign3860_e5226);
        let assign3860_e5230: f64 = (locals.var_bin_w * p.p520);
        let assign3860_e5231: f64 = (assign3860_e5227 + assign3860_e5230);
        let assign3860_e5234: f64 = (locals.var_bin_wl * p.p521);
        let assign3860_e5235: f64 = (assign3860_e5231 + assign3860_e5234);
        locals.var_a11_i = assign3860_e5235;
        locals.var_a11_i_rv = 0.0;

        let assign3870_e5239: f64 = (locals.var_bin_l * p.p523);
        let assign3870_e5240: f64 = (p.p522 + assign3870_e5239);
        let assign3870_e5243: f64 = (locals.var_bin_w * p.p524);
        let assign3870_e5244: f64 = (assign3870_e5240 + assign3870_e5243);
        let assign3870_e5247: f64 = (locals.var_bin_wl * p.p525);
        let assign3870_e5248: f64 = (assign3870_e5244 + assign3870_e5247);
        locals.var_a2_i = assign3870_e5248;
        locals.var_a2_i_rv = 0.0;

        let assign3880_e5252: f64 = (locals.var_bin_l * p.p527);
        let assign3880_e5253: f64 = (p.p526 + assign3880_e5252);
        let assign3880_e5256: f64 = (locals.var_bin_w * p.p528);
        let assign3880_e5257: f64 = (assign3880_e5253 + assign3880_e5256);
        let assign3880_e5260: f64 = (locals.var_bin_wl * p.p529);
        let assign3880_e5261: f64 = (assign3880_e5257 + assign3880_e5260);
        locals.var_a21_i = assign3880_e5261;
        locals.var_a21_i_rv = 0.0;

        let assign3890_e5265: f64 = (locals.var_bin_l * p.p1301);
        let assign3890_e5266: f64 = (p.p1300 + assign3890_e5265);
        let assign3890_e5269: f64 = (locals.var_bin_w * p.p1302);
        let assign3890_e5270: f64 = (assign3890_e5266 + assign3890_e5269);
        let assign3890_e5273: f64 = (locals.var_bin_wl * p.p1303);
        let assign3890_e5274: f64 = (assign3890_e5270 + assign3890_e5273);
        locals.var_k0_i = assign3890_e5274;
        locals.var_k0_i_rv = 0.0;

        let assign3900_e5278: f64 = (locals.var_bin_l * p.p1309);
        let assign3900_e5279: f64 = (p.p1308 + assign3900_e5278);
        let assign3900_e5282: f64 = (locals.var_bin_w * p.p1310);
        let assign3900_e5283: f64 = (assign3900_e5279 + assign3900_e5282);
        let assign3900_e5286: f64 = (locals.var_bin_wl * p.p1311);
        let assign3900_e5287: f64 = (assign3900_e5283 + assign3900_e5286);
        locals.var_m0_i = assign3900_e5287;
        locals.var_m0_i_rv = 0.0;

        let assign3910_e5291: f64 = (locals.var_bin_l * p.p1305);
        let assign3910_e5292: f64 = (p.p1304 + assign3910_e5291);
        let assign3910_e5295: f64 = (locals.var_bin_w * p.p1306);
        let assign3910_e5296: f64 = (assign3910_e5292 + assign3910_e5295);
        let assign3910_e5299: f64 = (locals.var_bin_wl * p.p1307);
        let assign3910_e5300: f64 = (assign3910_e5296 + assign3910_e5299);
        locals.var_k01_i = assign3910_e5300;
        locals.var_k01_i_rv = 0.0;

        let assign3920_e5304: f64 = (locals.var_bin_l * p.p1313);
        let assign3920_e5305: f64 = (p.p1312 + assign3920_e5304);
        let assign3920_e5308: f64 = (locals.var_bin_w * p.p1314);
        let assign3920_e5309: f64 = (assign3920_e5305 + assign3920_e5308);
        let assign3920_e5312: f64 = (locals.var_bin_wl * p.p1315);
        let assign3920_e5313: f64 = (assign3920_e5309 + assign3920_e5312);
        locals.var_m01_i = assign3920_e5313;
        locals.var_m01_i_rv = 0.0;

        let assign3930_e5317: f64 = (locals.var_bin_l * p.p1157);
        let assign3930_e5318: f64 = (p.p1156 + assign3930_e5317);
        let assign3930_e5321: f64 = (locals.var_bin_w * p.p1158);
        let assign3930_e5322: f64 = (assign3930_e5318 + assign3930_e5321);
        let assign3930_e5325: f64 = (locals.var_bin_wl * p.p1159);
        let assign3930_e5326: f64 = (assign3930_e5322 + assign3930_e5325);
        locals.var_nfactoredge_i = assign3930_e5326;
        locals.var_nfactoredge_i_rv = 0.0;

        let assign3940_e5330: f64 = (locals.var_bin_l * p.p1153);
        let assign3940_e5331: f64 = (p.p1152 + assign3940_e5330);
        let assign3940_e5334: f64 = (locals.var_bin_w * p.p1154);
        let assign3940_e5335: f64 = (assign3940_e5331 + assign3940_e5334);
        let assign3940_e5338: f64 = (locals.var_bin_wl * p.p1155);
        let assign3940_e5339: f64 = (assign3940_e5335 + assign3940_e5338);
        locals.var_ndepedge_i = assign3940_e5339;
        locals.var_ndepedge_i_rv = 0.0;

        let assign3950_e5343: f64 = (locals.var_bin_l * p.p1161);
        let assign3950_e5344: f64 = (p.p1160 + assign3950_e5343);
        let assign3950_e5347: f64 = (locals.var_bin_w * p.p1162);
        let assign3950_e5348: f64 = (assign3950_e5344 + assign3950_e5347);
        let assign3950_e5351: f64 = (locals.var_bin_wl * p.p1163);
        let assign3950_e5352: f64 = (assign3950_e5348 + assign3950_e5351);
        locals.var_citedge_i = assign3950_e5352;
        locals.var_citedge_i_rv = 0.0;

        let assign3960_e5356: f64 = (locals.var_bin_l * p.p1169);
        let assign3960_e5357: f64 = (p.p1168 + assign3960_e5356);
        let assign3960_e5360: f64 = (locals.var_bin_w * p.p1170);
        let assign3960_e5361: f64 = (assign3960_e5357 + assign3960_e5360);
        let assign3960_e5364: f64 = (locals.var_bin_wl * p.p1171);
        let assign3960_e5365: f64 = (assign3960_e5361 + assign3960_e5364);
        locals.var_cdscdedge_i = assign3960_e5365;
        locals.var_cdscdedge_i_rv = 0.0;

        let assign3970_e5369: f64 = (locals.var_bin_l * p.p1187);
        let assign3970_e5370: f64 = (p.p1186 + assign3970_e5369);
        let assign3970_e5373: f64 = (locals.var_bin_w * p.p1188);
        let assign3970_e5374: f64 = (assign3970_e5370 + assign3970_e5373);
        let assign3970_e5377: f64 = (locals.var_bin_wl * p.p1189);
        let assign3970_e5378: f64 = (assign3970_e5374 + assign3970_e5377);
        locals.var_cdscbedge_i = assign3970_e5378;
        locals.var_cdscbedge_i_rv = 0.0;

        let assign3980_e5382: f64 = (locals.var_bin_l * p.p1207);
        let assign3980_e5383: f64 = (p.p1206 + assign3980_e5382);
        let assign3980_e5386: f64 = (locals.var_bin_w * p.p1208);
        let assign3980_e5387: f64 = (assign3980_e5383 + assign3980_e5386);
        let assign3980_e5390: f64 = (locals.var_bin_wl * p.p1209);
        let assign3980_e5391: f64 = (assign3980_e5387 + assign3980_e5390);
        locals.var_eta0edge_i = assign3980_e5391;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_rv = 0.0;

        let assign3990_e5395: f64 = (locals.var_bin_l * p.p1211);
        let assign3990_e5396: f64 = (p.p1210 + assign3990_e5395);
        let assign3990_e5399: f64 = (locals.var_bin_w * p.p1212);
        let assign3990_e5400: f64 = (assign3990_e5396 + assign3990_e5399);
        let assign3990_e5403: f64 = (locals.var_bin_wl * p.p1213);
        let assign3990_e5404: f64 = (assign3990_e5400 + assign3990_e5403);
        locals.var_etabedge_i = assign3990_e5404;
        locals.var_etabedge_i_rv = 0.0;

        let assign4000_e5408: f64 = (locals.var_bin_l * p.p1215);
        let assign4000_e5409: f64 = (p.p1214 + assign4000_e5408);
        let assign4000_e5412: f64 = (locals.var_bin_w * p.p1216);
        let assign4000_e5413: f64 = (assign4000_e5409 + assign4000_e5412);
        let assign4000_e5416: f64 = (locals.var_bin_wl * p.p1217);
        let assign4000_e5417: f64 = (assign4000_e5413 + assign4000_e5416);
        locals.var_kt1edge_i = assign4000_e5417;
        locals.var_kt1edge_i_rv = 0.0;

        let assign4010_e5421: f64 = (locals.var_bin_l * p.p1219);
        let assign4010_e5422: f64 = (p.p1218 + assign4010_e5421);
        let assign4010_e5425: f64 = (locals.var_bin_w * p.p1220);
        let assign4010_e5426: f64 = (assign4010_e5422 + assign4010_e5425);
        let assign4010_e5429: f64 = (locals.var_bin_wl * p.p1221);
        let assign4010_e5430: f64 = (assign4010_e5426 + assign4010_e5429);
        locals.var_kt1ledge_i = assign4010_e5430;
        locals.var_kt1ledge_i_rv = 0.0;

        let assign4020_e5434: f64 = (locals.var_bin_l * p.p1223);
        let assign4020_e5435: f64 = (p.p1222 + assign4020_e5434);
        let assign4020_e5438: f64 = (locals.var_bin_w * p.p1224);
        let assign4020_e5439: f64 = (assign4020_e5435 + assign4020_e5438);
        let assign4020_e5442: f64 = (locals.var_bin_wl * p.p1225);
        let assign4020_e5443: f64 = (assign4020_e5439 + assign4020_e5442);
        locals.var_kt2edge_i = assign4020_e5443;
        locals.var_kt2edge_i_rv = 0.0;

        let assign4030_e5447: f64 = (locals.var_bin_l * p.p1227);
        let assign4030_e5448: f64 = (p.p1226 + assign4030_e5447);
        let assign4030_e5451: f64 = (locals.var_bin_w * p.p1228);
        let assign4030_e5452: f64 = (assign4030_e5448 + assign4030_e5451);
        let assign4030_e5455: f64 = (locals.var_bin_wl * p.p1229);
        let assign4030_e5456: f64 = (assign4030_e5452 + assign4030_e5455);
        locals.var_kt1expedge_i = assign4030_e5456;
        locals.var_kt1expedge_i_rv = 0.0;

        let assign4040_e5460: f64 = (locals.var_bin_l * p.p1231);
        let assign4040_e5461: f64 = (p.p1230 + assign4040_e5460);
        let assign4040_e5464: f64 = (locals.var_bin_w * p.p1232);
        let assign4040_e5465: f64 = (assign4040_e5461 + assign4040_e5464);
        let assign4040_e5468: f64 = (locals.var_bin_wl * p.p1233);
        let assign4040_e5469: f64 = (assign4040_e5465 + assign4040_e5468);
        locals.var_tnfactoredge_i = assign4040_e5469;
        locals.var_tnfactoredge_i_rv = 0.0;

        let assign4050_e5473: f64 = (locals.var_bin_l * p.p1235);
        let assign4050_e5474: f64 = (p.p1234 + assign4050_e5473);
        let assign4050_e5477: f64 = (locals.var_bin_w * p.p1236);
        let assign4050_e5478: f64 = (assign4050_e5474 + assign4050_e5477);
        let assign4050_e5481: f64 = (locals.var_bin_wl * p.p1237);
        let assign4050_e5482: f64 = (assign4050_e5478 + assign4050_e5481);
        locals.var_teta0edge_i = assign4050_e5482;
        locals.var_teta0edge_i_rv = 0.0;

        let assign4060_e5486: f64 = (locals.var_bin_l * p.p1272);
        let assign4060_e5487: f64 = (p.p1265 + assign4060_e5486);
        let assign4060_e5490: f64 = (locals.var_bin_w * p.p1273);
        let assign4060_e5491: f64 = (assign4060_e5487 + assign4060_e5490);
        let assign4060_e5494: f64 = (locals.var_bin_wl * p.p1274);
        let assign4060_e5495: f64 = (assign4060_e5491 + assign4060_e5494);
        locals.var_k2edge_i = assign4060_e5495;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_rv = 0.0;

        let assign4070_e5499: f64 = (locals.var_bin_l * p.p1276);
        let assign4070_e5500: f64 = (p.p1275 + assign4070_e5499);
        let assign4070_e5503: f64 = (locals.var_bin_w * p.p1277);
        let assign4070_e5504: f64 = (assign4070_e5500 + assign4070_e5503);
        let assign4070_e5507: f64 = (locals.var_bin_wl * p.p1278);
        let assign4070_e5508: f64 = (assign4070_e5504 + assign4070_e5507);
        locals.var_kvth0edge_i = assign4070_e5508;
        locals.var_kvth0edge_i_rv = 0.0;

        let assign4080_e5512: f64 = (locals.var_bin_l * p.p1284);
        let assign4080_e5513: f64 = (p.p1283 + assign4080_e5512);
        let assign4080_e5516: f64 = (locals.var_bin_w * p.p1285);
        let assign4080_e5517: f64 = (assign4080_e5513 + assign4080_e5516);
        let assign4080_e5520: f64 = (locals.var_bin_wl * p.p1286);
        let assign4080_e5521: f64 = (assign4080_e5517 + assign4080_e5520);
        locals.var_k2edgewe_i = assign4080_e5521;
        locals.var_k2edgewe_i_rv = 0.0;

        let assign4090_e5525: f64 = (locals.var_bin_l * p.p1280);
        let assign4090_e5526: f64 = (p.p1279 + assign4090_e5525);
        let assign4090_e5529: f64 = (locals.var_bin_w * p.p1281);
        let assign4090_e5530: f64 = (assign4090_e5526 + assign4090_e5529);
        let assign4090_e5533: f64 = (locals.var_bin_wl * p.p1282);
        let assign4090_e5534: f64 = (assign4090_e5530 + assign4090_e5533);
        locals.var_kvth0edgewe_i = assign4090_e5534;
        locals.var_kvth0edgewe_i_rv = 0.0;

        let assign4100_e5538: f64 = (locals.var_bin_l * p.p1288);
        let assign4100_e5539: f64 = (p.p1287 + assign4100_e5538);
        let assign4100_e5542: f64 = (locals.var_bin_w * p.p1289);
        let assign4100_e5543: f64 = (assign4100_e5539 + assign4100_e5542);
        let assign4100_e5546: f64 = (locals.var_bin_wl * p.p1290);
        let assign4100_e5547: f64 = (assign4100_e5543 + assign4100_e5546);
        locals.var_stk2edge_i = assign4100_e5547;
        locals.var_stk2edge_i_rv = 0.0;

        let assign4110_e5551: f64 = (locals.var_bin_l * p.p1292);
        let assign4110_e5552: f64 = (p.p1291 + assign4110_e5551);
        let assign4110_e5555: f64 = (locals.var_bin_w * p.p1293);
        let assign4110_e5556: f64 = (assign4110_e5552 + assign4110_e5555);
        let assign4110_e5559: f64 = (locals.var_bin_wl * p.p1294);
        let assign4110_e5560: f64 = (assign4110_e5556 + assign4110_e5559);
        locals.var_steta0edge_i = assign4110_e5560;
        locals.var_steta0edge_i_rv = 0.0;

        let assign4120_e5564: f64 = (locals.var_bin_l * p.p1324);
        let assign4120_e5565: f64 = (p.p1323 + assign4120_e5564);
        let assign4120_e5568: f64 = (locals.var_bin_w * p.p1325);
        let assign4120_e5569: f64 = (assign4120_e5565 + assign4120_e5568);
        let assign4120_e5572: f64 = (locals.var_bin_wl * p.p1326);
        let assign4120_e5573: f64 = (assign4120_e5569 + assign4120_e5572);
        locals.var_c0_i = assign4120_e5573;
        locals.var_c0_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4130_e5577: f64 = (locals.var_bin_l * p.p1328);
        let assign4130_e5578: f64 = (p.p1327 + assign4130_e5577);
        let assign4130_e5581: f64 = (locals.var_bin_w * p.p1329);
        let assign4130_e5582: f64 = (assign4130_e5578 + assign4130_e5581);
        let assign4130_e5585: f64 = (locals.var_bin_wl * p.p1330);
        let assign4130_e5586: f64 = (assign4130_e5582 + assign4130_e5585);
        locals.var_c01_i = assign4130_e5586;
        locals.var_c01_i_rv = 0.0;

        let assign4140_e5590: f64 = (locals.var_bin_l * p.p1332);
        let assign4140_e5591: f64 = (p.p1331 + assign4140_e5590);
        let assign4140_e5594: f64 = (locals.var_bin_w * p.p1333);
        let assign4140_e5595: f64 = (assign4140_e5591 + assign4140_e5594);
        let assign4140_e5598: f64 = (locals.var_bin_wl * p.p1334);
        let assign4140_e5599: f64 = (assign4140_e5595 + assign4140_e5598);
        locals.var_c0si_i = assign4140_e5599;
        locals.var_c0si_i_rv = 0.0;

        let assign4150_e5603: f64 = (locals.var_bin_l * p.p1336);
        let assign4150_e5604: f64 = (p.p1335 + assign4150_e5603);
        let assign4150_e5607: f64 = (locals.var_bin_w * p.p1337);
        let assign4150_e5608: f64 = (assign4150_e5604 + assign4150_e5607);
        let assign4150_e5611: f64 = (locals.var_bin_wl * p.p1338);
        let assign4150_e5612: f64 = (assign4150_e5608 + assign4150_e5611);
        locals.var_c0si1_i = assign4150_e5612;
        locals.var_c0si1_i_rv = 0.0;

        let assign4160_e5616: f64 = (locals.var_bin_l * p.p1340);
        let assign4160_e5617: f64 = (p.p1339 + assign4160_e5616);
        let assign4160_e5620: f64 = (locals.var_bin_w * p.p1341);
        let assign4160_e5621: f64 = (assign4160_e5617 + assign4160_e5620);
        let assign4160_e5624: f64 = (locals.var_bin_wl * p.p1342);
        let assign4160_e5625: f64 = (assign4160_e5621 + assign4160_e5624);
        locals.var_c0sisat_i = assign4160_e5625;
        locals.var_c0sisat_i_rv = 0.0;

        let assign4170_e5629: f64 = (locals.var_bin_l * p.p1344);
        let assign4170_e5630: f64 = (p.p1343 + assign4170_e5629);
        let assign4170_e5633: f64 = (locals.var_bin_w * p.p1345);
        let assign4170_e5634: f64 = (assign4170_e5630 + assign4170_e5633);
        let assign4170_e5637: f64 = (locals.var_bin_wl * p.p1346);
        let assign4170_e5638: f64 = (assign4170_e5634 + assign4170_e5637);
        locals.var_c0sisat1_i = assign4170_e5638;
        locals.var_c0sisat1_i_rv = 0.0;

        let assign4180_e5642: f64 = (locals.var_bin_l * p.p787);
        let assign4180_e5643: f64 = (p.p783 + assign4180_e5642);
        let assign4180_e5646: f64 = (locals.var_bin_w * p.p791);
        let assign4180_e5647: f64 = (assign4180_e5643 + assign4180_e5646);
        let assign4180_e5650: f64 = (locals.var_bin_wl * p.p795);
        let assign4180_e5651: f64 = (assign4180_e5647 + assign4180_e5650);
        locals.var_aigbcp2_i = assign4180_e5651;
        locals.var_aigbcp2_i_dn4 = 0.0;
        locals.var_aigbcp2_i_dn5 = 0.0;
        locals.var_aigbcp2_i_rv = 0.0;

        let assign4190_e5655: f64 = (p.p788 * locals.var_bin_l);
        let assign4190_e5656: f64 = (p.p784 + assign4190_e5655);
        let assign4190_e5659: f64 = (p.p792 * locals.var_bin_w);
        let assign4190_e5660: f64 = (assign4190_e5656 + assign4190_e5659);
        let assign4190_e5663: f64 = (p.p796 * locals.var_bin_wl);
        let assign4190_e5664: f64 = (assign4190_e5660 + assign4190_e5663);
        locals.var_aigbcp2_t_i = assign4190_e5664;
        locals.var_aigbcp2_t_i_rv = 0.0;

        let assign4200_e5668: f64 = (locals.var_bin_l * p.p789);
        let assign4200_e5669: f64 = (p.p785 + assign4200_e5668);
        let assign4200_e5672: f64 = (locals.var_bin_w * p.p793);
        let assign4200_e5673: f64 = (assign4200_e5669 + assign4200_e5672);
        let assign4200_e5676: f64 = (locals.var_bin_wl * p.p797);
        let assign4200_e5677: f64 = (assign4200_e5673 + assign4200_e5676);
        locals.var_bigbcp2_i = assign4200_e5677;
        locals.var_bigbcp2_i_rv = 0.0;

        let assign4210_e5681: f64 = (locals.var_bin_l * p.p790);
        let assign4210_e5682: f64 = (p.p786 + assign4210_e5681);
        let assign4210_e5685: f64 = (locals.var_bin_w * p.p794);
        let assign4210_e5686: f64 = (assign4210_e5682 + assign4210_e5685);
        let assign4210_e5689: f64 = (locals.var_bin_wl * p.p798);
        let assign4210_e5690: f64 = (assign4210_e5686 + assign4210_e5689);
        locals.var_cigbcp2_i = assign4210_e5690;
        locals.var_cigbcp2_i_rv = 0.0;

        let assign4220_e5694: f64 = (locals.var_bin_l * p.p1385);
        let assign4220_e5695: f64 = (p.p1384 + assign4220_e5694);
        let assign4220_e5698: f64 = (locals.var_bin_w * p.p1386);
        let assign4220_e5699: f64 = (assign4220_e5695 + assign4220_e5698);
        let assign4220_e5702: f64 = (locals.var_bin_wl * p.p1387);
        let assign4220_e5703: f64 = (assign4220_e5699 + assign4220_e5702);
        locals.var_nsub_i = assign4220_e5703;
        locals.var_nsub_i_rv = 0.0;

        let assign4230_e5707: f64 = (locals.var_bin_l * p.p1390);
        let assign4230_e5708: f64 = (p.p1389 + assign4230_e5707);
        let assign4230_e5711: f64 = (locals.var_bin_w * p.p1391);
        let assign4230_e5712: f64 = (assign4230_e5708 + assign4230_e5711);
        let assign4230_e5715: f64 = (locals.var_bin_wl * p.p1392);
        let assign4230_e5716: f64 = (assign4230_e5712 + assign4230_e5715);
        locals.var_kb1_i = assign4230_e5716;
        locals.var_kb1_i_rv = 0.0;

        let assign4240_e5719: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign4240_e5719;
        locals.var_guard21_rv = 0.0;

        let (assign4250_e5735, assign4250_e5735_d_n3, assign4250_e5735_d_n4, assign4250_e5735_d_n5, assign4250_e5735_d_n6, assign4250_e5735_d_n7, assign4250_e5735_d_n8, assign4250_e5735_d_n9, assign4250_e5735_d_n10, assign4250_e5735_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4250_e5724: f64 = (locals.var_bin_l * p.p1173);
        let assign4250_e5725: f64 = (p.p1172 + assign4250_e5724);
        let assign4250_e5728: f64 = (locals.var_bin_w * p.p1174);
        let assign4250_e5729: f64 = (assign4250_e5725 + assign4250_e5728);
        let assign4250_e5732: f64 = (locals.var_bin_wl * p.p1175);
        let assign4250_e5733: f64 = (assign4250_e5729 + assign4250_e5732);
        (assign4250_e5733, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscdedger_i, locals.var_cdscdedger_i_dn3, locals.var_cdscdedger_i_dn4, locals.var_cdscdedger_i_dn5, locals.var_cdscdedger_i_dn6, locals.var_cdscdedger_i_dn7, locals.var_cdscdedger_i_dn8, locals.var_cdscdedger_i_dn9, locals.var_cdscdedger_i_dn10, locals.var_cdscdedger_i_dn11,)
    }
};
        locals.var_cdscdedger_i = assign4250_e5735;
        locals.var_cdscdedger_i_dn3 = assign4250_e5735_d_n3;
        locals.var_cdscdedger_i_dn4 = assign4250_e5735_d_n4;
        locals.var_cdscdedger_i_dn5 = assign4250_e5735_d_n5;
        locals.var_cdscdedger_i_dn6 = assign4250_e5735_d_n6;
        locals.var_cdscdedger_i_dn7 = assign4250_e5735_d_n7;
        locals.var_cdscdedger_i_dn8 = assign4250_e5735_d_n8;
        locals.var_cdscdedger_i_dn9 = assign4250_e5735_d_n9;
        locals.var_cdscdedger_i_dn10 = assign4250_e5735_d_n10;
        locals.var_cdscdedger_i_dn11 = assign4250_e5735_d_n11;
        locals.var_cdscdedger_i_rv = 0.0;

        let (assign4260_e5751, assign4260_e5751_d_n3, assign4260_e5751_d_n4, assign4260_e5751_d_n5, assign4260_e5751_d_n6, assign4260_e5751_d_n7, assign4260_e5751_d_n8, assign4260_e5751_d_n9, assign4260_e5751_d_n10, assign4260_e5751_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4260_e5740: f64 = (locals.var_bin_l * p.p285);
        let assign4260_e5741: f64 = (p.p284 + assign4260_e5740);
        let assign4260_e5744: f64 = (locals.var_bin_w * p.p286);
        let assign4260_e5745: f64 = (assign4260_e5741 + assign4260_e5744);
        let assign4260_e5748: f64 = (locals.var_bin_wl * p.p287);
        let assign4260_e5749: f64 = (assign4260_e5745 + assign4260_e5748);
        (assign4260_e5749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11,)
    }
};
        locals.var_cdscdr_i = assign4260_e5751;
        locals.var_cdscdr_i_dn3 = assign4260_e5751_d_n3;
        locals.var_cdscdr_i_dn4 = assign4260_e5751_d_n4;
        locals.var_cdscdr_i_dn5 = assign4260_e5751_d_n5;
        locals.var_cdscdr_i_dn6 = assign4260_e5751_d_n6;
        locals.var_cdscdr_i_dn7 = assign4260_e5751_d_n7;
        locals.var_cdscdr_i_dn8 = assign4260_e5751_d_n8;
        locals.var_cdscdr_i_dn9 = assign4260_e5751_d_n9;
        locals.var_cdscdr_i_dn10 = assign4260_e5751_d_n10;
        locals.var_cdscdr_i_dn11 = assign4260_e5751_d_n11;
        locals.var_cdscdr_i_rv = 0.0;

        let (assign4270_e5767, assign4270_e5767_d_n3, assign4270_e5767_d_n4, assign4270_e5767_d_n5, assign4270_e5767_d_n6, assign4270_e5767_d_n7, assign4270_e5767_d_n8, assign4270_e5767_d_n9, assign4270_e5767_d_n10, assign4270_e5767_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4270_e5756: f64 = (locals.var_bin_l * p.p199);
        let assign4270_e5757: f64 = (p.p198 + assign4270_e5756);
        let assign4270_e5760: f64 = (locals.var_bin_w * p.p200);
        let assign4270_e5761: f64 = (assign4270_e5757 + assign4270_e5760);
        let assign4270_e5764: f64 = (locals.var_bin_wl * p.p201);
        let assign4270_e5765: f64 = (assign4270_e5761 + assign4270_e5764);
        (assign4270_e5765, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11,)
    }
};
        locals.var_eta0r_i = assign4270_e5767;
        locals.var_eta0r_i_dn3 = assign4270_e5767_d_n3;
        locals.var_eta0r_i_dn4 = assign4270_e5767_d_n4;
        locals.var_eta0r_i_dn5 = assign4270_e5767_d_n5;
        locals.var_eta0r_i_dn6 = assign4270_e5767_d_n6;
        locals.var_eta0r_i_dn7 = assign4270_e5767_d_n7;
        locals.var_eta0r_i_dn8 = assign4270_e5767_d_n8;
        locals.var_eta0r_i_dn9 = assign4270_e5767_d_n9;
        locals.var_eta0r_i_dn10 = assign4270_e5767_d_n10;
        locals.var_eta0r_i_dn11 = assign4270_e5767_d_n11;
        locals.var_eta0r_i_rv = 0.0;

        let (assign4280_e5783,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4280_e5772: f64 = (locals.var_bin_l * p.p344);
        let assign4280_e5773: f64 = (p.p343 + assign4280_e5772);
        let assign4280_e5776: f64 = (locals.var_bin_w * p.p345);
        let assign4280_e5777: f64 = (assign4280_e5773 + assign4280_e5776);
        let assign4280_e5780: f64 = (locals.var_bin_wl * p.p346);
        let assign4280_e5781: f64 = (assign4280_e5777 + assign4280_e5780);
        (assign4280_e5781,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4280_e5783;
        locals.var_u0r_i_rv = 0.0;

        let (assign4290_e5799, assign4290_e5799_d_n3, assign4290_e5799_d_n4, assign4290_e5799_d_n5, assign4290_e5799_d_n6, assign4290_e5799_d_n7, assign4290_e5799_d_n8, assign4290_e5799_d_n9, assign4290_e5799_d_n10, assign4290_e5799_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4290_e5788: f64 = (locals.var_bin_l * p.p359);
        let assign4290_e5789: f64 = (p.p358 + assign4290_e5788);
        let assign4290_e5792: f64 = (locals.var_bin_w * p.p360);
        let assign4290_e5793: f64 = (assign4290_e5789 + assign4290_e5792);
        let assign4290_e5796: f64 = (locals.var_bin_wl * p.p361);
        let assign4290_e5797: f64 = (assign4290_e5793 + assign4290_e5796);
        (assign4290_e5797, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11,)
    }
};
        locals.var_uar_i = assign4290_e5799;
        locals.var_uar_i_dn3 = assign4290_e5799_d_n3;
        locals.var_uar_i_dn4 = assign4290_e5799_d_n4;
        locals.var_uar_i_dn5 = assign4290_e5799_d_n5;
        locals.var_uar_i_dn6 = assign4290_e5799_d_n6;
        locals.var_uar_i_dn7 = assign4290_e5799_d_n7;
        locals.var_uar_i_dn8 = assign4290_e5799_d_n8;
        locals.var_uar_i_dn9 = assign4290_e5799_d_n9;
        locals.var_uar_i_dn10 = assign4290_e5799_d_n10;
        locals.var_uar_i_dn11 = assign4290_e5799_d_n11;
        locals.var_uar_i_rv = 0.0;

        let (assign4300_e5815, assign4300_e5815_d_n3, assign4300_e5815_d_n4, assign4300_e5815_d_n5, assign4300_e5815_d_n6, assign4300_e5815_d_n7, assign4300_e5815_d_n8, assign4300_e5815_d_n9, assign4300_e5815_d_n10, assign4300_e5815_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4300_e5804: f64 = (locals.var_bin_l * p.p379);
        let assign4300_e5805: f64 = (p.p378 + assign4300_e5804);
        let assign4300_e5808: f64 = (locals.var_bin_w * p.p380);
        let assign4300_e5809: f64 = (assign4300_e5805 + assign4300_e5808);
        let assign4300_e5812: f64 = (locals.var_bin_wl * p.p381);
        let assign4300_e5813: f64 = (assign4300_e5809 + assign4300_e5812);
        (assign4300_e5813, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11,)
    }
};
        locals.var_udr_i = assign4300_e5815;
        locals.var_udr_i_dn3 = assign4300_e5815_d_n3;
        locals.var_udr_i_dn4 = assign4300_e5815_d_n4;
        locals.var_udr_i_dn5 = assign4300_e5815_d_n5;
        locals.var_udr_i_dn6 = assign4300_e5815_d_n6;
        locals.var_udr_i_dn7 = assign4300_e5815_d_n7;
        locals.var_udr_i_dn8 = assign4300_e5815_d_n8;
        locals.var_udr_i_dn9 = assign4300_e5815_d_n9;
        locals.var_udr_i_dn10 = assign4300_e5815_d_n10;
        locals.var_udr_i_dn11 = assign4300_e5815_d_n11;
        locals.var_udr_i_rv = 0.0;

        let (assign4310_e5831,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4310_e5820: f64 = (locals.var_bin_l * p.p387);
        let assign4310_e5821: f64 = (p.p386 + assign4310_e5820);
        let assign4310_e5824: f64 = (locals.var_bin_w * p.p388);
        let assign4310_e5825: f64 = (assign4310_e5821 + assign4310_e5824);
        let assign4310_e5828: f64 = (locals.var_bin_wl * p.p389);
        let assign4310_e5829: f64 = (assign4310_e5825 + assign4310_e5828);
        (assign4310_e5829,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign4310_e5831;
        locals.var_ucsr_i_rv = 0.0;

        let (assign4320_e5847, assign4320_e5847_d_n3, assign4320_e5847_d_n4, assign4320_e5847_d_n5, assign4320_e5847_d_n6, assign4320_e5847_d_n7, assign4320_e5847_d_n8, assign4320_e5847_d_n9, assign4320_e5847_d_n10, assign4320_e5847_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4320_e5836: f64 = (locals.var_bin_l * p.p401);
        let assign4320_e5837: f64 = (p.p400 + assign4320_e5836);
        let assign4320_e5840: f64 = (locals.var_bin_w * p.p402);
        let assign4320_e5841: f64 = (assign4320_e5837 + assign4320_e5840);
        let assign4320_e5844: f64 = (locals.var_bin_wl * p.p403);
        let assign4320_e5845: f64 = (assign4320_e5841 + assign4320_e5844);
        (assign4320_e5845, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11,)
    }
};
        locals.var_ucr_i = assign4320_e5847;
        locals.var_ucr_i_dn3 = assign4320_e5847_d_n3;
        locals.var_ucr_i_dn4 = assign4320_e5847_d_n4;
        locals.var_ucr_i_dn5 = assign4320_e5847_d_n5;
        locals.var_ucr_i_dn6 = assign4320_e5847_d_n6;
        locals.var_ucr_i_dn7 = assign4320_e5847_d_n7;
        locals.var_ucr_i_dn8 = assign4320_e5847_d_n8;
        locals.var_ucr_i_dn9 = assign4320_e5847_d_n9;
        locals.var_ucr_i_dn10 = assign4320_e5847_d_n10;
        locals.var_ucr_i_dn11 = assign4320_e5847_d_n11;
        locals.var_ucr_i_rv = 0.0;

        let (assign4330_e5863, assign4330_e5863_d_n3, assign4330_e5863_d_n4, assign4330_e5863_d_n5, assign4330_e5863_d_n6, assign4330_e5863_d_n7, assign4330_e5863_d_n8, assign4330_e5863_d_n9, assign4330_e5863_d_n10, assign4330_e5863_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4330_e5852: f64 = (locals.var_bin_l * p.p411);
        let assign4330_e5853: f64 = (p.p410 + assign4330_e5852);
        let assign4330_e5856: f64 = (locals.var_bin_w * p.p412);
        let assign4330_e5857: f64 = (assign4330_e5853 + assign4330_e5856);
        let assign4330_e5860: f64 = (locals.var_bin_wl * p.p413);
        let assign4330_e5861: f64 = (assign4330_e5857 + assign4330_e5860);
        (assign4330_e5861, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11,)
    }
};
        locals.var_pclmr_i = assign4330_e5863;
        locals.var_pclmr_i_dn3 = assign4330_e5863_d_n3;
        locals.var_pclmr_i_dn4 = assign4330_e5863_d_n4;
        locals.var_pclmr_i_dn5 = assign4330_e5863_d_n5;
        locals.var_pclmr_i_dn6 = assign4330_e5863_d_n6;
        locals.var_pclmr_i_dn7 = assign4330_e5863_d_n7;
        locals.var_pclmr_i_dn8 = assign4330_e5863_d_n8;
        locals.var_pclmr_i_dn9 = assign4330_e5863_d_n9;
        locals.var_pclmr_i_dn10 = assign4330_e5863_d_n10;
        locals.var_pclmr_i_dn11 = assign4330_e5863_d_n11;
        locals.var_pclmr_i_rv = 0.0;

        let (assign4340_e5879, assign4340_e5879_d_n3, assign4340_e5879_d_n4, assign4340_e5879_d_n5, assign4340_e5879_d_n6, assign4340_e5879_d_n7, assign4340_e5879_d_n8, assign4340_e5879_d_n9, assign4340_e5879_d_n10, assign4340_e5879_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4340_e5868: f64 = (locals.var_bin_l * p.p537);
        let assign4340_e5869: f64 = (p.p536 + assign4340_e5868);
        let assign4340_e5872: f64 = (locals.var_bin_w * p.p538);
        let assign4340_e5873: f64 = (assign4340_e5869 + assign4340_e5872);
        let assign4340_e5876: f64 = (locals.var_bin_wl * p.p539);
        let assign4340_e5877: f64 = (assign4340_e5873 + assign4340_e5876);
        (assign4340_e5877, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11,)
    }
};
        locals.var_pdiblcr_i = assign4340_e5879;
        locals.var_pdiblcr_i_dn3 = assign4340_e5879_d_n3;
        locals.var_pdiblcr_i_dn4 = assign4340_e5879_d_n4;
        locals.var_pdiblcr_i_dn5 = assign4340_e5879_d_n5;
        locals.var_pdiblcr_i_dn6 = assign4340_e5879_d_n6;
        locals.var_pdiblcr_i_dn7 = assign4340_e5879_d_n7;
        locals.var_pdiblcr_i_dn8 = assign4340_e5879_d_n8;
        locals.var_pdiblcr_i_dn9 = assign4340_e5879_d_n9;
        locals.var_pdiblcr_i_dn10 = assign4340_e5879_d_n10;
        locals.var_pdiblcr_i_dn11 = assign4340_e5879_d_n11;
        locals.var_pdiblcr_i_rv = 0.0;

        let (assign4350_e5895, assign4350_e5895_d_n3, assign4350_e5895_d_n4, assign4350_e5895_d_n5, assign4350_e5895_d_n6, assign4350_e5895_d_n7, assign4350_e5895_d_n8, assign4350_e5895_d_n9, assign4350_e5895_d_n10, assign4350_e5895_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4350_e5884: f64 = (locals.var_bin_l * p.p306);
        let assign4350_e5885: f64 = (p.p305 + assign4350_e5884);
        let assign4350_e5888: f64 = (locals.var_bin_w * p.p307);
        let assign4350_e5889: f64 = (assign4350_e5885 + assign4350_e5888);
        let assign4350_e5892: f64 = (locals.var_bin_wl * p.p308);
        let assign4350_e5893: f64 = (assign4350_e5889 + assign4350_e5892);
        (assign4350_e5893, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11,)
    }
};
        locals.var_vsatr_i = assign4350_e5895;
        locals.var_vsatr_i_dn3 = assign4350_e5895_d_n3;
        locals.var_vsatr_i_dn4 = assign4350_e5895_d_n4;
        locals.var_vsatr_i_dn5 = assign4350_e5895_d_n5;
        locals.var_vsatr_i_dn6 = assign4350_e5895_d_n6;
        locals.var_vsatr_i_dn7 = assign4350_e5895_d_n7;
        locals.var_vsatr_i_dn8 = assign4350_e5895_d_n8;
        locals.var_vsatr_i_dn9 = assign4350_e5895_d_n9;
        locals.var_vsatr_i_dn10 = assign4350_e5895_d_n10;
        locals.var_vsatr_i_dn11 = assign4350_e5895_d_n11;
        locals.var_vsatr_i_rv = 0.0;

        let (assign4360_e5911,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4360_e5900: f64 = (locals.var_bin_l * p.p491);
        let assign4360_e5901: f64 = (p.p490 + assign4360_e5900);
        let assign4360_e5904: f64 = (locals.var_bin_w * p.p492);
        let assign4360_e5905: f64 = (assign4360_e5901 + assign4360_e5904);
        let assign4360_e5908: f64 = (locals.var_bin_wl * p.p493);
        let assign4360_e5909: f64 = (assign4360_e5905 + assign4360_e5908);
        (assign4360_e5909,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign4360_e5911;
        locals.var_psatr_i_rv = 0.0;

        let (assign4370_e5927, assign4370_e5927_d_n3, assign4370_e5927_d_n4, assign4370_e5927_d_n5, assign4370_e5927_d_n6, assign4370_e5927_d_n7, assign4370_e5927_d_n8, assign4370_e5927_d_n9, assign4370_e5927_d_n10, assign4370_e5927_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4370_e5916: f64 = (locals.var_bin_l * p.p507);
        let assign4370_e5917: f64 = (p.p506 + assign4370_e5916);
        let assign4370_e5920: f64 = (locals.var_bin_w * p.p508);
        let assign4370_e5921: f64 = (assign4370_e5917 + assign4370_e5920);
        let assign4370_e5924: f64 = (locals.var_bin_wl * p.p509);
        let assign4370_e5925: f64 = (assign4370_e5921 + assign4370_e5924);
        (assign4370_e5925, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11,)
    }
};
        locals.var_ptwgr_i = assign4370_e5927;
        locals.var_ptwgr_i_dn3 = assign4370_e5927_d_n3;
        locals.var_ptwgr_i_dn4 = assign4370_e5927_d_n4;
        locals.var_ptwgr_i_dn5 = assign4370_e5927_d_n5;
        locals.var_ptwgr_i_dn6 = assign4370_e5927_d_n6;
        locals.var_ptwgr_i_dn7 = assign4370_e5927_d_n7;
        locals.var_ptwgr_i_dn8 = assign4370_e5927_d_n8;
        locals.var_ptwgr_i_dn9 = assign4370_e5927_d_n9;
        locals.var_ptwgr_i_dn10 = assign4370_e5927_d_n10;
        locals.var_ptwgr_i_dn11 = assign4370_e5927_d_n11;
        locals.var_ptwgr_i_rv = 0.0;

        let assign4380_e5931: f64 = (locals.var_inv_l).powf(p.p81);
        let assign4380_e5934: f64 = (locals.var_inv_llong).powf(p.p81);
        let assign4380_e5935: f64 = (assign4380_e5931 - assign4380_e5934);
        let assign4380_e5937: f64 = (assign4380_e5935).max(0.0);
        let assign4380_e5938: f64 = (p.p80 * assign4380_e5937);
        let assign4380_e5942: f64 = (locals.var_inv_l).powf(p.p83);
        let assign4380_e5945: f64 = (locals.var_inv_llong).powf(p.p83);
        let assign4380_e5946: f64 = (assign4380_e5942 - assign4380_e5945);
        let assign4380_e5948: f64 = (assign4380_e5946).max(0.0);
        let assign4380_e5949: f64 = (p.p82 * assign4380_e5948);
        let assign4380_e5950: f64 = (assign4380_e5938 + assign4380_e5949);
        locals.var_t0 = assign4380_e5950;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4390_e5954: f64 = (locals.var_inv_w).powf(p.p85);
        let assign4390_e5957: f64 = (locals.var_inv_wwide).powf(p.p85);
        let assign4390_e5958: f64 = (assign4390_e5954 - assign4390_e5957);
        let assign4390_e5960: f64 = (assign4390_e5958).max(0.0);
        let assign4390_e5961: f64 = (p.p84 * assign4390_e5960);
        let assign4390_e5965: f64 = (locals.var_inv_w * locals.var_inv_l);
        let assign4390_e5967: f64 = (assign4390_e5965).powf(p.p87);
        let assign4390_e5968: f64 = (p.p86 * assign4390_e5967);
        let assign4390_e5969: f64 = (assign4390_e5961 + assign4390_e5968);
        locals.var_t1 = assign4390_e5969;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4400_e5973: f64 = (1.0 + locals.var_t0);
        let assign4400_e5975: f64 = (assign4400_e5973 + locals.var_t1);
        let assign4400_e5976: f64 = (locals.var_ndep_i * assign4400_e5975);
        locals.var_ndep_i = assign4400_e5976;
        locals.var_ndep_i_dn3 = ((locals.var_ndep_i_dn3 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndep_i_dn4 = ((locals.var_ndep_i_dn4 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndep_i_dn5 = ((locals.var_ndep_i_dn5 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndep_i_dn6 = ((locals.var_ndep_i_dn6 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndep_i_dn7 = ((locals.var_ndep_i_dn7 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndep_i_dn8 = ((locals.var_ndep_i_dn8 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndep_i_dn9 = ((locals.var_ndep_i_dn9 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndep_i_dn10 = ((locals.var_ndep_i_dn10 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndep_i_dn11 = ((locals.var_ndep_i_dn11 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndep_i_rv = 0.0;

        let assign4410_e5980: f64 = (locals.var_inv_l).powf(p.p238);
        let assign4410_e5983: f64 = (locals.var_inv_llong).powf(p.p238);
        let assign4410_e5984: f64 = (assign4410_e5980 - assign4410_e5983);
        let assign4410_e5986: f64 = (assign4410_e5984).max(0.0);
        let assign4410_e5987: f64 = (p.p237 * assign4410_e5986);
        locals.var_t0 = assign4410_e5987;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4420_e5991: f64 = (locals.var_inv_w).powf(p.p240);
        let assign4420_e5994: f64 = (locals.var_inv_wwide).powf(p.p240);
        let assign4420_e5995: f64 = (assign4420_e5991 - assign4420_e5994);
        let assign4420_e5997: f64 = (assign4420_e5995).max(0.0);
        let assign4420_e5998: f64 = (p.p239 * assign4420_e5997);
        let assign4420_e6002: f64 = (locals.var_inv_wl).powf(p.p242);
        let assign4420_e6003: f64 = (p.p241 * assign4420_e6002);
        let assign4420_e6004: f64 = (assign4420_e5998 + assign4420_e6003);
        locals.var_t1 = assign4420_e6004;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4430_e6008: f64 = (1.0 + locals.var_t0);
        let assign4430_e6010: f64 = (assign4430_e6008 + locals.var_t1);
        let assign4430_e6011: f64 = (locals.var_nfactor_i * assign4430_e6010);
        locals.var_nfactor_i = assign4430_e6011;
        locals.var_nfactor_i_dn3 = ((locals.var_nfactor_i_dn3 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_nfactor_i_dn4 = ((locals.var_nfactor_i_dn4 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_nfactor_i_dn5 = ((locals.var_nfactor_i_dn5 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_nfactor_i_dn6 = ((locals.var_nfactor_i_dn6 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_nfactor_i_dn7 = ((locals.var_nfactor_i_dn7 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_nfactor_i_dn8 = ((locals.var_nfactor_i_dn8 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_nfactor_i_dn9 = ((locals.var_nfactor_i_dn9 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_nfactor_i_dn10 = ((locals.var_nfactor_i_dn10 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_nfactor_i_dn11 = ((locals.var_nfactor_i_dn11 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_nfactor_i_rv = 0.0;

        let assign4440_e6016: f64 = (locals.var_inv_l).powf(p.p283);
        let assign4440_e6019: f64 = (locals.var_inv_llong).powf(p.p283);
        let assign4440_e6020: f64 = (assign4440_e6016 - assign4440_e6019);
        let assign4440_e6022: f64 = (assign4440_e6020).max(0.0);
        let assign4440_e6023: f64 = (p.p282 * assign4440_e6022);
        let assign4440_e6024: f64 = (1.0 + assign4440_e6023);
        locals.var_t0 = assign4440_e6024;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4450_e6027: f64 = (locals.var_cdscd_i * locals.var_t0);
        locals.var_cdscd_i = assign4450_e6027;
        locals.var_cdscd_i_dn3 = ((locals.var_cdscd_i_dn3 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn3));
        locals.var_cdscd_i_dn4 = ((locals.var_cdscd_i_dn4 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn4));
        locals.var_cdscd_i_dn5 = ((locals.var_cdscd_i_dn5 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn5));
        locals.var_cdscd_i_dn6 = ((locals.var_cdscd_i_dn6 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn6));
        locals.var_cdscd_i_dn7 = ((locals.var_cdscd_i_dn7 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn7));
        locals.var_cdscd_i_dn8 = ((locals.var_cdscd_i_dn8 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn8));
        locals.var_cdscd_i_dn9 = ((locals.var_cdscd_i_dn9 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn9));
        locals.var_cdscd_i_dn10 = ((locals.var_cdscd_i_dn10 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn10));
        locals.var_cdscd_i_dn11 = ((locals.var_cdscd_i_dn11 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn11));
        locals.var_cdscd_i_rv = 0.0;

        let assign4460_e6030: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign4460_e6030;
        locals.var_guard22_rv = 0.0;

        let (assign4470_e6036, assign4470_e6036_d_n3, assign4470_e6036_d_n4, assign4470_e6036_d_n5, assign4470_e6036_d_n6, assign4470_e6036_d_n7, assign4470_e6036_d_n8, assign4470_e6036_d_n9, assign4470_e6036_d_n10, assign4470_e6036_d_n11,) = {
    if (locals.var_guard22 != 0.0) {
        let assign4470_e6034: f64 = (locals.var_cdscdedger_i * locals.var_t0);
        (assign4470_e6034, ((locals.var_cdscdedger_i_dn3 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn3)), ((locals.var_cdscdedger_i_dn4 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn4)), ((locals.var_cdscdedger_i_dn5 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn5)), ((locals.var_cdscdedger_i_dn6 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn6)), ((locals.var_cdscdedger_i_dn7 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn7)), ((locals.var_cdscdedger_i_dn8 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn8)), ((locals.var_cdscdedger_i_dn9 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn9)), ((locals.var_cdscdedger_i_dn10 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn10)), ((locals.var_cdscdedger_i_dn11 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_cdscdedger_i, locals.var_cdscdedger_i_dn3, locals.var_cdscdedger_i_dn4, locals.var_cdscdedger_i_dn5, locals.var_cdscdedger_i_dn6, locals.var_cdscdedger_i_dn7, locals.var_cdscdedger_i_dn8, locals.var_cdscdedger_i_dn9, locals.var_cdscdedger_i_dn10, locals.var_cdscdedger_i_dn11,)
    }
};
        locals.var_cdscdedger_i = assign4470_e6036;
        locals.var_cdscdedger_i_dn3 = assign4470_e6036_d_n3;
        locals.var_cdscdedger_i_dn4 = assign4470_e6036_d_n4;
        locals.var_cdscdedger_i_dn5 = assign4470_e6036_d_n5;
        locals.var_cdscdedger_i_dn6 = assign4470_e6036_d_n6;
        locals.var_cdscdedger_i_dn7 = assign4470_e6036_d_n7;
        locals.var_cdscdedger_i_dn8 = assign4470_e6036_d_n8;
        locals.var_cdscdedger_i_dn9 = assign4470_e6036_d_n9;
        locals.var_cdscdedger_i_dn10 = assign4470_e6036_d_n10;
        locals.var_cdscdedger_i_dn11 = assign4470_e6036_d_n11;
        locals.var_cdscdedger_i_rv = 0.0;

        let (assign4480_e6042, assign4480_e6042_d_n3, assign4480_e6042_d_n4, assign4480_e6042_d_n5, assign4480_e6042_d_n6, assign4480_e6042_d_n7, assign4480_e6042_d_n8, assign4480_e6042_d_n9, assign4480_e6042_d_n10, assign4480_e6042_d_n11,) = {
    if (locals.var_guard22 != 0.0) {
        let assign4480_e6040: f64 = (locals.var_cdscdr_i * locals.var_t0);
        (assign4480_e6040, ((locals.var_cdscdr_i_dn3 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn3)), ((locals.var_cdscdr_i_dn4 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn4)), ((locals.var_cdscdr_i_dn5 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn5)), ((locals.var_cdscdr_i_dn6 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn6)), ((locals.var_cdscdr_i_dn7 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn7)), ((locals.var_cdscdr_i_dn8 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn8)), ((locals.var_cdscdr_i_dn9 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn9)), ((locals.var_cdscdr_i_dn10 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn10)), ((locals.var_cdscdr_i_dn11 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11,)
    }
};
        locals.var_cdscdr_i = assign4480_e6042;
        locals.var_cdscdr_i_dn3 = assign4480_e6042_d_n3;
        locals.var_cdscdr_i_dn4 = assign4480_e6042_d_n4;
        locals.var_cdscdr_i_dn5 = assign4480_e6042_d_n5;
        locals.var_cdscdr_i_dn6 = assign4480_e6042_d_n6;
        locals.var_cdscdr_i_dn7 = assign4480_e6042_d_n7;
        locals.var_cdscdr_i_dn8 = assign4480_e6042_d_n8;
        locals.var_cdscdr_i_dn9 = assign4480_e6042_d_n9;
        locals.var_cdscdr_i_dn10 = assign4480_e6042_d_n10;
        locals.var_cdscdr_i_dn11 = assign4480_e6042_d_n11;
        locals.var_cdscdr_i_rv = 0.0;

        let assign4490_e6048: f64 = (locals.var_inv_l).powf(p.p290);
        let assign4490_e6051: f64 = (locals.var_inv_llong).powf(p.p290);
        let assign4490_e6052: f64 = (assign4490_e6048 - assign4490_e6051);
        let assign4490_e6054: f64 = (assign4490_e6052).max(0.0);
        let assign4490_e6055: f64 = (p.p289 * assign4490_e6054);
        let assign4490_e6056: f64 = (1.0 + assign4490_e6055);
        let assign4490_e6057: f64 = (locals.var_cdscb_i * assign4490_e6056);
        locals.var_cdscb_i = assign4490_e6057;
        locals.var_cdscb_i_rv = 0.0;

        let assign4500_e6060: f64 = (p.p24 * locals.var_u0_i);
        locals.var_u0_i = assign4500_e6060;
        locals.var_u0_i_rv = 0.0;

        let assign4510_e6063: f64 = if p.p42 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign4510_e6063;
        locals.var_guard23_rv = 0.0;

        let assign4520_e6066: f64 = if p.p339 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign4520_e6066;
        locals.var_guard24_rv = 0.0;

        let (assign4530_e6086,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) {
        let assign4530_e6075: f64 = (locals.var_inv_l).powf(p.p339);
        let assign4530_e6078: f64 = (locals.var_inv_llong).powf(p.p339);
        let assign4530_e6079: f64 = (assign4530_e6075 - assign4530_e6078);
        let assign4530_e6081: f64 = (assign4530_e6079).max(0.0);
        let assign4530_e6082: f64 = (p.p338 * assign4530_e6081);
        let assign4530_e6083: f64 = (1.0 - assign4530_e6082);
        let assign4530_e6084: f64 = (locals.var_u0_i * assign4530_e6083);
        (assign4530_e6084,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4530_e6086;
        locals.var_u0_i_rv = 0.0;

        let assign4540_e6089: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign4540_e6089;
        locals.var_guard25_rv = 0.0;

        let (assign4550_e6111,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) && (locals.var_guard25 != 0.0)) {
        let assign4550_e6100: f64 = (locals.var_inv_l).powf(p.p339);
        let assign4550_e6103: f64 = (locals.var_inv_llong).powf(p.p339);
        let assign4550_e6104: f64 = (assign4550_e6100 - assign4550_e6103);
        let assign4550_e6106: f64 = (assign4550_e6104).max(0.0);
        let assign4550_e6107: f64 = (p.p338 * assign4550_e6106);
        let assign4550_e6108: f64 = (1.0 - assign4550_e6107);
        let assign4550_e6109: f64 = (locals.var_u0r_i * assign4550_e6108);
        (assign4550_e6109,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4550_e6111;
        locals.var_u0r_i_rv = 0.0;

        let (assign4560_e6122,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) {
        let assign4560_e6119: f64 = (1.0 - p.p338);
        let assign4560_e6120: f64 = (locals.var_u0_i * assign4560_e6119);
        (assign4560_e6120,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4560_e6122;
        locals.var_u0_i_rv = 0.0;

        let assign4570_e6125: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign4570_e6125;
        locals.var_guard26_rv = 0.0;

        let (assign4580_e6138,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) && (locals.var_guard26 != 0.0)) {
        let assign4580_e6135: f64 = (1.0 - p.p338);
        let assign4580_e6136: f64 = (locals.var_u0r_i * assign4580_e6135);
        (assign4580_e6136,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4580_e6138;
        locals.var_u0r_i_rv = 0.0;

        let (assign4590_e6161,) = {
    if (locals.var_guard23 == 0.0) {
        let assign4590_e6145: f64 = (-locals.var_leff);
        let assign4590_e6147: f64 = (assign4590_e6145 / p.p334);
        let assign4590_e6148: f64 = { let limited_exp_arg = assign4590_e6147; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4590_e6149: f64 = (p.p333 * assign4590_e6148);
        let assign4590_e6150: f64 = (1.0 - assign4590_e6149);
        let assign4590_e6153: f64 = (-locals.var_leff);
        let assign4590_e6155: f64 = (assign4590_e6153 / p.p336);
        let assign4590_e6156: f64 = { let limited_exp_arg = assign4590_e6155; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4590_e6157: f64 = (p.p335 * assign4590_e6156);
        let assign4590_e6158: f64 = (assign4590_e6150 - assign4590_e6157);
        let assign4590_e6159: f64 = (locals.var_u0_i * assign4590_e6158);
        (assign4590_e6159,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4590_e6161;
        locals.var_u0_i_rv = 0.0;

        let assign4600_e6164: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign4600_e6164;
        locals.var_guard27_rv = 0.0;

        let (assign4610_e6189,) = {
    if ((locals.var_guard23 == 0.0) && (locals.var_guard27 != 0.0)) {
        let assign4610_e6173: f64 = (-locals.var_leff);
        let assign4610_e6175: f64 = (assign4610_e6173 / p.p334);
        let assign4610_e6176: f64 = { let limited_exp_arg = assign4610_e6175; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4610_e6177: f64 = (p.p333 * assign4610_e6176);
        let assign4610_e6178: f64 = (1.0 - assign4610_e6177);
        let assign4610_e6181: f64 = (-locals.var_leff);
        let assign4610_e6183: f64 = (assign4610_e6181 / p.p336);
        let assign4610_e6184: f64 = { let limited_exp_arg = assign4610_e6183; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4610_e6185: f64 = (p.p335 * assign4610_e6184);
        let assign4610_e6186: f64 = (assign4610_e6178 - assign4610_e6185);
        let assign4610_e6187: f64 = (locals.var_u0r_i * assign4610_e6186);
        (assign4610_e6187,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4610_e6189;
        locals.var_u0r_i_rv = 0.0;

        let assign4620_e6193: f64 = (locals.var_inv_l).powf(p.p350);
        let assign4620_e6196: f64 = (locals.var_inv_llong).powf(p.p350);
        let assign4620_e6197: f64 = (assign4620_e6193 - assign4620_e6196);
        let assign4620_e6199: f64 = (assign4620_e6197).max(0.0);
        let assign4620_e6200: f64 = (p.p349 * assign4620_e6199);
        locals.var_t0 = assign4620_e6200;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4630_e6204: f64 = (locals.var_inv_w).powf(p.p352);
        let assign4630_e6207: f64 = (locals.var_inv_wwide).powf(p.p352);
        let assign4630_e6208: f64 = (assign4630_e6204 - assign4630_e6207);
        let assign4630_e6210: f64 = (assign4630_e6208).max(0.0);
        let assign4630_e6211: f64 = (p.p351 * assign4630_e6210);
        let assign4630_e6215: f64 = (locals.var_inv_wl).powf(p.p354);
        let assign4630_e6216: f64 = (p.p353 * assign4630_e6215);
        let assign4630_e6217: f64 = (assign4630_e6211 + assign4630_e6216);
        locals.var_t1 = assign4630_e6217;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4640_e6221: f64 = (1.0 + locals.var_t0);
        let assign4640_e6223: f64 = (assign4640_e6221 + locals.var_t1);
        let assign4640_e6224: f64 = (locals.var_ua_i * assign4640_e6223);
        locals.var_ua_i = assign4640_e6224;
        locals.var_ua_i_dn3 = ((locals.var_ua_i_dn3 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ua_i_dn4 = ((locals.var_ua_i_dn4 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ua_i_dn5 = ((locals.var_ua_i_dn5 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ua_i_dn6 = ((locals.var_ua_i_dn6 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ua_i_dn7 = ((locals.var_ua_i_dn7 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ua_i_dn8 = ((locals.var_ua_i_dn8 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ua_i_dn9 = ((locals.var_ua_i_dn9 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ua_i_dn10 = ((locals.var_ua_i_dn10 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ua_i_dn11 = ((locals.var_ua_i_dn11 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ua_i_rv = 0.0;

        let assign4650_e6227: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign4650_e6227;
        locals.var_guard28_rv = 0.0;

        let (assign4660_e6237, assign4660_e6237_d_n3, assign4660_e6237_d_n4, assign4660_e6237_d_n5, assign4660_e6237_d_n6, assign4660_e6237_d_n7, assign4660_e6237_d_n8, assign4660_e6237_d_n9, assign4660_e6237_d_n10, assign4660_e6237_d_n11,) = {
    if (locals.var_guard28 != 0.0) {
        let assign4660_e6232: f64 = (1.0 + locals.var_t0);
        let assign4660_e6234: f64 = (assign4660_e6232 + locals.var_t1);
        let assign4660_e6235: f64 = (locals.var_uar_i * assign4660_e6234);
        (assign4660_e6235, ((locals.var_uar_i_dn3 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_uar_i_dn4 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_uar_i_dn5 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_uar_i_dn6 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_uar_i_dn7 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_uar_i_dn8 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_uar_i_dn9 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_uar_i_dn10 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_uar_i_dn11 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn11 + locals.var_t1_dn11))),)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11,)
    }
};
        locals.var_uar_i = assign4660_e6237;
        locals.var_uar_i_dn3 = assign4660_e6237_d_n3;
        locals.var_uar_i_dn4 = assign4660_e6237_d_n4;
        locals.var_uar_i_dn5 = assign4660_e6237_d_n5;
        locals.var_uar_i_dn6 = assign4660_e6237_d_n6;
        locals.var_uar_i_dn7 = assign4660_e6237_d_n7;
        locals.var_uar_i_dn8 = assign4660_e6237_d_n8;
        locals.var_uar_i_dn9 = assign4660_e6237_d_n9;
        locals.var_uar_i_dn10 = assign4660_e6237_d_n10;
        locals.var_uar_i_dn11 = assign4660_e6237_d_n11;
        locals.var_uar_i_rv = 0.0;

        let assign4670_e6241: f64 = (locals.var_inv_l).powf(p.p367);
        let assign4670_e6244: f64 = (locals.var_inv_llong).powf(p.p367);
        let assign4670_e6245: f64 = (assign4670_e6241 - assign4670_e6244);
        let assign4670_e6247: f64 = (assign4670_e6245).max(0.0);
        let assign4670_e6248: f64 = (p.p366 * assign4670_e6247);
        locals.var_t0 = assign4670_e6248;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4680_e6252: f64 = (locals.var_inv_w).powf(p.p369);
        let assign4680_e6255: f64 = (locals.var_inv_wwide).powf(p.p369);
        let assign4680_e6256: f64 = (assign4680_e6252 - assign4680_e6255);
        let assign4680_e6258: f64 = (assign4680_e6256).max(0.0);
        let assign4680_e6259: f64 = (p.p368 * assign4680_e6258);
        let assign4680_e6263: f64 = (locals.var_inv_wl).powf(p.p371);
        let assign4680_e6264: f64 = (p.p370 * assign4680_e6263);
        let assign4680_e6265: f64 = (assign4680_e6259 + assign4680_e6264);
        locals.var_t1 = assign4680_e6265;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4690_e6269: f64 = (1.0 + locals.var_t0);
        let assign4690_e6271: f64 = (assign4690_e6269 + locals.var_t1);
        let assign4690_e6272: f64 = (locals.var_eu_i * assign4690_e6271);
        locals.var_eu_i = assign4690_e6272;
        locals.var_eu_i_dn3 = ((locals.var_eu_i_dn3 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_eu_i_dn4 = ((locals.var_eu_i_dn4 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_eu_i_dn5 = ((locals.var_eu_i_dn5 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_eu_i_dn6 = ((locals.var_eu_i_dn6 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_eu_i_dn7 = ((locals.var_eu_i_dn7 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_eu_i_dn8 = ((locals.var_eu_i_dn8 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_eu_i_dn9 = ((locals.var_eu_i_dn9 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_eu_i_dn10 = ((locals.var_eu_i_dn10 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_eu_i_dn11 = ((locals.var_eu_i_dn11 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_eu_i_rv = 0.0;

        let assign4700_e6277: f64 = (locals.var_inv_l).powf(p.p374);
        let assign4700_e6280: f64 = (locals.var_inv_llong).powf(p.p374);
        let assign4700_e6281: f64 = (assign4700_e6277 - assign4700_e6280);
        let assign4700_e6283: f64 = (assign4700_e6281).max(0.0);
        let assign4700_e6284: f64 = (p.p373 * assign4700_e6283);
        let assign4700_e6285: f64 = (1.0 + assign4700_e6284);
        locals.var_t0 = assign4700_e6285;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4710_e6288: f64 = (locals.var_ud_i * locals.var_t0);
        locals.var_ud_i = assign4710_e6288;
        locals.var_ud_i_dn3 = ((locals.var_ud_i_dn3 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn3));
        locals.var_ud_i_dn4 = ((locals.var_ud_i_dn4 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn4));
        locals.var_ud_i_dn5 = ((locals.var_ud_i_dn5 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn5));
        locals.var_ud_i_dn6 = ((locals.var_ud_i_dn6 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn6));
        locals.var_ud_i_dn7 = ((locals.var_ud_i_dn7 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn7));
        locals.var_ud_i_dn8 = ((locals.var_ud_i_dn8 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn8));
        locals.var_ud_i_dn9 = ((locals.var_ud_i_dn9 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn9));
        locals.var_ud_i_dn10 = ((locals.var_ud_i_dn10 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn10));
        locals.var_ud_i_dn11 = ((locals.var_ud_i_dn11 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn11));
        locals.var_ud_i_rv = 0.0;

        let assign4720_e6291: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign4720_e6291;
        locals.var_guard29_rv = 0.0;

        let (assign4730_e6297, assign4730_e6297_d_n3, assign4730_e6297_d_n4, assign4730_e6297_d_n5, assign4730_e6297_d_n6, assign4730_e6297_d_n7, assign4730_e6297_d_n8, assign4730_e6297_d_n9, assign4730_e6297_d_n10, assign4730_e6297_d_n11,) = {
    if (locals.var_guard29 != 0.0) {
        let assign4730_e6295: f64 = (locals.var_udr_i * locals.var_t0);
        (assign4730_e6295, ((locals.var_udr_i_dn3 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn3)), ((locals.var_udr_i_dn4 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn4)), ((locals.var_udr_i_dn5 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn5)), ((locals.var_udr_i_dn6 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn6)), ((locals.var_udr_i_dn7 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn7)), ((locals.var_udr_i_dn8 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn8)), ((locals.var_udr_i_dn9 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn9)), ((locals.var_udr_i_dn10 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn10)), ((locals.var_udr_i_dn11 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11,)
    }
};
        locals.var_udr_i = assign4730_e6297;
        locals.var_udr_i_dn3 = assign4730_e6297_d_n3;
        locals.var_udr_i_dn4 = assign4730_e6297_d_n4;
        locals.var_udr_i_dn5 = assign4730_e6297_d_n5;
        locals.var_udr_i_dn6 = assign4730_e6297_d_n6;
        locals.var_udr_i_dn7 = assign4730_e6297_d_n7;
        locals.var_udr_i_dn8 = assign4730_e6297_d_n8;
        locals.var_udr_i_dn9 = assign4730_e6297_d_n9;
        locals.var_udr_i_dn10 = assign4730_e6297_d_n10;
        locals.var_udr_i_dn11 = assign4730_e6297_d_n11;
        locals.var_udr_i_rv = 0.0;

        let assign4740_e6301: f64 = (locals.var_inv_l).powf(p.p392);
        let assign4740_e6304: f64 = (locals.var_inv_llong).powf(p.p392);
        let assign4740_e6305: f64 = (assign4740_e6301 - assign4740_e6304);
        let assign4740_e6307: f64 = (assign4740_e6305).max(0.0);
        let assign4740_e6308: f64 = (p.p391 * assign4740_e6307);
        locals.var_t0 = assign4740_e6308;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4750_e6312: f64 = (locals.var_inv_w).powf(p.p394);
        let assign4750_e6315: f64 = (locals.var_inv_wwide).powf(p.p394);
        let assign4750_e6316: f64 = (assign4750_e6312 - assign4750_e6315);
        let assign4750_e6318: f64 = (assign4750_e6316).max(0.0);
        let assign4750_e6319: f64 = (p.p393 * assign4750_e6318);
        let assign4750_e6323: f64 = (locals.var_inv_wl).powf(p.p396);
        let assign4750_e6324: f64 = (p.p395 * assign4750_e6323);
        let assign4750_e6325: f64 = (assign4750_e6319 + assign4750_e6324);
        locals.var_t1 = assign4750_e6325;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4760_e6329: f64 = (1.0 + locals.var_t0);
        let assign4760_e6331: f64 = (assign4760_e6329 + locals.var_t1);
        let assign4760_e6332: f64 = (locals.var_uc_i * assign4760_e6331);
        locals.var_uc_i = assign4760_e6332;
        locals.var_uc_i_dn3 = ((locals.var_uc_i_dn3 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_uc_i_dn4 = ((locals.var_uc_i_dn4 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_uc_i_dn5 = ((locals.var_uc_i_dn5 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_uc_i_dn6 = ((locals.var_uc_i_dn6 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_uc_i_dn7 = ((locals.var_uc_i_dn7 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_uc_i_dn8 = ((locals.var_uc_i_dn8 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_uc_i_dn9 = ((locals.var_uc_i_dn9 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_uc_i_dn10 = ((locals.var_uc_i_dn10 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_uc_i_dn11 = ((locals.var_uc_i_dn11 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_uc_i_rv = 0.0;

        let assign4770_e6335: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign4770_e6335;
        locals.var_guard30_rv = 0.0;

        let (assign4780_e6345, assign4780_e6345_d_n3, assign4780_e6345_d_n4, assign4780_e6345_d_n5, assign4780_e6345_d_n6, assign4780_e6345_d_n7, assign4780_e6345_d_n8, assign4780_e6345_d_n9, assign4780_e6345_d_n10, assign4780_e6345_d_n11,) = {
    if (locals.var_guard30 != 0.0) {
        let assign4780_e6340: f64 = (1.0 + locals.var_t0);
        let assign4780_e6342: f64 = (assign4780_e6340 + locals.var_t1);
        let assign4780_e6343: f64 = (locals.var_ucr_i * assign4780_e6342);
        (assign4780_e6343, ((locals.var_ucr_i_dn3 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ucr_i_dn4 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ucr_i_dn5 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ucr_i_dn6 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ucr_i_dn7 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ucr_i_dn8 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ucr_i_dn9 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ucr_i_dn10 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ucr_i_dn11 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))),)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11,)
    }
};
        locals.var_ucr_i = assign4780_e6345;
        locals.var_ucr_i_dn3 = assign4780_e6345_d_n3;
        locals.var_ucr_i_dn4 = assign4780_e6345_d_n4;
        locals.var_ucr_i_dn5 = assign4780_e6345_d_n5;
        locals.var_ucr_i_dn6 = assign4780_e6345_d_n6;
        locals.var_ucr_i_dn7 = assign4780_e6345_d_n7;
        locals.var_ucr_i_dn8 = assign4780_e6345_d_n8;
        locals.var_ucr_i_dn9 = assign4780_e6345_d_n9;
        locals.var_ucr_i_dn10 = assign4780_e6345_d_n10;
        locals.var_ucr_i_dn11 = assign4780_e6345_d_n11;
        locals.var_ucr_i_rv = 0.0;

        let assign4790_e6348: f64 = (locals.var_inv_l).powf(p.p202);
        let assign4790_e6351: f64 = (locals.var_inv_llong).powf(p.p202);
        let assign4790_e6352: f64 = (assign4790_e6348 - assign4790_e6351);
        let assign4790_e6354: f64 = (assign4790_e6352).max(0.0);
        locals.var_t0 = assign4790_e6354;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4800_e6357: f64 = (locals.var_eta0_i * locals.var_t0);
        locals.var_eta0_i = assign4800_e6357;
        locals.var_eta0_i_dn3 = ((locals.var_eta0_i_dn3 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn3));
        locals.var_eta0_i_dn4 = ((locals.var_eta0_i_dn4 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn4));
        locals.var_eta0_i_dn5 = ((locals.var_eta0_i_dn5 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn5));
        locals.var_eta0_i_dn6 = ((locals.var_eta0_i_dn6 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn6));
        locals.var_eta0_i_dn7 = ((locals.var_eta0_i_dn7 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn7));
        locals.var_eta0_i_dn8 = ((locals.var_eta0_i_dn8 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn8));
        locals.var_eta0_i_dn9 = ((locals.var_eta0_i_dn9 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn9));
        locals.var_eta0_i_dn10 = ((locals.var_eta0_i_dn10 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn10));
        locals.var_eta0_i_dn11 = ((locals.var_eta0_i_dn11 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn11));
        locals.var_eta0_i_rv = 0.0;

        let assign4810_e6360: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign4810_e6360;
        locals.var_guard31_rv = 0.0;

        let (assign4820_e6366, assign4820_e6366_d_n3, assign4820_e6366_d_n4, assign4820_e6366_d_n5, assign4820_e6366_d_n6, assign4820_e6366_d_n7, assign4820_e6366_d_n8, assign4820_e6366_d_n9, assign4820_e6366_d_n10, assign4820_e6366_d_n11,) = {
    if (locals.var_guard31 != 0.0) {
        let assign4820_e6364: f64 = (locals.var_eta0r_i * locals.var_t0);
        (assign4820_e6364, ((locals.var_eta0r_i_dn3 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn3)), ((locals.var_eta0r_i_dn4 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn4)), ((locals.var_eta0r_i_dn5 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn5)), ((locals.var_eta0r_i_dn6 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn6)), ((locals.var_eta0r_i_dn7 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn7)), ((locals.var_eta0r_i_dn8 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn8)), ((locals.var_eta0r_i_dn9 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn9)), ((locals.var_eta0r_i_dn10 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn10)), ((locals.var_eta0r_i_dn11 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11,)
    }
};
        locals.var_eta0r_i = assign4820_e6366;
        locals.var_eta0r_i_dn3 = assign4820_e6366_d_n3;
        locals.var_eta0r_i_dn4 = assign4820_e6366_d_n4;
        locals.var_eta0r_i_dn5 = assign4820_e6366_d_n5;
        locals.var_eta0r_i_dn6 = assign4820_e6366_d_n6;
        locals.var_eta0r_i_dn7 = assign4820_e6366_d_n7;
        locals.var_eta0r_i_dn8 = assign4820_e6366_d_n8;
        locals.var_eta0r_i_dn9 = assign4820_e6366_d_n9;
        locals.var_eta0r_i_dn10 = assign4820_e6366_d_n10;
        locals.var_eta0r_i_dn11 = assign4820_e6366_d_n11;
        locals.var_eta0r_i_rv = 0.0;

        let assign4830_e6370: f64 = (locals.var_inv_l).powf(p.p204);
        let assign4830_e6373: f64 = (locals.var_inv_llong).powf(p.p204);
        let assign4830_e6374: f64 = (assign4830_e6370 - assign4830_e6373);
        let assign4830_e6376: f64 = (assign4830_e6374).max(0.0);
        let assign4830_e6377: f64 = (locals.var_etab_i * assign4830_e6376);
        locals.var_etab_i = assign4830_e6377;
        locals.var_etab_i_rv = 0.0;

        let assign4840_e6382: f64 = (locals.var_inv_l).powf(p.p532);
        let assign4840_e6385: f64 = (locals.var_inv_llong).powf(p.p532);
        let assign4840_e6386: f64 = (assign4840_e6382 - assign4840_e6385);
        let assign4840_e6388: f64 = (assign4840_e6386).max(0.0);
        let assign4840_e6389: f64 = (p.p531 * assign4840_e6388);
        let assign4840_e6390: f64 = (1.0 + assign4840_e6389);
        locals.var_t0 = assign4840_e6390;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4850_e6393: f64 = (locals.var_pdiblc_i * locals.var_t0);
        locals.var_pdiblc_i = assign4850_e6393;
        locals.var_pdiblc_i_dn3 = ((locals.var_pdiblc_i_dn3 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn3));
        locals.var_pdiblc_i_dn4 = ((locals.var_pdiblc_i_dn4 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn4));
        locals.var_pdiblc_i_dn5 = ((locals.var_pdiblc_i_dn5 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn5));
        locals.var_pdiblc_i_dn6 = ((locals.var_pdiblc_i_dn6 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn6));
        locals.var_pdiblc_i_dn7 = ((locals.var_pdiblc_i_dn7 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn7));
        locals.var_pdiblc_i_dn8 = ((locals.var_pdiblc_i_dn8 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn8));
        locals.var_pdiblc_i_dn9 = ((locals.var_pdiblc_i_dn9 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn9));
        locals.var_pdiblc_i_dn10 = ((locals.var_pdiblc_i_dn10 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn10));
        locals.var_pdiblc_i_dn11 = ((locals.var_pdiblc_i_dn11 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn11));
        locals.var_pdiblc_i_rv = 0.0;

        let assign4860_e6396: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign4860_e6396;
        locals.var_guard32_rv = 0.0;

        let (assign4870_e6402, assign4870_e6402_d_n3, assign4870_e6402_d_n4, assign4870_e6402_d_n5, assign4870_e6402_d_n6, assign4870_e6402_d_n7, assign4870_e6402_d_n8, assign4870_e6402_d_n9, assign4870_e6402_d_n10, assign4870_e6402_d_n11,) = {
    if (locals.var_guard32 != 0.0) {
        let assign4870_e6400: f64 = (locals.var_pdiblcr_i * locals.var_t0);
        (assign4870_e6400, ((locals.var_pdiblcr_i_dn3 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn3)), ((locals.var_pdiblcr_i_dn4 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn4)), ((locals.var_pdiblcr_i_dn5 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn5)), ((locals.var_pdiblcr_i_dn6 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn6)), ((locals.var_pdiblcr_i_dn7 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn7)), ((locals.var_pdiblcr_i_dn8 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn8)), ((locals.var_pdiblcr_i_dn9 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn9)), ((locals.var_pdiblcr_i_dn10 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn10)), ((locals.var_pdiblcr_i_dn11 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11,)
    }
};
        locals.var_pdiblcr_i = assign4870_e6402;
        locals.var_pdiblcr_i_dn3 = assign4870_e6402_d_n3;
        locals.var_pdiblcr_i_dn4 = assign4870_e6402_d_n4;
        locals.var_pdiblcr_i_dn5 = assign4870_e6402_d_n5;
        locals.var_pdiblcr_i_dn6 = assign4870_e6402_d_n6;
        locals.var_pdiblcr_i_dn7 = assign4870_e6402_d_n7;
        locals.var_pdiblcr_i_dn8 = assign4870_e6402_d_n8;
        locals.var_pdiblcr_i_dn9 = assign4870_e6402_d_n9;
        locals.var_pdiblcr_i_dn10 = assign4870_e6402_d_n10;
        locals.var_pdiblcr_i_dn11 = assign4870_e6402_d_n11;
        locals.var_pdiblcr_i_rv = 0.0;

        let assign4880_e6408: f64 = (locals.var_inv_l).powf(p.p314);
        let assign4880_e6411: f64 = (locals.var_inv_llong).powf(p.p314);
        let assign4880_e6412: f64 = (assign4880_e6408 - assign4880_e6411);
        let assign4880_e6414: f64 = (assign4880_e6412).max(0.0);
        let assign4880_e6415: f64 = (p.p313 * assign4880_e6414);
        let assign4880_e6416: f64 = (1.0 + assign4880_e6415);
        let assign4880_e6417: f64 = (locals.var_delta_i * assign4880_e6416);
        locals.var_t0 = assign4880_e6417;
        locals.var_t0_dn3 = (locals.var_delta_i_dn3 * assign4880_e6416);
        locals.var_t0_dn4 = (locals.var_delta_i_dn4 * assign4880_e6416);
        locals.var_t0_dn5 = (locals.var_delta_i_dn5 * assign4880_e6416);
        locals.var_t0_dn6 = (locals.var_delta_i_dn6 * assign4880_e6416);
        locals.var_t0_dn7 = (locals.var_delta_i_dn7 * assign4880_e6416);
        locals.var_t0_dn8 = (locals.var_delta_i_dn8 * assign4880_e6416);
        locals.var_t0_dn9 = (locals.var_delta_i_dn9 * assign4880_e6416);
        locals.var_t0_dn10 = (locals.var_delta_i_dn10 * assign4880_e6416);
        locals.var_t0_dn11 = (locals.var_delta_i_dn11 * assign4880_e6416);
        locals.var_t0_rv = 0.0;

        let assign4890_e6420: f64 = (locals.var_t0).min(0.5);
        locals.var_delta_i = assign4890_e6420;
        locals.var_delta_i_dn3 = if locals.var_t0 <= 0.5 { locals.var_t0_dn3 } else { 0.0 };
        locals.var_delta_i_dn4 = if locals.var_t0 <= 0.5 { locals.var_t0_dn4 } else { 0.0 };
        locals.var_delta_i_dn5 = if locals.var_t0 <= 0.5 { locals.var_t0_dn5 } else { 0.0 };
        locals.var_delta_i_dn6 = if locals.var_t0 <= 0.5 { locals.var_t0_dn6 } else { 0.0 };
        locals.var_delta_i_dn7 = if locals.var_t0 <= 0.5 { locals.var_t0_dn7 } else { 0.0 };
        locals.var_delta_i_dn8 = if locals.var_t0 <= 0.5 { locals.var_t0_dn8 } else { 0.0 };
        locals.var_delta_i_dn9 = if locals.var_t0 <= 0.5 { locals.var_t0_dn9 } else { 0.0 };
        locals.var_delta_i_dn10 = if locals.var_t0 <= 0.5 { locals.var_t0_dn10 } else { 0.0 };
        locals.var_delta_i_dn11 = if locals.var_t0 <= 0.5 { locals.var_t0_dn11 } else { 0.0 };
        locals.var_delta_i_rv = 0.0;

        let assign4900_e6426: f64 = (locals.var_inv_l).powf(p.p550);
        let assign4900_e6429: f64 = (locals.var_inv_llong).powf(p.p550);
        let assign4900_e6430: f64 = (assign4900_e6426 - assign4900_e6429);
        let assign4900_e6432: f64 = (assign4900_e6430).max(0.0);
        let assign4900_e6433: f64 = (p.p549 * assign4900_e6432);
        let assign4900_e6434: f64 = (1.0 + assign4900_e6433);
        let assign4900_e6435: f64 = (locals.var_fprout_i * assign4900_e6434);
        locals.var_fprout_i = assign4900_e6435;
        locals.var_fprout_i_rv = 0.0;

        let assign4910_e6440: f64 = (locals.var_inv_l).powf(p.p406);
        let assign4910_e6443: f64 = (locals.var_inv_llong).powf(p.p406);
        let assign4910_e6444: f64 = (assign4910_e6440 - assign4910_e6443);
        let assign4910_e6446: f64 = (assign4910_e6444).max(0.0);
        let assign4910_e6447: f64 = (p.p405 * assign4910_e6446);
        let assign4910_e6448: f64 = (1.0 + assign4910_e6447);
        locals.var_t0 = assign4910_e6448;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4920_e6451: f64 = (locals.var_pclm_i * locals.var_t0);
        locals.var_pclm_i = assign4920_e6451;
        locals.var_pclm_i_dn3 = ((locals.var_pclm_i_dn3 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn3));
        locals.var_pclm_i_dn4 = ((locals.var_pclm_i_dn4 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn4));
        locals.var_pclm_i_dn5 = ((locals.var_pclm_i_dn5 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn5));
        locals.var_pclm_i_dn6 = ((locals.var_pclm_i_dn6 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn6));
        locals.var_pclm_i_dn7 = ((locals.var_pclm_i_dn7 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn7));
        locals.var_pclm_i_dn8 = ((locals.var_pclm_i_dn8 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn8));
        locals.var_pclm_i_dn9 = ((locals.var_pclm_i_dn9 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn9));
        locals.var_pclm_i_dn10 = ((locals.var_pclm_i_dn10 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn10));
        locals.var_pclm_i_dn11 = ((locals.var_pclm_i_dn11 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn11));
        locals.var_pclm_i_rv = 0.0;

        let assign4930_e6454: f64 = (locals.var_pclm_i).max(0.0);
        locals.var_pclm_i = assign4930_e6454;
        locals.var_pclm_i_dn3 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn3 } else { 0.0 };
        locals.var_pclm_i_dn4 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn4 } else { 0.0 };
        locals.var_pclm_i_dn5 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn5 } else { 0.0 };
        locals.var_pclm_i_dn6 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn6 } else { 0.0 };
        locals.var_pclm_i_dn7 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn7 } else { 0.0 };
        locals.var_pclm_i_dn8 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn8 } else { 0.0 };
        locals.var_pclm_i_dn9 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn9 } else { 0.0 };
        locals.var_pclm_i_dn10 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn10 } else { 0.0 };
        locals.var_pclm_i_dn11 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn11 } else { 0.0 };
        locals.var_pclm_i_rv = 0.0;

        let assign4940_e6457: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign4940_e6457;
        locals.var_guard33_rv = 0.0;

        let (assign4950_e6463, assign4950_e6463_d_n3, assign4950_e6463_d_n4, assign4950_e6463_d_n5, assign4950_e6463_d_n6, assign4950_e6463_d_n7, assign4950_e6463_d_n8, assign4950_e6463_d_n9, assign4950_e6463_d_n10, assign4950_e6463_d_n11,) = {
    if (locals.var_guard33 != 0.0) {
        let assign4950_e6461: f64 = (locals.var_pclmr_i * locals.var_t0);
        (assign4950_e6461, ((locals.var_pclmr_i_dn3 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn3)), ((locals.var_pclmr_i_dn4 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn4)), ((locals.var_pclmr_i_dn5 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn5)), ((locals.var_pclmr_i_dn6 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn6)), ((locals.var_pclmr_i_dn7 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn7)), ((locals.var_pclmr_i_dn8 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn8)), ((locals.var_pclmr_i_dn9 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn9)), ((locals.var_pclmr_i_dn10 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn10)), ((locals.var_pclmr_i_dn11 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11,)
    }
};
        locals.var_pclmr_i = assign4950_e6463;
        locals.var_pclmr_i_dn3 = assign4950_e6463_d_n3;
        locals.var_pclmr_i_dn4 = assign4950_e6463_d_n4;
        locals.var_pclmr_i_dn5 = assign4950_e6463_d_n5;
        locals.var_pclmr_i_dn6 = assign4950_e6463_d_n6;
        locals.var_pclmr_i_dn7 = assign4950_e6463_d_n7;
        locals.var_pclmr_i_dn8 = assign4950_e6463_d_n8;
        locals.var_pclmr_i_dn9 = assign4950_e6463_d_n9;
        locals.var_pclmr_i_dn10 = assign4950_e6463_d_n10;
        locals.var_pclmr_i_dn11 = assign4950_e6463_d_n11;
        locals.var_pclmr_i_rv = 0.0;

        let (assign4960_e6469, assign4960_e6469_d_n3, assign4960_e6469_d_n4, assign4960_e6469_d_n5, assign4960_e6469_d_n6, assign4960_e6469_d_n7, assign4960_e6469_d_n8, assign4960_e6469_d_n9, assign4960_e6469_d_n10, assign4960_e6469_d_n11,) = {
    if (locals.var_guard33 != 0.0) {
        let assign4960_e6467: f64 = (locals.var_pclmr_i).max(0.0);
        (assign4960_e6467, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn3 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn4 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn5 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn6 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn7 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn8 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn9 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn10 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn11 } else { 0.0 },)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11,)
    }
};
        locals.var_pclmr_i = assign4960_e6469;
        locals.var_pclmr_i_dn3 = assign4960_e6469_d_n3;
        locals.var_pclmr_i_dn4 = assign4960_e6469_d_n4;
        locals.var_pclmr_i_dn5 = assign4960_e6469_d_n5;
        locals.var_pclmr_i_dn6 = assign4960_e6469_d_n6;
        locals.var_pclmr_i_dn7 = assign4960_e6469_d_n7;
        locals.var_pclmr_i_dn8 = assign4960_e6469_d_n8;
        locals.var_pclmr_i_dn9 = assign4960_e6469_d_n9;
        locals.var_pclmr_i_dn10 = assign4960_e6469_d_n10;
        locals.var_pclmr_i_dn11 = assign4960_e6469_d_n11;
        locals.var_pclmr_i_rv = 0.0;

        let assign4970_e6473: f64 = (locals.var_inv_l).powf(p.p300);
        let assign4970_e6476: f64 = (locals.var_inv_llong).powf(p.p300);
        let assign4970_e6477: f64 = (assign4970_e6473 - assign4970_e6476);
        let assign4970_e6479: f64 = (assign4970_e6477).max(0.0);
        let assign4970_e6480: f64 = (p.p299 * assign4970_e6479);
        locals.var_t0 = assign4970_e6480;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4980_e6484: f64 = (locals.var_inv_w).powf(p.p302);
        let assign4980_e6487: f64 = (locals.var_inv_wwide).powf(p.p302);
        let assign4980_e6488: f64 = (assign4980_e6484 - assign4980_e6487);
        let assign4980_e6490: f64 = (assign4980_e6488).max(0.0);
        let assign4980_e6491: f64 = (p.p301 * assign4980_e6490);
        let assign4980_e6495: f64 = (locals.var_inv_wl).powf(p.p304);
        let assign4980_e6496: f64 = (p.p303 * assign4980_e6495);
        let assign4980_e6497: f64 = (assign4980_e6491 + assign4980_e6496);
        locals.var_t1 = assign4980_e6497;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4990_e6501: f64 = (1.0 + locals.var_t0);
        let assign4990_e6503: f64 = (assign4990_e6501 + locals.var_t1);
        let assign4990_e6504: f64 = (locals.var_vsat_i * assign4990_e6503);
        locals.var_vsat_i = assign4990_e6504;
        locals.var_vsat_i_dn3 = ((locals.var_vsat_i_dn3 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsat_i_dn4 = ((locals.var_vsat_i_dn4 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsat_i_dn5 = ((locals.var_vsat_i_dn5 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsat_i_dn6 = ((locals.var_vsat_i_dn6 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsat_i_dn7 = ((locals.var_vsat_i_dn7 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsat_i_dn8 = ((locals.var_vsat_i_dn8 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsat_i_dn9 = ((locals.var_vsat_i_dn9 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsat_i_dn10 = ((locals.var_vsat_i_dn10 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsat_i_dn11 = ((locals.var_vsat_i_dn11 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsat_i_rv = 0.0;

        let assign5000_e6507: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign5000_e6507;
        locals.var_guard34_rv = 0.0;

        let (assign5010_e6517, assign5010_e6517_d_n3, assign5010_e6517_d_n4, assign5010_e6517_d_n5, assign5010_e6517_d_n6, assign5010_e6517_d_n7, assign5010_e6517_d_n8, assign5010_e6517_d_n9, assign5010_e6517_d_n10, assign5010_e6517_d_n11,) = {
    if (locals.var_guard34 != 0.0) {
        let assign5010_e6512: f64 = (1.0 + locals.var_t0);
        let assign5010_e6514: f64 = (assign5010_e6512 + locals.var_t1);
        let assign5010_e6515: f64 = (locals.var_vsatr_i * assign5010_e6514);
        (assign5010_e6515, ((locals.var_vsatr_i_dn3 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsatr_i_dn4 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsatr_i_dn5 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsatr_i_dn6 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsatr_i_dn7 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsatr_i_dn8 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsatr_i_dn9 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsatr_i_dn10 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsatr_i_dn11 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))),)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11,)
    }
};
        locals.var_vsatr_i = assign5010_e6517;
        locals.var_vsatr_i_dn3 = assign5010_e6517_d_n3;
        locals.var_vsatr_i_dn4 = assign5010_e6517_d_n4;
        locals.var_vsatr_i_dn5 = assign5010_e6517_d_n5;
        locals.var_vsatr_i_dn6 = assign5010_e6517_d_n6;
        locals.var_vsatr_i_dn7 = assign5010_e6517_d_n7;
        locals.var_vsatr_i_dn8 = assign5010_e6517_d_n8;
        locals.var_vsatr_i_dn9 = assign5010_e6517_d_n9;
        locals.var_vsatr_i_dn10 = assign5010_e6517_d_n10;
        locals.var_vsatr_i_dn11 = assign5010_e6517_d_n11;
        locals.var_vsatr_i_rv = 0.0;

        let assign5020_e6523: f64 = (locals.var_inv_l).powf(p.p488);
        let assign5020_e6526: f64 = (locals.var_inv_llong).powf(p.p488);
        let assign5020_e6527: f64 = (assign5020_e6523 - assign5020_e6526);
        let assign5020_e6529: f64 = (assign5020_e6527).max(0.0);
        let assign5020_e6530: f64 = (p.p487 * assign5020_e6529);
        let assign5020_e6531: f64 = (1.0 + assign5020_e6530);
        let assign5020_e6532: f64 = (locals.var_psat_i * assign5020_e6531);
        let assign5020_e6534: f64 = (assign5020_e6532).max(0.25);
        locals.var_psat_i = assign5020_e6534;
        locals.var_psat_i_rv = 0.0;

        let assign5030_e6537: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign5030_e6537;
        locals.var_guard35_rv = 0.0;

        let (assign5040_e6557,) = {
    if (locals.var_guard35 != 0.0) {
        let assign5040_e6544: f64 = (locals.var_inv_l).powf(p.p488);
        let assign5040_e6547: f64 = (locals.var_inv_llong).powf(p.p488);
        let assign5040_e6548: f64 = (assign5040_e6544 - assign5040_e6547);
        let assign5040_e6550: f64 = (assign5040_e6548).max(0.0);
        let assign5040_e6551: f64 = (p.p487 * assign5040_e6550);
        let assign5040_e6552: f64 = (1.0 + assign5040_e6551);
        let assign5040_e6553: f64 = (locals.var_psatr_i * assign5040_e6552);
        let assign5040_e6555: f64 = (assign5040_e6553).max(0.25);
        (assign5040_e6555,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign5040_e6557;
        locals.var_psatr_i_rv = 0.0;

        let assign5050_e6562: f64 = (locals.var_inv_l).powf(p.p505);
        let assign5050_e6565: f64 = (locals.var_inv_llong).powf(p.p505);
        let assign5050_e6566: f64 = (assign5050_e6562 - assign5050_e6565);
        let assign5050_e6568: f64 = (assign5050_e6566).max(0.0);
        let assign5050_e6569: f64 = (p.p502 * assign5050_e6568);
        let assign5050_e6570: f64 = (1.0 + assign5050_e6569);
        locals.var_t0 = assign5050_e6570;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5060_e6573: f64 = (locals.var_ptwg_i * locals.var_t0);
        locals.var_ptwg_i = assign5060_e6573;
        locals.var_ptwg_i_dn3 = ((locals.var_ptwg_i_dn3 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn3));
        locals.var_ptwg_i_dn4 = ((locals.var_ptwg_i_dn4 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn4));
        locals.var_ptwg_i_dn5 = ((locals.var_ptwg_i_dn5 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn5));
        locals.var_ptwg_i_dn6 = ((locals.var_ptwg_i_dn6 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn6));
        locals.var_ptwg_i_dn7 = ((locals.var_ptwg_i_dn7 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn7));
        locals.var_ptwg_i_dn8 = ((locals.var_ptwg_i_dn8 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn8));
        locals.var_ptwg_i_dn9 = ((locals.var_ptwg_i_dn9 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn9));
        locals.var_ptwg_i_dn10 = ((locals.var_ptwg_i_dn10 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn10));
        locals.var_ptwg_i_dn11 = ((locals.var_ptwg_i_dn11 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn11));
        locals.var_ptwg_i_rv = 0.0;

        let assign5070_e6576: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign5070_e6576;
        locals.var_guard36_rv = 0.0;

        let (assign5080_e6582, assign5080_e6582_d_n3, assign5080_e6582_d_n4, assign5080_e6582_d_n5, assign5080_e6582_d_n6, assign5080_e6582_d_n7, assign5080_e6582_d_n8, assign5080_e6582_d_n9, assign5080_e6582_d_n10, assign5080_e6582_d_n11,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5080_e6580: f64 = (locals.var_ptwgr_i * locals.var_t0);
        (assign5080_e6580, ((locals.var_ptwgr_i_dn3 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn3)), ((locals.var_ptwgr_i_dn4 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn4)), ((locals.var_ptwgr_i_dn5 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn5)), ((locals.var_ptwgr_i_dn6 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn6)), ((locals.var_ptwgr_i_dn7 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn7)), ((locals.var_ptwgr_i_dn8 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn8)), ((locals.var_ptwgr_i_dn9 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn9)), ((locals.var_ptwgr_i_dn10 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn10)), ((locals.var_ptwgr_i_dn11 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11,)
    }
};
        locals.var_ptwgr_i = assign5080_e6582;
        locals.var_ptwgr_i_dn3 = assign5080_e6582_d_n3;
        locals.var_ptwgr_i_dn4 = assign5080_e6582_d_n4;
        locals.var_ptwgr_i_dn5 = assign5080_e6582_d_n5;
        locals.var_ptwgr_i_dn6 = assign5080_e6582_d_n6;
        locals.var_ptwgr_i_dn7 = assign5080_e6582_d_n7;
        locals.var_ptwgr_i_dn8 = assign5080_e6582_d_n8;
        locals.var_ptwgr_i_dn9 = assign5080_e6582_d_n9;
        locals.var_ptwgr_i_dn10 = assign5080_e6582_d_n10;
        locals.var_ptwgr_i_dn11 = assign5080_e6582_d_n11;
        locals.var_ptwgr_i_rv = 0.0;

        let assign5090_e6588: f64 = (locals.var_inv_l).powf(p.p603);
        let assign5090_e6591: f64 = (locals.var_inv_llong).powf(p.p603);
        let assign5090_e6592: f64 = (assign5090_e6588 - assign5090_e6591);
        let assign5090_e6594: f64 = (assign5090_e6592).max(0.0);
        let assign5090_e6595: f64 = (p.p602 * assign5090_e6594);
        let assign5090_e6596: f64 = (1.0 + assign5090_e6595);
        let assign5090_e6597: f64 = (locals.var_alpha0_i * assign5090_e6596);
        locals.var_alpha0_i = assign5090_e6597;
        locals.var_alpha0_i_rv = 0.0;

        let assign5100_e6602: f64 = (p.p800 * locals.var_inv_l);
        let assign5100_e6603: f64 = (1.0 + assign5100_e6602);
        let assign5100_e6606: f64 = (p.p801 * locals.var_inv_w);
        let assign5100_e6607: f64 = (assign5100_e6603 + assign5100_e6606);
        let assign5100_e6608: f64 = (locals.var_agidl_i * assign5100_e6607);
        locals.var_agidl_i = assign5100_e6608;
        locals.var_agidl_i_rv = 0.0;

        let assign5110_e6613: f64 = (p.p822 * locals.var_inv_l);
        let assign5110_e6614: f64 = (1.0 + assign5110_e6613);
        let assign5110_e6617: f64 = (p.p823 * locals.var_inv_w);
        let assign5110_e6618: f64 = (assign5110_e6614 + assign5110_e6617);
        let assign5110_e6619: f64 = (locals.var_agisl_i * assign5110_e6618);
        locals.var_agisl_i = assign5110_e6619;
        locals.var_agisl_i_rv = 0.0;

        let assign5120_e6624: f64 = (p.p724 * locals.var_inv_l);
        let assign5120_e6625: f64 = (1.0 + assign5120_e6624);
        let assign5120_e6628: f64 = (p.p725 * locals.var_inv_w);
        let assign5120_e6629: f64 = (assign5120_e6625 + assign5120_e6628);
        let assign5120_e6630: f64 = (locals.var_aigc_i * assign5120_e6629);
        locals.var_aigc_i = assign5120_e6630;
        locals.var_aigc_i_dn4 = (locals.var_aigc_i_dn4 * assign5120_e6629);
        locals.var_aigc_i_dn5 = (locals.var_aigc_i_dn5 * assign5120_e6629);
        locals.var_aigc_i_rv = 0.0;

        let assign5130_e6635: f64 = (p.p727 * locals.var_inv_l);
        let assign5130_e6636: f64 = (1.0 + assign5130_e6635);
        let assign5130_e6639: f64 = (p.p728 * locals.var_inv_w);
        let assign5130_e6640: f64 = (assign5130_e6636 + assign5130_e6639);
        let assign5130_e6641: f64 = (locals.var_aigs_i * assign5130_e6640);
        locals.var_aigs_i = assign5130_e6641;
        locals.var_aigs_i_dn4 = (locals.var_aigs_i_dn4 * assign5130_e6640);
        locals.var_aigs_i_dn5 = (locals.var_aigs_i_dn5 * assign5130_e6640);
        locals.var_aigs_i_rv = 0.0;

        let assign5140_e6646: f64 = (p.p729 * locals.var_inv_l);
        let assign5140_e6647: f64 = (1.0 + assign5140_e6646);
        let assign5140_e6650: f64 = (p.p730 * locals.var_inv_w);
        let assign5140_e6651: f64 = (assign5140_e6647 + assign5140_e6650);
        let assign5140_e6652: f64 = (locals.var_aigd_i * assign5140_e6651);
        locals.var_aigd_i = assign5140_e6652;
        locals.var_aigd_i_dn4 = (locals.var_aigd_i_dn4 * assign5140_e6651);
        locals.var_aigd_i_dn5 = (locals.var_aigd_i_dn5 * assign5140_e6651);
        locals.var_aigd_i_rv = 0.0;

        let assign5150_e6657: f64 = (p.p731 * locals.var_inv_l);
        let assign5150_e6658: f64 = (1.0 + assign5150_e6657);
        let assign5150_e6659: f64 = (p.p723 * assign5150_e6658);
        locals.var_pigcd_i = assign5150_e6659;
        locals.var_pigcd_i_rv = 0.0;

        let assign5160_e6663: f64 = (locals.var_inv_lact).powf(p.p93);
        let assign5160_e6666: f64 = (locals.var_inv_llong).powf(p.p93);
        let assign5160_e6667: f64 = (assign5160_e6663 - assign5160_e6666);
        let assign5160_e6669: f64 = (assign5160_e6667).max(0.0);
        let assign5160_e6670: f64 = (p.p92 * assign5160_e6669);
        let assign5160_e6674: f64 = (locals.var_inv_lact).powf(p.p95);
        let assign5160_e6677: f64 = (locals.var_inv_llong).powf(p.p95);
        let assign5160_e6678: f64 = (assign5160_e6674 - assign5160_e6677);
        let assign5160_e6680: f64 = (assign5160_e6678).max(0.0);
        let assign5160_e6681: f64 = (p.p94 * assign5160_e6680);
        let assign5160_e6682: f64 = (assign5160_e6670 + assign5160_e6681);
        locals.var_t0 = assign5160_e6682;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5170_e6686: f64 = (locals.var_inv_wact).powf(p.p97);
        let assign5170_e6689: f64 = (locals.var_inv_wwide).powf(p.p97);
        let assign5170_e6690: f64 = (assign5170_e6686 - assign5170_e6689);
        let assign5170_e6692: f64 = (assign5170_e6690).max(0.0);
        let assign5170_e6693: f64 = (p.p96 * assign5170_e6692);
        let assign5170_e6697: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign5170_e6699: f64 = (assign5170_e6697).powf(p.p99);
        let assign5170_e6700: f64 = (p.p98 * assign5170_e6699);
        let assign5170_e6701: f64 = (assign5170_e6693 + assign5170_e6700);
        locals.var_t1 = assign5170_e6701;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5180_e6705: f64 = (1.0 + locals.var_t0);
        let assign5180_e6707: f64 = (assign5180_e6705 + locals.var_t1);
        let assign5180_e6708: f64 = (locals.var_ndepcv_i * assign5180_e6707);
        locals.var_ndepcv_i = assign5180_e6708;
        locals.var_ndepcv_i_dn3 = ((locals.var_ndepcv_i_dn3 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndepcv_i_dn4 = ((locals.var_ndepcv_i_dn4 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndepcv_i_dn5 = ((locals.var_ndepcv_i_dn5 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndepcv_i_dn6 = ((locals.var_ndepcv_i_dn6 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndepcv_i_dn7 = ((locals.var_ndepcv_i_dn7 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndepcv_i_dn8 = ((locals.var_ndepcv_i_dn8 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndepcv_i_dn9 = ((locals.var_ndepcv_i_dn9 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndepcv_i_dn10 = ((locals.var_ndepcv_i_dn10 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndepcv_i_dn11 = ((locals.var_ndepcv_i_dn11 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndepcv_i_rv = 0.0;

        let assign5190_e6711: f64 = if p.p29 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign5190_e6711;
        locals.var_guard37_rv = 0.0;

        let (assign5200_e6715, assign5200_e6715_d_n3, assign5200_e6715_d_n4, assign5200_e6715_d_n5, assign5200_e6715_d_n6, assign5200_e6715_d_n7, assign5200_e6715_d_n8, assign5200_e6715_d_n9, assign5200_e6715_d_n10, assign5200_e6715_d_n11,) = {
    if (locals.var_guard37 != 0.0) {
        (locals.var_ndep_i, locals.var_ndep_i_dn3, locals.var_ndep_i_dn4, locals.var_ndep_i_dn5, locals.var_ndep_i_dn6, locals.var_ndep_i_dn7, locals.var_ndep_i_dn8, locals.var_ndep_i_dn9, locals.var_ndep_i_dn10, locals.var_ndep_i_dn11,)
    } else {
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11,)
    }
};
        locals.var_ndepcv_i = assign5200_e6715;
        locals.var_ndepcv_i_dn3 = assign5200_e6715_d_n3;
        locals.var_ndepcv_i_dn4 = assign5200_e6715_d_n4;
        locals.var_ndepcv_i_dn5 = assign5200_e6715_d_n5;
        locals.var_ndepcv_i_dn6 = assign5200_e6715_d_n6;
        locals.var_ndepcv_i_dn7 = assign5200_e6715_d_n7;
        locals.var_ndepcv_i_dn8 = assign5200_e6715_d_n8;
        locals.var_ndepcv_i_dn9 = assign5200_e6715_d_n9;
        locals.var_ndepcv_i_dn10 = assign5200_e6715_d_n10;
        locals.var_ndepcv_i_dn11 = assign5200_e6715_d_n11;
        locals.var_ndepcv_i_rv = 0.0;

        let (assign5210_e6720, assign5210_e6720_d_n3, assign5210_e6720_d_n4, assign5210_e6720_d_n5, assign5210_e6720_d_n6, assign5210_e6720_d_n7, assign5210_e6720_d_n8, assign5210_e6720_d_n9, assign5210_e6720_d_n10, assign5210_e6720_d_n11,) = {
    if (locals.var_guard37 == 0.0) {
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11,)
    } else {
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11,)
    }
};
        locals.var_ndepcv_i = assign5210_e6720;
        locals.var_ndepcv_i_dn3 = assign5210_e6720_d_n3;
        locals.var_ndepcv_i_dn4 = assign5210_e6720_d_n4;
        locals.var_ndepcv_i_dn5 = assign5210_e6720_d_n5;
        locals.var_ndepcv_i_dn6 = assign5210_e6720_d_n6;
        locals.var_ndepcv_i_dn7 = assign5210_e6720_d_n7;
        locals.var_ndepcv_i_dn8 = assign5210_e6720_d_n8;
        locals.var_ndepcv_i_dn9 = assign5210_e6720_d_n9;
        locals.var_ndepcv_i_dn10 = assign5210_e6720_d_n10;
        locals.var_ndepcv_i_dn11 = assign5210_e6720_d_n11;
        locals.var_ndepcv_i_rv = 0.0;

        let assign5220_e6724: f64 = (locals.var_inv_l).powf(p.p124);
        let assign5220_e6727: f64 = (locals.var_inv_llong).powf(p.p124);
        let assign5220_e6728: f64 = (assign5220_e6724 - assign5220_e6727);
        let assign5220_e6730: f64 = (assign5220_e6728).max(0.0);
        let assign5220_e6731: f64 = (p.p123 * assign5220_e6730);
        locals.var_t0 = assign5220_e6731;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5230_e6735: f64 = (locals.var_inv_w).powf(p.p126);
        let assign5230_e6738: f64 = (locals.var_inv_wwide).powf(p.p126);
        let assign5230_e6739: f64 = (assign5230_e6735 - assign5230_e6738);
        let assign5230_e6741: f64 = (assign5230_e6739).max(0.0);
        let assign5230_e6742: f64 = (p.p125 * assign5230_e6741);
        let assign5230_e6746: f64 = (locals.var_inv_wl).powf(p.p128);
        let assign5230_e6747: f64 = (p.p127 * assign5230_e6746);
        let assign5230_e6748: f64 = (assign5230_e6742 + assign5230_e6747);
        locals.var_t1 = assign5230_e6748;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5240_e6752: f64 = (1.0 + locals.var_t0);
        let assign5240_e6754: f64 = (assign5240_e6752 + locals.var_t1);
        let assign5240_e6755: f64 = (locals.var_vfb_i * assign5240_e6754);
        locals.var_vfb_i = assign5240_e6755;
        locals.var_vfb_i_dn3 = ((locals.var_vfb_i_dn3 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfb_i_dn4 = ((locals.var_vfb_i_dn4 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfb_i_dn5 = ((locals.var_vfb_i_dn5 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfb_i_dn6 = ((locals.var_vfb_i_dn6 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfb_i_dn7 = ((locals.var_vfb_i_dn7 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfb_i_dn8 = ((locals.var_vfb_i_dn8 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfb_i_dn9 = ((locals.var_vfb_i_dn9 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfb_i_dn10 = ((locals.var_vfb_i_dn10 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfb_i_dn11 = ((locals.var_vfb_i_dn11 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfb_i_rv = 0.0;

        let assign5250_e6759: f64 = (locals.var_inv_lact).powf(p.p134);
        let assign5250_e6762: f64 = (locals.var_inv_llong).powf(p.p134);
        let assign5250_e6763: f64 = (assign5250_e6759 - assign5250_e6762);
        let assign5250_e6765: f64 = (assign5250_e6763).max(0.0);
        let assign5250_e6766: f64 = (p.p133 * assign5250_e6765);
        locals.var_t0 = assign5250_e6766;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5260_e6770: f64 = (locals.var_inv_wact).powf(p.p136);
        let assign5260_e6773: f64 = (locals.var_inv_wwide).powf(p.p136);
        let assign5260_e6774: f64 = (assign5260_e6770 - assign5260_e6773);
        let assign5260_e6776: f64 = (assign5260_e6774).max(0.0);
        let assign5260_e6777: f64 = (p.p135 * assign5260_e6776);
        let assign5260_e6781: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign5260_e6783: f64 = (assign5260_e6781).powf(p.p138);
        let assign5260_e6784: f64 = (p.p137 * assign5260_e6783);
        let assign5260_e6785: f64 = (assign5260_e6777 + assign5260_e6784);
        locals.var_t1 = assign5260_e6785;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5270_e6789: f64 = (1.0 + locals.var_t0);
        let assign5270_e6791: f64 = (assign5270_e6789 + locals.var_t1);
        let assign5270_e6792: f64 = (locals.var_vfbcv_i * assign5270_e6791);
        locals.var_vfbcv_i = assign5270_e6792;
        locals.var_vfbcv_i_dn3 = ((locals.var_vfbcv_i_dn3 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfbcv_i_dn4 = ((locals.var_vfbcv_i_dn4 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfbcv_i_dn5 = ((locals.var_vfbcv_i_dn5 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfbcv_i_dn6 = ((locals.var_vfbcv_i_dn6 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfbcv_i_dn7 = ((locals.var_vfbcv_i_dn7 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfbcv_i_dn8 = ((locals.var_vfbcv_i_dn8 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfbcv_i_dn9 = ((locals.var_vfbcv_i_dn9 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfbcv_i_dn10 = ((locals.var_vfbcv_i_dn10 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfbcv_i_dn11 = ((locals.var_vfbcv_i_dn11 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfbcv_i_rv = 0.0;

        let assign5280_e6796: f64 = (locals.var_inv_lact).powf(p.p320);
        let assign5280_e6799: f64 = (locals.var_inv_llong).powf(p.p320);
        let assign5280_e6800: f64 = (assign5280_e6796 - assign5280_e6799);
        let assign5280_e6802: f64 = (assign5280_e6800).max(0.0);
        let assign5280_e6803: f64 = (p.p319 * assign5280_e6802);
        locals.var_t0 = assign5280_e6803;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5290_e6807: f64 = (locals.var_inv_wact).powf(p.p322);
        let assign5290_e6810: f64 = (locals.var_inv_wwide).powf(p.p322);
        let assign5290_e6811: f64 = (assign5290_e6807 - assign5290_e6810);
        let assign5290_e6813: f64 = (assign5290_e6811).max(0.0);
        let assign5290_e6814: f64 = (p.p321 * assign5290_e6813);
        let assign5290_e6818: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign5290_e6820: f64 = (assign5290_e6818).powf(p.p324);
        let assign5290_e6821: f64 = (p.p323 * assign5290_e6820);
        let assign5290_e6822: f64 = (assign5290_e6814 + assign5290_e6821);
        locals.var_t1 = assign5290_e6822;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5300_e6826: f64 = (1.0 + locals.var_t0);
        let assign5300_e6828: f64 = (assign5300_e6826 + locals.var_t1);
        let assign5300_e6829: f64 = (locals.var_vsatcv_i * assign5300_e6828);
        locals.var_vsatcv_i = assign5300_e6829;
        locals.var_vsatcv_i_dn3 = ((locals.var_vsatcv_i_dn3 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsatcv_i_dn4 = ((locals.var_vsatcv_i_dn4 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsatcv_i_dn5 = ((locals.var_vsatcv_i_dn5 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsatcv_i_dn6 = ((locals.var_vsatcv_i_dn6 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsatcv_i_dn7 = ((locals.var_vsatcv_i_dn7 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsatcv_i_dn8 = ((locals.var_vsatcv_i_dn8 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsatcv_i_dn9 = ((locals.var_vsatcv_i_dn9 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsatcv_i_dn10 = ((locals.var_vsatcv_i_dn10 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsatcv_i_dn11 = ((locals.var_vsatcv_i_dn11 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsatcv_i_rv = 0.0;

        let assign5310_e6835: f64 = (locals.var_inv_lact).powf(p.p417);
        let assign5310_e6838: f64 = (locals.var_inv_llong).powf(p.p417);
        let assign5310_e6839: f64 = (assign5310_e6835 - assign5310_e6838);
        let assign5310_e6841: f64 = (assign5310_e6839).max(0.0);
        let assign5310_e6842: f64 = (p.p416 * assign5310_e6841);
        let assign5310_e6843: f64 = (1.0 + assign5310_e6842);
        let assign5310_e6844: f64 = (locals.var_pclmcv_i * assign5310_e6843);
        locals.var_pclmcv_i = assign5310_e6844;
        locals.var_pclmcv_i_rv = 0.0;

        let assign5320_e6847: f64 = (locals.var_pclmcv_i).max(0.0);
        locals.var_pclmcv_i = assign5320_e6847;
        locals.var_pclmcv_i_rv = 0.0;

        let assign5330_e6851: f64 = (locals.var_inv_l).powf(p.p210);
        let assign5330_e6854: f64 = (locals.var_inv_llong).powf(p.p210);
        let assign5330_e6855: f64 = (assign5330_e6851 - assign5330_e6854);
        let assign5330_e6857: f64 = (assign5330_e6855).max(0.0);
        let assign5330_e6858: f64 = (p.p209 * assign5330_e6857);
        locals.var_t0 = assign5330_e6858;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5340_e6862: f64 = (locals.var_inv_w).powf(p.p212);
        let assign5340_e6865: f64 = (locals.var_inv_wwide).powf(p.p212);
        let assign5340_e6866: f64 = (assign5340_e6862 - assign5340_e6865);
        let assign5340_e6868: f64 = (assign5340_e6866).max(0.0);
        let assign5340_e6869: f64 = (p.p211 * assign5340_e6868);
        let assign5340_e6873: f64 = (locals.var_inv_wl).powf(p.p214);
        let assign5340_e6874: f64 = (p.p213 * assign5340_e6873);
        let assign5340_e6875: f64 = (assign5340_e6869 + assign5340_e6874);
        locals.var_t1 = assign5340_e6875;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5350_e6879: f64 = (1.0 + locals.var_t0);
        let assign5350_e6881: f64 = (assign5350_e6879 + locals.var_t1);
        let assign5350_e6882: f64 = (locals.var_k1_i * assign5350_e6881);
        locals.var_k1_i = assign5350_e6882;
        locals.var_k1_i_dn3 = ((locals.var_k1_i_dn3 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k1_i_dn4 = ((locals.var_k1_i_dn4 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k1_i_dn5 = ((locals.var_k1_i_dn5 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k1_i_dn6 = ((locals.var_k1_i_dn6 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k1_i_dn7 = ((locals.var_k1_i_dn7 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k1_i_dn8 = ((locals.var_k1_i_dn8 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k1_i_dn9 = ((locals.var_k1_i_dn9 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k1_i_dn10 = ((locals.var_k1_i_dn10 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k1_i_dn11 = ((locals.var_k1_i_dn11 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k1_i_rv = 0.0;

        let assign5360_e6886: f64 = (locals.var_inv_l).powf(p.p1198);
        let assign5360_e6889: f64 = (locals.var_inv_llong).powf(p.p1198);
        let assign5360_e6890: f64 = (assign5360_e6886 - assign5360_e6889);
        let assign5360_e6892: f64 = (assign5360_e6890).max(0.0);
        let assign5360_e6893: f64 = (p.p1197 * assign5360_e6892);
        locals.var_t0 = assign5360_e6893;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5370_e6897: f64 = (locals.var_inv_w).powf(p.p1200);
        let assign5370_e6900: f64 = (locals.var_inv_wwide).powf(p.p1200);
        let assign5370_e6901: f64 = (assign5370_e6897 - assign5370_e6900);
        let assign5370_e6903: f64 = (assign5370_e6901).max(0.0);
        let assign5370_e6904: f64 = (p.p1199 * assign5370_e6903);
        let assign5370_e6908: f64 = (locals.var_inv_wl).powf(p.p1202);
        let assign5370_e6909: f64 = (p.p1201 * assign5370_e6908);
        let assign5370_e6910: f64 = (assign5370_e6904 + assign5370_e6909);
        locals.var_t1 = assign5370_e6910;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5380_e6914: f64 = (1.0 + locals.var_t0);
        let assign5380_e6916: f64 = (assign5380_e6914 + locals.var_t1);
        let assign5380_e6917: f64 = (locals.var_k1edge_i * assign5380_e6916);
        locals.var_k1edge_i = assign5380_e6917;
        locals.var_k1edge_i_dn3 = ((locals.var_k1edge_i_dn3 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k1edge_i_dn4 = ((locals.var_k1edge_i_dn4 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k1edge_i_dn5 = ((locals.var_k1edge_i_dn5 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k1edge_i_dn6 = ((locals.var_k1edge_i_dn6 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k1edge_i_dn7 = ((locals.var_k1edge_i_dn7 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k1edge_i_dn8 = ((locals.var_k1edge_i_dn8 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k1edge_i_dn9 = ((locals.var_k1edge_i_dn9 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k1edge_i_dn10 = ((locals.var_k1edge_i_dn10 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k1edge_i_dn11 = ((locals.var_k1edge_i_dn11 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k1edge_i_rv = 0.0;

        let assign5390_e6921: f64 = (locals.var_inv_l).powf(p.p220);
        let assign5390_e6924: f64 = (locals.var_inv_llong).powf(p.p220);
        let assign5390_e6925: f64 = (assign5390_e6921 - assign5390_e6924);
        let assign5390_e6927: f64 = (assign5390_e6925).max(0.0);
        let assign5390_e6928: f64 = (p.p219 * assign5390_e6927);
        locals.var_t0 = assign5390_e6928;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5400_e6932: f64 = (locals.var_inv_w).powf(p.p222);
        let assign5400_e6935: f64 = (locals.var_inv_wwide).powf(p.p222);
        let assign5400_e6936: f64 = (assign5400_e6932 - assign5400_e6935);
        let assign5400_e6938: f64 = (assign5400_e6936).max(0.0);
        let assign5400_e6939: f64 = (p.p221 * assign5400_e6938);
        let assign5400_e6943: f64 = (locals.var_inv_wl).powf(p.p224);
        let assign5400_e6944: f64 = (p.p223 * assign5400_e6943);
        let assign5400_e6945: f64 = (assign5400_e6939 + assign5400_e6944);
        locals.var_t1 = assign5400_e6945;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5410_e6949: f64 = (1.0 + locals.var_t0);
        let assign5410_e6951: f64 = (assign5410_e6949 + locals.var_t1);
        let assign5410_e6952: f64 = (locals.var_k2_i * assign5410_e6951);
        locals.var_k2_i = assign5410_e6952;
        locals.var_k2_i_dn3 = ((locals.var_k2_i_dn3 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k2_i_dn4 = ((locals.var_k2_i_dn4 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k2_i_dn5 = ((locals.var_k2_i_dn5 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k2_i_dn6 = ((locals.var_k2_i_dn6 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k2_i_dn7 = ((locals.var_k2_i_dn7 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k2_i_dn8 = ((locals.var_k2_i_dn8 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k2_i_dn9 = ((locals.var_k2_i_dn9 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k2_i_dn10 = ((locals.var_k2_i_dn10 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k2_i_dn11 = ((locals.var_k2_i_dn11 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k2_i_rv = 0.0;

        let assign5420_e6956: f64 = (locals.var_inv_l).powf(p.p1267);
        let assign5420_e6959: f64 = (locals.var_inv_llong).powf(p.p1267);
        let assign5420_e6960: f64 = (assign5420_e6956 - assign5420_e6959);
        let assign5420_e6962: f64 = (assign5420_e6960).max(0.0);
        let assign5420_e6963: f64 = (p.p1266 * assign5420_e6962);
        locals.var_t0 = assign5420_e6963;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5430_e6967: f64 = (locals.var_inv_w).powf(p.p1269);
        let assign5430_e6970: f64 = (locals.var_inv_wwide).powf(p.p1269);
        let assign5430_e6971: f64 = (assign5430_e6967 - assign5430_e6970);
        let assign5430_e6973: f64 = (assign5430_e6971).max(0.0);
        let assign5430_e6974: f64 = (p.p1268 * assign5430_e6973);
        let assign5430_e6978: f64 = (locals.var_inv_wl).powf(p.p1271);
        let assign5430_e6979: f64 = (p.p1270 * assign5430_e6978);
        let assign5430_e6980: f64 = (assign5430_e6974 + assign5430_e6979);
        locals.var_t1 = assign5430_e6980;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5440_e6984: f64 = (1.0 + locals.var_t0);
        let assign5440_e6986: f64 = (assign5440_e6984 + locals.var_t1);
        let assign5440_e6987: f64 = (locals.var_k2edge_i * assign5440_e6986);
        locals.var_k2edge_i = assign5440_e6987;
        locals.var_k2edge_i_dn3 = ((locals.var_k2edge_i_dn3 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k2edge_i_dn4 = ((locals.var_k2edge_i_dn4 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k2edge_i_dn5 = ((locals.var_k2edge_i_dn5 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k2edge_i_dn6 = ((locals.var_k2edge_i_dn6 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k2edge_i_dn7 = ((locals.var_k2edge_i_dn7 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k2edge_i_dn8 = ((locals.var_k2edge_i_dn8 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k2edge_i_dn9 = ((locals.var_k2edge_i_dn9 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k2edge_i_dn10 = ((locals.var_k2edge_i_dn10 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k2edge_i_dn11 = ((locals.var_k2edge_i_dn11 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k2edge_i_rv = 0.0;

        let assign5450_e6993: f64 = (locals.var_inv_l).powf(p.p448);
        let assign5450_e6996: f64 = (locals.var_inv_llong).powf(p.p448);
        let assign5450_e6997: f64 = (assign5450_e6993 - assign5450_e6996);
        let assign5450_e6999: f64 = (assign5450_e6997).max(0.0);
        let assign5450_e7000: f64 = (p.p447 * assign5450_e6999);
        let assign5450_e7001: f64 = (1.0 + assign5450_e7000);
        let assign5450_e7002: f64 = (locals.var_prwb_i * assign5450_e7001);
        locals.var_prwb_i = assign5450_e7002;
        locals.var_prwb_i_rv = 0.0;

        let assign5460_e7007: f64 = (locals.var_inv_l * p.p1036);
        let assign5460_e7008: f64 = (1.0 + assign5460_e7007);
        let assign5460_e7009: f64 = (locals.var_ute_i * assign5460_e7008);
        locals.var_ute_i = assign5460_e7009;
        locals.var_ute_i_rv = 0.0;

        let assign5470_e7014: f64 = (locals.var_inv_l * p.p1041);
        let assign5470_e7015: f64 = (1.0 + assign5470_e7014);
        let assign5470_e7016: f64 = (locals.var_ua1_i * assign5470_e7015);
        locals.var_ua1_i = assign5470_e7016;
        locals.var_ua1_i_rv = 0.0;

        let assign5480_e7021: f64 = (locals.var_inv_l * p.p1050);
        let assign5480_e7022: f64 = (1.0 + assign5480_e7021);
        let assign5480_e7023: f64 = (locals.var_ud1_i * assign5480_e7022);
        locals.var_ud1_i = assign5480_e7023;
        locals.var_ud1_i_rv = 0.0;

        let assign5490_e7028: f64 = (locals.var_inv_l * p.p1068);
        let assign5490_e7029: f64 = (1.0 + assign5490_e7028);
        let assign5490_e7030: f64 = (locals.var_at_i * assign5490_e7029);
        locals.var_at_i = assign5490_e7030;
        locals.var_at_i_rv = 0.0;

        let assign5500_e7035: f64 = (locals.var_inv_l * p.p1074);
        let assign5500_e7036: f64 = (1.0 + assign5500_e7035);
        let assign5500_e7037: f64 = (locals.var_ptwgt_i * assign5500_e7036);
        locals.var_ptwgt_i = assign5500_e7037;
        locals.var_ptwgt_i_rv = 0.0;

        let assign5510_e7040: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign5510_e7040;
        locals.var_guard38_rv = 0.0;

        let (assign5520_e7058,) = {
    if (locals.var_guard38 != 0.0) {
        let assign5520_e7047: f64 = (locals.var_inv_l).powf(p.p462);
        let assign5520_e7050: f64 = (locals.var_inv_llong).powf(p.p462);
        let assign5520_e7051: f64 = (assign5520_e7047 - assign5520_e7050);
        let assign5520_e7053: f64 = (assign5520_e7051).max(0.0);
        let assign5520_e7054: f64 = (p.p461 * assign5520_e7053);
        let assign5520_e7055: f64 = (1.0 + assign5520_e7054);
        let assign5520_e7056: f64 = (locals.var_rsw_i * assign5520_e7055);
        (assign5520_e7056,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign5520_e7058;
        locals.var_rsw_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5530_e7076,) = {
    if (locals.var_guard38 != 0.0) {
        let assign5530_e7065: f64 = (locals.var_inv_l).powf(p.p472);
        let assign5530_e7068: f64 = (locals.var_inv_llong).powf(p.p472);
        let assign5530_e7069: f64 = (assign5530_e7065 - assign5530_e7068);
        let assign5530_e7071: f64 = (assign5530_e7069).max(0.0);
        let assign5530_e7072: f64 = (p.p471 * assign5530_e7071);
        let assign5530_e7073: f64 = (1.0 + assign5530_e7072);
        let assign5530_e7074: f64 = (locals.var_rdw_i * assign5530_e7073);
        (assign5530_e7074,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign5530_e7076;
        locals.var_rdw_i_rv = 0.0;

        let (assign5540_e7095,) = {
    if (locals.var_guard38 == 0.0) {
        let assign5540_e7084: f64 = (locals.var_inv_l).powf(p.p479);
        let assign5540_e7087: f64 = (locals.var_inv_llong).powf(p.p479);
        let assign5540_e7088: f64 = (assign5540_e7084 - assign5540_e7087);
        let assign5540_e7090: f64 = (assign5540_e7088).max(0.0);
        let assign5540_e7091: f64 = (p.p478 * assign5540_e7090);
        let assign5540_e7092: f64 = (1.0 + assign5540_e7091);
        let assign5540_e7093: f64 = (locals.var_rdsw_i * assign5540_e7092);
        (assign5540_e7093,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign5540_e7095;
        locals.var_rdsw_i_rv = 0.0;

        let assign5550_e7098: f64 = if locals.var_ucs_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign5550_e7098;
        locals.var_guard39_rv = 0.0;

        let (assign5560_e7102,) = {
    if (locals.var_guard39 != 0.0) {
        (1.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign5560_e7102;
        locals.var_ucs_i_rv = 0.0;

        let assign5570_e7105: f64 = if locals.var_ucs_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign5570_e7105;
        locals.var_guard40_rv = 0.0;

        let (assign5580_e7112,) = {
    if ((locals.var_guard39 == 0.0) && (locals.var_guard40 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign5580_e7112;
        locals.var_ucs_i_rv = 0.0;

        let assign5590_e7115: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign5590_e7115;
        locals.var_guard41_rv = 0.0;

        let assign5600_e7118: f64 = if locals.var_ucsr_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign5600_e7118;
        locals.var_guard42_rv = 0.0;

        let (assign5610_e7124,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign5610_e7124;
        locals.var_ucsr_i_rv = 0.0;

        let assign5620_e7127: f64 = if locals.var_ucsr_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign5620_e7127;
        locals.var_guard43_rv = 0.0;

        let (assign5630_e7136,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign5630_e7136;
        locals.var_ucsr_i_rv = 0.0;

        let assign5900_e7219: f64 = if locals.var_m0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign5900_e7219;
        locals.var_guard68_rv = 0.0;

        let (assign5910_e7223,) = {
    if (locals.var_guard68 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0_i,)
    }
};
        locals.var_m0_i = assign5910_e7223;
        locals.var_m0_i_rv = 0.0;

        let assign5920_e7226: f64 = if locals.var_u0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign5920_e7226;
        locals.var_guard69_rv = 0.0;

        let (assign5930_e7230,) = {
    if (locals.var_guard69 != 0.0) {
        (0.067,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign5930_e7230;
        locals.var_u0_i_rv = 0.0;

        let assign5940_e7233: f64 = if locals.var_ua_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign5940_e7233;
        locals.var_guard70_rv = 0.0;

        let (assign5950_e7237, assign5950_e7237_d_n3, assign5950_e7237_d_n4, assign5950_e7237_d_n5, assign5950_e7237_d_n6, assign5950_e7237_d_n7, assign5950_e7237_d_n8, assign5950_e7237_d_n9, assign5950_e7237_d_n10, assign5950_e7237_d_n11,) = {
    if (locals.var_guard70 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ua_i, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11,)
    }
};
        locals.var_ua_i = assign5950_e7237;
        locals.var_ua_i_dn3 = assign5950_e7237_d_n3;
        locals.var_ua_i_dn4 = assign5950_e7237_d_n4;
        locals.var_ua_i_dn5 = assign5950_e7237_d_n5;
        locals.var_ua_i_dn6 = assign5950_e7237_d_n6;
        locals.var_ua_i_dn7 = assign5950_e7237_d_n7;
        locals.var_ua_i_dn8 = assign5950_e7237_d_n8;
        locals.var_ua_i_dn9 = assign5950_e7237_d_n9;
        locals.var_ua_i_dn10 = assign5950_e7237_d_n10;
        locals.var_ua_i_dn11 = assign5950_e7237_d_n11;
        locals.var_ua_i_rv = 0.0;

        let assign5960_e7240: f64 = if locals.var_eu_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign5960_e7240;
        locals.var_guard71_rv = 0.0;

        let (assign5970_e7244, assign5970_e7244_d_n3, assign5970_e7244_d_n4, assign5970_e7244_d_n5, assign5970_e7244_d_n6, assign5970_e7244_d_n7, assign5970_e7244_d_n8, assign5970_e7244_d_n9, assign5970_e7244_d_n10, assign5970_e7244_d_n11,) = {
    if (locals.var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eu_i, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11,)
    }
};
        locals.var_eu_i = assign5970_e7244;
        locals.var_eu_i_dn3 = assign5970_e7244_d_n3;
        locals.var_eu_i_dn4 = assign5970_e7244_d_n4;
        locals.var_eu_i_dn5 = assign5970_e7244_d_n5;
        locals.var_eu_i_dn6 = assign5970_e7244_d_n6;
        locals.var_eu_i_dn7 = assign5970_e7244_d_n7;
        locals.var_eu_i_dn8 = assign5970_e7244_d_n8;
        locals.var_eu_i_dn9 = assign5970_e7244_d_n9;
        locals.var_eu_i_dn10 = assign5970_e7244_d_n10;
        locals.var_eu_i_dn11 = assign5970_e7244_d_n11;
        locals.var_eu_i_rv = 0.0;

        let assign5980_e7247: f64 = if locals.var_ud_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign5980_e7247;
        locals.var_guard72_rv = 0.0;

        let (assign5990_e7251, assign5990_e7251_d_n3, assign5990_e7251_d_n4, assign5990_e7251_d_n5, assign5990_e7251_d_n6, assign5990_e7251_d_n7, assign5990_e7251_d_n8, assign5990_e7251_d_n9, assign5990_e7251_d_n10, assign5990_e7251_d_n11,) = {
    if (locals.var_guard72 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ud_i, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11,)
    }
};
        locals.var_ud_i = assign5990_e7251;
        locals.var_ud_i_dn3 = assign5990_e7251_d_n3;
        locals.var_ud_i_dn4 = assign5990_e7251_d_n4;
        locals.var_ud_i_dn5 = assign5990_e7251_d_n5;
        locals.var_ud_i_dn6 = assign5990_e7251_d_n6;
        locals.var_ud_i_dn7 = assign5990_e7251_d_n7;
        locals.var_ud_i_dn8 = assign5990_e7251_d_n8;
        locals.var_ud_i_dn9 = assign5990_e7251_d_n9;
        locals.var_ud_i_dn10 = assign5990_e7251_d_n10;
        locals.var_ud_i_dn11 = assign5990_e7251_d_n11;
        locals.var_ud_i_rv = 0.0;

        let assign6000_e7254: f64 = if locals.var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign6000_e7254;
        locals.var_guard73_rv = 0.0;

        let (assign6010_e7258,) = {
    if (locals.var_guard73 != 0.0) {
        (0.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign6010_e7258;
        locals.var_ucs_i_rv = 0.0;

        let assign6020_e7261: f64 = if locals.var_ndiode_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign6020_e7261;
        locals.var_guard74_rv = 0.0;

        let (assign6030_e7265,) = {
    if (locals.var_guard74 != 0.0) {
        (1.0,)
    } else {
        (locals.var_ndiode_i,)
    }
};
        locals.var_ndiode_i = assign6030_e7265;
        locals.var_ndiode_i_rv = 0.0;

        let assign6040_e7268: f64 = if locals.var_nrecr0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign6040_e7268;
        locals.var_guard75_rv = 0.0;

        let (assign6050_e7272,) = {
    if (locals.var_guard75 != 0.0) {
        (10.0,)
    } else {
        (locals.var_nrecr0_i,)
    }
};
        locals.var_nrecr0_i = assign6050_e7272;
        locals.var_nrecr0_i_rv = 0.0;

        let assign6060_e7275: f64 = if locals.var_nrecf0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign6060_e7275;
        locals.var_guard76_rv = 0.0;

        let (assign6070_e7279,) = {
    if (locals.var_guard76 != 0.0) {
        (2.0,)
    } else {
        (locals.var_nrecf0_i,)
    }
};
        locals.var_nrecf0_i = assign6070_e7279;
        locals.var_nrecf0_i_rv = 0.0;

        locals.var_nuendd = 0.0;
        locals.var_nuendd_rv = 0.0;

        locals.var_nuends = 0.0;
        locals.var_nuends_rv = 0.0;

        locals.var_nuintd = 0.0;
        locals.var_nuintd_rv = 0.0;

        locals.var_nuints = 0.0;
        locals.var_nuints_rv = 0.0;

        locals.var_rend = 0.0;
        locals.var_rend_rv = 0.0;

        locals.var_rint = 0.0;
        locals.var_rint_rv = 0.0;

        let assign6150_e7291: f64 = (p.p895 - p.p898);
        locals.var_dmcgeff = assign6150_e7291;
        locals.var_dmcgeff_rv = 0.0;

        locals.var_dmcieff = p.p896;
        locals.var_dmcieff_rv = 0.0;

        let assign6170_e7295: f64 = (p.p897 - p.p898);
        locals.var_dmdgeff = assign6170_e7295;
        locals.var_dmdgeff_rv = 0.0;

        let assign6180_e7297: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard78 = assign6180_e7297;
        locals.var_guard78_rv = 0.0;

        let (assign6190_e7303,) = {
    if (locals.var_guard78 != 0.0) {
        let assign6190_e7301: f64 = (p.p438 * p.p3);
        (assign6190_e7301,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign6190_e7303;
        locals.var_rsourcegeo_rv = 0.0;

        let assign6200_e7310: f64 = if ((p.p9 > 0.0) && (p.p438 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign6200_e7310;
        locals.var_guard79_rv = 0.0;

        let assign6210_e7313: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign6210_e7313;
        locals.var_guard80_rv = 0.0;

        let assign6220_e7316: f64 = (p.p2 % 2.0);
        let assign6220_e7318: f64 = if assign6220_e7316 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign6220_e7318;
        locals.var_guard81_rv = 0.0;

        let (assign6230_e7329,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign6230_e7329;
        locals.var_nuendd_rv = 0.0;

        let (assign6240_e7340,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign6240_e7340;
        locals.var_nuends_rv = 0.0;

        let (assign6250_e7359,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        let assign6250_e7352: f64 = (p.p2 - 1.0);
        let assign6250_e7354: f64 = (assign6250_e7352 / 2.0);
        let assign6250_e7356: f64 = (assign6250_e7354).max(0.0);
        let assign6250_e7357: f64 = (2.0 * assign6250_e7356);
        (assign6250_e7357,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign6250_e7359;
        locals.var_nuintd_rv = 0.0;

        let (assign6260_e7370,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign6260_e7370;
        locals.var_nuints_rv = 0.0;

        let assign6270_e7373: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign6270_e7373;
        locals.var_guard82_rv = 0.0;

        let (assign6280_e7387,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign6280_e7387;
        locals.var_nuendd_rv = 0.0;

        let (assign6290_e7409,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        let assign6290_e7402: f64 = (p.p2 / 2.0);
        let assign6290_e7404: f64 = (assign6290_e7402 - 1.0);
        let assign6290_e7406: f64 = (assign6290_e7404).max(0.0);
        let assign6290_e7407: f64 = (2.0 * assign6290_e7406);
        (assign6290_e7407,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign6290_e7409;
        locals.var_nuintd_rv = 0.0;

        let (assign6300_e7423,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign6300_e7423;
        locals.var_nuends_rv = 0.0;

        let (assign6310_e7437,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign6310_e7437;
        locals.var_nuints_rv = 0.0;

        let (assign6320_e7452,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign6320_e7452;
        locals.var_nuendd_rv = 0.0;

        let (assign6330_e7467,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign6330_e7467;
        locals.var_nuintd_rv = 0.0;

        let (assign6340_e7482,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign6340_e7482;
        locals.var_nuends_rv = 0.0;

        let (assign6350_e7505,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        let assign6350_e7498: f64 = (p.p2 / 2.0);
        let assign6350_e7500: f64 = (assign6350_e7498 - 1.0);
        let assign6350_e7502: f64 = (assign6350_e7500).max(0.0);
        let assign6350_e7503: f64 = (2.0 * assign6350_e7502);
        (assign6350_e7503,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign6350_e7505;
        locals.var_nuints_rv = 0.0;

        let assign6360_e7508: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign6360_e7508;
        locals.var_guard83_rv = 0.0;

        let assign6370_e7511: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign6370_e7511;
        locals.var_guard84_rv = 0.0;

        let (assign6380_e7524,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 != 0.0)) && (locals.var_guard84 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6380_e7524;
        locals.var_rint_rv = 0.0;

        let (assign6390_e7544,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 != 0.0)) && (locals.var_guard84 == 0.0)) {
        let assign6390_e7538: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6390_e7541: f64 = (locals.var_weff * locals.var_nuints);
        let assign6390_e7542: f64 = (assign6390_e7538 / assign6390_e7541);
        (assign6390_e7542,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6390_e7544;
        locals.var_rint_rv = 0.0;

        let assign6400_e7547: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign6400_e7547;
        locals.var_guard85_rv = 0.0;

        let (assign6410_e7561,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 == 0.0)) && (locals.var_guard85 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6410_e7561;
        locals.var_rint_rv = 0.0;

        let (assign6420_e7582,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 == 0.0)) && (locals.var_guard85 == 0.0)) {
        let assign6420_e7576: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6420_e7579: f64 = (locals.var_weff * locals.var_nuintd);
        let assign6420_e7580: f64 = (assign6420_e7576 / assign6420_e7579);
        (assign6420_e7580,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6420_e7582;
        locals.var_rint_rv = 0.0;

        let assign6430_e7585: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign6430_e7585;
        locals.var_guard86_rv = 0.0;

        let assign6440_e7588: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign6440_e7588;
        locals.var_guard87_rv = 0.0;

        let assign6450_e7591: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign6450_e7591;
        locals.var_guard88_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6460_e7594: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign6460_e7594;
        locals.var_guard89_rv = 0.0;

        let assign6470_e7597: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign6470_e7597;
        locals.var_guard90_rv = 0.0;

        let assign6480_e7600: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign6480_e7600;
        locals.var_guard91_rv = 0.0;

        let assign6490_e7603: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign6490_e7603;
        locals.var_guard92_rv = 0.0;

        let assign6500_e7606: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign6500_e7606;
        locals.var_guard93_rv = 0.0;

        let assign6510_e7609: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign6510_e7609;
        locals.var_guard94_rv = 0.0;

        let assign6520_e7612: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign6520_e7612;
        locals.var_guard95_rv = 0.0;

        let assign6530_e7615: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign6530_e7615;
        locals.var_guard96_rv = 0.0;

        let assign6540_e7618: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign6540_e7618;
        locals.var_guard97_rv = 0.0;

        let assign6550_e7621: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign6550_e7621;
        locals.var_guard98_rv = 0.0;

        let assign6560_e7632: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign6560_e7632;
        locals.var_guard99_rv = 0.0;

        let assign6570_e7643: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign6570_e7643;
        locals.var_guard100_rv = 0.0;

        let assign6580_e7646: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign6580_e7646;
        locals.var_guard101_rv = 0.0;

        let (assign6590_e7663,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) && (locals.var_guard101 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6590_e7663;
        locals.var_rend_rv = 0.0;

        let (assign6600_e7687,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) && (locals.var_guard101 == 0.0)) {
        let assign6600_e7681: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6600_e7684: f64 = (locals.var_weff * locals.var_nuends);
        let assign6600_e7685: f64 = (assign6600_e7681 / assign6600_e7684);
        (assign6600_e7685,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6600_e7687;
        locals.var_rend_rv = 0.0;

        let assign6620_e7698: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6620_e7701: f64 = if ((locals.var_nuends == 0.0) || (assign6620_e7698 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign6620_e7701;
        locals.var_guard103_rv = 0.0;

        let (assign6630_e7721,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && ((locals.var_guard100 != 0.0) && (locals.var_guard99 == 0.0))) && (locals.var_guard103 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6630_e7721;
        locals.var_rend_rv = 0.0;

        let (assign6640_e7752,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && ((locals.var_guard100 != 0.0) && (locals.var_guard99 == 0.0))) && (locals.var_guard103 == 0.0)) {
        let assign6640_e7742: f64 = (p.p438 * locals.var_weff);
        let assign6640_e7745: f64 = (3.0 * locals.var_nuends);
        let assign6640_e7748: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6640_e7749: f64 = (assign6640_e7745 * assign6640_e7748);
        let assign6640_e7750: f64 = (assign6640_e7742 / assign6640_e7749);
        (assign6640_e7750,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6640_e7752;
        locals.var_rend_rv = 0.0;

        let (assign6650_e7770,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (!((locals.var_guard99 != 0.0) || (locals.var_guard100 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6650_e7770;
        locals.var_rend_rv = 0.0;

        let assign6660_e7781: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign6660_e7781;
        locals.var_guard104_rv = 0.0;

        let assign6670_e7792: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard105 = assign6670_e7792;
        locals.var_guard105_rv = 0.0;

        let assign6680_e7795: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign6680_e7795;
        locals.var_guard106_rv = 0.0;

        let (assign6690_e7813,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6690_e7813;
        locals.var_rend_rv = 0.0;

        let (assign6700_e7838,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign6700_e7832: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6700_e7835: f64 = (locals.var_weff * locals.var_nuends);
        let assign6700_e7836: f64 = (assign6700_e7832 / assign6700_e7835);
        (assign6700_e7836,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6700_e7838;
        locals.var_rend_rv = 0.0;

        let assign6720_e7849: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6720_e7852: f64 = if ((locals.var_nuends == 0.0) || (assign6720_e7849 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard108 = assign6720_e7852;
        locals.var_guard108_rv = 0.0;

        let (assign6730_e7873,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && ((locals.var_guard105 != 0.0) && (locals.var_guard104 == 0.0))) && (locals.var_guard108 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6730_e7873;
        locals.var_rend_rv = 0.0;

        let (assign6740_e7905,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && ((locals.var_guard105 != 0.0) && (locals.var_guard104 == 0.0))) && (locals.var_guard108 == 0.0)) {
        let assign6740_e7895: f64 = (p.p438 * locals.var_weff);
        let assign6740_e7898: f64 = (3.0 * locals.var_nuends);
        let assign6740_e7901: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6740_e7902: f64 = (assign6740_e7898 * assign6740_e7901);
        let assign6740_e7903: f64 = (assign6740_e7895 / assign6740_e7902);
        (assign6740_e7903,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6740_e7905;
        locals.var_rend_rv = 0.0;

        let (assign6750_e7924,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && (!((locals.var_guard104 != 0.0) || (locals.var_guard105 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6750_e7924;
        locals.var_rend_rv = 0.0;

        let assign6760_e7927: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign6760_e7927;
        locals.var_guard109_rv = 0.0;

        let assign6770_e7938: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6770_e7938;
        locals.var_guard110_rv = 0.0;

        let assign6780_e7949: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard111 = assign6780_e7949;
        locals.var_guard111_rv = 0.0;

        let assign6790_e7952: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6790_e7952;
        locals.var_guard112_rv = 0.0;

        let (assign6800_e7970,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6800_e7970;
        locals.var_rend_rv = 0.0;

        let (assign6810_e7995,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) && (locals.var_guard112 == 0.0)) {
        let assign6810_e7989: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6810_e7992: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6810_e7993: f64 = (assign6810_e7989 / assign6810_e7992);
        (assign6810_e7993,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6810_e7995;
        locals.var_rend_rv = 0.0;

        let assign6830_e8006: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6830_e8009: f64 = if ((locals.var_nuendd == 0.0) || (assign6830_e8006 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6830_e8009;
        locals.var_guard114_rv = 0.0;

        let (assign6840_e8030,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && ((locals.var_guard111 != 0.0) && (locals.var_guard110 == 0.0))) && (locals.var_guard114 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6840_e8030;
        locals.var_rend_rv = 0.0;

        let (assign6850_e8062,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && ((locals.var_guard111 != 0.0) && (locals.var_guard110 == 0.0))) && (locals.var_guard114 == 0.0)) {
        let assign6850_e8052: f64 = (p.p438 * locals.var_weff);
        let assign6850_e8055: f64 = (3.0 * locals.var_nuendd);
        let assign6850_e8058: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6850_e8059: f64 = (assign6850_e8055 * assign6850_e8058);
        let assign6850_e8060: f64 = (assign6850_e8052 / assign6850_e8059);
        (assign6850_e8060,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6850_e8062;
        locals.var_rend_rv = 0.0;

        let (assign6860_e8081,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && (!((locals.var_guard110 != 0.0) || (locals.var_guard111 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6860_e8081;
        locals.var_rend_rv = 0.0;

        let assign6870_e8092: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign6870_e8092;
        locals.var_guard115_rv = 0.0;

        let assign6880_e8103: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard116 = assign6880_e8103;
        locals.var_guard116_rv = 0.0;

        let assign6890_e8106: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign6890_e8106;
        locals.var_guard117_rv = 0.0;

        let (assign6900_e8125,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && (locals.var_guard115 != 0.0)) && (locals.var_guard117 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6900_e8125;
        locals.var_rend_rv = 0.0;

        let (assign6910_e8151,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && (locals.var_guard115 != 0.0)) && (locals.var_guard117 == 0.0)) {
        let assign6910_e8145: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6910_e8148: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6910_e8149: f64 = (assign6910_e8145 / assign6910_e8148);
        (assign6910_e8149,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6910_e8151;
        locals.var_rend_rv = 0.0;

        let assign6930_e8162: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6930_e8165: f64 = if ((locals.var_nuendd == 0.0) || (assign6930_e8162 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign6930_e8165;
        locals.var_guard119_rv = 0.0;

        let (assign6940_e8187,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && ((locals.var_guard116 != 0.0) && (locals.var_guard115 == 0.0))) && (locals.var_guard119 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6940_e8187;
        locals.var_rend_rv = 0.0;

        let (assign6950_e8220,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && ((locals.var_guard116 != 0.0) && (locals.var_guard115 == 0.0))) && (locals.var_guard119 == 0.0)) {
        let assign6950_e8210: f64 = (p.p438 * locals.var_weff);
        let assign6950_e8213: f64 = (3.0 * locals.var_nuendd);
        let assign6950_e8216: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6950_e8217: f64 = (assign6950_e8213 * assign6950_e8216);
        let assign6950_e8218: f64 = (assign6950_e8210 / assign6950_e8217);
        (assign6950_e8218,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6950_e8220;
        locals.var_rend_rv = 0.0;

        let (assign6960_e8240,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && (!((locals.var_guard115 != 0.0) || (locals.var_guard116 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6960_e8240;
        locals.var_rend_rv = 0.0;

        let assign6970_e8243: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign6970_e8243;
        locals.var_guard120_rv = 0.0;

        let assign6980_e8246: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign6980_e8246;
        locals.var_guard121_rv = 0.0;

        let assign6990_e8257: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign6990_e8257;
        locals.var_guard122_rv = 0.0;

        let assign7000_e8268: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign7000_e8268;
        locals.var_guard123_rv = 0.0;

        let assign7010_e8271: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign7010_e8271;
        locals.var_guard124_rv = 0.0;

        let (assign7020_e8291,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard122 != 0.0)) && (locals.var_guard124 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7020_e8291;
        locals.var_rend_rv = 0.0;

        let (assign7030_e8318,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard122 != 0.0)) && (locals.var_guard124 == 0.0)) {
        let assign7030_e8312: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7030_e8315: f64 = (locals.var_weff * locals.var_nuends);
        let assign7030_e8316: f64 = (assign7030_e8312 / assign7030_e8315);
        (assign7030_e8316,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7030_e8318;
        locals.var_rend_rv = 0.0;

        let assign7050_e8329: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7050_e8332: f64 = if ((locals.var_nuends == 0.0) || (assign7050_e8329 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard126 = assign7050_e8332;
        locals.var_guard126_rv = 0.0;

        let (assign7060_e8355,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && ((locals.var_guard123 != 0.0) && (locals.var_guard122 == 0.0))) && (locals.var_guard126 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7060_e8355;
        locals.var_rend_rv = 0.0;

        let (assign7070_e8389,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && ((locals.var_guard123 != 0.0) && (locals.var_guard122 == 0.0))) && (locals.var_guard126 == 0.0)) {
        let assign7070_e8379: f64 = (p.p438 * locals.var_weff);
        let assign7070_e8382: f64 = (3.0 * locals.var_nuends);
        let assign7070_e8385: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7070_e8386: f64 = (assign7070_e8382 * assign7070_e8385);
        let assign7070_e8387: f64 = (assign7070_e8379 / assign7070_e8386);
        (assign7070_e8387,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7070_e8389;
        locals.var_rend_rv = 0.0;

        let (assign7080_e8410,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (!((locals.var_guard122 != 0.0) || (locals.var_guard123 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7080_e8410;
        locals.var_rend_rv = 0.0;

        let assign7090_e8421: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign7090_e8421;
        locals.var_guard127_rv = 0.0;

        let assign7100_e8432: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign7100_e8432;
        locals.var_guard128_rv = 0.0;

        let assign7110_e8435: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign7110_e8435;
        locals.var_guard129_rv = 0.0;

        let (assign7120_e8456,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && (locals.var_guard127 != 0.0)) && (locals.var_guard129 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7120_e8456;
        locals.var_rend_rv = 0.0;

        let (assign7130_e8484,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && (locals.var_guard127 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign7130_e8478: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7130_e8481: f64 = (locals.var_weff * locals.var_nuends);
        let assign7130_e8482: f64 = (assign7130_e8478 / assign7130_e8481);
        (assign7130_e8482,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7130_e8484;
        locals.var_rend_rv = 0.0;

        let assign7150_e8495: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7150_e8498: f64 = if ((locals.var_nuends == 0.0) || (assign7150_e8495 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign7150_e8498;
        locals.var_guard131_rv = 0.0;

        let (assign7160_e8522,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && ((locals.var_guard128 != 0.0) && (locals.var_guard127 == 0.0))) && (locals.var_guard131 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7160_e8522;
        locals.var_rend_rv = 0.0;

        let (assign7170_e8557,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && ((locals.var_guard128 != 0.0) && (locals.var_guard127 == 0.0))) && (locals.var_guard131 == 0.0)) {
        let assign7170_e8547: f64 = (p.p438 * locals.var_weff);
        let assign7170_e8550: f64 = (3.0 * locals.var_nuends);
        let assign7170_e8553: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7170_e8554: f64 = (assign7170_e8550 * assign7170_e8553);
        let assign7170_e8555: f64 = (assign7170_e8547 / assign7170_e8554);
        (assign7170_e8555,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7170_e8557;
        locals.var_rend_rv = 0.0;

        let (assign7180_e8579,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && (!((locals.var_guard127 != 0.0) || (locals.var_guard128 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7180_e8579;
        locals.var_rend_rv = 0.0;

        let assign7190_e8582: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign7190_e8582;
        locals.var_guard132_rv = 0.0;

        let assign7200_e8593: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign7200_e8593;
        locals.var_guard133_rv = 0.0;

    }
}
