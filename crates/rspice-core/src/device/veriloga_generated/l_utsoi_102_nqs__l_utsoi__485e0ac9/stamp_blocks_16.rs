#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        var_fracinv_i: f64,
        var_guard1239: f64,
        var_idse: f64,
        var_idse_dn4: f64,
        var_idse_dn6: f64,
        var_idse_dn7: f64,
        var_idse_dn8: f64,
        var_idse_dn9: f64,
        var_igde: f64,
        var_igde_dn4: f64,
        var_igde_dn6: f64,
        var_igde_dn7: f64,
        var_igde_dn8: f64,
        var_igde_dn9: f64,
        var_igidle: f64,
        var_igidle_dn4: f64,
        var_igidle_dn6: f64,
        var_igidle_dn7: f64,
        var_igidle_dn8: f64,
        var_igidle_dn9: f64,
        var_igisle: f64,
        var_igisle_dn4: f64,
        var_igisle_dn6: f64,
        var_igisle_dn7: f64,
        var_igisle_dn8: f64,
        var_igisle_dn9: f64,
        var_igse: f64,
        var_igse_dn4: f64,
        var_igse_dn6: f64,
        var_igse_dn7: f64,
        var_igse_dn8: f64,
        var_igse_dn9: f64,
        var_itaueff: f64,
        var_itaueff_dn4: f64,
        var_itaueff_dn6: f64,
        var_itaueff_dn7: f64,
        var_itaueff_dn8: f64,
        var_itaueff_dn9: f64,
        var_ithpwre: f64,
        var_ithpwre_dn4: f64,
        var_ithpwre_dn6: f64,
        var_ithpwre_dn7: f64,
        var_ithpwre_dn8: f64,
        var_ithpwre_dn9: f64,
        var_ithrce: f64,
        var_ithrce_dn4: f64,
        var_ithrce_dn6: f64,
        var_ithrce_dn7: f64,
        var_ithrce_dn8: f64,
        var_ithrce_dn9: f64,
        var_kfracinv_i: f64,
        var_mult_i_int: f64,
        var_qb: f64,
        var_qb_dn4: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qb_wo_mult: f64,
        var_qb_wo_mult_dn4: f64,
        var_qb_wo_mult_dn6: f64,
        var_qb_wo_mult_dn7: f64,
        var_qb_wo_mult_dn8: f64,
        var_qb_wo_mult_dn9: f64,
        var_qbdif: f64,
        var_qbdif_dn4: f64,
        var_qbdif_dn6: f64,
        var_qbdif_dn7: f64,
        var_qbdif_dn8: f64,
        var_qbdif_dn9: f64,
        var_qbsif: f64,
        var_qbsif_dn4: f64,
        var_qbsif_dn6: f64,
        var_qbsif_dn7: f64,
        var_qbsif_dn8: f64,
        var_qbsif_dn9: f64,
        var_qd_wo_mult: f64,
        var_qd_wo_mult_dn4: f64,
        var_qd_wo_mult_dn6: f64,
        var_qd_wo_mult_dn7: f64,
        var_qd_wo_mult_dn8: f64,
        var_qd_wo_mult_dn9: f64,
        var_qdse: f64,
        var_qdse_dn6: f64,
        var_qdse_dn7: f64,
        var_qdsub: f64,
        var_qdsub_dn6: f64,
        var_qdsub_dn7: f64,
        var_qdsub_dn8: f64,
        var_qg_wo_mult: f64,
        var_qg_wo_mult_dn4: f64,
        var_qg_wo_mult_dn6: f64,
        var_qg_wo_mult_dn7: f64,
        var_qg_wo_mult_dn8: f64,
        var_qg_wo_mult_dn9: f64,
        var_qgbe: f64,
        var_qgbe_dn4: f64,
        var_qgbe_dn6: f64,
        var_qgbe_dn7: f64,
        var_qgbe_dn8: f64,
        var_qgbe_dn9: f64,
        var_qgde: f64,
        var_qgde_dn4: f64,
        var_qgde_dn6: f64,
        var_qgde_dn7: f64,
        var_qgde_dn8: f64,
        var_qgde_dn9: f64,
        var_qgdif: f64,
        var_qgdif_dn4: f64,
        var_qgdif_dn6: f64,
        var_qgdif_dn7: f64,
        var_qgdif_dn8: f64,
        var_qgdif_dn9: f64,
        var_qgse: f64,
        var_qgse_dn4: f64,
        var_qgse_dn6: f64,
        var_qgse_dn7: f64,
        var_qgse_dn8: f64,
        var_qgse_dn9: f64,
        var_qgsif: f64,
        var_qgsif_dn4: f64,
        var_qgsif_dn6: f64,
        var_qgsif_dn7: f64,
        var_qgsif_dn8: f64,
        var_qgsif_dn9: f64,
        var_qovd: f64,
        var_qovd_dn4: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovd_dn8: f64,
        var_qovd_dn9: f64,
        var_qovs: f64,
        var_qovs_dn4: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qovs_dn8: f64,
        var_qovs_dn9: f64,
        var_qssub: f64,
        var_qssub_dn6: f64,
        var_qssub_dn8: f64,
        var_qth: f64,
        var_qth_dn4: f64,
        var_qth_dn6: f64,
        var_qth_dn7: f64,
        var_qth_dn8: f64,
        var_qth_dn9: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq0_e510, eq0_e510_d_n4, eq0_e510_d_n6, eq0_e510_d_n7, eq0_e510_d_n8, eq0_e510_d_n9,) = {
    if (var_guard1239 != 0.0) {
        let eq0_e508: f64 = (p.p14 * var_idse);
        let eq0_e508_d_n4: f64 = (p.p14 * var_idse_dn4);
        let eq0_e508_d_n6: f64 = (p.p14 * var_idse_dn6);
        let eq0_e508_d_n7: f64 = (p.p14 * var_idse_dn7);
        let eq0_e508_d_n8: f64 = (p.p14 * var_idse_dn8);
        let eq0_e508_d_n9: f64 = (p.p14 * var_idse_dn9);
        (eq0_e508, eq0_e508_d_n4, eq0_e508_d_n6, eq0_e508_d_n7, eq0_e508_d_n8, eq0_e508_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e510;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e510_d_n4), multiplicity * (eq0_e510_d_n6), multiplicity * (eq0_e510_d_n7), multiplicity * (eq0_e510_d_n8), multiplicity * (eq0_e510_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e517, eq1_e517_d_n4, eq1_e517_d_n6, eq1_e517_d_n7, eq1_e517_d_n8, eq1_e517_d_n9,) = {
    if (var_guard1239 == 0.0) {
        let eq1_e515: f64 = (p.p14 * var_idse);
        let eq1_e515_d_n4: f64 = (p.p14 * var_idse_dn4);
        let eq1_e515_d_n6: f64 = (p.p14 * var_idse_dn6);
        let eq1_e515_d_n7: f64 = (p.p14 * var_idse_dn7);
        let eq1_e515_d_n8: f64 = (p.p14 * var_idse_dn8);
        let eq1_e515_d_n9: f64 = (p.p14 * var_idse_dn9);
        (eq1_e515, eq1_e515_d_n4, eq1_e515_d_n6, eq1_e515_d_n7, eq1_e515_d_n8, eq1_e515_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e517;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e517_d_n4), multiplicity * (eq1_e517_d_n6), multiplicity * (eq1_e517_d_n7), multiplicity * (eq1_e517_d_n8), multiplicity * (eq1_e517_d_n9)],
            [],
            [],
            1.0,
        );
        let eq2_e521: f64 = (var_igidle - var_igisle);
        let eq2_e521_d_n4: f64 = (var_igidle_dn4 - var_igisle_dn4);
        let eq2_e521_d_n6: f64 = (var_igidle_dn6 - var_igisle_dn6);
        let eq2_e521_d_n7: f64 = (var_igidle_dn7 - var_igisle_dn7);
        let eq2_e521_d_n8: f64 = (var_igidle_dn8 - var_igisle_dn8);
        let eq2_e521_d_n9: f64 = (var_igidle_dn9 - var_igisle_dn9);
        let eq2_e522: f64 = (p.p14 * eq2_e521);
        let eq2_e522_d_n4: f64 = (p.p14 * eq2_e521_d_n4);
        let eq2_e522_d_n6: f64 = (p.p14 * eq2_e521_d_n6);
        let eq2_e522_d_n7: f64 = (p.p14 * eq2_e521_d_n7);
        let eq2_e522_d_n8: f64 = (p.p14 * eq2_e521_d_n8);
        let eq2_e522_d_n9: f64 = (p.p14 * eq2_e521_d_n9);
        let eq2_value: f64 = eq2_e522;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e522_d_n4), multiplicity * (eq2_e522_d_n6), multiplicity * (eq2_e522_d_n7), multiplicity * (eq2_e522_d_n8), multiplicity * (eq2_e522_d_n9)],
            [],
            [],
            1.0,
        );
        let eq3_e525: f64 = (p.p14 * var_igse);
        let eq3_e525_d_n4: f64 = (p.p14 * var_igse_dn4);
        let eq3_e525_d_n6: f64 = (p.p14 * var_igse_dn6);
        let eq3_e525_d_n7: f64 = (p.p14 * var_igse_dn7);
        let eq3_e525_d_n8: f64 = (p.p14 * var_igse_dn8);
        let eq3_e525_d_n9: f64 = (p.p14 * var_igse_dn9);
        let eq3_value: f64 = eq3_e525;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e525_d_n4), multiplicity * (eq3_e525_d_n6), multiplicity * (eq3_e525_d_n7), multiplicity * (eq3_e525_d_n8), multiplicity * (eq3_e525_d_n9)],
            [],
            [],
            1.0,
        );
        let eq4_e528: f64 = (p.p14 * var_igde);
        let eq4_e528_d_n4: f64 = (p.p14 * var_igde_dn4);
        let eq4_e528_d_n6: f64 = (p.p14 * var_igde_dn6);
        let eq4_e528_d_n7: f64 = (p.p14 * var_igde_dn7);
        let eq4_e528_d_n8: f64 = (p.p14 * var_igde_dn8);
        let eq4_e528_d_n9: f64 = (p.p14 * var_igde_dn9);
        let eq4_value: f64 = eq4_e528;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e528_d_n4), multiplicity * (eq4_e528_d_n6), multiplicity * (eq4_e528_d_n7), multiplicity * (eq4_e528_d_n8), multiplicity * (eq4_e528_d_n9)],
            [],
            [],
            1.0,
        );
        let eq9_value: f64 = var_ithpwre;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (var_ithpwre_dn4), multiplicity * (var_ithpwre_dn6), multiplicity * (var_ithpwre_dn7), multiplicity * (var_ithpwre_dn8), multiplicity * (var_ithpwre_dn9)],
            [],
            [],
            1.0,
        );
        let eq10_value: f64 = var_ithrce;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (var_ithrce_dn4), multiplicity * (var_ithrce_dn6), multiplicity * (var_ithrce_dn7), multiplicity * (var_ithrce_dn8), multiplicity * (var_ithrce_dn9)],
            [],
            [],
            1.0,
        );
        let eq23_e642: f64 = (var_qg_wo_mult + var_qb_wo_mult);
        let eq23_e642_d_n4: f64 = (var_qg_wo_mult_dn4 + var_qb_wo_mult_dn4);
        let eq23_e642_d_n6: f64 = (var_qg_wo_mult_dn6 + var_qb_wo_mult_dn6);
        let eq23_e642_d_n7: f64 = (var_qg_wo_mult_dn7 + var_qb_wo_mult_dn7);
        let eq23_e642_d_n8: f64 = (var_qg_wo_mult_dn8 + var_qb_wo_mult_dn8);
        let eq23_e642_d_n9: f64 = (var_qg_wo_mult_dn9 + var_qb_wo_mult_dn9);
        let eq23_e643: f64 = (var_fracinv_i * eq23_e642);
        let eq23_e643_d_n4: f64 = (var_fracinv_i * eq23_e642_d_n4);
        let eq23_e643_d_n6: f64 = (var_fracinv_i * eq23_e642_d_n6);
        let eq23_e643_d_n7: f64 = (var_fracinv_i * eq23_e642_d_n7);
        let eq23_e643_d_n8: f64 = (var_fracinv_i * eq23_e642_d_n8);
        let eq23_e643_d_n9: f64 = (var_fracinv_i * eq23_e642_d_n9);
        let eq23_e644: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq23_e643);
        let eq23_value: f64 = eq23_e644;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(13),
            multiplicity * (eq23_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq23_e643_d_n4 * ddt_scale)), multiplicity * ((eq23_e643_d_n6 * ddt_scale)), multiplicity * ((eq23_e643_d_n7 * ddt_scale)), multiplicity * ((eq23_e643_d_n8 * ddt_scale)), multiplicity * ((eq23_e643_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq24_e647: f64 = (var_itaueff * (nv10 - nv13));
        let eq24_e647_d_n4: f64 = (var_itaueff_dn4 * (nv10 - nv13));
        let eq24_e647_d_n6: f64 = (var_itaueff_dn6 * (nv10 - nv13));
        let eq24_e647_d_n7: f64 = (var_itaueff_dn7 * (nv10 - nv13));
        let eq24_e647_d_n8: f64 = (var_itaueff_dn8 * (nv10 - nv13));
        let eq24_e647_d_n9: f64 = (var_itaueff_dn9 * (nv10 - nv13));
        let eq24_value: f64 = eq24_e647;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(13),
            multiplicity * (eq24_value),
            [4, 6, 7, 8, 9, 10, 13],
            [multiplicity * (eq24_e647_d_n4), multiplicity * (eq24_e647_d_n6), multiplicity * (eq24_e647_d_n7), multiplicity * (eq24_e647_d_n8), multiplicity * (eq24_e647_d_n9), multiplicity * (var_itaueff), multiplicity * ((-var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq26_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qd_wo_mult);
        let eq26_value: f64 = eq26_e653;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq26_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((var_qd_wo_mult_dn4 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn6 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn7 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn8 * ddt_scale)), multiplicity * ((var_qd_wo_mult_dn9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq27_e656: f64 = (var_itaueff * (nv12 - nv13));
        let eq27_e656_d_n4: f64 = (var_itaueff_dn4 * (nv12 - nv13));
        let eq27_e656_d_n6: f64 = (var_itaueff_dn6 * (nv12 - nv13));
        let eq27_e656_d_n7: f64 = (var_itaueff_dn7 * (nv12 - nv13));
        let eq27_e656_d_n8: f64 = (var_itaueff_dn8 * (nv12 - nv13));
        let eq27_e656_d_n9: f64 = (var_itaueff_dn9 * (nv12 - nv13));
        let eq27_value: f64 = eq27_e656;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq27_value),
            [4, 6, 7, 8, 9, 12, 13],
            [multiplicity * (eq27_e656_d_n4), multiplicity * (eq27_e656_d_n6), multiplicity * (eq27_e656_d_n7), multiplicity * (eq27_e656_d_n8), multiplicity * (eq27_e656_d_n9), multiplicity * (var_itaueff), multiplicity * ((-var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq29_e662: f64 = (var_kfracinv_i).sqrt();
        let eq29_e665: f64 = (1.0 - var_fracinv_i);
        let eq29_e668: f64 = (var_qg_wo_mult + var_qb_wo_mult);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n4: f64 = (eq29_e665 * eq23_e642_d_n4);
        let eq29_e669_d_n6: f64 = (eq29_e665 * eq23_e642_d_n6);
        let eq29_e669_d_n7: f64 = (eq29_e665 * eq23_e642_d_n7);
        let eq29_e669_d_n8: f64 = (eq29_e665 * eq23_e642_d_n8);
        let eq29_e669_d_n9: f64 = (eq29_e665 * eq23_e642_d_n9);
        let eq29_e670: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq29_e669);
        let eq29_e671: f64 = (eq29_e662 * eq29_e670);
        let eq29_e671_d_n4: f64 = (eq29_e662 * (eq29_e669_d_n4 * ddt_scale));
        let eq29_e671_d_n6: f64 = (eq29_e662 * (eq29_e669_d_n6 * ddt_scale));
        let eq29_e671_d_n7: f64 = (eq29_e662 * (eq29_e669_d_n7 * ddt_scale));
        let eq29_e671_d_n8: f64 = (eq29_e662 * (eq29_e669_d_n8 * ddt_scale));
        let eq29_e671_d_n9: f64 = (eq29_e662 * (eq29_e669_d_n9 * ddt_scale));
        let eq29_value: f64 = eq29_e671;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            Some(13),
            multiplicity * (eq29_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq29_e671_d_n4), multiplicity * (eq29_e671_d_n6), multiplicity * (eq29_e671_d_n7), multiplicity * (eq29_e671_d_n8), multiplicity * (eq29_e671_d_n9)],
            [],
            [],
            1.0,
        );
        let eq30_e674: f64 = (var_itaueff * (nv11 - nv13));
        let eq30_e674_d_n4: f64 = (var_itaueff_dn4 * (nv11 - nv13));
        let eq30_e674_d_n6: f64 = (var_itaueff_dn6 * (nv11 - nv13));
        let eq30_e674_d_n7: f64 = (var_itaueff_dn7 * (nv11 - nv13));
        let eq30_e674_d_n8: f64 = (var_itaueff_dn8 * (nv11 - nv13));
        let eq30_e674_d_n9: f64 = (var_itaueff_dn9 * (nv11 - nv13));
        let eq30_value: f64 = eq30_e674;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(13),
            multiplicity * (eq30_value),
            [4, 6, 7, 8, 9, 11, 13],
            [multiplicity * (eq30_e674_d_n4), multiplicity * (eq30_e674_d_n6), multiplicity * (eq30_e674_d_n7), multiplicity * (eq30_e674_d_n8), multiplicity * (eq30_e674_d_n9), multiplicity * (var_itaueff), multiplicity * ((-var_itaueff))],
            [],
            [],
            1.0,
        );
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e679: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e678);
        let eq31_e680: f64 = (var_kfracinv_i * eq31_e679);
        let eq31_e680_d_n11: f64 = (var_kfracinv_i * (1e-9 * ddt_scale));
        let eq31_e680_d_n13: f64 = (var_kfracinv_i * ((-1e-9) * ddt_scale));
        let eq31_value: f64 = eq31_e680;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(13),
            multiplicity * (eq31_value),
            11,
            multiplicity * (eq31_e680_d_n11),
            13,
            multiplicity * (eq31_e680_d_n13),
        );
        let eq32_e683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qb);
        let eq32_e685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qssub);
        let eq32_e686: f64 = (eq32_e683 + eq32_e685);
        let eq32_e686_d_n6: f64 = ((var_qb_dn6 * ddt_scale) + (var_qssub_dn6 * ddt_scale));
        let eq32_e686_d_n8: f64 = ((var_qb_dn8 * ddt_scale) + (var_qssub_dn8 * ddt_scale));
        let eq32_e688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, var_qbsif);
        let eq32_e689: f64 = (eq32_e686 + eq32_e688);
        let eq32_e689_d_n4: f64 = ((var_qb_dn4 * ddt_scale) + (var_qbsif_dn4 * ddt_scale));
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + (var_qbsif_dn6 * ddt_scale));
        let eq32_e689_d_n7: f64 = ((var_qb_dn7 * ddt_scale) + (var_qbsif_dn7 * ddt_scale));
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + (var_qbsif_dn8 * ddt_scale));
        let eq32_e689_d_n9: f64 = ((var_qb_dn9 * ddt_scale) + (var_qbsif_dn9 * ddt_scale));
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_value: f64 = eq32_e690;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq32_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq32_e690_d_n4), multiplicity * (eq32_e690_d_n6), multiplicity * (eq32_e690_d_n7), multiplicity * (eq32_e690_d_n8), multiplicity * (eq32_e690_d_n9)],
            [],
            [],
            1.0,
        );
        let eq33_e693: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, var_qgde);
        let eq33_e695: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qovd);
        let eq33_e696: f64 = (eq33_e693 + eq33_e695);
        let eq33_e696_d_n4: f64 = ((var_qgde_dn4 * ddt_scale) + (var_qovd_dn4 * ddt_scale));
        let eq33_e696_d_n6: f64 = ((var_qgde_dn6 * ddt_scale) + (var_qovd_dn6 * ddt_scale));
        let eq33_e696_d_n7: f64 = ((var_qgde_dn7 * ddt_scale) + (var_qovd_dn7 * ddt_scale));
        let eq33_e696_d_n8: f64 = ((var_qgde_dn8 * ddt_scale) + (var_qovd_dn8 * ddt_scale));
        let eq33_e696_d_n9: f64 = ((var_qgde_dn9 * ddt_scale) + (var_qovd_dn9 * ddt_scale));
        let eq33_e698: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qgdif);
        let eq33_e699: f64 = (eq33_e696 + eq33_e698);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + (var_qgdif_dn4 * ddt_scale));
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + (var_qgdif_dn6 * ddt_scale));
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + (var_qgdif_dn7 * ddt_scale));
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + (var_qgdif_dn8 * ddt_scale));
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + (var_qgdif_dn9 * ddt_scale));
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_value: f64 = eq33_e700;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq33_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq33_e700_d_n4), multiplicity * (eq33_e700_d_n6), multiplicity * (eq33_e700_d_n7), multiplicity * (eq33_e700_d_n8), multiplicity * (eq33_e700_d_n9)],
            [],
            [],
            1.0,
        );
        let eq34_e703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qdsub);
        let eq34_e705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, var_qbdif);
        let eq34_e706: f64 = (eq34_e703 + eq34_e705);
        let eq34_e706_d_n6: f64 = ((var_qdsub_dn6 * ddt_scale) + (var_qbdif_dn6 * ddt_scale));
        let eq34_e706_d_n7: f64 = ((var_qdsub_dn7 * ddt_scale) + (var_qbdif_dn7 * ddt_scale));
        let eq34_e706_d_n8: f64 = ((var_qdsub_dn8 * ddt_scale) + (var_qbdif_dn8 * ddt_scale));
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n4: f64 = (p.p14 * (var_qbdif_dn4 * ddt_scale));
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * (var_qbdif_dn9 * ddt_scale));
        let eq34_value: f64 = eq34_e707;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq34_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq34_e707_d_n4), multiplicity * (eq34_e707_d_n6), multiplicity * (eq34_e707_d_n7), multiplicity * (eq34_e707_d_n8), multiplicity * (eq34_e707_d_n9)],
            [],
            [],
            1.0,
        );
        let eq35_e710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qgbe);
        let eq35_e711: f64 = (p.p14 * eq35_e710);
        let eq35_e711_d_n4: f64 = (p.p14 * (var_qgbe_dn4 * ddt_scale));
        let eq35_e711_d_n6: f64 = (p.p14 * (var_qgbe_dn6 * ddt_scale));
        let eq35_e711_d_n7: f64 = (p.p14 * (var_qgbe_dn7 * ddt_scale));
        let eq35_e711_d_n8: f64 = (p.p14 * (var_qgbe_dn8 * ddt_scale));
        let eq35_e711_d_n9: f64 = (p.p14 * (var_qgbe_dn9 * ddt_scale));
        let eq35_value: f64 = eq35_e711;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq35_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq35_e711_d_n4), multiplicity * (eq35_e711_d_n6), multiplicity * (eq35_e711_d_n7), multiplicity * (eq35_e711_d_n8), multiplicity * (eq35_e711_d_n9)],
            [],
            [],
            1.0,
        );
        let eq36_e714: f64 = (-var_itaueff);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n4: f64 = ((-var_itaueff_dn4) * p.p32);
        let eq36_e716_d_n6: f64 = ((-var_itaueff_dn6) * p.p32);
        let eq36_e716_d_n7: f64 = ((-var_itaueff_dn7) * p.p32);
        let eq36_e716_d_n8: f64 = ((-var_itaueff_dn8) * p.p32);
        let eq36_e716_d_n9: f64 = ((-var_itaueff_dn9) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * var_mult_i_int);
        let eq36_e718_d_n4: f64 = (eq36_e716_d_n4 * var_mult_i_int);
        let eq36_e718_d_n6: f64 = (eq36_e716_d_n6 * var_mult_i_int);
        let eq36_e718_d_n7: f64 = (eq36_e716_d_n7 * var_mult_i_int);
        let eq36_e718_d_n8: f64 = (eq36_e716_d_n8 * var_mult_i_int);
        let eq36_e718_d_n9: f64 = (eq36_e716_d_n9 * var_mult_i_int);
        let eq36_e722: f64 = (var_kfracinv_i).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / eq36_e722;
        let eq36_e723: f64 = ((nv11 - nv13) * __rspice_inv_cse_0);
        let eq36_e723_d_n11: f64 = (1.0 * __rspice_inv_cse_0);
        let eq36_e723_d_n13: f64 = ((-1.0) * __rspice_inv_cse_0);
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n4: f64 = (eq36_e718_d_n4 * eq36_e724);
        let eq36_e725_d_n6: f64 = (eq36_e718_d_n6 * eq36_e724);
        let eq36_e725_d_n7: f64 = (eq36_e718_d_n7 * eq36_e724);
        let eq36_e725_d_n8: f64 = (eq36_e718_d_n8 * eq36_e724);
        let eq36_e725_d_n9: f64 = (eq36_e718_d_n9 * eq36_e724);
        let eq36_e725_d_n11: f64 = (eq36_e718 * eq36_e723_d_n11);
        let eq36_e725_d_n13: f64 = (eq36_e718 * eq36_e724_d_n13);
        let eq36_e727: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, var_qb);
        let eq36_e728: f64 = (eq36_e725 - eq36_e727);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - (var_qb_dn4 * ddt_scale));
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - (var_qb_dn6 * ddt_scale));
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - (var_qb_dn7 * ddt_scale));
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - (var_qb_dn8 * ddt_scale));
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - (var_qb_dn9 * ddt_scale));
        let eq36_e730: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qgse);
        let eq36_e731: f64 = (eq36_e728 + eq36_e730);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + (var_qgse_dn4 * ddt_scale));
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + (var_qgse_dn6 * ddt_scale));
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + (var_qgse_dn7 * ddt_scale));
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + (var_qgse_dn8 * ddt_scale));
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + (var_qgse_dn9 * ddt_scale));
        let eq36_e733: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, var_qovs);
        let eq36_e734: f64 = (eq36_e731 + eq36_e733);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + (var_qovs_dn4 * ddt_scale));
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + (var_qovs_dn6 * ddt_scale));
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + (var_qovs_dn7 * ddt_scale));
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + (var_qovs_dn8 * ddt_scale));
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + (var_qovs_dn9 * ddt_scale));
        let eq36_e736: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, var_qgsif);
        let eq36_e737: f64 = (eq36_e734 + eq36_e736);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + (var_qgsif_dn4 * ddt_scale));
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + (var_qgsif_dn6 * ddt_scale));
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + (var_qgsif_dn7 * ddt_scale));
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + (var_qgsif_dn8 * ddt_scale));
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + (var_qgsif_dn9 * ddt_scale));
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e718);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e725_d_n11);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e725_d_n13);
        let eq36_value: f64 = eq36_e738;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq36_value),
            [4, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq36_e738_d_n4), multiplicity * (eq36_e738_d_n6), multiplicity * (eq36_e738_d_n7), multiplicity * (eq36_e738_d_n8), multiplicity * (eq36_e738_d_n9), multiplicity * (eq36_e738_d_n10), multiplicity * (eq36_e738_d_n11), multiplicity * (eq36_e738_d_n13)],
            [],
            [],
            1.0,
        );
        let eq37_e741: f64 = (-var_itaueff);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n4: f64 = ((-var_itaueff_dn4) * p.p31);
        let eq37_e743_d_n6: f64 = ((-var_itaueff_dn6) * p.p31);
        let eq37_e743_d_n7: f64 = ((-var_itaueff_dn7) * p.p31);
        let eq37_e743_d_n8: f64 = ((-var_itaueff_dn8) * p.p31);
        let eq37_e743_d_n9: f64 = ((-var_itaueff_dn9) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * var_mult_i_int);
        let eq37_e745_d_n4: f64 = (eq37_e743_d_n4 * var_mult_i_int);
        let eq37_e745_d_n6: f64 = (eq37_e743_d_n6 * var_mult_i_int);
        let eq37_e745_d_n7: f64 = (eq37_e743_d_n7 * var_mult_i_int);
        let eq37_e745_d_n8: f64 = (eq37_e743_d_n8 * var_mult_i_int);
        let eq37_e745_d_n9: f64 = (eq37_e743_d_n9 * var_mult_i_int);
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e749: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, var_qdse);
        let eq37_e750: f64 = (eq37_e747 + eq37_e749);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + (var_qdse_dn6 * ddt_scale));
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + (var_qdse_dn7 * ddt_scale));
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e747_d_n4);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e747_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e747_d_n9);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e745);
        let eq37_e751_d_n13: f64 = (p.p14 * (-eq37_e745));
        let eq37_value: f64 = eq37_e751;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq37_value),
            [4, 6, 7, 8, 9, 12, 13],
            [multiplicity * (eq37_e751_d_n4), multiplicity * (eq37_e751_d_n6), multiplicity * (eq37_e751_d_n7), multiplicity * (eq37_e751_d_n8), multiplicity * (eq37_e751_d_n9), multiplicity * (eq37_e751_d_n12), multiplicity * (eq37_e751_d_n13)],
            [],
            [],
            1.0,
        );
        let eq38_e753: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, var_qth);
        let eq38_value: f64 = eq38_e753;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((var_qth_dn4 * ddt_scale)), multiplicity * ((var_qth_dn6 * ddt_scale)), multiplicity * ((var_qth_dn7 * ddt_scale)), multiplicity * ((var_qth_dn8 * ddt_scale)), multiplicity * ((var_qth_dn9 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_cdgeff: f64,
        var_cdgeff_dn4: f64,
        var_cdgeff_dn6: f64,
        var_cdgeff_dn7: f64,
        var_cdgeff_dn8: f64,
        var_cdgeff_dn9: f64,
        var_cgeff: f64,
        var_cgeff_dn4: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_cgeff_dn9: f64,
        var_csgeff: f64,
        var_csgeff_dn4: f64,
        var_csgeff_dn6: f64,
        var_csgeff_dn7: f64,
        var_csgeff_dn8: f64,
        var_csgeff_dn9: f64,
        var_gsig: f64,
        var_gsig_dn4: f64,
        var_gsig_dn6: f64,
        var_gsig_dn7: f64,
        var_gsig_dn8: f64,
        var_gsig_dn9: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq40_e759: f64 = (var_gsig * (nv5 - 0.0));
        let eq40_e759_d_n4: f64 = (var_gsig_dn4 * (nv5 - 0.0));
        let eq40_e759_d_n6: f64 = (var_gsig_dn6 * (nv5 - 0.0));
        let eq40_e759_d_n7: f64 = (var_gsig_dn7 * (nv5 - 0.0));
        let eq40_e759_d_n8: f64 = (var_gsig_dn8 * (nv5 - 0.0));
        let eq40_e759_d_n9: f64 = (var_gsig_dn9 * (nv5 - 0.0));
        let eq40_value: f64 = eq40_e759;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq40_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq40_e759_d_n4), multiplicity * (var_gsig), multiplicity * (eq40_e759_d_n6), multiplicity * (eq40_e759_d_n7), multiplicity * (eq40_e759_d_n8), multiplicity * (eq40_e759_d_n9)],
            [],
            [],
            1.0,
        );
        let eq41_e762: f64 = (var_cgeff * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (var_cgeff_dn4 * (nv5 - 0.0));
        let eq41_e762_d_n6: f64 = (var_cgeff_dn6 * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (var_cgeff_dn7 * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (var_cgeff_dn8 * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (var_cgeff_dn9 * (nv5 - 0.0));
        let eq41_e763: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq41_e762);
        let eq41_value: f64 = eq41_e763;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq41_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq41_e762_d_n4 * ddt_scale)), multiplicity * ((var_cgeff * ddt_scale)), multiplicity * ((eq41_e762_d_n6 * ddt_scale)), multiplicity * ((eq41_e762_d_n7 * ddt_scale)), multiplicity * ((eq41_e762_d_n8 * ddt_scale)), multiplicity * ((eq41_e762_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq42_e765: f64 = (-var_csgeff);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-var_csgeff_dn4) * (nv5 - 0.0));
        let eq42_e767_d_n6: f64 = ((-var_csgeff_dn6) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-var_csgeff_dn7) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-var_csgeff_dn8) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-var_csgeff_dn9) * (nv5 - 0.0));
        let eq42_e768: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, eq42_e767);
        let eq42_value: f64 = eq42_e768;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq42_e767_d_n4 * ddt_scale)), multiplicity * ((eq42_e765 * ddt_scale)), multiplicity * ((eq42_e767_d_n6 * ddt_scale)), multiplicity * ((eq42_e767_d_n7 * ddt_scale)), multiplicity * ((eq42_e767_d_n8 * ddt_scale)), multiplicity * ((eq42_e767_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e770: f64 = (-var_cdgeff);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-var_cdgeff_dn4) * (nv5 - 0.0));
        let eq43_e772_d_n6: f64 = ((-var_cdgeff_dn6) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-var_cdgeff_dn7) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-var_cdgeff_dn8) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-var_cdgeff_dn9) * (nv5 - 0.0));
        let eq43_e773: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq43_e772);
        let eq43_value: f64 = eq43_e773;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq43_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq43_e772_d_n4 * ddt_scale)), multiplicity * ((eq43_e770 * ddt_scale)), multiplicity * ((eq43_e772_d_n6 * ddt_scale)), multiplicity * ((eq43_e772_d_n7 * ddt_scale)), multiplicity * ((eq43_e772_d_n8 * ddt_scale)), multiplicity * ((eq43_e772_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cdgeff: f64,
        var_cdgeff_dn4: f64,
        var_cdgeff_dn6: f64,
        var_cdgeff_dn7: f64,
        var_cdgeff_dn8: f64,
        var_cdgeff_dn9: f64,
        var_cgeff: f64,
        var_cgeff_dn4: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_cgeff_dn9: f64,
        var_csgeff: f64,
        var_csgeff_dn4: f64,
        var_csgeff_dn6: f64,
        var_csgeff_dn7: f64,
        var_csgeff_dn8: f64,
        var_csgeff_dn9: f64,
        var_fracinv_i: f64,
        var_itaueff: f64,
        var_itaueff_dn4: f64,
        var_itaueff_dn6: f64,
        var_itaueff_dn7: f64,
        var_itaueff_dn8: f64,
        var_itaueff_dn9: f64,
        var_kfracinv_i: f64,
        var_mult_i_int: f64,
        var_qb: f64,
        var_qb_dn4: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qb_wo_mult: f64,
        var_qb_wo_mult_dn4: f64,
        var_qb_wo_mult_dn6: f64,
        var_qb_wo_mult_dn7: f64,
        var_qb_wo_mult_dn8: f64,
        var_qb_wo_mult_dn9: f64,
        var_qbdif: f64,
        var_qbdif_dn4: f64,
        var_qbdif_dn6: f64,
        var_qbdif_dn7: f64,
        var_qbdif_dn8: f64,
        var_qbdif_dn9: f64,
        var_qbsif: f64,
        var_qbsif_dn4: f64,
        var_qbsif_dn6: f64,
        var_qbsif_dn7: f64,
        var_qbsif_dn8: f64,
        var_qbsif_dn9: f64,
        var_qd_wo_mult: f64,
        var_qd_wo_mult_dn4: f64,
        var_qd_wo_mult_dn6: f64,
        var_qd_wo_mult_dn7: f64,
        var_qd_wo_mult_dn8: f64,
        var_qd_wo_mult_dn9: f64,
        var_qdse: f64,
        var_qdse_dn6: f64,
        var_qdse_dn7: f64,
        var_qdsub: f64,
        var_qdsub_dn6: f64,
        var_qdsub_dn7: f64,
        var_qdsub_dn8: f64,
        var_qg_wo_mult: f64,
        var_qg_wo_mult_dn4: f64,
        var_qg_wo_mult_dn6: f64,
        var_qg_wo_mult_dn7: f64,
        var_qg_wo_mult_dn8: f64,
        var_qg_wo_mult_dn9: f64,
        var_qgbe: f64,
        var_qgbe_dn4: f64,
        var_qgbe_dn6: f64,
        var_qgbe_dn7: f64,
        var_qgbe_dn8: f64,
        var_qgbe_dn9: f64,
        var_qgde: f64,
        var_qgde_dn4: f64,
        var_qgde_dn6: f64,
        var_qgde_dn7: f64,
        var_qgde_dn8: f64,
        var_qgde_dn9: f64,
        var_qgdif: f64,
        var_qgdif_dn4: f64,
        var_qgdif_dn6: f64,
        var_qgdif_dn7: f64,
        var_qgdif_dn8: f64,
        var_qgdif_dn9: f64,
        var_qgse: f64,
        var_qgse_dn4: f64,
        var_qgse_dn6: f64,
        var_qgse_dn7: f64,
        var_qgse_dn8: f64,
        var_qgse_dn9: f64,
        var_qgsif: f64,
        var_qgsif_dn4: f64,
        var_qgsif_dn6: f64,
        var_qgsif_dn7: f64,
        var_qgsif_dn8: f64,
        var_qgsif_dn9: f64,
        var_qovd: f64,
        var_qovd_dn4: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovd_dn8: f64,
        var_qovd_dn9: f64,
        var_qovs: f64,
        var_qovs_dn4: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qovs_dn8: f64,
        var_qovs_dn9: f64,
        var_qssub: f64,
        var_qssub_dn6: f64,
        var_qssub_dn8: f64,
        var_qth: f64,
        var_qth_dn4: f64,
        var_qth_dn6: f64,
        var_qth_dn7: f64,
        var_qth_dn8: f64,
        var_qth_dn9: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq23_e642: f64 = (var_qg_wo_mult + var_qb_wo_mult);
        let eq23_e642_d_n4: f64 = (var_qg_wo_mult_dn4 + var_qb_wo_mult_dn4);
        let eq23_e642_d_n6: f64 = (var_qg_wo_mult_dn6 + var_qb_wo_mult_dn6);
        let eq23_e642_d_n7: f64 = (var_qg_wo_mult_dn7 + var_qb_wo_mult_dn7);
        let eq23_e642_d_n8: f64 = (var_qg_wo_mult_dn8 + var_qb_wo_mult_dn8);
        let eq23_e642_d_n9: f64 = (var_qg_wo_mult_dn9 + var_qb_wo_mult_dn9);
        let eq23_e643: f64 = (var_fracinv_i * eq23_e642);
        let eq23_e643_d_n4: f64 = (var_fracinv_i * eq23_e642_d_n4);
        let eq23_e643_d_n6: f64 = (var_fracinv_i * eq23_e642_d_n6);
        let eq23_e643_d_n7: f64 = (var_fracinv_i * eq23_e642_d_n7);
        let eq23_e643_d_n8: f64 = (var_fracinv_i * eq23_e642_d_n8);
        let eq23_e643_d_n9: f64 = (var_fracinv_i * eq23_e642_d_n9);
        let eq23_e644_q: f64 = eq23_e643;
        let eq23_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq23_e643_d_n4, 0.0, eq23_e643_d_n6, eq23_e643_d_n7, eq23_e643_d_n8, eq23_e643_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[13]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e653_q: f64 = var_qd_wo_mult;
        let eq26_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, var_qd_wo_mult_dn4, 0.0, var_qd_wo_mult_dn6, var_qd_wo_mult_dn7, var_qd_wo_mult_dn8, var_qd_wo_mult_dn9, 0.0, 0.0, 0.0, 0.0];
        let eq26_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[13]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e662: f64 = (var_kfracinv_i).sqrt();
        let eq29_e665: f64 = (1.0 - var_fracinv_i);
        let eq29_e668: f64 = (var_qg_wo_mult + var_qb_wo_mult);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n4: f64 = (eq29_e665 * eq23_e642_d_n4);
        let eq29_e669_d_n6: f64 = (eq29_e665 * eq23_e642_d_n6);
        let eq29_e669_d_n7: f64 = (eq29_e665 * eq23_e642_d_n7);
        let eq29_e669_d_n8: f64 = (eq29_e665 * eq23_e642_d_n8);
        let eq29_e669_d_n9: f64 = (eq29_e665 * eq23_e642_d_n9);
        let eq29_e670_q: f64 = eq29_e669;
        let eq29_e671: f64 = (eq29_e662 * eq29_e669);
        let eq29_e671_d_n4: f64 = (eq29_e662 * eq29_e669_d_n4);
        let eq29_e671_d_n6: f64 = (eq29_e662 * eq29_e669_d_n6);
        let eq29_e671_d_n7: f64 = (eq29_e662 * eq29_e669_d_n7);
        let eq29_e671_d_n8: f64 = (eq29_e662 * eq29_e669_d_n8);
        let eq29_e671_d_n9: f64 = (eq29_e662 * eq29_e669_d_n9);
        let eq29_e671_q: f64 = (eq29_e662 * eq29_e670_q);
        let eq29_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq29_e671_d_n4, 0.0, eq29_e671_d_n6, eq29_e671_d_n7, eq29_e671_d_n8, eq29_e671_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq29_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e679_q: f64 = eq31_e678;
        let eq31_e680: f64 = (var_kfracinv_i * eq31_e678);
        let eq31_e680_d_n11: f64 = (var_kfracinv_i * 1e-9);
        let eq31_e680_d_n13: f64 = (var_kfracinv_i * (-1e-9));
        let eq31_e680_q: f64 = (var_kfracinv_i * eq31_e679_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[13]),
            nodes[11],
            multiplicity * (eq31_e680_d_n11),
            nodes[13],
            multiplicity * (eq31_e680_d_n13),
        );
        let eq32_e683_q: f64 = var_qb;
        let eq32_e685_q: f64 = var_qssub;
        let eq32_e686: f64 = (var_qb + var_qssub);
        let eq32_e686_d_n6: f64 = (var_qb_dn6 + var_qssub_dn6);
        let eq32_e686_d_n8: f64 = (var_qb_dn8 + var_qssub_dn8);
        let eq32_e686_q: f64 = (eq32_e683_q + eq32_e685_q);
        let eq32_e688_q: f64 = var_qbsif;
        let eq32_e689: f64 = (eq32_e686 + var_qbsif);
        let eq32_e689_d_n4: f64 = (var_qb_dn4 + var_qbsif_dn4);
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + var_qbsif_dn6);
        let eq32_e689_d_n7: f64 = (var_qb_dn7 + var_qbsif_dn7);
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + var_qbsif_dn8);
        let eq32_e689_d_n9: f64 = (var_qb_dn9 + var_qbsif_dn9);
        let eq32_e689_q: f64 = (eq32_e686_q + eq32_e688_q);
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_e690_q: f64 = (p.p14 * eq32_e689_q);
        let eq32_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq32_e690_d_n4, 0.0, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq32_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e693_q: f64 = var_qgde;
        let eq33_e695_q: f64 = var_qovd;
        let eq33_e696: f64 = (var_qgde + var_qovd);
        let eq33_e696_d_n4: f64 = (var_qgde_dn4 + var_qovd_dn4);
        let eq33_e696_d_n6: f64 = (var_qgde_dn6 + var_qovd_dn6);
        let eq33_e696_d_n7: f64 = (var_qgde_dn7 + var_qovd_dn7);
        let eq33_e696_d_n8: f64 = (var_qgde_dn8 + var_qovd_dn8);
        let eq33_e696_d_n9: f64 = (var_qgde_dn9 + var_qovd_dn9);
        let eq33_e696_q: f64 = (eq33_e693_q + eq33_e695_q);
        let eq33_e698_q: f64 = var_qgdif;
        let eq33_e699: f64 = (eq33_e696 + var_qgdif);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + var_qgdif_dn4);
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + var_qgdif_dn6);
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + var_qgdif_dn7);
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + var_qgdif_dn8);
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + var_qgdif_dn9);
        let eq33_e699_q: f64 = (eq33_e696_q + eq33_e698_q);
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_e700_q: f64 = (p.p14 * eq33_e699_q);
        let eq33_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq33_e700_d_n4, 0.0, eq33_e700_d_n6, eq33_e700_d_n7, eq33_e700_d_n8, eq33_e700_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq33_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e703_q: f64 = var_qdsub;
        let eq34_e705_q: f64 = var_qbdif;
        let eq34_e706: f64 = (var_qdsub + var_qbdif);
        let eq34_e706_d_n6: f64 = (var_qdsub_dn6 + var_qbdif_dn6);
        let eq34_e706_d_n7: f64 = (var_qdsub_dn7 + var_qbdif_dn7);
        let eq34_e706_d_n8: f64 = (var_qdsub_dn8 + var_qbdif_dn8);
        let eq34_e706_q: f64 = (eq34_e703_q + eq34_e705_q);
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n4: f64 = (p.p14 * var_qbdif_dn4);
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * var_qbdif_dn9);
        let eq34_e707_q: f64 = (p.p14 * eq34_e706_q);
        let eq34_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq34_e707_d_n4, 0.0, eq34_e707_d_n6, eq34_e707_d_n7, eq34_e707_d_n8, eq34_e707_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e710_q: f64 = var_qgbe;
        let eq35_e711: f64 = (p.p14 * var_qgbe);
        let eq35_e711_d_n4: f64 = (p.p14 * var_qgbe_dn4);
        let eq35_e711_d_n6: f64 = (p.p14 * var_qgbe_dn6);
        let eq35_e711_d_n7: f64 = (p.p14 * var_qgbe_dn7);
        let eq35_e711_d_n8: f64 = (p.p14 * var_qgbe_dn8);
        let eq35_e711_d_n9: f64 = (p.p14 * var_qgbe_dn9);
        let eq35_e711_q: f64 = (p.p14 * eq35_e710_q);
        let eq35_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq35_e711_d_n4, 0.0, eq35_e711_d_n6, eq35_e711_d_n7, eq35_e711_d_n8, eq35_e711_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e714: f64 = (-var_itaueff);
        let eq36_e716: f64 = (eq36_e714 * p.p32);
        let eq36_e716_d_n4: f64 = ((-var_itaueff_dn4) * p.p32);
        let eq36_e716_d_n6: f64 = ((-var_itaueff_dn6) * p.p32);
        let eq36_e716_d_n7: f64 = ((-var_itaueff_dn7) * p.p32);
        let eq36_e716_d_n8: f64 = ((-var_itaueff_dn8) * p.p32);
        let eq36_e716_d_n9: f64 = ((-var_itaueff_dn9) * p.p32);
        let eq36_e718: f64 = (eq36_e716 * var_mult_i_int);
        let eq36_e718_d_n4: f64 = (eq36_e716_d_n4 * var_mult_i_int);
        let eq36_e718_d_n6: f64 = (eq36_e716_d_n6 * var_mult_i_int);
        let eq36_e718_d_n7: f64 = (eq36_e716_d_n7 * var_mult_i_int);
        let eq36_e718_d_n8: f64 = (eq36_e716_d_n8 * var_mult_i_int);
        let eq36_e718_d_n9: f64 = (eq36_e716_d_n9 * var_mult_i_int);
        let eq36_e722: f64 = (var_kfracinv_i).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / eq36_e722;
        let eq36_e723: f64 = ((nv11 - nv13) * __rspice_inv_cse_0);
        let eq36_e723_d_n11: f64 = (1.0 * __rspice_inv_cse_0);
        let eq36_e723_d_n13: f64 = ((-1.0) * __rspice_inv_cse_0);
        let eq36_e724: f64 = ((nv10 - nv13) + eq36_e723);
        let eq36_e724_d_n13: f64 = (-1.0 + eq36_e723_d_n13);
        let eq36_e725: f64 = (eq36_e718 * eq36_e724);
        let eq36_e725_d_n4: f64 = (eq36_e718_d_n4 * eq36_e724);
        let eq36_e725_d_n6: f64 = (eq36_e718_d_n6 * eq36_e724);
        let eq36_e725_d_n7: f64 = (eq36_e718_d_n7 * eq36_e724);
        let eq36_e725_d_n8: f64 = (eq36_e718_d_n8 * eq36_e724);
        let eq36_e725_d_n9: f64 = (eq36_e718_d_n9 * eq36_e724);
        let eq36_e725_d_n11: f64 = (eq36_e718 * eq36_e723_d_n11);
        let eq36_e725_d_n13: f64 = (eq36_e718 * eq36_e724_d_n13);
        let eq36_e727_q: f64 = var_qb;
        let eq36_e728: f64 = (eq36_e725 - var_qb);
        let eq36_e728_d_n4: f64 = (eq36_e725_d_n4 - var_qb_dn4);
        let eq36_e728_d_n6: f64 = (eq36_e725_d_n6 - var_qb_dn6);
        let eq36_e728_d_n7: f64 = (eq36_e725_d_n7 - var_qb_dn7);
        let eq36_e728_d_n8: f64 = (eq36_e725_d_n8 - var_qb_dn8);
        let eq36_e728_d_n9: f64 = (eq36_e725_d_n9 - var_qb_dn9);
        let eq36_e728_q: f64 = (-eq36_e727_q);
        let eq36_e730_q: f64 = var_qgse;
        let eq36_e731: f64 = (eq36_e728 + var_qgse);
        let eq36_e731_d_n4: f64 = (eq36_e728_d_n4 + var_qgse_dn4);
        let eq36_e731_d_n6: f64 = (eq36_e728_d_n6 + var_qgse_dn6);
        let eq36_e731_d_n7: f64 = (eq36_e728_d_n7 + var_qgse_dn7);
        let eq36_e731_d_n8: f64 = (eq36_e728_d_n8 + var_qgse_dn8);
        let eq36_e731_d_n9: f64 = (eq36_e728_d_n9 + var_qgse_dn9);
        let eq36_e731_q: f64 = (eq36_e728_q + eq36_e730_q);
        let eq36_e731_q_d_n4: f64 = ((-var_qb_dn4) + var_qgse_dn4);
        let eq36_e731_q_d_n6: f64 = ((-var_qb_dn6) + var_qgse_dn6);
        let eq36_e731_q_d_n7: f64 = ((-var_qb_dn7) + var_qgse_dn7);
        let eq36_e731_q_d_n8: f64 = ((-var_qb_dn8) + var_qgse_dn8);
        let eq36_e731_q_d_n9: f64 = ((-var_qb_dn9) + var_qgse_dn9);
        let eq36_e733_q: f64 = var_qovs;
        let eq36_e734: f64 = (eq36_e731 + var_qovs);
        let eq36_e734_d_n4: f64 = (eq36_e731_d_n4 + var_qovs_dn4);
        let eq36_e734_d_n6: f64 = (eq36_e731_d_n6 + var_qovs_dn6);
        let eq36_e734_d_n7: f64 = (eq36_e731_d_n7 + var_qovs_dn7);
        let eq36_e734_d_n8: f64 = (eq36_e731_d_n8 + var_qovs_dn8);
        let eq36_e734_d_n9: f64 = (eq36_e731_d_n9 + var_qovs_dn9);
        let eq36_e734_q: f64 = (eq36_e731_q + eq36_e733_q);
        let eq36_e734_q_d_n4: f64 = (eq36_e731_q_d_n4 + var_qovs_dn4);
        let eq36_e734_q_d_n6: f64 = (eq36_e731_q_d_n6 + var_qovs_dn6);
        let eq36_e734_q_d_n7: f64 = (eq36_e731_q_d_n7 + var_qovs_dn7);
        let eq36_e734_q_d_n8: f64 = (eq36_e731_q_d_n8 + var_qovs_dn8);
        let eq36_e734_q_d_n9: f64 = (eq36_e731_q_d_n9 + var_qovs_dn9);
        let eq36_e736_q: f64 = var_qgsif;
        let eq36_e737: f64 = (eq36_e734 + var_qgsif);
        let eq36_e737_d_n4: f64 = (eq36_e734_d_n4 + var_qgsif_dn4);
        let eq36_e737_d_n6: f64 = (eq36_e734_d_n6 + var_qgsif_dn6);
        let eq36_e737_d_n7: f64 = (eq36_e734_d_n7 + var_qgsif_dn7);
        let eq36_e737_d_n8: f64 = (eq36_e734_d_n8 + var_qgsif_dn8);
        let eq36_e737_d_n9: f64 = (eq36_e734_d_n9 + var_qgsif_dn9);
        let eq36_e737_q: f64 = (eq36_e734_q + eq36_e736_q);
        let eq36_e737_q_d_n4: f64 = (eq36_e734_q_d_n4 + var_qgsif_dn4);
        let eq36_e737_q_d_n6: f64 = (eq36_e734_q_d_n6 + var_qgsif_dn6);
        let eq36_e737_q_d_n7: f64 = (eq36_e734_q_d_n7 + var_qgsif_dn7);
        let eq36_e737_q_d_n8: f64 = (eq36_e734_q_d_n8 + var_qgsif_dn8);
        let eq36_e737_q_d_n9: f64 = (eq36_e734_q_d_n9 + var_qgsif_dn9);
        let eq36_e738: f64 = (p.p14 * eq36_e737);
        let eq36_e738_d_n4: f64 = (p.p14 * eq36_e737_d_n4);
        let eq36_e738_d_n6: f64 = (p.p14 * eq36_e737_d_n6);
        let eq36_e738_d_n7: f64 = (p.p14 * eq36_e737_d_n7);
        let eq36_e738_d_n8: f64 = (p.p14 * eq36_e737_d_n8);
        let eq36_e738_d_n9: f64 = (p.p14 * eq36_e737_d_n9);
        let eq36_e738_d_n10: f64 = (p.p14 * eq36_e718);
        let eq36_e738_d_n11: f64 = (p.p14 * eq36_e725_d_n11);
        let eq36_e738_d_n13: f64 = (p.p14 * eq36_e725_d_n13);
        let eq36_e738_q: f64 = (p.p14 * eq36_e737_q);
        let eq36_e738_q_d_n4: f64 = (p.p14 * eq36_e737_q_d_n4);
        let eq36_e738_q_d_n6: f64 = (p.p14 * eq36_e737_q_d_n6);
        let eq36_e738_q_d_n7: f64 = (p.p14 * eq36_e737_q_d_n7);
        let eq36_e738_q_d_n8: f64 = (p.p14 * eq36_e737_q_d_n8);
        let eq36_e738_q_d_n9: f64 = (p.p14 * eq36_e737_q_d_n9);
        let eq36_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq36_e738_q_d_n4, 0.0, eq36_e738_q_d_n6, eq36_e738_q_d_n7, eq36_e738_q_d_n8, eq36_e738_q_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e741: f64 = (-var_itaueff);
        let eq37_e743: f64 = (eq37_e741 * p.p31);
        let eq37_e743_d_n4: f64 = ((-var_itaueff_dn4) * p.p31);
        let eq37_e743_d_n6: f64 = ((-var_itaueff_dn6) * p.p31);
        let eq37_e743_d_n7: f64 = ((-var_itaueff_dn7) * p.p31);
        let eq37_e743_d_n8: f64 = ((-var_itaueff_dn8) * p.p31);
        let eq37_e743_d_n9: f64 = ((-var_itaueff_dn9) * p.p31);
        let eq37_e745: f64 = (eq37_e743 * var_mult_i_int);
        let eq37_e745_d_n4: f64 = (eq37_e743_d_n4 * var_mult_i_int);
        let eq37_e745_d_n6: f64 = (eq37_e743_d_n6 * var_mult_i_int);
        let eq37_e745_d_n7: f64 = (eq37_e743_d_n7 * var_mult_i_int);
        let eq37_e745_d_n8: f64 = (eq37_e743_d_n8 * var_mult_i_int);
        let eq37_e745_d_n9: f64 = (eq37_e743_d_n9 * var_mult_i_int);
        let eq37_e747: f64 = (eq37_e745 * (nv12 - nv13));
        let eq37_e747_d_n4: f64 = (eq37_e745_d_n4 * (nv12 - nv13));
        let eq37_e747_d_n6: f64 = (eq37_e745_d_n6 * (nv12 - nv13));
        let eq37_e747_d_n7: f64 = (eq37_e745_d_n7 * (nv12 - nv13));
        let eq37_e747_d_n8: f64 = (eq37_e745_d_n8 * (nv12 - nv13));
        let eq37_e747_d_n9: f64 = (eq37_e745_d_n9 * (nv12 - nv13));
        let eq37_e749_q: f64 = var_qdse;
        let eq37_e750: f64 = (eq37_e747 + var_qdse);
        let eq37_e750_d_n6: f64 = (eq37_e747_d_n6 + var_qdse_dn6);
        let eq37_e750_d_n7: f64 = (eq37_e747_d_n7 + var_qdse_dn7);
        let eq37_e750_q: f64 = eq37_e749_q;
        let eq37_e751: f64 = (p.p14 * eq37_e750);
        let eq37_e751_d_n4: f64 = (p.p14 * eq37_e747_d_n4);
        let eq37_e751_d_n6: f64 = (p.p14 * eq37_e750_d_n6);
        let eq37_e751_d_n7: f64 = (p.p14 * eq37_e750_d_n7);
        let eq37_e751_d_n8: f64 = (p.p14 * eq37_e747_d_n8);
        let eq37_e751_d_n9: f64 = (p.p14 * eq37_e747_d_n9);
        let eq37_e751_d_n12: f64 = (p.p14 * eq37_e745);
        let eq37_e751_d_n13: f64 = (p.p14 * (-eq37_e745));
        let eq37_e751_q: f64 = (p.p14 * eq37_e750_q);
        let eq37_e751_q_d_n6: f64 = (p.p14 * var_qdse_dn6);
        let eq37_e751_q_d_n7: f64 = (p.p14 * var_qdse_dn7);
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes[6],
            multiplicity * (eq37_e751_q_d_n6),
            nodes[7],
            multiplicity * (eq37_e751_q_d_n7),
        );
        let eq38_e753_q: f64 = var_qth;
        let eq38_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, var_qth_dn4, 0.0, var_qth_dn6, var_qth_dn7, var_qth_dn8, var_qth_dn9, 0.0, 0.0, 0.0, 0.0];
        let eq38_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e762: f64 = (var_cgeff * (nv5 - 0.0));
        let eq41_e762_d_n4: f64 = (var_cgeff_dn4 * (nv5 - 0.0));
        let eq41_e762_d_n6: f64 = (var_cgeff_dn6 * (nv5 - 0.0));
        let eq41_e762_d_n7: f64 = (var_cgeff_dn7 * (nv5 - 0.0));
        let eq41_e762_d_n8: f64 = (var_cgeff_dn8 * (nv5 - 0.0));
        let eq41_e762_d_n9: f64 = (var_cgeff_dn9 * (nv5 - 0.0));
        let eq41_e763_q: f64 = eq41_e762;
        let eq41_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq41_e762_d_n4, var_cgeff, eq41_e762_d_n6, eq41_e762_d_n7, eq41_e762_d_n8, eq41_e762_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e765: f64 = (-var_csgeff);
        let eq42_e767: f64 = (eq42_e765 * (nv5 - 0.0));
        let eq42_e767_d_n4: f64 = ((-var_csgeff_dn4) * (nv5 - 0.0));
        let eq42_e767_d_n6: f64 = ((-var_csgeff_dn6) * (nv5 - 0.0));
        let eq42_e767_d_n7: f64 = ((-var_csgeff_dn7) * (nv5 - 0.0));
        let eq42_e767_d_n8: f64 = ((-var_csgeff_dn8) * (nv5 - 0.0));
        let eq42_e767_d_n9: f64 = ((-var_csgeff_dn9) * (nv5 - 0.0));
        let eq42_e768_q: f64 = eq42_e767;
        let eq42_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq42_e767_d_n4, eq42_e765, eq42_e767_d_n6, eq42_e767_d_n7, eq42_e767_d_n8, eq42_e767_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e770: f64 = (-var_cdgeff);
        let eq43_e772: f64 = (eq43_e770 * (nv5 - 0.0));
        let eq43_e772_d_n4: f64 = ((-var_cdgeff_dn4) * (nv5 - 0.0));
        let eq43_e772_d_n6: f64 = ((-var_cdgeff_dn6) * (nv5 - 0.0));
        let eq43_e772_d_n7: f64 = ((-var_cdgeff_dn7) * (nv5 - 0.0));
        let eq43_e772_d_n8: f64 = ((-var_cdgeff_dn8) * (nv5 - 0.0));
        let eq43_e772_d_n9: f64 = ((-var_cdgeff_dn9) * (nv5 - 0.0));
        let eq43_e773_q: f64 = eq43_e772;
        let eq43_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, eq43_e772_d_n4, eq43_e770, eq43_e772_d_n6, eq43_e772_d_n7, eq43_e772_d_n8, eq43_e772_d_n9, 0.0, 0.0, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
