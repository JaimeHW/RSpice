#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_128(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33250_e55736, assign33250_e55736_d_n0, assign33250_e55736_d_n2, assign33250_e55736_d_n3, assign33250_e55736_d_n4, assign33250_e55736_d_n5, assign33250_e55736_d_n6, assign33250_e55736_d_n7, assign33250_e55736_d_n8, assign33250_e55736_d_n9, assign33250_e55736_d_n10, assign33250_e55736_d_n11, assign33250_e55736_d_n13, assign33250_e55736_d_n14,) = {
    if ((((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign33250_e55699: f64 = (locals.var_qis + 0.5);
        let assign33250_e55702: f64 = (locals.var_qid + 0.5);
        let assign33250_e55703: f64 = (assign33250_e55699 / assign33250_e55702);
        let (assign33250_e55728, assign33250_e55728_d_n0, assign33250_e55728_d_n2, assign33250_e55728_d_n3, assign33250_e55728_d_n4, assign33250_e55728_d_n5, assign33250_e55728_d_n6, assign33250_e55728_d_n7, assign33250_e55728_d_n8, assign33250_e55728_d_n9, assign33250_e55728_d_n10, assign33250_e55728_d_n11, assign33250_e55728_d_n13, assign33250_e55728_d_n14,) = {
            if (!(assign33250_e55703 > 1e-38)) {
                let assign33250_e55708: f64 = (-87.498233534);
                (assign33250_e55708, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33250_e55711: f64 = (locals.var_qis + 0.5);
                let assign33250_e55714: f64 = (locals.var_qid + 0.5);
                let assign33250_e55715: f64 = (assign33250_e55711 / assign33250_e55714);
                let (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,) = {
                    if (assign33250_e55715 > 1e-38) {
                        let assign33250_e55720: f64 = (locals.var_qis + 0.5);
                        let assign33250_e55723: f64 = (locals.var_qid + 0.5);
                        let assign33250_e55724: f64 = (assign33250_e55720 / assign33250_e55723);
                        let assign33250_e55725: f64 = (assign33250_e55724).ln();
                        (assign33250_e55725, ((((locals.var_qis_dn0 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn0)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn2 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn2)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn3 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn3)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn4 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn4)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn5 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn5)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn6 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn6)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn7 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn7)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn8 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn8)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn9 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn9)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn10 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn10)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn11 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn11)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn13 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn13)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn14 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn14)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,)
            }
        };
        let assign33250_e55731: f64 = (locals.var_qis + locals.var_qid);
        let assign33250_e55733: f64 = (assign33250_e55731 + 1.0);
        let assign33250_e55734: f64 = (assign33250_e55728 * assign33250_e55733);
        (assign33250_e55734, ((assign33250_e55728_d_n0 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn0 + locals.var_qid_dn0))), ((assign33250_e55728_d_n2 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn2 + locals.var_qid_dn2))), ((assign33250_e55728_d_n3 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn3 + locals.var_qid_dn3))), ((assign33250_e55728_d_n4 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn4 + locals.var_qid_dn4))), ((assign33250_e55728_d_n5 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn5 + locals.var_qid_dn5))), ((assign33250_e55728_d_n6 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn6 + locals.var_qid_dn6))), ((assign33250_e55728_d_n7 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn7 + locals.var_qid_dn7))), ((assign33250_e55728_d_n8 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn8 + locals.var_qid_dn8))), ((assign33250_e55728_d_n9 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn9 + locals.var_qid_dn9))), ((assign33250_e55728_d_n10 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn10 + locals.var_qid_dn10))), ((assign33250_e55728_d_n11 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn11 + locals.var_qid_dn11))), ((assign33250_e55728_d_n13 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn13 + locals.var_qid_dn13))), ((assign33250_e55728_d_n14 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn14 + locals.var_qid_dn14))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33250_e55736;
        locals.var_t3_dn0 = assign33250_e55736_d_n0;
        locals.var_t3_dn2 = assign33250_e55736_d_n2;
        locals.var_t3_dn3 = assign33250_e55736_d_n3;
        locals.var_t3_dn4 = assign33250_e55736_d_n4;
        locals.var_t3_dn5 = assign33250_e55736_d_n5;
        locals.var_t3_dn6 = assign33250_e55736_d_n6;
        locals.var_t3_dn7 = assign33250_e55736_d_n7;
        locals.var_t3_dn8 = assign33250_e55736_d_n8;
        locals.var_t3_dn9 = assign33250_e55736_d_n9;
        locals.var_t3_dn10 = assign33250_e55736_d_n10;
        locals.var_t3_dn11 = assign33250_e55736_d_n11;
        locals.var_t3_dn13 = assign33250_e55736_d_n13;
        locals.var_t3_dn14 = assign33250_e55736_d_n14;

        let (assign33260_e55751, assign33260_e55751_d_n0, assign33260_e55751_d_n2, assign33260_e55751_d_n3, assign33260_e55751_d_n4, assign33260_e55751_d_n5, assign33260_e55751_d_n6, assign33260_e55751_d_n7, assign33260_e55751_d_n8, assign33260_e55751_d_n9, assign33260_e55751_d_n10, assign33260_e55751_d_n11, assign33260_e55751_d_n13, assign33260_e55751_d_n14,) = {
    if ((((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign33260_e55748: f64 = (locals.var_qis - locals.var_qid);
        let assign33260_e55749: f64 = (2.0 * assign33260_e55748);
        (assign33260_e55749, (2.0 * (locals.var_qis_dn0 - locals.var_qid_dn0)), (2.0 * (locals.var_qis_dn2 - locals.var_qid_dn2)), (2.0 * (locals.var_qis_dn3 - locals.var_qid_dn3)), (2.0 * (locals.var_qis_dn4 - locals.var_qid_dn4)), (2.0 * (locals.var_qis_dn5 - locals.var_qid_dn5)), (2.0 * (locals.var_qis_dn6 - locals.var_qid_dn6)), (2.0 * (locals.var_qis_dn7 - locals.var_qid_dn7)), (2.0 * (locals.var_qis_dn8 - locals.var_qid_dn8)), (2.0 * (locals.var_qis_dn9 - locals.var_qid_dn9)), (2.0 * (locals.var_qis_dn10 - locals.var_qid_dn10)), (2.0 * (locals.var_qis_dn11 - locals.var_qid_dn11)), (2.0 * (locals.var_qis_dn13 - locals.var_qid_dn13)), (2.0 * (locals.var_qis_dn14 - locals.var_qid_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33260_e55751;
        locals.var_t4_dn0 = assign33260_e55751_d_n0;
        locals.var_t4_dn2 = assign33260_e55751_d_n2;
        locals.var_t4_dn3 = assign33260_e55751_d_n3;
        locals.var_t4_dn4 = assign33260_e55751_d_n4;
        locals.var_t4_dn5 = assign33260_e55751_d_n5;
        locals.var_t4_dn6 = assign33260_e55751_d_n6;
        locals.var_t4_dn7 = assign33260_e55751_d_n7;
        locals.var_t4_dn8 = assign33260_e55751_d_n8;
        locals.var_t4_dn9 = assign33260_e55751_d_n9;
        locals.var_t4_dn10 = assign33260_e55751_d_n10;
        locals.var_t4_dn11 = assign33260_e55751_d_n11;
        locals.var_t4_dn13 = assign33260_e55751_d_n13;
        locals.var_t4_dn14 = assign33260_e55751_d_n14;

        let assign33300_e55814: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign33300_e55814;

        let assign33310_e55817: f64 = if p.p72 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign33310_e55817;

        let (assign33320_e55823, assign33320_e55823_d_n0, assign33320_e55823_d_n2, assign33320_e55823_d_n3, assign33320_e55823_d_n4, assign33320_e55823_d_n5, assign33320_e55823_d_n6, assign33320_e55823_d_n7, assign33320_e55823_d_n8, assign33320_e55823_d_n9, assign33320_e55823_d_n10, assign33320_e55823_d_n11, assign33320_e55823_d_n13, assign33320_e55823_d_n14,) = {
    if (locals.var_guard632 != 0.0) {
        let assign33320_e55821: f64 = (locals.var_ueff * locals.var_qinv);
        (assign33320_e55821, ((locals.var_ueff_dn0 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn0)), ((locals.var_ueff_dn2 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn2)), ((locals.var_ueff_dn3 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn3)), ((locals.var_ueff_dn4 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn4)), ((locals.var_ueff_dn5 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn5)), ((locals.var_ueff_dn6 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn6)), ((locals.var_ueff_dn7 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn7)), ((locals.var_ueff_dn8 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn8)), ((locals.var_ueff_dn9 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn9)), ((locals.var_ueff_dn10 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn10)), ((locals.var_ueff_dn11 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn11)), ((locals.var_ueff_dn13 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn13)), ((locals.var_ueff_dn14 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33320_e55823;
        locals.var_t0_dn0 = assign33320_e55823_d_n0;
        locals.var_t0_dn2 = assign33320_e55823_d_n2;
        locals.var_t0_dn3 = assign33320_e55823_d_n3;
        locals.var_t0_dn4 = assign33320_e55823_d_n4;
        locals.var_t0_dn5 = assign33320_e55823_d_n5;
        locals.var_t0_dn6 = assign33320_e55823_d_n6;
        locals.var_t0_dn7 = assign33320_e55823_d_n7;
        locals.var_t0_dn8 = assign33320_e55823_d_n8;
        locals.var_t0_dn9 = assign33320_e55823_d_n9;
        locals.var_t0_dn10 = assign33320_e55823_d_n10;
        locals.var_t0_dn11 = assign33320_e55823_d_n11;
        locals.var_t0_dn13 = assign33320_e55823_d_n13;
        locals.var_t0_dn14 = assign33320_e55823_d_n14;

        let (assign33330_e55833, assign33330_e55833_d_n0, assign33330_e55833_d_n2, assign33330_e55833_d_n3, assign33330_e55833_d_n4, assign33330_e55833_d_n5, assign33330_e55833_d_n6, assign33330_e55833_d_n7, assign33330_e55833_d_n8, assign33330_e55833_d_n9, assign33330_e55833_d_n10, assign33330_e55833_d_n11, assign33330_e55833_d_n13, assign33330_e55833_d_n14,) = {
    if (locals.var_guard632 != 0.0) {
        let assign33330_e55827: f64 = (locals.var_t0 * locals.var_rdsi);
        let assign33330_e55830: f64 = (locals.var_leff_1 * locals.var_leff_1);
        let assign33330_e55831: f64 = (assign33330_e55827 + assign33330_e55830);
        (assign33330_e55831, (((locals.var_t0_dn0 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn0)) + ((locals.var_leff_1_dn0 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn0))), (((locals.var_t0_dn2 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn2)) + ((locals.var_leff_1_dn2 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn2))), (((locals.var_t0_dn3 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn3)) + ((locals.var_leff_1_dn3 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn3))), (((locals.var_t0_dn4 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn4)) + ((locals.var_leff_1_dn4 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn4))), (((locals.var_t0_dn5 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn5)) + ((locals.var_leff_1_dn5 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn5))), (((locals.var_t0_dn6 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn6)) + ((locals.var_leff_1_dn6 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn6))), (((locals.var_t0_dn7 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn7)) + ((locals.var_leff_1_dn7 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn7))), (((locals.var_t0_dn8 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn8)) + ((locals.var_leff_1_dn8 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn8))), (((locals.var_t0_dn9 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn9)) + ((locals.var_leff_1_dn9 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn9))), (((locals.var_t0_dn10 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn10)) + ((locals.var_leff_1_dn10 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn10))), (((locals.var_t0_dn11 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn11)) + ((locals.var_leff_1_dn11 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn11))), (((locals.var_t0_dn13 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn13)) + ((locals.var_leff_1_dn13 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn13))), (((locals.var_t0_dn14 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn14)) + ((locals.var_leff_1_dn14 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33330_e55833;
        locals.var_t1_dn0 = assign33330_e55833_d_n0;
        locals.var_t1_dn2 = assign33330_e55833_d_n2;
        locals.var_t1_dn3 = assign33330_e55833_d_n3;
        locals.var_t1_dn4 = assign33330_e55833_d_n4;
        locals.var_t1_dn5 = assign33330_e55833_d_n5;
        locals.var_t1_dn6 = assign33330_e55833_d_n6;
        locals.var_t1_dn7 = assign33330_e55833_d_n7;
        locals.var_t1_dn8 = assign33330_e55833_d_n8;
        locals.var_t1_dn9 = assign33330_e55833_d_n9;
        locals.var_t1_dn10 = assign33330_e55833_d_n10;
        locals.var_t1_dn11 = assign33330_e55833_d_n11;
        locals.var_t1_dn13 = assign33330_e55833_d_n13;
        locals.var_t1_dn14 = assign33330_e55833_d_n14;

        let (assign33360_e55860, assign33360_e55860_d_n0, assign33360_e55860_d_n2, assign33360_e55860_d_n3, assign33360_e55860_d_n4, assign33360_e55860_d_n5, assign33360_e55860_d_n6, assign33360_e55860_d_n7, assign33360_e55860_d_n8, assign33360_e55860_d_n9, assign33360_e55860_d_n10, assign33360_e55860_d_n11, assign33360_e55860_d_n13, assign33360_e55860_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33360_e55858: f64 = (locals.var_qia / locals.var_esatl);
        (assign33360_e55858, (((locals.var_qia_dn0 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn0)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn2 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn2)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn3 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn4 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn5 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn6 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn7 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn8 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn9 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn10 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn11 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn13 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn13)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn14 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn14)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33360_e55860;
        locals.var_t0_dn0 = assign33360_e55860_d_n0;
        locals.var_t0_dn2 = assign33360_e55860_d_n2;
        locals.var_t0_dn3 = assign33360_e55860_d_n3;
        locals.var_t0_dn4 = assign33360_e55860_d_n4;
        locals.var_t0_dn5 = assign33360_e55860_d_n5;
        locals.var_t0_dn6 = assign33360_e55860_d_n6;
        locals.var_t0_dn7 = assign33360_e55860_d_n7;
        locals.var_t0_dn8 = assign33360_e55860_d_n8;
        locals.var_t0_dn9 = assign33360_e55860_d_n9;
        locals.var_t0_dn10 = assign33360_e55860_d_n10;
        locals.var_t0_dn11 = assign33360_e55860_d_n11;
        locals.var_t0_dn13 = assign33360_e55860_d_n13;
        locals.var_t0_dn14 = assign33360_e55860_d_n14;

        let (assign33370_e55869, assign33370_e55869_d_n0, assign33370_e55869_d_n2, assign33370_e55869_d_n3, assign33370_e55869_d_n4, assign33370_e55869_d_n5, assign33370_e55869_d_n6, assign33370_e55869_d_n7, assign33370_e55869_d_n8, assign33370_e55869_d_n9, assign33370_e55869_d_n10, assign33370_e55869_d_n11, assign33370_e55869_d_n13, assign33370_e55869_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33370_e55867: f64 = (locals.var_t0 * locals.var_t0);
        (assign33370_e55867, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33370_e55869;
        locals.var_t0_dn0 = assign33370_e55869_d_n0;
        locals.var_t0_dn2 = assign33370_e55869_d_n2;
        locals.var_t0_dn3 = assign33370_e55869_d_n3;
        locals.var_t0_dn4 = assign33370_e55869_d_n4;
        locals.var_t0_dn5 = assign33370_e55869_d_n5;
        locals.var_t0_dn6 = assign33370_e55869_d_n6;
        locals.var_t0_dn7 = assign33370_e55869_d_n7;
        locals.var_t0_dn8 = assign33370_e55869_d_n8;
        locals.var_t0_dn9 = assign33370_e55869_d_n9;
        locals.var_t0_dn10 = assign33370_e55869_d_n10;
        locals.var_t0_dn11 = assign33370_e55869_d_n11;
        locals.var_t0_dn13 = assign33370_e55869_d_n13;
        locals.var_t0_dn14 = assign33370_e55869_d_n14;

        let (assign33380_e55884, assign33380_e55884_d_n0, assign33380_e55884_d_n2, assign33380_e55884_d_n3, assign33380_e55884_d_n4, assign33380_e55884_d_n5, assign33380_e55884_d_n6, assign33380_e55884_d_n7, assign33380_e55884_d_n8, assign33380_e55884_d_n9, assign33380_e55884_d_n10, assign33380_e55884_d_n11, assign33380_e55884_d_n13, assign33380_e55884_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33380_e55878: f64 = (locals.var_t0 * p.p1709);
        let assign33380_e55880: f64 = (assign33380_e55878 * locals.var_leff_1);
        let assign33380_e55881: f64 = (1.0 + assign33380_e55880);
        let assign33380_e55882: f64 = (p.p1708 * assign33380_e55881);
        (assign33380_e55882, (p.p1708 * (((locals.var_t0_dn0 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn0))), (p.p1708 * (((locals.var_t0_dn2 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn2))), (p.p1708 * (((locals.var_t0_dn3 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn3))), (p.p1708 * (((locals.var_t0_dn4 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn4))), (p.p1708 * (((locals.var_t0_dn5 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn5))), (p.p1708 * (((locals.var_t0_dn6 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn6))), (p.p1708 * (((locals.var_t0_dn7 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn7))), (p.p1708 * (((locals.var_t0_dn8 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn8))), (p.p1708 * (((locals.var_t0_dn9 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn9))), (p.p1708 * (((locals.var_t0_dn10 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn10))), (p.p1708 * (((locals.var_t0_dn11 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn11))), (p.p1708 * (((locals.var_t0_dn13 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn13))), (p.p1708 * (((locals.var_t0_dn14 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noibeta, locals.var_noibeta_dn0, locals.var_noibeta_dn2, locals.var_noibeta_dn3, locals.var_noibeta_dn4, locals.var_noibeta_dn5, locals.var_noibeta_dn6, locals.var_noibeta_dn7, locals.var_noibeta_dn8, locals.var_noibeta_dn9, locals.var_noibeta_dn10, locals.var_noibeta_dn11, locals.var_noibeta_dn13, locals.var_noibeta_dn14,)
    }
};
        locals.var_noibeta = assign33380_e55884;
        locals.var_noibeta_dn0 = assign33380_e55884_d_n0;
        locals.var_noibeta_dn2 = assign33380_e55884_d_n2;
        locals.var_noibeta_dn3 = assign33380_e55884_d_n3;
        locals.var_noibeta_dn4 = assign33380_e55884_d_n4;
        locals.var_noibeta_dn5 = assign33380_e55884_d_n5;
        locals.var_noibeta_dn6 = assign33380_e55884_d_n6;
        locals.var_noibeta_dn7 = assign33380_e55884_d_n7;
        locals.var_noibeta_dn8 = assign33380_e55884_d_n8;
        locals.var_noibeta_dn9 = assign33380_e55884_d_n9;
        locals.var_noibeta_dn10 = assign33380_e55884_d_n10;
        locals.var_noibeta_dn11 = assign33380_e55884_d_n11;
        locals.var_noibeta_dn13 = assign33380_e55884_d_n13;
        locals.var_noibeta_dn14 = assign33380_e55884_d_n14;

        let (assign33390_e55899, assign33390_e55899_d_n0, assign33390_e55899_d_n2, assign33390_e55899_d_n3, assign33390_e55899_d_n4, assign33390_e55899_d_n5, assign33390_e55899_d_n6, assign33390_e55899_d_n7, assign33390_e55899_d_n8, assign33390_e55899_d_n9, assign33390_e55899_d_n10, assign33390_e55899_d_n11, assign33390_e55899_d_n13, assign33390_e55899_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33390_e55893: f64 = (locals.var_t0 * p.p1711);
        let assign33390_e55895: f64 = (assign33390_e55893 * locals.var_leff_1);
        let assign33390_e55896: f64 = (1.0 + assign33390_e55895);
        let assign33390_e55897: f64 = (p.p1710 * assign33390_e55896);
        (assign33390_e55897, (p.p1710 * (((locals.var_t0_dn0 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn0))), (p.p1710 * (((locals.var_t0_dn2 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn2))), (p.p1710 * (((locals.var_t0_dn3 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn3))), (p.p1710 * (((locals.var_t0_dn4 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn4))), (p.p1710 * (((locals.var_t0_dn5 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn5))), (p.p1710 * (((locals.var_t0_dn6 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn6))), (p.p1710 * (((locals.var_t0_dn7 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn7))), (p.p1710 * (((locals.var_t0_dn8 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn8))), (p.p1710 * (((locals.var_t0_dn9 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn9))), (p.p1710 * (((locals.var_t0_dn10 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn10))), (p.p1710 * (((locals.var_t0_dn11 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn11))), (p.p1710 * (((locals.var_t0_dn13 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn13))), (p.p1710 * (((locals.var_t0_dn14 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noitheta, locals.var_noitheta_dn0, locals.var_noitheta_dn2, locals.var_noitheta_dn3, locals.var_noitheta_dn4, locals.var_noitheta_dn5, locals.var_noitheta_dn6, locals.var_noitheta_dn7, locals.var_noitheta_dn8, locals.var_noitheta_dn9, locals.var_noitheta_dn10, locals.var_noitheta_dn11, locals.var_noitheta_dn13, locals.var_noitheta_dn14,)
    }
};
        locals.var_noitheta = assign33390_e55899;
        locals.var_noitheta_dn0 = assign33390_e55899_d_n0;
        locals.var_noitheta_dn2 = assign33390_e55899_d_n2;
        locals.var_noitheta_dn3 = assign33390_e55899_d_n3;
        locals.var_noitheta_dn4 = assign33390_e55899_d_n4;
        locals.var_noitheta_dn5 = assign33390_e55899_d_n5;
        locals.var_noitheta_dn6 = assign33390_e55899_d_n6;
        locals.var_noitheta_dn7 = assign33390_e55899_d_n7;
        locals.var_noitheta_dn8 = assign33390_e55899_d_n8;
        locals.var_noitheta_dn9 = assign33390_e55899_d_n9;
        locals.var_noitheta_dn10 = assign33390_e55899_d_n10;
        locals.var_noitheta_dn11 = assign33390_e55899_d_n11;
        locals.var_noitheta_dn13 = assign33390_e55899_d_n13;
        locals.var_noitheta_dn14 = assign33390_e55899_d_n14;

        let (assign33400_e55914, assign33400_e55914_d_n0, assign33400_e55914_d_n2, assign33400_e55914_d_n3, assign33400_e55914_d_n4, assign33400_e55914_d_n5, assign33400_e55914_d_n6, assign33400_e55914_d_n7, assign33400_e55914_d_n8, assign33400_e55914_d_n9, assign33400_e55914_d_n10, assign33400_e55914_d_n11, assign33400_e55914_d_n13, assign33400_e55914_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33400_e55908: f64 = (locals.var_t0 * p.p1713);
        let assign33400_e55910: f64 = (assign33400_e55908 * locals.var_leff_1);
        let assign33400_e55911: f64 = (1.0 + assign33400_e55910);
        let assign33400_e55912: f64 = (p.p1712 * assign33400_e55911);
        (assign33400_e55912, (p.p1712 * (((locals.var_t0_dn0 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn0))), (p.p1712 * (((locals.var_t0_dn2 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn2))), (p.p1712 * (((locals.var_t0_dn3 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn3))), (p.p1712 * (((locals.var_t0_dn4 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn4))), (p.p1712 * (((locals.var_t0_dn5 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn5))), (p.p1712 * (((locals.var_t0_dn6 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn6))), (p.p1712 * (((locals.var_t0_dn7 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn7))), (p.p1712 * (((locals.var_t0_dn8 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn8))), (p.p1712 * (((locals.var_t0_dn9 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn9))), (p.p1712 * (((locals.var_t0_dn10 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn10))), (p.p1712 * (((locals.var_t0_dn11 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn11))), (p.p1712 * (((locals.var_t0_dn13 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn13))), (p.p1712 * (((locals.var_t0_dn14 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noicorr, locals.var_noicorr_dn0, locals.var_noicorr_dn2, locals.var_noicorr_dn3, locals.var_noicorr_dn4, locals.var_noicorr_dn5, locals.var_noicorr_dn6, locals.var_noicorr_dn7, locals.var_noicorr_dn8, locals.var_noicorr_dn9, locals.var_noicorr_dn10, locals.var_noicorr_dn11, locals.var_noicorr_dn13, locals.var_noicorr_dn14,)
    }
};
        locals.var_noicorr = assign33400_e55914;
        locals.var_noicorr_dn0 = assign33400_e55914_d_n0;
        locals.var_noicorr_dn2 = assign33400_e55914_d_n2;
        locals.var_noicorr_dn3 = assign33400_e55914_d_n3;
        locals.var_noicorr_dn4 = assign33400_e55914_d_n4;
        locals.var_noicorr_dn5 = assign33400_e55914_d_n5;
        locals.var_noicorr_dn6 = assign33400_e55914_d_n6;
        locals.var_noicorr_dn7 = assign33400_e55914_d_n7;
        locals.var_noicorr_dn8 = assign33400_e55914_d_n8;
        locals.var_noicorr_dn9 = assign33400_e55914_d_n9;
        locals.var_noicorr_dn10 = assign33400_e55914_d_n10;
        locals.var_noicorr_dn11 = assign33400_e55914_d_n11;
        locals.var_noicorr_dn13 = assign33400_e55914_d_n13;
        locals.var_noicorr_dn14 = assign33400_e55914_d_n14;

        let (assign33410_e55929, assign33410_e55929_d_n0, assign33410_e55929_d_n2, assign33410_e55929_d_n3, assign33410_e55929_d_n4, assign33410_e55929_d_n5, assign33410_e55929_d_n6, assign33410_e55929_d_n7, assign33410_e55929_d_n8, assign33410_e55929_d_n9, assign33410_e55929_d_n10, assign33410_e55929_d_n11, assign33410_e55929_d_n13, assign33410_e55929_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33410_e55923: f64 = (locals.var_t0 * p.p1715);
        let assign33410_e55925: f64 = (assign33410_e55923 * locals.var_leff_1);
        let assign33410_e55926: f64 = (1.0 + assign33410_e55925);
        let assign33410_e55927: f64 = (p.p1714 * assign33410_e55926);
        (assign33410_e55927, (p.p1714 * (((locals.var_t0_dn0 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn0))), (p.p1714 * (((locals.var_t0_dn2 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn2))), (p.p1714 * (((locals.var_t0_dn3 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn3))), (p.p1714 * (((locals.var_t0_dn4 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn4))), (p.p1714 * (((locals.var_t0_dn5 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn5))), (p.p1714 * (((locals.var_t0_dn6 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn6))), (p.p1714 * (((locals.var_t0_dn7 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn7))), (p.p1714 * (((locals.var_t0_dn8 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn8))), (p.p1714 * (((locals.var_t0_dn9 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn9))), (p.p1714 * (((locals.var_t0_dn10 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn10))), (p.p1714 * (((locals.var_t0_dn11 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn11))), (p.p1714 * (((locals.var_t0_dn13 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn13))), (p.p1714 * (((locals.var_t0_dn14 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noilowid, locals.var_noilowid_dn0, locals.var_noilowid_dn2, locals.var_noilowid_dn3, locals.var_noilowid_dn4, locals.var_noilowid_dn5, locals.var_noilowid_dn6, locals.var_noilowid_dn7, locals.var_noilowid_dn8, locals.var_noilowid_dn9, locals.var_noilowid_dn10, locals.var_noilowid_dn11, locals.var_noilowid_dn13, locals.var_noilowid_dn14,)
    }
};
        locals.var_noilowid = assign33410_e55929;
        locals.var_noilowid_dn0 = assign33410_e55929_d_n0;
        locals.var_noilowid_dn2 = assign33410_e55929_d_n2;
        locals.var_noilowid_dn3 = assign33410_e55929_d_n3;
        locals.var_noilowid_dn4 = assign33410_e55929_d_n4;
        locals.var_noilowid_dn5 = assign33410_e55929_d_n5;
        locals.var_noilowid_dn6 = assign33410_e55929_d_n6;
        locals.var_noilowid_dn7 = assign33410_e55929_d_n7;
        locals.var_noilowid_dn8 = assign33410_e55929_d_n8;
        locals.var_noilowid_dn9 = assign33410_e55929_d_n9;
        locals.var_noilowid_dn10 = assign33410_e55929_d_n10;
        locals.var_noilowid_dn11 = assign33410_e55929_d_n11;
        locals.var_noilowid_dn13 = assign33410_e55929_d_n13;
        locals.var_noilowid_dn14 = assign33410_e55929_d_n14;

        let (assign33420_e55940, assign33420_e55940_d_n0, assign33420_e55940_d_n2, assign33420_e55940_d_n3, assign33420_e55940_d_n4, assign33420_e55940_d_n5, assign33420_e55940_d_n6, assign33420_e55940_d_n7, assign33420_e55940_d_n8, assign33420_e55940_d_n9, assign33420_e55940_d_n10, assign33420_e55940_d_n11, assign33420_e55940_d_n13, assign33420_e55940_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33420_e55936: f64 = (3.0 * locals.var_noibeta);
        let assign33420_e55938: f64 = (assign33420_e55936 * locals.var_noibeta);
        (assign33420_e55938, (((3.0 * locals.var_noibeta_dn0) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn0)), (((3.0 * locals.var_noibeta_dn2) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn2)), (((3.0 * locals.var_noibeta_dn3) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn3)), (((3.0 * locals.var_noibeta_dn4) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn4)), (((3.0 * locals.var_noibeta_dn5) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn5)), (((3.0 * locals.var_noibeta_dn6) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn6)), (((3.0 * locals.var_noibeta_dn7) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn7)), (((3.0 * locals.var_noibeta_dn8) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn8)), (((3.0 * locals.var_noibeta_dn9) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn9)), (((3.0 * locals.var_noibeta_dn10) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn10)), (((3.0 * locals.var_noibeta_dn11) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn11)), (((3.0 * locals.var_noibeta_dn13) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn13)), (((3.0 * locals.var_noibeta_dn14) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33420_e55940;
        locals.var_t1_dn0 = assign33420_e55940_d_n0;
        locals.var_t1_dn2 = assign33420_e55940_d_n2;
        locals.var_t1_dn3 = assign33420_e55940_d_n3;
        locals.var_t1_dn4 = assign33420_e55940_d_n4;
        locals.var_t1_dn5 = assign33420_e55940_d_n5;
        locals.var_t1_dn6 = assign33420_e55940_d_n6;
        locals.var_t1_dn7 = assign33420_e55940_d_n7;
        locals.var_t1_dn8 = assign33420_e55940_d_n8;
        locals.var_t1_dn9 = assign33420_e55940_d_n9;
        locals.var_t1_dn10 = assign33420_e55940_d_n10;
        locals.var_t1_dn11 = assign33420_e55940_d_n11;
        locals.var_t1_dn13 = assign33420_e55940_d_n13;
        locals.var_t1_dn14 = assign33420_e55940_d_n14;

        let (assign33430_e55951, assign33430_e55951_d_n0, assign33430_e55951_d_n2, assign33430_e55951_d_n3, assign33430_e55951_d_n4, assign33430_e55951_d_n5, assign33430_e55951_d_n6, assign33430_e55951_d_n7, assign33430_e55951_d_n8, assign33430_e55951_d_n9, assign33430_e55951_d_n10, assign33430_e55951_d_n11, assign33430_e55951_d_n13, assign33430_e55951_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33430_e55947: f64 = (7.5 * locals.var_noitheta);
        let assign33430_e55949: f64 = (assign33430_e55947 * locals.var_noitheta);
        (assign33430_e55949, (((7.5 * locals.var_noitheta_dn0) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn0)), (((7.5 * locals.var_noitheta_dn2) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn2)), (((7.5 * locals.var_noitheta_dn3) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn3)), (((7.5 * locals.var_noitheta_dn4) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn4)), (((7.5 * locals.var_noitheta_dn5) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn5)), (((7.5 * locals.var_noitheta_dn6) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn6)), (((7.5 * locals.var_noitheta_dn7) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn7)), (((7.5 * locals.var_noitheta_dn8) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn8)), (((7.5 * locals.var_noitheta_dn9) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn9)), (((7.5 * locals.var_noitheta_dn10) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn10)), (((7.5 * locals.var_noitheta_dn11) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn11)), (((7.5 * locals.var_noitheta_dn13) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn13)), (((7.5 * locals.var_noitheta_dn14) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33430_e55951;
        locals.var_t2_dn0 = assign33430_e55951_d_n0;
        locals.var_t2_dn2 = assign33430_e55951_d_n2;
        locals.var_t2_dn3 = assign33430_e55951_d_n3;
        locals.var_t2_dn4 = assign33430_e55951_d_n4;
        locals.var_t2_dn5 = assign33430_e55951_d_n5;
        locals.var_t2_dn6 = assign33430_e55951_d_n6;
        locals.var_t2_dn7 = assign33430_e55951_d_n7;
        locals.var_t2_dn8 = assign33430_e55951_d_n8;
        locals.var_t2_dn9 = assign33430_e55951_d_n9;
        locals.var_t2_dn10 = assign33430_e55951_d_n10;
        locals.var_t2_dn11 = assign33430_e55951_d_n11;
        locals.var_t2_dn13 = assign33430_e55951_d_n13;
        locals.var_t2_dn14 = assign33430_e55951_d_n14;

        let (assign33440_e55960, assign33440_e55960_d_n0, assign33440_e55960_d_n2, assign33440_e55960_d_n3, assign33440_e55960_d_n4, assign33440_e55960_d_n5, assign33440_e55960_d_n6, assign33440_e55960_d_n7, assign33440_e55960_d_n8, assign33440_e55960_d_n9, assign33440_e55960_d_n10, assign33440_e55960_d_n11, assign33440_e55960_d_n13, assign33440_e55960_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33440_e55958: f64 = (2.5298 * locals.var_noicorr);
        (assign33440_e55958, (2.5298 * locals.var_noicorr_dn0), (2.5298 * locals.var_noicorr_dn2), (2.5298 * locals.var_noicorr_dn3), (2.5298 * locals.var_noicorr_dn4), (2.5298 * locals.var_noicorr_dn5), (2.5298 * locals.var_noicorr_dn6), (2.5298 * locals.var_noicorr_dn7), (2.5298 * locals.var_noicorr_dn8), (2.5298 * locals.var_noicorr_dn9), (2.5298 * locals.var_noicorr_dn10), (2.5298 * locals.var_noicorr_dn11), (2.5298 * locals.var_noicorr_dn13), (2.5298 * locals.var_noicorr_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33440_e55960;
        locals.var_t3_dn0 = assign33440_e55960_d_n0;
        locals.var_t3_dn2 = assign33440_e55960_d_n2;
        locals.var_t3_dn3 = assign33440_e55960_d_n3;
        locals.var_t3_dn4 = assign33440_e55960_d_n4;
        locals.var_t3_dn5 = assign33440_e55960_d_n5;
        locals.var_t3_dn6 = assign33440_e55960_d_n6;
        locals.var_t3_dn7 = assign33440_e55960_d_n7;
        locals.var_t3_dn8 = assign33440_e55960_d_n8;
        locals.var_t3_dn9 = assign33440_e55960_d_n9;
        locals.var_t3_dn10 = assign33440_e55960_d_n10;
        locals.var_t3_dn11 = assign33440_e55960_d_n11;
        locals.var_t3_dn13 = assign33440_e55960_d_n13;
        locals.var_t3_dn14 = assign33440_e55960_d_n14;

        let (assign33450_e55975, assign33450_e55975_d_n0, assign33450_e55975_d_n2, assign33450_e55975_d_n3, assign33450_e55975_d_n4, assign33450_e55975_d_n5, assign33450_e55975_d_n6, assign33450_e55975_d_n7, assign33450_e55975_d_n8, assign33450_e55975_d_n9, assign33450_e55975_d_n10, assign33450_e55975_d_n11, assign33450_e55975_d_n13, assign33450_e55975_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33450_e55967: f64 = (locals.var_qid / locals.var_qis);
        let assign33450_e55971: f64 = (locals.var_vdseff_1 / locals.var_vdsat);
        let assign33450_e55972: f64 = (1.0 - assign33450_e55971);
        let assign33450_e55973: f64 = (assign33450_e55967 * assign33450_e55972);
        (assign33450_e55973, (((((locals.var_qid_dn0 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn0)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn0 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn0)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn2 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn2)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn2 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn2)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn3 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn3)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn3 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn3)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn4 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn4 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn4)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn5 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn5)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn5 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn6 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn6 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn7 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn7 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn7)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn8 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn8 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn8)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn9 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn9 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn9)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn10 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn10)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn10 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn10)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn11 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn11)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn11 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn11)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn13 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn13)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn13 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn13)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn14 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn14)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn14 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn14)) / (locals.var_vdsat * locals.var_vdsat))))),)
    } else {
        (locals.var_noieta, locals.var_noieta_dn0, locals.var_noieta_dn2, locals.var_noieta_dn3, locals.var_noieta_dn4, locals.var_noieta_dn5, locals.var_noieta_dn6, locals.var_noieta_dn7, locals.var_noieta_dn8, locals.var_noieta_dn9, locals.var_noieta_dn10, locals.var_noieta_dn11, locals.var_noieta_dn13, locals.var_noieta_dn14,)
    }
};
        locals.var_noieta = assign33450_e55975;
        locals.var_noieta_dn0 = assign33450_e55975_d_n0;
        locals.var_noieta_dn2 = assign33450_e55975_d_n2;
        locals.var_noieta_dn3 = assign33450_e55975_d_n3;
        locals.var_noieta_dn4 = assign33450_e55975_d_n4;
        locals.var_noieta_dn5 = assign33450_e55975_d_n5;
        locals.var_noieta_dn6 = assign33450_e55975_d_n6;
        locals.var_noieta_dn7 = assign33450_e55975_d_n7;
        locals.var_noieta_dn8 = assign33450_e55975_d_n8;
        locals.var_noieta_dn9 = assign33450_e55975_d_n9;
        locals.var_noieta_dn10 = assign33450_e55975_d_n10;
        locals.var_noieta_dn11 = assign33450_e55975_d_n11;
        locals.var_noieta_dn13 = assign33450_e55975_d_n13;
        locals.var_noieta_dn14 = assign33450_e55975_d_n14;

        let (assign33460_e55986, assign33460_e55986_d_n0, assign33460_e55986_d_n2, assign33460_e55986_d_n3, assign33460_e55986_d_n4, assign33460_e55986_d_n5, assign33460_e55986_d_n6, assign33460_e55986_d_n7, assign33460_e55986_d_n8, assign33460_e55986_d_n9, assign33460_e55986_d_n10, assign33460_e55986_d_n11, assign33460_e55986_d_n13, assign33460_e55986_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33460_e55982: f64 = (locals.var_dvsat * locals.var_dvsat);
        let assign33460_e55984: f64 = (assign33460_e55982 * locals.var_dvsat);
        (assign33460_e55984, ((((locals.var_dvsat_dn0 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn0)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn0)), ((((locals.var_dvsat_dn2 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn2)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn2)), ((((locals.var_dvsat_dn3 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn3)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn3)), ((((locals.var_dvsat_dn4 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn4)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn4)), ((((locals.var_dvsat_dn5 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn5)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn5)), ((((locals.var_dvsat_dn6 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn6)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn6)), ((((locals.var_dvsat_dn7 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn7)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn7)), ((((locals.var_dvsat_dn8 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn8)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn8)), ((((locals.var_dvsat_dn9 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn9)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn9)), ((((locals.var_dvsat_dn10 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn10)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn10)), ((((locals.var_dvsat_dn11 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn11)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn11)), ((((locals.var_dvsat_dn13 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn13)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn13)), ((((locals.var_dvsat_dn14 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn14)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn14)),)
    } else {
        (locals.var_dvsat3, locals.var_dvsat3_dn0, locals.var_dvsat3_dn2, locals.var_dvsat3_dn3, locals.var_dvsat3_dn4, locals.var_dvsat3_dn5, locals.var_dvsat3_dn6, locals.var_dvsat3_dn7, locals.var_dvsat3_dn8, locals.var_dvsat3_dn9, locals.var_dvsat3_dn10, locals.var_dvsat3_dn11, locals.var_dvsat3_dn13, locals.var_dvsat3_dn14,)
    }
};
        locals.var_dvsat3 = assign33460_e55986;
        locals.var_dvsat3_dn0 = assign33460_e55986_d_n0;
        locals.var_dvsat3_dn2 = assign33460_e55986_d_n2;
        locals.var_dvsat3_dn3 = assign33460_e55986_d_n3;
        locals.var_dvsat3_dn4 = assign33460_e55986_d_n4;
        locals.var_dvsat3_dn5 = assign33460_e55986_d_n5;
        locals.var_dvsat3_dn6 = assign33460_e55986_d_n6;
        locals.var_dvsat3_dn7 = assign33460_e55986_d_n7;
        locals.var_dvsat3_dn8 = assign33460_e55986_d_n8;
        locals.var_dvsat3_dn9 = assign33460_e55986_d_n9;
        locals.var_dvsat3_dn10 = assign33460_e55986_d_n10;
        locals.var_dvsat3_dn11 = assign33460_e55986_d_n11;
        locals.var_dvsat3_dn13 = assign33460_e55986_d_n13;
        locals.var_dvsat3_dn14 = assign33460_e55986_d_n14;

        let (assign33470_e55997, assign33470_e55997_d_n0, assign33470_e55997_d_n2, assign33470_e55997_d_n3, assign33470_e55997_d_n4, assign33470_e55997_d_n5, assign33470_e55997_d_n6, assign33470_e55997_d_n7, assign33470_e55997_d_n8, assign33470_e55997_d_n9, assign33470_e55997_d_n10, assign33470_e55997_d_n11, assign33470_e55997_d_n13, assign33470_e55997_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33470_e55994: f64 = (locals.var_q0 + locals.var_qia);
        let assign33470_e55995: f64 = (locals.var_q0 / assign33470_e55994);
        (assign33470_e55995, (((locals.var_q0_dn0 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn0 + locals.var_qia_dn0))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn2 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn2 + locals.var_qia_dn2))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn3 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn3 + locals.var_qia_dn3))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn4 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn4 + locals.var_qia_dn4))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn5 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn5 + locals.var_qia_dn5))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn6 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn6 + locals.var_qia_dn6))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn7 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn7 + locals.var_qia_dn7))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn8 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn8 + locals.var_qia_dn8))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn9 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn9 + locals.var_qia_dn9))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn10 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn10 + locals.var_qia_dn10))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn11 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn11 + locals.var_qia_dn11))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn13 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn13 + locals.var_qia_dn13))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn14 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn14 + locals.var_qia_dn14))) / (assign33470_e55994 * assign33470_e55994)),)
    } else {
        (locals.var_noiwi, locals.var_noiwi_dn0, locals.var_noiwi_dn2, locals.var_noiwi_dn3, locals.var_noiwi_dn4, locals.var_noiwi_dn5, locals.var_noiwi_dn6, locals.var_noiwi_dn7, locals.var_noiwi_dn8, locals.var_noiwi_dn9, locals.var_noiwi_dn10, locals.var_noiwi_dn11, locals.var_noiwi_dn13, locals.var_noiwi_dn14,)
    }
};
        locals.var_noiwi = assign33470_e55997;
        locals.var_noiwi_dn0 = assign33470_e55997_d_n0;
        locals.var_noiwi_dn2 = assign33470_e55997_d_n2;
        locals.var_noiwi_dn3 = assign33470_e55997_d_n3;
        locals.var_noiwi_dn4 = assign33470_e55997_d_n4;
        locals.var_noiwi_dn5 = assign33470_e55997_d_n5;
        locals.var_noiwi_dn6 = assign33470_e55997_d_n6;
        locals.var_noiwi_dn7 = assign33470_e55997_d_n7;
        locals.var_noiwi_dn8 = assign33470_e55997_d_n8;
        locals.var_noiwi_dn9 = assign33470_e55997_d_n9;
        locals.var_noiwi_dn10 = assign33470_e55997_d_n10;
        locals.var_noiwi_dn11 = assign33470_e55997_d_n11;
        locals.var_noiwi_dn13 = assign33470_e55997_d_n13;
        locals.var_noiwi_dn14 = assign33470_e55997_d_n14;

        let (assign33480_e56014, assign33480_e56014_d_n0, assign33480_e56014_d_n2, assign33480_e56014_d_n3, assign33480_e56014_d_n4, assign33480_e56014_d_n5, assign33480_e56014_d_n6, assign33480_e56014_d_n7, assign33480_e56014_d_n8, assign33480_e56014_d_n9, assign33480_e56014_d_n10, assign33480_e56014_d_n11, assign33480_e56014_d_n13, assign33480_e56014_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33480_e56005: f64 = (0.0_f64).max(locals.var_k0si_t);
        let assign33480_e56007: f64 = (assign33480_e56005 * locals.var_qis);
        let assign33480_e56010: f64 = (2.0 * locals.var_nvtm);
        let assign33480_e56011: f64 = (assign33480_e56007 + assign33480_e56010);
        let assign33480_e56012: f64 = (locals.var_k0_t / assign33480_e56011);
        (assign33480_e56012, (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn0) + (2.0 * locals.var_nvtm_dn0))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn2) + (2.0 * locals.var_nvtm_dn2))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn3) + (2.0 * locals.var_nvtm_dn3))) / (assign33480_e56011 * assign33480_e56011))), (((locals.var_k0_t_dn4 * assign33480_e56011) - (locals.var_k0_t * (((if 0.0 >= locals.var_k0si_t { 0.0 } else { locals.var_k0si_t_dn4 } * locals.var_qis) + (assign33480_e56005 * locals.var_qis_dn4)) + (2.0 * locals.var_nvtm_dn4)))) / (assign33480_e56011 * assign33480_e56011)), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn5) + (2.0 * locals.var_nvtm_dn5))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn6) + (2.0 * locals.var_nvtm_dn6))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn7) + (2.0 * locals.var_nvtm_dn7))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn8) + (2.0 * locals.var_nvtm_dn8))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn9) + (2.0 * locals.var_nvtm_dn9))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn10) + (2.0 * locals.var_nvtm_dn10))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn11) + (2.0 * locals.var_nvtm_dn11))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn13) + (2.0 * locals.var_nvtm_dn13))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn14) + (2.0 * locals.var_nvtm_dn14))) / (assign33480_e56011 * assign33480_e56011))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33480_e56014;
        locals.var_t4_dn0 = assign33480_e56014_d_n0;
        locals.var_t4_dn2 = assign33480_e56014_d_n2;
        locals.var_t4_dn3 = assign33480_e56014_d_n3;
        locals.var_t4_dn4 = assign33480_e56014_d_n4;
        locals.var_t4_dn5 = assign33480_e56014_d_n5;
        locals.var_t4_dn6 = assign33480_e56014_d_n6;
        locals.var_t4_dn7 = assign33480_e56014_d_n7;
        locals.var_t4_dn8 = assign33480_e56014_d_n8;
        locals.var_t4_dn9 = assign33480_e56014_d_n9;
        locals.var_t4_dn10 = assign33480_e56014_d_n10;
        locals.var_t4_dn11 = assign33480_e56014_d_n11;
        locals.var_t4_dn13 = assign33480_e56014_d_n13;
        locals.var_t4_dn14 = assign33480_e56014_d_n14;

        let (assign33490_e56023, assign33490_e56023_d_n0, assign33490_e56023_d_n2, assign33490_e56023_d_n3, assign33490_e56023_d_n4, assign33490_e56023_d_n5, assign33490_e56023_d_n6, assign33490_e56023_d_n7, assign33490_e56023_d_n8, assign33490_e56023_d_n9, assign33490_e56023_d_n10, assign33490_e56023_d_n11, assign33490_e56023_d_n13, assign33490_e56023_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33490_e56020: f64 = (-locals.var_t4);
        let assign33490_e56021: f64 = { let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33490_e56021, ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn0)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn2)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn3)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn4)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn5)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn6)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn7)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn8)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn9)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn10)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn11)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn13)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn14)),)
    } else {
        (locals.var_mnud0, locals.var_mnud0_dn0, locals.var_mnud0_dn2, locals.var_mnud0_dn3, locals.var_mnud0_dn4, locals.var_mnud0_dn5, locals.var_mnud0_dn6, locals.var_mnud0_dn7, locals.var_mnud0_dn8, locals.var_mnud0_dn9, locals.var_mnud0_dn10, locals.var_mnud0_dn11, locals.var_mnud0_dn13, locals.var_mnud0_dn14,)
    }
};
        locals.var_mnud0 = assign33490_e56023;
        locals.var_mnud0_dn0 = assign33490_e56023_d_n0;
        locals.var_mnud0_dn2 = assign33490_e56023_d_n2;
        locals.var_mnud0_dn3 = assign33490_e56023_d_n3;
        locals.var_mnud0_dn4 = assign33490_e56023_d_n4;
        locals.var_mnud0_dn5 = assign33490_e56023_d_n5;
        locals.var_mnud0_dn6 = assign33490_e56023_d_n6;
        locals.var_mnud0_dn7 = assign33490_e56023_d_n7;
        locals.var_mnud0_dn8 = assign33490_e56023_d_n8;
        locals.var_mnud0_dn9 = assign33490_e56023_d_n9;
        locals.var_mnud0_dn10 = assign33490_e56023_d_n10;
        locals.var_mnud0_dn11 = assign33490_e56023_d_n11;
        locals.var_mnud0_dn13 = assign33490_e56023_d_n13;
        locals.var_mnud0_dn14 = assign33490_e56023_d_n14;

        let assign33500_e56026: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign33500_e56026;

    }

    pub(super) fn stamp_transient_block_129(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33510_e56070, assign33510_e56070_d_n0, assign33510_e56070_d_n2, assign33510_e56070_d_n3, assign33510_e56070_d_n4, assign33510_e56070_d_n5, assign33510_e56070_d_n6, assign33510_e56070_d_n7, assign33510_e56070_d_n8, assign33510_e56070_d_n9, assign33510_e56070_d_n10, assign33510_e56070_d_n11, assign33510_e56070_d_n13, assign33510_e56070_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33510_e56035: f64 = (-10000.0);
        let assign33510_e56037: f64 = (assign33510_e56035 * 1e-6);
        let (assign33510_e56068, assign33510_e56068_d_n4,) = {
            if (!(locals.var_k2_t < assign33510_e56037)) {
                let assign33510_e56044: f64 = (locals.var_k2_t * locals.var_k2_t);
                let assign33510_e56047: f64 = (4.0 * 1e-6);
                let assign33510_e56049: f64 = (assign33510_e56047 * 1e-6);
                let assign33510_e56050: f64 = (assign33510_e56044 + assign33510_e56049);
                let assign33510_e56051: f64 = (assign33510_e56050).sqrt();
                let assign33510_e56052: f64 = (locals.var_k2_t + assign33510_e56051);
                let assign33510_e56053: f64 = (0.5 * assign33510_e56052);
                (assign33510_e56053, (0.5 * (locals.var_k2_t_dn4 + (((locals.var_k2_t_dn4 * locals.var_k2_t) + (locals.var_k2_t * locals.var_k2_t_dn4)) / (2.0 * assign33510_e56051)))),)
            } else {
                let assign33510_e56056: f64 = (-10000.0);
                let assign33510_e56058: f64 = (assign33510_e56056 * 1e-6);
                let (assign33510_e56067, assign33510_e56067_d_n4,) = {
                    if (locals.var_k2_t < assign33510_e56058) {
                        let assign33510_e56061: f64 = (-1e-6);
                        let assign33510_e56063: f64 = (assign33510_e56061 * 1e-6);
                        let assign33510_e56065: f64 = (assign33510_e56063 / locals.var_k2_t);
                        (assign33510_e56065, (-((assign33510_e56063 * locals.var_k2_t_dn4) / (locals.var_k2_t * locals.var_k2_t))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign33510_e56067, assign33510_e56067_d_n4,)
            }
        };
        (assign33510_e56068, 0.0, 0.0, 0.0, assign33510_e56068_d_n4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33510_e56070;
        locals.var_t4_dn0 = assign33510_e56070_d_n0;
        locals.var_t4_dn2 = assign33510_e56070_d_n2;
        locals.var_t4_dn3 = assign33510_e56070_d_n3;
        locals.var_t4_dn4 = assign33510_e56070_d_n4;
        locals.var_t4_dn5 = assign33510_e56070_d_n5;
        locals.var_t4_dn6 = assign33510_e56070_d_n6;
        locals.var_t4_dn7 = assign33510_e56070_d_n7;
        locals.var_t4_dn8 = assign33510_e56070_d_n8;
        locals.var_t4_dn9 = assign33510_e56070_d_n9;
        locals.var_t4_dn10 = assign33510_e56070_d_n10;
        locals.var_t4_dn11 = assign33510_e56070_d_n11;
        locals.var_t4_dn13 = assign33510_e56070_d_n13;
        locals.var_t4_dn14 = assign33510_e56070_d_n14;

        let (assign33520_e56089, assign33520_e56089_d_n0, assign33520_e56089_d_n2, assign33520_e56089_d_n3, assign33520_e56089_d_n4, assign33520_e56089_d_n5, assign33520_e56089_d_n6, assign33520_e56089_d_n7, assign33520_e56089_d_n8, assign33520_e56089_d_n9, assign33520_e56089_d_n10, assign33520_e56089_d_n11, assign33520_e56089_d_n13, assign33520_e56089_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33520_e56080: f64 = (0.0_f64).max(locals.var_k2si_t);
        let assign33520_e56082: f64 = (assign33520_e56080 * locals.var_qis);
        let assign33520_e56085: f64 = (2.0 * locals.var_nvtm);
        let assign33520_e56086: f64 = (assign33520_e56082 + assign33520_e56085);
        let assign33520_e56087: f64 = (locals.var_t4 / assign33520_e56086);
        (assign33520_e56087, (((locals.var_t4_dn0 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn0) + (2.0 * locals.var_nvtm_dn0)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn2 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn2) + (2.0 * locals.var_nvtm_dn2)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn3 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn3) + (2.0 * locals.var_nvtm_dn3)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn4 * assign33520_e56086) - (locals.var_t4 * (((if 0.0 >= locals.var_k2si_t { 0.0 } else { locals.var_k2si_t_dn4 } * locals.var_qis) + (assign33520_e56080 * locals.var_qis_dn4)) + (2.0 * locals.var_nvtm_dn4)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn5 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn5) + (2.0 * locals.var_nvtm_dn5)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn6 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn6) + (2.0 * locals.var_nvtm_dn6)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn7 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn7) + (2.0 * locals.var_nvtm_dn7)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn8 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn8) + (2.0 * locals.var_nvtm_dn8)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn9 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn9) + (2.0 * locals.var_nvtm_dn9)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn10 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn10) + (2.0 * locals.var_nvtm_dn10)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn11 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn11) + (2.0 * locals.var_nvtm_dn11)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn13 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn13) + (2.0 * locals.var_nvtm_dn13)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn14 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn14) + (2.0 * locals.var_nvtm_dn14)))) / (assign33520_e56086 * assign33520_e56086)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33520_e56089;
        locals.var_t5_dn0 = assign33520_e56089_d_n0;
        locals.var_t5_dn2 = assign33520_e56089_d_n2;
        locals.var_t5_dn3 = assign33520_e56089_d_n3;
        locals.var_t5_dn4 = assign33520_e56089_d_n4;
        locals.var_t5_dn5 = assign33520_e56089_d_n5;
        locals.var_t5_dn6 = assign33520_e56089_d_n6;
        locals.var_t5_dn7 = assign33520_e56089_d_n7;
        locals.var_t5_dn8 = assign33520_e56089_d_n8;
        locals.var_t5_dn9 = assign33520_e56089_d_n9;
        locals.var_t5_dn10 = assign33520_e56089_d_n10;
        locals.var_t5_dn11 = assign33520_e56089_d_n11;
        locals.var_t5_dn13 = assign33520_e56089_d_n13;
        locals.var_t5_dn14 = assign33520_e56089_d_n14;

        let (assign33530_e56104, assign33530_e56104_d_n0, assign33530_e56104_d_n2, assign33530_e56104_d_n3, assign33530_e56104_d_n4, assign33530_e56104_d_n5, assign33530_e56104_d_n6, assign33530_e56104_d_n7, assign33530_e56104_d_n8, assign33530_e56104_d_n9, assign33530_e56104_d_n10, assign33530_e56104_d_n11, assign33530_e56104_d_n13, assign33530_e56104_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33530_e56098: f64 = (locals.var_phibe_i - locals.var_veseff);
        let assign33530_e56099: f64 = (assign33530_e56098).sqrt();
        let assign33530_e56101: f64 = (locals.var_phibe_i).sqrt();
        let assign33530_e56102: f64 = (assign33530_e56099 - assign33530_e56101);
        (assign33530_e56102, ((-locals.var_veseff_dn0) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn2) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn3) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn4) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn5) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn6) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn7) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn8) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn9) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn10) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn11) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn13) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn14) / (2.0 * assign33530_e56099)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33530_e56104;
        locals.var_t6_dn0 = assign33530_e56104_d_n0;
        locals.var_t6_dn2 = assign33530_e56104_d_n2;
        locals.var_t6_dn3 = assign33530_e56104_d_n3;
        locals.var_t6_dn4 = assign33530_e56104_d_n4;
        locals.var_t6_dn5 = assign33530_e56104_d_n5;
        locals.var_t6_dn6 = assign33530_e56104_d_n6;
        locals.var_t6_dn7 = assign33530_e56104_d_n7;
        locals.var_t6_dn8 = assign33530_e56104_d_n8;
        locals.var_t6_dn9 = assign33530_e56104_d_n9;
        locals.var_t6_dn10 = assign33530_e56104_d_n10;
        locals.var_t6_dn11 = assign33530_e56104_d_n11;
        locals.var_t6_dn13 = assign33530_e56104_d_n13;
        locals.var_t6_dn14 = assign33530_e56104_d_n14;

        let (assign33540_e56117, assign33540_e56117_d_n0, assign33540_e56117_d_n2, assign33540_e56117_d_n3, assign33540_e56117_d_n4, assign33540_e56117_d_n5, assign33540_e56117_d_n6, assign33540_e56117_d_n7, assign33540_e56117_d_n8, assign33540_e56117_d_n9, assign33540_e56117_d_n10, assign33540_e56117_d_n11, assign33540_e56117_d_n13, assign33540_e56117_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33540_e56112: f64 = (-locals.var_t5);
        let assign33540_e56114: f64 = (assign33540_e56112 * locals.var_t6);
        let assign33540_e56115: f64 = { let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33540_e56115, ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn0) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn0))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn2) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn2))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn3) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn3))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn4) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn4))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn5) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn5))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn6) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn6))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn7) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn7))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn8) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn8))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn9) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn9))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn10) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn10))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn11) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn11))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn13) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn13))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn14) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn14))),)
    } else {
        (locals.var_mob0, locals.var_mob0_dn0, locals.var_mob0_dn2, locals.var_mob0_dn3, locals.var_mob0_dn4, locals.var_mob0_dn5, locals.var_mob0_dn6, locals.var_mob0_dn7, locals.var_mob0_dn8, locals.var_mob0_dn9, locals.var_mob0_dn10, locals.var_mob0_dn11, locals.var_mob0_dn13, locals.var_mob0_dn14,)
    }
};
        locals.var_mob0 = assign33540_e56117;
        locals.var_mob0_dn0 = assign33540_e56117_d_n0;
        locals.var_mob0_dn2 = assign33540_e56117_d_n2;
        locals.var_mob0_dn3 = assign33540_e56117_d_n3;
        locals.var_mob0_dn4 = assign33540_e56117_d_n4;
        locals.var_mob0_dn5 = assign33540_e56117_d_n5;
        locals.var_mob0_dn6 = assign33540_e56117_d_n6;
        locals.var_mob0_dn7 = assign33540_e56117_d_n7;
        locals.var_mob0_dn8 = assign33540_e56117_d_n8;
        locals.var_mob0_dn9 = assign33540_e56117_d_n9;
        locals.var_mob0_dn10 = assign33540_e56117_d_n10;
        locals.var_mob0_dn11 = assign33540_e56117_d_n11;
        locals.var_mob0_dn13 = assign33540_e56117_d_n13;
        locals.var_mob0_dn14 = assign33540_e56117_d_n14;

        let (assign33550_e56127, assign33550_e56127_d_n0, assign33550_e56127_d_n2, assign33550_e56127_d_n3, assign33550_e56127_d_n4, assign33550_e56127_d_n5, assign33550_e56127_d_n6, assign33550_e56127_d_n7, assign33550_e56127_d_n8, assign33550_e56127_d_n9, assign33550_e56127_d_n10, assign33550_e56127_d_n11, assign33550_e56127_d_n13, assign33550_e56127_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mob0, locals.var_mob0_dn0, locals.var_mob0_dn2, locals.var_mob0_dn3, locals.var_mob0_dn4, locals.var_mob0_dn5, locals.var_mob0_dn6, locals.var_mob0_dn7, locals.var_mob0_dn8, locals.var_mob0_dn9, locals.var_mob0_dn10, locals.var_mob0_dn11, locals.var_mob0_dn13, locals.var_mob0_dn14,)
    }
};
        locals.var_mob0 = assign33550_e56127;
        locals.var_mob0_dn0 = assign33550_e56127_d_n0;
        locals.var_mob0_dn2 = assign33550_e56127_d_n2;
        locals.var_mob0_dn3 = assign33550_e56127_d_n3;
        locals.var_mob0_dn4 = assign33550_e56127_d_n4;
        locals.var_mob0_dn5 = assign33550_e56127_d_n5;
        locals.var_mob0_dn6 = assign33550_e56127_d_n6;
        locals.var_mob0_dn7 = assign33550_e56127_d_n7;
        locals.var_mob0_dn8 = assign33550_e56127_d_n8;
        locals.var_mob0_dn9 = assign33550_e56127_d_n9;
        locals.var_mob0_dn10 = assign33550_e56127_d_n10;
        locals.var_mob0_dn11 = assign33550_e56127_d_n11;
        locals.var_mob0_dn13 = assign33550_e56127_d_n13;
        locals.var_mob0_dn14 = assign33550_e56127_d_n14;

        let (assign33560_e56140, assign33560_e56140_d_n0, assign33560_e56140_d_n2, assign33560_e56140_d_n3, assign33560_e56140_d_n4, assign33560_e56140_d_n5, assign33560_e56140_d_n6, assign33560_e56140_d_n7, assign33560_e56140_d_n8, assign33560_e56140_d_n9, assign33560_e56140_d_n10, assign33560_e56140_d_n11, assign33560_e56140_d_n13, assign33560_e56140_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33560_e56136: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign33560_e56137: f64 = (locals.var_qba + assign33560_e56136);
        let assign33560_e56138: f64 = (locals.var_eefffactor * assign33560_e56137);
        (assign33560_e56138, (locals.var_eefffactor * (locals.var_qba_dn0 + (locals.var_eta_mu * locals.var_qis_dn0))), (locals.var_eefffactor * (locals.var_qba_dn2 + (locals.var_eta_mu * locals.var_qis_dn2))), (locals.var_eefffactor * (locals.var_qba_dn3 + (locals.var_eta_mu * locals.var_qis_dn3))), (locals.var_eefffactor * (locals.var_qba_dn4 + ((locals.var_eta_mu_dn4 * locals.var_qis) + (locals.var_eta_mu * locals.var_qis_dn4)))), (locals.var_eefffactor * (locals.var_qba_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_eefffactor * (locals.var_qba_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_eefffactor * (locals.var_qba_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_eefffactor * (locals.var_qba_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_eefffactor * (locals.var_qba_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))), (locals.var_eefffactor * (locals.var_qba_dn10 + (locals.var_eta_mu * locals.var_qis_dn10))), (locals.var_eefffactor * (locals.var_qba_dn11 + (locals.var_eta_mu * locals.var_qis_dn11))), (locals.var_eefffactor * (locals.var_qba_dn13 + (locals.var_eta_mu * locals.var_qis_dn13))), (locals.var_eefffactor * (locals.var_qba_dn14 + (locals.var_eta_mu * locals.var_qis_dn14))),)
    } else {
        (locals.var_eeffm0, locals.var_eeffm0_dn0, locals.var_eeffm0_dn2, locals.var_eeffm0_dn3, locals.var_eeffm0_dn4, locals.var_eeffm0_dn5, locals.var_eeffm0_dn6, locals.var_eeffm0_dn7, locals.var_eeffm0_dn8, locals.var_eeffm0_dn9, locals.var_eeffm0_dn10, locals.var_eeffm0_dn11, locals.var_eeffm0_dn13, locals.var_eeffm0_dn14,)
    }
};
        locals.var_eeffm0 = assign33560_e56140;
        locals.var_eeffm0_dn0 = assign33560_e56140_d_n0;
        locals.var_eeffm0_dn2 = assign33560_e56140_d_n2;
        locals.var_eeffm0_dn3 = assign33560_e56140_d_n3;
        locals.var_eeffm0_dn4 = assign33560_e56140_d_n4;
        locals.var_eeffm0_dn5 = assign33560_e56140_d_n5;
        locals.var_eeffm0_dn6 = assign33560_e56140_d_n6;
        locals.var_eeffm0_dn7 = assign33560_e56140_d_n7;
        locals.var_eeffm0_dn8 = assign33560_e56140_d_n8;
        locals.var_eeffm0_dn9 = assign33560_e56140_d_n9;
        locals.var_eeffm0_dn10 = assign33560_e56140_d_n10;
        locals.var_eeffm0_dn11 = assign33560_e56140_d_n11;
        locals.var_eeffm0_dn13 = assign33560_e56140_d_n13;
        locals.var_eeffm0_dn14 = assign33560_e56140_d_n14;

        let (assign33570_e56156, assign33570_e56156_d_n0, assign33570_e56156_d_n2, assign33570_e56156_d_n3, assign33570_e56156_d_n4, assign33570_e56156_d_n5, assign33570_e56156_d_n6, assign33570_e56156_d_n7, assign33570_e56156_d_n8, assign33570_e56156_d_n9, assign33570_e56156_d_n10, assign33570_e56156_d_n11, assign33570_e56156_d_n13, assign33570_e56156_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33570_e56149: f64 = (locals.var_qis / locals.var_qb0);
        let assign33570_e56150: f64 = (assign33570_e56149).abs();
        let assign33570_e56151: f64 = (1.0 + assign33570_e56150);
        let assign33570_e56152: f64 = (0.5 * assign33570_e56151);
        let assign33570_e56154: f64 = (assign33570_e56152).powf(locals.var_ucs_t);
        (assign33570_e56154, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn0 / locals.var_qb0) } else { (-(locals.var_qis_dn0 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn0 / locals.var_qb0) } else { (-(locals.var_qis_dn0 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn2 / locals.var_qb0) } else { (-(locals.var_qis_dn2 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn2 / locals.var_qb0) } else { (-(locals.var_qis_dn2 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn3 / locals.var_qb0) } else { (-(locals.var_qis_dn3 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn3 / locals.var_qb0) } else { (-(locals.var_qis_dn3 / locals.var_qb0)) }) / assign33570_e56152))) }, if locals.var_ucs_t_dn4 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn4 / locals.var_qb0) } else { (-(locals.var_qis_dn4 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * ((locals.var_ucs_t_dn4 * (assign33570_e56152).ln()) + (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn4 / locals.var_qb0) } else { (-(locals.var_qis_dn4 / locals.var_qb0)) }) / assign33570_e56152)))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn5 / locals.var_qb0) } else { (-(locals.var_qis_dn5 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn5 / locals.var_qb0) } else { (-(locals.var_qis_dn5 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn6 / locals.var_qb0) } else { (-(locals.var_qis_dn6 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn6 / locals.var_qb0) } else { (-(locals.var_qis_dn6 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn7 / locals.var_qb0) } else { (-(locals.var_qis_dn7 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn7 / locals.var_qb0) } else { (-(locals.var_qis_dn7 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn8 / locals.var_qb0) } else { (-(locals.var_qis_dn8 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn8 / locals.var_qb0) } else { (-(locals.var_qis_dn8 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn9 / locals.var_qb0) } else { (-(locals.var_qis_dn9 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn9 / locals.var_qb0) } else { (-(locals.var_qis_dn9 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn10 / locals.var_qb0) } else { (-(locals.var_qis_dn10 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn10 / locals.var_qb0) } else { (-(locals.var_qis_dn10 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn11 / locals.var_qb0) } else { (-(locals.var_qis_dn11 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn11 / locals.var_qb0) } else { (-(locals.var_qis_dn11 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn13 / locals.var_qb0) } else { (-(locals.var_qis_dn13 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn13 / locals.var_qb0) } else { (-(locals.var_qis_dn13 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn14 / locals.var_qb0) } else { (-(locals.var_qis_dn14 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn14 / locals.var_qb0) } else { (-(locals.var_qis_dn14 / locals.var_qb0)) }) / assign33570_e56152))) },)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33570_e56156;
        locals.var_t4_dn0 = assign33570_e56156_d_n0;
        locals.var_t4_dn2 = assign33570_e56156_d_n2;
        locals.var_t4_dn3 = assign33570_e56156_d_n3;
        locals.var_t4_dn4 = assign33570_e56156_d_n4;
        locals.var_t4_dn5 = assign33570_e56156_d_n5;
        locals.var_t4_dn6 = assign33570_e56156_d_n6;
        locals.var_t4_dn7 = assign33570_e56156_d_n7;
        locals.var_t4_dn8 = assign33570_e56156_d_n8;
        locals.var_t4_dn9 = assign33570_e56156_d_n9;
        locals.var_t4_dn10 = assign33570_e56156_d_n10;
        locals.var_t4_dn11 = assign33570_e56156_d_n11;
        locals.var_t4_dn13 = assign33570_e56156_d_n13;
        locals.var_t4_dn14 = assign33570_e56156_d_n14;

        let assign33580_e56159: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign33580_e56159;

        let (assign33590_e56181, assign33590_e56181_d_n0, assign33590_e56181_d_n2, assign33590_e56181_d_n3, assign33590_e56181_d_n4, assign33590_e56181_d_n5, assign33590_e56181_d_n6, assign33590_e56181_d_n7, assign33590_e56181_d_n8, assign33590_e56181_d_n9, assign33590_e56181_d_n10, assign33590_e56181_d_n11, assign33590_e56181_d_n13, assign33590_e56181_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign33590_e56169: f64 = (locals.var_uc_a * locals.var_veseff);
        let assign33590_e56170: f64 = (locals.var_ua_a + assign33590_e56169);
        let assign33590_e56172: f64 = (locals.var_eeffm0).abs();
        let assign33590_e56174: f64 = (assign33590_e56172).powf(locals.var_eu_a);
        let assign33590_e56175: f64 = (assign33590_e56170 * assign33590_e56174);
        let assign33590_e56178: f64 = (locals.var_ud_a / locals.var_t4);
        let assign33590_e56179: f64 = (assign33590_e56175 + assign33590_e56178);
        (assign33590_e56179, ((((locals.var_ua_a_dn0 + ((locals.var_uc_a_dn0 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn0))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn0 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn0 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn0 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn2 + ((locals.var_uc_a_dn2 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn2))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn2 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn2 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn2 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn3))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn3 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn3 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn3 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn4))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn4 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn4 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn4 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn5))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn5 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn5 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn5 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn6))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn6 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn6 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn6 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn7))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn7 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn7 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn7 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn8))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn8 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn8 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn8 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn9))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn9 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn9 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn9 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn10))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn10 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn10 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn10 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn11))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn11 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn11 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn11 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn13 + ((locals.var_uc_a_dn13 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn13))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn13 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn13 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn13 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn14 + ((locals.var_uc_a_dn14 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn14))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn14 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn14 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn14 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33590_e56181;
        locals.var_t5_dn0 = assign33590_e56181_d_n0;
        locals.var_t5_dn2 = assign33590_e56181_d_n2;
        locals.var_t5_dn3 = assign33590_e56181_d_n3;
        locals.var_t5_dn4 = assign33590_e56181_d_n4;
        locals.var_t5_dn5 = assign33590_e56181_d_n5;
        locals.var_t5_dn6 = assign33590_e56181_d_n6;
        locals.var_t5_dn7 = assign33590_e56181_d_n7;
        locals.var_t5_dn8 = assign33590_e56181_d_n8;
        locals.var_t5_dn9 = assign33590_e56181_d_n9;
        locals.var_t5_dn10 = assign33590_e56181_d_n10;
        locals.var_t5_dn11 = assign33590_e56181_d_n11;
        locals.var_t5_dn13 = assign33590_e56181_d_n13;
        locals.var_t5_dn14 = assign33590_e56181_d_n14;

        let (assign33600_e56200, assign33600_e56200_d_n0, assign33600_e56200_d_n2, assign33600_e56200_d_n3, assign33600_e56200_d_n4, assign33600_e56200_d_n5, assign33600_e56200_d_n6, assign33600_e56200_d_n7, assign33600_e56200_d_n8, assign33600_e56200_d_n9, assign33600_e56200_d_n10, assign33600_e56200_d_n11, assign33600_e56200_d_n13, assign33600_e56200_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign33600_e56191: f64 = (locals.var_eeffm0).abs();
        let assign33600_e56193: f64 = (assign33600_e56191).powf(locals.var_eu_a);
        let assign33600_e56194: f64 = (locals.var_ua_a * assign33600_e56193);
        let assign33600_e56197: f64 = (locals.var_ud_a / locals.var_t4);
        let assign33600_e56198: f64 = (assign33600_e56194 + assign33600_e56197);
        (assign33600_e56198, (((locals.var_ua_a_dn0 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn0 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn0 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn0 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn2 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn2 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn2 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn2 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn3 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn3 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn3 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn3 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn4 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn4 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn4 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn4 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn5 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn5 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn5 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn5 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn6 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn6 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn6 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn6 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn7 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn7 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn7 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn7 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn8 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn8 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn8 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn8 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn9 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn9 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn9 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn9 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn10 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn10 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn10 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn10 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn11 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn11 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn11 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn11 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn13 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn13 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn13 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn13 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn14 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn14 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn14 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn14 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33600_e56200;
        locals.var_t5_dn0 = assign33600_e56200_d_n0;
        locals.var_t5_dn2 = assign33600_e56200_d_n2;
        locals.var_t5_dn3 = assign33600_e56200_d_n3;
        locals.var_t5_dn4 = assign33600_e56200_d_n4;
        locals.var_t5_dn5 = assign33600_e56200_d_n5;
        locals.var_t5_dn6 = assign33600_e56200_d_n6;
        locals.var_t5_dn7 = assign33600_e56200_d_n7;
        locals.var_t5_dn8 = assign33600_e56200_d_n8;
        locals.var_t5_dn9 = assign33600_e56200_d_n9;
        locals.var_t5_dn10 = assign33600_e56200_d_n10;
        locals.var_t5_dn11 = assign33600_e56200_d_n11;
        locals.var_t5_dn13 = assign33600_e56200_d_n13;
        locals.var_t5_dn14 = assign33600_e56200_d_n14;

        let (assign33610_e56209, assign33610_e56209_d_n0, assign33610_e56209_d_n2, assign33610_e56209_d_n3, assign33610_e56209_d_n4, assign33610_e56209_d_n5, assign33610_e56209_d_n6, assign33610_e56209_d_n7, assign33610_e56209_d_n8, assign33610_e56209_d_n9, assign33610_e56209_d_n10, assign33610_e56209_d_n11, assign33610_e56209_d_n13, assign33610_e56209_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33610_e56207: f64 = (1.0 + locals.var_t5);
        (assign33610_e56207, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33610_e56209;
        locals.var_dmob0_dn0 = assign33610_e56209_d_n0;
        locals.var_dmob0_dn2 = assign33610_e56209_d_n2;
        locals.var_dmob0_dn3 = assign33610_e56209_d_n3;
        locals.var_dmob0_dn4 = assign33610_e56209_d_n4;
        locals.var_dmob0_dn5 = assign33610_e56209_d_n5;
        locals.var_dmob0_dn6 = assign33610_e56209_d_n6;
        locals.var_dmob0_dn7 = assign33610_e56209_d_n7;
        locals.var_dmob0_dn8 = assign33610_e56209_d_n8;
        locals.var_dmob0_dn9 = assign33610_e56209_d_n9;
        locals.var_dmob0_dn10 = assign33610_e56209_d_n10;
        locals.var_dmob0_dn11 = assign33610_e56209_d_n11;
        locals.var_dmob0_dn13 = assign33610_e56209_d_n13;
        locals.var_dmob0_dn14 = assign33610_e56209_d_n14;

        let (assign33620_e56235, assign33620_e56235_d_n0, assign33620_e56235_d_n2, assign33620_e56235_d_n3, assign33620_e56235_d_n4, assign33620_e56235_d_n5, assign33620_e56235_d_n6, assign33620_e56235_d_n7, assign33620_e56235_d_n8, assign33620_e56235_d_n9, assign33620_e56235_d_n10, assign33620_e56235_d_n11, assign33620_e56235_d_n13, assign33620_e56235_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33620_e56217: f64 = (locals.var_dmob0 + 1.0);
        let assign33620_e56220: f64 = (locals.var_dmob0 - 1.0);
        let assign33620_e56223: f64 = (locals.var_dmob0 - 1.0);
        let assign33620_e56224: f64 = (assign33620_e56220 * assign33620_e56223);
        let assign33620_e56227: f64 = (0.25 * p.p604);
        let assign33620_e56229: f64 = (assign33620_e56227 * p.p604);
        let assign33620_e56230: f64 = (assign33620_e56224 + assign33620_e56229);
        let assign33620_e56231: f64 = (assign33620_e56230).sqrt();
        let assign33620_e56232: f64 = (assign33620_e56217 + assign33620_e56231);
        let assign33620_e56233: f64 = (0.5 * assign33620_e56232);
        (assign33620_e56233, (0.5 * (locals.var_dmob0_dn0 + (((locals.var_dmob0_dn0 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn0)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn2 + (((locals.var_dmob0_dn2 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn2)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn3 + (((locals.var_dmob0_dn3 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn3)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn4 + (((locals.var_dmob0_dn4 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn4)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn5 + (((locals.var_dmob0_dn5 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn5)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn6 + (((locals.var_dmob0_dn6 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn6)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn7 + (((locals.var_dmob0_dn7 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn7)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn8 + (((locals.var_dmob0_dn8 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn8)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn9 + (((locals.var_dmob0_dn9 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn9)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn10 + (((locals.var_dmob0_dn10 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn10)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn11 + (((locals.var_dmob0_dn11 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn11)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn13 + (((locals.var_dmob0_dn13 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn13)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn14 + (((locals.var_dmob0_dn14 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn14)) / (2.0 * assign33620_e56231)))),)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33620_e56235;
        locals.var_dmob0_dn0 = assign33620_e56235_d_n0;
        locals.var_dmob0_dn2 = assign33620_e56235_d_n2;
        locals.var_dmob0_dn3 = assign33620_e56235_d_n3;
        locals.var_dmob0_dn4 = assign33620_e56235_d_n4;
        locals.var_dmob0_dn5 = assign33620_e56235_d_n5;
        locals.var_dmob0_dn6 = assign33620_e56235_d_n6;
        locals.var_dmob0_dn7 = assign33620_e56235_d_n7;
        locals.var_dmob0_dn8 = assign33620_e56235_d_n8;
        locals.var_dmob0_dn9 = assign33620_e56235_d_n9;
        locals.var_dmob0_dn10 = assign33620_e56235_d_n10;
        locals.var_dmob0_dn11 = assign33620_e56235_d_n11;
        locals.var_dmob0_dn13 = assign33620_e56235_d_n13;
        locals.var_dmob0_dn14 = assign33620_e56235_d_n14;

        let (assign33630_e56244, assign33630_e56244_d_n0, assign33630_e56244_d_n2, assign33630_e56244_d_n3, assign33630_e56244_d_n4, assign33630_e56244_d_n5, assign33630_e56244_d_n6, assign33630_e56244_d_n7, assign33630_e56244_d_n8, assign33630_e56244_d_n9, assign33630_e56244_d_n10, assign33630_e56244_d_n11, assign33630_e56244_d_n13, assign33630_e56244_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33630_e56242: f64 = (locals.var_dmob0 / p.p24);
        (assign33630_e56242, (locals.var_dmob0_dn0 / p.p24), (locals.var_dmob0_dn2 / p.p24), (locals.var_dmob0_dn3 / p.p24), (locals.var_dmob0_dn4 / p.p24), (locals.var_dmob0_dn5 / p.p24), (locals.var_dmob0_dn6 / p.p24), (locals.var_dmob0_dn7 / p.p24), (locals.var_dmob0_dn8 / p.p24), (locals.var_dmob0_dn9 / p.p24), (locals.var_dmob0_dn10 / p.p24), (locals.var_dmob0_dn11 / p.p24), (locals.var_dmob0_dn13 / p.p24), (locals.var_dmob0_dn14 / p.p24),)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33630_e56244;
        locals.var_dmob0_dn0 = assign33630_e56244_d_n0;
        locals.var_dmob0_dn2 = assign33630_e56244_d_n2;
        locals.var_dmob0_dn3 = assign33630_e56244_d_n3;
        locals.var_dmob0_dn4 = assign33630_e56244_d_n4;
        locals.var_dmob0_dn5 = assign33630_e56244_d_n5;
        locals.var_dmob0_dn6 = assign33630_e56244_d_n6;
        locals.var_dmob0_dn7 = assign33630_e56244_d_n7;
        locals.var_dmob0_dn8 = assign33630_e56244_d_n8;
        locals.var_dmob0_dn9 = assign33630_e56244_d_n9;
        locals.var_dmob0_dn10 = assign33630_e56244_d_n10;
        locals.var_dmob0_dn11 = assign33630_e56244_d_n11;
        locals.var_dmob0_dn13 = assign33630_e56244_d_n13;
        locals.var_dmob0_dn14 = assign33630_e56244_d_n14;

        let (assign33640_e56255,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33640_e56252: f64 = (0.25 * p.p453);
        let assign33640_e56253: f64 = (1.0 + assign33640_e56252);
        (assign33640_e56253,)
    } else {
        (locals.var_dvsat0,)
    }
};
        locals.var_dvsat0 = assign33640_e56255;

        let (assign33650_e56266, assign33650_e56266_d_n0, assign33650_e56266_d_n2, assign33650_e56266_d_n3, assign33650_e56266_d_n4, assign33650_e56266_d_n5, assign33650_e56266_d_n6, assign33650_e56266_d_n7, assign33650_e56266_d_n8, assign33650_e56266_d_n9, assign33650_e56266_d_n10, assign33650_e56266_d_n11, assign33650_e56266_d_n13, assign33650_e56266_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33650_e56263: f64 = (locals.var_q0 + locals.var_qis);
        let assign33650_e56264: f64 = (locals.var_q0 / assign33650_e56263);
        (assign33650_e56264, (((locals.var_q0_dn0 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn0 + locals.var_qis_dn0))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn2 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn2 + locals.var_qis_dn2))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn3 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn3 + locals.var_qis_dn3))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn4 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn4 + locals.var_qis_dn4))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn5 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn5 + locals.var_qis_dn5))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn6 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn6 + locals.var_qis_dn6))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn7 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn7 + locals.var_qis_dn7))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn8 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn8 + locals.var_qis_dn8))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn9 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn9 + locals.var_qis_dn9))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn10 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn10 + locals.var_qis_dn10))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn11 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn11 + locals.var_qis_dn11))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn13 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn13 + locals.var_qis_dn13))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn14 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn14 + locals.var_qis_dn14))) / (assign33650_e56263 * assign33650_e56263)),)
    } else {
        (locals.var_etaiv0, locals.var_etaiv0_dn0, locals.var_etaiv0_dn2, locals.var_etaiv0_dn3, locals.var_etaiv0_dn4, locals.var_etaiv0_dn5, locals.var_etaiv0_dn6, locals.var_etaiv0_dn7, locals.var_etaiv0_dn8, locals.var_etaiv0_dn9, locals.var_etaiv0_dn10, locals.var_etaiv0_dn11, locals.var_etaiv0_dn13, locals.var_etaiv0_dn14,)
    }
};
        locals.var_etaiv0 = assign33650_e56266;
        locals.var_etaiv0_dn0 = assign33650_e56266_d_n0;
        locals.var_etaiv0_dn2 = assign33650_e56266_d_n2;
        locals.var_etaiv0_dn3 = assign33650_e56266_d_n3;
        locals.var_etaiv0_dn4 = assign33650_e56266_d_n4;
        locals.var_etaiv0_dn5 = assign33650_e56266_d_n5;
        locals.var_etaiv0_dn6 = assign33650_e56266_d_n6;
        locals.var_etaiv0_dn7 = assign33650_e56266_d_n7;
        locals.var_etaiv0_dn8 = assign33650_e56266_d_n8;
        locals.var_etaiv0_dn9 = assign33650_e56266_d_n9;
        locals.var_etaiv0_dn10 = assign33650_e56266_d_n10;
        locals.var_etaiv0_dn11 = assign33650_e56266_d_n11;
        locals.var_etaiv0_dn13 = assign33650_e56266_d_n13;
        locals.var_etaiv0_dn14 = assign33650_e56266_d_n14;

        let (assign33660_e56277, assign33660_e56277_d_n0, assign33660_e56277_d_n2, assign33660_e56277_d_n3, assign33660_e56277_d_n4, assign33660_e56277_d_n5, assign33660_e56277_d_n6, assign33660_e56277_d_n7, assign33660_e56277_d_n8, assign33660_e56277_d_n9, assign33660_e56277_d_n10, assign33660_e56277_d_n11, assign33660_e56277_d_n13, assign33660_e56277_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33660_e56273: f64 = (2.0 - locals.var_etaiv0);
        let assign33660_e56275: f64 = (assign33660_e56273 * locals.var_nvtm);
        (assign33660_e56275, (((-locals.var_etaiv0_dn0) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn0)), (((-locals.var_etaiv0_dn2) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn2)), (((-locals.var_etaiv0_dn3) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn3)), (((-locals.var_etaiv0_dn4) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn4)), (((-locals.var_etaiv0_dn5) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn5)), (((-locals.var_etaiv0_dn6) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn6)), (((-locals.var_etaiv0_dn7) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn7)), (((-locals.var_etaiv0_dn8) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn8)), (((-locals.var_etaiv0_dn9) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn9)), (((-locals.var_etaiv0_dn10) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn10)), (((-locals.var_etaiv0_dn11) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn11)), (((-locals.var_etaiv0_dn13) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn13)), (((-locals.var_etaiv0_dn14) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33660_e56277;
        locals.var_t4_dn0 = assign33660_e56277_d_n0;
        locals.var_t4_dn2 = assign33660_e56277_d_n2;
        locals.var_t4_dn3 = assign33660_e56277_d_n3;
        locals.var_t4_dn4 = assign33660_e56277_d_n4;
        locals.var_t4_dn5 = assign33660_e56277_d_n5;
        locals.var_t4_dn6 = assign33660_e56277_d_n6;
        locals.var_t4_dn7 = assign33660_e56277_d_n7;
        locals.var_t4_dn8 = assign33660_e56277_d_n8;
        locals.var_t4_dn9 = assign33660_e56277_d_n9;
        locals.var_t4_dn10 = assign33660_e56277_d_n10;
        locals.var_t4_dn11 = assign33660_e56277_d_n11;
        locals.var_t4_dn13 = assign33660_e56277_d_n13;
        locals.var_t4_dn14 = assign33660_e56277_d_n14;

        let (assign33670_e56286, assign33670_e56286_d_n0, assign33670_e56286_d_n2, assign33670_e56286_d_n3, assign33670_e56286_d_n4, assign33670_e56286_d_n5, assign33670_e56286_d_n6, assign33670_e56286_d_n7, assign33670_e56286_d_n8, assign33670_e56286_d_n9, assign33670_e56286_d_n10, assign33670_e56286_d_n11, assign33670_e56286_d_n13, assign33670_e56286_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33670_e56284: f64 = (locals.var_qis + locals.var_t4);
        (assign33670_e56284, (locals.var_qis_dn0 + locals.var_t4_dn0), (locals.var_qis_dn2 + locals.var_t4_dn2), (locals.var_qis_dn3 + locals.var_t4_dn3), (locals.var_qis_dn4 + locals.var_t4_dn4), (locals.var_qis_dn5 + locals.var_t4_dn5), (locals.var_qis_dn6 + locals.var_t4_dn6), (locals.var_qis_dn7 + locals.var_t4_dn7), (locals.var_qis_dn8 + locals.var_t4_dn8), (locals.var_qis_dn9 + locals.var_t4_dn9), (locals.var_qis_dn10 + locals.var_t4_dn10), (locals.var_qis_dn11 + locals.var_t4_dn11), (locals.var_qis_dn13 + locals.var_t4_dn13), (locals.var_qis_dn14 + locals.var_t4_dn14),)
    } else {
        (locals.var_ids0_ov_dqi0, locals.var_ids0_ov_dqi0_dn0, locals.var_ids0_ov_dqi0_dn2, locals.var_ids0_ov_dqi0_dn3, locals.var_ids0_ov_dqi0_dn4, locals.var_ids0_ov_dqi0_dn5, locals.var_ids0_ov_dqi0_dn6, locals.var_ids0_ov_dqi0_dn7, locals.var_ids0_ov_dqi0_dn8, locals.var_ids0_ov_dqi0_dn9, locals.var_ids0_ov_dqi0_dn10, locals.var_ids0_ov_dqi0_dn11, locals.var_ids0_ov_dqi0_dn13, locals.var_ids0_ov_dqi0_dn14,)
    }
};
        locals.var_ids0_ov_dqi0 = assign33670_e56286;
        locals.var_ids0_ov_dqi0_dn0 = assign33670_e56286_d_n0;
        locals.var_ids0_ov_dqi0_dn2 = assign33670_e56286_d_n2;
        locals.var_ids0_ov_dqi0_dn3 = assign33670_e56286_d_n3;
        locals.var_ids0_ov_dqi0_dn4 = assign33670_e56286_d_n4;
        locals.var_ids0_ov_dqi0_dn5 = assign33670_e56286_d_n5;
        locals.var_ids0_ov_dqi0_dn6 = assign33670_e56286_d_n6;
        locals.var_ids0_ov_dqi0_dn7 = assign33670_e56286_d_n7;
        locals.var_ids0_ov_dqi0_dn8 = assign33670_e56286_d_n8;
        locals.var_ids0_ov_dqi0_dn9 = assign33670_e56286_d_n9;
        locals.var_ids0_ov_dqi0_dn10 = assign33670_e56286_d_n10;
        locals.var_ids0_ov_dqi0_dn11 = assign33670_e56286_d_n11;
        locals.var_ids0_ov_dqi0_dn13 = assign33670_e56286_d_n13;
        locals.var_ids0_ov_dqi0_dn14 = assign33670_e56286_d_n14;

        let assign33680_e56289: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign33680_e56289;

        let assign33690_e56292: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign33690_e56292;

        let assign33700_e56295: f64 = if p.p64 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign33700_e56295;

        let (assign33710_e56308, assign33710_e56308_d_n0, assign33710_e56308_d_n2, assign33710_e56308_d_n3, assign33710_e56308_d_n4, assign33710_e56308_d_n5, assign33710_e56308_d_n6, assign33710_e56308_d_n7, assign33710_e56308_d_n8, assign33710_e56308_d_n9, assign33710_e56308_d_n10, assign33710_e56308_d_n11, assign33710_e56308_d_n13, assign33710_e56308_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33710_e56305: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign33710_e56306: f64 = (1.0 + assign33710_e56305);
        (assign33710_e56306, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33710_e56308;
        locals.var_t4_dn0 = assign33710_e56308_d_n0;
        locals.var_t4_dn2 = assign33710_e56308_d_n2;
        locals.var_t4_dn3 = assign33710_e56308_d_n3;
        locals.var_t4_dn4 = assign33710_e56308_d_n4;
        locals.var_t4_dn5 = assign33710_e56308_d_n5;
        locals.var_t4_dn6 = assign33710_e56308_d_n6;
        locals.var_t4_dn7 = assign33710_e56308_d_n7;
        locals.var_t4_dn8 = assign33710_e56308_d_n8;
        locals.var_t4_dn9 = assign33710_e56308_d_n9;
        locals.var_t4_dn10 = assign33710_e56308_d_n10;
        locals.var_t4_dn11 = assign33710_e56308_d_n11;
        locals.var_t4_dn13 = assign33710_e56308_d_n13;
        locals.var_t4_dn14 = assign33710_e56308_d_n14;

        let (assign33720_e56319, assign33720_e56319_d_n0, assign33720_e56319_d_n2, assign33720_e56319_d_n3, assign33720_e56319_d_n4, assign33720_e56319_d_n5, assign33720_e56319_d_n6, assign33720_e56319_d_n7, assign33720_e56319_d_n8, assign33720_e56319_d_n9, assign33720_e56319_d_n10, assign33720_e56319_d_n11, assign33720_e56319_d_n13, assign33720_e56319_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33720_e56317: f64 = (1.0 / locals.var_t4);
        (assign33720_e56317, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33720_e56319;
        locals.var_t5_dn0 = assign33720_e56319_d_n0;
        locals.var_t5_dn2 = assign33720_e56319_d_n2;
        locals.var_t5_dn3 = assign33720_e56319_d_n3;
        locals.var_t5_dn4 = assign33720_e56319_d_n4;
        locals.var_t5_dn5 = assign33720_e56319_d_n5;
        locals.var_t5_dn6 = assign33720_e56319_d_n6;
        locals.var_t5_dn7 = assign33720_e56319_d_n7;
        locals.var_t5_dn8 = assign33720_e56319_d_n8;
        locals.var_t5_dn9 = assign33720_e56319_d_n9;
        locals.var_t5_dn10 = assign33720_e56319_d_n10;
        locals.var_t5_dn11 = assign33720_e56319_d_n11;
        locals.var_t5_dn13 = assign33720_e56319_d_n13;
        locals.var_t5_dn14 = assign33720_e56319_d_n14;

        let (assign33730_e56337, assign33730_e56337_d_n0, assign33730_e56337_d_n2, assign33730_e56337_d_n3, assign33730_e56337_d_n4, assign33730_e56337_d_n5, assign33730_e56337_d_n6, assign33730_e56337_d_n7, assign33730_e56337_d_n8, assign33730_e56337_d_n9, assign33730_e56337_d_n10, assign33730_e56337_d_n11, assign33730_e56337_d_n13, assign33730_e56337_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33730_e56330: f64 = (locals.var_t5 * locals.var_t5);
        let assign33730_e56332: f64 = (assign33730_e56330 + 0.01);
        let assign33730_e56333: f64 = (assign33730_e56332).sqrt();
        let assign33730_e56334: f64 = (locals.var_t5 + assign33730_e56333);
        let assign33730_e56335: f64 = (0.5 * assign33730_e56334);
        (assign33730_e56335, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign33730_e56333)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33730_e56337;
        locals.var_t6_dn0 = assign33730_e56337_d_n0;
        locals.var_t6_dn2 = assign33730_e56337_d_n2;
        locals.var_t6_dn3 = assign33730_e56337_d_n3;
        locals.var_t6_dn4 = assign33730_e56337_d_n4;
        locals.var_t6_dn5 = assign33730_e56337_d_n5;
        locals.var_t6_dn6 = assign33730_e56337_d_n6;
        locals.var_t6_dn7 = assign33730_e56337_d_n7;
        locals.var_t6_dn8 = assign33730_e56337_d_n8;
        locals.var_t6_dn9 = assign33730_e56337_d_n9;
        locals.var_t6_dn10 = assign33730_e56337_d_n10;
        locals.var_t6_dn11 = assign33730_e56337_d_n11;
        locals.var_t6_dn13 = assign33730_e56337_d_n13;
        locals.var_t6_dn14 = assign33730_e56337_d_n14;

    }

    pub(super) fn stamp_transient_block_130(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33740_e56354, assign33740_e56354_d_n0, assign33740_e56354_d_n2, assign33740_e56354_d_n3, assign33740_e56354_d_n4, assign33740_e56354_d_n5, assign33740_e56354_d_n6, assign33740_e56354_d_n7, assign33740_e56354_d_n8, assign33740_e56354_d_n9, assign33740_e56354_d_n10, assign33740_e56354_d_n11, assign33740_e56354_d_n13, assign33740_e56354_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33740_e56348: f64 = (locals.var_rdsw_i * locals.var_t6);
        let assign33740_e56349: f64 = (p.p908 + assign33740_e56348);
        let assign33740_e56350: f64 = (locals.var_rdstemp * assign33740_e56349);
        let assign33740_e56352: f64 = (assign33740_e56350 * locals.var_weffwrfactor);
        (assign33740_e56352, (((locals.var_rdstemp_dn0 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn0 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn0)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn2 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn2 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn2)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn3 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn3 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn3)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn4 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn4 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn4)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn5 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn5 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn5)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn6 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn6 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn6)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn7 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn7 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn7)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn8 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn8 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn8)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn9 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn9 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn9)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn10 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn10 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn10)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn11 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn11 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn11)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn13 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn13 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn13)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn14 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn14 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn14)))) * locals.var_weffwrfactor),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33740_e56354;
        locals.var_rdsi0_dn0 = assign33740_e56354_d_n0;
        locals.var_rdsi0_dn2 = assign33740_e56354_d_n2;
        locals.var_rdsi0_dn3 = assign33740_e56354_d_n3;
        locals.var_rdsi0_dn4 = assign33740_e56354_d_n4;
        locals.var_rdsi0_dn5 = assign33740_e56354_d_n5;
        locals.var_rdsi0_dn6 = assign33740_e56354_d_n6;
        locals.var_rdsi0_dn7 = assign33740_e56354_d_n7;
        locals.var_rdsi0_dn8 = assign33740_e56354_d_n8;
        locals.var_rdsi0_dn9 = assign33740_e56354_d_n9;
        locals.var_rdsi0_dn10 = assign33740_e56354_d_n10;
        locals.var_rdsi0_dn11 = assign33740_e56354_d_n11;
        locals.var_rdsi0_dn13 = assign33740_e56354_d_n13;
        locals.var_rdsi0_dn14 = assign33740_e56354_d_n14;

        let (assign33750_e56375, assign33750_e56375_d_n0, assign33750_e56375_d_n2, assign33750_e56375_d_n3, assign33750_e56375_d_n4, assign33750_e56375_d_n5, assign33750_e56375_d_n6, assign33750_e56375_d_n7, assign33750_e56375_d_n8, assign33750_e56375_d_n9, assign33750_e56375_d_n10, assign33750_e56375_d_n11, assign33750_e56375_d_n13, assign33750_e56375_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33750_e56364: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33750_e56366: f64 = (assign33750_e56364 * locals.var_ids0_ov_dqi0);
        let assign33750_e56369: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33750_e56370: f64 = (assign33750_e56366 / assign33750_e56369);
        let assign33750_e56372: f64 = (assign33750_e56370 * locals.var_rdsi0);
        let assign33750_e56373: f64 = (1.0 + assign33750_e56372);
        (assign33750_e56373, ((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn0)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn0 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn0)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn2)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn2 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn2)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn3)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn3 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn3)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn4)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn4 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn4)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn5)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn5 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn5)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn6)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn6 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn6)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn7)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn7 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn7)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn8)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn8 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn8)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn9)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn9 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn9)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn10)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn10 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn10)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn11)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn11 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn11)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn13)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn13 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn13)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn14)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn14 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn14)),)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33750_e56375;
        locals.var_dr0_dn0 = assign33750_e56375_d_n0;
        locals.var_dr0_dn2 = assign33750_e56375_d_n2;
        locals.var_dr0_dn3 = assign33750_e56375_d_n3;
        locals.var_dr0_dn4 = assign33750_e56375_d_n4;
        locals.var_dr0_dn5 = assign33750_e56375_d_n5;
        locals.var_dr0_dn6 = assign33750_e56375_d_n6;
        locals.var_dr0_dn7 = assign33750_e56375_d_n7;
        locals.var_dr0_dn8 = assign33750_e56375_d_n8;
        locals.var_dr0_dn9 = assign33750_e56375_d_n9;
        locals.var_dr0_dn10 = assign33750_e56375_d_n10;
        locals.var_dr0_dn11 = assign33750_e56375_d_n11;
        locals.var_dr0_dn13 = assign33750_e56375_d_n13;
        locals.var_dr0_dn14 = assign33750_e56375_d_n14;

        let (assign33760_e56387, assign33760_e56387_d_n0, assign33760_e56387_d_n2, assign33760_e56387_d_n3, assign33760_e56387_d_n4, assign33760_e56387_d_n5, assign33760_e56387_d_n6, assign33760_e56387_d_n7, assign33760_e56387_d_n8, assign33760_e56387_d_n9, assign33760_e56387_d_n10, assign33760_e56387_d_n11, assign33760_e56387_d_n13, assign33760_e56387_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard637 != 0.0) && (locals.var_guard636 == 0.0))) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33760_e56387;
        locals.var_dr0_dn0 = assign33760_e56387_d_n0;
        locals.var_dr0_dn2 = assign33760_e56387_d_n2;
        locals.var_dr0_dn3 = assign33760_e56387_d_n3;
        locals.var_dr0_dn4 = assign33760_e56387_d_n4;
        locals.var_dr0_dn5 = assign33760_e56387_d_n5;
        locals.var_dr0_dn6 = assign33760_e56387_d_n6;
        locals.var_dr0_dn7 = assign33760_e56387_d_n7;
        locals.var_dr0_dn8 = assign33760_e56387_d_n8;
        locals.var_dr0_dn9 = assign33760_e56387_d_n9;
        locals.var_dr0_dn10 = assign33760_e56387_d_n10;
        locals.var_dr0_dn11 = assign33760_e56387_d_n11;
        locals.var_dr0_dn13 = assign33760_e56387_d_n13;
        locals.var_dr0_dn14 = assign33760_e56387_d_n14;

        let (assign33770_e56405, assign33770_e56405_d_n0, assign33770_e56405_d_n2, assign33770_e56405_d_n3, assign33770_e56405_d_n4, assign33770_e56405_d_n5, assign33770_e56405_d_n6, assign33770_e56405_d_n7, assign33770_e56405_d_n8, assign33770_e56405_d_n9, assign33770_e56405_d_n10, assign33770_e56405_d_n11, assign33770_e56405_d_n13, assign33770_e56405_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33770_e56402: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign33770_e56403: f64 = (1.0 + assign33770_e56402);
        (assign33770_e56403, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33770_e56405;
        locals.var_t4_dn0 = assign33770_e56405_d_n0;
        locals.var_t4_dn2 = assign33770_e56405_d_n2;
        locals.var_t4_dn3 = assign33770_e56405_d_n3;
        locals.var_t4_dn4 = assign33770_e56405_d_n4;
        locals.var_t4_dn5 = assign33770_e56405_d_n5;
        locals.var_t4_dn6 = assign33770_e56405_d_n6;
        locals.var_t4_dn7 = assign33770_e56405_d_n7;
        locals.var_t4_dn8 = assign33770_e56405_d_n8;
        locals.var_t4_dn9 = assign33770_e56405_d_n9;
        locals.var_t4_dn10 = assign33770_e56405_d_n10;
        locals.var_t4_dn11 = assign33770_e56405_d_n11;
        locals.var_t4_dn13 = assign33770_e56405_d_n13;
        locals.var_t4_dn14 = assign33770_e56405_d_n14;

        let (assign33780_e56421, assign33780_e56421_d_n0, assign33780_e56421_d_n2, assign33780_e56421_d_n3, assign33780_e56421_d_n4, assign33780_e56421_d_n5, assign33780_e56421_d_n6, assign33780_e56421_d_n7, assign33780_e56421_d_n8, assign33780_e56421_d_n9, assign33780_e56421_d_n10, assign33780_e56421_d_n11, assign33780_e56421_d_n13, assign33780_e56421_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33780_e56419: f64 = (1.0 / locals.var_t4);
        (assign33780_e56419, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33780_e56421;
        locals.var_t5_dn0 = assign33780_e56421_d_n0;
        locals.var_t5_dn2 = assign33780_e56421_d_n2;
        locals.var_t5_dn3 = assign33780_e56421_d_n3;
        locals.var_t5_dn4 = assign33780_e56421_d_n4;
        locals.var_t5_dn5 = assign33780_e56421_d_n5;
        locals.var_t5_dn6 = assign33780_e56421_d_n6;
        locals.var_t5_dn7 = assign33780_e56421_d_n7;
        locals.var_t5_dn8 = assign33780_e56421_d_n8;
        locals.var_t5_dn9 = assign33780_e56421_d_n9;
        locals.var_t5_dn10 = assign33780_e56421_d_n10;
        locals.var_t5_dn11 = assign33780_e56421_d_n11;
        locals.var_t5_dn13 = assign33780_e56421_d_n13;
        locals.var_t5_dn14 = assign33780_e56421_d_n14;

        let (assign33790_e56444, assign33790_e56444_d_n0, assign33790_e56444_d_n2, assign33790_e56444_d_n3, assign33790_e56444_d_n4, assign33790_e56444_d_n5, assign33790_e56444_d_n6, assign33790_e56444_d_n7, assign33790_e56444_d_n8, assign33790_e56444_d_n9, assign33790_e56444_d_n10, assign33790_e56444_d_n11, assign33790_e56444_d_n13, assign33790_e56444_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33790_e56437: f64 = (locals.var_t5 * locals.var_t5);
        let assign33790_e56439: f64 = (assign33790_e56437 + 0.01);
        let assign33790_e56440: f64 = (assign33790_e56439).sqrt();
        let assign33790_e56441: f64 = (locals.var_t5 + assign33790_e56440);
        let assign33790_e56442: f64 = (0.5 * assign33790_e56441);
        (assign33790_e56442, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign33790_e56440)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33790_e56444;
        locals.var_t6_dn0 = assign33790_e56444_d_n0;
        locals.var_t6_dn2 = assign33790_e56444_d_n2;
        locals.var_t6_dn3 = assign33790_e56444_d_n3;
        locals.var_t6_dn4 = assign33790_e56444_d_n4;
        locals.var_t6_dn5 = assign33790_e56444_d_n5;
        locals.var_t6_dn6 = assign33790_e56444_d_n6;
        locals.var_t6_dn7 = assign33790_e56444_d_n7;
        locals.var_t6_dn8 = assign33790_e56444_d_n8;
        locals.var_t6_dn9 = assign33790_e56444_d_n9;
        locals.var_t6_dn10 = assign33790_e56444_d_n10;
        locals.var_t6_dn11 = assign33790_e56444_d_n11;
        locals.var_t6_dn13 = assign33790_e56444_d_n13;
        locals.var_t6_dn14 = assign33790_e56444_d_n14;

        let (assign33800_e56464, assign33800_e56464_d_n0, assign33800_e56464_d_n2, assign33800_e56464_d_n3, assign33800_e56464_d_n4, assign33800_e56464_d_n5, assign33800_e56464_d_n6, assign33800_e56464_d_n7, assign33800_e56464_d_n8, assign33800_e56464_d_n9, assign33800_e56464_d_n10, assign33800_e56464_d_n11, assign33800_e56464_d_n13, assign33800_e56464_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33800_e56459: f64 = (locals.var_rdsw_i * locals.var_t6);
        let assign33800_e56460: f64 = (p.p908 + assign33800_e56459);
        let assign33800_e56462: f64 = (assign33800_e56460 * locals.var_weffwrfactor);
        (assign33800_e56462, (((locals.var_rdsw_i_dn0 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn0)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn2 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn2)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn3 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn3)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn4 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn4)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn5 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn5)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn6 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn6)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn7 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn7)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn8 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn8)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn9 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn9)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn10 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn10)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn11 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn11)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn13 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn13)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn14 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn14)) * locals.var_weffwrfactor),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33800_e56464;
        locals.var_rdsi0_dn0 = assign33800_e56464_d_n0;
        locals.var_rdsi0_dn2 = assign33800_e56464_d_n2;
        locals.var_rdsi0_dn3 = assign33800_e56464_d_n3;
        locals.var_rdsi0_dn4 = assign33800_e56464_d_n4;
        locals.var_rdsi0_dn5 = assign33800_e56464_d_n5;
        locals.var_rdsi0_dn6 = assign33800_e56464_d_n6;
        locals.var_rdsi0_dn7 = assign33800_e56464_d_n7;
        locals.var_rdsi0_dn8 = assign33800_e56464_d_n8;
        locals.var_rdsi0_dn9 = assign33800_e56464_d_n9;
        locals.var_rdsi0_dn10 = assign33800_e56464_d_n10;
        locals.var_rdsi0_dn11 = assign33800_e56464_d_n11;
        locals.var_rdsi0_dn13 = assign33800_e56464_d_n13;
        locals.var_rdsi0_dn14 = assign33800_e56464_d_n14;

        let (assign33810_e56484, assign33810_e56484_d_n0, assign33810_e56484_d_n2, assign33810_e56484_d_n3, assign33810_e56484_d_n4, assign33810_e56484_d_n5, assign33810_e56484_d_n6, assign33810_e56484_d_n7, assign33810_e56484_d_n8, assign33810_e56484_d_n9, assign33810_e56484_d_n10, assign33810_e56484_d_n11, assign33810_e56484_d_n13, assign33810_e56484_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33810_e56479: f64 = (locals.var_rsourcegeo + locals.var_rdraingeo);
        let assign33810_e56481: f64 = (assign33810_e56479 + locals.var_rdsi0);
        let assign33810_e56482: f64 = (locals.var_rdstemp * assign33810_e56481);
        (assign33810_e56482, ((locals.var_rdstemp_dn0 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn0 + locals.var_rdraingeo_dn0) + locals.var_rdsi0_dn0))), ((locals.var_rdstemp_dn2 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn2 + locals.var_rdraingeo_dn2) + locals.var_rdsi0_dn2))), ((locals.var_rdstemp_dn3 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn3 + locals.var_rdraingeo_dn3) + locals.var_rdsi0_dn3))), ((locals.var_rdstemp_dn4 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn4 + locals.var_rdraingeo_dn4) + locals.var_rdsi0_dn4))), ((locals.var_rdstemp_dn5 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn5 + locals.var_rdraingeo_dn5) + locals.var_rdsi0_dn5))), ((locals.var_rdstemp_dn6 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn6 + locals.var_rdraingeo_dn6) + locals.var_rdsi0_dn6))), ((locals.var_rdstemp_dn7 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn7 + locals.var_rdraingeo_dn7) + locals.var_rdsi0_dn7))), ((locals.var_rdstemp_dn8 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn8 + locals.var_rdraingeo_dn8) + locals.var_rdsi0_dn8))), ((locals.var_rdstemp_dn9 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn9 + locals.var_rdraingeo_dn9) + locals.var_rdsi0_dn9))), ((locals.var_rdstemp_dn10 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn10 + locals.var_rdraingeo_dn10) + locals.var_rdsi0_dn10))), ((locals.var_rdstemp_dn11 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn11 + locals.var_rdraingeo_dn11) + locals.var_rdsi0_dn11))), ((locals.var_rdstemp_dn13 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn13 + locals.var_rdraingeo_dn13) + locals.var_rdsi0_dn13))), ((locals.var_rdstemp_dn14 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn14 + locals.var_rdraingeo_dn14) + locals.var_rdsi0_dn14))),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33810_e56484;
        locals.var_rdsi0_dn0 = assign33810_e56484_d_n0;
        locals.var_rdsi0_dn2 = assign33810_e56484_d_n2;
        locals.var_rdsi0_dn3 = assign33810_e56484_d_n3;
        locals.var_rdsi0_dn4 = assign33810_e56484_d_n4;
        locals.var_rdsi0_dn5 = assign33810_e56484_d_n5;
        locals.var_rdsi0_dn6 = assign33810_e56484_d_n6;
        locals.var_rdsi0_dn7 = assign33810_e56484_d_n7;
        locals.var_rdsi0_dn8 = assign33810_e56484_d_n8;
        locals.var_rdsi0_dn9 = assign33810_e56484_d_n9;
        locals.var_rdsi0_dn10 = assign33810_e56484_d_n10;
        locals.var_rdsi0_dn11 = assign33810_e56484_d_n11;
        locals.var_rdsi0_dn13 = assign33810_e56484_d_n13;
        locals.var_rdsi0_dn14 = assign33810_e56484_d_n14;

        let (assign33820_e56510, assign33820_e56510_d_n0, assign33820_e56510_d_n2, assign33820_e56510_d_n3, assign33820_e56510_d_n4, assign33820_e56510_d_n5, assign33820_e56510_d_n6, assign33820_e56510_d_n7, assign33820_e56510_d_n8, assign33820_e56510_d_n9, assign33820_e56510_d_n10, assign33820_e56510_d_n11, assign33820_e56510_d_n13, assign33820_e56510_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33820_e56499: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33820_e56501: f64 = (assign33820_e56499 * locals.var_ids0_ov_dqi0);
        let assign33820_e56504: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33820_e56505: f64 = (assign33820_e56501 / assign33820_e56504);
        let assign33820_e56507: f64 = (assign33820_e56505 * locals.var_rdsi0);
        let assign33820_e56508: f64 = (1.0 + assign33820_e56507);
        (assign33820_e56508, ((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn0)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn0 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn0)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn2)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn2 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn2)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn3)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn3 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn3)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn4)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn4 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn4)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn5)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn5 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn5)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn6)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn6 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn6)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn7)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn7 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn7)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn8)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn8 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn8)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn9)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn9 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn9)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn10)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn10 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn10)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn11)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn11 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn11)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn13)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn13 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn13)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn14)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn14 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn14)),)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33820_e56510;
        locals.var_dr0_dn0 = assign33820_e56510_d_n0;
        locals.var_dr0_dn2 = assign33820_e56510_d_n2;
        locals.var_dr0_dn3 = assign33820_e56510_d_n3;
        locals.var_dr0_dn4 = assign33820_e56510_d_n4;
        locals.var_dr0_dn5 = assign33820_e56510_d_n5;
        locals.var_dr0_dn6 = assign33820_e56510_d_n6;
        locals.var_dr0_dn7 = assign33820_e56510_d_n7;
        locals.var_dr0_dn8 = assign33820_e56510_d_n8;
        locals.var_dr0_dn9 = assign33820_e56510_d_n9;
        locals.var_dr0_dn10 = assign33820_e56510_d_n10;
        locals.var_dr0_dn11 = assign33820_e56510_d_n11;
        locals.var_dr0_dn13 = assign33820_e56510_d_n13;
        locals.var_dr0_dn14 = assign33820_e56510_d_n14;

        let (assign33830_e56531, assign33830_e56531_d_n0, assign33830_e56531_d_n2, assign33830_e56531_d_n3, assign33830_e56531_d_n4, assign33830_e56531_d_n5, assign33830_e56531_d_n6, assign33830_e56531_d_n7, assign33830_e56531_d_n8, assign33830_e56531_d_n9, assign33830_e56531_d_n10, assign33830_e56531_d_n11, assign33830_e56531_d_n13, assign33830_e56531_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33830_e56517: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33830_e56519: f64 = (assign33830_e56517 * locals.var_qis);
        let assign33830_e56521: f64 = (assign33830_e56519 * locals.var_mnud0);
        let assign33830_e56523: f64 = (assign33830_e56521 * locals.var_mob0);
        let assign33830_e56526: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33830_e56528: f64 = (assign33830_e56526 * locals.var_dr0);
        let assign33830_e56529: f64 = (assign33830_e56523 / assign33830_e56528);
        (assign33830_e56529, ((((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn0)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn0)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn0)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn0 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn0)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn2)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn2)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn2)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn2 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn2)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn3)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn3)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn3)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn3 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn3)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn4)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn4)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn4)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn4 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn4)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn5)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn5)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn5)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn5 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn5)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn6)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn6)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn6)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn6 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn6)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn7)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn7)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn7)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn7 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn7)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn8)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn8)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn8)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn8 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn8)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn9)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn9)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn9)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn9 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn9)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn10)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn10)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn10)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn10 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn10)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn11)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn11)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn11)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn11 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn11)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn13)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn13)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn13)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn13 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn13)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn14)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn14)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn14)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn14 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn14)))) / (assign33830_e56528 * assign33830_e56528)),)
    } else {
        (locals.var_noigd0, locals.var_noigd0_dn0, locals.var_noigd0_dn2, locals.var_noigd0_dn3, locals.var_noigd0_dn4, locals.var_noigd0_dn5, locals.var_noigd0_dn6, locals.var_noigd0_dn7, locals.var_noigd0_dn8, locals.var_noigd0_dn9, locals.var_noigd0_dn10, locals.var_noigd0_dn11, locals.var_noigd0_dn13, locals.var_noigd0_dn14,)
    }
};
        locals.var_noigd0 = assign33830_e56531;
        locals.var_noigd0_dn0 = assign33830_e56531_d_n0;
        locals.var_noigd0_dn2 = assign33830_e56531_d_n2;
        locals.var_noigd0_dn3 = assign33830_e56531_d_n3;
        locals.var_noigd0_dn4 = assign33830_e56531_d_n4;
        locals.var_noigd0_dn5 = assign33830_e56531_d_n5;
        locals.var_noigd0_dn6 = assign33830_e56531_d_n6;
        locals.var_noigd0_dn7 = assign33830_e56531_d_n7;
        locals.var_noigd0_dn8 = assign33830_e56531_d_n8;
        locals.var_noigd0_dn9 = assign33830_e56531_d_n9;
        locals.var_noigd0_dn10 = assign33830_e56531_d_n10;
        locals.var_noigd0_dn11 = assign33830_e56531_d_n11;
        locals.var_noigd0_dn13 = assign33830_e56531_d_n13;
        locals.var_noigd0_dn14 = assign33830_e56531_d_n14;

        let (assign33840_e56540, assign33840_e56540_d_n0, assign33840_e56540_d_n2, assign33840_e56540_d_n3, assign33840_e56540_d_n4, assign33840_e56540_d_n5, assign33840_e56540_d_n6, assign33840_e56540_d_n7, assign33840_e56540_d_n8, assign33840_e56540_d_n9, assign33840_e56540_d_n10, assign33840_e56540_d_n11, assign33840_e56540_d_n13, assign33840_e56540_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33840_e56538: f64 = (1.0 + locals.var_noieta);
        (assign33840_e56538, locals.var_noieta_dn0, locals.var_noieta_dn2, locals.var_noieta_dn3, locals.var_noieta_dn4, locals.var_noieta_dn5, locals.var_noieta_dn6, locals.var_noieta_dn7, locals.var_noieta_dn8, locals.var_noieta_dn9, locals.var_noieta_dn10, locals.var_noieta_dn11, locals.var_noieta_dn13, locals.var_noieta_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33840_e56540;
        locals.var_t4_dn0 = assign33840_e56540_d_n0;
        locals.var_t4_dn2 = assign33840_e56540_d_n2;
        locals.var_t4_dn3 = assign33840_e56540_d_n3;
        locals.var_t4_dn4 = assign33840_e56540_d_n4;
        locals.var_t4_dn5 = assign33840_e56540_d_n5;
        locals.var_t4_dn6 = assign33840_e56540_d_n6;
        locals.var_t4_dn7 = assign33840_e56540_d_n7;
        locals.var_t4_dn8 = assign33840_e56540_d_n8;
        locals.var_t4_dn9 = assign33840_e56540_d_n9;
        locals.var_t4_dn10 = assign33840_e56540_d_n10;
        locals.var_t4_dn11 = assign33840_e56540_d_n11;
        locals.var_t4_dn13 = assign33840_e56540_d_n13;
        locals.var_t4_dn14 = assign33840_e56540_d_n14;

        let (assign33850_e56549, assign33850_e56549_d_n0, assign33850_e56549_d_n2, assign33850_e56549_d_n3, assign33850_e56549_d_n4, assign33850_e56549_d_n5, assign33850_e56549_d_n6, assign33850_e56549_d_n7, assign33850_e56549_d_n8, assign33850_e56549_d_n9, assign33850_e56549_d_n10, assign33850_e56549_d_n11, assign33850_e56549_d_n13, assign33850_e56549_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33850_e56547: f64 = (1.0 - locals.var_noieta);
        (assign33850_e56547, (-locals.var_noieta_dn0), (-locals.var_noieta_dn2), (-locals.var_noieta_dn3), (-locals.var_noieta_dn4), (-locals.var_noieta_dn5), (-locals.var_noieta_dn6), (-locals.var_noieta_dn7), (-locals.var_noieta_dn8), (-locals.var_noieta_dn9), (-locals.var_noieta_dn10), (-locals.var_noieta_dn11), (-locals.var_noieta_dn13), (-locals.var_noieta_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33850_e56549;
        locals.var_t5_dn0 = assign33850_e56549_d_n0;
        locals.var_t5_dn2 = assign33850_e56549_d_n2;
        locals.var_t5_dn3 = assign33850_e56549_d_n3;
        locals.var_t5_dn4 = assign33850_e56549_d_n4;
        locals.var_t5_dn5 = assign33850_e56549_d_n5;
        locals.var_t5_dn6 = assign33850_e56549_d_n6;
        locals.var_t5_dn7 = assign33850_e56549_d_n7;
        locals.var_t5_dn8 = assign33850_e56549_d_n8;
        locals.var_t5_dn9 = assign33850_e56549_d_n9;
        locals.var_t5_dn10 = assign33850_e56549_d_n10;
        locals.var_t5_dn11 = assign33850_e56549_d_n11;
        locals.var_t5_dn13 = assign33850_e56549_d_n13;
        locals.var_t5_dn14 = assign33850_e56549_d_n14;

        let (assign33860_e56562, assign33860_e56562_d_n0, assign33860_e56562_d_n2, assign33860_e56562_d_n3, assign33860_e56562_d_n4, assign33860_e56562_d_n5, assign33860_e56562_d_n6, assign33860_e56562_d_n7, assign33860_e56562_d_n8, assign33860_e56562_d_n9, assign33860_e56562_d_n10, assign33860_e56562_d_n11, assign33860_e56562_d_n13, assign33860_e56562_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33860_e56556: f64 = (2.0 * locals.var_noiwi);
        let assign33860_e56558: f64 = (assign33860_e56556 / locals.var_qis);
        let assign33860_e56560: f64 = (assign33860_e56558 * locals.var_nvtm);
        (assign33860_e56560, ((((((2.0 * locals.var_noiwi_dn0) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn0)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn0)), ((((((2.0 * locals.var_noiwi_dn2) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn2)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn2)), ((((((2.0 * locals.var_noiwi_dn3) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn3)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn3)), ((((((2.0 * locals.var_noiwi_dn4) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn4)), ((((((2.0 * locals.var_noiwi_dn5) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn5)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn5)), ((((((2.0 * locals.var_noiwi_dn6) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn6)), ((((((2.0 * locals.var_noiwi_dn7) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn7)), ((((((2.0 * locals.var_noiwi_dn8) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn8)), ((((((2.0 * locals.var_noiwi_dn9) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn9)), ((((((2.0 * locals.var_noiwi_dn10) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn10)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn10)), ((((((2.0 * locals.var_noiwi_dn11) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn11)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn11)), ((((((2.0 * locals.var_noiwi_dn13) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn13)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn13)), ((((((2.0 * locals.var_noiwi_dn14) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn14)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33860_e56562;
        locals.var_t6_dn0 = assign33860_e56562_d_n0;
        locals.var_t6_dn2 = assign33860_e56562_d_n2;
        locals.var_t6_dn3 = assign33860_e56562_d_n3;
        locals.var_t6_dn4 = assign33860_e56562_d_n4;
        locals.var_t6_dn5 = assign33860_e56562_d_n5;
        locals.var_t6_dn6 = assign33860_e56562_d_n6;
        locals.var_t6_dn7 = assign33860_e56562_d_n7;
        locals.var_t6_dn8 = assign33860_e56562_d_n8;
        locals.var_t6_dn9 = assign33860_e56562_d_n9;
        locals.var_t6_dn10 = assign33860_e56562_d_n10;
        locals.var_t6_dn11 = assign33860_e56562_d_n11;
        locals.var_t6_dn13 = assign33860_e56562_d_n13;
        locals.var_t6_dn14 = assign33860_e56562_d_n14;

        let (assign33870_e56571, assign33870_e56571_d_n0, assign33870_e56571_d_n2, assign33870_e56571_d_n3, assign33870_e56571_d_n4, assign33870_e56571_d_n5, assign33870_e56571_d_n6, assign33870_e56571_d_n7, assign33870_e56571_d_n8, assign33870_e56571_d_n9, assign33870_e56571_d_n10, assign33870_e56571_d_n11, assign33870_e56571_d_n13, assign33870_e56571_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33870_e56569: f64 = (locals.var_t4 + locals.var_t6);
        (assign33870_e56569, (locals.var_t4_dn0 + locals.var_t6_dn0), (locals.var_t4_dn2 + locals.var_t6_dn2), (locals.var_t4_dn3 + locals.var_t6_dn3), (locals.var_t4_dn4 + locals.var_t6_dn4), (locals.var_t4_dn5 + locals.var_t6_dn5), (locals.var_t4_dn6 + locals.var_t6_dn6), (locals.var_t4_dn7 + locals.var_t6_dn7), (locals.var_t4_dn8 + locals.var_t6_dn8), (locals.var_t4_dn9 + locals.var_t6_dn9), (locals.var_t4_dn10 + locals.var_t6_dn10), (locals.var_t4_dn11 + locals.var_t6_dn11), (locals.var_t4_dn13 + locals.var_t6_dn13), (locals.var_t4_dn14 + locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33870_e56571;
        locals.var_t7_dn0 = assign33870_e56571_d_n0;
        locals.var_t7_dn2 = assign33870_e56571_d_n2;
        locals.var_t7_dn3 = assign33870_e56571_d_n3;
        locals.var_t7_dn4 = assign33870_e56571_d_n4;
        locals.var_t7_dn5 = assign33870_e56571_d_n5;
        locals.var_t7_dn6 = assign33870_e56571_d_n6;
        locals.var_t7_dn7 = assign33870_e56571_d_n7;
        locals.var_t7_dn8 = assign33870_e56571_d_n8;
        locals.var_t7_dn9 = assign33870_e56571_d_n9;
        locals.var_t7_dn10 = assign33870_e56571_d_n10;
        locals.var_t7_dn11 = assign33870_e56571_d_n11;
        locals.var_t7_dn13 = assign33870_e56571_d_n13;
        locals.var_t7_dn14 = assign33870_e56571_d_n14;

        let (assign33880_e56580, assign33880_e56580_d_n0, assign33880_e56580_d_n2, assign33880_e56580_d_n3, assign33880_e56580_d_n4, assign33880_e56580_d_n5, assign33880_e56580_d_n6, assign33880_e56580_d_n7, assign33880_e56580_d_n8, assign33880_e56580_d_n9, assign33880_e56580_d_n10, assign33880_e56580_d_n11, assign33880_e56580_d_n13, assign33880_e56580_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33880_e56578: f64 = (locals.var_t5 * locals.var_t5);
        (assign33880_e56578, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)), ((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_2, locals.var_t5_2_dn0, locals.var_t5_2_dn2, locals.var_t5_2_dn3, locals.var_t5_2_dn4, locals.var_t5_2_dn5, locals.var_t5_2_dn6, locals.var_t5_2_dn7, locals.var_t5_2_dn8, locals.var_t5_2_dn9, locals.var_t5_2_dn10, locals.var_t5_2_dn11, locals.var_t5_2_dn13, locals.var_t5_2_dn14,)
    }
};
        locals.var_t5_2 = assign33880_e56580;
        locals.var_t5_2_dn0 = assign33880_e56580_d_n0;
        locals.var_t5_2_dn2 = assign33880_e56580_d_n2;
        locals.var_t5_2_dn3 = assign33880_e56580_d_n3;
        locals.var_t5_2_dn4 = assign33880_e56580_d_n4;
        locals.var_t5_2_dn5 = assign33880_e56580_d_n5;
        locals.var_t5_2_dn6 = assign33880_e56580_d_n6;
        locals.var_t5_2_dn7 = assign33880_e56580_d_n7;
        locals.var_t5_2_dn8 = assign33880_e56580_d_n8;
        locals.var_t5_2_dn9 = assign33880_e56580_d_n9;
        locals.var_t5_2_dn10 = assign33880_e56580_d_n10;
        locals.var_t5_2_dn11 = assign33880_e56580_d_n11;
        locals.var_t5_2_dn13 = assign33880_e56580_d_n13;
        locals.var_t5_2_dn14 = assign33880_e56580_d_n14;

        let (assign33890_e56589, assign33890_e56589_d_n0, assign33890_e56589_d_n2, assign33890_e56589_d_n3, assign33890_e56589_d_n4, assign33890_e56589_d_n5, assign33890_e56589_d_n6, assign33890_e56589_d_n7, assign33890_e56589_d_n8, assign33890_e56589_d_n9, assign33890_e56589_d_n10, assign33890_e56589_d_n11, assign33890_e56589_d_n13, assign33890_e56589_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33890_e56587: f64 = (locals.var_t5_2 * locals.var_t5);
        (assign33890_e56587, ((locals.var_t5_2_dn0 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn0)), ((locals.var_t5_2_dn2 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn2)), ((locals.var_t5_2_dn3 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn3)), ((locals.var_t5_2_dn4 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn4)), ((locals.var_t5_2_dn5 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn5)), ((locals.var_t5_2_dn6 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn6)), ((locals.var_t5_2_dn7 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn7)), ((locals.var_t5_2_dn8 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn8)), ((locals.var_t5_2_dn9 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn9)), ((locals.var_t5_2_dn10 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn10)), ((locals.var_t5_2_dn11 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn11)), ((locals.var_t5_2_dn13 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn13)), ((locals.var_t5_2_dn14 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_3, locals.var_t5_3_dn0, locals.var_t5_3_dn2, locals.var_t5_3_dn3, locals.var_t5_3_dn4, locals.var_t5_3_dn5, locals.var_t5_3_dn6, locals.var_t5_3_dn7, locals.var_t5_3_dn8, locals.var_t5_3_dn9, locals.var_t5_3_dn10, locals.var_t5_3_dn11, locals.var_t5_3_dn13, locals.var_t5_3_dn14,)
    }
};
        locals.var_t5_3 = assign33890_e56589;
        locals.var_t5_3_dn0 = assign33890_e56589_d_n0;
        locals.var_t5_3_dn2 = assign33890_e56589_d_n2;
        locals.var_t5_3_dn3 = assign33890_e56589_d_n3;
        locals.var_t5_3_dn4 = assign33890_e56589_d_n4;
        locals.var_t5_3_dn5 = assign33890_e56589_d_n5;
        locals.var_t5_3_dn6 = assign33890_e56589_d_n6;
        locals.var_t5_3_dn7 = assign33890_e56589_d_n7;
        locals.var_t5_3_dn8 = assign33890_e56589_d_n8;
        locals.var_t5_3_dn9 = assign33890_e56589_d_n9;
        locals.var_t5_3_dn10 = assign33890_e56589_d_n10;
        locals.var_t5_3_dn11 = assign33890_e56589_d_n11;
        locals.var_t5_3_dn13 = assign33890_e56589_d_n13;
        locals.var_t5_3_dn14 = assign33890_e56589_d_n14;

        let (assign33900_e56598, assign33900_e56598_d_n0, assign33900_e56598_d_n2, assign33900_e56598_d_n3, assign33900_e56598_d_n4, assign33900_e56598_d_n5, assign33900_e56598_d_n6, assign33900_e56598_d_n7, assign33900_e56598_d_n8, assign33900_e56598_d_n9, assign33900_e56598_d_n10, assign33900_e56598_d_n11, assign33900_e56598_d_n13, assign33900_e56598_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33900_e56596: f64 = (locals.var_t5_3 * locals.var_t5);
        (assign33900_e56596, ((locals.var_t5_3_dn0 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn0)), ((locals.var_t5_3_dn2 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn2)), ((locals.var_t5_3_dn3 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn3)), ((locals.var_t5_3_dn4 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn4)), ((locals.var_t5_3_dn5 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn5)), ((locals.var_t5_3_dn6 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn6)), ((locals.var_t5_3_dn7 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn7)), ((locals.var_t5_3_dn8 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn8)), ((locals.var_t5_3_dn9 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn9)), ((locals.var_t5_3_dn10 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn10)), ((locals.var_t5_3_dn11 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn11)), ((locals.var_t5_3_dn13 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn13)), ((locals.var_t5_3_dn14 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_4, locals.var_t5_4_dn0, locals.var_t5_4_dn2, locals.var_t5_4_dn3, locals.var_t5_4_dn4, locals.var_t5_4_dn5, locals.var_t5_4_dn6, locals.var_t5_4_dn7, locals.var_t5_4_dn8, locals.var_t5_4_dn9, locals.var_t5_4_dn10, locals.var_t5_4_dn11, locals.var_t5_4_dn13, locals.var_t5_4_dn14,)
    }
};
        locals.var_t5_4 = assign33900_e56598;
        locals.var_t5_4_dn0 = assign33900_e56598_d_n0;
        locals.var_t5_4_dn2 = assign33900_e56598_d_n2;
        locals.var_t5_4_dn3 = assign33900_e56598_d_n3;
        locals.var_t5_4_dn4 = assign33900_e56598_d_n4;
        locals.var_t5_4_dn5 = assign33900_e56598_d_n5;
        locals.var_t5_4_dn6 = assign33900_e56598_d_n6;
        locals.var_t5_4_dn7 = assign33900_e56598_d_n7;
        locals.var_t5_4_dn8 = assign33900_e56598_d_n8;
        locals.var_t5_4_dn9 = assign33900_e56598_d_n9;
        locals.var_t5_4_dn10 = assign33900_e56598_d_n10;
        locals.var_t5_4_dn11 = assign33900_e56598_d_n11;
        locals.var_t5_4_dn13 = assign33900_e56598_d_n13;
        locals.var_t5_4_dn14 = assign33900_e56598_d_n14;

        let (assign33910_e56607, assign33910_e56607_d_n0, assign33910_e56607_d_n2, assign33910_e56607_d_n3, assign33910_e56607_d_n4, assign33910_e56607_d_n5, assign33910_e56607_d_n6, assign33910_e56607_d_n7, assign33910_e56607_d_n8, assign33910_e56607_d_n9, assign33910_e56607_d_n10, assign33910_e56607_d_n11, assign33910_e56607_d_n13, assign33910_e56607_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33910_e56605: f64 = (locals.var_t7 * locals.var_t7);
        (assign33910_e56605, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)), ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)), ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_2, locals.var_t7_2_dn0, locals.var_t7_2_dn2, locals.var_t7_2_dn3, locals.var_t7_2_dn4, locals.var_t7_2_dn5, locals.var_t7_2_dn6, locals.var_t7_2_dn7, locals.var_t7_2_dn8, locals.var_t7_2_dn9, locals.var_t7_2_dn10, locals.var_t7_2_dn11, locals.var_t7_2_dn13, locals.var_t7_2_dn14,)
    }
};
        locals.var_t7_2 = assign33910_e56607;
        locals.var_t7_2_dn0 = assign33910_e56607_d_n0;
        locals.var_t7_2_dn2 = assign33910_e56607_d_n2;
        locals.var_t7_2_dn3 = assign33910_e56607_d_n3;
        locals.var_t7_2_dn4 = assign33910_e56607_d_n4;
        locals.var_t7_2_dn5 = assign33910_e56607_d_n5;
        locals.var_t7_2_dn6 = assign33910_e56607_d_n6;
        locals.var_t7_2_dn7 = assign33910_e56607_d_n7;
        locals.var_t7_2_dn8 = assign33910_e56607_d_n8;
        locals.var_t7_2_dn9 = assign33910_e56607_d_n9;
        locals.var_t7_2_dn10 = assign33910_e56607_d_n10;
        locals.var_t7_2_dn11 = assign33910_e56607_d_n11;
        locals.var_t7_2_dn13 = assign33910_e56607_d_n13;
        locals.var_t7_2_dn14 = assign33910_e56607_d_n14;

        let (assign33920_e56616, assign33920_e56616_d_n0, assign33920_e56616_d_n2, assign33920_e56616_d_n3, assign33920_e56616_d_n4, assign33920_e56616_d_n5, assign33920_e56616_d_n6, assign33920_e56616_d_n7, assign33920_e56616_d_n8, assign33920_e56616_d_n9, assign33920_e56616_d_n10, assign33920_e56616_d_n11, assign33920_e56616_d_n13, assign33920_e56616_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33920_e56614: f64 = (locals.var_t7_2 * locals.var_t7);
        (assign33920_e56614, ((locals.var_t7_2_dn0 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn0)), ((locals.var_t7_2_dn2 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn2)), ((locals.var_t7_2_dn3 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn3)), ((locals.var_t7_2_dn4 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn4)), ((locals.var_t7_2_dn5 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn5)), ((locals.var_t7_2_dn6 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn6)), ((locals.var_t7_2_dn7 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn7)), ((locals.var_t7_2_dn8 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn8)), ((locals.var_t7_2_dn9 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn9)), ((locals.var_t7_2_dn10 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn10)), ((locals.var_t7_2_dn11 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn11)), ((locals.var_t7_2_dn13 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn13)), ((locals.var_t7_2_dn14 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_3, locals.var_t7_3_dn0, locals.var_t7_3_dn2, locals.var_t7_3_dn3, locals.var_t7_3_dn4, locals.var_t7_3_dn5, locals.var_t7_3_dn6, locals.var_t7_3_dn7, locals.var_t7_3_dn8, locals.var_t7_3_dn9, locals.var_t7_3_dn10, locals.var_t7_3_dn11, locals.var_t7_3_dn13, locals.var_t7_3_dn14,)
    }
};
        locals.var_t7_3 = assign33920_e56616;
        locals.var_t7_3_dn0 = assign33920_e56616_d_n0;
        locals.var_t7_3_dn2 = assign33920_e56616_d_n2;
        locals.var_t7_3_dn3 = assign33920_e56616_d_n3;
        locals.var_t7_3_dn4 = assign33920_e56616_d_n4;
        locals.var_t7_3_dn5 = assign33920_e56616_d_n5;
        locals.var_t7_3_dn6 = assign33920_e56616_d_n6;
        locals.var_t7_3_dn7 = assign33920_e56616_d_n7;
        locals.var_t7_3_dn8 = assign33920_e56616_d_n8;
        locals.var_t7_3_dn9 = assign33920_e56616_d_n9;
        locals.var_t7_3_dn10 = assign33920_e56616_d_n10;
        locals.var_t7_3_dn11 = assign33920_e56616_d_n11;
        locals.var_t7_3_dn13 = assign33920_e56616_d_n13;
        locals.var_t7_3_dn14 = assign33920_e56616_d_n14;

        let (assign33930_e56625, assign33930_e56625_d_n0, assign33930_e56625_d_n2, assign33930_e56625_d_n3, assign33930_e56625_d_n4, assign33930_e56625_d_n5, assign33930_e56625_d_n6, assign33930_e56625_d_n7, assign33930_e56625_d_n8, assign33930_e56625_d_n9, assign33930_e56625_d_n10, assign33930_e56625_d_n11, assign33930_e56625_d_n13, assign33930_e56625_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33930_e56623: f64 = (locals.var_t7_3 * locals.var_t7);
        (assign33930_e56623, ((locals.var_t7_3_dn0 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn0)), ((locals.var_t7_3_dn2 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn2)), ((locals.var_t7_3_dn3 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn3)), ((locals.var_t7_3_dn4 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn4)), ((locals.var_t7_3_dn5 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn5)), ((locals.var_t7_3_dn6 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn6)), ((locals.var_t7_3_dn7 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn7)), ((locals.var_t7_3_dn8 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn8)), ((locals.var_t7_3_dn9 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn9)), ((locals.var_t7_3_dn10 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn10)), ((locals.var_t7_3_dn11 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn11)), ((locals.var_t7_3_dn13 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn13)), ((locals.var_t7_3_dn14 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_4, locals.var_t7_4_dn0, locals.var_t7_4_dn2, locals.var_t7_4_dn3, locals.var_t7_4_dn4, locals.var_t7_4_dn5, locals.var_t7_4_dn6, locals.var_t7_4_dn7, locals.var_t7_4_dn8, locals.var_t7_4_dn9, locals.var_t7_4_dn10, locals.var_t7_4_dn11, locals.var_t7_4_dn13, locals.var_t7_4_dn14,)
    }
};
        locals.var_t7_4 = assign33930_e56625;
        locals.var_t7_4_dn0 = assign33930_e56625_d_n0;
        locals.var_t7_4_dn2 = assign33930_e56625_d_n2;
        locals.var_t7_4_dn3 = assign33930_e56625_d_n3;
        locals.var_t7_4_dn4 = assign33930_e56625_d_n4;
        locals.var_t7_4_dn5 = assign33930_e56625_d_n5;
        locals.var_t7_4_dn6 = assign33930_e56625_d_n6;
        locals.var_t7_4_dn7 = assign33930_e56625_d_n7;
        locals.var_t7_4_dn8 = assign33930_e56625_d_n8;
        locals.var_t7_4_dn9 = assign33930_e56625_d_n9;
        locals.var_t7_4_dn10 = assign33930_e56625_d_n10;
        locals.var_t7_4_dn11 = assign33930_e56625_d_n11;
        locals.var_t7_4_dn13 = assign33930_e56625_d_n13;
        locals.var_t7_4_dn14 = assign33930_e56625_d_n14;

        let (assign33940_e56634, assign33940_e56634_d_n0, assign33940_e56634_d_n2, assign33940_e56634_d_n3, assign33940_e56634_d_n4, assign33940_e56634_d_n5, assign33940_e56634_d_n6, assign33940_e56634_d_n7, assign33940_e56634_d_n8, assign33940_e56634_d_n9, assign33940_e56634_d_n10, assign33940_e56634_d_n11, assign33940_e56634_d_n13, assign33940_e56634_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33940_e56632: f64 = (locals.var_t7_4 * locals.var_t7);
        (assign33940_e56632, ((locals.var_t7_4_dn0 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn0)), ((locals.var_t7_4_dn2 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn2)), ((locals.var_t7_4_dn3 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn3)), ((locals.var_t7_4_dn4 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn4)), ((locals.var_t7_4_dn5 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn5)), ((locals.var_t7_4_dn6 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn6)), ((locals.var_t7_4_dn7 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn7)), ((locals.var_t7_4_dn8 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn8)), ((locals.var_t7_4_dn9 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn9)), ((locals.var_t7_4_dn10 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn10)), ((locals.var_t7_4_dn11 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn11)), ((locals.var_t7_4_dn13 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn13)), ((locals.var_t7_4_dn14 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_5, locals.var_t7_5_dn0, locals.var_t7_5_dn2, locals.var_t7_5_dn3, locals.var_t7_5_dn4, locals.var_t7_5_dn5, locals.var_t7_5_dn6, locals.var_t7_5_dn7, locals.var_t7_5_dn8, locals.var_t7_5_dn9, locals.var_t7_5_dn10, locals.var_t7_5_dn11, locals.var_t7_5_dn13, locals.var_t7_5_dn14,)
    }
};
        locals.var_t7_5 = assign33940_e56634;
        locals.var_t7_5_dn0 = assign33940_e56634_d_n0;
        locals.var_t7_5_dn2 = assign33940_e56634_d_n2;
        locals.var_t7_5_dn3 = assign33940_e56634_d_n3;
        locals.var_t7_5_dn4 = assign33940_e56634_d_n4;
        locals.var_t7_5_dn5 = assign33940_e56634_d_n5;
        locals.var_t7_5_dn6 = assign33940_e56634_d_n6;
        locals.var_t7_5_dn7 = assign33940_e56634_d_n7;
        locals.var_t7_5_dn8 = assign33940_e56634_d_n8;
        locals.var_t7_5_dn9 = assign33940_e56634_d_n9;
        locals.var_t7_5_dn10 = assign33940_e56634_d_n10;
        locals.var_t7_5_dn11 = assign33940_e56634_d_n11;
        locals.var_t7_5_dn13 = assign33940_e56634_d_n13;
        locals.var_t7_5_dn14 = assign33940_e56634_d_n14;

    }

    pub(super) fn stamp_transient_block_131(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33950_e56643, assign33950_e56643_d_n0, assign33950_e56643_d_n2, assign33950_e56643_d_n3, assign33950_e56643_d_n4, assign33950_e56643_d_n5, assign33950_e56643_d_n6, assign33950_e56643_d_n7, assign33950_e56643_d_n8, assign33950_e56643_d_n9, assign33950_e56643_d_n10, assign33950_e56643_d_n11, assign33950_e56643_d_n13, assign33950_e56643_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33950_e56641: f64 = (0.5 * locals.var_t4);
        (assign33950_e56641, (0.5 * locals.var_t4_dn0), (0.5 * locals.var_t4_dn2), (0.5 * locals.var_t4_dn3), (0.5 * locals.var_t4_dn4), (0.5 * locals.var_t4_dn5), (0.5 * locals.var_t4_dn6), (0.5 * locals.var_t4_dn7), (0.5 * locals.var_t4_dn8), (0.5 * locals.var_t4_dn9), (0.5 * locals.var_t4_dn10), (0.5 * locals.var_t4_dn11), (0.5 * locals.var_t4_dn13), (0.5 * locals.var_t4_dn14),)
    } else {
        (locals.var_gamma1, locals.var_gamma1_dn0, locals.var_gamma1_dn2, locals.var_gamma1_dn3, locals.var_gamma1_dn4, locals.var_gamma1_dn5, locals.var_gamma1_dn6, locals.var_gamma1_dn7, locals.var_gamma1_dn8, locals.var_gamma1_dn9, locals.var_gamma1_dn10, locals.var_gamma1_dn11, locals.var_gamma1_dn13, locals.var_gamma1_dn14,)
    }
};
        locals.var_gamma1 = assign33950_e56643;
        locals.var_gamma1_dn0 = assign33950_e56643_d_n0;
        locals.var_gamma1_dn2 = assign33950_e56643_d_n2;
        locals.var_gamma1_dn3 = assign33950_e56643_d_n3;
        locals.var_gamma1_dn4 = assign33950_e56643_d_n4;
        locals.var_gamma1_dn5 = assign33950_e56643_d_n5;
        locals.var_gamma1_dn6 = assign33950_e56643_d_n6;
        locals.var_gamma1_dn7 = assign33950_e56643_d_n7;
        locals.var_gamma1_dn8 = assign33950_e56643_d_n8;
        locals.var_gamma1_dn9 = assign33950_e56643_d_n9;
        locals.var_gamma1_dn10 = assign33950_e56643_d_n10;
        locals.var_gamma1_dn11 = assign33950_e56643_d_n11;
        locals.var_gamma1_dn13 = assign33950_e56643_d_n13;
        locals.var_gamma1_dn14 = assign33950_e56643_d_n14;

        let (assign33960_e56654, assign33960_e56654_d_n0, assign33960_e56654_d_n2, assign33960_e56654_d_n3, assign33960_e56654_d_n4, assign33960_e56654_d_n5, assign33960_e56654_d_n6, assign33960_e56654_d_n7, assign33960_e56654_d_n8, assign33960_e56654_d_n9, assign33960_e56654_d_n10, assign33960_e56654_d_n11, assign33960_e56654_d_n13, assign33960_e56654_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33960_e56651: f64 = (6.0 * locals.var_t7);
        let assign33960_e56652: f64 = (locals.var_t5_2 / assign33960_e56651);
        (assign33960_e56652, (((locals.var_t5_2_dn0 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn0))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn2 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn2))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn3 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn3))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn4 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn4))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn5 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn5))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn6 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn6))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn7 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn7))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn8 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn8))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn9 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn9))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn10 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn10))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn11 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn11))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn13 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn13))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn14 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn14))) / (assign33960_e56651 * assign33960_e56651)),)
    } else {
        (locals.var_gamma2, locals.var_gamma2_dn0, locals.var_gamma2_dn2, locals.var_gamma2_dn3, locals.var_gamma2_dn4, locals.var_gamma2_dn5, locals.var_gamma2_dn6, locals.var_gamma2_dn7, locals.var_gamma2_dn8, locals.var_gamma2_dn9, locals.var_gamma2_dn10, locals.var_gamma2_dn11, locals.var_gamma2_dn13, locals.var_gamma2_dn14,)
    }
};
        locals.var_gamma2 = assign33960_e56654;
        locals.var_gamma2_dn0 = assign33960_e56654_d_n0;
        locals.var_gamma2_dn2 = assign33960_e56654_d_n2;
        locals.var_gamma2_dn3 = assign33960_e56654_d_n3;
        locals.var_gamma2_dn4 = assign33960_e56654_d_n4;
        locals.var_gamma2_dn5 = assign33960_e56654_d_n5;
        locals.var_gamma2_dn6 = assign33960_e56654_d_n6;
        locals.var_gamma2_dn7 = assign33960_e56654_d_n7;
        locals.var_gamma2_dn8 = assign33960_e56654_d_n8;
        locals.var_gamma2_dn9 = assign33960_e56654_d_n9;
        locals.var_gamma2_dn10 = assign33960_e56654_d_n10;
        locals.var_gamma2_dn11 = assign33960_e56654_d_n11;
        locals.var_gamma2_dn13 = assign33960_e56654_d_n13;
        locals.var_gamma2_dn14 = assign33960_e56654_d_n14;

        let (assign33970_e56667, assign33970_e56667_d_n0, assign33970_e56667_d_n2, assign33970_e56667_d_n3, assign33970_e56667_d_n4, assign33970_e56667_d_n5, assign33970_e56667_d_n6, assign33970_e56667_d_n7, assign33970_e56667_d_n8, assign33970_e56667_d_n9, assign33970_e56667_d_n10, assign33970_e56667_d_n11, assign33970_e56667_d_n13, assign33970_e56667_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33970_e56661: f64 = (locals.var_moc / locals.var_dvsat);
        let assign33970_e56664: f64 = (locals.var_gamma1 + locals.var_gamma2);
        let assign33970_e56665: f64 = (assign33970_e56661 * assign33970_e56664);
        (assign33970_e56665, (((((locals.var_moc_dn0 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn0)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn0 + locals.var_gamma2_dn0))), (((((locals.var_moc_dn2 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn2)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn2 + locals.var_gamma2_dn2))), (((((locals.var_moc_dn3 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn3 + locals.var_gamma2_dn3))), (((((locals.var_moc_dn4 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn4 + locals.var_gamma2_dn4))), (((((locals.var_moc_dn5 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn5 + locals.var_gamma2_dn5))), (((((locals.var_moc_dn6 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn6 + locals.var_gamma2_dn6))), (((((locals.var_moc_dn7 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn7 + locals.var_gamma2_dn7))), (((((locals.var_moc_dn8 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn8 + locals.var_gamma2_dn8))), (((((locals.var_moc_dn9 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn9 + locals.var_gamma2_dn9))), (((((locals.var_moc_dn10 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn10 + locals.var_gamma2_dn10))), (((((locals.var_moc_dn11 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn11 + locals.var_gamma2_dn11))), (((((locals.var_moc_dn13 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn13)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn13 + locals.var_gamma2_dn13))), (((((locals.var_moc_dn14 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn14)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn14 + locals.var_gamma2_dn14))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn3, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn13, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign33970_e56667;
        locals.var_gamma_dn0 = assign33970_e56667_d_n0;
        locals.var_gamma_dn2 = assign33970_e56667_d_n2;
        locals.var_gamma_dn3 = assign33970_e56667_d_n3;
        locals.var_gamma_dn4 = assign33970_e56667_d_n4;
        locals.var_gamma_dn5 = assign33970_e56667_d_n5;
        locals.var_gamma_dn6 = assign33970_e56667_d_n6;
        locals.var_gamma_dn7 = assign33970_e56667_d_n7;
        locals.var_gamma_dn8 = assign33970_e56667_d_n8;
        locals.var_gamma_dn9 = assign33970_e56667_d_n9;
        locals.var_gamma_dn10 = assign33970_e56667_d_n10;
        locals.var_gamma_dn11 = assign33970_e56667_d_n11;
        locals.var_gamma_dn13 = assign33970_e56667_d_n13;
        locals.var_gamma_dn14 = assign33970_e56667_d_n14;

        let (assign33980_e56676, assign33980_e56676_d_n0, assign33980_e56676_d_n2, assign33980_e56676_d_n3, assign33980_e56676_d_n4, assign33980_e56676_d_n5, assign33980_e56676_d_n6, assign33980_e56676_d_n7, assign33980_e56676_d_n8, assign33980_e56676_d_n9, assign33980_e56676_d_n10, assign33980_e56676_d_n11, assign33980_e56676_d_n13, assign33980_e56676_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33980_e56674: f64 = (locals.var_t4 / locals.var_t7_2);
        (assign33980_e56674, (((locals.var_t4_dn0 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn0)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn2 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn2)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn3 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn3)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn4 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn4)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn5 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn5)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn6 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn6)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn7 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn7)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn8 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn8)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn9 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn9)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn10 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn10)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn11 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn11)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn13 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn13)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn14 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn14)) / (locals.var_t7_2 * locals.var_t7_2)),)
    } else {
        (locals.var_delta1, locals.var_delta1_dn0, locals.var_delta1_dn2, locals.var_delta1_dn3, locals.var_delta1_dn4, locals.var_delta1_dn5, locals.var_delta1_dn6, locals.var_delta1_dn7, locals.var_delta1_dn8, locals.var_delta1_dn9, locals.var_delta1_dn10, locals.var_delta1_dn11, locals.var_delta1_dn13, locals.var_delta1_dn14,)
    }
};
        locals.var_delta1 = assign33980_e56676;
        locals.var_delta1_dn0 = assign33980_e56676_d_n0;
        locals.var_delta1_dn2 = assign33980_e56676_d_n2;
        locals.var_delta1_dn3 = assign33980_e56676_d_n3;
        locals.var_delta1_dn4 = assign33980_e56676_d_n4;
        locals.var_delta1_dn5 = assign33980_e56676_d_n5;
        locals.var_delta1_dn6 = assign33980_e56676_d_n6;
        locals.var_delta1_dn7 = assign33980_e56676_d_n7;
        locals.var_delta1_dn8 = assign33980_e56676_d_n8;
        locals.var_delta1_dn9 = assign33980_e56676_d_n9;
        locals.var_delta1_dn10 = assign33980_e56676_d_n10;
        locals.var_delta1_dn11 = assign33980_e56676_d_n11;
        locals.var_delta1_dn13 = assign33980_e56676_d_n13;
        locals.var_delta1_dn14 = assign33980_e56676_d_n14;

        let (assign33990_e56693, assign33990_e56693_d_n0, assign33990_e56693_d_n2, assign33990_e56693_d_n3, assign33990_e56693_d_n4, assign33990_e56693_d_n5, assign33990_e56693_d_n6, assign33990_e56693_d_n7, assign33990_e56693_d_n8, assign33990_e56693_d_n9, assign33990_e56693_d_n10, assign33990_e56693_d_n11, assign33990_e56693_d_n13, assign33990_e56693_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33990_e56683: f64 = (6.0 * locals.var_t4);
        let assign33990_e56685: f64 = (assign33990_e56683 + locals.var_t6);
        let assign33990_e56687: f64 = (assign33990_e56685 * locals.var_t5_2);
        let assign33990_e56690: f64 = (15.0 * locals.var_t7_4);
        let assign33990_e56691: f64 = (assign33990_e56687 / assign33990_e56690);
        (assign33990_e56691, (((((((6.0 * locals.var_t4_dn0) + locals.var_t6_dn0) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn0)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn0))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn2) + locals.var_t6_dn2) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn2)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn2))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn3) + locals.var_t6_dn3) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn3)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn3))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn4) + locals.var_t6_dn4) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn4)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn4))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn5) + locals.var_t6_dn5) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn5)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn5))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn6) + locals.var_t6_dn6) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn6)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn6))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn7) + locals.var_t6_dn7) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn7)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn7))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn8) + locals.var_t6_dn8) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn8)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn8))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn9) + locals.var_t6_dn9) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn9)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn9))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn10) + locals.var_t6_dn10) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn10)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn10))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn11) + locals.var_t6_dn11) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn11)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn11))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn13) + locals.var_t6_dn13) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn13)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn13))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn14) + locals.var_t6_dn14) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn14)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn14))) / (assign33990_e56690 * assign33990_e56690)),)
    } else {
        (locals.var_delta2, locals.var_delta2_dn0, locals.var_delta2_dn2, locals.var_delta2_dn3, locals.var_delta2_dn4, locals.var_delta2_dn5, locals.var_delta2_dn6, locals.var_delta2_dn7, locals.var_delta2_dn8, locals.var_delta2_dn9, locals.var_delta2_dn10, locals.var_delta2_dn11, locals.var_delta2_dn13, locals.var_delta2_dn14,)
    }
};
        locals.var_delta2 = assign33990_e56693;
        locals.var_delta2_dn0 = assign33990_e56693_d_n0;
        locals.var_delta2_dn2 = assign33990_e56693_d_n2;
        locals.var_delta2_dn3 = assign33990_e56693_d_n3;
        locals.var_delta2_dn4 = assign33990_e56693_d_n4;
        locals.var_delta2_dn5 = assign33990_e56693_d_n5;
        locals.var_delta2_dn6 = assign33990_e56693_d_n6;
        locals.var_delta2_dn7 = assign33990_e56693_d_n7;
        locals.var_delta2_dn8 = assign33990_e56693_d_n8;
        locals.var_delta2_dn9 = assign33990_e56693_d_n9;
        locals.var_delta2_dn10 = assign33990_e56693_d_n10;
        locals.var_delta2_dn11 = assign33990_e56693_d_n11;
        locals.var_delta2_dn13 = assign33990_e56693_d_n13;
        locals.var_delta2_dn14 = assign33990_e56693_d_n14;

        let (assign34000_e56704, assign34000_e56704_d_n0, assign34000_e56704_d_n2, assign34000_e56704_d_n3, assign34000_e56704_d_n4, assign34000_e56704_d_n5, assign34000_e56704_d_n6, assign34000_e56704_d_n7, assign34000_e56704_d_n8, assign34000_e56704_d_n9, assign34000_e56704_d_n10, assign34000_e56704_d_n11, assign34000_e56704_d_n13, assign34000_e56704_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34000_e56701: f64 = (9.0 * locals.var_t7_5);
        let assign34000_e56702: f64 = (locals.var_t5_4 / assign34000_e56701);
        (assign34000_e56702, (((locals.var_t5_4_dn0 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn0))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn2 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn2))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn3 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn3))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn4 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn4))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn5 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn5))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn6 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn6))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn7 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn7))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn8 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn8))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn9 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn9))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn10 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn10))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn11 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn11))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn13 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn13))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn14 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn14))) / (assign34000_e56701 * assign34000_e56701)),)
    } else {
        (locals.var_delta3, locals.var_delta3_dn0, locals.var_delta3_dn2, locals.var_delta3_dn3, locals.var_delta3_dn4, locals.var_delta3_dn5, locals.var_delta3_dn6, locals.var_delta3_dn7, locals.var_delta3_dn8, locals.var_delta3_dn9, locals.var_delta3_dn10, locals.var_delta3_dn11, locals.var_delta3_dn13, locals.var_delta3_dn14,)
    }
};
        locals.var_delta3 = assign34000_e56704;
        locals.var_delta3_dn0 = assign34000_e56704_d_n0;
        locals.var_delta3_dn2 = assign34000_e56704_d_n2;
        locals.var_delta3_dn3 = assign34000_e56704_d_n3;
        locals.var_delta3_dn4 = assign34000_e56704_d_n4;
        locals.var_delta3_dn5 = assign34000_e56704_d_n5;
        locals.var_delta3_dn6 = assign34000_e56704_d_n6;
        locals.var_delta3_dn7 = assign34000_e56704_d_n7;
        locals.var_delta3_dn8 = assign34000_e56704_d_n8;
        locals.var_delta3_dn9 = assign34000_e56704_d_n9;
        locals.var_delta3_dn10 = assign34000_e56704_d_n10;
        locals.var_delta3_dn11 = assign34000_e56704_d_n11;
        locals.var_delta3_dn13 = assign34000_e56704_d_n13;
        locals.var_delta3_dn14 = assign34000_e56704_d_n14;

        let (assign34010_e56721, assign34010_e56721_d_n0, assign34010_e56721_d_n2, assign34010_e56721_d_n3, assign34010_e56721_d_n4, assign34010_e56721_d_n5, assign34010_e56721_d_n6, assign34010_e56721_d_n7, assign34010_e56721_d_n8, assign34010_e56721_d_n9, assign34010_e56721_d_n10, assign34010_e56721_d_n11, assign34010_e56721_d_n13, assign34010_e56721_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34010_e56711: f64 = (locals.var_moc / 6.0);
        let assign34010_e56713: f64 = (assign34010_e56711 * locals.var_dvsat3);
        let assign34010_e56716: f64 = (locals.var_delta1 - locals.var_delta2);
        let assign34010_e56718: f64 = (assign34010_e56716 + locals.var_delta3);
        let assign34010_e56719: f64 = (assign34010_e56713 * assign34010_e56718);
        (assign34010_e56719, (((((locals.var_moc_dn0 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn0)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn0 - locals.var_delta2_dn0) + locals.var_delta3_dn0))), (((((locals.var_moc_dn2 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn2)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn2 - locals.var_delta2_dn2) + locals.var_delta3_dn2))), (((((locals.var_moc_dn3 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn3)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn3 - locals.var_delta2_dn3) + locals.var_delta3_dn3))), (((((locals.var_moc_dn4 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn4)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn4 - locals.var_delta2_dn4) + locals.var_delta3_dn4))), (((((locals.var_moc_dn5 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn5)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn5 - locals.var_delta2_dn5) + locals.var_delta3_dn5))), (((((locals.var_moc_dn6 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn6)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn6 - locals.var_delta2_dn6) + locals.var_delta3_dn6))), (((((locals.var_moc_dn7 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn7)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn7 - locals.var_delta2_dn7) + locals.var_delta3_dn7))), (((((locals.var_moc_dn8 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn8)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn8 - locals.var_delta2_dn8) + locals.var_delta3_dn8))), (((((locals.var_moc_dn9 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn9)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn9 - locals.var_delta2_dn9) + locals.var_delta3_dn9))), (((((locals.var_moc_dn10 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn10)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn10 - locals.var_delta2_dn10) + locals.var_delta3_dn10))), (((((locals.var_moc_dn11 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn11)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn11 - locals.var_delta2_dn11) + locals.var_delta3_dn11))), (((((locals.var_moc_dn13 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn13)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn13 - locals.var_delta2_dn13) + locals.var_delta3_dn13))), (((((locals.var_moc_dn14 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn14)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn14 - locals.var_delta2_dn14) + locals.var_delta3_dn14))),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign34010_e56721;
        locals.var_delta_dn0 = assign34010_e56721_d_n0;
        locals.var_delta_dn2 = assign34010_e56721_d_n2;
        locals.var_delta_dn3 = assign34010_e56721_d_n3;
        locals.var_delta_dn4 = assign34010_e56721_d_n4;
        locals.var_delta_dn5 = assign34010_e56721_d_n5;
        locals.var_delta_dn6 = assign34010_e56721_d_n6;
        locals.var_delta_dn7 = assign34010_e56721_d_n7;
        locals.var_delta_dn8 = assign34010_e56721_d_n8;
        locals.var_delta_dn9 = assign34010_e56721_d_n9;
        locals.var_delta_dn10 = assign34010_e56721_d_n10;
        locals.var_delta_dn11 = assign34010_e56721_d_n11;
        locals.var_delta_dn13 = assign34010_e56721_d_n13;
        locals.var_delta_dn14 = assign34010_e56721_d_n14;

        let (assign34020_e56730, assign34020_e56730_d_n0, assign34020_e56730_d_n2, assign34020_e56730_d_n3, assign34020_e56730_d_n4, assign34020_e56730_d_n5, assign34020_e56730_d_n6, assign34020_e56730_d_n7, assign34020_e56730_d_n8, assign34020_e56730_d_n9, assign34020_e56730_d_n10, assign34020_e56730_d_n11, assign34020_e56730_d_n13, assign34020_e56730_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34020_e56728: f64 = (locals.var_t5 / locals.var_t7);
        (assign34020_e56728, (((locals.var_t5_dn0 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn0)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn2 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn2)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn3 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn3)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn4 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn4)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn5 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn5)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn6 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn6)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn7 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn7)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn8 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn8)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn9 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn9)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn10 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn10)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn11 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn11)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn13 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn13)) / (locals.var_t7 * locals.var_t7)), (((locals.var_t5_dn14 * locals.var_t7) - (locals.var_t5 * locals.var_t7_dn14)) / (locals.var_t7 * locals.var_t7)),)
    } else {
        (locals.var_epsilon1, locals.var_epsilon1_dn0, locals.var_epsilon1_dn2, locals.var_epsilon1_dn3, locals.var_epsilon1_dn4, locals.var_epsilon1_dn5, locals.var_epsilon1_dn6, locals.var_epsilon1_dn7, locals.var_epsilon1_dn8, locals.var_epsilon1_dn9, locals.var_epsilon1_dn10, locals.var_epsilon1_dn11, locals.var_epsilon1_dn13, locals.var_epsilon1_dn14,)
    }
};
        locals.var_epsilon1 = assign34020_e56730;
        locals.var_epsilon1_dn0 = assign34020_e56730_d_n0;
        locals.var_epsilon1_dn2 = assign34020_e56730_d_n2;
        locals.var_epsilon1_dn3 = assign34020_e56730_d_n3;
        locals.var_epsilon1_dn4 = assign34020_e56730_d_n4;
        locals.var_epsilon1_dn5 = assign34020_e56730_d_n5;
        locals.var_epsilon1_dn6 = assign34020_e56730_d_n6;
        locals.var_epsilon1_dn7 = assign34020_e56730_d_n7;
        locals.var_epsilon1_dn8 = assign34020_e56730_d_n8;
        locals.var_epsilon1_dn9 = assign34020_e56730_d_n9;
        locals.var_epsilon1_dn10 = assign34020_e56730_d_n10;
        locals.var_epsilon1_dn11 = assign34020_e56730_d_n11;
        locals.var_epsilon1_dn13 = assign34020_e56730_d_n13;
        locals.var_epsilon1_dn14 = assign34020_e56730_d_n14;

        let (assign34030_e56741, assign34030_e56741_d_n0, assign34030_e56741_d_n2, assign34030_e56741_d_n3, assign34030_e56741_d_n4, assign34030_e56741_d_n5, assign34030_e56741_d_n6, assign34030_e56741_d_n7, assign34030_e56741_d_n8, assign34030_e56741_d_n9, assign34030_e56741_d_n10, assign34030_e56741_d_n11, assign34030_e56741_d_n13, assign34030_e56741_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34030_e56738: f64 = (3.0 * locals.var_t7_3);
        let assign34030_e56739: f64 = (locals.var_t5_3 / assign34030_e56738);
        (assign34030_e56739, (((locals.var_t5_3_dn0 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn0))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn2 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn2))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn3 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn3))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn4 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn4))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn5 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn5))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn6 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn6))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn7 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn7))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn8 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn8))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn9 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn9))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn10 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn10))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn11 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn11))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn13 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn13))) / (assign34030_e56738 * assign34030_e56738)), (((locals.var_t5_3_dn14 * assign34030_e56738) - (locals.var_t5_3 * (3.0 * locals.var_t7_3_dn14))) / (assign34030_e56738 * assign34030_e56738)),)
    } else {
        (locals.var_epsilon2, locals.var_epsilon2_dn0, locals.var_epsilon2_dn2, locals.var_epsilon2_dn3, locals.var_epsilon2_dn4, locals.var_epsilon2_dn5, locals.var_epsilon2_dn6, locals.var_epsilon2_dn7, locals.var_epsilon2_dn8, locals.var_epsilon2_dn9, locals.var_epsilon2_dn10, locals.var_epsilon2_dn11, locals.var_epsilon2_dn13, locals.var_epsilon2_dn14,)
    }
};
        locals.var_epsilon2 = assign34030_e56741;
        locals.var_epsilon2_dn0 = assign34030_e56741_d_n0;
        locals.var_epsilon2_dn2 = assign34030_e56741_d_n2;
        locals.var_epsilon2_dn3 = assign34030_e56741_d_n3;
        locals.var_epsilon2_dn4 = assign34030_e56741_d_n4;
        locals.var_epsilon2_dn5 = assign34030_e56741_d_n5;
        locals.var_epsilon2_dn6 = assign34030_e56741_d_n6;
        locals.var_epsilon2_dn7 = assign34030_e56741_d_n7;
        locals.var_epsilon2_dn8 = assign34030_e56741_d_n8;
        locals.var_epsilon2_dn9 = assign34030_e56741_d_n9;
        locals.var_epsilon2_dn10 = assign34030_e56741_d_n10;
        locals.var_epsilon2_dn11 = assign34030_e56741_d_n11;
        locals.var_epsilon2_dn13 = assign34030_e56741_d_n13;
        locals.var_epsilon2_dn14 = assign34030_e56741_d_n14;

        let (assign34040_e56756, assign34040_e56756_d_n0, assign34040_e56756_d_n2, assign34040_e56756_d_n3, assign34040_e56756_d_n4, assign34040_e56756_d_n5, assign34040_e56756_d_n6, assign34040_e56756_d_n7, assign34040_e56756_d_n8, assign34040_e56756_d_n9, assign34040_e56756_d_n10, assign34040_e56756_d_n11, assign34040_e56756_d_n13, assign34040_e56756_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34040_e56748: f64 = (locals.var_moc / 6.0);
        let assign34040_e56750: f64 = (assign34040_e56748 * locals.var_dvsat);
        let assign34040_e56753: f64 = (locals.var_epsilon1 - locals.var_epsilon2);
        let assign34040_e56754: f64 = (assign34040_e56750 * assign34040_e56753);
        (assign34040_e56754, (((((locals.var_moc_dn0 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn0)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn0 - locals.var_epsilon2_dn0))), (((((locals.var_moc_dn2 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn2)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn2 - locals.var_epsilon2_dn2))), (((((locals.var_moc_dn3 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn3)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn3 - locals.var_epsilon2_dn3))), (((((locals.var_moc_dn4 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn4)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn4 - locals.var_epsilon2_dn4))), (((((locals.var_moc_dn5 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn5)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn5 - locals.var_epsilon2_dn5))), (((((locals.var_moc_dn6 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn6)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn6 - locals.var_epsilon2_dn6))), (((((locals.var_moc_dn7 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn7)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn7 - locals.var_epsilon2_dn7))), (((((locals.var_moc_dn8 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn8)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn8 - locals.var_epsilon2_dn8))), (((((locals.var_moc_dn9 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn9)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn9 - locals.var_epsilon2_dn9))), (((((locals.var_moc_dn10 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn10)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn10 - locals.var_epsilon2_dn10))), (((((locals.var_moc_dn11 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn11)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn11 - locals.var_epsilon2_dn11))), (((((locals.var_moc_dn13 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn13)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn13 - locals.var_epsilon2_dn13))), (((((locals.var_moc_dn14 / 6.0) * locals.var_dvsat) + (assign34040_e56748 * locals.var_dvsat_dn14)) * assign34040_e56753) + (assign34040_e56750 * (locals.var_epsilon1_dn14 - locals.var_epsilon2_dn14))),)
    } else {
        (locals.var_epsilon, locals.var_epsilon_dn0, locals.var_epsilon_dn2, locals.var_epsilon_dn3, locals.var_epsilon_dn4, locals.var_epsilon_dn5, locals.var_epsilon_dn6, locals.var_epsilon_dn7, locals.var_epsilon_dn8, locals.var_epsilon_dn9, locals.var_epsilon_dn10, locals.var_epsilon_dn11, locals.var_epsilon_dn13, locals.var_epsilon_dn14,)
    }
};
        locals.var_epsilon = assign34040_e56756;
        locals.var_epsilon_dn0 = assign34040_e56756_d_n0;
        locals.var_epsilon_dn2 = assign34040_e56756_d_n2;
        locals.var_epsilon_dn3 = assign34040_e56756_d_n3;
        locals.var_epsilon_dn4 = assign34040_e56756_d_n4;
        locals.var_epsilon_dn5 = assign34040_e56756_d_n5;
        locals.var_epsilon_dn6 = assign34040_e56756_d_n6;
        locals.var_epsilon_dn7 = assign34040_e56756_d_n7;
        locals.var_epsilon_dn8 = assign34040_e56756_d_n8;
        locals.var_epsilon_dn9 = assign34040_e56756_d_n9;
        locals.var_epsilon_dn10 = assign34040_e56756_d_n10;
        locals.var_epsilon_dn11 = assign34040_e56756_d_n11;
        locals.var_epsilon_dn13 = assign34040_e56756_d_n13;
        locals.var_epsilon_dn14 = assign34040_e56756_d_n14;

        let (assign34050_e56770, assign34050_e56770_d_n0, assign34050_e56770_d_n2, assign34050_e56770_d_n3, assign34050_e56770_d_n4, assign34050_e56770_d_n5, assign34050_e56770_d_n6, assign34050_e56770_d_n7, assign34050_e56770_d_n8, assign34050_e56770_d_n9, assign34050_e56770_d_n10, assign34050_e56770_d_n11, assign34050_e56770_d_n13, assign34050_e56770_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34050_e56763: f64 = (locals.var_t3 * locals.var_epsilon);
        let assign34050_e56766: f64 = (locals.var_gamma * locals.var_delta);
        let assign34050_e56767: f64 = (assign34050_e56766).sqrt();
        let assign34050_e56768: f64 = (assign34050_e56763 / assign34050_e56767);
        (assign34050_e56768, (((((locals.var_t3_dn0 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn0)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn0 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn0)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn2 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn2)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn2 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn2)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn3 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn3)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn3 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn3)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn4 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn4)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn4 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn4)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn5 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn5)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn5 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn5)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn6 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn6)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn6 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn6)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn7 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn7)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn7 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn7)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn8 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn8)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn8 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn8)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn9 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn9)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn9 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn9)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn10 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn10)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn10 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn10)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn11 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn11)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn11 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn11)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn13 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn13)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn13 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn13)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((locals.var_t3_dn14 * locals.var_epsilon) + (locals.var_t3 * locals.var_epsilon_dn14)) * assign34050_e56767) - (assign34050_e56763 * (((locals.var_gamma_dn14 * locals.var_delta) + (locals.var_gamma * locals.var_delta_dn14)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)),)
    } else {
        (locals.var_ctnoi, locals.var_ctnoi_dn0, locals.var_ctnoi_dn2, locals.var_ctnoi_dn3, locals.var_ctnoi_dn4, locals.var_ctnoi_dn5, locals.var_ctnoi_dn6, locals.var_ctnoi_dn7, locals.var_ctnoi_dn8, locals.var_ctnoi_dn9, locals.var_ctnoi_dn10, locals.var_ctnoi_dn11, locals.var_ctnoi_dn13, locals.var_ctnoi_dn14,)
    }
};
        locals.var_ctnoi = assign34050_e56770;
        locals.var_ctnoi_dn0 = assign34050_e56770_d_n0;
        locals.var_ctnoi_dn2 = assign34050_e56770_d_n2;
        locals.var_ctnoi_dn3 = assign34050_e56770_d_n3;
        locals.var_ctnoi_dn4 = assign34050_e56770_d_n4;
        locals.var_ctnoi_dn5 = assign34050_e56770_d_n5;
        locals.var_ctnoi_dn6 = assign34050_e56770_d_n6;
        locals.var_ctnoi_dn7 = assign34050_e56770_d_n7;
        locals.var_ctnoi_dn8 = assign34050_e56770_d_n8;
        locals.var_ctnoi_dn9 = assign34050_e56770_d_n9;
        locals.var_ctnoi_dn10 = assign34050_e56770_d_n10;
        locals.var_ctnoi_dn11 = assign34050_e56770_d_n11;
        locals.var_ctnoi_dn13 = assign34050_e56770_d_n13;
        locals.var_ctnoi_dn14 = assign34050_e56770_d_n14;

        let assign34060_e56773: f64 = if locals.var_ctnoi > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign34060_e56773;

        let (assign34070_e56782, assign34070_e56782_d_n0, assign34070_e56782_d_n2, assign34070_e56782_d_n3, assign34070_e56782_d_n4, assign34070_e56782_d_n5, assign34070_e56782_d_n6, assign34070_e56782_d_n7, assign34070_e56782_d_n8, assign34070_e56782_d_n9, assign34070_e56782_d_n10, assign34070_e56782_d_n11, assign34070_e56782_d_n13, assign34070_e56782_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ctnoi, locals.var_ctnoi_dn0, locals.var_ctnoi_dn2, locals.var_ctnoi_dn3, locals.var_ctnoi_dn4, locals.var_ctnoi_dn5, locals.var_ctnoi_dn6, locals.var_ctnoi_dn7, locals.var_ctnoi_dn8, locals.var_ctnoi_dn9, locals.var_ctnoi_dn10, locals.var_ctnoi_dn11, locals.var_ctnoi_dn13, locals.var_ctnoi_dn14,)
    }
};
        locals.var_ctnoi = assign34070_e56782;
        locals.var_ctnoi_dn0 = assign34070_e56782_d_n0;
        locals.var_ctnoi_dn2 = assign34070_e56782_d_n2;
        locals.var_ctnoi_dn3 = assign34070_e56782_d_n3;
        locals.var_ctnoi_dn4 = assign34070_e56782_d_n4;
        locals.var_ctnoi_dn5 = assign34070_e56782_d_n5;
        locals.var_ctnoi_dn6 = assign34070_e56782_d_n6;
        locals.var_ctnoi_dn7 = assign34070_e56782_d_n7;
        locals.var_ctnoi_dn8 = assign34070_e56782_d_n8;
        locals.var_ctnoi_dn9 = assign34070_e56782_d_n9;
        locals.var_ctnoi_dn10 = assign34070_e56782_d_n10;
        locals.var_ctnoi_dn11 = assign34070_e56782_d_n11;
        locals.var_ctnoi_dn13 = assign34070_e56782_d_n13;
        locals.var_ctnoi_dn14 = assign34070_e56782_d_n14;

        let assign34080_e56785: f64 = if locals.var_ctnoi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign34080_e56785;

        let (assign34090_e56797, assign34090_e56797_d_n0, assign34090_e56797_d_n2, assign34090_e56797_d_n3, assign34090_e56797_d_n4, assign34090_e56797_d_n5, assign34090_e56797_d_n6, assign34090_e56797_d_n7, assign34090_e56797_d_n8, assign34090_e56797_d_n9, assign34090_e56797_d_n10, assign34090_e56797_d_n11, assign34090_e56797_d_n13, assign34090_e56797_d_n14,) = {
    if ((((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard639 == 0.0)) && (locals.var_guard640 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ctnoi, locals.var_ctnoi_dn0, locals.var_ctnoi_dn2, locals.var_ctnoi_dn3, locals.var_ctnoi_dn4, locals.var_ctnoi_dn5, locals.var_ctnoi_dn6, locals.var_ctnoi_dn7, locals.var_ctnoi_dn8, locals.var_ctnoi_dn9, locals.var_ctnoi_dn10, locals.var_ctnoi_dn11, locals.var_ctnoi_dn13, locals.var_ctnoi_dn14,)
    }
};
        locals.var_ctnoi = assign34090_e56797;
        locals.var_ctnoi_dn0 = assign34090_e56797_d_n0;
        locals.var_ctnoi_dn2 = assign34090_e56797_d_n2;
        locals.var_ctnoi_dn3 = assign34090_e56797_d_n3;
        locals.var_ctnoi_dn4 = assign34090_e56797_d_n4;
        locals.var_ctnoi_dn5 = assign34090_e56797_d_n5;
        locals.var_ctnoi_dn6 = assign34090_e56797_d_n6;
        locals.var_ctnoi_dn7 = assign34090_e56797_d_n7;
        locals.var_ctnoi_dn8 = assign34090_e56797_d_n8;
        locals.var_ctnoi_dn9 = assign34090_e56797_d_n9;
        locals.var_ctnoi_dn10 = assign34090_e56797_d_n10;
        locals.var_ctnoi_dn11 = assign34090_e56797_d_n11;
        locals.var_ctnoi_dn13 = assign34090_e56797_d_n13;
        locals.var_ctnoi_dn14 = assign34090_e56797_d_n14;

        let (assign34100_e56816, assign34100_e56816_d_n0, assign34100_e56816_d_n2, assign34100_e56816_d_n3, assign34100_e56816_d_n4, assign34100_e56816_d_n5, assign34100_e56816_d_n6, assign34100_e56816_d_n7, assign34100_e56816_d_n8, assign34100_e56816_d_n9, assign34100_e56816_d_n10, assign34100_e56816_d_n11, assign34100_e56816_d_n13, assign34100_e56816_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34100_e56805: f64 = (locals.var_noilowid * locals.var_noilowid);
        let assign34100_e56808: f64 = (p.p1716 + locals.var_qia);
        let assign34100_e56809: f64 = (assign34100_e56805 / assign34100_e56808);
        let assign34100_e56812: f64 = (locals.var_vdseff_1 / locals.var_vdsat);
        let assign34100_e56813: f64 = (assign34100_e56809 * assign34100_e56812);
        let assign34100_e56814: f64 = (1.0 + assign34100_e56813);
        (assign34100_e56814, (((((((locals.var_noilowid_dn0 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn0)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn0)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn0 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn0)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn2 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn2)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn2)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn2 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn2)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn3 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn3)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn3)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn3 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn3)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn4 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn4)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn4)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn4 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn4)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn5 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn5)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn5)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn5 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn6 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn6)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn6)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn6 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn7 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn7)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn7)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn7 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn7)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn8 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn8)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn8)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn8 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn8)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn9 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn9)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn9)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn9 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn9)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn10 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn10)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn10)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn10 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn10)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn11 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn11)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn11)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn11 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn11)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn13 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn13)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn13)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn13 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn13)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn14 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn14)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn14)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn14 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn14)) / (locals.var_vdsat * locals.var_vdsat)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign34100_e56816;
        locals.var_t8_dn0 = assign34100_e56816_d_n0;
        locals.var_t8_dn2 = assign34100_e56816_d_n2;
        locals.var_t8_dn3 = assign34100_e56816_d_n3;
        locals.var_t8_dn4 = assign34100_e56816_d_n4;
        locals.var_t8_dn5 = assign34100_e56816_d_n5;
        locals.var_t8_dn6 = assign34100_e56816_d_n6;
        locals.var_t8_dn7 = assign34100_e56816_d_n7;
        locals.var_t8_dn8 = assign34100_e56816_d_n8;
        locals.var_t8_dn9 = assign34100_e56816_d_n9;
        locals.var_t8_dn10 = assign34100_e56816_d_n10;
        locals.var_t8_dn11 = assign34100_e56816_d_n11;
        locals.var_t8_dn13 = assign34100_e56816_d_n13;
        locals.var_t8_dn14 = assign34100_e56816_d_n14;

        let (assign34110_e56833, assign34110_e56833_d_n0, assign34110_e56833_d_n2, assign34110_e56833_d_n3, assign34110_e56833_d_n4, assign34110_e56833_d_n5, assign34110_e56833_d_n6, assign34110_e56833_d_n7, assign34110_e56833_d_n8, assign34110_e56833_d_n9, assign34110_e56833_d_n10, assign34110_e56833_d_n11, assign34110_e56833_d_n13, assign34110_e56833_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34110_e56823: f64 = (locals.var_moc / locals.var_dvsat);
        let assign34110_e56826: f64 = (locals.var_t8 * locals.var_gamma1);
        let assign34110_e56829: f64 = (locals.var_t1 * locals.var_gamma2);
        let assign34110_e56830: f64 = (assign34110_e56826 + assign34110_e56829);
        let assign34110_e56831: f64 = (assign34110_e56823 * assign34110_e56830);
        (assign34110_e56831, (((((locals.var_moc_dn0 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn0)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn0 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn0)) + ((locals.var_t1_dn0 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn0))))), (((((locals.var_moc_dn2 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn2)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn2 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn2)) + ((locals.var_t1_dn2 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn2))))), (((((locals.var_moc_dn3 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn3 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn3)) + ((locals.var_t1_dn3 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn3))))), (((((locals.var_moc_dn4 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn4 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn4)) + ((locals.var_t1_dn4 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn4))))), (((((locals.var_moc_dn5 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn5 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn5)) + ((locals.var_t1_dn5 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn5))))), (((((locals.var_moc_dn6 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn6 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn6)) + ((locals.var_t1_dn6 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn6))))), (((((locals.var_moc_dn7 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn7 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn7)) + ((locals.var_t1_dn7 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn7))))), (((((locals.var_moc_dn8 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn8 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn8)) + ((locals.var_t1_dn8 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn8))))), (((((locals.var_moc_dn9 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn9 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn9)) + ((locals.var_t1_dn9 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn9))))), (((((locals.var_moc_dn10 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn10 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn10)) + ((locals.var_t1_dn10 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn10))))), (((((locals.var_moc_dn11 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn11 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn11)) + ((locals.var_t1_dn11 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn11))))), (((((locals.var_moc_dn13 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn13)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn13 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn13)) + ((locals.var_t1_dn13 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn13))))), (((((locals.var_moc_dn14 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn14)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn14 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn14)) + ((locals.var_t1_dn14 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn14))))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn3, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn13, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign34110_e56833;
        locals.var_gamma_dn0 = assign34110_e56833_d_n0;
        locals.var_gamma_dn2 = assign34110_e56833_d_n2;
        locals.var_gamma_dn3 = assign34110_e56833_d_n3;
        locals.var_gamma_dn4 = assign34110_e56833_d_n4;
        locals.var_gamma_dn5 = assign34110_e56833_d_n5;
        locals.var_gamma_dn6 = assign34110_e56833_d_n6;
        locals.var_gamma_dn7 = assign34110_e56833_d_n7;
        locals.var_gamma_dn8 = assign34110_e56833_d_n8;
        locals.var_gamma_dn9 = assign34110_e56833_d_n9;
        locals.var_gamma_dn10 = assign34110_e56833_d_n10;
        locals.var_gamma_dn11 = assign34110_e56833_d_n11;
        locals.var_gamma_dn13 = assign34110_e56833_d_n13;
        locals.var_gamma_dn14 = assign34110_e56833_d_n14;

        let (assign34130_e56867, assign34130_e56867_d_n0, assign34130_e56867_d_n2, assign34130_e56867_d_n3, assign34130_e56867_d_n4, assign34130_e56867_d_n5, assign34130_e56867_d_n6, assign34130_e56867_d_n7, assign34130_e56867_d_n8, assign34130_e56867_d_n9, assign34130_e56867_d_n10, assign34130_e56867_d_n11, assign34130_e56867_d_n13, assign34130_e56867_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34130_e56855: f64 = (locals.var_moc / 6.0);
        let assign34130_e56857: f64 = (assign34130_e56855 * locals.var_dvsat3);
        let assign34130_e56859: f64 = (assign34130_e56857 * locals.var_t2);
        let assign34130_e56862: f64 = (locals.var_delta1 - locals.var_delta2);
        let assign34130_e56864: f64 = (assign34130_e56862 + locals.var_delta3);
        let assign34130_e56865: f64 = (assign34130_e56859 * assign34130_e56864);
        (assign34130_e56865, (((((((locals.var_moc_dn0 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn0)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn0)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn0 - locals.var_delta2_dn0) + locals.var_delta3_dn0))), (((((((locals.var_moc_dn2 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn2)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn2)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn2 - locals.var_delta2_dn2) + locals.var_delta3_dn2))), (((((((locals.var_moc_dn3 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn3)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn3)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn3 - locals.var_delta2_dn3) + locals.var_delta3_dn3))), (((((((locals.var_moc_dn4 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn4)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn4)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn4 - locals.var_delta2_dn4) + locals.var_delta3_dn4))), (((((((locals.var_moc_dn5 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn5)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn5)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn5 - locals.var_delta2_dn5) + locals.var_delta3_dn5))), (((((((locals.var_moc_dn6 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn6)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn6)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn6 - locals.var_delta2_dn6) + locals.var_delta3_dn6))), (((((((locals.var_moc_dn7 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn7)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn7)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn7 - locals.var_delta2_dn7) + locals.var_delta3_dn7))), (((((((locals.var_moc_dn8 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn8)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn8)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn8 - locals.var_delta2_dn8) + locals.var_delta3_dn8))), (((((((locals.var_moc_dn9 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn9)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn9)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn9 - locals.var_delta2_dn9) + locals.var_delta3_dn9))), (((((((locals.var_moc_dn10 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn10)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn10)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn10 - locals.var_delta2_dn10) + locals.var_delta3_dn10))), (((((((locals.var_moc_dn11 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn11)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn11)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn11 - locals.var_delta2_dn11) + locals.var_delta3_dn11))), (((((((locals.var_moc_dn13 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn13)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn13)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn13 - locals.var_delta2_dn13) + locals.var_delta3_dn13))), (((((((locals.var_moc_dn14 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn14)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn14)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn14 - locals.var_delta2_dn14) + locals.var_delta3_dn14))),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign34130_e56867;
        locals.var_delta_dn0 = assign34130_e56867_d_n0;
        locals.var_delta_dn2 = assign34130_e56867_d_n2;
        locals.var_delta_dn3 = assign34130_e56867_d_n3;
        locals.var_delta_dn4 = assign34130_e56867_d_n4;
        locals.var_delta_dn5 = assign34130_e56867_d_n5;
        locals.var_delta_dn6 = assign34130_e56867_d_n6;
        locals.var_delta_dn7 = assign34130_e56867_d_n7;
        locals.var_delta_dn8 = assign34130_e56867_d_n8;
        locals.var_delta_dn9 = assign34130_e56867_d_n9;
        locals.var_delta_dn10 = assign34130_e56867_d_n10;
        locals.var_delta_dn11 = assign34130_e56867_d_n11;
        locals.var_delta_dn13 = assign34130_e56867_d_n13;
        locals.var_delta_dn14 = assign34130_e56867_d_n14;

        let (assign34140_e56887, assign34140_e56887_d_n0, assign34140_e56887_d_n2, assign34140_e56887_d_n3, assign34140_e56887_d_n4, assign34140_e56887_d_n5, assign34140_e56887_d_n6, assign34140_e56887_d_n7, assign34140_e56887_d_n8, assign34140_e56887_d_n9, assign34140_e56887_d_n10, assign34140_e56887_d_n11, assign34140_e56887_d_n13, assign34140_e56887_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34140_e56874: f64 = (locals.var_delta / locals.var_gamma);
        let assign34140_e56875: f64 = (assign34140_e56874).sqrt();
        let assign34140_e56877: f64 = (assign34140_e56875 * locals.var_nfintotal);
        let assign34140_e56879: f64 = (assign34140_e56877 * locals.var_coxeff);
        let assign34140_e56881: f64 = (assign34140_e56879 * locals.var_weffcv0);
        let assign34140_e56883: f64 = (assign34140_e56881 * locals.var_leffcv_1);
        let assign34140_e56885: f64 = (assign34140_e56883 / locals.var_noigd0);
        (assign34140_e56885, (((((((((((((locals.var_delta_dn0 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn0)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn0)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn0)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn0)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn2 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn2)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn2)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn2)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn2)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn3 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn3)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn3)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn3)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn3)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn4 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn4)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn4)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn4)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn4)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn5 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn5)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn5)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn5)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn5)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn6 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn6)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn6)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn6)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn6)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn7 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn7)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn7)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn7)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn7)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn8 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn8)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn8)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn8)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn8)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn9 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn9)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn9)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn9)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn9)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn10 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn10)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn10)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn10)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn10)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn11 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn11)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn11)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn11)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn11)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn13 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn13)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn13)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn13)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn13)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn14 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn14)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn14)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn14)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn14)) / (locals.var_noigd0 * locals.var_noigd0)),)
    } else {
        (locals.var_sigrat, locals.var_sigrat_dn0, locals.var_sigrat_dn2, locals.var_sigrat_dn3, locals.var_sigrat_dn4, locals.var_sigrat_dn5, locals.var_sigrat_dn6, locals.var_sigrat_dn7, locals.var_sigrat_dn8, locals.var_sigrat_dn9, locals.var_sigrat_dn10, locals.var_sigrat_dn11, locals.var_sigrat_dn13, locals.var_sigrat_dn14,)
    }
};
        locals.var_sigrat = assign34140_e56887;
        locals.var_sigrat_dn0 = assign34140_e56887_d_n0;
        locals.var_sigrat_dn2 = assign34140_e56887_d_n2;
        locals.var_sigrat_dn3 = assign34140_e56887_d_n3;
        locals.var_sigrat_dn4 = assign34140_e56887_d_n4;
        locals.var_sigrat_dn5 = assign34140_e56887_d_n5;
        locals.var_sigrat_dn6 = assign34140_e56887_d_n6;
        locals.var_sigrat_dn7 = assign34140_e56887_d_n7;
        locals.var_sigrat_dn8 = assign34140_e56887_d_n8;
        locals.var_sigrat_dn9 = assign34140_e56887_d_n9;
        locals.var_sigrat_dn10 = assign34140_e56887_d_n10;
        locals.var_sigrat_dn11 = assign34140_e56887_d_n11;
        locals.var_sigrat_dn13 = assign34140_e56887_d_n13;
        locals.var_sigrat_dn14 = assign34140_e56887_d_n14;

        let assign34150_e56890: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard641 = assign34150_e56890;

        let assign34160_e56893: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign34160_e56893;

        let assign34180_e56899: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign34180_e56899;

        let assign34190_e56902: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard645 = assign34190_e56902;

        let assign34200_e56917: f64 = if ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        locals.var_guard646 = assign34200_e56917;

        let assign34210_e56920: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard647 = assign34210_e56920;

        let assign34220_e56935: f64 = if ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        locals.var_guard648 = assign34220_e56935;

        let assign34230_e56938: f64 = if p.p61 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard649 = assign34230_e56938;

        let assign34240_e56941: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard650 = assign34240_e56941;

        let assign34250_e56944: f64 = if p.p76 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign34250_e56944;

        let assign34260_e56947: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign34260_e56947;

        let assign34270_e56950: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign34270_e56950;

        let assign34280_e56953: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign34280_e56953;

        let assign34290_e56956: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign34290_e56956;

        let assign34300_e56959: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign34300_e56959;

        let assign34310_e56962: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign34310_e56962;

        let assign34320_e56965: f64 = if p.p1910 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign34320_e56965;

    }

    pub(super) fn stamp_transient_block_132(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (assign34330_e57042, assign34330_e57042_d_n4,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34330_e56972: f64 = (p.p1912 * locals.var_deltemp);
        let assign34330_e56973: f64 = (1.0 + assign34330_e56972);
        let assign34330_e56975: f64 = (assign34330_e56973 - 1e-6);
        let assign34330_e56977: f64 = (-10000.0);
        let assign34330_e56979: f64 = (assign34330_e56977 * 0.001);
        let (assign34330_e57040, assign34330_e57040_d_n4,) = {
            if (!(assign34330_e56975 < assign34330_e56979)) {
                let assign34330_e56986: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e56987: f64 = (1.0 + assign34330_e56986);
                let assign34330_e56989: f64 = (assign34330_e56987 - 1e-6);
                let assign34330_e56993: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e56994: f64 = (1.0 + assign34330_e56993);
                let assign34330_e56996: f64 = (assign34330_e56994 - 1e-6);
                let assign34330_e57000: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e57001: f64 = (1.0 + assign34330_e57000);
                let assign34330_e57003: f64 = (assign34330_e57001 - 1e-6);
                let assign34330_e57004: f64 = (assign34330_e56996 * assign34330_e57003);
                let assign34330_e57007: f64 = (4.0 * 0.001);
                let assign34330_e57009: f64 = (assign34330_e57007 * 0.001);
                let assign34330_e57010: f64 = (assign34330_e57004 + assign34330_e57009);
                let assign34330_e57011: f64 = (assign34330_e57010).sqrt();
                let assign34330_e57012: f64 = (assign34330_e56989 + assign34330_e57011);
                let assign34330_e57013: f64 = (0.5 * assign34330_e57012);
                (assign34330_e57013, (0.5 * ((p.p1912 * locals.var_deltemp_dn4) + ((((p.p1912 * locals.var_deltemp_dn4) * assign34330_e57003) + (assign34330_e56996 * (p.p1912 * locals.var_deltemp_dn4))) / (2.0 * assign34330_e57011)))),)
            } else {
                let assign34330_e57017: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e57018: f64 = (1.0 + assign34330_e57017);
                let assign34330_e57020: f64 = (assign34330_e57018 - 1e-6);
                let assign34330_e57022: f64 = (-10000.0);
                let assign34330_e57024: f64 = (assign34330_e57022 * 0.001);
                let (assign34330_e57039, assign34330_e57039_d_n4,) = {
                    if (assign34330_e57020 < assign34330_e57024) {
                        let assign34330_e57027: f64 = (-0.001);
                        let assign34330_e57029: f64 = (assign34330_e57027 * 0.001);
                        let assign34330_e57033: f64 = (p.p1912 * locals.var_deltemp);
                        let assign34330_e57034: f64 = (1.0 + assign34330_e57033);
                        let assign34330_e57036: f64 = (assign34330_e57034 - 1e-6);
                        let assign34330_e57037: f64 = (assign34330_e57029 / assign34330_e57036);
                        (assign34330_e57037, (-((assign34330_e57029 * (p.p1912 * locals.var_deltemp_dn4)) / (assign34330_e57036 * assign34330_e57036))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34330_e57039, assign34330_e57039_d_n4,)
            }
        };
        (assign34330_e57040, assign34330_e57040_d_n4,)
    } else {
        (locals.var_rdstempvs, locals.var_rdstempvs_dn4,)
    }
};
        locals.var_rdstempvs = assign34330_e57042;
        locals.var_rdstempvs_dn4 = assign34330_e57042_d_n4;

        let assign34340_e57045: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign34340_e57045;

        let (assign34350_e57096, assign34350_e57096_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign34350_e57053: f64 = (-p.p1904);
        let assign34350_e57056: f64 = (-p.p1913);
        let assign34350_e57058: f64 = (assign34350_e57056 * locals.var_deltemp);
        let assign34350_e57060: f64 = (-p.p1904);
        let assign34350_e57061: f64 = (assign34350_e57058 - assign34350_e57060);
        let assign34350_e57063: f64 = (assign34350_e57061 - 1e-6);
        let assign34350_e57065: f64 = (-p.p1913);
        let assign34350_e57067: f64 = (assign34350_e57065 * locals.var_deltemp);
        let assign34350_e57069: f64 = (-p.p1904);
        let assign34350_e57070: f64 = (assign34350_e57067 - assign34350_e57069);
        let assign34350_e57072: f64 = (assign34350_e57070 - 1e-6);
        let assign34350_e57074: f64 = (-p.p1913);
        let assign34350_e57076: f64 = (assign34350_e57074 * locals.var_deltemp);
        let assign34350_e57078: f64 = (-p.p1904);
        let assign34350_e57079: f64 = (assign34350_e57076 - assign34350_e57078);
        let assign34350_e57081: f64 = (assign34350_e57079 - 1e-6);
        let assign34350_e57082: f64 = (assign34350_e57072 * assign34350_e57081);
        let assign34350_e57085: f64 = (-p.p1904);
        let assign34350_e57086: f64 = (4.0 * assign34350_e57085);
        let assign34350_e57088: f64 = (assign34350_e57086 * 1e-6);
        let assign34350_e57089: f64 = (assign34350_e57082 - assign34350_e57088);
        let assign34350_e57090: f64 = (assign34350_e57089).sqrt();
        let assign34350_e57091: f64 = (assign34350_e57063 + assign34350_e57090);
        let assign34350_e57092: f64 = (0.5 * assign34350_e57091);
        let assign34350_e57093: f64 = (assign34350_e57053 + assign34350_e57092);
        let assign34350_e57094: f64 = (p.p1904 + assign34350_e57093);
        (assign34350_e57094, (0.5 * ((assign34350_e57056 * locals.var_deltemp_dn4) + ((((assign34350_e57065 * locals.var_deltemp_dn4) * assign34350_e57081) + (assign34350_e57072 * (assign34350_e57074 * locals.var_deltemp_dn4))) / (2.0 * assign34350_e57090)))),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34350_e57096;
        locals.var_vsatrsd_t_dn4 = assign34350_e57096_d_n4;

        let (assign34360_e57184, assign34360_e57184_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 == 0.0)) {
        let assign34360_e57106: f64 = (-p.p1913);
        let assign34360_e57108: f64 = (assign34360_e57106 * locals.var_deltemp);
        let assign34360_e57109: f64 = (1.0 + assign34360_e57108);
        let assign34360_e57111: f64 = (assign34360_e57109 - 1e-6);
        let assign34360_e57113: f64 = (-10000.0);
        let assign34360_e57115: f64 = (assign34360_e57113 * 0.001);
        let (assign34360_e57181, assign34360_e57181_d_n4,) = {
            if (!(assign34360_e57111 < assign34360_e57115)) {
                let assign34360_e57121: f64 = (-p.p1913);
                let assign34360_e57123: f64 = (assign34360_e57121 * locals.var_deltemp);
                let assign34360_e57124: f64 = (1.0 + assign34360_e57123);
                let assign34360_e57126: f64 = (assign34360_e57124 - 1e-6);
                let assign34360_e57129: f64 = (-p.p1913);
                let assign34360_e57131: f64 = (assign34360_e57129 * locals.var_deltemp);
                let assign34360_e57132: f64 = (1.0 + assign34360_e57131);
                let assign34360_e57134: f64 = (assign34360_e57132 - 1e-6);
                let assign34360_e57137: f64 = (-p.p1913);
                let assign34360_e57139: f64 = (assign34360_e57137 * locals.var_deltemp);
                let assign34360_e57140: f64 = (1.0 + assign34360_e57139);
                let assign34360_e57142: f64 = (assign34360_e57140 - 1e-6);
                let assign34360_e57143: f64 = (assign34360_e57134 * assign34360_e57142);
                let assign34360_e57146: f64 = (4.0 * 0.001);
                let assign34360_e57148: f64 = (assign34360_e57146 * 0.001);
                let assign34360_e57149: f64 = (assign34360_e57143 + assign34360_e57148);
                let assign34360_e57150: f64 = (assign34360_e57149).sqrt();
                let assign34360_e57151: f64 = (assign34360_e57126 + assign34360_e57150);
                let assign34360_e57152: f64 = (0.5 * assign34360_e57151);
                (assign34360_e57152, (0.5 * ((assign34360_e57121 * locals.var_deltemp_dn4) + ((((assign34360_e57129 * locals.var_deltemp_dn4) * assign34360_e57142) + (assign34360_e57134 * (assign34360_e57137 * locals.var_deltemp_dn4))) / (2.0 * assign34360_e57150)))),)
            } else {
                let assign34360_e57155: f64 = (-p.p1913);
                let assign34360_e57157: f64 = (assign34360_e57155 * locals.var_deltemp);
                let assign34360_e57158: f64 = (1.0 + assign34360_e57157);
                let assign34360_e57160: f64 = (assign34360_e57158 - 1e-6);
                let assign34360_e57162: f64 = (-10000.0);
                let assign34360_e57164: f64 = (assign34360_e57162 * 0.001);
                let (assign34360_e57180, assign34360_e57180_d_n4,) = {
                    if (assign34360_e57160 < assign34360_e57164) {
                        let assign34360_e57167: f64 = (-0.001);
                        let assign34360_e57169: f64 = (assign34360_e57167 * 0.001);
                        let assign34360_e57172: f64 = (-p.p1913);
                        let assign34360_e57174: f64 = (assign34360_e57172 * locals.var_deltemp);
                        let assign34360_e57175: f64 = (1.0 + assign34360_e57174);
                        let assign34360_e57177: f64 = (assign34360_e57175 - 1e-6);
                        let assign34360_e57178: f64 = (assign34360_e57169 / assign34360_e57177);
                        (assign34360_e57178, (-((assign34360_e57169 * (assign34360_e57172 * locals.var_deltemp_dn4)) / (assign34360_e57177 * assign34360_e57177))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34360_e57180, assign34360_e57180_d_n4,)
            }
        };
        let assign34360_e57182: f64 = (p.p1904 * assign34360_e57181);
        (assign34360_e57182, (p.p1904 * assign34360_e57181_d_n4),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34360_e57184;
        locals.var_vsatrsd_t_dn4 = assign34360_e57184_d_n4;

        let (assign34370_e57192, assign34370_e57192_d_n0, assign34370_e57192_d_n2, assign34370_e57192_d_n3, assign34370_e57192_d_n4, assign34370_e57192_d_n5, assign34370_e57192_d_n6, assign34370_e57192_d_n7, assign34370_e57192_d_n8, assign34370_e57192_d_n9, assign34370_e57192_d_n10, assign34370_e57192_d_n11, assign34370_e57192_d_n13, assign34370_e57192_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34370_e57190: f64 = (locals.var_qis - p.p1906);
        (assign34370_e57190, locals.var_qis_dn0, locals.var_qis_dn2, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11, locals.var_qis_dn13, locals.var_qis_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34370_e57192;
        locals.var_t0_dn0 = assign34370_e57192_d_n0;
        locals.var_t0_dn2 = assign34370_e57192_d_n2;
        locals.var_t0_dn3 = assign34370_e57192_d_n3;
        locals.var_t0_dn4 = assign34370_e57192_d_n4;
        locals.var_t0_dn5 = assign34370_e57192_d_n5;
        locals.var_t0_dn6 = assign34370_e57192_d_n6;
        locals.var_t0_dn7 = assign34370_e57192_d_n7;
        locals.var_t0_dn8 = assign34370_e57192_d_n8;
        locals.var_t0_dn9 = assign34370_e57192_d_n9;
        locals.var_t0_dn10 = assign34370_e57192_d_n10;
        locals.var_t0_dn11 = assign34370_e57192_d_n11;
        locals.var_t0_dn13 = assign34370_e57192_d_n13;
        locals.var_t0_dn14 = assign34370_e57192_d_n14;

        let (assign34380_e57217, assign34380_e57217_d_n0, assign34380_e57217_d_n2, assign34380_e57217_d_n3, assign34380_e57217_d_n4, assign34380_e57217_d_n5, assign34380_e57217_d_n6, assign34380_e57217_d_n7, assign34380_e57217_d_n8, assign34380_e57217_d_n9, assign34380_e57217_d_n10, assign34380_e57217_d_n11, assign34380_e57217_d_n13, assign34380_e57217_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34380_e57199: f64 = (locals.var_t0 + 0.1);
        let assign34380_e57202: f64 = (locals.var_t0 - 0.1);
        let assign34380_e57205: f64 = (locals.var_t0 - 0.1);
        let assign34380_e57206: f64 = (assign34380_e57202 * assign34380_e57205);
        let assign34380_e57209: f64 = (0.25 * 2.0);
        let assign34380_e57211: f64 = (assign34380_e57209 * 2.0);
        let assign34380_e57212: f64 = (assign34380_e57206 + assign34380_e57211);
        let assign34380_e57213: f64 = (assign34380_e57212).sqrt();
        let assign34380_e57214: f64 = (assign34380_e57199 + assign34380_e57213);
        let assign34380_e57215: f64 = (0.5 * assign34380_e57214);
        (assign34380_e57215, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn0)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn2)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn3)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn4)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn5)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn6)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn7)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn8)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn9)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn10)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn11)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn13)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn14)) / (2.0 * assign34380_e57213)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34380_e57217;
        locals.var_t0_dn0 = assign34380_e57217_d_n0;
        locals.var_t0_dn2 = assign34380_e57217_d_n2;
        locals.var_t0_dn3 = assign34380_e57217_d_n3;
        locals.var_t0_dn4 = assign34380_e57217_d_n4;
        locals.var_t0_dn5 = assign34380_e57217_d_n5;
        locals.var_t0_dn6 = assign34380_e57217_d_n6;
        locals.var_t0_dn7 = assign34380_e57217_d_n7;
        locals.var_t0_dn8 = assign34380_e57217_d_n8;
        locals.var_t0_dn9 = assign34380_e57217_d_n9;
        locals.var_t0_dn10 = assign34380_e57217_d_n10;
        locals.var_t0_dn11 = assign34380_e57217_d_n11;
        locals.var_t0_dn13 = assign34380_e57217_d_n13;
        locals.var_t0_dn14 = assign34380_e57217_d_n14;

        let (assign34390_e57233, assign34390_e57233_d_n0, assign34390_e57233_d_n2, assign34390_e57233_d_n3, assign34390_e57233_d_n4, assign34390_e57233_d_n5, assign34390_e57233_d_n6, assign34390_e57233_d_n7, assign34390_e57233_d_n8, assign34390_e57233_d_n9, assign34390_e57233_d_n10, assign34390_e57233_d_n11, assign34390_e57233_d_n13, assign34390_e57233_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34390_e57223: f64 = (10.0 * p.p1907);
        let assign34390_e57225: f64 = (assign34390_e57223 * locals.var_t0);
        let assign34390_e57228: f64 = (10.0 * p.p1907);
        let assign34390_e57230: f64 = (assign34390_e57228 + locals.var_t0);
        let assign34390_e57231: f64 = (assign34390_e57225 / assign34390_e57230);
        (assign34390_e57231, ((((assign34390_e57223 * locals.var_t0_dn0) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn0)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn2) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn2)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn3) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn3)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn4) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn4)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn5) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn5)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn6) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn6)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn7) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn7)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn8) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn8)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn9) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn9)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn10) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn10)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn11) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn11)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn13) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn13)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn14) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn14)) / (assign34390_e57230 * assign34390_e57230)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34390_e57233;
        locals.var_t1_dn0 = assign34390_e57233_d_n0;
        locals.var_t1_dn2 = assign34390_e57233_d_n2;
        locals.var_t1_dn3 = assign34390_e57233_d_n3;
        locals.var_t1_dn4 = assign34390_e57233_d_n4;
        locals.var_t1_dn5 = assign34390_e57233_d_n5;
        locals.var_t1_dn6 = assign34390_e57233_d_n6;
        locals.var_t1_dn7 = assign34390_e57233_d_n7;
        locals.var_t1_dn8 = assign34390_e57233_d_n8;
        locals.var_t1_dn9 = assign34390_e57233_d_n9;
        locals.var_t1_dn10 = assign34390_e57233_d_n10;
        locals.var_t1_dn11 = assign34390_e57233_d_n11;
        locals.var_t1_dn13 = assign34390_e57233_d_n13;
        locals.var_t1_dn14 = assign34390_e57233_d_n14;

        let (assign34400_e57245, assign34400_e57245_d_n0, assign34400_e57245_d_n2, assign34400_e57245_d_n3, assign34400_e57245_d_n4, assign34400_e57245_d_n5, assign34400_e57245_d_n6, assign34400_e57245_d_n7, assign34400_e57245_d_n8, assign34400_e57245_d_n9, assign34400_e57245_d_n10, assign34400_e57245_d_n11, assign34400_e57245_d_n13, assign34400_e57245_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34400_e57241: f64 = (p.p1905 * locals.var_t1);
        let assign34400_e57242: f64 = (1.0 + assign34400_e57241);
        let assign34400_e57243: f64 = (locals.var_vsatrsd_t * assign34400_e57242);
        (assign34400_e57243, (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn0)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn2)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn3)), ((locals.var_vsatrsd_t_dn4 * assign34400_e57242) + (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn4))), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn5)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn6)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn7)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn8)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn9)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn10)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn11)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn13)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn14)),)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34400_e57245;
        locals.var_vsatrsd_eff_dn0 = assign34400_e57245_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34400_e57245_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34400_e57245_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34400_e57245_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34400_e57245_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34400_e57245_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34400_e57245_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34400_e57245_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34400_e57245_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34400_e57245_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34400_e57245_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34400_e57245_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34400_e57245_d_n14;

        let (assign34410_e57286, assign34410_e57286_d_n0, assign34410_e57286_d_n2, assign34410_e57286_d_n3, assign34410_e57286_d_n4, assign34410_e57286_d_n5, assign34410_e57286_d_n6, assign34410_e57286_d_n7, assign34410_e57286_d_n8, assign34410_e57286_d_n9, assign34410_e57286_d_n10, assign34410_e57286_d_n11, assign34410_e57286_d_n13, assign34410_e57286_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34410_e57251: f64 = (-10000.0);
        let assign34410_e57253: f64 = (assign34410_e57251 * 10.0);
        let (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,) = {
            if (!(locals.var_vsatrsd_eff < assign34410_e57253)) {
                let assign34410_e57260: f64 = (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff);
                let assign34410_e57263: f64 = (4.0 * 10.0);
                let assign34410_e57265: f64 = (assign34410_e57263 * 10.0);
                let assign34410_e57266: f64 = (assign34410_e57260 + assign34410_e57265);
                let assign34410_e57267: f64 = (assign34410_e57266).sqrt();
                let assign34410_e57268: f64 = (locals.var_vsatrsd_eff + assign34410_e57267);
                let assign34410_e57269: f64 = (0.5 * assign34410_e57268);
                (assign34410_e57269, (0.5 * (locals.var_vsatrsd_eff_dn0 + (((locals.var_vsatrsd_eff_dn0 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn0)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn2 + (((locals.var_vsatrsd_eff_dn2 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn2)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn3 + (((locals.var_vsatrsd_eff_dn3 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn3)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn4 + (((locals.var_vsatrsd_eff_dn4 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn4)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn5 + (((locals.var_vsatrsd_eff_dn5 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn5)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn6 + (((locals.var_vsatrsd_eff_dn6 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn6)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn7 + (((locals.var_vsatrsd_eff_dn7 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn7)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn8 + (((locals.var_vsatrsd_eff_dn8 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn8)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn9 + (((locals.var_vsatrsd_eff_dn9 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn9)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn10 + (((locals.var_vsatrsd_eff_dn10 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn10)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn11 + (((locals.var_vsatrsd_eff_dn11 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn11)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn13 + (((locals.var_vsatrsd_eff_dn13 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn13)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn14 + (((locals.var_vsatrsd_eff_dn14 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn14)) / (2.0 * assign34410_e57267)))),)
            } else {
                let assign34410_e57272: f64 = (-10000.0);
                let assign34410_e57274: f64 = (assign34410_e57272 * 10.0);
                let (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,) = {
                    if (locals.var_vsatrsd_eff < assign34410_e57274) {
                        let assign34410_e57277: f64 = (-10.0);
                        let assign34410_e57279: f64 = (assign34410_e57277 * 10.0);
                        let assign34410_e57281: f64 = (assign34410_e57279 / locals.var_vsatrsd_eff);
                        (assign34410_e57281, (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn0) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn2) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn3) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn4) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn5) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn6) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn7) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn8) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn9) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn10) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn11) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn13) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn14) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,)
            }
        };
        (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34410_e57286;
        locals.var_vsatrsd_eff_dn0 = assign34410_e57286_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34410_e57286_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34410_e57286_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34410_e57286_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34410_e57286_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34410_e57286_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34410_e57286_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34410_e57286_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34410_e57286_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34410_e57286_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34410_e57286_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34410_e57286_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34410_e57286_d_n14;

        let (assign34420_e57298, assign34420_e57298_d_n0, assign34420_e57298_d_n2, assign34420_e57298_d_n3, assign34420_e57298_d_n4, assign34420_e57298_d_n5, assign34420_e57298_d_n6, assign34420_e57298_d_n7, assign34420_e57298_d_n8, assign34420_e57298_d_n9, assign34420_e57298_d_n10, assign34420_e57298_d_n11, assign34420_e57298_d_n13, assign34420_e57298_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34420_e57292: f64 = (locals.var_nfintotal * locals.var_weff0);
        let assign34420_e57294: f64 = (assign34420_e57292 * 1.60219e-19);
        let assign34420_e57296: f64 = (assign34420_e57294 * locals.var_vsatrsd_eff);
        (assign34420_e57296, (assign34420_e57294 * locals.var_vsatrsd_eff_dn0), (assign34420_e57294 * locals.var_vsatrsd_eff_dn2), (assign34420_e57294 * locals.var_vsatrsd_eff_dn3), (assign34420_e57294 * locals.var_vsatrsd_eff_dn4), (assign34420_e57294 * locals.var_vsatrsd_eff_dn5), (assign34420_e57294 * locals.var_vsatrsd_eff_dn6), (assign34420_e57294 * locals.var_vsatrsd_eff_dn7), (assign34420_e57294 * locals.var_vsatrsd_eff_dn8), (assign34420_e57294 * locals.var_vsatrsd_eff_dn9), (assign34420_e57294 * locals.var_vsatrsd_eff_dn10), (assign34420_e57294 * locals.var_vsatrsd_eff_dn11), (assign34420_e57294 * locals.var_vsatrsd_eff_dn13), (assign34420_e57294 * locals.var_vsatrsd_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34420_e57298;
        locals.var_t2_dn0 = assign34420_e57298_d_n0;
        locals.var_t2_dn2 = assign34420_e57298_d_n2;
        locals.var_t2_dn3 = assign34420_e57298_d_n3;
        locals.var_t2_dn4 = assign34420_e57298_d_n4;
        locals.var_t2_dn5 = assign34420_e57298_d_n5;
        locals.var_t2_dn6 = assign34420_e57298_d_n6;
        locals.var_t2_dn7 = assign34420_e57298_d_n7;
        locals.var_t2_dn8 = assign34420_e57298_d_n8;
        locals.var_t2_dn9 = assign34420_e57298_d_n9;
        locals.var_t2_dn10 = assign34420_e57298_d_n10;
        locals.var_t2_dn11 = assign34420_e57298_d_n11;
        locals.var_t2_dn13 = assign34420_e57298_d_n13;
        locals.var_t2_dn14 = assign34420_e57298_d_n14;

        let (assign34430_e57305, assign34430_e57305_d_n0, assign34430_e57305_d_n2, assign34430_e57305_d_n3, assign34430_e57305_d_n4, assign34430_e57305_d_n5, assign34430_e57305_d_n6, assign34430_e57305_d_n7, assign34430_e57305_d_n8, assign34430_e57305_d_n9, assign34430_e57305_d_n10, assign34430_e57305_d_n11, assign34430_e57305_d_n13, assign34430_e57305_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34430_e57303: f64 = ((nv9 - nv7)).abs();
        (assign34430_e57303, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, if (nv9 - nv7) >= 0.0 { -1.0 } else { 1.0 }, 0.0, if (nv9 - nv7) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34430_e57305;
        locals.var_t5_dn0 = assign34430_e57305_d_n0;
        locals.var_t5_dn2 = assign34430_e57305_d_n2;
        locals.var_t5_dn3 = assign34430_e57305_d_n3;
        locals.var_t5_dn4 = assign34430_e57305_d_n4;
        locals.var_t5_dn5 = assign34430_e57305_d_n5;
        locals.var_t5_dn6 = assign34430_e57305_d_n6;
        locals.var_t5_dn7 = assign34430_e57305_d_n7;
        locals.var_t5_dn8 = assign34430_e57305_d_n8;
        locals.var_t5_dn9 = assign34430_e57305_d_n9;
        locals.var_t5_dn10 = assign34430_e57305_d_n10;
        locals.var_t5_dn11 = assign34430_e57305_d_n11;
        locals.var_t5_dn13 = assign34430_e57305_d_n13;
        locals.var_t5_dn14 = assign34430_e57305_d_n14;

        let assign34440_e57308: f64 = if p.p1917 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign34440_e57308;

        let (assign34450_e57316, assign34450_e57316_d_n0, assign34450_e57316_d_n2, assign34450_e57316_d_n3, assign34450_e57316_d_n4, assign34450_e57316_d_n5, assign34450_e57316_d_n6, assign34450_e57316_d_n7, assign34450_e57316_d_n8, assign34450_e57316_d_n9, assign34450_e57316_d_n10, assign34450_e57316_d_n11, assign34450_e57316_d_n13, assign34450_e57316_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34450_e57316;
        locals.var_t3_dn0 = assign34450_e57316_d_n0;
        locals.var_t3_dn2 = assign34450_e57316_d_n2;
        locals.var_t3_dn3 = assign34450_e57316_d_n3;
        locals.var_t3_dn4 = assign34450_e57316_d_n4;
        locals.var_t3_dn5 = assign34450_e57316_d_n5;
        locals.var_t3_dn6 = assign34450_e57316_d_n6;
        locals.var_t3_dn7 = assign34450_e57316_d_n7;
        locals.var_t3_dn8 = assign34450_e57316_d_n8;
        locals.var_t3_dn9 = assign34450_e57316_d_n9;
        locals.var_t3_dn10 = assign34450_e57316_d_n10;
        locals.var_t3_dn11 = assign34450_e57316_d_n11;
        locals.var_t3_dn13 = assign34450_e57316_d_n13;
        locals.var_t3_dn14 = assign34450_e57316_d_n14;

        let (assign34460_e57350, assign34460_e57350_d_n0, assign34460_e57350_d_n2, assign34460_e57350_d_n3, assign34460_e57350_d_n4, assign34460_e57350_d_n5, assign34460_e57350_d_n6, assign34460_e57350_d_n7, assign34460_e57350_d_n8, assign34460_e57350_d_n9, assign34460_e57350_d_n10, assign34460_e57350_d_n11, assign34460_e57350_d_n13, assign34460_e57350_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign34460_e57326: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57328: f64 = assign34460_e57326;
        let assign34460_e57331: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57333: f64 = assign34460_e57331;
        let assign34460_e57336: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57338: f64 = assign34460_e57336;
        let assign34460_e57339: f64 = (assign34460_e57333 * assign34460_e57338);
        let assign34460_e57342: f64 = (0.25 * 0.5);
        let assign34460_e57344: f64 = (assign34460_e57342 * 0.5);
        let assign34460_e57345: f64 = (assign34460_e57339 + assign34460_e57344);
        let assign34460_e57346: f64 = (assign34460_e57345).sqrt();
        let assign34460_e57347: f64 = (assign34460_e57328 + assign34460_e57346);
        let assign34460_e57348: f64 = (0.5 * assign34460_e57347);
        (assign34460_e57348, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn0)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn2)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn3)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn4)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn5)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn6)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn7)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn8)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn9)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn10)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn11)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn13)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn14)) / (2.0 * assign34460_e57346)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34460_e57350;
        locals.var_t3_dn0 = assign34460_e57350_d_n0;
        locals.var_t3_dn2 = assign34460_e57350_d_n2;
        locals.var_t3_dn3 = assign34460_e57350_d_n3;
        locals.var_t3_dn4 = assign34460_e57350_d_n4;
        locals.var_t3_dn5 = assign34460_e57350_d_n5;
        locals.var_t3_dn6 = assign34460_e57350_d_n6;
        locals.var_t3_dn7 = assign34460_e57350_d_n7;
        locals.var_t3_dn8 = assign34460_e57350_d_n8;
        locals.var_t3_dn9 = assign34460_e57350_d_n9;
        locals.var_t3_dn10 = assign34460_e57350_d_n10;
        locals.var_t3_dn11 = assign34460_e57350_d_n11;
        locals.var_t3_dn13 = assign34460_e57350_d_n13;
        locals.var_t3_dn14 = assign34460_e57350_d_n14;

        let (assign34470_e57363, assign34470_e57363_d_n0, assign34470_e57363_d_n2, assign34470_e57363_d_n3, assign34470_e57363_d_n4, assign34470_e57363_d_n5, assign34470_e57363_d_n6, assign34470_e57363_d_n7, assign34470_e57363_d_n8, assign34470_e57363_d_n9, assign34470_e57363_d_n10, assign34470_e57363_d_n11, assign34470_e57363_d_n13, assign34470_e57363_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign34470_e57360: f64 = (locals.var_t3 * p.p1917);
        let assign34470_e57361: f64 = (1.0 + assign34470_e57360);
        (assign34470_e57361, (locals.var_t3_dn0 * p.p1917), (locals.var_t3_dn2 * p.p1917), (locals.var_t3_dn3 * p.p1917), (locals.var_t3_dn4 * p.p1917), (locals.var_t3_dn5 * p.p1917), (locals.var_t3_dn6 * p.p1917), (locals.var_t3_dn7 * p.p1917), (locals.var_t3_dn8 * p.p1917), (locals.var_t3_dn9 * p.p1917), (locals.var_t3_dn10 * p.p1917), (locals.var_t3_dn11 * p.p1917), (locals.var_t3_dn13 * p.p1917), (locals.var_t3_dn14 * p.p1917),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34470_e57363;
        locals.var_t3_dn0 = assign34470_e57363_d_n0;
        locals.var_t3_dn2 = assign34470_e57363_d_n2;
        locals.var_t3_dn3 = assign34470_e57363_d_n3;
        locals.var_t3_dn4 = assign34470_e57363_d_n4;
        locals.var_t3_dn5 = assign34470_e57363_d_n5;
        locals.var_t3_dn6 = assign34470_e57363_d_n6;
        locals.var_t3_dn7 = assign34470_e57363_d_n7;
        locals.var_t3_dn8 = assign34470_e57363_d_n8;
        locals.var_t3_dn9 = assign34470_e57363_d_n9;
        locals.var_t3_dn10 = assign34470_e57363_d_n10;
        locals.var_t3_dn11 = assign34470_e57363_d_n11;
        locals.var_t3_dn13 = assign34470_e57363_d_n13;
        locals.var_t3_dn14 = assign34470_e57363_d_n14;

        let (assign34480_e57373, assign34480_e57373_d_n0, assign34480_e57373_d_n2, assign34480_e57373_d_n3, assign34480_e57373_d_n4, assign34480_e57373_d_n5, assign34480_e57373_d_n6, assign34480_e57373_d_n7, assign34480_e57373_d_n8, assign34480_e57373_d_n9, assign34480_e57373_d_n10, assign34480_e57373_d_n11, assign34480_e57373_d_n13, assign34480_e57373_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34480_e57369: f64 = (locals.var_t2 * p.p1903);
        let assign34480_e57371: f64 = (assign34480_e57369 * locals.var_t3);
        (assign34480_e57371, (((locals.var_t2_dn0 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn0)), (((locals.var_t2_dn2 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn2)), (((locals.var_t2_dn3 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn3)), (((locals.var_t2_dn4 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn4)), (((locals.var_t2_dn5 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn5)), (((locals.var_t2_dn6 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn6)), (((locals.var_t2_dn7 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn7)), (((locals.var_t2_dn8 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn8)), (((locals.var_t2_dn9 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn9)), (((locals.var_t2_dn10 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn10)), (((locals.var_t2_dn11 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn11)), (((locals.var_t2_dn13 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn13)), (((locals.var_t2_dn14 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn14)),)
    } else {
        (locals.var_isat_rd, locals.var_isat_rd_dn0, locals.var_isat_rd_dn2, locals.var_isat_rd_dn3, locals.var_isat_rd_dn4, locals.var_isat_rd_dn5, locals.var_isat_rd_dn6, locals.var_isat_rd_dn7, locals.var_isat_rd_dn8, locals.var_isat_rd_dn9, locals.var_isat_rd_dn10, locals.var_isat_rd_dn11, locals.var_isat_rd_dn13, locals.var_isat_rd_dn14,)
    }
};
        locals.var_isat_rd = assign34480_e57373;
        locals.var_isat_rd_dn0 = assign34480_e57373_d_n0;
        locals.var_isat_rd_dn2 = assign34480_e57373_d_n2;
        locals.var_isat_rd_dn3 = assign34480_e57373_d_n3;
        locals.var_isat_rd_dn4 = assign34480_e57373_d_n4;
        locals.var_isat_rd_dn5 = assign34480_e57373_d_n5;
        locals.var_isat_rd_dn6 = assign34480_e57373_d_n6;
        locals.var_isat_rd_dn7 = assign34480_e57373_d_n7;
        locals.var_isat_rd_dn8 = assign34480_e57373_d_n8;
        locals.var_isat_rd_dn9 = assign34480_e57373_d_n9;
        locals.var_isat_rd_dn10 = assign34480_e57373_d_n10;
        locals.var_isat_rd_dn11 = assign34480_e57373_d_n11;
        locals.var_isat_rd_dn13 = assign34480_e57373_d_n13;
        locals.var_isat_rd_dn14 = assign34480_e57373_d_n14;

        let (assign34490_e57383, assign34490_e57383_d_n0, assign34490_e57383_d_n2, assign34490_e57383_d_n3, assign34490_e57383_d_n4, assign34490_e57383_d_n5, assign34490_e57383_d_n6, assign34490_e57383_d_n7, assign34490_e57383_d_n8, assign34490_e57383_d_n9, assign34490_e57383_d_n10, assign34490_e57383_d_n11, assign34490_e57383_d_n13, assign34490_e57383_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34490_e57379: f64 = (locals.var_rdstempvs * p.p1910);
        let assign34490_e57381: f64 = (assign34490_e57379 * locals.var_weffwrfactor);
        (assign34490_e57381, 0.0, 0.0, 0.0, ((locals.var_rdstempvs_dn4 * p.p1910) * locals.var_weffwrfactor), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34490_e57383;
        locals.var_t4_dn0 = assign34490_e57383_d_n0;
        locals.var_t4_dn2 = assign34490_e57383_d_n2;
        locals.var_t4_dn3 = assign34490_e57383_d_n3;
        locals.var_t4_dn4 = assign34490_e57383_d_n4;
        locals.var_t4_dn5 = assign34490_e57383_d_n5;
        locals.var_t4_dn6 = assign34490_e57383_d_n6;
        locals.var_t4_dn7 = assign34490_e57383_d_n7;
        locals.var_t4_dn8 = assign34490_e57383_d_n8;
        locals.var_t4_dn9 = assign34490_e57383_d_n9;
        locals.var_t4_dn10 = assign34490_e57383_d_n10;
        locals.var_t4_dn11 = assign34490_e57383_d_n11;
        locals.var_t4_dn13 = assign34490_e57383_d_n13;
        locals.var_t4_dn14 = assign34490_e57383_d_n14;

    }

    pub(super) fn stamp_transient_block_133(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34500_e57391, assign34500_e57391_d_n0, assign34500_e57391_d_n2, assign34500_e57391_d_n3, assign34500_e57391_d_n4, assign34500_e57391_d_n5, assign34500_e57391_d_n6, assign34500_e57391_d_n7, assign34500_e57391_d_n8, assign34500_e57391_d_n9, assign34500_e57391_d_n10, assign34500_e57391_d_n11, assign34500_e57391_d_n13, assign34500_e57391_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34500_e57389: f64 = (locals.var_isat_rd * locals.var_t4);
        (assign34500_e57389, ((locals.var_isat_rd_dn0 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn0)), ((locals.var_isat_rd_dn2 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn2)), ((locals.var_isat_rd_dn3 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn3)), ((locals.var_isat_rd_dn4 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn4)), ((locals.var_isat_rd_dn5 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn5)), ((locals.var_isat_rd_dn6 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn6)), ((locals.var_isat_rd_dn7 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn7)), ((locals.var_isat_rd_dn8 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn8)), ((locals.var_isat_rd_dn9 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn9)), ((locals.var_isat_rd_dn10 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn10)), ((locals.var_isat_rd_dn11 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn11)), ((locals.var_isat_rd_dn13 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn13)), ((locals.var_isat_rd_dn14 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn14)),)
    } else {
        (locals.var_vsat_rd, locals.var_vsat_rd_dn0, locals.var_vsat_rd_dn2, locals.var_vsat_rd_dn3, locals.var_vsat_rd_dn4, locals.var_vsat_rd_dn5, locals.var_vsat_rd_dn6, locals.var_vsat_rd_dn7, locals.var_vsat_rd_dn8, locals.var_vsat_rd_dn9, locals.var_vsat_rd_dn10, locals.var_vsat_rd_dn11, locals.var_vsat_rd_dn13, locals.var_vsat_rd_dn14,)
    }
};
        locals.var_vsat_rd = assign34500_e57391;
        locals.var_vsat_rd_dn0 = assign34500_e57391_d_n0;
        locals.var_vsat_rd_dn2 = assign34500_e57391_d_n2;
        locals.var_vsat_rd_dn3 = assign34500_e57391_d_n3;
        locals.var_vsat_rd_dn4 = assign34500_e57391_d_n4;
        locals.var_vsat_rd_dn5 = assign34500_e57391_d_n5;
        locals.var_vsat_rd_dn6 = assign34500_e57391_d_n6;
        locals.var_vsat_rd_dn7 = assign34500_e57391_d_n7;
        locals.var_vsat_rd_dn8 = assign34500_e57391_d_n8;
        locals.var_vsat_rd_dn9 = assign34500_e57391_d_n9;
        locals.var_vsat_rd_dn10 = assign34500_e57391_d_n10;
        locals.var_vsat_rd_dn11 = assign34500_e57391_d_n11;
        locals.var_vsat_rd_dn13 = assign34500_e57391_d_n13;
        locals.var_vsat_rd_dn14 = assign34500_e57391_d_n14;

        let (assign34510_e57415, assign34510_e57415_d_n0, assign34510_e57415_d_n2, assign34510_e57415_d_n3, assign34510_e57415_d_n4, assign34510_e57415_d_n5, assign34510_e57415_d_n6, assign34510_e57415_d_n7, assign34510_e57415_d_n8, assign34510_e57415_d_n9, assign34510_e57415_d_n10, assign34510_e57415_d_n11, assign34510_e57415_d_n13, assign34510_e57415_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34510_e57398: f64 = (4.0 - p.p1908);
        let assign34510_e57399: f64 = (locals.var_t5).powf(assign34510_e57398);
        let assign34510_e57403: f64 = (4.0 - p.p1908);
        let assign34510_e57404: f64 = (locals.var_t5).powf(assign34510_e57403);
        let assign34510_e57409: f64 = (4.0 - p.p1908);
        let assign34510_e57410: f64 = (locals.var_vsat_rd).powf(assign34510_e57409);
        let assign34510_e57411: f64 = (p.p1914 * assign34510_e57410);
        let assign34510_e57412: f64 = (assign34510_e57404 + assign34510_e57411);
        let assign34510_e57413: f64 = (assign34510_e57399 / assign34510_e57412);
        (assign34510_e57413, (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn0)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn0 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn0)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn0 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn0)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn0 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn2)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn2 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn2)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn2 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn2)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn2 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn3)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn3 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn3)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn3 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn3)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn3 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn4)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn4 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn4)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn4 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn4)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn4 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn5)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn5 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn5)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn5 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn5)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn5 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn6)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn6 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn6)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn6 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn6)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn6 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn7)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn7 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn7)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn7 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn7)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn7 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn8)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn8 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn8)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn8 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn8)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn8 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn9)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn9 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn9)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn9 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn9)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn9 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn10)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn10 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn10)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn10 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn10)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn10 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn11)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn11 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn11)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn11 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn11)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn11 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn13)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn13 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn13)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn13 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn13)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn13 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn14)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn14 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn14)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn14 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn14)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn14 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)),)
    } else {
        (locals.var_delta_vsrd, locals.var_delta_vsrd_dn0, locals.var_delta_vsrd_dn2, locals.var_delta_vsrd_dn3, locals.var_delta_vsrd_dn4, locals.var_delta_vsrd_dn5, locals.var_delta_vsrd_dn6, locals.var_delta_vsrd_dn7, locals.var_delta_vsrd_dn8, locals.var_delta_vsrd_dn9, locals.var_delta_vsrd_dn10, locals.var_delta_vsrd_dn11, locals.var_delta_vsrd_dn13, locals.var_delta_vsrd_dn14,)
    }
};
        locals.var_delta_vsrd = assign34510_e57415;
        locals.var_delta_vsrd_dn0 = assign34510_e57415_d_n0;
        locals.var_delta_vsrd_dn2 = assign34510_e57415_d_n2;
        locals.var_delta_vsrd_dn3 = assign34510_e57415_d_n3;
        locals.var_delta_vsrd_dn4 = assign34510_e57415_d_n4;
        locals.var_delta_vsrd_dn5 = assign34510_e57415_d_n5;
        locals.var_delta_vsrd_dn6 = assign34510_e57415_d_n6;
        locals.var_delta_vsrd_dn7 = assign34510_e57415_d_n7;
        locals.var_delta_vsrd_dn8 = assign34510_e57415_d_n8;
        locals.var_delta_vsrd_dn9 = assign34510_e57415_d_n9;
        locals.var_delta_vsrd_dn10 = assign34510_e57415_d_n10;
        locals.var_delta_vsrd_dn11 = assign34510_e57415_d_n11;
        locals.var_delta_vsrd_dn13 = assign34510_e57415_d_n13;
        locals.var_delta_vsrd_dn14 = assign34510_e57415_d_n14;

        let (assign34520_e57429, assign34520_e57429_d_n0, assign34520_e57429_d_n2, assign34520_e57429_d_n3, assign34520_e57429_d_n4, assign34520_e57429_d_n5, assign34520_e57429_d_n6, assign34520_e57429_d_n7, assign34520_e57429_d_n8, assign34520_e57429_d_n9, assign34520_e57429_d_n10, assign34520_e57429_d_n11, assign34520_e57429_d_n13, assign34520_e57429_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34520_e57422: f64 = (1.0 / p.p1908);
        let assign34520_e57423: f64 = (locals.var_delta_vsrd).powf(assign34520_e57422);
        let assign34520_e57425: f64 = (assign34520_e57423 * locals.var_t5);
        let assign34520_e57427: f64 = (assign34520_e57425 / locals.var_vsat_rd);
        (assign34520_e57427, (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn0)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn0 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn0)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn0)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn2)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn2 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn2)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn2)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn3)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn3 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn3)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn3)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn4)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn4 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn4)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn4)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn5)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn5 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn5)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn5)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn6)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn6 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn6)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn6)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn7)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn7 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn7)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn7)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn8)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn8 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn8)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn8)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn9)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn9 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn9)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn9)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn10)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn10 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn10)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn10)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn11)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn11 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn11)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn11)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn13)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn13 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn13)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn13)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn14)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn14 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn14)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn14)) / (locals.var_vsat_rd * locals.var_vsat_rd)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34520_e57429;
        locals.var_t6_dn0 = assign34520_e57429_d_n0;
        locals.var_t6_dn2 = assign34520_e57429_d_n2;
        locals.var_t6_dn3 = assign34520_e57429_d_n3;
        locals.var_t6_dn4 = assign34520_e57429_d_n4;
        locals.var_t6_dn5 = assign34520_e57429_d_n5;
        locals.var_t6_dn6 = assign34520_e57429_d_n6;
        locals.var_t6_dn7 = assign34520_e57429_d_n7;
        locals.var_t6_dn8 = assign34520_e57429_d_n8;
        locals.var_t6_dn9 = assign34520_e57429_d_n9;
        locals.var_t6_dn10 = assign34520_e57429_d_n10;
        locals.var_t6_dn11 = assign34520_e57429_d_n11;
        locals.var_t6_dn13 = assign34520_e57429_d_n13;
        locals.var_t6_dn14 = assign34520_e57429_d_n14;

        let (assign34530_e57445, assign34530_e57445_d_n0, assign34530_e57445_d_n2, assign34530_e57445_d_n3, assign34530_e57445_d_n4, assign34530_e57445_d_n5, assign34530_e57445_d_n6, assign34530_e57445_d_n7, assign34530_e57445_d_n8, assign34530_e57445_d_n9, assign34530_e57445_d_n10, assign34530_e57445_d_n11, assign34530_e57445_d_n13, assign34530_e57445_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34530_e57437: f64 = (locals.var_t6).powf(p.p1908);
        let assign34530_e57438: f64 = (1.0 + assign34530_e57437);
        let assign34530_e57441: f64 = (1.0 / p.p1908);
        let assign34530_e57442: f64 = (assign34530_e57438).powf(assign34530_e57441);
        let assign34530_e57443: f64 = (locals.var_t4 * assign34530_e57442);
        (assign34530_e57443, ((locals.var_t4_dn0 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn0)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn0 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn0)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn0 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn2 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn2)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn2 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn2)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn2 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn3 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn3)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn3 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn3)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn3 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn4 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn4)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn4 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn4)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn4 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn5 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn5)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn5 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn5)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn5 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn6 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn6)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn6 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn6)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn6 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn7 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn7)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn7 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn7)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn7 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn8 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn8)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn8 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn8)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn8 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn9 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn9)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn9 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn9)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn9 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn10 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn10)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn10 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn10)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn10 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn11 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn11)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn11 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn11)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn11 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn13 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn13)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn13 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn13)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn13 / locals.var_t6))) } / assign34530_e57438))) })), ((locals.var_t4_dn14 * assign34530_e57442) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn14)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn14 / locals.var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn14)) } } else { (assign34530_e57437 * (p.p1908 * (locals.var_t6_dn14 / locals.var_t6))) } / assign34530_e57438))) })),)
    } else {
        (locals.var_rvs_d, locals.var_rvs_d_dn0, locals.var_rvs_d_dn2, locals.var_rvs_d_dn3, locals.var_rvs_d_dn4, locals.var_rvs_d_dn5, locals.var_rvs_d_dn6, locals.var_rvs_d_dn7, locals.var_rvs_d_dn8, locals.var_rvs_d_dn9, locals.var_rvs_d_dn10, locals.var_rvs_d_dn11, locals.var_rvs_d_dn13, locals.var_rvs_d_dn14,)
    }
};
        locals.var_rvs_d = assign34530_e57445;
        locals.var_rvs_d_dn0 = assign34530_e57445_d_n0;
        locals.var_rvs_d_dn2 = assign34530_e57445_d_n2;
        locals.var_rvs_d_dn3 = assign34530_e57445_d_n3;
        locals.var_rvs_d_dn4 = assign34530_e57445_d_n4;
        locals.var_rvs_d_dn5 = assign34530_e57445_d_n5;
        locals.var_rvs_d_dn6 = assign34530_e57445_d_n6;
        locals.var_rvs_d_dn7 = assign34530_e57445_d_n7;
        locals.var_rvs_d_dn8 = assign34530_e57445_d_n8;
        locals.var_rvs_d_dn9 = assign34530_e57445_d_n9;
        locals.var_rvs_d_dn10 = assign34530_e57445_d_n10;
        locals.var_rvs_d_dn11 = assign34530_e57445_d_n11;
        locals.var_rvs_d_dn13 = assign34530_e57445_d_n13;
        locals.var_rvs_d_dn14 = assign34530_e57445_d_n14;

        let assign34540_e57448: f64 = if p.p1911 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign34540_e57448;

        let assign34550_e57451: f64 = if p.p1910 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign34550_e57451;

        let (assign34560_e57530, assign34560_e57530_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34560_e57460: f64 = (p.p1912 * locals.var_deltemp);
        let assign34560_e57461: f64 = (1.0 + assign34560_e57460);
        let assign34560_e57463: f64 = (assign34560_e57461 - 1e-6);
        let assign34560_e57465: f64 = (-10000.0);
        let assign34560_e57467: f64 = (assign34560_e57465 * 0.001);
        let (assign34560_e57528, assign34560_e57528_d_n4,) = {
            if (!(assign34560_e57463 < assign34560_e57467)) {
                let assign34560_e57474: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57475: f64 = (1.0 + assign34560_e57474);
                let assign34560_e57477: f64 = (assign34560_e57475 - 1e-6);
                let assign34560_e57481: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57482: f64 = (1.0 + assign34560_e57481);
                let assign34560_e57484: f64 = (assign34560_e57482 - 1e-6);
                let assign34560_e57488: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57489: f64 = (1.0 + assign34560_e57488);
                let assign34560_e57491: f64 = (assign34560_e57489 - 1e-6);
                let assign34560_e57492: f64 = (assign34560_e57484 * assign34560_e57491);
                let assign34560_e57495: f64 = (4.0 * 0.001);
                let assign34560_e57497: f64 = (assign34560_e57495 * 0.001);
                let assign34560_e57498: f64 = (assign34560_e57492 + assign34560_e57497);
                let assign34560_e57499: f64 = (assign34560_e57498).sqrt();
                let assign34560_e57500: f64 = (assign34560_e57477 + assign34560_e57499);
                let assign34560_e57501: f64 = (0.5 * assign34560_e57500);
                (assign34560_e57501, (0.5 * ((p.p1912 * locals.var_deltemp_dn4) + ((((p.p1912 * locals.var_deltemp_dn4) * assign34560_e57491) + (assign34560_e57484 * (p.p1912 * locals.var_deltemp_dn4))) / (2.0 * assign34560_e57499)))),)
            } else {
                let assign34560_e57505: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57506: f64 = (1.0 + assign34560_e57505);
                let assign34560_e57508: f64 = (assign34560_e57506 - 1e-6);
                let assign34560_e57510: f64 = (-10000.0);
                let assign34560_e57512: f64 = (assign34560_e57510 * 0.001);
                let (assign34560_e57527, assign34560_e57527_d_n4,) = {
                    if (assign34560_e57508 < assign34560_e57512) {
                        let assign34560_e57515: f64 = (-0.001);
                        let assign34560_e57517: f64 = (assign34560_e57515 * 0.001);
                        let assign34560_e57521: f64 = (p.p1912 * locals.var_deltemp);
                        let assign34560_e57522: f64 = (1.0 + assign34560_e57521);
                        let assign34560_e57524: f64 = (assign34560_e57522 - 1e-6);
                        let assign34560_e57525: f64 = (assign34560_e57517 / assign34560_e57524);
                        (assign34560_e57525, (-((assign34560_e57517 * (p.p1912 * locals.var_deltemp_dn4)) / (assign34560_e57524 * assign34560_e57524))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34560_e57527, assign34560_e57527_d_n4,)
            }
        };
        (assign34560_e57528, assign34560_e57528_d_n4,)
    } else {
        (locals.var_rdstempvs, locals.var_rdstempvs_dn4,)
    }
};
        locals.var_rdstempvs = assign34560_e57530;
        locals.var_rdstempvs_dn4 = assign34560_e57530_d_n4;

        let assign34570_e57533: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign34570_e57533;

        let (assign34580_e57586, assign34580_e57586_d_n4,) = {
    if ((((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign34580_e57543: f64 = (-p.p1904);
        let assign34580_e57546: f64 = (-p.p1913);
        let assign34580_e57548: f64 = (assign34580_e57546 * locals.var_deltemp);
        let assign34580_e57550: f64 = (-p.p1904);
        let assign34580_e57551: f64 = (assign34580_e57548 - assign34580_e57550);
        let assign34580_e57553: f64 = (assign34580_e57551 - 1e-6);
        let assign34580_e57555: f64 = (-p.p1913);
        let assign34580_e57557: f64 = (assign34580_e57555 * locals.var_deltemp);
        let assign34580_e57559: f64 = (-p.p1904);
        let assign34580_e57560: f64 = (assign34580_e57557 - assign34580_e57559);
        let assign34580_e57562: f64 = (assign34580_e57560 - 1e-6);
        let assign34580_e57564: f64 = (-p.p1913);
        let assign34580_e57566: f64 = (assign34580_e57564 * locals.var_deltemp);
        let assign34580_e57568: f64 = (-p.p1904);
        let assign34580_e57569: f64 = (assign34580_e57566 - assign34580_e57568);
        let assign34580_e57571: f64 = (assign34580_e57569 - 1e-6);
        let assign34580_e57572: f64 = (assign34580_e57562 * assign34580_e57571);
        let assign34580_e57575: f64 = (-p.p1904);
        let assign34580_e57576: f64 = (4.0 * assign34580_e57575);
        let assign34580_e57578: f64 = (assign34580_e57576 * 1e-6);
        let assign34580_e57579: f64 = (assign34580_e57572 - assign34580_e57578);
        let assign34580_e57580: f64 = (assign34580_e57579).sqrt();
        let assign34580_e57581: f64 = (assign34580_e57553 + assign34580_e57580);
        let assign34580_e57582: f64 = (0.5 * assign34580_e57581);
        let assign34580_e57583: f64 = (assign34580_e57543 + assign34580_e57582);
        let assign34580_e57584: f64 = (p.p1904 + assign34580_e57583);
        (assign34580_e57584, (0.5 * ((assign34580_e57546 * locals.var_deltemp_dn4) + ((((assign34580_e57555 * locals.var_deltemp_dn4) * assign34580_e57571) + (assign34580_e57562 * (assign34580_e57564 * locals.var_deltemp_dn4))) / (2.0 * assign34580_e57580)))),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34580_e57586;
        locals.var_vsatrsd_t_dn4 = assign34580_e57586_d_n4;

        let (assign34590_e57676, assign34590_e57676_d_n4,) = {
    if ((((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) {
        let assign34590_e57598: f64 = (-p.p1913);
        let assign34590_e57600: f64 = (assign34590_e57598 * locals.var_deltemp);
        let assign34590_e57601: f64 = (1.0 + assign34590_e57600);
        let assign34590_e57603: f64 = (assign34590_e57601 - 1e-6);
        let assign34590_e57605: f64 = (-10000.0);
        let assign34590_e57607: f64 = (assign34590_e57605 * 0.001);
        let (assign34590_e57673, assign34590_e57673_d_n4,) = {
            if (!(assign34590_e57603 < assign34590_e57607)) {
                let assign34590_e57613: f64 = (-p.p1913);
                let assign34590_e57615: f64 = (assign34590_e57613 * locals.var_deltemp);
                let assign34590_e57616: f64 = (1.0 + assign34590_e57615);
                let assign34590_e57618: f64 = (assign34590_e57616 - 1e-6);
                let assign34590_e57621: f64 = (-p.p1913);
                let assign34590_e57623: f64 = (assign34590_e57621 * locals.var_deltemp);
                let assign34590_e57624: f64 = (1.0 + assign34590_e57623);
                let assign34590_e57626: f64 = (assign34590_e57624 - 1e-6);
                let assign34590_e57629: f64 = (-p.p1913);
                let assign34590_e57631: f64 = (assign34590_e57629 * locals.var_deltemp);
                let assign34590_e57632: f64 = (1.0 + assign34590_e57631);
                let assign34590_e57634: f64 = (assign34590_e57632 - 1e-6);
                let assign34590_e57635: f64 = (assign34590_e57626 * assign34590_e57634);
                let assign34590_e57638: f64 = (4.0 * 0.001);
                let assign34590_e57640: f64 = (assign34590_e57638 * 0.001);
                let assign34590_e57641: f64 = (assign34590_e57635 + assign34590_e57640);
                let assign34590_e57642: f64 = (assign34590_e57641).sqrt();
                let assign34590_e57643: f64 = (assign34590_e57618 + assign34590_e57642);
                let assign34590_e57644: f64 = (0.5 * assign34590_e57643);
                (assign34590_e57644, (0.5 * ((assign34590_e57613 * locals.var_deltemp_dn4) + ((((assign34590_e57621 * locals.var_deltemp_dn4) * assign34590_e57634) + (assign34590_e57626 * (assign34590_e57629 * locals.var_deltemp_dn4))) / (2.0 * assign34590_e57642)))),)
            } else {
                let assign34590_e57647: f64 = (-p.p1913);
                let assign34590_e57649: f64 = (assign34590_e57647 * locals.var_deltemp);
                let assign34590_e57650: f64 = (1.0 + assign34590_e57649);
                let assign34590_e57652: f64 = (assign34590_e57650 - 1e-6);
                let assign34590_e57654: f64 = (-10000.0);
                let assign34590_e57656: f64 = (assign34590_e57654 * 0.001);
                let (assign34590_e57672, assign34590_e57672_d_n4,) = {
                    if (assign34590_e57652 < assign34590_e57656) {
                        let assign34590_e57659: f64 = (-0.001);
                        let assign34590_e57661: f64 = (assign34590_e57659 * 0.001);
                        let assign34590_e57664: f64 = (-p.p1913);
                        let assign34590_e57666: f64 = (assign34590_e57664 * locals.var_deltemp);
                        let assign34590_e57667: f64 = (1.0 + assign34590_e57666);
                        let assign34590_e57669: f64 = (assign34590_e57667 - 1e-6);
                        let assign34590_e57670: f64 = (assign34590_e57661 / assign34590_e57669);
                        (assign34590_e57670, (-((assign34590_e57661 * (assign34590_e57664 * locals.var_deltemp_dn4)) / (assign34590_e57669 * assign34590_e57669))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34590_e57672, assign34590_e57672_d_n4,)
            }
        };
        let assign34590_e57674: f64 = (p.p1904 * assign34590_e57673);
        (assign34590_e57674, (p.p1904 * assign34590_e57673_d_n4),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34590_e57676;
        locals.var_vsatrsd_t_dn4 = assign34590_e57676_d_n4;

        let (assign34600_e57686, assign34600_e57686_d_n0, assign34600_e57686_d_n2, assign34600_e57686_d_n3, assign34600_e57686_d_n4, assign34600_e57686_d_n5, assign34600_e57686_d_n6, assign34600_e57686_d_n7, assign34600_e57686_d_n8, assign34600_e57686_d_n9, assign34600_e57686_d_n10, assign34600_e57686_d_n11, assign34600_e57686_d_n13, assign34600_e57686_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34600_e57684: f64 = (locals.var_qis - p.p1906);
        (assign34600_e57684, locals.var_qis_dn0, locals.var_qis_dn2, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11, locals.var_qis_dn13, locals.var_qis_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34600_e57686;
        locals.var_t0_dn0 = assign34600_e57686_d_n0;
        locals.var_t0_dn2 = assign34600_e57686_d_n2;
        locals.var_t0_dn3 = assign34600_e57686_d_n3;
        locals.var_t0_dn4 = assign34600_e57686_d_n4;
        locals.var_t0_dn5 = assign34600_e57686_d_n5;
        locals.var_t0_dn6 = assign34600_e57686_d_n6;
        locals.var_t0_dn7 = assign34600_e57686_d_n7;
        locals.var_t0_dn8 = assign34600_e57686_d_n8;
        locals.var_t0_dn9 = assign34600_e57686_d_n9;
        locals.var_t0_dn10 = assign34600_e57686_d_n10;
        locals.var_t0_dn11 = assign34600_e57686_d_n11;
        locals.var_t0_dn13 = assign34600_e57686_d_n13;
        locals.var_t0_dn14 = assign34600_e57686_d_n14;

        let (assign34610_e57713, assign34610_e57713_d_n0, assign34610_e57713_d_n2, assign34610_e57713_d_n3, assign34610_e57713_d_n4, assign34610_e57713_d_n5, assign34610_e57713_d_n6, assign34610_e57713_d_n7, assign34610_e57713_d_n8, assign34610_e57713_d_n9, assign34610_e57713_d_n10, assign34610_e57713_d_n11, assign34610_e57713_d_n13, assign34610_e57713_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34610_e57695: f64 = (locals.var_t0 + 0.1);
        let assign34610_e57698: f64 = (locals.var_t0 - 0.1);
        let assign34610_e57701: f64 = (locals.var_t0 - 0.1);
        let assign34610_e57702: f64 = (assign34610_e57698 * assign34610_e57701);
        let assign34610_e57705: f64 = (0.25 * 2.0);
        let assign34610_e57707: f64 = (assign34610_e57705 * 2.0);
        let assign34610_e57708: f64 = (assign34610_e57702 + assign34610_e57707);
        let assign34610_e57709: f64 = (assign34610_e57708).sqrt();
        let assign34610_e57710: f64 = (assign34610_e57695 + assign34610_e57709);
        let assign34610_e57711: f64 = (0.5 * assign34610_e57710);
        (assign34610_e57711, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn0)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn2)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn3)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn4)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn5)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn6)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn7)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn8)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn9)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn10)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn11)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn13)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn14)) / (2.0 * assign34610_e57709)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34610_e57713;
        locals.var_t0_dn0 = assign34610_e57713_d_n0;
        locals.var_t0_dn2 = assign34610_e57713_d_n2;
        locals.var_t0_dn3 = assign34610_e57713_d_n3;
        locals.var_t0_dn4 = assign34610_e57713_d_n4;
        locals.var_t0_dn5 = assign34610_e57713_d_n5;
        locals.var_t0_dn6 = assign34610_e57713_d_n6;
        locals.var_t0_dn7 = assign34610_e57713_d_n7;
        locals.var_t0_dn8 = assign34610_e57713_d_n8;
        locals.var_t0_dn9 = assign34610_e57713_d_n9;
        locals.var_t0_dn10 = assign34610_e57713_d_n10;
        locals.var_t0_dn11 = assign34610_e57713_d_n11;
        locals.var_t0_dn13 = assign34610_e57713_d_n13;
        locals.var_t0_dn14 = assign34610_e57713_d_n14;

        let (assign34620_e57731, assign34620_e57731_d_n0, assign34620_e57731_d_n2, assign34620_e57731_d_n3, assign34620_e57731_d_n4, assign34620_e57731_d_n5, assign34620_e57731_d_n6, assign34620_e57731_d_n7, assign34620_e57731_d_n8, assign34620_e57731_d_n9, assign34620_e57731_d_n10, assign34620_e57731_d_n11, assign34620_e57731_d_n13, assign34620_e57731_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34620_e57721: f64 = (10.0 * p.p1907);
        let assign34620_e57723: f64 = (assign34620_e57721 * locals.var_t0);
        let assign34620_e57726: f64 = (10.0 * p.p1907);
        let assign34620_e57728: f64 = (assign34620_e57726 + locals.var_t0);
        let assign34620_e57729: f64 = (assign34620_e57723 / assign34620_e57728);
        (assign34620_e57729, ((((assign34620_e57721 * locals.var_t0_dn0) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn0)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn2) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn2)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn3) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn3)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn4) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn4)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn5) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn5)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn6) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn6)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn7) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn7)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn8) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn8)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn9) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn9)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn10) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn10)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn11) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn11)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn13) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn13)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn14) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn14)) / (assign34620_e57728 * assign34620_e57728)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34620_e57731;
        locals.var_t1_dn0 = assign34620_e57731_d_n0;
        locals.var_t1_dn2 = assign34620_e57731_d_n2;
        locals.var_t1_dn3 = assign34620_e57731_d_n3;
        locals.var_t1_dn4 = assign34620_e57731_d_n4;
        locals.var_t1_dn5 = assign34620_e57731_d_n5;
        locals.var_t1_dn6 = assign34620_e57731_d_n6;
        locals.var_t1_dn7 = assign34620_e57731_d_n7;
        locals.var_t1_dn8 = assign34620_e57731_d_n8;
        locals.var_t1_dn9 = assign34620_e57731_d_n9;
        locals.var_t1_dn10 = assign34620_e57731_d_n10;
        locals.var_t1_dn11 = assign34620_e57731_d_n11;
        locals.var_t1_dn13 = assign34620_e57731_d_n13;
        locals.var_t1_dn14 = assign34620_e57731_d_n14;

        let (assign34630_e57745, assign34630_e57745_d_n0, assign34630_e57745_d_n2, assign34630_e57745_d_n3, assign34630_e57745_d_n4, assign34630_e57745_d_n5, assign34630_e57745_d_n6, assign34630_e57745_d_n7, assign34630_e57745_d_n8, assign34630_e57745_d_n9, assign34630_e57745_d_n10, assign34630_e57745_d_n11, assign34630_e57745_d_n13, assign34630_e57745_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34630_e57741: f64 = (p.p1905 * locals.var_t1);
        let assign34630_e57742: f64 = (1.0 + assign34630_e57741);
        let assign34630_e57743: f64 = (locals.var_vsatrsd_t * assign34630_e57742);
        (assign34630_e57743, (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn0)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn2)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn3)), ((locals.var_vsatrsd_t_dn4 * assign34630_e57742) + (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn4))), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn5)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn6)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn7)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn8)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn9)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn10)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn11)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn13)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn14)),)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34630_e57745;
        locals.var_vsatrsd_eff_dn0 = assign34630_e57745_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34630_e57745_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34630_e57745_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34630_e57745_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34630_e57745_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34630_e57745_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34630_e57745_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34630_e57745_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34630_e57745_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34630_e57745_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34630_e57745_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34630_e57745_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34630_e57745_d_n14;

        let (assign34640_e57788, assign34640_e57788_d_n0, assign34640_e57788_d_n2, assign34640_e57788_d_n3, assign34640_e57788_d_n4, assign34640_e57788_d_n5, assign34640_e57788_d_n6, assign34640_e57788_d_n7, assign34640_e57788_d_n8, assign34640_e57788_d_n9, assign34640_e57788_d_n10, assign34640_e57788_d_n11, assign34640_e57788_d_n13, assign34640_e57788_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34640_e57753: f64 = (-10000.0);
        let assign34640_e57755: f64 = (assign34640_e57753 * 10.0);
        let (assign34640_e57786, assign34640_e57786_d_n0, assign34640_e57786_d_n2, assign34640_e57786_d_n3, assign34640_e57786_d_n4, assign34640_e57786_d_n5, assign34640_e57786_d_n6, assign34640_e57786_d_n7, assign34640_e57786_d_n8, assign34640_e57786_d_n9, assign34640_e57786_d_n10, assign34640_e57786_d_n11, assign34640_e57786_d_n13, assign34640_e57786_d_n14,) = {
            if (!(locals.var_vsatrsd_eff < assign34640_e57755)) {
                let assign34640_e57762: f64 = (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff);
                let assign34640_e57765: f64 = (4.0 * 10.0);
                let assign34640_e57767: f64 = (assign34640_e57765 * 10.0);
                let assign34640_e57768: f64 = (assign34640_e57762 + assign34640_e57767);
                let assign34640_e57769: f64 = (assign34640_e57768).sqrt();
                let assign34640_e57770: f64 = (locals.var_vsatrsd_eff + assign34640_e57769);
                let assign34640_e57771: f64 = (0.5 * assign34640_e57770);
                (assign34640_e57771, (0.5 * (locals.var_vsatrsd_eff_dn0 + (((locals.var_vsatrsd_eff_dn0 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn0)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn2 + (((locals.var_vsatrsd_eff_dn2 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn2)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn3 + (((locals.var_vsatrsd_eff_dn3 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn3)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn4 + (((locals.var_vsatrsd_eff_dn4 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn4)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn5 + (((locals.var_vsatrsd_eff_dn5 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn5)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn6 + (((locals.var_vsatrsd_eff_dn6 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn6)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn7 + (((locals.var_vsatrsd_eff_dn7 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn7)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn8 + (((locals.var_vsatrsd_eff_dn8 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn8)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn9 + (((locals.var_vsatrsd_eff_dn9 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn9)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn10 + (((locals.var_vsatrsd_eff_dn10 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn10)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn11 + (((locals.var_vsatrsd_eff_dn11 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn11)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn13 + (((locals.var_vsatrsd_eff_dn13 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn13)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn14 + (((locals.var_vsatrsd_eff_dn14 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn14)) / (2.0 * assign34640_e57769)))),)
            } else {
                let assign34640_e57774: f64 = (-10000.0);
                let assign34640_e57776: f64 = (assign34640_e57774 * 10.0);
                let (assign34640_e57785, assign34640_e57785_d_n0, assign34640_e57785_d_n2, assign34640_e57785_d_n3, assign34640_e57785_d_n4, assign34640_e57785_d_n5, assign34640_e57785_d_n6, assign34640_e57785_d_n7, assign34640_e57785_d_n8, assign34640_e57785_d_n9, assign34640_e57785_d_n10, assign34640_e57785_d_n11, assign34640_e57785_d_n13, assign34640_e57785_d_n14,) = {
                    if (locals.var_vsatrsd_eff < assign34640_e57776) {
                        let assign34640_e57779: f64 = (-10.0);
                        let assign34640_e57781: f64 = (assign34640_e57779 * 10.0);
                        let assign34640_e57783: f64 = (assign34640_e57781 / locals.var_vsatrsd_eff);
                        (assign34640_e57783, (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn0) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn2) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn3) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn4) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn5) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn6) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn7) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn8) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn9) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn10) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn11) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn13) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn14) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign34640_e57785, assign34640_e57785_d_n0, assign34640_e57785_d_n2, assign34640_e57785_d_n3, assign34640_e57785_d_n4, assign34640_e57785_d_n5, assign34640_e57785_d_n6, assign34640_e57785_d_n7, assign34640_e57785_d_n8, assign34640_e57785_d_n9, assign34640_e57785_d_n10, assign34640_e57785_d_n11, assign34640_e57785_d_n13, assign34640_e57785_d_n14,)
            }
        };
        (assign34640_e57786, assign34640_e57786_d_n0, assign34640_e57786_d_n2, assign34640_e57786_d_n3, assign34640_e57786_d_n4, assign34640_e57786_d_n5, assign34640_e57786_d_n6, assign34640_e57786_d_n7, assign34640_e57786_d_n8, assign34640_e57786_d_n9, assign34640_e57786_d_n10, assign34640_e57786_d_n11, assign34640_e57786_d_n13, assign34640_e57786_d_n14,)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34640_e57788;
        locals.var_vsatrsd_eff_dn0 = assign34640_e57788_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34640_e57788_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34640_e57788_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34640_e57788_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34640_e57788_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34640_e57788_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34640_e57788_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34640_e57788_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34640_e57788_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34640_e57788_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34640_e57788_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34640_e57788_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34640_e57788_d_n14;

        let (assign34650_e57802, assign34650_e57802_d_n0, assign34650_e57802_d_n2, assign34650_e57802_d_n3, assign34650_e57802_d_n4, assign34650_e57802_d_n5, assign34650_e57802_d_n6, assign34650_e57802_d_n7, assign34650_e57802_d_n8, assign34650_e57802_d_n9, assign34650_e57802_d_n10, assign34650_e57802_d_n11, assign34650_e57802_d_n13, assign34650_e57802_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34650_e57796: f64 = (locals.var_nfintotal * locals.var_weff0);
        let assign34650_e57798: f64 = (assign34650_e57796 * 1.60219e-19);
        let assign34650_e57800: f64 = (assign34650_e57798 * locals.var_vsatrsd_eff);
        (assign34650_e57800, (assign34650_e57798 * locals.var_vsatrsd_eff_dn0), (assign34650_e57798 * locals.var_vsatrsd_eff_dn2), (assign34650_e57798 * locals.var_vsatrsd_eff_dn3), (assign34650_e57798 * locals.var_vsatrsd_eff_dn4), (assign34650_e57798 * locals.var_vsatrsd_eff_dn5), (assign34650_e57798 * locals.var_vsatrsd_eff_dn6), (assign34650_e57798 * locals.var_vsatrsd_eff_dn7), (assign34650_e57798 * locals.var_vsatrsd_eff_dn8), (assign34650_e57798 * locals.var_vsatrsd_eff_dn9), (assign34650_e57798 * locals.var_vsatrsd_eff_dn10), (assign34650_e57798 * locals.var_vsatrsd_eff_dn11), (assign34650_e57798 * locals.var_vsatrsd_eff_dn13), (assign34650_e57798 * locals.var_vsatrsd_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34650_e57802;
        locals.var_t2_dn0 = assign34650_e57802_d_n0;
        locals.var_t2_dn2 = assign34650_e57802_d_n2;
        locals.var_t2_dn3 = assign34650_e57802_d_n3;
        locals.var_t2_dn4 = assign34650_e57802_d_n4;
        locals.var_t2_dn5 = assign34650_e57802_d_n5;
        locals.var_t2_dn6 = assign34650_e57802_d_n6;
        locals.var_t2_dn7 = assign34650_e57802_d_n7;
        locals.var_t2_dn8 = assign34650_e57802_d_n8;
        locals.var_t2_dn9 = assign34650_e57802_d_n9;
        locals.var_t2_dn10 = assign34650_e57802_d_n10;
        locals.var_t2_dn11 = assign34650_e57802_d_n11;
        locals.var_t2_dn13 = assign34650_e57802_d_n13;
        locals.var_t2_dn14 = assign34650_e57802_d_n14;

        let (assign34660_e57810, assign34660_e57810_d_n0, assign34660_e57810_d_n2, assign34660_e57810_d_n3, assign34660_e57810_d_n4, assign34660_e57810_d_n5, assign34660_e57810_d_n6, assign34660_e57810_d_n7, assign34660_e57810_d_n8, assign34660_e57810_d_n9, assign34660_e57810_d_n10, assign34660_e57810_d_n11, assign34660_e57810_d_n13, assign34660_e57810_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34660_e57808: f64 = (locals.var_t2 * p.p1909);
        (assign34660_e57808, (locals.var_t2_dn0 * p.p1909), (locals.var_t2_dn2 * p.p1909), (locals.var_t2_dn3 * p.p1909), (locals.var_t2_dn4 * p.p1909), (locals.var_t2_dn5 * p.p1909), (locals.var_t2_dn6 * p.p1909), (locals.var_t2_dn7 * p.p1909), (locals.var_t2_dn8 * p.p1909), (locals.var_t2_dn9 * p.p1909), (locals.var_t2_dn10 * p.p1909), (locals.var_t2_dn11 * p.p1909), (locals.var_t2_dn13 * p.p1909), (locals.var_t2_dn14 * p.p1909),)
    } else {
        (locals.var_isat_rs, locals.var_isat_rs_dn0, locals.var_isat_rs_dn2, locals.var_isat_rs_dn3, locals.var_isat_rs_dn4, locals.var_isat_rs_dn5, locals.var_isat_rs_dn6, locals.var_isat_rs_dn7, locals.var_isat_rs_dn8, locals.var_isat_rs_dn9, locals.var_isat_rs_dn10, locals.var_isat_rs_dn11, locals.var_isat_rs_dn13, locals.var_isat_rs_dn14,)
    }
};
        locals.var_isat_rs = assign34660_e57810;
        locals.var_isat_rs_dn0 = assign34660_e57810_d_n0;
        locals.var_isat_rs_dn2 = assign34660_e57810_d_n2;
        locals.var_isat_rs_dn3 = assign34660_e57810_d_n3;
        locals.var_isat_rs_dn4 = assign34660_e57810_d_n4;
        locals.var_isat_rs_dn5 = assign34660_e57810_d_n5;
        locals.var_isat_rs_dn6 = assign34660_e57810_d_n6;
        locals.var_isat_rs_dn7 = assign34660_e57810_d_n7;
        locals.var_isat_rs_dn8 = assign34660_e57810_d_n8;
        locals.var_isat_rs_dn9 = assign34660_e57810_d_n9;
        locals.var_isat_rs_dn10 = assign34660_e57810_d_n10;
        locals.var_isat_rs_dn11 = assign34660_e57810_d_n11;
        locals.var_isat_rs_dn13 = assign34660_e57810_d_n13;
        locals.var_isat_rs_dn14 = assign34660_e57810_d_n14;

        let (assign34670_e57820, assign34670_e57820_d_n0, assign34670_e57820_d_n2, assign34670_e57820_d_n3, assign34670_e57820_d_n4, assign34670_e57820_d_n5, assign34670_e57820_d_n6, assign34670_e57820_d_n7, assign34670_e57820_d_n8, assign34670_e57820_d_n9, assign34670_e57820_d_n10, assign34670_e57820_d_n11, assign34670_e57820_d_n13, assign34670_e57820_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34670_e57816: f64 = (locals.var_rdstempvs * p.p1911);
        let assign34670_e57818: f64 = (assign34670_e57816 * locals.var_weffwrfactor);
        (assign34670_e57818, 0.0, 0.0, 0.0, ((locals.var_rdstempvs_dn4 * p.p1911) * locals.var_weffwrfactor), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34670_e57820;
        locals.var_t4_dn0 = assign34670_e57820_d_n0;
        locals.var_t4_dn2 = assign34670_e57820_d_n2;
        locals.var_t4_dn3 = assign34670_e57820_d_n3;
        locals.var_t4_dn4 = assign34670_e57820_d_n4;
        locals.var_t4_dn5 = assign34670_e57820_d_n5;
        locals.var_t4_dn6 = assign34670_e57820_d_n6;
        locals.var_t4_dn7 = assign34670_e57820_d_n7;
        locals.var_t4_dn8 = assign34670_e57820_d_n8;
        locals.var_t4_dn9 = assign34670_e57820_d_n9;
        locals.var_t4_dn10 = assign34670_e57820_d_n10;
        locals.var_t4_dn11 = assign34670_e57820_d_n11;
        locals.var_t4_dn13 = assign34670_e57820_d_n13;
        locals.var_t4_dn14 = assign34670_e57820_d_n14;

    }

    pub(super) fn stamp_transient_block_134(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign34680_e57828, assign34680_e57828_d_n0, assign34680_e57828_d_n2, assign34680_e57828_d_n3, assign34680_e57828_d_n4, assign34680_e57828_d_n5, assign34680_e57828_d_n6, assign34680_e57828_d_n7, assign34680_e57828_d_n8, assign34680_e57828_d_n9, assign34680_e57828_d_n10, assign34680_e57828_d_n11, assign34680_e57828_d_n13, assign34680_e57828_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34680_e57826: f64 = (locals.var_isat_rs * locals.var_t4);
        (assign34680_e57826, ((locals.var_isat_rs_dn0 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn0)), ((locals.var_isat_rs_dn2 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn2)), ((locals.var_isat_rs_dn3 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn3)), ((locals.var_isat_rs_dn4 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn4)), ((locals.var_isat_rs_dn5 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn5)), ((locals.var_isat_rs_dn6 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn6)), ((locals.var_isat_rs_dn7 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn7)), ((locals.var_isat_rs_dn8 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn8)), ((locals.var_isat_rs_dn9 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn9)), ((locals.var_isat_rs_dn10 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn10)), ((locals.var_isat_rs_dn11 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn11)), ((locals.var_isat_rs_dn13 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn13)), ((locals.var_isat_rs_dn14 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn14)),)
    } else {
        (locals.var_vsat_rs, locals.var_vsat_rs_dn0, locals.var_vsat_rs_dn2, locals.var_vsat_rs_dn3, locals.var_vsat_rs_dn4, locals.var_vsat_rs_dn5, locals.var_vsat_rs_dn6, locals.var_vsat_rs_dn7, locals.var_vsat_rs_dn8, locals.var_vsat_rs_dn9, locals.var_vsat_rs_dn10, locals.var_vsat_rs_dn11, locals.var_vsat_rs_dn13, locals.var_vsat_rs_dn14,)
    }
};
        locals.var_vsat_rs = assign34680_e57828;
        locals.var_vsat_rs_dn0 = assign34680_e57828_d_n0;
        locals.var_vsat_rs_dn2 = assign34680_e57828_d_n2;
        locals.var_vsat_rs_dn3 = assign34680_e57828_d_n3;
        locals.var_vsat_rs_dn4 = assign34680_e57828_d_n4;
        locals.var_vsat_rs_dn5 = assign34680_e57828_d_n5;
        locals.var_vsat_rs_dn6 = assign34680_e57828_d_n6;
        locals.var_vsat_rs_dn7 = assign34680_e57828_d_n7;
        locals.var_vsat_rs_dn8 = assign34680_e57828_d_n8;
        locals.var_vsat_rs_dn9 = assign34680_e57828_d_n9;
        locals.var_vsat_rs_dn10 = assign34680_e57828_d_n10;
        locals.var_vsat_rs_dn11 = assign34680_e57828_d_n11;
        locals.var_vsat_rs_dn13 = assign34680_e57828_d_n13;
        locals.var_vsat_rs_dn14 = assign34680_e57828_d_n14;

        let (assign34690_e57835, assign34690_e57835_d_n0, assign34690_e57835_d_n2, assign34690_e57835_d_n3, assign34690_e57835_d_n4, assign34690_e57835_d_n5, assign34690_e57835_d_n6, assign34690_e57835_d_n7, assign34690_e57835_d_n8, assign34690_e57835_d_n9, assign34690_e57835_d_n10, assign34690_e57835_d_n11, assign34690_e57835_d_n13, assign34690_e57835_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34690_e57833: f64 = ((nv6 - nv8)).abs();
        (assign34690_e57833, 0.0, 0.0, 0.0, 0.0, 0.0, if (nv6 - nv8) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, if (nv6 - nv8) >= 0.0 { -1.0 } else { 1.0 }, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34690_e57835;
        locals.var_t5_dn0 = assign34690_e57835_d_n0;
        locals.var_t5_dn2 = assign34690_e57835_d_n2;
        locals.var_t5_dn3 = assign34690_e57835_d_n3;
        locals.var_t5_dn4 = assign34690_e57835_d_n4;
        locals.var_t5_dn5 = assign34690_e57835_d_n5;
        locals.var_t5_dn6 = assign34690_e57835_d_n6;
        locals.var_t5_dn7 = assign34690_e57835_d_n7;
        locals.var_t5_dn8 = assign34690_e57835_d_n8;
        locals.var_t5_dn9 = assign34690_e57835_d_n9;
        locals.var_t5_dn10 = assign34690_e57835_d_n10;
        locals.var_t5_dn11 = assign34690_e57835_d_n11;
        locals.var_t5_dn13 = assign34690_e57835_d_n13;
        locals.var_t5_dn14 = assign34690_e57835_d_n14;

        let (assign34700_e57859, assign34700_e57859_d_n0, assign34700_e57859_d_n2, assign34700_e57859_d_n3, assign34700_e57859_d_n4, assign34700_e57859_d_n5, assign34700_e57859_d_n6, assign34700_e57859_d_n7, assign34700_e57859_d_n8, assign34700_e57859_d_n9, assign34700_e57859_d_n10, assign34700_e57859_d_n11, assign34700_e57859_d_n13, assign34700_e57859_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34700_e57842: f64 = (4.0 - p.p1908);
        let assign34700_e57843: f64 = (locals.var_t5).powf(assign34700_e57842);
        let assign34700_e57847: f64 = (4.0 - p.p1908);
        let assign34700_e57848: f64 = (locals.var_t5).powf(assign34700_e57847);
        let assign34700_e57853: f64 = (4.0 - p.p1908);
        let assign34700_e57854: f64 = (locals.var_vsat_rs).powf(assign34700_e57853);
        let assign34700_e57855: f64 = (p.p1915 * assign34700_e57854);
        let assign34700_e57856: f64 = (assign34700_e57848 + assign34700_e57855);
        let assign34700_e57857: f64 = (assign34700_e57843 / assign34700_e57856);
        (assign34700_e57857, (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn0)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn0 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn0)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn0 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn0)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn0 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn2)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn2 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn2)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn2 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn2)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn2 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn3)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn3 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn3)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn3 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn3)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn3 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn4)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn4 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn4)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn4 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn4)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn4 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn5)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn5 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn5)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn5 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn5)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn5 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn6)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn6 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn6)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn6 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn6)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn6 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn7)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn7 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn7)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn7 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn7)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn7 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn8)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn8 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn8)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn8 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn8)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn8 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn9)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn9 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn9)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn9 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn9)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn9 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn10)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn10 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn10)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn10 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn10)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn10 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn11)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn11 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn11)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn11 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn11)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn11 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn13)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn13 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn13)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn13 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn13)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn13 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn14)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn14 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn14)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn14 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn14)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn14 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)),)
    } else {
        (locals.var_delta_vsrs, locals.var_delta_vsrs_dn0, locals.var_delta_vsrs_dn2, locals.var_delta_vsrs_dn3, locals.var_delta_vsrs_dn4, locals.var_delta_vsrs_dn5, locals.var_delta_vsrs_dn6, locals.var_delta_vsrs_dn7, locals.var_delta_vsrs_dn8, locals.var_delta_vsrs_dn9, locals.var_delta_vsrs_dn10, locals.var_delta_vsrs_dn11, locals.var_delta_vsrs_dn13, locals.var_delta_vsrs_dn14,)
    }
};
        locals.var_delta_vsrs = assign34700_e57859;
        locals.var_delta_vsrs_dn0 = assign34700_e57859_d_n0;
        locals.var_delta_vsrs_dn2 = assign34700_e57859_d_n2;
        locals.var_delta_vsrs_dn3 = assign34700_e57859_d_n3;
        locals.var_delta_vsrs_dn4 = assign34700_e57859_d_n4;
        locals.var_delta_vsrs_dn5 = assign34700_e57859_d_n5;
        locals.var_delta_vsrs_dn6 = assign34700_e57859_d_n6;
        locals.var_delta_vsrs_dn7 = assign34700_e57859_d_n7;
        locals.var_delta_vsrs_dn8 = assign34700_e57859_d_n8;
        locals.var_delta_vsrs_dn9 = assign34700_e57859_d_n9;
        locals.var_delta_vsrs_dn10 = assign34700_e57859_d_n10;
        locals.var_delta_vsrs_dn11 = assign34700_e57859_d_n11;
        locals.var_delta_vsrs_dn13 = assign34700_e57859_d_n13;
        locals.var_delta_vsrs_dn14 = assign34700_e57859_d_n14;

        let (assign34710_e57873, assign34710_e57873_d_n0, assign34710_e57873_d_n2, assign34710_e57873_d_n3, assign34710_e57873_d_n4, assign34710_e57873_d_n5, assign34710_e57873_d_n6, assign34710_e57873_d_n7, assign34710_e57873_d_n8, assign34710_e57873_d_n9, assign34710_e57873_d_n10, assign34710_e57873_d_n11, assign34710_e57873_d_n13, assign34710_e57873_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34710_e57866: f64 = (1.0 / p.p1908);
        let assign34710_e57867: f64 = (locals.var_delta_vsrs).powf(assign34710_e57866);
        let assign34710_e57869: f64 = (assign34710_e57867 * locals.var_t5);
        let assign34710_e57871: f64 = (assign34710_e57869 / locals.var_vsat_rs);
        (assign34710_e57871, (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn0)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn0 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn0)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn0)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn2)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn2 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn2)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn2)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn3)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn3 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn3)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn3)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn4)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn4 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn4)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn4)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn5)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn5 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn5)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn5)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn6)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn6 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn6)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn6)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn7)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn7 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn7)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn7)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn8)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn8 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn8)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn8)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn9)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn9 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn9)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn9)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn10)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn10 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn10)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn10)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn11)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn11 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn11)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn11)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn13)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn13 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn13)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn13)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn14)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn14 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn14)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn14)) / (locals.var_vsat_rs * locals.var_vsat_rs)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34710_e57873;
        locals.var_t6_dn0 = assign34710_e57873_d_n0;
        locals.var_t6_dn2 = assign34710_e57873_d_n2;
        locals.var_t6_dn3 = assign34710_e57873_d_n3;
        locals.var_t6_dn4 = assign34710_e57873_d_n4;
        locals.var_t6_dn5 = assign34710_e57873_d_n5;
        locals.var_t6_dn6 = assign34710_e57873_d_n6;
        locals.var_t6_dn7 = assign34710_e57873_d_n7;
        locals.var_t6_dn8 = assign34710_e57873_d_n8;
        locals.var_t6_dn9 = assign34710_e57873_d_n9;
        locals.var_t6_dn10 = assign34710_e57873_d_n10;
        locals.var_t6_dn11 = assign34710_e57873_d_n11;
        locals.var_t6_dn13 = assign34710_e57873_d_n13;
        locals.var_t6_dn14 = assign34710_e57873_d_n14;

        let (assign34720_e57889, assign34720_e57889_d_n0, assign34720_e57889_d_n2, assign34720_e57889_d_n3, assign34720_e57889_d_n4, assign34720_e57889_d_n5, assign34720_e57889_d_n6, assign34720_e57889_d_n7, assign34720_e57889_d_n8, assign34720_e57889_d_n9, assign34720_e57889_d_n10, assign34720_e57889_d_n11, assign34720_e57889_d_n13, assign34720_e57889_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34720_e57881: f64 = (locals.var_t6).powf(p.p1908);
        let assign34720_e57882: f64 = (1.0 + assign34720_e57881);
        let assign34720_e57885: f64 = (1.0 / p.p1908);
        let assign34720_e57886: f64 = (assign34720_e57882).powf(assign34720_e57885);
        let assign34720_e57887: f64 = (locals.var_t4 * assign34720_e57886);
        (assign34720_e57887, ((locals.var_t4_dn0 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn0)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn0 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn0)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn0 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn2 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn2)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn2 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn2)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn2 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn3 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn3)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn3 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn3)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn3 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn4 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn4)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn4 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn4)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn4 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn5 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn5)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn5 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn5)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn5 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn6 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn6)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn6 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn6)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn6 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn7 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn7)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn7 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn7)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn7 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn8 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn8)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn8 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn8)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn8 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn9 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn9)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn9 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn9)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn9 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn10 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn10)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn10 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn10)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn10 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn11 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn11)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn11 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn11)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn11 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn13 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn13)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn13 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn13)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn13 / locals.var_t6))) } / assign34720_e57882))) })), ((locals.var_t4_dn14 * assign34720_e57886) + (locals.var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn14)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn14 / locals.var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((locals.var_t6).powf(p.p1908 - 1.0) * locals.var_t6_dn14)) } } else { (assign34720_e57881 * (p.p1908 * (locals.var_t6_dn14 / locals.var_t6))) } / assign34720_e57882))) })),)
    } else {
        (locals.var_rvs_s, locals.var_rvs_s_dn0, locals.var_rvs_s_dn2, locals.var_rvs_s_dn3, locals.var_rvs_s_dn4, locals.var_rvs_s_dn5, locals.var_rvs_s_dn6, locals.var_rvs_s_dn7, locals.var_rvs_s_dn8, locals.var_rvs_s_dn9, locals.var_rvs_s_dn10, locals.var_rvs_s_dn11, locals.var_rvs_s_dn13, locals.var_rvs_s_dn14,)
    }
};
        locals.var_rvs_s = assign34720_e57889;
        locals.var_rvs_s_dn0 = assign34720_e57889_d_n0;
        locals.var_rvs_s_dn2 = assign34720_e57889_d_n2;
        locals.var_rvs_s_dn3 = assign34720_e57889_d_n3;
        locals.var_rvs_s_dn4 = assign34720_e57889_d_n4;
        locals.var_rvs_s_dn5 = assign34720_e57889_d_n5;
        locals.var_rvs_s_dn6 = assign34720_e57889_d_n6;
        locals.var_rvs_s_dn7 = assign34720_e57889_d_n7;
        locals.var_rvs_s_dn8 = assign34720_e57889_d_n8;
        locals.var_rvs_s_dn9 = assign34720_e57889_d_n9;
        locals.var_rvs_s_dn10 = assign34720_e57889_d_n10;
        locals.var_rvs_s_dn11 = assign34720_e57889_d_n11;
        locals.var_rvs_s_dn13 = assign34720_e57889_d_n13;
        locals.var_rvs_s_dn14 = assign34720_e57889_d_n14;

        let assign34730_e57896: f64 = if ((p.p64 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard664 = assign34730_e57896;

        let (assign34740_e57902, assign34740_e57902_d_n0, assign34740_e57902_d_n2, assign34740_e57902_d_n3, assign34740_e57902_d_n4, assign34740_e57902_d_n5, assign34740_e57902_d_n6, assign34740_e57902_d_n7, assign34740_e57902_d_n8, assign34740_e57902_d_n9, assign34740_e57902_d_n10, assign34740_e57902_d_n11, assign34740_e57902_d_n13, assign34740_e57902_d_n14,) = {
    if (locals.var_guard664 != 0.0) {
        let assign34740_e57900: f64 = (1.0 / locals.var_rdrain);
        (assign34740_e57900, (-(locals.var_rdrain_dn0 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn2 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn3 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn4 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn5 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn6 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn7 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn8 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn9 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn10 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn11 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn13 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn14 / (locals.var_rdrain * locals.var_rdrain))),)
    } else {
        (locals.var_gdpr, locals.var_gdpr_dn0, locals.var_gdpr_dn2, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11, locals.var_gdpr_dn13, locals.var_gdpr_dn14,)
    }
};
        locals.var_gdpr = assign34740_e57902;
        locals.var_gdpr_dn0 = assign34740_e57902_d_n0;
        locals.var_gdpr_dn2 = assign34740_e57902_d_n2;
        locals.var_gdpr_dn3 = assign34740_e57902_d_n3;
        locals.var_gdpr_dn4 = assign34740_e57902_d_n4;
        locals.var_gdpr_dn5 = assign34740_e57902_d_n5;
        locals.var_gdpr_dn6 = assign34740_e57902_d_n6;
        locals.var_gdpr_dn7 = assign34740_e57902_d_n7;
        locals.var_gdpr_dn8 = assign34740_e57902_d_n8;
        locals.var_gdpr_dn9 = assign34740_e57902_d_n9;
        locals.var_gdpr_dn10 = assign34740_e57902_d_n10;
        locals.var_gdpr_dn11 = assign34740_e57902_d_n11;
        locals.var_gdpr_dn13 = assign34740_e57902_d_n13;
        locals.var_gdpr_dn14 = assign34740_e57902_d_n14;

        let assign34750_e57909: f64 = if ((p.p64 == 1.0) && (p.p1910 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard665 = assign34750_e57909;

        let (assign34760_e57917, assign34760_e57917_d_n0, assign34760_e57917_d_n2, assign34760_e57917_d_n3, assign34760_e57917_d_n4, assign34760_e57917_d_n5, assign34760_e57917_d_n6, assign34760_e57917_d_n7, assign34760_e57917_d_n8, assign34760_e57917_d_n9, assign34760_e57917_d_n10, assign34760_e57917_d_n11, assign34760_e57917_d_n13, assign34760_e57917_d_n14,) = {
    if ((locals.var_guard664 != 0.0) && (locals.var_guard665 != 0.0)) {
        let assign34760_e57915: f64 = (1.0 / locals.var_rvs_d);
        (assign34760_e57915, (-(locals.var_rvs_d_dn0 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn2 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn3 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn4 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn5 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn6 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn7 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn8 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn9 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn10 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn11 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn13 / (locals.var_rvs_d * locals.var_rvs_d))), (-(locals.var_rvs_d_dn14 / (locals.var_rvs_d * locals.var_rvs_d))),)
    } else {
        (locals.var_gvs_d, locals.var_gvs_d_dn0, locals.var_gvs_d_dn2, locals.var_gvs_d_dn3, locals.var_gvs_d_dn4, locals.var_gvs_d_dn5, locals.var_gvs_d_dn6, locals.var_gvs_d_dn7, locals.var_gvs_d_dn8, locals.var_gvs_d_dn9, locals.var_gvs_d_dn10, locals.var_gvs_d_dn11, locals.var_gvs_d_dn13, locals.var_gvs_d_dn14,)
    }
};
        locals.var_gvs_d = assign34760_e57917;
        locals.var_gvs_d_dn0 = assign34760_e57917_d_n0;
        locals.var_gvs_d_dn2 = assign34760_e57917_d_n2;
        locals.var_gvs_d_dn3 = assign34760_e57917_d_n3;
        locals.var_gvs_d_dn4 = assign34760_e57917_d_n4;
        locals.var_gvs_d_dn5 = assign34760_e57917_d_n5;
        locals.var_gvs_d_dn6 = assign34760_e57917_d_n6;
        locals.var_gvs_d_dn7 = assign34760_e57917_d_n7;
        locals.var_gvs_d_dn8 = assign34760_e57917_d_n8;
        locals.var_gvs_d_dn9 = assign34760_e57917_d_n9;
        locals.var_gvs_d_dn10 = assign34760_e57917_d_n10;
        locals.var_gvs_d_dn11 = assign34760_e57917_d_n11;
        locals.var_gvs_d_dn13 = assign34760_e57917_d_n13;
        locals.var_gvs_d_dn14 = assign34760_e57917_d_n14;

        let assign34770_e57924: f64 = if ((p.p64 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard666 = assign34770_e57924;

        let (assign34780_e57930, assign34780_e57930_d_n0, assign34780_e57930_d_n2, assign34780_e57930_d_n3, assign34780_e57930_d_n4, assign34780_e57930_d_n5, assign34780_e57930_d_n6, assign34780_e57930_d_n7, assign34780_e57930_d_n8, assign34780_e57930_d_n9, assign34780_e57930_d_n10, assign34780_e57930_d_n11, assign34780_e57930_d_n13, assign34780_e57930_d_n14,) = {
    if (locals.var_guard666 != 0.0) {
        let assign34780_e57928: f64 = (1.0 / locals.var_rsource);
        (assign34780_e57928, (-(locals.var_rsource_dn0 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn2 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn3 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn4 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn5 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn6 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn7 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn8 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn9 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn10 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn11 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn13 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn14 / (locals.var_rsource * locals.var_rsource))),)
    } else {
        (locals.var_gspr, locals.var_gspr_dn0, locals.var_gspr_dn2, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11, locals.var_gspr_dn13, locals.var_gspr_dn14,)
    }
};
        locals.var_gspr = assign34780_e57930;
        locals.var_gspr_dn0 = assign34780_e57930_d_n0;
        locals.var_gspr_dn2 = assign34780_e57930_d_n2;
        locals.var_gspr_dn3 = assign34780_e57930_d_n3;
        locals.var_gspr_dn4 = assign34780_e57930_d_n4;
        locals.var_gspr_dn5 = assign34780_e57930_d_n5;
        locals.var_gspr_dn6 = assign34780_e57930_d_n6;
        locals.var_gspr_dn7 = assign34780_e57930_d_n7;
        locals.var_gspr_dn8 = assign34780_e57930_d_n8;
        locals.var_gspr_dn9 = assign34780_e57930_d_n9;
        locals.var_gspr_dn10 = assign34780_e57930_d_n10;
        locals.var_gspr_dn11 = assign34780_e57930_d_n11;
        locals.var_gspr_dn13 = assign34780_e57930_d_n13;
        locals.var_gspr_dn14 = assign34780_e57930_d_n14;

        let assign34790_e57937: f64 = if ((p.p64 == 1.0) && (p.p1911 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard667 = assign34790_e57937;

        let (assign34800_e57945, assign34800_e57945_d_n0, assign34800_e57945_d_n2, assign34800_e57945_d_n3, assign34800_e57945_d_n4, assign34800_e57945_d_n5, assign34800_e57945_d_n6, assign34800_e57945_d_n7, assign34800_e57945_d_n8, assign34800_e57945_d_n9, assign34800_e57945_d_n10, assign34800_e57945_d_n11, assign34800_e57945_d_n13, assign34800_e57945_d_n14,) = {
    if ((locals.var_guard666 != 0.0) && (locals.var_guard667 != 0.0)) {
        let assign34800_e57943: f64 = (1.0 / locals.var_rvs_s);
        (assign34800_e57943, (-(locals.var_rvs_s_dn0 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn2 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn3 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn4 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn5 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn6 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn7 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn8 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn9 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn10 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn11 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn13 / (locals.var_rvs_s * locals.var_rvs_s))), (-(locals.var_rvs_s_dn14 / (locals.var_rvs_s * locals.var_rvs_s))),)
    } else {
        (locals.var_gvs_s, locals.var_gvs_s_dn0, locals.var_gvs_s_dn2, locals.var_gvs_s_dn3, locals.var_gvs_s_dn4, locals.var_gvs_s_dn5, locals.var_gvs_s_dn6, locals.var_gvs_s_dn7, locals.var_gvs_s_dn8, locals.var_gvs_s_dn9, locals.var_gvs_s_dn10, locals.var_gvs_s_dn11, locals.var_gvs_s_dn13, locals.var_gvs_s_dn14,)
    }
};
        locals.var_gvs_s = assign34800_e57945;
        locals.var_gvs_s_dn0 = assign34800_e57945_d_n0;
        locals.var_gvs_s_dn2 = assign34800_e57945_d_n2;
        locals.var_gvs_s_dn3 = assign34800_e57945_d_n3;
        locals.var_gvs_s_dn4 = assign34800_e57945_d_n4;
        locals.var_gvs_s_dn5 = assign34800_e57945_d_n5;
        locals.var_gvs_s_dn6 = assign34800_e57945_d_n6;
        locals.var_gvs_s_dn7 = assign34800_e57945_d_n7;
        locals.var_gvs_s_dn8 = assign34800_e57945_d_n8;
        locals.var_gvs_s_dn9 = assign34800_e57945_d_n9;
        locals.var_gvs_s_dn10 = assign34800_e57945_d_n10;
        locals.var_gvs_s_dn11 = assign34800_e57945_d_n11;
        locals.var_gvs_s_dn13 = assign34800_e57945_d_n13;
        locals.var_gvs_s_dn14 = assign34800_e57945_d_n14;

        let assign34810_e57952: f64 = if ((p.p73 == 1.0) && (locals.var_xrcrg1_i != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard668 = assign34810_e57952;

        let assign34820_e57955: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign34820_e57955;

        let assign34900_e57995: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign34900_e57995;

        let assign34950_e58014: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard682 = assign34950_e58014;

        let assign34960_e58021: f64 = if ((p.p64 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard683 = assign34960_e58021;

        let assign34970_e58028: f64 = if ((p.p64 == 1.0) && (p.p1910 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard684 = assign34970_e58028;

        let assign34980_e58035: f64 = if ((p.p64 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard685 = assign34980_e58035;

        let assign34990_e58042: f64 = if ((p.p64 == 1.0) && (p.p1911 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard686 = assign34990_e58042;

        let assign35580_e58822: f64 = (10.0 * locals.var_vtm);
        let assign35580_e58824: f64 = (assign35580_e58822 / locals.var_rc);
        let assign35580_e58827: f64 = (2.0 * locals.var_qbs);
        let assign35580_e58828: f64 = (assign35580_e58824 + assign35580_e58827);
        locals.var_q0 = assign35580_e58828;
        locals.var_q0_dn0 = 0.0;
        locals.var_q0_dn2 = 0.0;
        locals.var_q0_dn3 = 0.0;
        locals.var_q0_dn4 = ((10.0 * locals.var_vtm_dn4) / locals.var_rc);
        locals.var_q0_dn5 = 0.0;
        locals.var_q0_dn6 = 0.0;
        locals.var_q0_dn7 = 0.0;
        locals.var_q0_dn8 = 0.0;
        locals.var_q0_dn9 = 0.0;
        locals.var_q0_dn10 = 0.0;
        locals.var_q0_dn11 = 0.0;
        locals.var_q0_dn13 = 0.0;
        locals.var_q0_dn14 = 0.0;

        let assign35590_e58832: f64 = (locals.var_vtm + locals.var_q0);
        let assign35590_e58833: f64 = (locals.var_vtm * assign35590_e58832);
        locals.var_t1 = assign35590_e58833;
        locals.var_t1_dn0 = (locals.var_vtm * locals.var_q0_dn0);
        locals.var_t1_dn2 = (locals.var_vtm * locals.var_q0_dn2);
        locals.var_t1_dn3 = (locals.var_vtm * locals.var_q0_dn3);
        locals.var_t1_dn4 = ((locals.var_vtm_dn4 * assign35590_e58832) + (locals.var_vtm * (locals.var_vtm_dn4 + locals.var_q0_dn4)));
        locals.var_t1_dn5 = (locals.var_vtm * locals.var_q0_dn5);
        locals.var_t1_dn6 = (locals.var_vtm * locals.var_q0_dn6);
        locals.var_t1_dn7 = (locals.var_vtm * locals.var_q0_dn7);
        locals.var_t1_dn8 = (locals.var_vtm * locals.var_q0_dn8);
        locals.var_t1_dn9 = (locals.var_vtm * locals.var_q0_dn9);
        locals.var_t1_dn10 = (locals.var_vtm * locals.var_q0_dn10);
        locals.var_t1_dn11 = (locals.var_vtm * locals.var_q0_dn11);
        locals.var_t1_dn13 = (locals.var_vtm * locals.var_q0_dn13);
        locals.var_t1_dn14 = (locals.var_vtm * locals.var_q0_dn14);

        let assign35600_e58836: f64 = (locals.var_cox * locals.var_cox);
        let assign35600_e58838: f64 = (assign35600_e58836 * locals.var_t1);
        locals.var_t2 = assign35600_e58838;
        locals.var_t2_dn0 = (assign35600_e58836 * locals.var_t1_dn0);
        locals.var_t2_dn2 = (assign35600_e58836 * locals.var_t1_dn2);
        locals.var_t2_dn3 = (assign35600_e58836 * locals.var_t1_dn3);
        locals.var_t2_dn4 = (assign35600_e58836 * locals.var_t1_dn4);
        locals.var_t2_dn5 = (assign35600_e58836 * locals.var_t1_dn5);
        locals.var_t2_dn6 = (assign35600_e58836 * locals.var_t1_dn6);
        locals.var_t2_dn7 = (assign35600_e58836 * locals.var_t1_dn7);
        locals.var_t2_dn8 = (assign35600_e58836 * locals.var_t1_dn8);
        locals.var_t2_dn9 = (assign35600_e58836 * locals.var_t1_dn9);
        locals.var_t2_dn10 = (assign35600_e58836 * locals.var_t1_dn10);
        locals.var_t2_dn11 = (assign35600_e58836 * locals.var_t1_dn11);
        locals.var_t2_dn13 = (assign35600_e58836 * locals.var_t1_dn13);
        locals.var_t2_dn14 = (assign35600_e58836 * locals.var_t1_dn14);

        let assign35610_e58841: f64 = (2.0 * 1.60219e-19);
        let assign35610_e58843: f64 = (assign35610_e58841 * locals.var_ni);
        let assign35610_e58845: f64 = (assign35610_e58843 * locals.var_epssub);
        let assign35610_e58847: f64 = (assign35610_e58845 * locals.var_vtm);
        locals.var_t3 = assign35610_e58847;
        locals.var_t3_dn0 = (((assign35610_e58841 * locals.var_ni_dn0) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn2 = (((assign35610_e58841 * locals.var_ni_dn2) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn3 = (((assign35610_e58841 * locals.var_ni_dn3) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn4 = ((((assign35610_e58841 * locals.var_ni_dn4) * locals.var_epssub) * locals.var_vtm) + (assign35610_e58845 * locals.var_vtm_dn4));
        locals.var_t3_dn5 = (((assign35610_e58841 * locals.var_ni_dn5) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn6 = (((assign35610_e58841 * locals.var_ni_dn6) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn7 = (((assign35610_e58841 * locals.var_ni_dn7) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn8 = (((assign35610_e58841 * locals.var_ni_dn8) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn9 = (((assign35610_e58841 * locals.var_ni_dn9) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn10 = (((assign35610_e58841 * locals.var_ni_dn10) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn11 = (((assign35610_e58841 * locals.var_ni_dn11) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn13 = (((assign35610_e58841 * locals.var_ni_dn13) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn14 = (((assign35610_e58841 * locals.var_ni_dn14) * locals.var_epssub) * locals.var_vtm);

    }

    pub(super) fn stamp_reactive_block_0(
        locals: &mut StampLocals,
    ) {
        locals.var_dr = 0.0;
        locals.var_dr_dn0 = 0.0;
        locals.var_dr_dn2 = 0.0;
        locals.var_dr_dn3 = 0.0;
        locals.var_dr_dn4 = 0.0;
        locals.var_dr_dn5 = 0.0;
        locals.var_dr_dn6 = 0.0;
        locals.var_dr_dn7 = 0.0;
        locals.var_dr_dn8 = 0.0;
        locals.var_dr_dn9 = 0.0;
        locals.var_dr_dn10 = 0.0;
        locals.var_dr_dn11 = 0.0;
        locals.var_dr_dn13 = 0.0;
        locals.var_dr_dn14 = 0.0;
        locals.var_dr_rv = 0.0;

        locals.var_rdsi = 0.0;
        locals.var_rdsi_dn0 = 0.0;
        locals.var_rdsi_dn2 = 0.0;
        locals.var_rdsi_dn3 = 0.0;
        locals.var_rdsi_dn4 = 0.0;
        locals.var_rdsi_dn5 = 0.0;
        locals.var_rdsi_dn6 = 0.0;
        locals.var_rdsi_dn7 = 0.0;
        locals.var_rdsi_dn8 = 0.0;
        locals.var_rdsi_dn9 = 0.0;
        locals.var_rdsi_dn10 = 0.0;
        locals.var_rdsi_dn11 = 0.0;
        locals.var_rdsi_dn13 = 0.0;
        locals.var_rdsi_dn14 = 0.0;
        locals.var_rdsi_rv = 0.0;

        locals.var_cjs_t = 0.0;
        locals.var_cjs_t_dn4 = 0.0;
        locals.var_cjs_t_rv = 0.0;

        locals.var_cjsws_t = 0.0;
        locals.var_cjsws_t_dn4 = 0.0;
        locals.var_cjsws_t_rv = 0.0;

        locals.var_cjswgd_t = 0.0;
        locals.var_cjswgd_t_dn4 = 0.0;
        locals.var_cjswgd_t_rv = 0.0;

        locals.var_cjd_t = 0.0;
        locals.var_cjd_t_dn4 = 0.0;
        locals.var_cjd_t_rv = 0.0;

        locals.var_cjswd_t = 0.0;
        locals.var_cjswd_t_dn4 = 0.0;
        locals.var_cjswd_t_rv = 0.0;

        locals.var_cjswgs_t = 0.0;
        locals.var_cjswgs_t_dn4 = 0.0;
        locals.var_cjswgs_t_rv = 0.0;

        locals.var_pbs_t = 0.0;
        locals.var_pbs_t_dn4 = 0.0;
        locals.var_pbs_t_rv = 0.0;

        locals.var_pbsws_t = 0.0;
        locals.var_pbsws_t_dn4 = 0.0;
        locals.var_pbsws_t_rv = 0.0;

        locals.var_pbswgs_t = 0.0;
        locals.var_pbswgs_t_dn4 = 0.0;
        locals.var_pbswgs_t_rv = 0.0;

        locals.var_pbd_t = 0.0;
        locals.var_pbd_t_dn4 = 0.0;
        locals.var_pbd_t_rv = 0.0;

        locals.var_pbswd_t = 0.0;
        locals.var_pbswd_t_dn4 = 0.0;
        locals.var_pbswd_t_rv = 0.0;

        locals.var_pbswgd_t = 0.0;
        locals.var_pbswgd_t_dn4 = 0.0;
        locals.var_pbswgd_t_rv = 0.0;

        locals.var_jss_t = 0.0;
        locals.var_jss_t_dn0 = 0.0;
        locals.var_jss_t_dn2 = 0.0;
        locals.var_jss_t_dn3 = 0.0;
        locals.var_jss_t_dn4 = 0.0;
        locals.var_jss_t_dn5 = 0.0;
        locals.var_jss_t_dn6 = 0.0;
        locals.var_jss_t_dn7 = 0.0;
        locals.var_jss_t_dn8 = 0.0;
        locals.var_jss_t_dn9 = 0.0;
        locals.var_jss_t_dn10 = 0.0;
        locals.var_jss_t_dn11 = 0.0;
        locals.var_jss_t_dn13 = 0.0;
        locals.var_jss_t_dn14 = 0.0;
        locals.var_jss_t_rv = 0.0;

        locals.var_jsws_t = 0.0;
        locals.var_jsws_t_dn0 = 0.0;
        locals.var_jsws_t_dn2 = 0.0;
        locals.var_jsws_t_dn3 = 0.0;
        locals.var_jsws_t_dn4 = 0.0;
        locals.var_jsws_t_dn5 = 0.0;
        locals.var_jsws_t_dn6 = 0.0;
        locals.var_jsws_t_dn7 = 0.0;
        locals.var_jsws_t_dn8 = 0.0;
        locals.var_jsws_t_dn9 = 0.0;
        locals.var_jsws_t_dn10 = 0.0;
        locals.var_jsws_t_dn11 = 0.0;
        locals.var_jsws_t_dn13 = 0.0;
        locals.var_jsws_t_dn14 = 0.0;
        locals.var_jsws_t_rv = 0.0;

        locals.var_jswgs_t = 0.0;
        locals.var_jswgs_t_dn0 = 0.0;
        locals.var_jswgs_t_dn2 = 0.0;
        locals.var_jswgs_t_dn3 = 0.0;
        locals.var_jswgs_t_dn4 = 0.0;
        locals.var_jswgs_t_dn5 = 0.0;
        locals.var_jswgs_t_dn6 = 0.0;
        locals.var_jswgs_t_dn7 = 0.0;
        locals.var_jswgs_t_dn8 = 0.0;
        locals.var_jswgs_t_dn9 = 0.0;
        locals.var_jswgs_t_dn10 = 0.0;
        locals.var_jswgs_t_dn11 = 0.0;
        locals.var_jswgs_t_dn13 = 0.0;
        locals.var_jswgs_t_dn14 = 0.0;
        locals.var_jswgs_t_rv = 0.0;

        locals.var_jsd_t = 0.0;
        locals.var_jsd_t_dn0 = 0.0;
        locals.var_jsd_t_dn2 = 0.0;
        locals.var_jsd_t_dn3 = 0.0;
        locals.var_jsd_t_dn4 = 0.0;
        locals.var_jsd_t_dn5 = 0.0;
        locals.var_jsd_t_dn6 = 0.0;
        locals.var_jsd_t_dn7 = 0.0;
        locals.var_jsd_t_dn8 = 0.0;
        locals.var_jsd_t_dn9 = 0.0;
        locals.var_jsd_t_dn10 = 0.0;
        locals.var_jsd_t_dn11 = 0.0;
        locals.var_jsd_t_dn13 = 0.0;
        locals.var_jsd_t_dn14 = 0.0;
        locals.var_jsd_t_rv = 0.0;

        locals.var_jswd_t = 0.0;
        locals.var_jswd_t_dn0 = 0.0;
        locals.var_jswd_t_dn2 = 0.0;
        locals.var_jswd_t_dn3 = 0.0;
        locals.var_jswd_t_dn4 = 0.0;
        locals.var_jswd_t_dn5 = 0.0;
        locals.var_jswd_t_dn6 = 0.0;
        locals.var_jswd_t_dn7 = 0.0;
        locals.var_jswd_t_dn8 = 0.0;
        locals.var_jswd_t_dn9 = 0.0;
        locals.var_jswd_t_dn10 = 0.0;
        locals.var_jswd_t_dn11 = 0.0;
        locals.var_jswd_t_dn13 = 0.0;
        locals.var_jswd_t_dn14 = 0.0;
        locals.var_jswd_t_rv = 0.0;

        locals.var_jswgd_t = 0.0;
        locals.var_jswgd_t_dn0 = 0.0;
        locals.var_jswgd_t_dn2 = 0.0;
        locals.var_jswgd_t_dn3 = 0.0;
        locals.var_jswgd_t_dn4 = 0.0;
        locals.var_jswgd_t_dn5 = 0.0;
        locals.var_jswgd_t_dn6 = 0.0;
        locals.var_jswgd_t_dn7 = 0.0;
        locals.var_jswgd_t_dn8 = 0.0;
        locals.var_jswgd_t_dn9 = 0.0;
        locals.var_jswgd_t_dn10 = 0.0;
        locals.var_jswgd_t_dn11 = 0.0;
        locals.var_jswgd_t_dn13 = 0.0;
        locals.var_jswgd_t_dn14 = 0.0;
        locals.var_jswgd_t_rv = 0.0;

        locals.var_jtss_t = 0.0;
        locals.var_jtss_t_dn4 = 0.0;
        locals.var_jtss_t_rv = 0.0;

        locals.var_jtsd_t = 0.0;
        locals.var_jtsd_t_dn4 = 0.0;
        locals.var_jtsd_t_rv = 0.0;

        locals.var_jtssws_t = 0.0;
        locals.var_jtssws_t_dn4 = 0.0;
        locals.var_jtssws_t_rv = 0.0;

        locals.var_jtsswd_t = 0.0;
        locals.var_jtsswd_t_dn4 = 0.0;
        locals.var_jtsswd_t_rv = 0.0;

        locals.var_jtsswgs_t = 0.0;
        locals.var_jtsswgs_t_dn4 = 0.0;
        locals.var_jtsswgs_t_rv = 0.0;

        locals.var_jtsswgd_t = 0.0;
        locals.var_jtsswgd_t_dn4 = 0.0;
        locals.var_jtsswgd_t_rv = 0.0;

        locals.var_njts_t = 0.0;
        locals.var_njts_t_dn4 = 0.0;
        locals.var_njts_t_rv = 0.0;

        locals.var_njtsd_t = 0.0;
        locals.var_njtsd_t_dn4 = 0.0;
        locals.var_njtsd_t_rv = 0.0;

        locals.var_njtssw_t = 0.0;
        locals.var_njtssw_t_dn4 = 0.0;
        locals.var_njtssw_t_rv = 0.0;

        locals.var_njtsswd_t = 0.0;
        locals.var_njtsswd_t_dn4 = 0.0;
        locals.var_njtsswd_t_rv = 0.0;

        locals.var_njtsswg_t = 0.0;
        locals.var_njtsswg_t_dn4 = 0.0;
        locals.var_njtsswg_t_rv = 0.0;

        locals.var_njtsswgd_t = 0.0;
        locals.var_njtsswgd_t_dn4 = 0.0;
        locals.var_njtsswgd_t_rv = 0.0;

        locals.var_rsdrr_t = 0.0;
        locals.var_rsdrr_t_dn4 = 0.0;
        locals.var_rsdrr_t_rv = 0.0;

        locals.var_rddrr_t = 0.0;
        locals.var_rddrr_t_dn4 = 0.0;
        locals.var_rddrr_t_rv = 0.0;

        locals.var_uar_t = 0.0;
        locals.var_uar_t_dn0 = 0.0;
        locals.var_uar_t_dn2 = 0.0;
        locals.var_uar_t_dn3 = 0.0;
        locals.var_uar_t_dn4 = 0.0;
        locals.var_uar_t_dn5 = 0.0;
        locals.var_uar_t_dn6 = 0.0;
        locals.var_uar_t_dn7 = 0.0;
        locals.var_uar_t_dn8 = 0.0;
        locals.var_uar_t_dn9 = 0.0;
        locals.var_uar_t_dn10 = 0.0;
        locals.var_uar_t_dn11 = 0.0;
        locals.var_uar_t_dn13 = 0.0;
        locals.var_uar_t_dn14 = 0.0;
        locals.var_uar_t_rv = 0.0;

        locals.var_uc_t = 0.0;
        locals.var_uc_t_dn4 = 0.0;
        locals.var_uc_t_rv = 0.0;

        locals.var_uccv_t = 0.0;
        locals.var_uccv_t_dn4 = 0.0;
        locals.var_uccv_t_rv = 0.0;

        locals.var_ucr_t = 0.0;
        locals.var_ucr_t_dn4 = 0.0;
        locals.var_ucr_t_rv = 0.0;

        locals.var_udr_t = 0.0;
        locals.var_udr_t_dn0 = 0.0;
        locals.var_udr_t_dn2 = 0.0;
        locals.var_udr_t_dn3 = 0.0;
        locals.var_udr_t_dn4 = 0.0;
        locals.var_udr_t_dn5 = 0.0;
        locals.var_udr_t_dn6 = 0.0;
        locals.var_udr_t_dn7 = 0.0;
        locals.var_udr_t_dn8 = 0.0;
        locals.var_udr_t_dn9 = 0.0;
        locals.var_udr_t_dn10 = 0.0;
        locals.var_udr_t_dn11 = 0.0;
        locals.var_udr_t_dn13 = 0.0;
        locals.var_udr_t_dn14 = 0.0;
        locals.var_udr_t_rv = 0.0;

        locals.var_vsatr_t = 0.0;
        locals.var_vsatr_t_dn4 = 0.0;
        locals.var_vsatr_t_rv = 0.0;

        locals.var_vsat1r_t = 0.0;
        locals.var_vsat1r_t_dn0 = 0.0;
        locals.var_vsat1r_t_dn2 = 0.0;
        locals.var_vsat1r_t_dn3 = 0.0;
        locals.var_vsat1r_t_dn4 = 0.0;
        locals.var_vsat1r_t_dn5 = 0.0;
        locals.var_vsat1r_t_dn6 = 0.0;
        locals.var_vsat1r_t_dn7 = 0.0;
        locals.var_vsat1r_t_dn8 = 0.0;
        locals.var_vsat1r_t_dn9 = 0.0;
        locals.var_vsat1r_t_dn10 = 0.0;
        locals.var_vsat1r_t_dn11 = 0.0;
        locals.var_vsat1r_t_dn13 = 0.0;
        locals.var_vsat1r_t_dn14 = 0.0;
        locals.var_vsat1r_t_rv = 0.0;

        locals.var_mexpr_t = 0.0;
        locals.var_mexpr_t_dn0 = 0.0;
        locals.var_mexpr_t_dn2 = 0.0;
        locals.var_mexpr_t_dn3 = 0.0;
        locals.var_mexpr_t_dn4 = 0.0;
        locals.var_mexpr_t_dn5 = 0.0;
        locals.var_mexpr_t_dn6 = 0.0;
        locals.var_mexpr_t_dn7 = 0.0;
        locals.var_mexpr_t_dn8 = 0.0;
        locals.var_mexpr_t_dn9 = 0.0;
        locals.var_mexpr_t_dn10 = 0.0;
        locals.var_mexpr_t_dn11 = 0.0;
        locals.var_mexpr_t_dn13 = 0.0;
        locals.var_mexpr_t_dn14 = 0.0;
        locals.var_mexpr_t_rv = 0.0;

        locals.var_ptwgr_t = 0.0;
        locals.var_ptwgr_t_dn0 = 0.0;
        locals.var_ptwgr_t_dn2 = 0.0;
        locals.var_ptwgr_t_dn3 = 0.0;
        locals.var_ptwgr_t_dn4 = 0.0;
        locals.var_ptwgr_t_dn5 = 0.0;
        locals.var_ptwgr_t_dn6 = 0.0;
        locals.var_ptwgr_t_dn7 = 0.0;
        locals.var_ptwgr_t_dn8 = 0.0;
        locals.var_ptwgr_t_dn9 = 0.0;
        locals.var_ptwgr_t_dn10 = 0.0;
        locals.var_ptwgr_t_dn11 = 0.0;
        locals.var_ptwgr_t_dn13 = 0.0;
        locals.var_ptwgr_t_dn14 = 0.0;
        locals.var_ptwgr_t_rv = 0.0;

        locals.var_sprt_i = 0.0;
        locals.var_sprt_i_rv = 0.0;

        locals.var_tcen0 = 0.0;
        locals.var_tcen0_rv = 0.0;

        locals.var_qba = 0.0;
        locals.var_qba_dn0 = 0.0;
        locals.var_qba_dn2 = 0.0;
        locals.var_qba_dn3 = 0.0;
        locals.var_qba_dn4 = 0.0;
        locals.var_qba_dn5 = 0.0;
        locals.var_qba_dn6 = 0.0;
        locals.var_qba_dn7 = 0.0;
        locals.var_qba_dn8 = 0.0;
        locals.var_qba_dn9 = 0.0;
        locals.var_qba_dn10 = 0.0;
        locals.var_qba_dn11 = 0.0;
        locals.var_qba_dn13 = 0.0;
        locals.var_qba_dn14 = 0.0;
        locals.var_qba_rv = 0.0;

        locals.var_u0r_v = 0.0;
        locals.var_u0r_v_dn0 = 0.0;
        locals.var_u0r_v_dn2 = 0.0;
        locals.var_u0r_v_dn3 = 0.0;
        locals.var_u0r_v_dn4 = 0.0;
        locals.var_u0r_v_dn5 = 0.0;
        locals.var_u0r_v_dn6 = 0.0;
        locals.var_u0r_v_dn7 = 0.0;
        locals.var_u0r_v_dn8 = 0.0;
        locals.var_u0r_v_dn9 = 0.0;
        locals.var_u0r_v_dn10 = 0.0;
        locals.var_u0r_v_dn11 = 0.0;
        locals.var_u0r_v_dn13 = 0.0;
        locals.var_u0r_v_dn14 = 0.0;
        locals.var_u0r_v_rv = 0.0;

        locals.var_cfr_geo = 0.0;
        locals.var_cfr_geo_dn0 = 0.0;
        locals.var_cfr_geo_dn2 = 0.0;
        locals.var_cfr_geo_dn3 = 0.0;
        locals.var_cfr_geo_dn4 = 0.0;
        locals.var_cfr_geo_dn5 = 0.0;
        locals.var_cfr_geo_dn6 = 0.0;
        locals.var_cfr_geo_dn7 = 0.0;
        locals.var_cfr_geo_dn8 = 0.0;
        locals.var_cfr_geo_dn9 = 0.0;
        locals.var_cfr_geo_dn10 = 0.0;
        locals.var_cfr_geo_dn11 = 0.0;
        locals.var_cfr_geo_dn13 = 0.0;
        locals.var_cfr_geo_dn14 = 0.0;
        locals.var_cfr_geo_rv = 0.0;

        locals.var_agidlb_i = 0.0;
        locals.var_agidlb_i_rv = 0.0;

        locals.var_bgidlb_i = 0.0;
        locals.var_bgidlb_i_rv = 0.0;

        locals.var_bgidlb_t = 0.0;
        locals.var_bgidlb_t_dn4 = 0.0;
        locals.var_bgidlb_t_rv = 0.0;

        locals.var_cgidlb_i = 0.0;
        locals.var_cgidlb_i_rv = 0.0;

        locals.var_egidlb_i = 0.0;
        locals.var_egidlb_i_rv = 0.0;

        locals.var_pgidlb_i = 0.0;
        locals.var_pgidlb_i_rv = 0.0;

        locals.var_agislb_i = 0.0;
        locals.var_agislb_i_rv = 0.0;

        locals.var_bgislb_i = 0.0;
        locals.var_bgislb_i_rv = 0.0;

        locals.var_bgislb_t = 0.0;
        locals.var_bgislb_t_dn4 = 0.0;
        locals.var_bgislb_t_rv = 0.0;

        locals.var_cgislb_i = 0.0;
        locals.var_cgislb_i_rv = 0.0;

        locals.var_egislb_i = 0.0;
        locals.var_egislb_i_rv = 0.0;

        locals.var_pgislb_i = 0.0;
        locals.var_pgislb_i_rv = 0.0;

        locals.var_cox_acc = 0.0;
        locals.var_cox_acc_dn0 = 0.0;
        locals.var_cox_acc_dn2 = 0.0;
        locals.var_cox_acc_dn3 = 0.0;
        locals.var_cox_acc_dn4 = 0.0;
        locals.var_cox_acc_dn5 = 0.0;
        locals.var_cox_acc_dn6 = 0.0;
        locals.var_cox_acc_dn7 = 0.0;
        locals.var_cox_acc_dn8 = 0.0;
        locals.var_cox_acc_dn9 = 0.0;
        locals.var_cox_acc_dn10 = 0.0;
        locals.var_cox_acc_dn11 = 0.0;
        locals.var_cox_acc_dn13 = 0.0;
        locals.var_cox_acc_dn14 = 0.0;
        locals.var_cox_acc_rv = 0.0;

        locals.var_qg_acc = 0.0;
        locals.var_qg_acc_dn0 = 0.0;
        locals.var_qg_acc_dn2 = 0.0;
        locals.var_qg_acc_dn3 = 0.0;
        locals.var_qg_acc_dn4 = 0.0;
        locals.var_qg_acc_dn5 = 0.0;
        locals.var_qg_acc_dn6 = 0.0;
        locals.var_qg_acc_dn7 = 0.0;
        locals.var_qg_acc_dn8 = 0.0;
        locals.var_qg_acc_dn9 = 0.0;
        locals.var_qg_acc_dn10 = 0.0;
        locals.var_qg_acc_dn11 = 0.0;
        locals.var_qg_acc_dn13 = 0.0;
        locals.var_qg_acc_dn14 = 0.0;
        locals.var_qg_acc_rv = 0.0;

        locals.var_qb_acc = 0.0;
        locals.var_qb_acc_dn0 = 0.0;
        locals.var_qb_acc_dn2 = 0.0;
        locals.var_qb_acc_dn3 = 0.0;
        locals.var_qb_acc_dn4 = 0.0;
        locals.var_qb_acc_dn5 = 0.0;
        locals.var_qb_acc_dn6 = 0.0;
        locals.var_qb_acc_dn7 = 0.0;
        locals.var_qb_acc_dn8 = 0.0;
        locals.var_qb_acc_dn9 = 0.0;
        locals.var_qb_acc_dn10 = 0.0;
        locals.var_qb_acc_dn11 = 0.0;
        locals.var_qb_acc_dn13 = 0.0;
        locals.var_qb_acc_dn14 = 0.0;
        locals.var_qb_acc_rv = 0.0;

        locals.var_qgs_ov = 0.0;
        locals.var_qgs_ov_dn0 = 0.0;
        locals.var_qgs_ov_dn2 = 0.0;
        locals.var_qgs_ov_dn3 = 0.0;
        locals.var_qgs_ov_dn4 = 0.0;
        locals.var_qgs_ov_dn5 = 0.0;
        locals.var_qgs_ov_dn6 = 0.0;
        locals.var_qgs_ov_dn7 = 0.0;
        locals.var_qgs_ov_dn8 = 0.0;
        locals.var_qgs_ov_dn9 = 0.0;
        locals.var_qgs_ov_dn10 = 0.0;
        locals.var_qgs_ov_dn11 = 0.0;
        locals.var_qgs_ov_dn13 = 0.0;
        locals.var_qgs_ov_dn14 = 0.0;
        locals.var_qgs_ov_rv = 0.0;

        locals.var_qgd_ov = 0.0;
        locals.var_qgd_ov_dn0 = 0.0;
        locals.var_qgd_ov_dn2 = 0.0;
        locals.var_qgd_ov_dn3 = 0.0;
        locals.var_qgd_ov_dn4 = 0.0;
        locals.var_qgd_ov_dn5 = 0.0;
        locals.var_qgd_ov_dn6 = 0.0;
        locals.var_qgd_ov_dn7 = 0.0;
        locals.var_qgd_ov_dn8 = 0.0;
        locals.var_qgd_ov_dn9 = 0.0;
        locals.var_qgd_ov_dn10 = 0.0;
        locals.var_qgd_ov_dn11 = 0.0;
        locals.var_qgd_ov_dn13 = 0.0;
        locals.var_qgd_ov_dn14 = 0.0;
        locals.var_qgd_ov_rv = 0.0;

        locals.var_czbs = 0.0;
        locals.var_czbs_dn4 = 0.0;
        locals.var_czbs_rv = 0.0;

        locals.var_czbssw = 0.0;
        locals.var_czbssw_dn4 = 0.0;
        locals.var_czbssw_rv = 0.0;

        locals.var_czbsswg = 0.0;
        locals.var_czbsswg_dn4 = 0.0;
        locals.var_czbsswg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        locals: &mut StampLocals,
    ) {
        locals.var_czbd = 0.0;
        locals.var_czbd_dn4 = 0.0;
        locals.var_czbd_rv = 0.0;

        locals.var_czbdsw = 0.0;
        locals.var_czbdsw_dn4 = 0.0;
        locals.var_czbdsw_rv = 0.0;

        locals.var_czbdswg = 0.0;
        locals.var_czbdswg_dn4 = 0.0;
        locals.var_czbdswg_rv = 0.0;

        locals.var_qesj = 0.0;
        locals.var_qesj_dn3 = 0.0;
        locals.var_qesj_dn4 = 0.0;
        locals.var_qesj_dn6 = 0.0;
        locals.var_qesj_rv = 0.0;

        locals.var_qedj = 0.0;
        locals.var_qedj_dn3 = 0.0;
        locals.var_qedj_dn4 = 0.0;
        locals.var_qedj_dn5 = 0.0;
        locals.var_qedj_rv = 0.0;

        locals.var_isbs = 0.0;
        locals.var_isbs_dn0 = 0.0;
        locals.var_isbs_dn2 = 0.0;
        locals.var_isbs_dn3 = 0.0;
        locals.var_isbs_dn4 = 0.0;
        locals.var_isbs_dn5 = 0.0;
        locals.var_isbs_dn6 = 0.0;
        locals.var_isbs_dn7 = 0.0;
        locals.var_isbs_dn8 = 0.0;
        locals.var_isbs_dn9 = 0.0;
        locals.var_isbs_dn10 = 0.0;
        locals.var_isbs_dn11 = 0.0;
        locals.var_isbs_dn13 = 0.0;
        locals.var_isbs_dn14 = 0.0;
        locals.var_isbs_rv = 0.0;

        locals.var_isbd = 0.0;
        locals.var_isbd_dn0 = 0.0;
        locals.var_isbd_dn2 = 0.0;
        locals.var_isbd_dn3 = 0.0;
        locals.var_isbd_dn4 = 0.0;
        locals.var_isbd_dn5 = 0.0;
        locals.var_isbd_dn6 = 0.0;
        locals.var_isbd_dn7 = 0.0;
        locals.var_isbd_dn8 = 0.0;
        locals.var_isbd_dn9 = 0.0;
        locals.var_isbd_dn10 = 0.0;
        locals.var_isbd_dn11 = 0.0;
        locals.var_isbd_dn13 = 0.0;
        locals.var_isbd_dn14 = 0.0;
        locals.var_isbd_rv = 0.0;

        locals.var_nvtms = 0.0;
        locals.var_nvtms_dn4 = 0.0;
        locals.var_nvtms_rv = 0.0;

        locals.var_nvtmd = 0.0;
        locals.var_nvtmd_dn4 = 0.0;
        locals.var_nvtmd_rv = 0.0;

        locals.var_vjsmfwd = 0.0;
        locals.var_vjsmfwd_dn0 = 0.0;
        locals.var_vjsmfwd_dn2 = 0.0;
        locals.var_vjsmfwd_dn3 = 0.0;
        locals.var_vjsmfwd_dn4 = 0.0;
        locals.var_vjsmfwd_dn5 = 0.0;
        locals.var_vjsmfwd_dn6 = 0.0;
        locals.var_vjsmfwd_dn7 = 0.0;
        locals.var_vjsmfwd_dn8 = 0.0;
        locals.var_vjsmfwd_dn9 = 0.0;
        locals.var_vjsmfwd_dn10 = 0.0;
        locals.var_vjsmfwd_dn11 = 0.0;
        locals.var_vjsmfwd_dn13 = 0.0;
        locals.var_vjsmfwd_dn14 = 0.0;
        locals.var_vjsmfwd_rv = 0.0;

        locals.var_xexpbvs = 0.0;
        locals.var_xexpbvs_dn4 = 0.0;
        locals.var_xexpbvs_rv = 0.0;

        locals.var_sslprev = 0.0;
        locals.var_sslprev_dn0 = 0.0;
        locals.var_sslprev_dn2 = 0.0;
        locals.var_sslprev_dn3 = 0.0;
        locals.var_sslprev_dn4 = 0.0;
        locals.var_sslprev_dn5 = 0.0;
        locals.var_sslprev_dn6 = 0.0;
        locals.var_sslprev_dn7 = 0.0;
        locals.var_sslprev_dn8 = 0.0;
        locals.var_sslprev_dn9 = 0.0;
        locals.var_sslprev_dn10 = 0.0;
        locals.var_sslprev_dn11 = 0.0;
        locals.var_sslprev_dn13 = 0.0;
        locals.var_sslprev_dn14 = 0.0;
        locals.var_sslprev_rv = 0.0;

        locals.var_ivjsmrev = 0.0;
        locals.var_ivjsmrev_dn0 = 0.0;
        locals.var_ivjsmrev_dn2 = 0.0;
        locals.var_ivjsmrev_dn3 = 0.0;
        locals.var_ivjsmrev_dn4 = 0.0;
        locals.var_ivjsmrev_dn5 = 0.0;
        locals.var_ivjsmrev_dn6 = 0.0;
        locals.var_ivjsmrev_dn7 = 0.0;
        locals.var_ivjsmrev_dn8 = 0.0;
        locals.var_ivjsmrev_dn9 = 0.0;
        locals.var_ivjsmrev_dn10 = 0.0;
        locals.var_ivjsmrev_dn11 = 0.0;
        locals.var_ivjsmrev_dn13 = 0.0;
        locals.var_ivjsmrev_dn14 = 0.0;
        locals.var_ivjsmrev_rv = 0.0;

        locals.var_vjsmrev = 0.0;
        locals.var_vjsmrev_dn0 = 0.0;
        locals.var_vjsmrev_dn2 = 0.0;
        locals.var_vjsmrev_dn3 = 0.0;
        locals.var_vjsmrev_dn4 = 0.0;
        locals.var_vjsmrev_dn5 = 0.0;
        locals.var_vjsmrev_dn6 = 0.0;
        locals.var_vjsmrev_dn7 = 0.0;
        locals.var_vjsmrev_dn8 = 0.0;
        locals.var_vjsmrev_dn9 = 0.0;
        locals.var_vjsmrev_dn10 = 0.0;
        locals.var_vjsmrev_dn11 = 0.0;
        locals.var_vjsmrev_dn13 = 0.0;
        locals.var_vjsmrev_dn14 = 0.0;
        locals.var_vjsmrev_rv = 0.0;

        locals.var_vjdmfwd = 0.0;
        locals.var_vjdmfwd_dn0 = 0.0;
        locals.var_vjdmfwd_dn2 = 0.0;
        locals.var_vjdmfwd_dn3 = 0.0;
        locals.var_vjdmfwd_dn4 = 0.0;
        locals.var_vjdmfwd_dn5 = 0.0;
        locals.var_vjdmfwd_dn6 = 0.0;
        locals.var_vjdmfwd_dn7 = 0.0;
        locals.var_vjdmfwd_dn8 = 0.0;
        locals.var_vjdmfwd_dn9 = 0.0;
        locals.var_vjdmfwd_dn10 = 0.0;
        locals.var_vjdmfwd_dn11 = 0.0;
        locals.var_vjdmfwd_dn13 = 0.0;
        locals.var_vjdmfwd_dn14 = 0.0;
        locals.var_vjdmfwd_rv = 0.0;

        locals.var_xexpbvd = 0.0;
        locals.var_xexpbvd_dn4 = 0.0;
        locals.var_xexpbvd_rv = 0.0;

        locals.var_dslprev = 0.0;
        locals.var_dslprev_dn0 = 0.0;
        locals.var_dslprev_dn2 = 0.0;
        locals.var_dslprev_dn3 = 0.0;
        locals.var_dslprev_dn4 = 0.0;
        locals.var_dslprev_dn5 = 0.0;
        locals.var_dslprev_dn6 = 0.0;
        locals.var_dslprev_dn7 = 0.0;
        locals.var_dslprev_dn8 = 0.0;
        locals.var_dslprev_dn9 = 0.0;
        locals.var_dslprev_dn10 = 0.0;
        locals.var_dslprev_dn11 = 0.0;
        locals.var_dslprev_dn13 = 0.0;
        locals.var_dslprev_dn14 = 0.0;
        locals.var_dslprev_rv = 0.0;

        locals.var_ivjdmrev = 0.0;
        locals.var_ivjdmrev_dn0 = 0.0;
        locals.var_ivjdmrev_dn2 = 0.0;
        locals.var_ivjdmrev_dn3 = 0.0;
        locals.var_ivjdmrev_dn4 = 0.0;
        locals.var_ivjdmrev_dn5 = 0.0;
        locals.var_ivjdmrev_dn6 = 0.0;
        locals.var_ivjdmrev_dn7 = 0.0;
        locals.var_ivjdmrev_dn8 = 0.0;
        locals.var_ivjdmrev_dn9 = 0.0;
        locals.var_ivjdmrev_dn10 = 0.0;
        locals.var_ivjdmrev_dn11 = 0.0;
        locals.var_ivjdmrev_dn13 = 0.0;
        locals.var_ivjdmrev_dn14 = 0.0;
        locals.var_ivjdmrev_rv = 0.0;

        locals.var_vjdmrev = 0.0;
        locals.var_vjdmrev_dn0 = 0.0;
        locals.var_vjdmrev_dn2 = 0.0;
        locals.var_vjdmrev_dn3 = 0.0;
        locals.var_vjdmrev_dn4 = 0.0;
        locals.var_vjdmrev_dn5 = 0.0;
        locals.var_vjdmrev_dn6 = 0.0;
        locals.var_vjdmrev_dn7 = 0.0;
        locals.var_vjdmrev_dn8 = 0.0;
        locals.var_vjdmrev_dn9 = 0.0;
        locals.var_vjdmrev_dn10 = 0.0;
        locals.var_vjdmrev_dn11 = 0.0;
        locals.var_vjdmrev_dn13 = 0.0;
        locals.var_vjdmrev_dn14 = 0.0;
        locals.var_vjdmrev_rv = 0.0;

        locals.var_vec1s = 0.0;
        locals.var_vec1s_dn4 = 0.0;
        locals.var_vec1s_rv = 0.0;

        locals.var_pb21s = 0.0;
        locals.var_pb21s_dn4 = 0.0;
        locals.var_pb21s_rv = 0.0;

        locals.var_vec2s = 0.0;
        locals.var_vec2s_dn4 = 0.0;
        locals.var_vec2s_rv = 0.0;

        locals.var_pb22s = 0.0;
        locals.var_pb22s_dn4 = 0.0;
        locals.var_pb22s_rv = 0.0;

        locals.var_vec3s = 0.0;
        locals.var_vec3s_dn4 = 0.0;
        locals.var_vec3s_rv = 0.0;

        locals.var_pb23s = 0.0;
        locals.var_pb23s_dn4 = 0.0;
        locals.var_pb23s_rv = 0.0;

        locals.var_vec1d = 0.0;
        locals.var_vec1d_dn4 = 0.0;
        locals.var_vec1d_rv = 0.0;

        locals.var_pb21d = 0.0;
        locals.var_pb21d_dn4 = 0.0;
        locals.var_pb21d_rv = 0.0;

        locals.var_vec2d = 0.0;
        locals.var_vec2d_dn4 = 0.0;
        locals.var_vec2d_rv = 0.0;

        locals.var_pb22d = 0.0;
        locals.var_pb22d_dn4 = 0.0;
        locals.var_pb22d_rv = 0.0;

        locals.var_vec3d = 0.0;
        locals.var_vec3d_dn4 = 0.0;
        locals.var_vec3d_rv = 0.0;

        locals.var_pb23d = 0.0;
        locals.var_pb23d_dn4 = 0.0;
        locals.var_pb23d_rv = 0.0;

        locals.var_lambdac_fn2 = 0.0;
        locals.var_lambdac_fn2_dn0 = 0.0;
        locals.var_lambdac_fn2_dn2 = 0.0;
        locals.var_lambdac_fn2_dn3 = 0.0;
        locals.var_lambdac_fn2_dn4 = 0.0;
        locals.var_lambdac_fn2_dn5 = 0.0;
        locals.var_lambdac_fn2_dn6 = 0.0;
        locals.var_lambdac_fn2_dn7 = 0.0;
        locals.var_lambdac_fn2_dn8 = 0.0;
        locals.var_lambdac_fn2_dn9 = 0.0;
        locals.var_lambdac_fn2_dn10 = 0.0;
        locals.var_lambdac_fn2_dn11 = 0.0;
        locals.var_lambdac_fn2_dn13 = 0.0;
        locals.var_lambdac_fn2_dn14 = 0.0;
        locals.var_lambdac_fn2_rv = 0.0;

        locals.var_noia2_i = 0.0;
        locals.var_noia2_i_rv = 0.0;

        locals.var_mpower_i = 0.0;
        locals.var_mpower_i_rv = 0.0;

        locals.var_dr0 = 0.0;
        locals.var_dr0_dn0 = 0.0;
        locals.var_dr0_dn2 = 0.0;
        locals.var_dr0_dn3 = 0.0;
        locals.var_dr0_dn4 = 0.0;
        locals.var_dr0_dn5 = 0.0;
        locals.var_dr0_dn6 = 0.0;
        locals.var_dr0_dn7 = 0.0;
        locals.var_dr0_dn8 = 0.0;
        locals.var_dr0_dn9 = 0.0;
        locals.var_dr0_dn10 = 0.0;
        locals.var_dr0_dn11 = 0.0;
        locals.var_dr0_dn13 = 0.0;
        locals.var_dr0_dn14 = 0.0;
        locals.var_dr0_rv = 0.0;

        locals.var_sigrat = 0.0;
        locals.var_sigrat_dn0 = 0.0;
        locals.var_sigrat_dn2 = 0.0;
        locals.var_sigrat_dn3 = 0.0;
        locals.var_sigrat_dn4 = 0.0;
        locals.var_sigrat_dn5 = 0.0;
        locals.var_sigrat_dn6 = 0.0;
        locals.var_sigrat_dn7 = 0.0;
        locals.var_sigrat_dn8 = 0.0;
        locals.var_sigrat_dn9 = 0.0;
        locals.var_sigrat_dn10 = 0.0;
        locals.var_sigrat_dn11 = 0.0;
        locals.var_sigrat_dn13 = 0.0;
        locals.var_sigrat_dn14 = 0.0;
        locals.var_sigrat_rv = 0.0;

        locals.var_cth = 0.0;
        locals.var_cth_dn0 = 0.0;
        locals.var_cth_dn2 = 0.0;
        locals.var_cth_dn3 = 0.0;
        locals.var_cth_dn4 = 0.0;
        locals.var_cth_dn5 = 0.0;
        locals.var_cth_dn6 = 0.0;
        locals.var_cth_dn7 = 0.0;
        locals.var_cth_dn8 = 0.0;
        locals.var_cth_dn9 = 0.0;
        locals.var_cth_dn10 = 0.0;
        locals.var_cth_dn11 = 0.0;
        locals.var_cth_dn13 = 0.0;
        locals.var_cth_dn14 = 0.0;
        locals.var_cth_rv = 0.0;

        locals.var_citr_i = 0.0;
        locals.var_citr_i_rv = 0.0;

        locals.var_cdscdr_i = 0.0;
        locals.var_cdscdr_i_rv = 0.0;

        locals.var_eta0r_i = 0.0;
        locals.var_eta0r_i_rv = 0.0;

        locals.var_dvtshiftr_i = 0.0;
        locals.var_dvtshiftr_i_rv = 0.0;

        locals.var_veseff = 0.0;
        locals.var_veseff_dn0 = 0.0;
        locals.var_veseff_dn2 = 0.0;
        locals.var_veseff_dn3 = 0.0;
        locals.var_veseff_dn4 = 0.0;
        locals.var_veseff_dn5 = 0.0;
        locals.var_veseff_dn6 = 0.0;
        locals.var_veseff_dn7 = 0.0;
        locals.var_veseff_dn8 = 0.0;
        locals.var_veseff_dn9 = 0.0;
        locals.var_veseff_dn10 = 0.0;
        locals.var_veseff_dn11 = 0.0;
        locals.var_veseff_dn13 = 0.0;
        locals.var_veseff_dn14 = 0.0;
        locals.var_veseff_rv = 0.0;

        locals.var_phibe_i = 0.0;
        locals.var_phibe_i_rv = 0.0;

        locals.var_k1_i = 0.0;
        locals.var_k1_i_rv = 0.0;

        locals.var_k11_i = 0.0;
        locals.var_k11_i_rv = 0.0;

        locals.var_k2sat_i = 0.0;
        locals.var_k2sat_i_rv = 0.0;

        locals.var_k2sat1_i = 0.0;
        locals.var_k2sat1_i_rv = 0.0;

        locals.var_k2_i = 0.0;
        locals.var_k2_i_rv = 0.0;

        locals.var_k21_i = 0.0;
        locals.var_k21_i_rv = 0.0;

        locals.var_vsatr_i = 0.0;
        locals.var_vsatr_i_rv = 0.0;

        locals.var_vsat1r_i = 0.0;
        locals.var_vsat1r_i_dn0 = 0.0;
        locals.var_vsat1r_i_dn2 = 0.0;
        locals.var_vsat1r_i_dn3 = 0.0;
        locals.var_vsat1r_i_dn4 = 0.0;
        locals.var_vsat1r_i_dn5 = 0.0;
        locals.var_vsat1r_i_dn6 = 0.0;
        locals.var_vsat1r_i_dn7 = 0.0;
        locals.var_vsat1r_i_dn8 = 0.0;
        locals.var_vsat1r_i_dn9 = 0.0;
        locals.var_vsat1r_i_dn10 = 0.0;
        locals.var_vsat1r_i_dn11 = 0.0;
        locals.var_vsat1r_i_dn13 = 0.0;
        locals.var_vsat1r_i_dn14 = 0.0;
        locals.var_vsat1r_i_rv = 0.0;

        locals.var_ksativr_i = 0.0;
        locals.var_ksativr_i_rv = 0.0;

        locals.var_mexpr_i = 0.0;
        locals.var_mexpr_i_dn0 = 0.0;
        locals.var_mexpr_i_dn2 = 0.0;
        locals.var_mexpr_i_dn3 = 0.0;
        locals.var_mexpr_i_dn4 = 0.0;
        locals.var_mexpr_i_dn5 = 0.0;
        locals.var_mexpr_i_dn6 = 0.0;
        locals.var_mexpr_i_dn7 = 0.0;
        locals.var_mexpr_i_dn8 = 0.0;
        locals.var_mexpr_i_dn9 = 0.0;
        locals.var_mexpr_i_dn10 = 0.0;
        locals.var_mexpr_i_dn11 = 0.0;
        locals.var_mexpr_i_dn13 = 0.0;
        locals.var_mexpr_i_dn14 = 0.0;
        locals.var_mexpr_i_rv = 0.0;

        locals.var_ptwgr_i = 0.0;
        locals.var_ptwgr_i_dn0 = 0.0;
        locals.var_ptwgr_i_dn2 = 0.0;
        locals.var_ptwgr_i_dn3 = 0.0;
        locals.var_ptwgr_i_dn4 = 0.0;
        locals.var_ptwgr_i_dn5 = 0.0;
        locals.var_ptwgr_i_dn6 = 0.0;
        locals.var_ptwgr_i_dn7 = 0.0;
        locals.var_ptwgr_i_dn8 = 0.0;
        locals.var_ptwgr_i_dn9 = 0.0;
        locals.var_ptwgr_i_dn10 = 0.0;
        locals.var_ptwgr_i_dn11 = 0.0;
        locals.var_ptwgr_i_dn13 = 0.0;
        locals.var_ptwgr_i_dn14 = 0.0;
        locals.var_ptwgr_i_rv = 0.0;

        locals.var_atr_i = 0.0;
        locals.var_atr_i_rv = 0.0;

        locals.var_u0r_i = 0.0;
        locals.var_u0r_i_dn0 = 0.0;
        locals.var_u0r_i_dn2 = 0.0;
        locals.var_u0r_i_dn3 = 0.0;
        locals.var_u0r_i_dn4 = 0.0;
        locals.var_u0r_i_dn5 = 0.0;
        locals.var_u0r_i_dn6 = 0.0;
        locals.var_u0r_i_dn7 = 0.0;
        locals.var_u0r_i_dn8 = 0.0;
        locals.var_u0r_i_dn9 = 0.0;
        locals.var_u0r_i_dn10 = 0.0;
        locals.var_u0r_i_dn11 = 0.0;
        locals.var_u0r_i_dn13 = 0.0;
        locals.var_u0r_i_dn14 = 0.0;
        locals.var_u0r_i_rv = 0.0;

        locals.var_upr_i = 0.0;
        locals.var_upr_i_rv = 0.0;

        locals.var_uar_i = 0.0;
        locals.var_uar_i_dn0 = 0.0;
        locals.var_uar_i_dn2 = 0.0;
        locals.var_uar_i_dn3 = 0.0;
        locals.var_uar_i_dn4 = 0.0;
        locals.var_uar_i_dn5 = 0.0;
        locals.var_uar_i_dn6 = 0.0;
        locals.var_uar_i_dn7 = 0.0;
        locals.var_uar_i_dn8 = 0.0;
        locals.var_uar_i_dn9 = 0.0;
        locals.var_uar_i_dn10 = 0.0;
        locals.var_uar_i_dn11 = 0.0;
        locals.var_uar_i_dn13 = 0.0;
        locals.var_uar_i_dn14 = 0.0;
        locals.var_uar_i_rv = 0.0;

        locals.var_uc_i = 0.0;
        locals.var_uc_i_rv = 0.0;

        locals.var_ucr_i = 0.0;
        locals.var_ucr_i_rv = 0.0;

        locals.var_eur_i = 0.0;
        locals.var_eur_i_dn0 = 0.0;
        locals.var_eur_i_dn2 = 0.0;
        locals.var_eur_i_dn3 = 0.0;
        locals.var_eur_i_dn4 = 0.0;
        locals.var_eur_i_dn5 = 0.0;
        locals.var_eur_i_dn6 = 0.0;
        locals.var_eur_i_dn7 = 0.0;
        locals.var_eur_i_dn8 = 0.0;
        locals.var_eur_i_dn9 = 0.0;
        locals.var_eur_i_dn10 = 0.0;
        locals.var_eur_i_dn11 = 0.0;
        locals.var_eur_i_dn13 = 0.0;
        locals.var_eur_i_dn14 = 0.0;
        locals.var_eur_i_rv = 0.0;

        locals.var_udr_i = 0.0;
        locals.var_udr_i_dn0 = 0.0;
        locals.var_udr_i_dn2 = 0.0;
        locals.var_udr_i_dn3 = 0.0;
        locals.var_udr_i_dn4 = 0.0;
        locals.var_udr_i_dn5 = 0.0;
        locals.var_udr_i_dn6 = 0.0;
        locals.var_udr_i_dn7 = 0.0;
        locals.var_udr_i_dn8 = 0.0;
        locals.var_udr_i_dn9 = 0.0;
        locals.var_udr_i_dn10 = 0.0;
        locals.var_udr_i_dn11 = 0.0;
        locals.var_udr_i_dn13 = 0.0;
        locals.var_udr_i_dn14 = 0.0;
        locals.var_udr_i_rv = 0.0;

        locals.var_uter_i = 0.0;
        locals.var_uter_i_rv = 0.0;

        locals.var_utlr_i = 0.0;
        locals.var_utlr_i_rv = 0.0;

        locals.var_ua1r_i = 0.0;
        locals.var_ua1r_i_rv = 0.0;

        locals.var_uc1_i = 0.0;
        locals.var_uc1_i_rv = 0.0;

        locals.var_uc1r_i = 0.0;
        locals.var_uc1r_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        locals: &mut StampLocals,
    ) {
        locals.var_ud1r_i = 0.0;
        locals.var_ud1r_i_rv = 0.0;

        locals.var_pdibl1r_i = 0.0;
        locals.var_pdibl1r_i_rv = 0.0;

        locals.var_pdibl2r_i = 0.0;
        locals.var_pdibl2r_i_rv = 0.0;

        locals.var_pclmr_i = 0.0;
        locals.var_pclmr_i_dn0 = 0.0;
        locals.var_pclmr_i_dn2 = 0.0;
        locals.var_pclmr_i_dn3 = 0.0;
        locals.var_pclmr_i_dn4 = 0.0;
        locals.var_pclmr_i_dn5 = 0.0;
        locals.var_pclmr_i_dn6 = 0.0;
        locals.var_pclmr_i_dn7 = 0.0;
        locals.var_pclmr_i_dn8 = 0.0;
        locals.var_pclmr_i_dn9 = 0.0;
        locals.var_pclmr_i_dn10 = 0.0;
        locals.var_pclmr_i_dn11 = 0.0;
        locals.var_pclmr_i_dn13 = 0.0;
        locals.var_pclmr_i_dn14 = 0.0;
        locals.var_pclmr_i_rv = 0.0;

        locals.var_cgso_i = 0.0;
        locals.var_cgso_i_rv = 0.0;

        locals.var_cgdo_i = 0.0;
        locals.var_cgdo_i_rv = 0.0;

        locals.var_covd_i = 0.0;
        locals.var_covd_i_dn0 = 0.0;
        locals.var_covd_i_dn2 = 0.0;
        locals.var_covd_i_dn3 = 0.0;
        locals.var_covd_i_dn4 = 0.0;
        locals.var_covd_i_dn5 = 0.0;
        locals.var_covd_i_dn6 = 0.0;
        locals.var_covd_i_dn7 = 0.0;
        locals.var_covd_i_dn8 = 0.0;
        locals.var_covd_i_dn9 = 0.0;
        locals.var_covd_i_dn10 = 0.0;
        locals.var_covd_i_dn11 = 0.0;
        locals.var_covd_i_dn13 = 0.0;
        locals.var_covd_i_dn14 = 0.0;
        locals.var_covd_i_rv = 0.0;

        locals.var_covs_i = 0.0;
        locals.var_covs_i_dn0 = 0.0;
        locals.var_covs_i_dn2 = 0.0;
        locals.var_covs_i_dn3 = 0.0;
        locals.var_covs_i_dn4 = 0.0;
        locals.var_covs_i_dn5 = 0.0;
        locals.var_covs_i_dn6 = 0.0;
        locals.var_covs_i_dn7 = 0.0;
        locals.var_covs_i_dn8 = 0.0;
        locals.var_covs_i_dn9 = 0.0;
        locals.var_covs_i_dn10 = 0.0;
        locals.var_covs_i_dn11 = 0.0;
        locals.var_covs_i_dn13 = 0.0;
        locals.var_covs_i_dn14 = 0.0;
        locals.var_covs_i_rv = 0.0;

        locals.var_cins = 0.0;
        locals.var_cins_rv = 0.0;

        locals.var_ach = 0.0;
        locals.var_ach_rv = 0.0;

        locals.var_weff_ufcm = 0.0;
        locals.var_weff_ufcm_rv = 0.0;

        locals.var_weffb = 0.0;
        locals.var_weffb_rv = 0.0;

        locals.var_rc = 0.0;
        locals.var_rc_rv = 0.0;

        locals.var_qdep_ov_cins = 0.0;
        locals.var_qdep_ov_cins_rv = 0.0;

        locals.var_qi_acc_for_qm = 0.0;
        locals.var_qi_acc_for_qm_dn0 = 0.0;
        locals.var_qi_acc_for_qm_dn2 = 0.0;
        locals.var_qi_acc_for_qm_dn3 = 0.0;
        locals.var_qi_acc_for_qm_dn4 = 0.0;
        locals.var_qi_acc_for_qm_dn5 = 0.0;
        locals.var_qi_acc_for_qm_dn6 = 0.0;
        locals.var_qi_acc_for_qm_dn7 = 0.0;
        locals.var_qi_acc_for_qm_dn8 = 0.0;
        locals.var_qi_acc_for_qm_dn9 = 0.0;
        locals.var_qi_acc_for_qm_dn10 = 0.0;
        locals.var_qi_acc_for_qm_dn11 = 0.0;
        locals.var_qi_acc_for_qm_dn13 = 0.0;
        locals.var_qi_acc_for_qm_dn14 = 0.0;
        locals.var_qi_acc_for_qm_rv = 0.0;

        locals.var_nq = 0.0;
        locals.var_nq_dn0 = 0.0;
        locals.var_nq_dn2 = 0.0;
        locals.var_nq_dn3 = 0.0;
        locals.var_nq_dn4 = 0.0;
        locals.var_nq_dn5 = 0.0;
        locals.var_nq_dn6 = 0.0;
        locals.var_nq_dn7 = 0.0;
        locals.var_nq_dn8 = 0.0;
        locals.var_nq_dn9 = 0.0;
        locals.var_nq_dn10 = 0.0;
        locals.var_nq_dn11 = 0.0;
        locals.var_nq_dn13 = 0.0;
        locals.var_nq_dn14 = 0.0;
        locals.var_nq_rv = 0.0;

        locals.var_qis = 0.0;
        locals.var_qis_dn0 = 0.0;
        locals.var_qis_dn2 = 0.0;
        locals.var_qis_dn3 = 0.0;
        locals.var_qis_dn4 = 0.0;
        locals.var_qis_dn5 = 0.0;
        locals.var_qis_dn6 = 0.0;
        locals.var_qis_dn7 = 0.0;
        locals.var_qis_dn8 = 0.0;
        locals.var_qis_dn9 = 0.0;
        locals.var_qis_dn10 = 0.0;
        locals.var_qis_dn11 = 0.0;
        locals.var_qis_dn13 = 0.0;
        locals.var_qis_dn14 = 0.0;
        locals.var_qis_rv = 0.0;

        locals.var_qid = 0.0;
        locals.var_qid_dn0 = 0.0;
        locals.var_qid_dn2 = 0.0;
        locals.var_qid_dn3 = 0.0;
        locals.var_qid_dn4 = 0.0;
        locals.var_qid_dn5 = 0.0;
        locals.var_qid_dn6 = 0.0;
        locals.var_qid_dn7 = 0.0;
        locals.var_qid_dn8 = 0.0;
        locals.var_qid_dn9 = 0.0;
        locals.var_qid_dn10 = 0.0;
        locals.var_qid_dn11 = 0.0;
        locals.var_qid_dn13 = 0.0;
        locals.var_qid_dn14 = 0.0;
        locals.var_qid_rv = 0.0;

        locals.var_qbov = 0.0;
        locals.var_qbov_dn0 = 0.0;
        locals.var_qbov_dn2 = 0.0;
        locals.var_qbov_dn3 = 0.0;
        locals.var_qbov_dn4 = 0.0;
        locals.var_qbov_dn5 = 0.0;
        locals.var_qbov_dn6 = 0.0;
        locals.var_qbov_dn7 = 0.0;
        locals.var_qbov_dn8 = 0.0;
        locals.var_qbov_dn9 = 0.0;
        locals.var_qbov_dn10 = 0.0;
        locals.var_qbov_dn11 = 0.0;
        locals.var_qbov_dn13 = 0.0;
        locals.var_qbov_dn14 = 0.0;
        locals.var_qbov_rv = 0.0;

        locals.var_qbov_s = 0.0;
        locals.var_qbov_s_dn0 = 0.0;
        locals.var_qbov_s_dn2 = 0.0;
        locals.var_qbov_s_dn3 = 0.0;
        locals.var_qbov_s_dn4 = 0.0;
        locals.var_qbov_s_dn5 = 0.0;
        locals.var_qbov_s_dn6 = 0.0;
        locals.var_qbov_s_dn7 = 0.0;
        locals.var_qbov_s_dn8 = 0.0;
        locals.var_qbov_s_dn9 = 0.0;
        locals.var_qbov_s_dn10 = 0.0;
        locals.var_qbov_s_dn11 = 0.0;
        locals.var_qbov_s_dn13 = 0.0;
        locals.var_qbov_s_dn14 = 0.0;
        locals.var_qbov_s_rv = 0.0;

        locals.var_ach2 = 0.0;
        locals.var_ach2_rv = 0.0;

        locals.var_ach3 = 0.0;
        locals.var_ach3_rv = 0.0;

        locals.var_ach4 = 0.0;
        locals.var_ach4_rv = 0.0;

        locals.var_ach5 = 0.0;
        locals.var_ach5_rv = 0.0;

        locals.var_ach6 = 0.0;
        locals.var_ach6_rv = 0.0;

        locals.var_weff2 = 0.0;
        locals.var_weff2_rv = 0.0;

        locals.var_weff3 = 0.0;
        locals.var_weff3_rv = 0.0;

        locals.var_weff4 = 0.0;
        locals.var_weff4_rv = 0.0;

        locals.var_weff5 = 0.0;
        locals.var_weff5_rv = 0.0;

        locals.var_weff6 = 0.0;
        locals.var_weff6_rv = 0.0;

        locals.var_qnds1 = 0.0;
        locals.var_qnds1_dn0 = 0.0;
        locals.var_qnds1_dn2 = 0.0;
        locals.var_qnds1_dn3 = 0.0;
        locals.var_qnds1_dn4 = 0.0;
        locals.var_qnds1_dn5 = 0.0;
        locals.var_qnds1_dn6 = 0.0;
        locals.var_qnds1_dn7 = 0.0;
        locals.var_qnds1_dn8 = 0.0;
        locals.var_qnds1_dn9 = 0.0;
        locals.var_qnds1_dn10 = 0.0;
        locals.var_qnds1_dn11 = 0.0;
        locals.var_qnds1_dn13 = 0.0;
        locals.var_qnds1_dn14 = 0.0;
        locals.var_qnds1_rv = 0.0;

        locals.var_qnds2 = 0.0;
        locals.var_qnds2_dn0 = 0.0;
        locals.var_qnds2_dn2 = 0.0;
        locals.var_qnds2_dn3 = 0.0;
        locals.var_qnds2_dn4 = 0.0;
        locals.var_qnds2_dn5 = 0.0;
        locals.var_qnds2_dn6 = 0.0;
        locals.var_qnds2_dn7 = 0.0;
        locals.var_qnds2_dn8 = 0.0;
        locals.var_qnds2_dn9 = 0.0;
        locals.var_qnds2_dn10 = 0.0;
        locals.var_qnds2_dn11 = 0.0;
        locals.var_qnds2_dn13 = 0.0;
        locals.var_qnds2_dn14 = 0.0;
        locals.var_qnds2_rv = 0.0;

        locals.var_qnds3 = 0.0;
        locals.var_qnds3_dn0 = 0.0;
        locals.var_qnds3_dn2 = 0.0;
        locals.var_qnds3_dn3 = 0.0;
        locals.var_qnds3_dn4 = 0.0;
        locals.var_qnds3_dn5 = 0.0;
        locals.var_qnds3_dn6 = 0.0;
        locals.var_qnds3_dn7 = 0.0;
        locals.var_qnds3_dn8 = 0.0;
        locals.var_qnds3_dn9 = 0.0;
        locals.var_qnds3_dn10 = 0.0;
        locals.var_qnds3_dn11 = 0.0;
        locals.var_qnds3_dn13 = 0.0;
        locals.var_qnds3_dn14 = 0.0;
        locals.var_qnds3_rv = 0.0;

        locals.var_qndd1 = 0.0;
        locals.var_qndd1_dn0 = 0.0;
        locals.var_qndd1_dn2 = 0.0;
        locals.var_qndd1_dn3 = 0.0;
        locals.var_qndd1_dn4 = 0.0;
        locals.var_qndd1_dn5 = 0.0;
        locals.var_qndd1_dn6 = 0.0;
        locals.var_qndd1_dn7 = 0.0;
        locals.var_qndd1_dn8 = 0.0;
        locals.var_qndd1_dn9 = 0.0;
        locals.var_qndd1_dn10 = 0.0;
        locals.var_qndd1_dn11 = 0.0;
        locals.var_qndd1_dn13 = 0.0;
        locals.var_qndd1_dn14 = 0.0;
        locals.var_qndd1_rv = 0.0;

        locals.var_qndd2 = 0.0;
        locals.var_qndd2_dn0 = 0.0;
        locals.var_qndd2_dn2 = 0.0;
        locals.var_qndd2_dn3 = 0.0;
        locals.var_qndd2_dn4 = 0.0;
        locals.var_qndd2_dn5 = 0.0;
        locals.var_qndd2_dn6 = 0.0;
        locals.var_qndd2_dn7 = 0.0;
        locals.var_qndd2_dn8 = 0.0;
        locals.var_qndd2_dn9 = 0.0;
        locals.var_qndd2_dn10 = 0.0;
        locals.var_qndd2_dn11 = 0.0;
        locals.var_qndd2_dn13 = 0.0;
        locals.var_qndd2_dn14 = 0.0;
        locals.var_qndd2_rv = 0.0;

        locals.var_qndd3 = 0.0;
        locals.var_qndd3_dn0 = 0.0;
        locals.var_qndd3_dn2 = 0.0;
        locals.var_qndd3_dn3 = 0.0;
        locals.var_qndd3_dn4 = 0.0;
        locals.var_qndd3_dn5 = 0.0;
        locals.var_qndd3_dn6 = 0.0;
        locals.var_qndd3_dn7 = 0.0;
        locals.var_qndd3_dn8 = 0.0;
        locals.var_qndd3_dn9 = 0.0;
        locals.var_qndd3_dn10 = 0.0;
        locals.var_qndd3_dn11 = 0.0;
        locals.var_qndd3_dn13 = 0.0;
        locals.var_qndd3_dn14 = 0.0;
        locals.var_qndd3_rv = 0.0;

        locals.var_nc3d = 1.0;
        locals.var_nc3d_rv = 0.0;

        locals.var_d1 = 0.0;
        locals.var_d1_rv = 0.0;

        locals.var_d2 = 0.0;
        locals.var_d2_rv = 0.0;

        locals.var_d3 = 0.0;
        locals.var_d3_rv = 0.0;

        locals.var_p1 = 0.0;
        locals.var_p1_rv = 0.0;

        locals.var_p2 = 0.0;
        locals.var_p2_rv = 0.0;

        locals.var_p3 = 0.0;
        locals.var_p3_rv = 0.0;

        locals.var_ncq = 0.0;
        locals.var_ncq_rv = 0.0;

        locals.var_qe2 = 0.0;
        locals.var_qe2_dn0 = 0.0;
        locals.var_qe2_dn2 = 0.0;
        locals.var_qe2_dn3 = 0.0;
        locals.var_qe2_dn4 = 0.0;
        locals.var_qe2_dn5 = 0.0;
        locals.var_qe2_dn6 = 0.0;
        locals.var_qe2_dn7 = 0.0;
        locals.var_qe2_dn8 = 0.0;
        locals.var_qe2_dn9 = 0.0;
        locals.var_qe2_dn10 = 0.0;
        locals.var_qe2_dn11 = 0.0;
        locals.var_qe2_dn13 = 0.0;
        locals.var_qe2_dn14 = 0.0;
        locals.var_qe2_rv = 0.0;

        locals.var_qe3 = 0.0;
        locals.var_qe3_rv = 0.0;

        locals.var_qnd10 = 0.0;
        locals.var_qnd10_rv = 0.0;

        locals.var_qnd20 = 0.0;
        locals.var_qnd20_rv = 0.0;

        locals.var_qnd30 = 0.0;
        locals.var_qnd30_rv = 0.0;

        locals.var_dimension1_i = 0.0;
        locals.var_dimension1_i_rv = 0.0;

        locals.var_dimension2_i = 0.0;
        locals.var_dimension2_i_rv = 0.0;

        locals.var_dimension3_i = 0.0;
        locals.var_dimension3_i_rv = 0.0;

        locals.var_ssp1_i = 0.0;
        locals.var_ssp1_i_rv = 0.0;

        locals.var_ssp2_i = 0.0;
        locals.var_ssp2_i_rv = 0.0;

        locals.var_ssp3_i = 0.0;
        locals.var_ssp3_i_rv = 0.0;

        locals.var_e2nom_i = 0.0;
        locals.var_e2nom_i_rv = 0.0;

        locals.var_e3nom_i = 0.0;
        locals.var_e3nom_i_rv = 0.0;

        locals.var_mfq1nom_i = 0.0;
        locals.var_mfq1nom_i_rv = 0.0;

        locals.var_mfq2nom_i = 0.0;
        locals.var_mfq2nom_i_rv = 0.0;

        locals.var_mfq3nom_i = 0.0;
        locals.var_mfq3nom_i_rv = 0.0;

        locals.var_devtemplow0 = 0.0;
        locals.var_devtemplow0_dn4 = 0.0;
        locals.var_devtemplow0_rv = 0.0;

        locals.var_devtemplow1 = 0.0;
        locals.var_devtemplow1_dn4 = 0.0;
        locals.var_devtemplow1_rv = 0.0;

        locals.var_devtempeff = 0.0;
        locals.var_devtempeff_dn0 = 0.0;
        locals.var_devtempeff_dn2 = 0.0;
        locals.var_devtempeff_dn3 = 0.0;
        locals.var_devtempeff_dn4 = 0.0;
        locals.var_devtempeff_dn5 = 0.0;
        locals.var_devtempeff_dn6 = 0.0;
        locals.var_devtempeff_dn7 = 0.0;
        locals.var_devtempeff_dn8 = 0.0;
        locals.var_devtempeff_dn9 = 0.0;
        locals.var_devtempeff_dn10 = 0.0;
        locals.var_devtempeff_dn11 = 0.0;
        locals.var_devtempeff_dn13 = 0.0;
        locals.var_devtempeff_dn14 = 0.0;
        locals.var_devtempeff_rv = 0.0;

        locals.var_devtemp1 = 0.0;
        locals.var_devtemp1_dn4 = 0.0;
        locals.var_devtemp1_rv = 0.0;

        locals.var_deltemp1 = 0.0;
        locals.var_deltemp1_dn4 = 0.0;
        locals.var_deltemp1_rv = 0.0;

        locals.var_deltratio1 = 0.0;
        locals.var_deltratio1_dn4 = 0.0;
        locals.var_deltratio1_rv = 0.0;

        locals.var_vtmeff = 0.0;
        locals.var_vtmeff_dn0 = 0.0;
        locals.var_vtmeff_dn2 = 0.0;
        locals.var_vtmeff_dn3 = 0.0;
        locals.var_vtmeff_dn4 = 0.0;
        locals.var_vtmeff_dn5 = 0.0;
        locals.var_vtmeff_dn6 = 0.0;
        locals.var_vtmeff_dn7 = 0.0;
        locals.var_vtmeff_dn8 = 0.0;
        locals.var_vtmeff_dn9 = 0.0;
        locals.var_vtmeff_dn10 = 0.0;
        locals.var_vtmeff_dn11 = 0.0;
        locals.var_vtmeff_dn13 = 0.0;
        locals.var_vtmeff_dn14 = 0.0;
        locals.var_vtmeff_rv = 0.0;

        locals.var_niln = 0.0;
        locals.var_niln_dn0 = 0.0;
        locals.var_niln_dn2 = 0.0;
        locals.var_niln_dn3 = 0.0;
        locals.var_niln_dn4 = 0.0;
        locals.var_niln_dn5 = 0.0;
        locals.var_niln_dn6 = 0.0;
        locals.var_niln_dn7 = 0.0;
        locals.var_niln_dn8 = 0.0;
        locals.var_niln_dn9 = 0.0;
        locals.var_niln_dn10 = 0.0;
        locals.var_niln_dn11 = 0.0;
        locals.var_niln_dn13 = 0.0;
        locals.var_niln_dn14 = 0.0;
        locals.var_niln_rv = 0.0;

        locals.var_uds_t = 0.0;
        locals.var_uds_t_dn4 = 0.0;
        locals.var_uds_t_rv = 0.0;

        locals.var_udd_t = 0.0;
        locals.var_udd_t_dn4 = 0.0;
        locals.var_udd_t_rv = 0.0;

        locals.var_ua_tl = 0.0;
        locals.var_ua_tl_dn0 = 0.0;
        locals.var_ua_tl_dn2 = 0.0;
        locals.var_ua_tl_dn3 = 0.0;
        locals.var_ua_tl_dn4 = 0.0;
        locals.var_ua_tl_dn5 = 0.0;
        locals.var_ua_tl_dn6 = 0.0;
        locals.var_ua_tl_dn7 = 0.0;
        locals.var_ua_tl_dn8 = 0.0;
        locals.var_ua_tl_dn9 = 0.0;
        locals.var_ua_tl_dn10 = 0.0;
        locals.var_ua_tl_dn11 = 0.0;
        locals.var_ua_tl_dn13 = 0.0;
        locals.var_ua_tl_dn14 = 0.0;
        locals.var_ua_tl_rv = 0.0;

        locals.var_ua_th = 0.0;
        locals.var_ua_th_dn0 = 0.0;
        locals.var_ua_th_dn2 = 0.0;
        locals.var_ua_th_dn3 = 0.0;
        locals.var_ua_th_dn4 = 0.0;
        locals.var_ua_th_dn5 = 0.0;
        locals.var_ua_th_dn6 = 0.0;
        locals.var_ua_th_dn7 = 0.0;
        locals.var_ua_th_dn8 = 0.0;
        locals.var_ua_th_dn9 = 0.0;
        locals.var_ua_th_dn10 = 0.0;
        locals.var_ua_th_dn11 = 0.0;
        locals.var_ua_th_dn13 = 0.0;
        locals.var_ua_th_dn14 = 0.0;
        locals.var_ua_th_rv = 0.0;

        locals.var_uar_tl = 0.0;
        locals.var_uar_tl_dn0 = 0.0;
        locals.var_uar_tl_dn2 = 0.0;
        locals.var_uar_tl_dn3 = 0.0;
        locals.var_uar_tl_dn4 = 0.0;
        locals.var_uar_tl_dn5 = 0.0;
        locals.var_uar_tl_dn6 = 0.0;
        locals.var_uar_tl_dn7 = 0.0;
        locals.var_uar_tl_dn8 = 0.0;
        locals.var_uar_tl_dn9 = 0.0;
        locals.var_uar_tl_dn10 = 0.0;
        locals.var_uar_tl_dn11 = 0.0;
        locals.var_uar_tl_dn13 = 0.0;
        locals.var_uar_tl_dn14 = 0.0;
        locals.var_uar_tl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_uar_th = 0.0;
        locals.var_uar_th_dn0 = 0.0;
        locals.var_uar_th_dn2 = 0.0;
        locals.var_uar_th_dn3 = 0.0;
        locals.var_uar_th_dn4 = 0.0;
        locals.var_uar_th_dn5 = 0.0;
        locals.var_uar_th_dn6 = 0.0;
        locals.var_uar_th_dn7 = 0.0;
        locals.var_uar_th_dn8 = 0.0;
        locals.var_uar_th_dn9 = 0.0;
        locals.var_uar_th_dn10 = 0.0;
        locals.var_uar_th_dn11 = 0.0;
        locals.var_uar_th_dn13 = 0.0;
        locals.var_uar_th_dn14 = 0.0;
        locals.var_uar_th_rv = 0.0;

        locals.var_wl = 0.0;
        locals.var_wl_dn4 = 0.0;
        locals.var_wl_rv = 0.0;

        locals.var_wh = 0.0;
        locals.var_wh_dn4 = 0.0;
        locals.var_wh_rv = 0.0;

        locals.var_uddeff_t = 0.0;
        locals.var_uddeff_t_dn4 = 0.0;
        locals.var_uddeff_t_rv = 0.0;

        locals.var_udseff_t = 0.0;
        locals.var_udseff_t_dn4 = 0.0;
        locals.var_udseff_t_rv = 0.0;

        locals.var_rdstempvs = 0.0;
        locals.var_rdstempvs_dn4 = 0.0;
        locals.var_rdstempvs_rv = 0.0;

        locals.var_deltaprsd_v = 0.0;
        locals.var_deltaprsd_v_rv = 0.0;

        locals.var_qsref_i = 0.0;
        locals.var_qsref_i_rv = 0.0;

        let assign2600_e3187: f64 = if p.p60 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign2600_e3187;
        locals.var_guard3_rv = 0.0;

        let (assign2610_e3191,) = {
    if (locals.var_guard3 != 0.0) {
        (1.0,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign2610_e3191;
        locals.var_devsign_rv = 0.0;

        let (assign2620_e3197,) = {
    if (locals.var_guard3 == 0.0) {
        let assign2620_e3195: f64 = (-1.0);
        (assign2620_e3195,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign2620_e3197;
        locals.var_devsign_rv = 0.0;

        let assign2630_e3200: f64 = (p.p103 * 8.8542e-12);
        locals.var_epssub = assign2630_e3200;
        locals.var_epssub_rv = 0.0;

        let assign2640_e3203: f64 = (p.p1088 * 8.8542e-12);
        locals.var_epssp = assign2640_e3203;
        locals.var_epssp_rv = 0.0;

        let assign2650_e3206: f64 = (p.p102 * 8.8542e-12);
        let assign2650_e3208: f64 = (assign2650_e3206 / p.p91);
        locals.var_cbox = assign2650_e3208;
        locals.var_cbox_rv = 0.0;

        let assign2660_e3211: f64 = (p.p103 / p.p102);
        locals.var_epsratio = assign2660_e3211;
        locals.var_epsratio_rv = 0.0;

        let assign2670_e3214: f64 = (0.916 * 9.11e-31);
        locals.var_mx = assign2670_e3214;
        locals.var_mx_rv = 0.0;

        let assign2680_e3217: f64 = (0.19 * 9.11e-31);
        locals.var_mxprime = assign2680_e3217;
        locals.var_mxprime_rv = 0.0;

        let assign2690_e3220: f64 = (0.19 * 9.11e-31);
        locals.var_md = assign2690_e3220;
        locals.var_md_rv = 0.0;

        let assign2700_e3223: f64 = (0.417 * 9.11e-31);
        locals.var_mdprime = assign2700_e3223;
        locals.var_mdprime_rv = 0.0;

        locals.var_gprime = 4.0;
        locals.var_gprime_rv = 0.0;

        locals.var_gfactor = 2.0;
        locals.var_gfactor_rv = 0.0;

        let assign2730_e3229: f64 = (1e-6 * p.p110);
        let assign2730_e3231: f64 = (assign2730_e3229 / p.p0);
        let assign2730_e3232: f64 = (p.p109 + assign2730_e3231);
        let assign2730_e3235: f64 = (p.p111 / p.p5);
        let assign2730_e3236: f64 = (assign2730_e3232 + assign2730_e3235);
        let assign2730_e3239: f64 = (p.p112 * 1e-6);
        let assign2730_e3242: f64 = (p.p0 * p.p5);
        let assign2730_e3243: f64 = (assign2730_e3239 / assign2730_e3242);
        let assign2730_e3244: f64 = (assign2730_e3236 + assign2730_e3243);
        locals.var_xl_i = assign2730_e3244;
        locals.var_xl_i_rv = 0.0;

        let assign2740_e3248: f64 = (1e-6 * p.p118);
        let assign2740_e3250: f64 = (assign2740_e3248 / p.p0);
        let assign2740_e3251: f64 = (p.p117 + assign2740_e3250);
        let assign2740_e3254: f64 = (p.p119 / p.p5);
        let assign2740_e3255: f64 = (assign2740_e3251 + assign2740_e3254);
        let assign2740_e3258: f64 = (p.p120 * 1e-6);
        let assign2740_e3261: f64 = (p.p0 * p.p5);
        let assign2740_e3262: f64 = (assign2740_e3258 / assign2740_e3261);
        let assign2740_e3263: f64 = (assign2740_e3255 + assign2740_e3262);
        locals.var_dlbin_i = assign2740_e3263;
        locals.var_dlbin_i_rv = 0.0;

        let assign2750_e3267: f64 = (1e-6 * p.p114);
        let assign2750_e3269: f64 = (assign2750_e3267 / p.p0);
        let assign2750_e3270: f64 = (p.p113 + assign2750_e3269);
        let assign2750_e3273: f64 = (p.p115 / p.p5);
        let assign2750_e3274: f64 = (assign2750_e3270 + assign2750_e3273);
        let assign2750_e3277: f64 = (p.p116 * 1e-6);
        let assign2750_e3280: f64 = (p.p0 * p.p5);
        let assign2750_e3281: f64 = (assign2750_e3277 / assign2750_e3280);
        let assign2750_e3282: f64 = (assign2750_e3274 + assign2750_e3281);
        locals.var_lint_i = assign2750_e3282;
        locals.var_lint_i_rv = 0.0;

        let assign2760_e3285: f64 = (p.p0 + locals.var_xl_i);
        locals.var_lg = assign2760_e3285;
        locals.var_lg_rv = 0.0;

        let assign2770_e3288: f64 = if locals.var_lg <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign2770_e3288;
        locals.var_guard4_rv = 0.0;

        let (assign2780_e3292,) = {
    if (locals.var_guard4 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_lg,)
    }
};
        locals.var_lg = assign2780_e3292;
        locals.var_lg_rv = 0.0;

        let assign2790_e3295: f64 = (-p.p84);
        let assign2790_e3296: f64 = (locals.var_lg).powf(assign2790_e3295);
        locals.var_t0 = assign2790_e3296;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign2800_e3300: f64 = (p.p83 * locals.var_t0);
        let assign2800_e3301: f64 = (locals.var_lint_i + assign2800_e3300);
        locals.var_deltal = assign2800_e3301;
        locals.var_deltal_dn0 = (p.p83 * locals.var_t0_dn0);
        locals.var_deltal_dn2 = (p.p83 * locals.var_t0_dn2);
        locals.var_deltal_dn3 = (p.p83 * locals.var_t0_dn3);
        locals.var_deltal_dn4 = (p.p83 * locals.var_t0_dn4);
        locals.var_deltal_dn5 = (p.p83 * locals.var_t0_dn5);
        locals.var_deltal_dn6 = (p.p83 * locals.var_t0_dn6);
        locals.var_deltal_dn7 = (p.p83 * locals.var_t0_dn7);
        locals.var_deltal_dn8 = (p.p83 * locals.var_t0_dn8);
        locals.var_deltal_dn9 = (p.p83 * locals.var_t0_dn9);
        locals.var_deltal_dn10 = (p.p83 * locals.var_t0_dn10);
        locals.var_deltal_dn11 = (p.p83 * locals.var_t0_dn11);
        locals.var_deltal_dn13 = (p.p83 * locals.var_t0_dn13);
        locals.var_deltal_dn14 = (p.p83 * locals.var_t0_dn14);
        locals.var_deltal_rv = 0.0;

        let assign2810_e3306: f64 = (locals.var_lg + locals.var_dlbin_i);
        let assign2810_e3308: f64 = (-p.p84);
        let assign2810_e3309: f64 = (assign2810_e3306).powf(assign2810_e3308);
        let assign2810_e3310: f64 = (p.p83 * assign2810_e3309);
        let assign2810_e3311: f64 = (locals.var_lint_i + assign2810_e3310);
        locals.var_deltal1 = assign2810_e3311;
        locals.var_deltal1_rv = 0.0;

        let assign2820_e3315: f64 = (p.p88 * locals.var_t0);
        let assign2820_e3316: f64 = (p.p85 + assign2820_e3315);
        locals.var_deltalcv = assign2820_e3316;
        locals.var_deltalcv_dn0 = (p.p88 * locals.var_t0_dn0);
        locals.var_deltalcv_dn2 = (p.p88 * locals.var_t0_dn2);
        locals.var_deltalcv_dn3 = (p.p88 * locals.var_t0_dn3);
        locals.var_deltalcv_dn4 = (p.p88 * locals.var_t0_dn4);
        locals.var_deltalcv_dn5 = (p.p88 * locals.var_t0_dn5);
        locals.var_deltalcv_dn6 = (p.p88 * locals.var_t0_dn6);
        locals.var_deltalcv_dn7 = (p.p88 * locals.var_t0_dn7);
        locals.var_deltalcv_dn8 = (p.p88 * locals.var_t0_dn8);
        locals.var_deltalcv_dn9 = (p.p88 * locals.var_t0_dn9);
        locals.var_deltalcv_dn10 = (p.p88 * locals.var_t0_dn10);
        locals.var_deltalcv_dn11 = (p.p88 * locals.var_t0_dn11);
        locals.var_deltalcv_dn13 = (p.p88 * locals.var_t0_dn13);
        locals.var_deltalcv_dn14 = (p.p88 * locals.var_t0_dn14);
        locals.var_deltalcv_rv = 0.0;

        let assign2830_e3320: f64 = (2.0 * locals.var_deltal);
        let assign2830_e3321: f64 = (locals.var_lg - assign2830_e3320);
        locals.var_leff_1 = assign2830_e3321;
        locals.var_leff_1_dn0 = (-(2.0 * locals.var_deltal_dn0));
        locals.var_leff_1_dn2 = (-(2.0 * locals.var_deltal_dn2));
        locals.var_leff_1_dn3 = (-(2.0 * locals.var_deltal_dn3));
        locals.var_leff_1_dn4 = (-(2.0 * locals.var_deltal_dn4));
        locals.var_leff_1_dn5 = (-(2.0 * locals.var_deltal_dn5));
        locals.var_leff_1_dn6 = (-(2.0 * locals.var_deltal_dn6));
        locals.var_leff_1_dn7 = (-(2.0 * locals.var_deltal_dn7));
        locals.var_leff_1_dn8 = (-(2.0 * locals.var_deltal_dn8));
        locals.var_leff_1_dn9 = (-(2.0 * locals.var_deltal_dn9));
        locals.var_leff_1_dn10 = (-(2.0 * locals.var_deltal_dn10));
        locals.var_leff_1_dn11 = (-(2.0 * locals.var_deltal_dn11));
        locals.var_leff_1_dn13 = (-(2.0 * locals.var_deltal_dn13));
        locals.var_leff_1_dn14 = (-(2.0 * locals.var_deltal_dn14));
        locals.var_leff_1_rv = 0.0;

        let assign2840_e3324: f64 = (locals.var_lg + locals.var_dlbin_i);
        let assign2840_e3327: f64 = (2.0 * locals.var_deltal1);
        let assign2840_e3328: f64 = (assign2840_e3324 - assign2840_e3327);
        locals.var_leff1 = assign2840_e3328;
        locals.var_leff1_rv = 0.0;

        let assign2850_e3332: f64 = (2.0 * locals.var_deltalcv);
        let assign2850_e3333: f64 = (locals.var_lg - assign2850_e3332);
        locals.var_leffcv_1 = assign2850_e3333;
        locals.var_leffcv_1_dn0 = (-(2.0 * locals.var_deltalcv_dn0));
        locals.var_leffcv_1_dn2 = (-(2.0 * locals.var_deltalcv_dn2));
        locals.var_leffcv_1_dn3 = (-(2.0 * locals.var_deltalcv_dn3));
        locals.var_leffcv_1_dn4 = (-(2.0 * locals.var_deltalcv_dn4));
        locals.var_leffcv_1_dn5 = (-(2.0 * locals.var_deltalcv_dn5));
        locals.var_leffcv_1_dn6 = (-(2.0 * locals.var_deltalcv_dn6));
        locals.var_leffcv_1_dn7 = (-(2.0 * locals.var_deltalcv_dn7));
        locals.var_leffcv_1_dn8 = (-(2.0 * locals.var_deltalcv_dn8));
        locals.var_leffcv_1_dn9 = (-(2.0 * locals.var_deltalcv_dn9));
        locals.var_leffcv_1_dn10 = (-(2.0 * locals.var_deltalcv_dn10));
        locals.var_leffcv_1_dn11 = (-(2.0 * locals.var_deltalcv_dn11));
        locals.var_leffcv_1_dn13 = (-(2.0 * locals.var_deltalcv_dn13));
        locals.var_leffcv_1_dn14 = (-(2.0 * locals.var_deltalcv_dn14));
        locals.var_leffcv_1_rv = 0.0;

        let assign2860_e3336: f64 = (locals.var_leffcv_1 - p.p86);
        locals.var_leffcv_acc = assign2860_e3336;
        locals.var_leffcv_acc_dn0 = locals.var_leffcv_1_dn0;
        locals.var_leffcv_acc_dn2 = locals.var_leffcv_1_dn2;
        locals.var_leffcv_acc_dn3 = locals.var_leffcv_1_dn3;
        locals.var_leffcv_acc_dn4 = locals.var_leffcv_1_dn4;
        locals.var_leffcv_acc_dn5 = locals.var_leffcv_1_dn5;
        locals.var_leffcv_acc_dn6 = locals.var_leffcv_1_dn6;
        locals.var_leffcv_acc_dn7 = locals.var_leffcv_1_dn7;
        locals.var_leffcv_acc_dn8 = locals.var_leffcv_1_dn8;
        locals.var_leffcv_acc_dn9 = locals.var_leffcv_1_dn9;
        locals.var_leffcv_acc_dn10 = locals.var_leffcv_1_dn10;
        locals.var_leffcv_acc_dn11 = locals.var_leffcv_1_dn11;
        locals.var_leffcv_acc_dn13 = locals.var_leffcv_1_dn13;
        locals.var_leffcv_acc_dn14 = locals.var_leffcv_1_dn14;
        locals.var_leffcv_acc_rv = 0.0;

        let assign2870_e3339: f64 = if locals.var_leff_1 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign2870_e3339;
        locals.var_guard5_rv = 0.0;

        let (assign2880_e3343, assign2880_e3343_d_n0, assign2880_e3343_d_n2, assign2880_e3343_d_n3, assign2880_e3343_d_n4, assign2880_e3343_d_n5, assign2880_e3343_d_n6, assign2880_e3343_d_n7, assign2880_e3343_d_n8, assign2880_e3343_d_n9, assign2880_e3343_d_n10, assign2880_e3343_d_n11, assign2880_e3343_d_n13, assign2880_e3343_d_n14,) = {
    if (locals.var_guard5 != 0.0) {
        (locals.var_lg, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_leff_1, locals.var_leff_1_dn0, locals.var_leff_1_dn2, locals.var_leff_1_dn3, locals.var_leff_1_dn4, locals.var_leff_1_dn5, locals.var_leff_1_dn6, locals.var_leff_1_dn7, locals.var_leff_1_dn8, locals.var_leff_1_dn9, locals.var_leff_1_dn10, locals.var_leff_1_dn11, locals.var_leff_1_dn13, locals.var_leff_1_dn14,)
    }
};
        locals.var_leff_1 = assign2880_e3343;
        locals.var_leff_1_dn0 = assign2880_e3343_d_n0;
        locals.var_leff_1_dn2 = assign2880_e3343_d_n2;
        locals.var_leff_1_dn3 = assign2880_e3343_d_n3;
        locals.var_leff_1_dn4 = assign2880_e3343_d_n4;
        locals.var_leff_1_dn5 = assign2880_e3343_d_n5;
        locals.var_leff_1_dn6 = assign2880_e3343_d_n6;
        locals.var_leff_1_dn7 = assign2880_e3343_d_n7;
        locals.var_leff_1_dn8 = assign2880_e3343_d_n8;
        locals.var_leff_1_dn9 = assign2880_e3343_d_n9;
        locals.var_leff_1_dn10 = assign2880_e3343_d_n10;
        locals.var_leff_1_dn11 = assign2880_e3343_d_n11;
        locals.var_leff_1_dn13 = assign2880_e3343_d_n13;
        locals.var_leff_1_dn14 = assign2880_e3343_d_n14;
        locals.var_leff_1_rv = 0.0;

        let assign2900_e3349: f64 = if locals.var_leff1 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign2900_e3349;
        locals.var_guard7_rv = 0.0;

        let (assign2910_e3353,) = {
    if (locals.var_guard7 != 0.0) {
        (locals.var_lg,)
    } else {
        (locals.var_leff1,)
    }
};
        locals.var_leff1 = assign2910_e3353;
        locals.var_leff1_rv = 0.0;

        let assign2930_e3359: f64 = if locals.var_leffcv_1 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign2930_e3359;
        locals.var_guard9_rv = 0.0;

        let (assign2940_e3363, assign2940_e3363_d_n0, assign2940_e3363_d_n2, assign2940_e3363_d_n3, assign2940_e3363_d_n4, assign2940_e3363_d_n5, assign2940_e3363_d_n6, assign2940_e3363_d_n7, assign2940_e3363_d_n8, assign2940_e3363_d_n9, assign2940_e3363_d_n10, assign2940_e3363_d_n11, assign2940_e3363_d_n13, assign2940_e3363_d_n14,) = {
    if (locals.var_guard9 != 0.0) {
        (locals.var_lg, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_leffcv_1, locals.var_leffcv_1_dn0, locals.var_leffcv_1_dn2, locals.var_leffcv_1_dn3, locals.var_leffcv_1_dn4, locals.var_leffcv_1_dn5, locals.var_leffcv_1_dn6, locals.var_leffcv_1_dn7, locals.var_leffcv_1_dn8, locals.var_leffcv_1_dn9, locals.var_leffcv_1_dn10, locals.var_leffcv_1_dn11, locals.var_leffcv_1_dn13, locals.var_leffcv_1_dn14,)
    }
};
        locals.var_leffcv_1 = assign2940_e3363;
        locals.var_leffcv_1_dn0 = assign2940_e3363_d_n0;
        locals.var_leffcv_1_dn2 = assign2940_e3363_d_n2;
        locals.var_leffcv_1_dn3 = assign2940_e3363_d_n3;
        locals.var_leffcv_1_dn4 = assign2940_e3363_d_n4;
        locals.var_leffcv_1_dn5 = assign2940_e3363_d_n5;
        locals.var_leffcv_1_dn6 = assign2940_e3363_d_n6;
        locals.var_leffcv_1_dn7 = assign2940_e3363_d_n7;
        locals.var_leffcv_1_dn8 = assign2940_e3363_d_n8;
        locals.var_leffcv_1_dn9 = assign2940_e3363_d_n9;
        locals.var_leffcv_1_dn10 = assign2940_e3363_d_n10;
        locals.var_leffcv_1_dn11 = assign2940_e3363_d_n11;
        locals.var_leffcv_1_dn13 = assign2940_e3363_d_n13;
        locals.var_leffcv_1_dn14 = assign2940_e3363_d_n14;
        locals.var_leffcv_1_rv = 0.0;

        let assign2960_e3369: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign2960_e3369;
        locals.var_guard11_rv = 0.0;

        let assign2970_e3372: f64 = if locals.var_leffcv_acc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign2970_e3372;
        locals.var_guard12_rv = 0.0;

        let (assign2980_e3378, assign2980_e3378_d_n0, assign2980_e3378_d_n2, assign2980_e3378_d_n3, assign2980_e3378_d_n4, assign2980_e3378_d_n5, assign2980_e3378_d_n6, assign2980_e3378_d_n7, assign2980_e3378_d_n8, assign2980_e3378_d_n9, assign2980_e3378_d_n10, assign2980_e3378_d_n11, assign2980_e3378_d_n13, assign2980_e3378_d_n14,) = {
    if ((locals.var_guard11 != 0.0) && (locals.var_guard12 != 0.0)) {
        (locals.var_lg, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_leffcv_acc, locals.var_leffcv_acc_dn0, locals.var_leffcv_acc_dn2, locals.var_leffcv_acc_dn3, locals.var_leffcv_acc_dn4, locals.var_leffcv_acc_dn5, locals.var_leffcv_acc_dn6, locals.var_leffcv_acc_dn7, locals.var_leffcv_acc_dn8, locals.var_leffcv_acc_dn9, locals.var_leffcv_acc_dn10, locals.var_leffcv_acc_dn11, locals.var_leffcv_acc_dn13, locals.var_leffcv_acc_dn14,)
    }
};
        locals.var_leffcv_acc = assign2980_e3378;
        locals.var_leffcv_acc_dn0 = assign2980_e3378_d_n0;
        locals.var_leffcv_acc_dn2 = assign2980_e3378_d_n2;
        locals.var_leffcv_acc_dn3 = assign2980_e3378_d_n3;
        locals.var_leffcv_acc_dn4 = assign2980_e3378_d_n4;
        locals.var_leffcv_acc_dn5 = assign2980_e3378_d_n5;
        locals.var_leffcv_acc_dn6 = assign2980_e3378_d_n6;
        locals.var_leffcv_acc_dn7 = assign2980_e3378_d_n7;
        locals.var_leffcv_acc_dn8 = assign2980_e3378_d_n8;
        locals.var_leffcv_acc_dn9 = assign2980_e3378_d_n9;
        locals.var_leffcv_acc_dn10 = assign2980_e3378_d_n10;
        locals.var_leffcv_acc_dn11 = assign2980_e3378_d_n11;
        locals.var_leffcv_acc_dn13 = assign2980_e3378_d_n13;
        locals.var_leffcv_acc_dn14 = assign2980_e3378_d_n14;
        locals.var_leffcv_acc_rv = 0.0;

        let assign3000_e3384: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign3000_e3384;
        locals.var_guard14_rv = 0.0;

        let (assign3010_e3420,) = {
    if (locals.var_guard14 != 0.0) {
        let assign3010_e3389: f64 = (1e-6 * p.p122);
        let assign3010_e3391: f64 = (assign3010_e3389 / p.p0);
        let assign3010_e3392: f64 = (p.p121 + assign3010_e3391);
        let assign3010_e3395: f64 = (p.p123 / p.p5);
        let assign3010_e3396: f64 = (assign3010_e3392 + assign3010_e3395);
        let assign3010_e3399: f64 = (p.p124 * 1e-6);
        let assign3010_e3402: f64 = (p.p0 * p.p5);
        let assign3010_e3403: f64 = (assign3010_e3399 / assign3010_e3402);
        let assign3010_e3404: f64 = (assign3010_e3396 + assign3010_e3403);
        let assign3010_e3407: f64 = (1e-6 * p.p125);
        let assign3010_e3409: f64 = (assign3010_e3407 / p.p43);
        let assign3010_e3410: f64 = (assign3010_e3404 + assign3010_e3409);
        let assign3010_e3413: f64 = (p.p126 * 1e-12);
        let assign3010_e3416: f64 = (p.p0 * p.p43);
        let assign3010_e3417: f64 = (assign3010_e3413 / assign3010_e3416);
        let assign3010_e3418: f64 = (assign3010_e3410 + assign3010_e3417);
        (assign3010_e3418,)
    } else {
        (locals.var_xw_i,)
    }
};
        locals.var_xw_i = assign3010_e3420;
        locals.var_xw_i_rv = 0.0;

        let (assign3020_e3456,) = {
    if (locals.var_guard14 != 0.0) {
        let assign3020_e3425: f64 = (1e-6 * p.p128);
        let assign3020_e3427: f64 = (assign3020_e3425 / p.p0);
        let assign3020_e3428: f64 = (p.p127 + assign3020_e3427);
        let assign3020_e3431: f64 = (p.p129 / p.p5);
        let assign3020_e3432: f64 = (assign3020_e3428 + assign3020_e3431);
        let assign3020_e3435: f64 = (p.p130 * 1e-6);
        let assign3020_e3438: f64 = (p.p0 * p.p5);
        let assign3020_e3439: f64 = (assign3020_e3435 / assign3020_e3438);
        let assign3020_e3440: f64 = (assign3020_e3432 + assign3020_e3439);
        let assign3020_e3443: f64 = (1e-6 * p.p131);
        let assign3020_e3445: f64 = (assign3020_e3443 / p.p43);
        let assign3020_e3446: f64 = (assign3020_e3440 + assign3020_e3445);
        let assign3020_e3449: f64 = (p.p132 * 1e-12);
        let assign3020_e3452: f64 = (p.p0 * p.p43);
        let assign3020_e3453: f64 = (assign3020_e3449 / assign3020_e3452);
        let assign3020_e3454: f64 = (assign3020_e3446 + assign3020_e3453);
        (assign3020_e3454,)
    } else {
        (locals.var_dwbin_i,)
    }
};
        locals.var_dwbin_i = assign3020_e3456;
        locals.var_dwbin_i_rv = 0.0;

        let (assign3030_e3461,) = {
    if (locals.var_guard14 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xw_i,)
    }
};
        locals.var_xw_i = assign3030_e3461;
        locals.var_xw_i_rv = 0.0;

        let (assign3040_e3466,) = {
    if (locals.var_guard14 == 0.0) {
        (0.0,)
    } else {
        (locals.var_dwbin_i,)
    }
};
        locals.var_dwbin_i = assign3040_e3466;
        locals.var_dwbin_i_rv = 0.0;

        let assign3050_e3469: f64 = (p.p43 + locals.var_xw_i);
        locals.var_wgaaeff = assign3050_e3469;
        locals.var_wgaaeff_rv = 0.0;

        let assign3060_e3472: f64 = (locals.var_wgaaeff + locals.var_dwbin_i);
        locals.var_wgaaeff1 = assign3060_e3472;
        locals.var_wgaaeff1_rv = 0.0;

        let assign3070_e3475: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign3070_e3475;
        locals.var_guard15_rv = 0.0;

        let assign3080_e3478: f64 = if locals.var_wgaaeff1 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign3080_e3478;
        locals.var_guard16_rv = 0.0;

        let (assign3090_e3484,) = {
    if ((locals.var_guard15 != 0.0) && (locals.var_guard16 != 0.0)) {
        (p.p43,)
    } else {
        (locals.var_wgaaeff1,)
    }
};
        locals.var_wgaaeff1 = assign3090_e3484;
        locals.var_wgaaeff1_rv = 0.0;

        let assign3110_e3490: f64 = (p.p5 * p.p59);
        locals.var_nfintotal = assign3110_e3490;
        locals.var_nfintotal_rv = 0.0;

        let assign3120_e3493: f64 = (1e-6 / locals.var_leff1);
        locals.var_inv_l = assign3120_e3493;
        locals.var_inv_l_rv = 0.0;

        let assign3130_e3496: f64 = (1.0 / p.p5);
        locals.var_inv_nfin = assign3130_e3496;
        locals.var_inv_nfin_rv = 0.0;

        let assign3140_e3500: f64 = (locals.var_leff1 * p.p5);
        let assign3140_e3501: f64 = (1e-6 / assign3140_e3500);
        locals.var_inv_lnfin = assign3140_e3501;
        locals.var_inv_lnfin_rv = 0.0;

        let assign3150_e3504: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign3150_e3504;
        locals.var_guard18_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3160_e3510,) = {
    if (locals.var_guard18 != 0.0) {
        let assign3160_e3508: f64 = (1e-6 / locals.var_wgaaeff1);
        (assign3160_e3508,)
    } else {
        (locals.var_inv_w,)
    }
};
        locals.var_inv_w = assign3160_e3510;
        locals.var_inv_w_rv = 0.0;

        let (assign3170_e3518,) = {
    if (locals.var_guard18 != 0.0) {
        let assign3170_e3515: f64 = (locals.var_wgaaeff1 * locals.var_leff1);
        let assign3170_e3516: f64 = (1e-12 / assign3170_e3515);
        (assign3170_e3516,)
    } else {
        (locals.var_inv_wl,)
    }
};
        locals.var_inv_wl = assign3170_e3518;
        locals.var_inv_wl_rv = 0.0;

        let (assign3180_e3523,) = {
    if (locals.var_guard18 == 0.0) {
        (0.0,)
    } else {
        (locals.var_inv_w,)
    }
};
        locals.var_inv_w = assign3180_e3523;
        locals.var_inv_w_rv = 0.0;

        let (assign3190_e3528,) = {
    if (locals.var_guard18 == 0.0) {
        (0.0,)
    } else {
        (locals.var_inv_wl,)
    }
};
        locals.var_inv_wl = assign3190_e3528;
        locals.var_inv_wl_rv = 0.0;

        let assign3200_e3532: f64 = (locals.var_inv_l * p.p134);
        let assign3200_e3533: f64 = (p.p133 + assign3200_e3532);
        let assign3200_e3536: f64 = (locals.var_inv_nfin * p.p135);
        let assign3200_e3537: f64 = (assign3200_e3533 + assign3200_e3536);
        let assign3200_e3540: f64 = (locals.var_inv_lnfin * p.p136);
        let assign3200_e3541: f64 = (assign3200_e3537 + assign3200_e3540);
        let assign3200_e3545: f64 = assign3200_e3541;
        let assign3200_e3549: f64 = assign3200_e3545;
        locals.var_nbody_i = assign3200_e3549;
        locals.var_nbody_i_rv = 0.0;

        let assign3210_e3552: f64 = if p.p95 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign3210_e3552;
        locals.var_guard19_rv = 0.0;

        let (assign3220_e3589,) = {
    if (locals.var_guard19 != 0.0) {
        let assign3220_e3558: f64 = (p.p95 / p.p5);
        let assign3220_e3562: f64 = (p.p5 / p.p96);
        let assign3220_e3563: f64 = (1.0 + assign3220_e3562);
        let (assign3220_e3584,) = {
            if (!(assign3220_e3563 > 1e-38)) {
                let assign3220_e3568: f64 = (-87.498233534);
                (assign3220_e3568,)
            } else {
                let assign3220_e3572: f64 = (p.p5 / p.p96);
                let assign3220_e3573: f64 = (1.0 + assign3220_e3572);
                let (assign3220_e3583,) = {
                    if (assign3220_e3573 > 1e-38) {
                        let assign3220_e3579: f64 = (p.p5 / p.p96);
                        let assign3220_e3580: f64 = (1.0 + assign3220_e3579);
                        let assign3220_e3581: f64 = (assign3220_e3580).ln();
                        (assign3220_e3581,)
                    } else {
                        (0.0,)
                    }
                };
                (assign3220_e3583,)
            }
        };
        let assign3220_e3585: f64 = (assign3220_e3558 * assign3220_e3584);
        let assign3220_e3586: f64 = (1.0 + assign3220_e3585);
        let assign3220_e3587: f64 = (locals.var_nbody_i * assign3220_e3586);
        (assign3220_e3587,)
    } else {
        (locals.var_nbody_i,)
    }
};
        locals.var_nbody_i = assign3220_e3589;
        locals.var_nbody_i_rv = 0.0;

        let assign3230_e3592: f64 = if locals.var_nbody_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign3230_e3592;
        locals.var_guard20_rv = 0.0;

        let (assign3240_e3596,) = {
    if (locals.var_guard20 != 0.0) {
        (1e22,)
    } else {
        (locals.var_nbody_i,)
    }
};
        locals.var_nbody_i = assign3240_e3596;
        locals.var_nbody_i_rv = 0.0;

        let assign3260_e3602: f64 = if p.p62 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign3260_e3602;
        locals.var_guard22_rv = 0.0;

        let assign3270_e3605: f64 = if p.p62 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign3270_e3605;
        locals.var_guard23_rv = 0.0;

        let assign3280_e3608: f64 = if p.p62 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign3280_e3608;
        locals.var_guard24_rv = 0.0;

        let assign3290_e3611: f64 = if p.p62 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign3290_e3611;
        locals.var_guard25_rv = 0.0;

        let assign3300_e3614: f64 = if p.p62 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3300_e3614;
        locals.var_guard26_rv = 0.0;

        let assign3310_e3617: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3310_e3617;
        locals.var_guard27_rv = 0.0;

        let assign3320_e3624: f64 = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3320_e3624;
        locals.var_guard28_rv = 0.0;

        let (assign3330_e3632,) = {
    if ((locals.var_guard22 != 0.0) && (locals.var_guard28 != 0.0)) {
        let assign3330_e3630: f64 = (2.0 * p.p92);
        (assign3330_e3630,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3330_e3632;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3340_e3644,) = {
    if ((locals.var_guard22 != 0.0) && (locals.var_guard28 != 0.0)) {
        let assign3340_e3638: f64 = (locals.var_weff_ufcm * p.p102);
        let assign3340_e3640: f64 = (assign3340_e3638 * 8.8542e-12);
        let assign3340_e3642: f64 = (assign3340_e3640 / p.p89);
        (assign3340_e3642,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3340_e3644;
        locals.var_cins_rv = 0.0;

        let (assign3350_e3652,) = {
    if ((locals.var_guard22 != 0.0) && (locals.var_guard28 != 0.0)) {
        let assign3350_e3650: f64 = (p.p92 * p.p3);
        (assign3350_e3650,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3350_e3652;
        locals.var_ach_rv = 0.0;

        let (assign3360_e3674,) = {
    if ((locals.var_guard22 != 0.0) && (locals.var_guard28 == 0.0)) {
        let assign3360_e3660: f64 = (p.p92 * p.p92);
        let assign3360_e3663: f64 = (p.p1802 - p.p1803);
        let assign3360_e3666: f64 = (p.p1802 - p.p1803);
        let assign3360_e3667: f64 = (assign3360_e3663 * assign3360_e3666);
        let assign3360_e3669: f64 = (assign3360_e3667 / 4.0);
        let assign3360_e3670: f64 = (assign3360_e3660 + assign3360_e3669);
        let assign3360_e3671: f64 = (assign3360_e3670).sqrt();
        let assign3360_e3672: f64 = (2.0 * assign3360_e3671);
        (assign3360_e3672,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3360_e3674;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3370_e3687,) = {
    if ((locals.var_guard22 != 0.0) && (locals.var_guard28 == 0.0)) {
        let assign3370_e3681: f64 = (locals.var_weff_ufcm * p.p102);
        let assign3370_e3683: f64 = (assign3370_e3681 * 8.8542e-12);
        let assign3370_e3685: f64 = (assign3370_e3683 / p.p89);
        (assign3370_e3685,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3370_e3687;
        locals.var_cins_rv = 0.0;

        let (assign3380_e3700,) = {
    if ((locals.var_guard22 != 0.0) && (locals.var_guard28 == 0.0)) {
        let assign3380_e3695: f64 = (p.p1802 + p.p1803);
        let assign3380_e3696: f64 = (p.p92 * assign3380_e3695);
        let assign3380_e3698: f64 = (assign3380_e3696 / 2.0);
        (assign3380_e3698,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3380_e3700;
        locals.var_ach_rv = 0.0;

        let assign3390_e3707: f64 = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3390_e3707;
        locals.var_guard29_rv = 0.0;

        let (assign3400_e3720,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard22 == 0.0)) && (locals.var_guard29 != 0.0)) {
        let assign3400_e3716: f64 = (2.0 * p.p92);
        let assign3400_e3718: f64 = (assign3400_e3716 + p.p3);
        (assign3400_e3718,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3400_e3720;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3410_e3735,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard22 == 0.0)) && (locals.var_guard29 != 0.0)) {
        let assign3410_e3729: f64 = (locals.var_weff_ufcm * p.p102);
        let assign3410_e3731: f64 = (assign3410_e3729 * 8.8542e-12);
        let assign3410_e3733: f64 = (assign3410_e3731 / p.p89);
        (assign3410_e3733,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3410_e3735;
        locals.var_cins_rv = 0.0;

        let (assign3420_e3746,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard22 == 0.0)) && (locals.var_guard29 != 0.0)) {
        let assign3420_e3744: f64 = (p.p92 * p.p3);
        (assign3420_e3744,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3420_e3746;
        locals.var_ach_rv = 0.0;

        let (assign3430_e3773,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard22 == 0.0)) && (locals.var_guard29 == 0.0)) {
        let assign3430_e3757: f64 = (p.p92 * p.p92);
        let assign3430_e3760: f64 = (p.p1802 - p.p1803);
        let assign3430_e3763: f64 = (p.p1802 - p.p1803);
        let assign3430_e3764: f64 = (assign3430_e3760 * assign3430_e3763);
        let assign3430_e3766: f64 = (assign3430_e3764 / 4.0);
        let assign3430_e3767: f64 = (assign3430_e3757 + assign3430_e3766);
        let assign3430_e3768: f64 = (assign3430_e3767).sqrt();
        let assign3430_e3769: f64 = (2.0 * assign3430_e3768);
        let assign3430_e3771: f64 = (assign3430_e3769 + p.p1802);
        (assign3430_e3771,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3430_e3773;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3440_e3789,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard22 == 0.0)) && (locals.var_guard29 == 0.0)) {
        let assign3440_e3783: f64 = (locals.var_weff_ufcm * p.p102);
        let assign3440_e3785: f64 = (assign3440_e3783 * 8.8542e-12);
        let assign3440_e3787: f64 = (assign3440_e3785 / p.p89);
        (assign3440_e3787,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3440_e3789;
        locals.var_cins_rv = 0.0;

        let (assign3450_e3805,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard22 == 0.0)) && (locals.var_guard29 == 0.0)) {
        let assign3450_e3800: f64 = (p.p1802 + p.p1803);
        let assign3450_e3801: f64 = (p.p92 * assign3450_e3800);
        let assign3450_e3803: f64 = (assign3450_e3801 / 2.0);
        (assign3450_e3803,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3450_e3805;
        locals.var_ach_rv = 0.0;

        let assign3460_e3812: f64 = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3460_e3812;
        locals.var_guard30_rv = 0.0;

        let (assign3470_e3829,) = {
    if (((locals.var_guard24 != 0.0) && (!((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)))) && (locals.var_guard30 != 0.0)) {
        let assign3470_e3823: f64 = (2.0 * p.p92);
        let assign3470_e3826: f64 = (2.0 * p.p3);
        let assign3470_e3827: f64 = (assign3470_e3823 + assign3470_e3826);
        (assign3470_e3827,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3470_e3829;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3480_e3846,) = {
    if (((locals.var_guard24 != 0.0) && (!((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)))) && (locals.var_guard30 != 0.0)) {
        let assign3480_e3840: f64 = (locals.var_weff_ufcm * p.p102);
        let assign3480_e3842: f64 = (assign3480_e3840 * 8.8542e-12);
        let assign3480_e3844: f64 = (assign3480_e3842 / p.p89);
        (assign3480_e3844,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3480_e3846;
        locals.var_cins_rv = 0.0;

        let (assign3490_e3859,) = {
    if (((locals.var_guard24 != 0.0) && (!((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)))) && (locals.var_guard30 != 0.0)) {
        let assign3490_e3857: f64 = (p.p92 * p.p3);
        (assign3490_e3857,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3490_e3859;
        locals.var_ach_rv = 0.0;

        let (assign3500_e3890,) = {
    if (((locals.var_guard24 != 0.0) && (!((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)))) && (locals.var_guard30 == 0.0)) {
        let assign3500_e3872: f64 = (p.p92 * p.p92);
        let assign3500_e3875: f64 = (p.p1802 - p.p1803);
        let assign3500_e3878: f64 = (p.p1802 - p.p1803);
        let assign3500_e3879: f64 = (assign3500_e3875 * assign3500_e3878);
        let assign3500_e3881: f64 = (assign3500_e3879 / 4.0);
        let assign3500_e3882: f64 = (assign3500_e3872 + assign3500_e3881);
        let assign3500_e3883: f64 = (assign3500_e3882).sqrt();
        let assign3500_e3884: f64 = (2.0 * assign3500_e3883);
        let assign3500_e3886: f64 = (assign3500_e3884 + p.p1802);
        let assign3500_e3888: f64 = (assign3500_e3886 + p.p1803);
        (assign3500_e3888,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3500_e3890;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3510_e3908,) = {
    if (((locals.var_guard24 != 0.0) && (!((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)))) && (locals.var_guard30 == 0.0)) {
        let assign3510_e3902: f64 = (locals.var_weff_ufcm * p.p102);
        let assign3510_e3904: f64 = (assign3510_e3902 * 8.8542e-12);
        let assign3510_e3906: f64 = (assign3510_e3904 / p.p89);
        (assign3510_e3906,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3510_e3908;
        locals.var_cins_rv = 0.0;

        let (assign3520_e3926,) = {
    if (((locals.var_guard24 != 0.0) && (!((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)))) && (locals.var_guard30 == 0.0)) {
        let assign3520_e3921: f64 = (p.p1802 + p.p1803);
        let assign3520_e3922: f64 = (p.p92 * assign3520_e3921);
        let assign3520_e3924: f64 = (assign3520_e3922 / 2.0);
        (assign3520_e3924,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3520_e3926;
        locals.var_ach_rv = 0.0;

        let (assign3530_e3935,) = {
    if ((locals.var_guard24 != 0.0) && (!((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)))) {
        (p.p1803,)
    } else {
        (locals.var_weffb,)
    }
};
        locals.var_weffb = assign3530_e3935;
        locals.var_weffb_rv = 0.0;

        let (assign3540_e3948,) = {
    if ((locals.var_guard25 != 0.0) && (!(((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)))) {
        let assign3540_e3946: f64 = (3.141592653589793 * p.p2);
        (assign3540_e3946,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3540_e3948;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3550_e3998,) = {
    if ((locals.var_guard25 != 0.0) && (!(((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)))) {
        let assign3550_e3959: f64 = (2.0 * 3.141592653589793);
        let assign3550_e3961: f64 = (assign3550_e3959 * p.p102);
        let assign3550_e3963: f64 = (assign3550_e3961 * 8.8542e-12);
        let assign3550_e3967: f64 = (2.0 * p.p89);
        let assign3550_e3969: f64 = (assign3550_e3967 / p.p2);
        let assign3550_e3970: f64 = (1.0 + assign3550_e3969);
        let (assign3550_e3995,) = {
            if (!(assign3550_e3970 > 1e-38)) {
                let assign3550_e3975: f64 = (-87.498233534);
                (assign3550_e3975,)
            } else {
                let assign3550_e3979: f64 = (2.0 * p.p89);
                let assign3550_e3981: f64 = (assign3550_e3979 / p.p2);
                let assign3550_e3982: f64 = (1.0 + assign3550_e3981);
                let (assign3550_e3994,) = {
                    if (assign3550_e3982 > 1e-38) {
                        let assign3550_e3988: f64 = (2.0 * p.p89);
                        let assign3550_e3990: f64 = (assign3550_e3988 / p.p2);
                        let assign3550_e3991: f64 = (1.0 + assign3550_e3990);
                        let assign3550_e3992: f64 = (assign3550_e3991).ln();
                        (assign3550_e3992,)
                    } else {
                        (0.0,)
                    }
                };
                (assign3550_e3994,)
            }
        };
        let assign3550_e3996: f64 = (assign3550_e3963 / assign3550_e3995);
        (assign3550_e3996,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3550_e3998;
        locals.var_cins_rv = 0.0;

        let (assign3560_e4015,) = {
    if ((locals.var_guard25 != 0.0) && (!(((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)))) {
        let assign3560_e4009: f64 = (3.141592653589793 * p.p2);
        let assign3560_e4011: f64 = (assign3560_e4009 * p.p2);
        let assign3560_e4013: f64 = (assign3560_e4011 / 4.0);
        (assign3560_e4013,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3560_e4015;
        locals.var_ach_rv = 0.0;

        let (assign3570_e4026,) = {
    if ((locals.var_guard25 != 0.0) && (!(((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)))) {
        (p.p2,)
    } else {
        (locals.var_weffb,)
    }
};
        locals.var_weffb = assign3570_e4026;
        locals.var_weffb_rv = 0.0;

        let (assign3580_e4039,) = {
    if ((locals.var_guard26 != 0.0) && (!((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)))) {
        (p.p1801,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3580_e4039;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3590_e4052,) = {
    if ((locals.var_guard26 != 0.0) && (!((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)))) {
        (p.p1800,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3590_e4052;
        locals.var_cins_rv = 0.0;

        let (assign3600_e4065,) = {
    if ((locals.var_guard26 != 0.0) && (!((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)))) {
        (p.p1799,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3600_e4065;
        locals.var_ach_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3610_e4086,) = {
    if ((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) {
        let assign3610_e4081: f64 = (locals.var_wgaaeff + p.p40);
        let assign3610_e4082: f64 = (2.0 * assign3610_e4081);
        let assign3610_e4084: f64 = (assign3610_e4082 + p.p44);
        (assign3610_e4084,)
    } else {
        (locals.var_weff1,)
    }
};
        locals.var_weff1 = assign3610_e4086;
        locals.var_weff1_rv = 0.0;

        let (assign3620_e4105,) = {
    if ((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) {
        let assign3620_e4101: f64 = (locals.var_wgaaeff * p.p40);
        let assign3620_e4103: f64 = (assign3620_e4101 + p.p45);
        (assign3620_e4103,)
    } else {
        (locals.var_ach1,)
    }
};
        locals.var_ach1 = assign3620_e4105;
        locals.var_ach1_rv = 0.0;

        let (assign3630_e4120,) = {
    if ((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) {
        (locals.var_weff1,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3630_e4120;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3640_e4135,) = {
    if ((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) {
        (locals.var_ach1,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3640_e4135;
        locals.var_ach_rv = 0.0;

        let assign3650_e4138: f64 = if p.p56 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3650_e4138;
        locals.var_guard31_rv = 0.0;

        let (assign3660_e4161,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard31 != 0.0)) {
        let assign3660_e4156: f64 = (locals.var_wgaaeff + p.p40);
        let assign3660_e4157: f64 = (2.0 * assign3660_e4156);
        let assign3660_e4159: f64 = (assign3660_e4157 + p.p46);
        (assign3660_e4159,)
    } else {
        (locals.var_weff2,)
    }
};
        locals.var_weff2 = assign3660_e4161;
        locals.var_weff2_rv = 0.0;

        let (assign3670_e4182,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard31 != 0.0)) {
        let assign3670_e4178: f64 = (locals.var_wgaaeff * p.p40);
        let assign3670_e4180: f64 = (assign3670_e4178 + p.p47);
        (assign3670_e4180,)
    } else {
        (locals.var_ach2,)
    }
};
        locals.var_ach2 = assign3670_e4182;
        locals.var_ach2_rv = 0.0;

        let (assign3680_e4201,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard31 != 0.0)) {
        let assign3680_e4199: f64 = (locals.var_weff1 + locals.var_weff2);
        (assign3680_e4199,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3680_e4201;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3690_e4220,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard31 != 0.0)) {
        let assign3690_e4218: f64 = (locals.var_ach1 + locals.var_ach2);
        (assign3690_e4218,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3690_e4220;
        locals.var_ach_rv = 0.0;

        let assign3700_e4223: f64 = if p.p56 > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3700_e4223;
        locals.var_guard32_rv = 0.0;

        let (assign3710_e4246,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard32 != 0.0)) {
        let assign3710_e4241: f64 = (locals.var_wgaaeff + p.p40);
        let assign3710_e4242: f64 = (2.0 * assign3710_e4241);
        let assign3710_e4244: f64 = (assign3710_e4242 + p.p48);
        (assign3710_e4244,)
    } else {
        (locals.var_weff3,)
    }
};
        locals.var_weff3 = assign3710_e4246;
        locals.var_weff3_rv = 0.0;

        let (assign3720_e4267,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard32 != 0.0)) {
        let assign3720_e4263: f64 = (locals.var_wgaaeff * p.p40);
        let assign3720_e4265: f64 = (assign3720_e4263 + p.p49);
        (assign3720_e4265,)
    } else {
        (locals.var_ach3,)
    }
};
        locals.var_ach3 = assign3720_e4267;
        locals.var_ach3_rv = 0.0;

        let (assign3730_e4288,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard32 != 0.0)) {
        let assign3730_e4284: f64 = (locals.var_weff1 + locals.var_weff2);
        let assign3730_e4286: f64 = (assign3730_e4284 + locals.var_weff3);
        (assign3730_e4286,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3730_e4288;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3740_e4309,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard32 != 0.0)) {
        let assign3740_e4305: f64 = (locals.var_ach1 + locals.var_ach2);
        let assign3740_e4307: f64 = (assign3740_e4305 + locals.var_ach3);
        (assign3740_e4307,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3740_e4309;
        locals.var_ach_rv = 0.0;

        let assign3750_e4312: f64 = if p.p56 > 3.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3750_e4312;
        locals.var_guard33_rv = 0.0;

        let (assign3760_e4335,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard33 != 0.0)) {
        let assign3760_e4330: f64 = (locals.var_wgaaeff + p.p40);
        let assign3760_e4331: f64 = (2.0 * assign3760_e4330);
        let assign3760_e4333: f64 = (assign3760_e4331 + p.p50);
        (assign3760_e4333,)
    } else {
        (locals.var_weff4,)
    }
};
        locals.var_weff4 = assign3760_e4335;
        locals.var_weff4_rv = 0.0;

        let (assign3770_e4356,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard33 != 0.0)) {
        let assign3770_e4352: f64 = (locals.var_wgaaeff * p.p40);
        let assign3770_e4354: f64 = (assign3770_e4352 + p.p51);
        (assign3770_e4354,)
    } else {
        (locals.var_ach4,)
    }
};
        locals.var_ach4 = assign3770_e4356;
        locals.var_ach4_rv = 0.0;

        let (assign3780_e4379,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard33 != 0.0)) {
        let assign3780_e4373: f64 = (locals.var_weff1 + locals.var_weff2);
        let assign3780_e4375: f64 = (assign3780_e4373 + locals.var_weff3);
        let assign3780_e4377: f64 = (assign3780_e4375 + locals.var_weff4);
        (assign3780_e4377,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3780_e4379;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3790_e4402,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard33 != 0.0)) {
        let assign3790_e4396: f64 = (locals.var_ach1 + locals.var_ach2);
        let assign3790_e4398: f64 = (assign3790_e4396 + locals.var_ach3);
        let assign3790_e4400: f64 = (assign3790_e4398 + locals.var_ach4);
        (assign3790_e4400,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3790_e4402;
        locals.var_ach_rv = 0.0;

        let assign3800_e4405: f64 = if p.p56 > 4.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3800_e4405;
        locals.var_guard34_rv = 0.0;

        let (assign3810_e4428,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard34 != 0.0)) {
        let assign3810_e4423: f64 = (locals.var_wgaaeff + p.p40);
        let assign3810_e4424: f64 = (2.0 * assign3810_e4423);
        let assign3810_e4426: f64 = (assign3810_e4424 + p.p52);
        (assign3810_e4426,)
    } else {
        (locals.var_weff5,)
    }
};
        locals.var_weff5 = assign3810_e4428;
        locals.var_weff5_rv = 0.0;

        let (assign3820_e4449,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard34 != 0.0)) {
        let assign3820_e4445: f64 = (locals.var_wgaaeff * p.p40);
        let assign3820_e4447: f64 = (assign3820_e4445 + p.p53);
        (assign3820_e4447,)
    } else {
        (locals.var_ach5,)
    }
};
        locals.var_ach5 = assign3820_e4449;
        locals.var_ach5_rv = 0.0;

        let (assign3830_e4474,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard34 != 0.0)) {
        let assign3830_e4466: f64 = (locals.var_weff1 + locals.var_weff2);
        let assign3830_e4468: f64 = (assign3830_e4466 + locals.var_weff3);
        let assign3830_e4470: f64 = (assign3830_e4468 + locals.var_weff4);
        let assign3830_e4472: f64 = (assign3830_e4470 + locals.var_weff5);
        (assign3830_e4472,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3830_e4474;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3840_e4499,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard34 != 0.0)) {
        let assign3840_e4491: f64 = (locals.var_ach1 + locals.var_ach2);
        let assign3840_e4493: f64 = (assign3840_e4491 + locals.var_ach3);
        let assign3840_e4495: f64 = (assign3840_e4493 + locals.var_ach4);
        let assign3840_e4497: f64 = (assign3840_e4495 + locals.var_ach5);
        (assign3840_e4497,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3840_e4499;
        locals.var_ach_rv = 0.0;

        let assign3850_e4502: f64 = if p.p56 > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign3850_e4502;
        locals.var_guard35_rv = 0.0;

        let (assign3860_e4525,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard35 != 0.0)) {
        let assign3860_e4520: f64 = (locals.var_wgaaeff + p.p40);
        let assign3860_e4521: f64 = (2.0 * assign3860_e4520);
        let assign3860_e4523: f64 = (assign3860_e4521 + p.p54);
        (assign3860_e4523,)
    } else {
        (locals.var_weff6,)
    }
};
        locals.var_weff6 = assign3860_e4525;
        locals.var_weff6_rv = 0.0;

        let (assign3870_e4546,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard35 != 0.0)) {
        let assign3870_e4542: f64 = (locals.var_wgaaeff * p.p40);
        let assign3870_e4544: f64 = (assign3870_e4542 + p.p55);
        (assign3870_e4544,)
    } else {
        (locals.var_ach6,)
    }
};
        locals.var_ach6 = assign3870_e4546;
        locals.var_ach6_rv = 0.0;

        let (assign3880_e4573,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard35 != 0.0)) {
        let assign3880_e4563: f64 = (locals.var_weff1 + locals.var_weff2);
        let assign3880_e4565: f64 = (assign3880_e4563 + locals.var_weff3);
        let assign3880_e4567: f64 = (assign3880_e4565 + locals.var_weff4);
        let assign3880_e4569: f64 = (assign3880_e4567 + locals.var_weff5);
        let assign3880_e4571: f64 = (assign3880_e4569 + locals.var_weff6);
        (assign3880_e4571,)
    } else {
        (locals.var_weff_ufcm,)
    }
};
        locals.var_weff_ufcm = assign3880_e4573;
        locals.var_weff_ufcm_rv = 0.0;

        let (assign3890_e4600,) = {
    if (((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) && (locals.var_guard35 != 0.0)) {
        let assign3890_e4590: f64 = (locals.var_ach1 + locals.var_ach2);
        let assign3890_e4592: f64 = (assign3890_e4590 + locals.var_ach3);
        let assign3890_e4594: f64 = (assign3890_e4592 + locals.var_ach4);
        let assign3890_e4596: f64 = (assign3890_e4594 + locals.var_ach5);
        let assign3890_e4598: f64 = (assign3890_e4596 + locals.var_ach6);
        (assign3890_e4598,)
    } else {
        (locals.var_ach,)
    }
};
        locals.var_ach = assign3890_e4600;
        locals.var_ach_rv = 0.0;

        let (assign3900_e4615,) = {
    if ((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) {
        (p.p43,)
    } else {
        (locals.var_weffb,)
    }
};
        locals.var_weffb = assign3900_e4615;
        locals.var_weffb_rv = 0.0;

        let (assign3910_e4636,) = {
    if ((locals.var_guard27 != 0.0) && (!(((((locals.var_guard22 != 0.0) || (locals.var_guard23 != 0.0)) || (locals.var_guard24 != 0.0)) || (locals.var_guard25 != 0.0)) || (locals.var_guard26 != 0.0)))) {
        let assign3910_e4630: f64 = (locals.var_weff_ufcm * p.p102);
        let assign3910_e4632: f64 = (assign3910_e4630 * 8.8542e-12);
        let assign3910_e4634: f64 = (assign3910_e4632 / p.p89);
        (assign3910_e4634,)
    } else {
        (locals.var_cins,)
    }
};
        locals.var_cins = assign3910_e4636;
        locals.var_cins_rv = 0.0;

        let assign3920_e4639: f64 = (2.0 * locals.var_cins);
        let assign3920_e4642: f64 = (locals.var_weff_ufcm * locals.var_weff_ufcm);
        let assign3920_e4644: f64 = (assign3920_e4642 * locals.var_epssub);
        let assign3920_e4646: f64 = (assign3920_e4644 / locals.var_ach);
        let assign3920_e4647: f64 = (assign3920_e4639 / assign3920_e4646);
        locals.var_rc = assign3920_e4647;
        locals.var_rc_rv = 0.0;

        let assign3930_e4649: f64 = (-1.60219e-19);
        let assign3930_e4651: f64 = (assign3930_e4649 * locals.var_nbody_i);
        let assign3930_e4653: f64 = (assign3930_e4651 * locals.var_ach);
        let assign3930_e4655: f64 = (assign3930_e4653 / locals.var_cins);
        locals.var_qdep_ov_cins = assign3930_e4655;
        locals.var_qdep_ov_cins_rv = 0.0;

        let assign3940_e4658: f64 = (locals.var_cins / locals.var_weff_ufcm);
        locals.var_cox = assign3940_e4658;
        locals.var_cox_rv = 0.0;

        let assign3950_e4661: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign3950_e4661;
        locals.var_guard36_rv = 0.0;

        let (assign3960_e4669, assign3960_e4669_d_n0, assign3960_e4669_d_n2, assign3960_e4669_d_n3, assign3960_e4669_d_n4, assign3960_e4669_d_n5, assign3960_e4669_d_n6, assign3960_e4669_d_n7, assign3960_e4669_d_n8, assign3960_e4669_d_n9, assign3960_e4669_d_n10, assign3960_e4669_d_n11, assign3960_e4669_d_n13, assign3960_e4669_d_n14,) = {
    if (locals.var_guard36 != 0.0) {
        let assign3960_e4665: f64 = (locals.var_cox * p.p89);
        let assign3960_e4667: f64 = (assign3960_e4665 / p.p1528);
        (assign3960_e4667, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox_acc, locals.var_cox_acc_dn0, locals.var_cox_acc_dn2, locals.var_cox_acc_dn3, locals.var_cox_acc_dn4, locals.var_cox_acc_dn5, locals.var_cox_acc_dn6, locals.var_cox_acc_dn7, locals.var_cox_acc_dn8, locals.var_cox_acc_dn9, locals.var_cox_acc_dn10, locals.var_cox_acc_dn11, locals.var_cox_acc_dn13, locals.var_cox_acc_dn14,)
    }
};
        locals.var_cox_acc = assign3960_e4669;
        locals.var_cox_acc_dn0 = assign3960_e4669_d_n0;
        locals.var_cox_acc_dn2 = assign3960_e4669_d_n2;
        locals.var_cox_acc_dn3 = assign3960_e4669_d_n3;
        locals.var_cox_acc_dn4 = assign3960_e4669_d_n4;
        locals.var_cox_acc_dn5 = assign3960_e4669_d_n5;
        locals.var_cox_acc_dn6 = assign3960_e4669_d_n6;
        locals.var_cox_acc_dn7 = assign3960_e4669_d_n7;
        locals.var_cox_acc_dn8 = assign3960_e4669_d_n8;
        locals.var_cox_acc_dn9 = assign3960_e4669_d_n9;
        locals.var_cox_acc_dn10 = assign3960_e4669_d_n10;
        locals.var_cox_acc_dn11 = assign3960_e4669_d_n11;
        locals.var_cox_acc_dn13 = assign3960_e4669_d_n13;
        locals.var_cox_acc_dn14 = assign3960_e4669_d_n14;
        locals.var_cox_acc_rv = 0.0;

        let assign3970_e4672: f64 = (locals.var_weff_ufcm - p.p93);
        locals.var_weff0 = assign3970_e4672;
        locals.var_weff0_rv = 0.0;

        let assign3980_e4675: f64 = (locals.var_weff_ufcm - p.p94);
        locals.var_weffcv0 = assign3980_e4675;
        locals.var_weffcv0_rv = 0.0;

        let assign3990_e4678: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign3990_e4678;
        locals.var_guard37_rv = 0.0;

        let (assign4000_e4688,) = {
    if (locals.var_guard37 != 0.0) {
        let assign4000_e4683: f64 = (2.0 * p.p56);
        let assign4000_e4685: f64 = (assign4000_e4683 * p.p87);
        let assign4000_e4686: f64 = (locals.var_weff0 - assign4000_e4685);
        (assign4000_e4686,)
    } else {
        (locals.var_weffcv_acc,)
    }
};
        locals.var_weffcv_acc = assign4000_e4688;
        locals.var_weffcv_acc_rv = 0.0;

        let (assign4010_e4693,) = {
    if (locals.var_guard37 == 0.0) {
        (locals.var_weff0,)
    } else {
        (locals.var_weffcv_acc,)
    }
};
        locals.var_weffcv_acc = assign4010_e4693;
        locals.var_weffcv_acc_rv = 0.0;

        let assign4020_e4696: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign4020_e4696;
        locals.var_guard38_rv = 0.0;

        let assign4030_e4699: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign4030_e4699;
        locals.var_guard39_rv = 0.0;

        let assign4040_e4702: f64 = if locals.var_weffcv_acc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4040_e4702;
        locals.var_guard40_rv = 0.0;

        let (assign4050_e4710,) = {
    if (((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) && (locals.var_guard40 != 0.0)) {
        (locals.var_weff_ufcm,)
    } else {
        (locals.var_weffcv_acc,)
    }
};
        locals.var_weffcv_acc = assign4050_e4710;
        locals.var_weffcv_acc_rv = 0.0;

        locals.var_deltaprsd_v = p.p1085;
        locals.var_deltaprsd_v_rv = 0.0;

        let assign4090_e4719: f64 = (locals.var_inv_l * p.p138);
        let assign4090_e4720: f64 = (p.p137 + assign4090_e4719);
        let assign4090_e4723: f64 = (locals.var_inv_nfin * p.p139);
        let assign4090_e4724: f64 = (assign4090_e4720 + assign4090_e4723);
        let assign4090_e4727: f64 = (locals.var_inv_lnfin * p.p140);
        let assign4090_e4728: f64 = (assign4090_e4724 + assign4090_e4727);
        let assign4090_e4731: f64 = (locals.var_inv_w * p.p141);
        let assign4090_e4732: f64 = (assign4090_e4728 + assign4090_e4731);
        let assign4090_e4735: f64 = (locals.var_inv_wl * p.p142);
        let assign4090_e4736: f64 = (assign4090_e4732 + assign4090_e4735);
        locals.var_phig_i = assign4090_e4736;
        locals.var_phig_i_dn0 = 0.0;
        locals.var_phig_i_dn2 = 0.0;
        locals.var_phig_i_dn3 = 0.0;
        locals.var_phig_i_dn4 = 0.0;
        locals.var_phig_i_dn5 = 0.0;
        locals.var_phig_i_dn6 = 0.0;
        locals.var_phig_i_dn7 = 0.0;
        locals.var_phig_i_dn8 = 0.0;
        locals.var_phig_i_dn9 = 0.0;
        locals.var_phig_i_dn10 = 0.0;
        locals.var_phig_i_dn11 = 0.0;
        locals.var_phig_i_dn13 = 0.0;
        locals.var_phig_i_dn14 = 0.0;
        locals.var_phig_i_rv = 0.0;

        let assign4110_e4761: f64 = (locals.var_inv_l * p.p189);
        let assign4110_e4762: f64 = (p.p188 + assign4110_e4761);
        let assign4110_e4765: f64 = (locals.var_inv_nfin * p.p190);
        let assign4110_e4766: f64 = (assign4110_e4762 + assign4110_e4765);
        let assign4110_e4769: f64 = (locals.var_inv_lnfin * p.p191);
        let assign4110_e4770: f64 = (assign4110_e4766 + assign4110_e4769);
        let assign4110_e4773: f64 = (locals.var_inv_w * p.p192);
        let assign4110_e4774: f64 = (assign4110_e4770 + assign4110_e4773);
        let assign4110_e4777: f64 = (locals.var_inv_wl * p.p193);
        let assign4110_e4778: f64 = (assign4110_e4774 + assign4110_e4777);
        locals.var_cit_i = assign4110_e4778;
        locals.var_cit_i_rv = 0.0;

        let assign4120_e4782: f64 = (locals.var_inv_l * p.p201);
        let assign4120_e4783: f64 = (p.p200 + assign4120_e4782);
        let assign4120_e4786: f64 = (locals.var_inv_nfin * p.p202);
        let assign4120_e4787: f64 = (assign4120_e4783 + assign4120_e4786);
        let assign4120_e4790: f64 = (locals.var_inv_lnfin * p.p203);
        let assign4120_e4791: f64 = (assign4120_e4787 + assign4120_e4790);
        let assign4120_e4794: f64 = (locals.var_inv_w * p.p204);
        let assign4120_e4795: f64 = (assign4120_e4791 + assign4120_e4794);
        let assign4120_e4798: f64 = (locals.var_inv_wl * p.p205);
        let assign4120_e4799: f64 = (assign4120_e4795 + assign4120_e4798);
        locals.var_cdsc_i = assign4120_e4799;
        locals.var_cdsc_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4130_e4803: f64 = (locals.var_inv_l * p.p207);
        let assign4130_e4804: f64 = (p.p206 + assign4130_e4803);
        let assign4130_e4807: f64 = (locals.var_inv_nfin * p.p208);
        let assign4130_e4808: f64 = (assign4130_e4804 + assign4130_e4807);
        let assign4130_e4811: f64 = (locals.var_inv_lnfin * p.p209);
        let assign4130_e4812: f64 = (assign4130_e4808 + assign4130_e4811);
        let assign4130_e4815: f64 = (locals.var_inv_w * p.p210);
        let assign4130_e4816: f64 = (assign4130_e4812 + assign4130_e4815);
        let assign4130_e4819: f64 = (locals.var_inv_wl * p.p211);
        let assign4130_e4820: f64 = (assign4130_e4816 + assign4130_e4819);
        locals.var_cdscd_i = assign4130_e4820;
        locals.var_cdscd_i_rv = 0.0;

        let assign4140_e4824: f64 = (locals.var_inv_l * p.p219);
        let assign4140_e4825: f64 = (p.p218 + assign4140_e4824);
        let assign4140_e4828: f64 = (locals.var_inv_nfin * p.p220);
        let assign4140_e4829: f64 = (assign4140_e4825 + assign4140_e4828);
        let assign4140_e4832: f64 = (locals.var_inv_lnfin * p.p221);
        let assign4140_e4833: f64 = (assign4140_e4829 + assign4140_e4832);
        let assign4140_e4836: f64 = (locals.var_inv_w * p.p222);
        let assign4140_e4837: f64 = (assign4140_e4833 + assign4140_e4836);
        let assign4140_e4840: f64 = (locals.var_inv_wl * p.p223);
        let assign4140_e4841: f64 = (assign4140_e4837 + assign4140_e4840);
        locals.var_dvt0_i = assign4140_e4841;
        locals.var_dvt0_i_rv = 0.0;

        let assign4150_e4845: f64 = (locals.var_inv_l * p.p225);
        let assign4150_e4846: f64 = (p.p224 + assign4150_e4845);
        let assign4150_e4849: f64 = (locals.var_inv_nfin * p.p226);
        let assign4150_e4850: f64 = (assign4150_e4846 + assign4150_e4849);
        let assign4150_e4853: f64 = (locals.var_inv_lnfin * p.p227);
        let assign4150_e4854: f64 = (assign4150_e4850 + assign4150_e4853);
        let assign4150_e4857: f64 = (locals.var_inv_w * p.p228);
        let assign4150_e4858: f64 = (assign4150_e4854 + assign4150_e4857);
        let assign4150_e4861: f64 = (locals.var_inv_wl * p.p229);
        let assign4150_e4862: f64 = (assign4150_e4858 + assign4150_e4861);
        locals.var_dvt1_i = assign4150_e4862;
        locals.var_dvt1_i_rv = 0.0;

        let assign4160_e4866: f64 = (locals.var_inv_l * p.p231);
        let assign4160_e4867: f64 = (p.p230 + assign4160_e4866);
        let assign4160_e4870: f64 = (locals.var_inv_nfin * p.p232);
        let assign4160_e4871: f64 = (assign4160_e4867 + assign4160_e4870);
        let assign4160_e4874: f64 = (locals.var_inv_lnfin * p.p233);
        let assign4160_e4875: f64 = (assign4160_e4871 + assign4160_e4874);
        let assign4160_e4878: f64 = (locals.var_inv_w * p.p234);
        let assign4160_e4879: f64 = (assign4160_e4875 + assign4160_e4878);
        let assign4160_e4882: f64 = (locals.var_inv_wl * p.p235);
        let assign4160_e4883: f64 = (assign4160_e4879 + assign4160_e4882);
        locals.var_dvt1ss_i = assign4160_e4883;
        locals.var_dvt1ss_i_rv = 0.0;

        let assign4170_e4887: f64 = (locals.var_inv_l * p.p237);
        let assign4170_e4888: f64 = (p.p236 + assign4170_e4887);
        let assign4170_e4891: f64 = (locals.var_inv_nfin * p.p238);
        let assign4170_e4892: f64 = (assign4170_e4888 + assign4170_e4891);
        let assign4170_e4895: f64 = (locals.var_inv_lnfin * p.p239);
        let assign4170_e4896: f64 = (assign4170_e4892 + assign4170_e4895);
        let assign4170_e4899: f64 = (locals.var_inv_w * p.p240);
        let assign4170_e4900: f64 = (assign4170_e4896 + assign4170_e4899);
        let assign4170_e4903: f64 = (locals.var_inv_wl * p.p241);
        let assign4170_e4904: f64 = (assign4170_e4900 + assign4170_e4903);
        locals.var_phin_i = assign4170_e4904;
        locals.var_phin_i_rv = 0.0;

        let assign4180_e4908: f64 = (locals.var_inv_l * p.p243);
        let assign4180_e4909: f64 = (p.p242 + assign4180_e4908);
        let assign4180_e4912: f64 = (locals.var_inv_nfin * p.p244);
        let assign4180_e4913: f64 = (assign4180_e4909 + assign4180_e4912);
        let assign4180_e4916: f64 = (locals.var_inv_lnfin * p.p245);
        let assign4180_e4917: f64 = (assign4180_e4913 + assign4180_e4916);
        let assign4180_e4920: f64 = (locals.var_inv_w * p.p246);
        let assign4180_e4921: f64 = (assign4180_e4917 + assign4180_e4920);
        let assign4180_e4924: f64 = (locals.var_inv_wl * p.p247);
        let assign4180_e4925: f64 = (assign4180_e4921 + assign4180_e4924);
        locals.var_eta0_i = assign4180_e4925;
        locals.var_eta0_i_dn0 = 0.0;
        locals.var_eta0_i_dn2 = 0.0;
        locals.var_eta0_i_dn3 = 0.0;
        locals.var_eta0_i_dn4 = 0.0;
        locals.var_eta0_i_dn5 = 0.0;
        locals.var_eta0_i_dn6 = 0.0;
        locals.var_eta0_i_dn7 = 0.0;
        locals.var_eta0_i_dn8 = 0.0;
        locals.var_eta0_i_dn9 = 0.0;
        locals.var_eta0_i_dn10 = 0.0;
        locals.var_eta0_i_dn11 = 0.0;
        locals.var_eta0_i_dn13 = 0.0;
        locals.var_eta0_i_dn14 = 0.0;
        locals.var_eta0_i_rv = 0.0;

        let assign4190_e4929: f64 = (locals.var_inv_l * p.p249);
        let assign4190_e4930: f64 = (p.p248 + assign4190_e4929);
        let assign4190_e4933: f64 = (locals.var_inv_nfin * p.p250);
        let assign4190_e4934: f64 = (assign4190_e4930 + assign4190_e4933);
        let assign4190_e4937: f64 = (locals.var_inv_lnfin * p.p251);
        let assign4190_e4938: f64 = (assign4190_e4934 + assign4190_e4937);
        let assign4190_e4941: f64 = (locals.var_inv_w * p.p252);
        let assign4190_e4942: f64 = (assign4190_e4938 + assign4190_e4941);
        let assign4190_e4945: f64 = (locals.var_inv_wl * p.p253);
        let assign4190_e4946: f64 = (assign4190_e4942 + assign4190_e4945);
        locals.var_eta1_i = assign4190_e4946;
        locals.var_eta1_i_rv = 0.0;

        let assign4200_e4950: f64 = (locals.var_inv_l * p.p267);
        let assign4200_e4951: f64 = (p.p266 + assign4200_e4950);
        let assign4200_e4954: f64 = (locals.var_inv_nfin * p.p268);
        let assign4200_e4955: f64 = (assign4200_e4951 + assign4200_e4954);
        let assign4200_e4958: f64 = (locals.var_inv_lnfin * p.p269);
        let assign4200_e4959: f64 = (assign4200_e4955 + assign4200_e4958);
        let assign4200_e4962: f64 = (locals.var_inv_w * p.p270);
        let assign4200_e4963: f64 = (assign4200_e4959 + assign4200_e4962);
        let assign4200_e4966: f64 = (locals.var_inv_wl * p.p271);
        let assign4200_e4967: f64 = (assign4200_e4963 + assign4200_e4966);
        locals.var_dsub_i = assign4200_e4967;
        locals.var_dsub_i_rv = 0.0;

        let assign4210_e4971: f64 = (locals.var_inv_l * p.p273);
        let assign4210_e4972: f64 = (p.p272 + assign4210_e4971);
        let assign4210_e4975: f64 = (locals.var_inv_nfin * p.p274);
        let assign4210_e4976: f64 = (assign4210_e4972 + assign4210_e4975);
        let assign4210_e4979: f64 = (locals.var_inv_lnfin * p.p275);
        let assign4210_e4980: f64 = (assign4210_e4976 + assign4210_e4979);
        let assign4210_e4983: f64 = (locals.var_inv_w * p.p276);
        let assign4210_e4984: f64 = (assign4210_e4980 + assign4210_e4983);
        let assign4210_e4987: f64 = (locals.var_inv_wl * p.p277);
        let assign4210_e4988: f64 = (assign4210_e4984 + assign4210_e4987);
        locals.var_k1rsce_i = assign4210_e4988;
        locals.var_k1rsce_i_rv = 0.0;

        let assign4220_e4992: f64 = (locals.var_inv_l * p.p279);
        let assign4220_e4993: f64 = (p.p278 + assign4220_e4992);
        let assign4220_e4996: f64 = (locals.var_inv_nfin * p.p280);
        let assign4220_e4997: f64 = (assign4220_e4993 + assign4220_e4996);
        let assign4220_e5000: f64 = (locals.var_inv_lnfin * p.p281);
        let assign4220_e5001: f64 = (assign4220_e4997 + assign4220_e5000);
        let assign4220_e5004: f64 = (locals.var_inv_w * p.p282);
        let assign4220_e5005: f64 = (assign4220_e5001 + assign4220_e5004);
        let assign4220_e5008: f64 = (locals.var_inv_wl * p.p283);
        let assign4220_e5009: f64 = (assign4220_e5005 + assign4220_e5008);
        locals.var_lpe0_i = assign4220_e5009;
        locals.var_lpe0_i_rv = 0.0;

        let assign4230_e5013: f64 = (locals.var_inv_l * p.p285);
        let assign4230_e5014: f64 = (p.p284 + assign4230_e5013);
        let assign4230_e5017: f64 = (locals.var_inv_nfin * p.p286);
        let assign4230_e5018: f64 = (assign4230_e5014 + assign4230_e5017);
        let assign4230_e5021: f64 = (locals.var_inv_lnfin * p.p287);
        let assign4230_e5022: f64 = (assign4230_e5018 + assign4230_e5021);
        let assign4230_e5025: f64 = (locals.var_inv_w * p.p288);
        let assign4230_e5026: f64 = (assign4230_e5022 + assign4230_e5025);
        let assign4230_e5029: f64 = (locals.var_inv_wl * p.p289);
        let assign4230_e5030: f64 = (assign4230_e5026 + assign4230_e5029);
        locals.var_dvtshift_i = assign4230_e5030;
        locals.var_dvtshift_i_rv = 0.0;

        let assign4240_e5034: f64 = (locals.var_inv_l * p.p297);
        let assign4240_e5035: f64 = (p.p296 + assign4240_e5034);
        let assign4240_e5038: f64 = (locals.var_inv_nfin * p.p298);
        let assign4240_e5039: f64 = (assign4240_e5035 + assign4240_e5038);
        let assign4240_e5042: f64 = (locals.var_inv_lnfin * p.p299);
        let assign4240_e5043: f64 = (assign4240_e5039 + assign4240_e5042);
        let assign4240_e5046: f64 = (locals.var_inv_w * p.p300);
        let assign4240_e5047: f64 = (assign4240_e5043 + assign4240_e5046);
        let assign4240_e5050: f64 = (locals.var_inv_wl * p.p301);
        let assign4240_e5051: f64 = (assign4240_e5047 + assign4240_e5050);
        locals.var_k0_i = assign4240_e5051;
        locals.var_k0_i_rv = 0.0;

        let assign4250_e5055: f64 = (locals.var_inv_l * p.p303);
        let assign4250_e5056: f64 = (p.p302 + assign4250_e5055);
        let assign4250_e5059: f64 = (locals.var_inv_nfin * p.p304);
        let assign4250_e5060: f64 = (assign4250_e5056 + assign4250_e5059);
        let assign4250_e5063: f64 = (locals.var_inv_lnfin * p.p305);
        let assign4250_e5064: f64 = (assign4250_e5060 + assign4250_e5063);
        let assign4250_e5067: f64 = (locals.var_inv_w * p.p306);
        let assign4250_e5068: f64 = (assign4250_e5064 + assign4250_e5067);
        let assign4250_e5071: f64 = (locals.var_inv_wl * p.p307);
        let assign4250_e5072: f64 = (assign4250_e5068 + assign4250_e5071);
        locals.var_k01_i = assign4250_e5072;
        locals.var_k01_i_rv = 0.0;

        let assign4260_e5076: f64 = (locals.var_inv_l * p.p309);
        let assign4260_e5077: f64 = (p.p308 + assign4260_e5076);
        let assign4260_e5080: f64 = (locals.var_inv_nfin * p.p310);
        let assign4260_e5081: f64 = (assign4260_e5077 + assign4260_e5080);
        let assign4260_e5084: f64 = (locals.var_inv_lnfin * p.p311);
        let assign4260_e5085: f64 = (assign4260_e5081 + assign4260_e5084);
        let assign4260_e5088: f64 = (locals.var_inv_w * p.p312);
        let assign4260_e5089: f64 = (assign4260_e5085 + assign4260_e5088);
        let assign4260_e5092: f64 = (locals.var_inv_wl * p.p313);
        let assign4260_e5093: f64 = (assign4260_e5089 + assign4260_e5092);
        locals.var_k0si_i = assign4260_e5093;
        locals.var_k0si_i_rv = 0.0;

        let assign4270_e5097: f64 = (locals.var_inv_l * p.p315);
        let assign4270_e5098: f64 = (p.p314 + assign4270_e5097);
        let assign4270_e5101: f64 = (locals.var_inv_nfin * p.p316);
        let assign4270_e5102: f64 = (assign4270_e5098 + assign4270_e5101);
        let assign4270_e5105: f64 = (locals.var_inv_lnfin * p.p317);
        let assign4270_e5106: f64 = (assign4270_e5102 + assign4270_e5105);
        let assign4270_e5109: f64 = (locals.var_inv_w * p.p318);
        let assign4270_e5110: f64 = (assign4270_e5106 + assign4270_e5109);
        let assign4270_e5113: f64 = (locals.var_inv_wl * p.p319);
        let assign4270_e5114: f64 = (assign4270_e5110 + assign4270_e5113);
        locals.var_k0si1_i = assign4270_e5114;
        locals.var_k0si1_i_rv = 0.0;

        let assign4280_e5118: f64 = (locals.var_inv_l * p.p321);
        let assign4280_e5119: f64 = (p.p320 + assign4280_e5118);
        let assign4280_e5122: f64 = (locals.var_inv_nfin * p.p322);
        let assign4280_e5123: f64 = (assign4280_e5119 + assign4280_e5122);
        let assign4280_e5126: f64 = (locals.var_inv_lnfin * p.p323);
        let assign4280_e5127: f64 = (assign4280_e5123 + assign4280_e5126);
        let assign4280_e5130: f64 = (locals.var_inv_w * p.p324);
        let assign4280_e5131: f64 = (assign4280_e5127 + assign4280_e5130);
        let assign4280_e5134: f64 = (locals.var_inv_wl * p.p325);
        let assign4280_e5135: f64 = (assign4280_e5131 + assign4280_e5134);
        locals.var_k2si_i = assign4280_e5135;
        locals.var_k2si_i_rv = 0.0;

        let assign4290_e5139: f64 = (locals.var_inv_l * p.p327);
        let assign4290_e5140: f64 = (p.p326 + assign4290_e5139);
        let assign4290_e5143: f64 = (locals.var_inv_nfin * p.p328);
        let assign4290_e5144: f64 = (assign4290_e5140 + assign4290_e5143);
        let assign4290_e5147: f64 = (locals.var_inv_lnfin * p.p329);
        let assign4290_e5148: f64 = (assign4290_e5144 + assign4290_e5147);
        let assign4290_e5151: f64 = (locals.var_inv_w * p.p330);
        let assign4290_e5152: f64 = (assign4290_e5148 + assign4290_e5151);
        let assign4290_e5155: f64 = (locals.var_inv_wl * p.p331);
        let assign4290_e5156: f64 = (assign4290_e5152 + assign4290_e5155);
        locals.var_k2si1_i = assign4290_e5156;
        locals.var_k2si1_i_rv = 0.0;

        let assign4300_e5160: f64 = (locals.var_inv_l * p.p333);
        let assign4300_e5161: f64 = (p.p332 + assign4300_e5160);
        let assign4300_e5164: f64 = (locals.var_inv_nfin * p.p334);
        let assign4300_e5165: f64 = (assign4300_e5161 + assign4300_e5164);
        let assign4300_e5168: f64 = (locals.var_inv_lnfin * p.p335);
        let assign4300_e5169: f64 = (assign4300_e5165 + assign4300_e5168);
        let assign4300_e5172: f64 = (locals.var_inv_w * p.p336);
        let assign4300_e5173: f64 = (assign4300_e5169 + assign4300_e5172);
        let assign4300_e5176: f64 = (locals.var_inv_wl * p.p337);
        let assign4300_e5177: f64 = (assign4300_e5173 + assign4300_e5176);
        locals.var_k0sisat_i = assign4300_e5177;
        locals.var_k0sisat_i_rv = 0.0;

        let assign4310_e5181: f64 = (locals.var_inv_l * p.p339);
        let assign4310_e5182: f64 = (p.p338 + assign4310_e5181);
        let assign4310_e5185: f64 = (locals.var_inv_nfin * p.p340);
        let assign4310_e5186: f64 = (assign4310_e5182 + assign4310_e5185);
        let assign4310_e5189: f64 = (locals.var_inv_lnfin * p.p341);
        let assign4310_e5190: f64 = (assign4310_e5186 + assign4310_e5189);
        let assign4310_e5193: f64 = (locals.var_inv_w * p.p342);
        let assign4310_e5194: f64 = (assign4310_e5190 + assign4310_e5193);
        let assign4310_e5197: f64 = (locals.var_inv_wl * p.p343);
        let assign4310_e5198: f64 = (assign4310_e5194 + assign4310_e5197);
        locals.var_k0sisat1_i = assign4310_e5198;
        locals.var_k0sisat1_i_rv = 0.0;

        let assign4320_e5202: f64 = (locals.var_inv_l * p.p345);
        let assign4320_e5203: f64 = (p.p344 + assign4320_e5202);
        let assign4320_e5206: f64 = (locals.var_inv_nfin * p.p346);
        let assign4320_e5207: f64 = (assign4320_e5203 + assign4320_e5206);
        let assign4320_e5210: f64 = (locals.var_inv_lnfin * p.p347);
        let assign4320_e5211: f64 = (assign4320_e5207 + assign4320_e5210);
        let assign4320_e5214: f64 = (locals.var_inv_w * p.p348);
        let assign4320_e5215: f64 = (assign4320_e5211 + assign4320_e5214);
        let assign4320_e5218: f64 = (locals.var_inv_wl * p.p349);
        let assign4320_e5219: f64 = (assign4320_e5215 + assign4320_e5218);
        locals.var_k2sisat_i = assign4320_e5219;
        locals.var_k2sisat_i_rv = 0.0;

        let assign4330_e5223: f64 = (locals.var_inv_l * p.p351);
        let assign4330_e5224: f64 = (p.p350 + assign4330_e5223);
        let assign4330_e5227: f64 = (locals.var_inv_nfin * p.p352);
        let assign4330_e5228: f64 = (assign4330_e5224 + assign4330_e5227);
        let assign4330_e5231: f64 = (locals.var_inv_lnfin * p.p353);
        let assign4330_e5232: f64 = (assign4330_e5228 + assign4330_e5231);
        let assign4330_e5235: f64 = (locals.var_inv_w * p.p354);
        let assign4330_e5236: f64 = (assign4330_e5232 + assign4330_e5235);
        let assign4330_e5239: f64 = (locals.var_inv_wl * p.p355);
        let assign4330_e5240: f64 = (assign4330_e5236 + assign4330_e5239);
        locals.var_k2sisat1_i = assign4330_e5240;
        locals.var_k2sisat1_i_rv = 0.0;

        let assign4340_e5244: f64 = (locals.var_inv_l * p.p404);
        let assign4340_e5245: f64 = (p.p403 + assign4340_e5244);
        let assign4340_e5248: f64 = (locals.var_inv_nfin * p.p405);
        let assign4340_e5249: f64 = (assign4340_e5245 + assign4340_e5248);
        let assign4340_e5252: f64 = (locals.var_inv_lnfin * p.p406);
        let assign4340_e5253: f64 = (assign4340_e5249 + assign4340_e5252);
        let assign4340_e5256: f64 = (locals.var_inv_w * p.p407);
        let assign4340_e5257: f64 = (assign4340_e5253 + assign4340_e5256);
        let assign4340_e5260: f64 = (locals.var_inv_wl * p.p408);
        let assign4340_e5261: f64 = (assign4340_e5257 + assign4340_e5260);
        locals.var_qmfactor_i = assign4340_e5261;
        locals.var_qmfactor_i_rv = 0.0;

        let assign4350_e5265: f64 = (locals.var_inv_l * p.p410);
        let assign4350_e5266: f64 = (p.p409 + assign4350_e5265);
        let assign4350_e5269: f64 = (locals.var_inv_nfin * p.p411);
        let assign4350_e5270: f64 = (assign4350_e5266 + assign4350_e5269);
        let assign4350_e5273: f64 = (locals.var_inv_lnfin * p.p412);
        let assign4350_e5274: f64 = (assign4350_e5270 + assign4350_e5273);
        let assign4350_e5277: f64 = (locals.var_inv_w * p.p413);
        let assign4350_e5278: f64 = (assign4350_e5274 + assign4350_e5277);
        let assign4350_e5281: f64 = (locals.var_inv_wl * p.p414);
        let assign4350_e5282: f64 = (assign4350_e5278 + assign4350_e5281);
        locals.var_qmtcencv_i = assign4350_e5282;
        locals.var_qmtcencv_i_rv = 0.0;

        let assign4360_e5286: f64 = (locals.var_inv_l * p.p416);
        let assign4360_e5287: f64 = (p.p415 + assign4360_e5286);
        let assign4360_e5290: f64 = (locals.var_inv_nfin * p.p417);
        let assign4360_e5291: f64 = (assign4360_e5287 + assign4360_e5290);
        let assign4360_e5294: f64 = (locals.var_inv_lnfin * p.p418);
        let assign4360_e5295: f64 = (assign4360_e5291 + assign4360_e5294);
        let assign4360_e5298: f64 = (locals.var_inv_w * p.p419);
        let assign4360_e5299: f64 = (assign4360_e5295 + assign4360_e5298);
        let assign4360_e5302: f64 = (locals.var_inv_wl * p.p420);
        let assign4360_e5303: f64 = (assign4360_e5299 + assign4360_e5302);
        locals.var_qmtcencva_i = assign4360_e5303;
        locals.var_qmtcencva_i_rv = 0.0;

        let assign4370_e5307: f64 = (locals.var_inv_l * p.p422);
        let assign4370_e5308: f64 = (p.p421 + assign4370_e5307);
        let assign4370_e5311: f64 = (locals.var_inv_nfin * p.p423);
        let assign4370_e5312: f64 = (assign4370_e5308 + assign4370_e5311);
        let assign4370_e5315: f64 = (locals.var_inv_lnfin * p.p424);
        let assign4370_e5316: f64 = (assign4370_e5312 + assign4370_e5315);
        let assign4370_e5319: f64 = (locals.var_inv_w * p.p425);
        let assign4370_e5320: f64 = (assign4370_e5316 + assign4370_e5319);
        let assign4370_e5323: f64 = (locals.var_inv_wl * p.p426);
        let assign4370_e5324: f64 = (assign4370_e5320 + assign4370_e5323);
        locals.var_pqm_i = assign4370_e5324;
        locals.var_pqm_i_dn0 = 0.0;
        locals.var_pqm_i_dn2 = 0.0;
        locals.var_pqm_i_dn3 = 0.0;
        locals.var_pqm_i_dn4 = 0.0;
        locals.var_pqm_i_dn5 = 0.0;
        locals.var_pqm_i_dn6 = 0.0;
        locals.var_pqm_i_dn7 = 0.0;
        locals.var_pqm_i_dn8 = 0.0;
        locals.var_pqm_i_dn9 = 0.0;
        locals.var_pqm_i_dn10 = 0.0;
        locals.var_pqm_i_dn11 = 0.0;
        locals.var_pqm_i_dn13 = 0.0;
        locals.var_pqm_i_dn14 = 0.0;
        locals.var_pqm_i_rv = 0.0;

        let assign4380_e5328: f64 = (locals.var_inv_l * p.p456);
        let assign4380_e5329: f64 = (p.p455 + assign4380_e5328);
        let assign4380_e5332: f64 = (locals.var_inv_nfin * p.p457);
        let assign4380_e5333: f64 = (assign4380_e5329 + assign4380_e5332);
        let assign4380_e5336: f64 = (locals.var_inv_lnfin * p.p458);
        let assign4380_e5337: f64 = (assign4380_e5333 + assign4380_e5336);
        let assign4380_e5340: f64 = (locals.var_inv_w * p.p459);
        let assign4380_e5341: f64 = (assign4380_e5337 + assign4380_e5340);
        let assign4380_e5344: f64 = (locals.var_inv_wl * p.p460);
        let assign4380_e5345: f64 = (assign4380_e5341 + assign4380_e5344);
        locals.var_vsat_i = assign4380_e5345;
        locals.var_vsat_i_dn0 = 0.0;
        locals.var_vsat_i_dn2 = 0.0;
        locals.var_vsat_i_dn3 = 0.0;
        locals.var_vsat_i_dn4 = 0.0;
        locals.var_vsat_i_dn5 = 0.0;
        locals.var_vsat_i_dn6 = 0.0;
        locals.var_vsat_i_dn7 = 0.0;
        locals.var_vsat_i_dn8 = 0.0;
        locals.var_vsat_i_dn9 = 0.0;
        locals.var_vsat_i_dn10 = 0.0;
        locals.var_vsat_i_dn11 = 0.0;
        locals.var_vsat_i_dn13 = 0.0;
        locals.var_vsat_i_dn14 = 0.0;
        locals.var_vsat_i_rv = 0.0;

        let assign4390_e5349: f64 = (locals.var_inv_l * p.p468);
        let assign4390_e5350: f64 = (p.p467 + assign4390_e5349);
        let assign4390_e5353: f64 = (locals.var_inv_nfin * p.p469);
        let assign4390_e5354: f64 = (assign4390_e5350 + assign4390_e5353);
        let assign4390_e5357: f64 = (locals.var_inv_lnfin * p.p470);
        let assign4390_e5358: f64 = (assign4390_e5354 + assign4390_e5357);
        let assign4390_e5361: f64 = (locals.var_inv_w * p.p471);
        let assign4390_e5362: f64 = (assign4390_e5358 + assign4390_e5361);
        let assign4390_e5365: f64 = (locals.var_inv_wl * p.p472);
        let assign4390_e5366: f64 = (assign4390_e5362 + assign4390_e5365);
        locals.var_vsat1_i = assign4390_e5366;
        locals.var_vsat1_i_dn0 = 0.0;
        locals.var_vsat1_i_dn2 = 0.0;
        locals.var_vsat1_i_dn3 = 0.0;
        locals.var_vsat1_i_dn4 = 0.0;
        locals.var_vsat1_i_dn5 = 0.0;
        locals.var_vsat1_i_dn6 = 0.0;
        locals.var_vsat1_i_dn7 = 0.0;
        locals.var_vsat1_i_dn8 = 0.0;
        locals.var_vsat1_i_dn9 = 0.0;
        locals.var_vsat1_i_dn10 = 0.0;
        locals.var_vsat1_i_dn11 = 0.0;
        locals.var_vsat1_i_dn13 = 0.0;
        locals.var_vsat1_i_dn14 = 0.0;
        locals.var_vsat1_i_rv = 0.0;

        let assign4400_e5370: f64 = (locals.var_inv_l * p.p507);
        let assign4400_e5371: f64 = (p.p506 + assign4400_e5370);
        let assign4400_e5374: f64 = (locals.var_inv_nfin * p.p508);
        let assign4400_e5375: f64 = (assign4400_e5371 + assign4400_e5374);
        let assign4400_e5378: f64 = (locals.var_inv_lnfin * p.p509);
        let assign4400_e5379: f64 = (assign4400_e5375 + assign4400_e5378);
        let assign4400_e5382: f64 = (locals.var_inv_w * p.p510);
        let assign4400_e5383: f64 = (assign4400_e5379 + assign4400_e5382);
        let assign4400_e5386: f64 = (locals.var_inv_wl * p.p511);
        let assign4400_e5387: f64 = (assign4400_e5383 + assign4400_e5386);
        locals.var_vsatcv_i = assign4400_e5387;
        locals.var_vsatcv_i_dn0 = 0.0;
        locals.var_vsatcv_i_dn2 = 0.0;
        locals.var_vsatcv_i_dn3 = 0.0;
        locals.var_vsatcv_i_dn4 = 0.0;
        locals.var_vsatcv_i_dn5 = 0.0;
        locals.var_vsatcv_i_dn6 = 0.0;
        locals.var_vsatcv_i_dn7 = 0.0;
        locals.var_vsatcv_i_dn8 = 0.0;
        locals.var_vsatcv_i_dn9 = 0.0;
        locals.var_vsatcv_i_dn10 = 0.0;
        locals.var_vsatcv_i_dn11 = 0.0;
        locals.var_vsatcv_i_dn13 = 0.0;
        locals.var_vsatcv_i_dn14 = 0.0;
        locals.var_vsatcv_i_rv = 0.0;

        let assign4410_e5391: f64 = (locals.var_inv_l * p.p513);
        let assign4410_e5392: f64 = (p.p512 + assign4410_e5391);
        let assign4410_e5395: f64 = (locals.var_inv_nfin * p.p514);
        let assign4410_e5396: f64 = (assign4410_e5392 + assign4410_e5395);
        let assign4410_e5399: f64 = (locals.var_inv_lnfin * p.p515);
        let assign4410_e5400: f64 = (assign4410_e5396 + assign4410_e5399);
        let assign4410_e5403: f64 = (locals.var_inv_w * p.p516);
        let assign4410_e5404: f64 = (assign4410_e5400 + assign4410_e5403);
        let assign4410_e5407: f64 = (locals.var_inv_wl * p.p517);
        let assign4410_e5408: f64 = (assign4410_e5404 + assign4410_e5407);
        locals.var_asat_i = assign4410_e5408;
        locals.var_asat_i_rv = 0.0;

        let assign4420_e5412: f64 = (locals.var_inv_l * p.p480);
        let assign4420_e5413: f64 = (p.p479 + assign4420_e5412);
        let assign4420_e5416: f64 = (locals.var_inv_nfin * p.p481);
        let assign4420_e5417: f64 = (assign4420_e5413 + assign4420_e5416);
        let assign4420_e5420: f64 = (locals.var_inv_lnfin * p.p482);
        let assign4420_e5421: f64 = (assign4420_e5417 + assign4420_e5420);
        let assign4420_e5424: f64 = (locals.var_inv_w * p.p483);
        let assign4420_e5425: f64 = (assign4420_e5421 + assign4420_e5424);
        let assign4420_e5428: f64 = (locals.var_inv_wl * p.p484);
        let assign4420_e5429: f64 = (assign4420_e5425 + assign4420_e5428);
        locals.var_deltavsat_i = assign4420_e5429;
        locals.var_deltavsat_i_rv = 0.0;

        let assign4430_e5433: f64 = (locals.var_inv_l * p.p486);
        let assign4430_e5434: f64 = (p.p485 + assign4430_e5433);
        let assign4430_e5437: f64 = (locals.var_inv_nfin * p.p487);
        let assign4430_e5438: f64 = (assign4430_e5434 + assign4430_e5437);
        let assign4430_e5441: f64 = (locals.var_inv_lnfin * p.p488);
        let assign4430_e5442: f64 = (assign4430_e5438 + assign4430_e5441);
        let assign4430_e5445: f64 = (locals.var_inv_w * p.p489);
        let assign4430_e5446: f64 = (assign4430_e5442 + assign4430_e5445);
        let assign4430_e5449: f64 = (locals.var_inv_wl * p.p490);
        let assign4430_e5450: f64 = (assign4430_e5446 + assign4430_e5449);
        locals.var_psat_i = assign4430_e5450;
        locals.var_psat_i_dn0 = 0.0;
        locals.var_psat_i_dn2 = 0.0;
        locals.var_psat_i_dn3 = 0.0;
        locals.var_psat_i_dn4 = 0.0;
        locals.var_psat_i_dn5 = 0.0;
        locals.var_psat_i_dn6 = 0.0;
        locals.var_psat_i_dn7 = 0.0;
        locals.var_psat_i_dn8 = 0.0;
        locals.var_psat_i_dn9 = 0.0;
        locals.var_psat_i_dn10 = 0.0;
        locals.var_psat_i_dn11 = 0.0;
        locals.var_psat_i_dn13 = 0.0;
        locals.var_psat_i_dn14 = 0.0;
        locals.var_psat_i_rv = 0.0;

        let assign4440_e5454: f64 = (locals.var_inv_l * p.p519);
        let assign4440_e5455: f64 = (p.p518 + assign4440_e5454);
        let assign4440_e5458: f64 = (locals.var_inv_nfin * p.p520);
        let assign4440_e5459: f64 = (assign4440_e5455 + assign4440_e5458);
        let assign4440_e5462: f64 = (locals.var_inv_lnfin * p.p521);
        let assign4440_e5463: f64 = (assign4440_e5459 + assign4440_e5462);
        let assign4440_e5466: f64 = (locals.var_inv_w * p.p522);
        let assign4440_e5467: f64 = (assign4440_e5463 + assign4440_e5466);
        let assign4440_e5470: f64 = (locals.var_inv_wl * p.p523);
        let assign4440_e5471: f64 = (assign4440_e5467 + assign4440_e5470);
        locals.var_deltavsatcv_i = assign4440_e5471;
        locals.var_deltavsatcv_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4450_e5475: f64 = (locals.var_inv_l * p.p525);
        let assign4450_e5476: f64 = (p.p524 + assign4450_e5475);
        let assign4450_e5479: f64 = (locals.var_inv_nfin * p.p526);
        let assign4450_e5480: f64 = (assign4450_e5476 + assign4450_e5479);
        let assign4450_e5483: f64 = (locals.var_inv_lnfin * p.p527);
        let assign4450_e5484: f64 = (assign4450_e5480 + assign4450_e5483);
        let assign4450_e5487: f64 = (locals.var_inv_w * p.p528);
        let assign4450_e5488: f64 = (assign4450_e5484 + assign4450_e5487);
        let assign4450_e5491: f64 = (locals.var_inv_wl * p.p529);
        let assign4450_e5492: f64 = (assign4450_e5488 + assign4450_e5491);
        locals.var_psatcv_i = assign4450_e5492;
        locals.var_psatcv_i_dn0 = 0.0;
        locals.var_psatcv_i_dn2 = 0.0;
        locals.var_psatcv_i_dn3 = 0.0;
        locals.var_psatcv_i_dn4 = 0.0;
        locals.var_psatcv_i_dn5 = 0.0;
        locals.var_psatcv_i_dn6 = 0.0;
        locals.var_psatcv_i_dn7 = 0.0;
        locals.var_psatcv_i_dn8 = 0.0;
        locals.var_psatcv_i_dn9 = 0.0;
        locals.var_psatcv_i_dn10 = 0.0;
        locals.var_psatcv_i_dn11 = 0.0;
        locals.var_psatcv_i_dn13 = 0.0;
        locals.var_psatcv_i_dn14 = 0.0;
        locals.var_psatcv_i_rv = 0.0;

        let assign4460_e5496: f64 = (locals.var_inv_l * p.p493);
        let assign4460_e5497: f64 = (p.p492 + assign4460_e5496);
        let assign4460_e5500: f64 = (locals.var_inv_nfin * p.p494);
        let assign4460_e5501: f64 = (assign4460_e5497 + assign4460_e5500);
        let assign4460_e5504: f64 = (locals.var_inv_lnfin * p.p495);
        let assign4460_e5505: f64 = (assign4460_e5501 + assign4460_e5504);
        let assign4460_e5508: f64 = (locals.var_inv_w * p.p496);
        let assign4460_e5509: f64 = (assign4460_e5505 + assign4460_e5508);
        let assign4460_e5512: f64 = (locals.var_inv_wl * p.p497);
        let assign4460_e5513: f64 = (assign4460_e5509 + assign4460_e5512);
        locals.var_ksativ_i = assign4460_e5513;
        locals.var_ksativ_i_rv = 0.0;

        let assign4470_e5517: f64 = (locals.var_inv_l * p.p532);
        let assign4470_e5518: f64 = (p.p531 + assign4470_e5517);
        let assign4470_e5521: f64 = (locals.var_inv_nfin * p.p533);
        let assign4470_e5522: f64 = (assign4470_e5518 + assign4470_e5521);
        let assign4470_e5525: f64 = (locals.var_inv_lnfin * p.p534);
        let assign4470_e5526: f64 = (assign4470_e5522 + assign4470_e5525);
        let assign4470_e5529: f64 = (locals.var_inv_w * p.p535);
        let assign4470_e5530: f64 = (assign4470_e5526 + assign4470_e5529);
        let assign4470_e5533: f64 = (locals.var_inv_wl * p.p536);
        let assign4470_e5534: f64 = (assign4470_e5530 + assign4470_e5533);
        locals.var_mexp_i = assign4470_e5534;
        locals.var_mexp_i_dn0 = 0.0;
        locals.var_mexp_i_dn2 = 0.0;
        locals.var_mexp_i_dn3 = 0.0;
        locals.var_mexp_i_dn4 = 0.0;
        locals.var_mexp_i_dn5 = 0.0;
        locals.var_mexp_i_dn6 = 0.0;
        locals.var_mexp_i_dn7 = 0.0;
        locals.var_mexp_i_dn8 = 0.0;
        locals.var_mexp_i_dn9 = 0.0;
        locals.var_mexp_i_dn10 = 0.0;
        locals.var_mexp_i_dn11 = 0.0;
        locals.var_mexp_i_dn13 = 0.0;
        locals.var_mexp_i_dn14 = 0.0;
        locals.var_mexp_i_rv = 0.0;

        let assign4480_e5538: f64 = (locals.var_inv_l * p.p544);
        let assign4480_e5539: f64 = (p.p543 + assign4480_e5538);
        let assign4480_e5542: f64 = (locals.var_inv_nfin * p.p545);
        let assign4480_e5543: f64 = (assign4480_e5539 + assign4480_e5542);
        let assign4480_e5546: f64 = (locals.var_inv_lnfin * p.p546);
        let assign4480_e5547: f64 = (assign4480_e5543 + assign4480_e5546);
        let assign4480_e5550: f64 = (locals.var_inv_w * p.p547);
        let assign4480_e5551: f64 = (assign4480_e5547 + assign4480_e5550);
        let assign4480_e5554: f64 = (locals.var_inv_wl * p.p548);
        let assign4480_e5555: f64 = (assign4480_e5551 + assign4480_e5554);
        locals.var_ptwg_i = assign4480_e5555;
        locals.var_ptwg_i_dn0 = 0.0;
        locals.var_ptwg_i_dn2 = 0.0;
        locals.var_ptwg_i_dn3 = 0.0;
        locals.var_ptwg_i_dn4 = 0.0;
        locals.var_ptwg_i_dn5 = 0.0;
        locals.var_ptwg_i_dn6 = 0.0;
        locals.var_ptwg_i_dn7 = 0.0;
        locals.var_ptwg_i_dn8 = 0.0;
        locals.var_ptwg_i_dn9 = 0.0;
        locals.var_ptwg_i_dn10 = 0.0;
        locals.var_ptwg_i_dn11 = 0.0;
        locals.var_ptwg_i_dn13 = 0.0;
        locals.var_ptwg_i_dn14 = 0.0;
        locals.var_ptwg_i_rv = 0.0;

        let assign4490_e5559: f64 = (locals.var_inv_l * p.p606);
        let assign4490_e5560: f64 = (p.p605 + assign4490_e5559);
        let assign4490_e5563: f64 = (locals.var_inv_nfin * p.p607);
        let assign4490_e5564: f64 = (assign4490_e5560 + assign4490_e5563);
        let assign4490_e5567: f64 = (locals.var_inv_lnfin * p.p608);
        let assign4490_e5568: f64 = (assign4490_e5564 + assign4490_e5567);
        let assign4490_e5571: f64 = (locals.var_inv_w * p.p609);
        let assign4490_e5572: f64 = (assign4490_e5568 + assign4490_e5571);
        let assign4490_e5575: f64 = (locals.var_inv_wl * p.p610);
        let assign4490_e5576: f64 = (assign4490_e5572 + assign4490_e5575);
        locals.var_u0_i = assign4490_e5576;
        locals.var_u0_i_dn0 = 0.0;
        locals.var_u0_i_dn2 = 0.0;
        locals.var_u0_i_dn3 = 0.0;
        locals.var_u0_i_dn4 = 0.0;
        locals.var_u0_i_dn5 = 0.0;
        locals.var_u0_i_dn6 = 0.0;
        locals.var_u0_i_dn7 = 0.0;
        locals.var_u0_i_dn8 = 0.0;
        locals.var_u0_i_dn9 = 0.0;
        locals.var_u0_i_dn10 = 0.0;
        locals.var_u0_i_dn11 = 0.0;
        locals.var_u0_i_dn13 = 0.0;
        locals.var_u0_i_dn14 = 0.0;
        locals.var_u0_i_rv = 0.0;

        let assign4500_e5580: f64 = (locals.var_inv_l * p.p624);
        let assign4500_e5581: f64 = (p.p623 + assign4500_e5580);
        let assign4500_e5584: f64 = (locals.var_inv_nfin * p.p625);
        let assign4500_e5585: f64 = (assign4500_e5581 + assign4500_e5584);
        let assign4500_e5588: f64 = (locals.var_inv_lnfin * p.p626);
        let assign4500_e5589: f64 = (assign4500_e5585 + assign4500_e5588);
        let assign4500_e5592: f64 = (locals.var_inv_w * p.p627);
        let assign4500_e5593: f64 = (assign4500_e5589 + assign4500_e5592);
        let assign4500_e5596: f64 = (locals.var_inv_wl * p.p628);
        let assign4500_e5597: f64 = (assign4500_e5593 + assign4500_e5596);
        locals.var_etamob_i = assign4500_e5597;
        locals.var_etamob_i_rv = 0.0;

        let assign4510_e5601: f64 = (locals.var_inv_l * p.p630);
        let assign4510_e5602: f64 = (p.p629 + assign4510_e5601);
        let assign4510_e5605: f64 = (locals.var_inv_nfin * p.p631);
        let assign4510_e5606: f64 = (assign4510_e5602 + assign4510_e5605);
        let assign4510_e5609: f64 = (locals.var_inv_lnfin * p.p632);
        let assign4510_e5610: f64 = (assign4510_e5606 + assign4510_e5609);
        let assign4510_e5613: f64 = (locals.var_inv_w * p.p633);
        let assign4510_e5614: f64 = (assign4510_e5610 + assign4510_e5613);
        let assign4510_e5617: f64 = (locals.var_inv_wl * p.p634);
        let assign4510_e5618: f64 = (assign4510_e5614 + assign4510_e5617);
        locals.var_up_i = assign4510_e5618;
        locals.var_up_i_rv = 0.0;

        let assign4520_e5622: f64 = (locals.var_inv_l * p.p642);
        let assign4520_e5623: f64 = (p.p641 + assign4520_e5622);
        let assign4520_e5626: f64 = (locals.var_inv_nfin * p.p643);
        let assign4520_e5627: f64 = (assign4520_e5623 + assign4520_e5626);
        let assign4520_e5630: f64 = (locals.var_inv_lnfin * p.p644);
        let assign4520_e5631: f64 = (assign4520_e5627 + assign4520_e5630);
        let assign4520_e5634: f64 = (locals.var_inv_w * p.p645);
        let assign4520_e5635: f64 = (assign4520_e5631 + assign4520_e5634);
        let assign4520_e5638: f64 = (locals.var_inv_wl * p.p646);
        let assign4520_e5639: f64 = (assign4520_e5635 + assign4520_e5638);
        locals.var_ua_i = assign4520_e5639;
        locals.var_ua_i_dn0 = 0.0;
        locals.var_ua_i_dn2 = 0.0;
        locals.var_ua_i_dn3 = 0.0;
        locals.var_ua_i_dn4 = 0.0;
        locals.var_ua_i_dn5 = 0.0;
        locals.var_ua_i_dn6 = 0.0;
        locals.var_ua_i_dn7 = 0.0;
        locals.var_ua_i_dn8 = 0.0;
        locals.var_ua_i_dn9 = 0.0;
        locals.var_ua_i_dn10 = 0.0;
        locals.var_ua_i_dn11 = 0.0;
        locals.var_ua_i_dn13 = 0.0;
        locals.var_ua_i_dn14 = 0.0;
        locals.var_ua_i_rv = 0.0;

        let assign4530_e5643: f64 = (locals.var_inv_l * p.p678);
        let assign4530_e5644: f64 = (p.p677 + assign4530_e5643);
        let assign4530_e5647: f64 = (locals.var_inv_nfin * p.p679);
        let assign4530_e5648: f64 = (assign4530_e5644 + assign4530_e5647);
        let assign4530_e5651: f64 = (locals.var_inv_lnfin * p.p680);
        let assign4530_e5652: f64 = (assign4530_e5648 + assign4530_e5651);
        let assign4530_e5655: f64 = (locals.var_inv_w * p.p681);
        let assign4530_e5656: f64 = (assign4530_e5652 + assign4530_e5655);
        let assign4530_e5659: f64 = (locals.var_inv_wl * p.p682);
        let assign4530_e5660: f64 = (assign4530_e5656 + assign4530_e5659);
        locals.var_eu_i = assign4530_e5660;
        locals.var_eu_i_dn0 = 0.0;
        locals.var_eu_i_dn2 = 0.0;
        locals.var_eu_i_dn3 = 0.0;
        locals.var_eu_i_dn4 = 0.0;
        locals.var_eu_i_dn5 = 0.0;
        locals.var_eu_i_dn6 = 0.0;
        locals.var_eu_i_dn7 = 0.0;
        locals.var_eu_i_dn8 = 0.0;
        locals.var_eu_i_dn9 = 0.0;
        locals.var_eu_i_dn10 = 0.0;
        locals.var_eu_i_dn11 = 0.0;
        locals.var_eu_i_dn13 = 0.0;
        locals.var_eu_i_dn14 = 0.0;
        locals.var_eu_i_rv = 0.0;

        let assign4540_e5664: f64 = (locals.var_inv_l * p.p690);
        let assign4540_e5665: f64 = (p.p689 + assign4540_e5664);
        let assign4540_e5668: f64 = (locals.var_inv_nfin * p.p691);
        let assign4540_e5669: f64 = (assign4540_e5665 + assign4540_e5668);
        let assign4540_e5672: f64 = (locals.var_inv_lnfin * p.p692);
        let assign4540_e5673: f64 = (assign4540_e5669 + assign4540_e5672);
        let assign4540_e5676: f64 = (locals.var_inv_w * p.p693);
        let assign4540_e5677: f64 = (assign4540_e5673 + assign4540_e5676);
        let assign4540_e5680: f64 = (locals.var_inv_wl * p.p694);
        let assign4540_e5681: f64 = (assign4540_e5677 + assign4540_e5680);
        locals.var_ud_i = assign4540_e5681;
        locals.var_ud_i_dn0 = 0.0;
        locals.var_ud_i_dn2 = 0.0;
        locals.var_ud_i_dn3 = 0.0;
        locals.var_ud_i_dn4 = 0.0;
        locals.var_ud_i_dn5 = 0.0;
        locals.var_ud_i_dn6 = 0.0;
        locals.var_ud_i_dn7 = 0.0;
        locals.var_ud_i_dn8 = 0.0;
        locals.var_ud_i_dn9 = 0.0;
        locals.var_ud_i_dn10 = 0.0;
        locals.var_ud_i_dn11 = 0.0;
        locals.var_ud_i_dn13 = 0.0;
        locals.var_ud_i_dn14 = 0.0;
        locals.var_ud_i_rv = 0.0;

        let assign4550_e5685: f64 = (locals.var_inv_l * p.p708);
        let assign4550_e5686: f64 = (p.p707 + assign4550_e5685);
        let assign4550_e5689: f64 = (locals.var_inv_nfin * p.p709);
        let assign4550_e5690: f64 = (assign4550_e5686 + assign4550_e5689);
        let assign4550_e5693: f64 = (locals.var_inv_lnfin * p.p710);
        let assign4550_e5694: f64 = (assign4550_e5690 + assign4550_e5693);
        let assign4550_e5697: f64 = (locals.var_inv_w * p.p711);
        let assign4550_e5698: f64 = (assign4550_e5694 + assign4550_e5697);
        let assign4550_e5701: f64 = (locals.var_inv_wl * p.p712);
        let assign4550_e5702: f64 = (assign4550_e5698 + assign4550_e5701);
        locals.var_ucs_i = assign4550_e5702;
        locals.var_ucs_i_rv = 0.0;

        let assign4560_e5706: f64 = (locals.var_inv_l * p.p714);
        let assign4560_e5707: f64 = (p.p713 + assign4560_e5706);
        let assign4560_e5710: f64 = (locals.var_inv_nfin * p.p715);
        let assign4560_e5711: f64 = (assign4560_e5707 + assign4560_e5710);
        let assign4560_e5714: f64 = (locals.var_inv_lnfin * p.p716);
        let assign4560_e5715: f64 = (assign4560_e5711 + assign4560_e5714);
        let assign4560_e5718: f64 = (locals.var_inv_w * p.p717);
        let assign4560_e5719: f64 = (assign4560_e5715 + assign4560_e5718);
        let assign4560_e5722: f64 = (locals.var_inv_wl * p.p718);
        let assign4560_e5723: f64 = (assign4560_e5719 + assign4560_e5722);
        locals.var_uds_i = assign4560_e5723;
        locals.var_uds_i_rv = 0.0;

        let assign4570_e5727: f64 = (locals.var_inv_l * p.p720);
        let assign4570_e5728: f64 = (p.p719 + assign4570_e5727);
        let assign4570_e5731: f64 = (locals.var_inv_nfin * p.p721);
        let assign4570_e5732: f64 = (assign4570_e5728 + assign4570_e5731);
        let assign4570_e5735: f64 = (locals.var_inv_lnfin * p.p722);
        let assign4570_e5736: f64 = (assign4570_e5732 + assign4570_e5735);
        let assign4570_e5739: f64 = (locals.var_inv_w * p.p723);
        let assign4570_e5740: f64 = (assign4570_e5736 + assign4570_e5739);
        let assign4570_e5743: f64 = (locals.var_inv_wl * p.p724);
        let assign4570_e5744: f64 = (assign4570_e5740 + assign4570_e5743);
        locals.var_uds1_i = assign4570_e5744;
        locals.var_uds1_i_rv = 0.0;

        let assign4580_e5748: f64 = (locals.var_inv_l * p.p726);
        let assign4580_e5749: f64 = (p.p725 + assign4580_e5748);
        let assign4580_e5752: f64 = (locals.var_inv_nfin * p.p727);
        let assign4580_e5753: f64 = (assign4580_e5749 + assign4580_e5752);
        let assign4580_e5756: f64 = (locals.var_inv_lnfin * p.p728);
        let assign4580_e5757: f64 = (assign4580_e5753 + assign4580_e5756);
        let assign4580_e5760: f64 = (locals.var_inv_w * p.p729);
        let assign4580_e5761: f64 = (assign4580_e5757 + assign4580_e5760);
        let assign4580_e5764: f64 = (locals.var_inv_wl * p.p730);
        let assign4580_e5765: f64 = (assign4580_e5761 + assign4580_e5764);
        locals.var_udd_i = assign4580_e5765;
        locals.var_udd_i_rv = 0.0;

        let assign4590_e5769: f64 = (locals.var_inv_l * p.p732);
        let assign4590_e5770: f64 = (p.p731 + assign4590_e5769);
        let assign4590_e5773: f64 = (locals.var_inv_nfin * p.p733);
        let assign4590_e5774: f64 = (assign4590_e5770 + assign4590_e5773);
        let assign4590_e5777: f64 = (locals.var_inv_lnfin * p.p734);
        let assign4590_e5778: f64 = (assign4590_e5774 + assign4590_e5777);
        let assign4590_e5781: f64 = (locals.var_inv_w * p.p735);
        let assign4590_e5782: f64 = (assign4590_e5778 + assign4590_e5781);
        let assign4590_e5785: f64 = (locals.var_inv_wl * p.p736);
        let assign4590_e5786: f64 = (assign4590_e5782 + assign4590_e5785);
        locals.var_udd1_i = assign4590_e5786;
        locals.var_udd1_i_rv = 0.0;

        let assign4600_e5790: f64 = (locals.var_inv_l * p.p1027);
        let assign4600_e5791: f64 = (p.p1025 + assign4600_e5790);
        let assign4600_e5794: f64 = (locals.var_inv_nfin * p.p1028);
        let assign4600_e5795: f64 = (assign4600_e5791 + assign4600_e5794);
        let assign4600_e5798: f64 = (locals.var_inv_lnfin * p.p1029);
        let assign4600_e5799: f64 = (assign4600_e5795 + assign4600_e5798);
        let assign4600_e5802: f64 = (locals.var_inv_w * p.p1030);
        let assign4600_e5803: f64 = (assign4600_e5799 + assign4600_e5802);
        let assign4600_e5806: f64 = (locals.var_inv_wl * p.p1031);
        let assign4600_e5807: f64 = (assign4600_e5803 + assign4600_e5806);
        locals.var_pclm_i = assign4600_e5807;
        locals.var_pclm_i_dn0 = 0.0;
        locals.var_pclm_i_dn2 = 0.0;
        locals.var_pclm_i_dn3 = 0.0;
        locals.var_pclm_i_dn4 = 0.0;
        locals.var_pclm_i_dn5 = 0.0;
        locals.var_pclm_i_dn6 = 0.0;
        locals.var_pclm_i_dn7 = 0.0;
        locals.var_pclm_i_dn8 = 0.0;
        locals.var_pclm_i_dn9 = 0.0;
        locals.var_pclm_i_dn10 = 0.0;
        locals.var_pclm_i_dn11 = 0.0;
        locals.var_pclm_i_dn13 = 0.0;
        locals.var_pclm_i_dn14 = 0.0;
        locals.var_pclm_i_rv = 0.0;

        let assign4610_e5811: f64 = (locals.var_inv_l * p.p1039);
        let assign4610_e5812: f64 = (p.p1038 + assign4610_e5811);
        let assign4610_e5815: f64 = (locals.var_inv_nfin * p.p1040);
        let assign4610_e5816: f64 = (assign4610_e5812 + assign4610_e5815);
        let assign4610_e5819: f64 = (locals.var_inv_lnfin * p.p1041);
        let assign4610_e5820: f64 = (assign4610_e5816 + assign4610_e5819);
        let assign4610_e5823: f64 = (locals.var_inv_w * p.p1042);
        let assign4610_e5824: f64 = (assign4610_e5820 + assign4610_e5823);
        let assign4610_e5827: f64 = (locals.var_inv_wl * p.p1043);
        let assign4610_e5828: f64 = (assign4610_e5824 + assign4610_e5827);
        locals.var_pclmg_i = assign4610_e5828;
        locals.var_pclmg_i_rv = 0.0;

        let assign4620_e5832: f64 = (locals.var_inv_l * p.p1045);
        let assign4620_e5833: f64 = (p.p1044 + assign4620_e5832);
        let assign4620_e5836: f64 = (locals.var_inv_nfin * p.p1046);
        let assign4620_e5837: f64 = (assign4620_e5833 + assign4620_e5836);
        let assign4620_e5840: f64 = (locals.var_inv_lnfin * p.p1047);
        let assign4620_e5841: f64 = (assign4620_e5837 + assign4620_e5840);
        let assign4620_e5844: f64 = (locals.var_inv_w * p.p1048);
        let assign4620_e5845: f64 = (assign4620_e5841 + assign4620_e5844);
        let assign4620_e5848: f64 = (locals.var_inv_wl * p.p1049);
        let assign4620_e5849: f64 = (assign4620_e5845 + assign4620_e5848);
        locals.var_pclmcv_i = assign4620_e5849;
        locals.var_pclmcv_i_rv = 0.0;

        let assign4630_e5853: f64 = (locals.var_inv_l * p.p1051);
        let assign4630_e5854: f64 = (p.p1050 + assign4630_e5853);
        let assign4630_e5857: f64 = (locals.var_inv_nfin * p.p1052);
        let assign4630_e5858: f64 = (assign4630_e5854 + assign4630_e5857);
        let assign4630_e5861: f64 = (locals.var_inv_lnfin * p.p1053);
        let assign4630_e5862: f64 = (assign4630_e5858 + assign4630_e5861);
        let assign4630_e5865: f64 = (locals.var_inv_w * p.p1054);
        let assign4630_e5866: f64 = (assign4630_e5862 + assign4630_e5865);
        let assign4630_e5869: f64 = (locals.var_inv_wl * p.p1055);
        let assign4630_e5870: f64 = (assign4630_e5866 + assign4630_e5869);
        locals.var_a1_i = assign4630_e5870;
        locals.var_a1_i_rv = 0.0;

        let assign4640_e5874: f64 = (locals.var_inv_l * p.p1057);
        let assign4640_e5875: f64 = (p.p1056 + assign4640_e5874);
        let assign4640_e5878: f64 = (locals.var_inv_nfin * p.p1058);
        let assign4640_e5879: f64 = (assign4640_e5875 + assign4640_e5878);
        let assign4640_e5882: f64 = (locals.var_inv_lnfin * p.p1059);
        let assign4640_e5883: f64 = (assign4640_e5879 + assign4640_e5882);
        let assign4640_e5886: f64 = (locals.var_inv_w * p.p1060);
        let assign4640_e5887: f64 = (assign4640_e5883 + assign4640_e5886);
        let assign4640_e5890: f64 = (locals.var_inv_wl * p.p1061);
        let assign4640_e5891: f64 = (assign4640_e5887 + assign4640_e5890);
        locals.var_a11_i = assign4640_e5891;
        locals.var_a11_i_rv = 0.0;

        let assign4650_e5895: f64 = (locals.var_inv_l * p.p1063);
        let assign4650_e5896: f64 = (p.p1062 + assign4650_e5895);
        let assign4650_e5899: f64 = (locals.var_inv_nfin * p.p1064);
        let assign4650_e5900: f64 = (assign4650_e5896 + assign4650_e5899);
        let assign4650_e5903: f64 = (locals.var_inv_lnfin * p.p1065);
        let assign4650_e5904: f64 = (assign4650_e5900 + assign4650_e5903);
        let assign4650_e5907: f64 = (locals.var_inv_w * p.p1066);
        let assign4650_e5908: f64 = (assign4650_e5904 + assign4650_e5907);
        let assign4650_e5911: f64 = (locals.var_inv_wl * p.p1067);
        let assign4650_e5912: f64 = (assign4650_e5908 + assign4650_e5911);
        locals.var_a2_i = assign4650_e5912;
        locals.var_a2_i_rv = 0.0;

        let assign4660_e5916: f64 = (locals.var_inv_l * p.p1069);
        let assign4660_e5917: f64 = (p.p1068 + assign4660_e5916);
        let assign4660_e5920: f64 = (locals.var_inv_nfin * p.p1070);
        let assign4660_e5921: f64 = (assign4660_e5917 + assign4660_e5920);
        let assign4660_e5924: f64 = (locals.var_inv_lnfin * p.p1071);
        let assign4660_e5925: f64 = (assign4660_e5921 + assign4660_e5924);
        let assign4660_e5928: f64 = (locals.var_inv_w * p.p1072);
        let assign4660_e5929: f64 = (assign4660_e5925 + assign4660_e5928);
        let assign4660_e5932: f64 = (locals.var_inv_wl * p.p1073);
        let assign4660_e5933: f64 = (assign4660_e5929 + assign4660_e5932);
        locals.var_a21_i = assign4660_e5933;
        locals.var_a21_i_rv = 0.0;

        let assign4670_e5937: f64 = (locals.var_inv_l * p.p926);
        let assign4670_e5938: f64 = (p.p925 + assign4670_e5937);
        let assign4670_e5941: f64 = (locals.var_inv_nfin * p.p927);
        let assign4670_e5942: f64 = (assign4670_e5938 + assign4670_e5941);
        let assign4670_e5945: f64 = (locals.var_inv_lnfin * p.p928);
        let assign4670_e5946: f64 = (assign4670_e5942 + assign4670_e5945);
        let assign4670_e5949: f64 = (locals.var_inv_w * p.p929);
        let assign4670_e5950: f64 = (assign4670_e5946 + assign4670_e5949);
        let assign4670_e5953: f64 = (locals.var_inv_wl * p.p930);
        let assign4670_e5954: f64 = (assign4670_e5950 + assign4670_e5953);
        locals.var_rdsw_i = assign4670_e5954;
        locals.var_rdsw_i_dn0 = 0.0;
        locals.var_rdsw_i_dn2 = 0.0;
        locals.var_rdsw_i_dn3 = 0.0;
        locals.var_rdsw_i_dn4 = 0.0;
        locals.var_rdsw_i_dn5 = 0.0;
        locals.var_rdsw_i_dn6 = 0.0;
        locals.var_rdsw_i_dn7 = 0.0;
        locals.var_rdsw_i_dn8 = 0.0;
        locals.var_rdsw_i_dn9 = 0.0;
        locals.var_rdsw_i_dn10 = 0.0;
        locals.var_rdsw_i_dn11 = 0.0;
        locals.var_rdsw_i_dn13 = 0.0;
        locals.var_rdsw_i_dn14 = 0.0;
        locals.var_rdsw_i_rv = 0.0;

        let assign4680_e5958: f64 = (locals.var_inv_l * p.p932);
        let assign4680_e5959: f64 = (p.p931 + assign4680_e5958);
        let assign4680_e5962: f64 = (locals.var_inv_nfin * p.p933);
        let assign4680_e5963: f64 = (assign4680_e5959 + assign4680_e5962);
        let assign4680_e5966: f64 = (locals.var_inv_lnfin * p.p934);
        let assign4680_e5967: f64 = (assign4680_e5963 + assign4680_e5966);
        let assign4680_e5970: f64 = (locals.var_inv_w * p.p935);
        let assign4680_e5971: f64 = (assign4680_e5967 + assign4680_e5970);
        let assign4680_e5974: f64 = (locals.var_inv_wl * p.p936);
        let assign4680_e5975: f64 = (assign4680_e5971 + assign4680_e5974);
        locals.var_rsw_i = assign4680_e5975;
        locals.var_rsw_i_dn0 = 0.0;
        locals.var_rsw_i_dn2 = 0.0;
        locals.var_rsw_i_dn3 = 0.0;
        locals.var_rsw_i_dn4 = 0.0;
        locals.var_rsw_i_dn5 = 0.0;
        locals.var_rsw_i_dn6 = 0.0;
        locals.var_rsw_i_dn7 = 0.0;
        locals.var_rsw_i_dn8 = 0.0;
        locals.var_rsw_i_dn9 = 0.0;
        locals.var_rsw_i_dn10 = 0.0;
        locals.var_rsw_i_dn11 = 0.0;
        locals.var_rsw_i_dn13 = 0.0;
        locals.var_rsw_i_dn14 = 0.0;
        locals.var_rsw_i_rv = 0.0;

        let assign4690_e5979: f64 = (locals.var_inv_l * p.p938);
        let assign4690_e5980: f64 = (p.p937 + assign4690_e5979);
        let assign4690_e5983: f64 = (locals.var_inv_nfin * p.p939);
        let assign4690_e5984: f64 = (assign4690_e5980 + assign4690_e5983);
        let assign4690_e5987: f64 = (locals.var_inv_lnfin * p.p940);
        let assign4690_e5988: f64 = (assign4690_e5984 + assign4690_e5987);
        let assign4690_e5991: f64 = (locals.var_inv_w * p.p941);
        let assign4690_e5992: f64 = (assign4690_e5988 + assign4690_e5991);
        let assign4690_e5995: f64 = (locals.var_inv_wl * p.p942);
        let assign4690_e5996: f64 = (assign4690_e5992 + assign4690_e5995);
        locals.var_rdw_i = assign4690_e5996;
        locals.var_rdw_i_dn0 = 0.0;
        locals.var_rdw_i_dn2 = 0.0;
        locals.var_rdw_i_dn3 = 0.0;
        locals.var_rdw_i_dn4 = 0.0;
        locals.var_rdw_i_dn5 = 0.0;
        locals.var_rdw_i_dn6 = 0.0;
        locals.var_rdw_i_dn7 = 0.0;
        locals.var_rdw_i_dn8 = 0.0;
        locals.var_rdw_i_dn9 = 0.0;
        locals.var_rdw_i_dn10 = 0.0;
        locals.var_rdw_i_dn11 = 0.0;
        locals.var_rdw_i_dn13 = 0.0;
        locals.var_rdw_i_dn14 = 0.0;
        locals.var_rdw_i_rv = 0.0;

        let assign4700_e6000: f64 = (locals.var_inv_l * p.p950);
        let assign4700_e6001: f64 = (p.p949 + assign4700_e6000);
        let assign4700_e6004: f64 = (locals.var_inv_nfin * p.p951);
        let assign4700_e6005: f64 = (assign4700_e6001 + assign4700_e6004);
        let assign4700_e6008: f64 = (locals.var_inv_lnfin * p.p952);
        let assign4700_e6009: f64 = (assign4700_e6005 + assign4700_e6008);
        let assign4700_e6012: f64 = (locals.var_inv_w * p.p953);
        let assign4700_e6013: f64 = (assign4700_e6009 + assign4700_e6012);
        let assign4700_e6016: f64 = (locals.var_inv_wl * p.p954);
        let assign4700_e6017: f64 = (assign4700_e6013 + assign4700_e6016);
        locals.var_prwgd_i = assign4700_e6017;
        locals.var_prwgd_i_rv = 0.0;

        let assign4710_e6021: f64 = (locals.var_inv_l * p.p944);
        let assign4710_e6022: f64 = (p.p943 + assign4710_e6021);
        let assign4710_e6025: f64 = (locals.var_inv_nfin * p.p945);
        let assign4710_e6026: f64 = (assign4710_e6022 + assign4710_e6025);
        let assign4710_e6029: f64 = (locals.var_inv_lnfin * p.p946);
        let assign4710_e6030: f64 = (assign4710_e6026 + assign4710_e6029);
        let assign4710_e6033: f64 = (locals.var_inv_w * p.p947);
        let assign4710_e6034: f64 = (assign4710_e6030 + assign4710_e6033);
        let assign4710_e6037: f64 = (locals.var_inv_wl * p.p948);
        let assign4710_e6038: f64 = (assign4710_e6034 + assign4710_e6037);
        locals.var_prwgs_i = assign4710_e6038;
        locals.var_prwgs_i_rv = 0.0;

        let assign4720_e6042: f64 = (locals.var_inv_l * p.p956);
        let assign4720_e6043: f64 = (p.p955 + assign4720_e6042);
        let assign4720_e6046: f64 = (locals.var_inv_nfin * p.p957);
        let assign4720_e6047: f64 = (assign4720_e6043 + assign4720_e6046);
        let assign4720_e6050: f64 = (locals.var_inv_lnfin * p.p958);
        let assign4720_e6051: f64 = (assign4720_e6047 + assign4720_e6050);
        let assign4720_e6054: f64 = (locals.var_inv_w * p.p959);
        let assign4720_e6055: f64 = (assign4720_e6051 + assign4720_e6054);
        let assign4720_e6058: f64 = (locals.var_inv_wl * p.p960);
        let assign4720_e6059: f64 = (assign4720_e6055 + assign4720_e6058);
        locals.var_wr_i = assign4720_e6059;
        locals.var_wr_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4730_e6063: f64 = (locals.var_inv_l * p.p986);
        let assign4730_e6064: f64 = (p.p985 + assign4730_e6063);
        let assign4730_e6067: f64 = (locals.var_inv_nfin * p.p987);
        let assign4730_e6068: f64 = (assign4730_e6064 + assign4730_e6067);
        let assign4730_e6071: f64 = (locals.var_inv_lnfin * p.p988);
        let assign4730_e6072: f64 = (assign4730_e6068 + assign4730_e6071);
        let assign4730_e6075: f64 = (locals.var_inv_w * p.p989);
        let assign4730_e6076: f64 = (assign4730_e6072 + assign4730_e6075);
        let assign4730_e6079: f64 = (locals.var_inv_wl * p.p990);
        let assign4730_e6080: f64 = (assign4730_e6076 + assign4730_e6079);
        locals.var_pdibl1_i = assign4730_e6080;
        locals.var_pdibl1_i_rv = 0.0;

        let assign4740_e6084: f64 = (locals.var_inv_l * p.p992);
        let assign4740_e6085: f64 = (p.p991 + assign4740_e6084);
        let assign4740_e6088: f64 = (locals.var_inv_nfin * p.p993);
        let assign4740_e6089: f64 = (assign4740_e6085 + assign4740_e6088);
        let assign4740_e6092: f64 = (locals.var_inv_lnfin * p.p994);
        let assign4740_e6093: f64 = (assign4740_e6089 + assign4740_e6092);
        let assign4740_e6096: f64 = (locals.var_inv_w * p.p995);
        let assign4740_e6097: f64 = (assign4740_e6093 + assign4740_e6096);
        let assign4740_e6100: f64 = (locals.var_inv_wl * p.p996);
        let assign4740_e6101: f64 = (assign4740_e6097 + assign4740_e6100);
        locals.var_pdibl2_i = assign4740_e6101;
        locals.var_pdibl2_i_rv = 0.0;

        let assign4750_e6105: f64 = (locals.var_inv_l * p.p1010);
        let assign4750_e6106: f64 = (p.p1009 + assign4750_e6105);
        let assign4750_e6109: f64 = (locals.var_inv_nfin * p.p1011);
        let assign4750_e6110: f64 = (assign4750_e6106 + assign4750_e6109);
        let assign4750_e6113: f64 = (locals.var_inv_lnfin * p.p1012);
        let assign4750_e6114: f64 = (assign4750_e6110 + assign4750_e6113);
        let assign4750_e6117: f64 = (locals.var_inv_w * p.p1013);
        let assign4750_e6118: f64 = (assign4750_e6114 + assign4750_e6117);
        let assign4750_e6121: f64 = (locals.var_inv_wl * p.p1014);
        let assign4750_e6122: f64 = (assign4750_e6118 + assign4750_e6121);
        locals.var_drout_i = assign4750_e6122;
        locals.var_drout_i_rv = 0.0;

        let assign4760_e6126: f64 = (locals.var_inv_l * p.p1016);
        let assign4760_e6127: f64 = (p.p1015 + assign4760_e6126);
        let assign4760_e6130: f64 = (locals.var_inv_nfin * p.p1017);
        let assign4760_e6131: f64 = (assign4760_e6127 + assign4760_e6130);
        let assign4760_e6134: f64 = (locals.var_inv_lnfin * p.p1018);
        let assign4760_e6135: f64 = (assign4760_e6131 + assign4760_e6134);
        let assign4760_e6138: f64 = (locals.var_inv_w * p.p1019);
        let assign4760_e6139: f64 = (assign4760_e6135 + assign4760_e6138);
        let assign4760_e6142: f64 = (locals.var_inv_wl * p.p1020);
        let assign4760_e6143: f64 = (assign4760_e6139 + assign4760_e6142);
        locals.var_pvag_i = assign4760_e6143;
        locals.var_pvag_i_rv = 0.0;

        let assign4770_e6147: f64 = (locals.var_inv_l * p.p1120);
        let assign4770_e6148: f64 = (p.p1119 + assign4770_e6147);
        let assign4770_e6151: f64 = (locals.var_inv_nfin * p.p1121);
        let assign4770_e6152: f64 = (assign4770_e6148 + assign4770_e6151);
        let assign4770_e6155: f64 = (locals.var_inv_lnfin * p.p1122);
        let assign4770_e6156: f64 = (assign4770_e6152 + assign4770_e6155);
        let assign4770_e6159: f64 = (locals.var_inv_w * p.p1123);
        let assign4770_e6160: f64 = (assign4770_e6156 + assign4770_e6159);
        let assign4770_e6163: f64 = (locals.var_inv_wl * p.p1124);
        let assign4770_e6164: f64 = (assign4770_e6160 + assign4770_e6163);
        locals.var_aigbinv_i = assign4770_e6164;
        locals.var_aigbinv_i_rv = 0.0;

        let assign4780_e6168: f64 = (locals.var_inv_l * p.p1126);
        let assign4780_e6169: f64 = (p.p1125 + assign4780_e6168);
        let assign4780_e6172: f64 = (locals.var_inv_nfin * p.p1127);
        let assign4780_e6173: f64 = (assign4780_e6169 + assign4780_e6172);
        let assign4780_e6176: f64 = (locals.var_inv_lnfin * p.p1128);
        let assign4780_e6177: f64 = (assign4780_e6173 + assign4780_e6176);
        let assign4780_e6180: f64 = (locals.var_inv_w * p.p1129);
        let assign4780_e6181: f64 = (assign4780_e6177 + assign4780_e6180);
        let assign4780_e6184: f64 = (locals.var_inv_wl * p.p1130);
        let assign4780_e6185: f64 = (assign4780_e6181 + assign4780_e6184);
        locals.var_aigbinv1_i = assign4780_e6185;
        locals.var_aigbinv1_i_rv = 0.0;

        let assign4790_e6189: f64 = (locals.var_inv_l * p.p1132);
        let assign4790_e6190: f64 = (p.p1131 + assign4790_e6189);
        let assign4790_e6193: f64 = (locals.var_inv_nfin * p.p1133);
        let assign4790_e6194: f64 = (assign4790_e6190 + assign4790_e6193);
        let assign4790_e6197: f64 = (locals.var_inv_lnfin * p.p1134);
        let assign4790_e6198: f64 = (assign4790_e6194 + assign4790_e6197);
        let assign4790_e6201: f64 = (locals.var_inv_w * p.p1135);
        let assign4790_e6202: f64 = (assign4790_e6198 + assign4790_e6201);
        let assign4790_e6205: f64 = (locals.var_inv_wl * p.p1136);
        let assign4790_e6206: f64 = (assign4790_e6202 + assign4790_e6205);
        locals.var_bigbinv_i = assign4790_e6206;
        locals.var_bigbinv_i_rv = 0.0;

        let assign4800_e6210: f64 = (locals.var_inv_l * p.p1138);
        let assign4800_e6211: f64 = (p.p1137 + assign4800_e6210);
        let assign4800_e6214: f64 = (locals.var_inv_nfin * p.p1139);
        let assign4800_e6215: f64 = (assign4800_e6211 + assign4800_e6214);
        let assign4800_e6218: f64 = (locals.var_inv_lnfin * p.p1140);
        let assign4800_e6219: f64 = (assign4800_e6215 + assign4800_e6218);
        let assign4800_e6222: f64 = (locals.var_inv_w * p.p1141);
        let assign4800_e6223: f64 = (assign4800_e6219 + assign4800_e6222);
        let assign4800_e6226: f64 = (locals.var_inv_wl * p.p1142);
        let assign4800_e6227: f64 = (assign4800_e6223 + assign4800_e6226);
        locals.var_cigbinv_i = assign4800_e6227;
        locals.var_cigbinv_i_rv = 0.0;

        let assign4810_e6231: f64 = (locals.var_inv_l * p.p1144);
        let assign4810_e6232: f64 = (p.p1143 + assign4810_e6231);
        let assign4810_e6235: f64 = (locals.var_inv_nfin * p.p1145);
        let assign4810_e6236: f64 = (assign4810_e6232 + assign4810_e6235);
        let assign4810_e6239: f64 = (locals.var_inv_lnfin * p.p1146);
        let assign4810_e6240: f64 = (assign4810_e6236 + assign4810_e6239);
        let assign4810_e6243: f64 = (locals.var_inv_w * p.p1147);
        let assign4810_e6244: f64 = (assign4810_e6240 + assign4810_e6243);
        let assign4810_e6247: f64 = (locals.var_inv_wl * p.p1148);
        let assign4810_e6248: f64 = (assign4810_e6244 + assign4810_e6247);
        locals.var_eigbinv_i = assign4810_e6248;
        locals.var_eigbinv_i_rv = 0.0;

        let assign4820_e6252: f64 = (locals.var_inv_l * p.p1150);
        let assign4820_e6253: f64 = (p.p1149 + assign4820_e6252);
        let assign4820_e6256: f64 = (locals.var_inv_nfin * p.p1151);
        let assign4820_e6257: f64 = (assign4820_e6253 + assign4820_e6256);
        let assign4820_e6260: f64 = (locals.var_inv_lnfin * p.p1152);
        let assign4820_e6261: f64 = (assign4820_e6257 + assign4820_e6260);
        let assign4820_e6264: f64 = (locals.var_inv_w * p.p1153);
        let assign4820_e6265: f64 = (assign4820_e6261 + assign4820_e6264);
        let assign4820_e6268: f64 = (locals.var_inv_wl * p.p1154);
        let assign4820_e6269: f64 = (assign4820_e6265 + assign4820_e6268);
        locals.var_nigbinv_i = assign4820_e6269;
        locals.var_nigbinv_i_rv = 0.0;

        let assign4830_e6273: f64 = (locals.var_inv_l * p.p1156);
        let assign4830_e6274: f64 = (p.p1155 + assign4830_e6273);
        let assign4830_e6277: f64 = (locals.var_inv_nfin * p.p1157);
        let assign4830_e6278: f64 = (assign4830_e6274 + assign4830_e6277);
        let assign4830_e6281: f64 = (locals.var_inv_lnfin * p.p1158);
        let assign4830_e6282: f64 = (assign4830_e6278 + assign4830_e6281);
        let assign4830_e6285: f64 = (locals.var_inv_w * p.p1159);
        let assign4830_e6286: f64 = (assign4830_e6282 + assign4830_e6285);
        let assign4830_e6289: f64 = (locals.var_inv_wl * p.p1160);
        let assign4830_e6290: f64 = (assign4830_e6286 + assign4830_e6289);
        locals.var_aigbacc_i = assign4830_e6290;
        locals.var_aigbacc_i_rv = 0.0;

        let assign4840_e6294: f64 = (locals.var_inv_l * p.p1162);
        let assign4840_e6295: f64 = (p.p1161 + assign4840_e6294);
        let assign4840_e6298: f64 = (locals.var_inv_nfin * p.p1163);
        let assign4840_e6299: f64 = (assign4840_e6295 + assign4840_e6298);
        let assign4840_e6302: f64 = (locals.var_inv_lnfin * p.p1164);
        let assign4840_e6303: f64 = (assign4840_e6299 + assign4840_e6302);
        let assign4840_e6306: f64 = (locals.var_inv_w * p.p1165);
        let assign4840_e6307: f64 = (assign4840_e6303 + assign4840_e6306);
        let assign4840_e6310: f64 = (locals.var_inv_wl * p.p1166);
        let assign4840_e6311: f64 = (assign4840_e6307 + assign4840_e6310);
        locals.var_aigbacc1_i = assign4840_e6311;
        locals.var_aigbacc1_i_rv = 0.0;

        let assign4850_e6315: f64 = (locals.var_inv_l * p.p1168);
        let assign4850_e6316: f64 = (p.p1167 + assign4850_e6315);
        let assign4850_e6319: f64 = (locals.var_inv_nfin * p.p1169);
        let assign4850_e6320: f64 = (assign4850_e6316 + assign4850_e6319);
        let assign4850_e6323: f64 = (locals.var_inv_lnfin * p.p1170);
        let assign4850_e6324: f64 = (assign4850_e6320 + assign4850_e6323);
        let assign4850_e6327: f64 = (locals.var_inv_w * p.p1171);
        let assign4850_e6328: f64 = (assign4850_e6324 + assign4850_e6327);
        let assign4850_e6331: f64 = (locals.var_inv_wl * p.p1172);
        let assign4850_e6332: f64 = (assign4850_e6328 + assign4850_e6331);
        locals.var_bigbacc_i = assign4850_e6332;
        locals.var_bigbacc_i_rv = 0.0;

        let assign4860_e6336: f64 = (locals.var_inv_l * p.p1174);
        let assign4860_e6337: f64 = (p.p1173 + assign4860_e6336);
        let assign4860_e6340: f64 = (locals.var_inv_nfin * p.p1175);
        let assign4860_e6341: f64 = (assign4860_e6337 + assign4860_e6340);
        let assign4860_e6344: f64 = (locals.var_inv_lnfin * p.p1176);
        let assign4860_e6345: f64 = (assign4860_e6341 + assign4860_e6344);
        let assign4860_e6348: f64 = (locals.var_inv_w * p.p1177);
        let assign4860_e6349: f64 = (assign4860_e6345 + assign4860_e6348);
        let assign4860_e6352: f64 = (locals.var_inv_wl * p.p1178);
        let assign4860_e6353: f64 = (assign4860_e6349 + assign4860_e6352);
        locals.var_cigbacc_i = assign4860_e6353;
        locals.var_cigbacc_i_rv = 0.0;

        let assign4870_e6357: f64 = (locals.var_inv_l * p.p1180);
        let assign4870_e6358: f64 = (p.p1179 + assign4870_e6357);
        let assign4870_e6361: f64 = (locals.var_inv_nfin * p.p1181);
        let assign4870_e6362: f64 = (assign4870_e6358 + assign4870_e6361);
        let assign4870_e6365: f64 = (locals.var_inv_lnfin * p.p1182);
        let assign4870_e6366: f64 = (assign4870_e6362 + assign4870_e6365);
        let assign4870_e6369: f64 = (locals.var_inv_w * p.p1183);
        let assign4870_e6370: f64 = (assign4870_e6366 + assign4870_e6369);
        let assign4870_e6373: f64 = (locals.var_inv_wl * p.p1184);
        let assign4870_e6374: f64 = (assign4870_e6370 + assign4870_e6373);
        locals.var_nigbacc_i = assign4870_e6374;
        locals.var_nigbacc_i_rv = 0.0;

        let assign4880_e6378: f64 = (locals.var_inv_l * p.p1186);
        let assign4880_e6379: f64 = (p.p1185 + assign4880_e6378);
        let assign4880_e6382: f64 = (locals.var_inv_nfin * p.p1187);
        let assign4880_e6383: f64 = (assign4880_e6379 + assign4880_e6382);
        let assign4880_e6386: f64 = (locals.var_inv_lnfin * p.p1188);
        let assign4880_e6387: f64 = (assign4880_e6383 + assign4880_e6386);
        let assign4880_e6390: f64 = (locals.var_inv_w * p.p1189);
        let assign4880_e6391: f64 = (assign4880_e6387 + assign4880_e6390);
        let assign4880_e6394: f64 = (locals.var_inv_wl * p.p1190);
        let assign4880_e6395: f64 = (assign4880_e6391 + assign4880_e6394);
        locals.var_aigc_i = assign4880_e6395;
        locals.var_aigc_i_rv = 0.0;

        let assign4890_e6399: f64 = (locals.var_inv_l * p.p1192);
        let assign4890_e6400: f64 = (p.p1191 + assign4890_e6399);
        let assign4890_e6403: f64 = (locals.var_inv_nfin * p.p1193);
        let assign4890_e6404: f64 = (assign4890_e6400 + assign4890_e6403);
        let assign4890_e6407: f64 = (locals.var_inv_lnfin * p.p1194);
        let assign4890_e6408: f64 = (assign4890_e6404 + assign4890_e6407);
        let assign4890_e6411: f64 = (locals.var_inv_w * p.p1195);
        let assign4890_e6412: f64 = (assign4890_e6408 + assign4890_e6411);
        let assign4890_e6415: f64 = (locals.var_inv_wl * p.p1196);
        let assign4890_e6416: f64 = (assign4890_e6412 + assign4890_e6415);
        locals.var_aigc1_i = assign4890_e6416;
        locals.var_aigc1_i_rv = 0.0;

        let assign4900_e6420: f64 = (locals.var_inv_l * p.p1198);
        let assign4900_e6421: f64 = (p.p1197 + assign4900_e6420);
        let assign4900_e6424: f64 = (locals.var_inv_nfin * p.p1199);
        let assign4900_e6425: f64 = (assign4900_e6421 + assign4900_e6424);
        let assign4900_e6428: f64 = (locals.var_inv_lnfin * p.p1200);
        let assign4900_e6429: f64 = (assign4900_e6425 + assign4900_e6428);
        let assign4900_e6432: f64 = (locals.var_inv_w * p.p1201);
        let assign4900_e6433: f64 = (assign4900_e6429 + assign4900_e6432);
        let assign4900_e6436: f64 = (locals.var_inv_wl * p.p1202);
        let assign4900_e6437: f64 = (assign4900_e6433 + assign4900_e6436);
        locals.var_bigc_i = assign4900_e6437;
        locals.var_bigc_i_rv = 0.0;

        let assign4910_e6441: f64 = (locals.var_inv_l * p.p1204);
        let assign4910_e6442: f64 = (p.p1203 + assign4910_e6441);
        let assign4910_e6445: f64 = (locals.var_inv_nfin * p.p1205);
        let assign4910_e6446: f64 = (assign4910_e6442 + assign4910_e6445);
        let assign4910_e6449: f64 = (locals.var_inv_lnfin * p.p1206);
        let assign4910_e6450: f64 = (assign4910_e6446 + assign4910_e6449);
        let assign4910_e6453: f64 = (locals.var_inv_w * p.p1207);
        let assign4910_e6454: f64 = (assign4910_e6450 + assign4910_e6453);
        let assign4910_e6457: f64 = (locals.var_inv_wl * p.p1208);
        let assign4910_e6458: f64 = (assign4910_e6454 + assign4910_e6457);
        locals.var_cigc_i = assign4910_e6458;
        locals.var_cigc_i_rv = 0.0;

        let assign4920_e6462: f64 = (locals.var_inv_l * p.p1210);
        let assign4920_e6463: f64 = (p.p1209 + assign4920_e6462);
        let assign4920_e6466: f64 = (locals.var_inv_nfin * p.p1211);
        let assign4920_e6467: f64 = (assign4920_e6463 + assign4920_e6466);
        let assign4920_e6470: f64 = (locals.var_inv_lnfin * p.p1212);
        let assign4920_e6471: f64 = (assign4920_e6467 + assign4920_e6470);
        let assign4920_e6474: f64 = (locals.var_inv_w * p.p1213);
        let assign4920_e6475: f64 = (assign4920_e6471 + assign4920_e6474);
        let assign4920_e6478: f64 = (locals.var_inv_wl * p.p1214);
        let assign4920_e6479: f64 = (assign4920_e6475 + assign4920_e6478);
        locals.var_pigcd_i = assign4920_e6479;
        locals.var_pigcd_i_rv = 0.0;

        let assign4930_e6483: f64 = (locals.var_inv_l * p.p1216);
        let assign4930_e6484: f64 = (p.p1215 + assign4930_e6483);
        let assign4930_e6487: f64 = (locals.var_inv_nfin * p.p1217);
        let assign4930_e6488: f64 = (assign4930_e6484 + assign4930_e6487);
        let assign4930_e6491: f64 = (locals.var_inv_lnfin * p.p1218);
        let assign4930_e6492: f64 = (assign4930_e6488 + assign4930_e6491);
        let assign4930_e6495: f64 = (locals.var_inv_w * p.p1219);
        let assign4930_e6496: f64 = (assign4930_e6492 + assign4930_e6495);
        let assign4930_e6499: f64 = (locals.var_inv_wl * p.p1220);
        let assign4930_e6500: f64 = (assign4930_e6496 + assign4930_e6499);
        locals.var_aigs_i = assign4930_e6500;
        locals.var_aigs_i_rv = 0.0;

        let assign4940_e6504: f64 = (locals.var_inv_l * p.p1222);
        let assign4940_e6505: f64 = (p.p1221 + assign4940_e6504);
        let assign4940_e6508: f64 = (locals.var_inv_nfin * p.p1223);
        let assign4940_e6509: f64 = (assign4940_e6505 + assign4940_e6508);
        let assign4940_e6512: f64 = (locals.var_inv_lnfin * p.p1224);
        let assign4940_e6513: f64 = (assign4940_e6509 + assign4940_e6512);
        let assign4940_e6516: f64 = (locals.var_inv_w * p.p1225);
        let assign4940_e6517: f64 = (assign4940_e6513 + assign4940_e6516);
        let assign4940_e6520: f64 = (locals.var_inv_wl * p.p1226);
        let assign4940_e6521: f64 = (assign4940_e6517 + assign4940_e6520);
        locals.var_aigs1_i = assign4940_e6521;
        locals.var_aigs1_i_rv = 0.0;

        let assign4950_e6525: f64 = (locals.var_inv_l * p.p1228);
        let assign4950_e6526: f64 = (p.p1227 + assign4950_e6525);
        let assign4950_e6529: f64 = (locals.var_inv_nfin * p.p1229);
        let assign4950_e6530: f64 = (assign4950_e6526 + assign4950_e6529);
        let assign4950_e6533: f64 = (locals.var_inv_lnfin * p.p1230);
        let assign4950_e6534: f64 = (assign4950_e6530 + assign4950_e6533);
        let assign4950_e6537: f64 = (locals.var_inv_w * p.p1231);
        let assign4950_e6538: f64 = (assign4950_e6534 + assign4950_e6537);
        let assign4950_e6541: f64 = (locals.var_inv_wl * p.p1232);
        let assign4950_e6542: f64 = (assign4950_e6538 + assign4950_e6541);
        locals.var_bigs_i = assign4950_e6542;
        locals.var_bigs_i_rv = 0.0;

        let assign4960_e6546: f64 = (locals.var_inv_l * p.p1234);
        let assign4960_e6547: f64 = (p.p1233 + assign4960_e6546);
        let assign4960_e6550: f64 = (locals.var_inv_nfin * p.p1235);
        let assign4960_e6551: f64 = (assign4960_e6547 + assign4960_e6550);
        let assign4960_e6554: f64 = (locals.var_inv_lnfin * p.p1236);
        let assign4960_e6555: f64 = (assign4960_e6551 + assign4960_e6554);
        let assign4960_e6558: f64 = (locals.var_inv_w * p.p1237);
        let assign4960_e6559: f64 = (assign4960_e6555 + assign4960_e6558);
        let assign4960_e6562: f64 = (locals.var_inv_wl * p.p1238);
        let assign4960_e6563: f64 = (assign4960_e6559 + assign4960_e6562);
        locals.var_cigs_i = assign4960_e6563;
        locals.var_cigs_i_rv = 0.0;

        let assign4970_e6567: f64 = (locals.var_inv_l * p.p1240);
        let assign4970_e6568: f64 = (p.p1239 + assign4970_e6567);
        let assign4970_e6571: f64 = (locals.var_inv_nfin * p.p1241);
        let assign4970_e6572: f64 = (assign4970_e6568 + assign4970_e6571);
        let assign4970_e6575: f64 = (locals.var_inv_lnfin * p.p1242);
        let assign4970_e6576: f64 = (assign4970_e6572 + assign4970_e6575);
        let assign4970_e6579: f64 = (locals.var_inv_w * p.p1243);
        let assign4970_e6580: f64 = (assign4970_e6576 + assign4970_e6579);
        let assign4970_e6583: f64 = (locals.var_inv_wl * p.p1244);
        let assign4970_e6584: f64 = (assign4970_e6580 + assign4970_e6583);
        locals.var_aigd_i = assign4970_e6584;
        locals.var_aigd_i_rv = 0.0;

        let assign4980_e6588: f64 = (locals.var_inv_l * p.p1246);
        let assign4980_e6589: f64 = (p.p1245 + assign4980_e6588);
        let assign4980_e6592: f64 = (locals.var_inv_nfin * p.p1247);
        let assign4980_e6593: f64 = (assign4980_e6589 + assign4980_e6592);
        let assign4980_e6596: f64 = (locals.var_inv_lnfin * p.p1248);
        let assign4980_e6597: f64 = (assign4980_e6593 + assign4980_e6596);
        let assign4980_e6600: f64 = (locals.var_inv_w * p.p1249);
        let assign4980_e6601: f64 = (assign4980_e6597 + assign4980_e6600);
        let assign4980_e6604: f64 = (locals.var_inv_wl * p.p1250);
        let assign4980_e6605: f64 = (assign4980_e6601 + assign4980_e6604);
        locals.var_aigd1_i = assign4980_e6605;
        locals.var_aigd1_i_rv = 0.0;

        let assign4990_e6609: f64 = (locals.var_inv_l * p.p1252);
        let assign4990_e6610: f64 = (p.p1251 + assign4990_e6609);
        let assign4990_e6613: f64 = (locals.var_inv_nfin * p.p1253);
        let assign4990_e6614: f64 = (assign4990_e6610 + assign4990_e6613);
        let assign4990_e6617: f64 = (locals.var_inv_lnfin * p.p1254);
        let assign4990_e6618: f64 = (assign4990_e6614 + assign4990_e6617);
        let assign4990_e6621: f64 = (locals.var_inv_w * p.p1255);
        let assign4990_e6622: f64 = (assign4990_e6618 + assign4990_e6621);
        let assign4990_e6625: f64 = (locals.var_inv_wl * p.p1256);
        let assign4990_e6626: f64 = (assign4990_e6622 + assign4990_e6625);
        locals.var_bigd_i = assign4990_e6626;
        locals.var_bigd_i_rv = 0.0;

        let assign5000_e6630: f64 = (locals.var_inv_l * p.p1258);
        let assign5000_e6631: f64 = (p.p1257 + assign5000_e6630);
        let assign5000_e6634: f64 = (locals.var_inv_nfin * p.p1259);
        let assign5000_e6635: f64 = (assign5000_e6631 + assign5000_e6634);
        let assign5000_e6638: f64 = (locals.var_inv_lnfin * p.p1260);
        let assign5000_e6639: f64 = (assign5000_e6635 + assign5000_e6638);
        let assign5000_e6642: f64 = (locals.var_inv_w * p.p1261);
        let assign5000_e6643: f64 = (assign5000_e6639 + assign5000_e6642);
        let assign5000_e6646: f64 = (locals.var_inv_wl * p.p1262);
        let assign5000_e6647: f64 = (assign5000_e6643 + assign5000_e6646);
        locals.var_cigd_i = assign5000_e6647;
        locals.var_cigd_i_rv = 0.0;

        let assign5020_e6672: f64 = (locals.var_inv_l * p.p1264);
        let assign5020_e6673: f64 = (p.p1263 + assign5020_e6672);
        let assign5020_e6676: f64 = (locals.var_inv_nfin * p.p1265);
        let assign5020_e6677: f64 = (assign5020_e6673 + assign5020_e6676);
        let assign5020_e6680: f64 = (locals.var_inv_lnfin * p.p1266);
        let assign5020_e6681: f64 = (assign5020_e6677 + assign5020_e6680);
        let assign5020_e6684: f64 = (locals.var_inv_w * p.p1267);
        let assign5020_e6685: f64 = (assign5020_e6681 + assign5020_e6684);
        let assign5020_e6688: f64 = (locals.var_inv_wl * p.p1268);
        let assign5020_e6689: f64 = (assign5020_e6685 + assign5020_e6688);
        locals.var_poxedge_i = assign5020_e6689;
        locals.var_poxedge_i_rv = 0.0;

        let assign5030_e6693: f64 = (locals.var_inv_l * p.p1270);
        let assign5030_e6694: f64 = (p.p1269 + assign5030_e6693);
        let assign5030_e6697: f64 = (locals.var_inv_nfin * p.p1271);
        let assign5030_e6698: f64 = (assign5030_e6694 + assign5030_e6697);
        let assign5030_e6701: f64 = (locals.var_inv_lnfin * p.p1272);
        let assign5030_e6702: f64 = (assign5030_e6698 + assign5030_e6701);
        let assign5030_e6705: f64 = (locals.var_inv_w * p.p1273);
        let assign5030_e6706: f64 = (assign5030_e6702 + assign5030_e6705);
        let assign5030_e6709: f64 = (locals.var_inv_wl * p.p1274);
        let assign5030_e6710: f64 = (assign5030_e6706 + assign5030_e6709);
        locals.var_agidl_i = assign5030_e6710;
        locals.var_agidl_i_rv = 0.0;

        let assign5040_e6714: f64 = (locals.var_inv_l * p.p1276);
        let assign5040_e6715: f64 = (p.p1275 + assign5040_e6714);
        let assign5040_e6718: f64 = (locals.var_inv_nfin * p.p1277);
        let assign5040_e6719: f64 = (assign5040_e6715 + assign5040_e6718);
        let assign5040_e6722: f64 = (locals.var_inv_lnfin * p.p1278);
        let assign5040_e6723: f64 = (assign5040_e6719 + assign5040_e6722);
        let assign5040_e6726: f64 = (locals.var_inv_w * p.p1279);
        let assign5040_e6727: f64 = (assign5040_e6723 + assign5040_e6726);
        let assign5040_e6730: f64 = (locals.var_inv_wl * p.p1280);
        let assign5040_e6731: f64 = (assign5040_e6727 + assign5040_e6730);
        locals.var_bgidl_i = assign5040_e6731;
        locals.var_bgidl_i_rv = 0.0;

        let assign5050_e6735: f64 = (locals.var_inv_l * p.p1282);
        let assign5050_e6736: f64 = (p.p1281 + assign5050_e6735);
        let assign5050_e6739: f64 = (locals.var_inv_nfin * p.p1283);
        let assign5050_e6740: f64 = (assign5050_e6736 + assign5050_e6739);
        let assign5050_e6743: f64 = (locals.var_inv_lnfin * p.p1284);
        let assign5050_e6744: f64 = (assign5050_e6740 + assign5050_e6743);
        let assign5050_e6747: f64 = (locals.var_inv_w * p.p1285);
        let assign5050_e6748: f64 = (assign5050_e6744 + assign5050_e6747);
        let assign5050_e6751: f64 = (locals.var_inv_wl * p.p1286);
        let assign5050_e6752: f64 = (assign5050_e6748 + assign5050_e6751);
        locals.var_cgidl_i = assign5050_e6752;
        locals.var_cgidl_i_rv = 0.0;

        let assign5060_e6756: f64 = (locals.var_inv_l * p.p1288);
        let assign5060_e6757: f64 = (p.p1287 + assign5060_e6756);
        let assign5060_e6760: f64 = (locals.var_inv_nfin * p.p1289);
        let assign5060_e6761: f64 = (assign5060_e6757 + assign5060_e6760);
        let assign5060_e6764: f64 = (locals.var_inv_lnfin * p.p1290);
        let assign5060_e6765: f64 = (assign5060_e6761 + assign5060_e6764);
        let assign5060_e6768: f64 = (locals.var_inv_w * p.p1291);
        let assign5060_e6769: f64 = (assign5060_e6765 + assign5060_e6768);
        let assign5060_e6772: f64 = (locals.var_inv_wl * p.p1292);
        let assign5060_e6773: f64 = (assign5060_e6769 + assign5060_e6772);
        locals.var_egidl_i = assign5060_e6773;
        locals.var_egidl_i_rv = 0.0;

        let assign5070_e6777: f64 = (locals.var_inv_l * p.p1294);
        let assign5070_e6778: f64 = (p.p1293 + assign5070_e6777);
        let assign5070_e6781: f64 = (locals.var_inv_nfin * p.p1295);
        let assign5070_e6782: f64 = (assign5070_e6778 + assign5070_e6781);
        let assign5070_e6785: f64 = (locals.var_inv_lnfin * p.p1296);
        let assign5070_e6786: f64 = (assign5070_e6782 + assign5070_e6785);
        let assign5070_e6789: f64 = (locals.var_inv_w * p.p1297);
        let assign5070_e6790: f64 = (assign5070_e6786 + assign5070_e6789);
        let assign5070_e6793: f64 = (locals.var_inv_wl * p.p1298);
        let assign5070_e6794: f64 = (assign5070_e6790 + assign5070_e6793);
        locals.var_pgidl_i = assign5070_e6794;
        locals.var_pgidl_i_rv = 0.0;

        let assign5080_e6798: f64 = (locals.var_inv_l * p.p1330);
        let assign5080_e6799: f64 = (p.p1329 + assign5080_e6798);
        let assign5080_e6802: f64 = (locals.var_inv_nfin * p.p1331);
        let assign5080_e6803: f64 = (assign5080_e6799 + assign5080_e6802);
        let assign5080_e6806: f64 = (locals.var_inv_lnfin * p.p1332);
        let assign5080_e6807: f64 = (assign5080_e6803 + assign5080_e6806);
        let assign5080_e6810: f64 = (locals.var_inv_w * p.p1333);
        let assign5080_e6811: f64 = (assign5080_e6807 + assign5080_e6810);
        let assign5080_e6814: f64 = (locals.var_inv_wl * p.p1334);
        let assign5080_e6815: f64 = (assign5080_e6811 + assign5080_e6814);
        locals.var_atatd_i = assign5080_e6815;
        locals.var_atatd_i_rv = 0.0;

        let assign5090_e6819: f64 = (locals.var_inv_l * p.p1336);
        let assign5090_e6820: f64 = (p.p1335 + assign5090_e6819);
        let assign5090_e6823: f64 = (locals.var_inv_nfin * p.p1337);
        let assign5090_e6824: f64 = (assign5090_e6820 + assign5090_e6823);
        let assign5090_e6827: f64 = (locals.var_inv_lnfin * p.p1338);
        let assign5090_e6828: f64 = (assign5090_e6824 + assign5090_e6827);
        let assign5090_e6831: f64 = (locals.var_inv_w * p.p1339);
        let assign5090_e6832: f64 = (assign5090_e6828 + assign5090_e6831);
        let assign5090_e6835: f64 = (locals.var_inv_wl * p.p1340);
        let assign5090_e6836: f64 = (assign5090_e6832 + assign5090_e6835);
        locals.var_btatd_i = assign5090_e6836;
        locals.var_btatd_i_rv = 0.0;

        let assign5100_e6840: f64 = (locals.var_inv_l * p.p1342);
        let assign5100_e6841: f64 = (p.p1341 + assign5100_e6840);
        let assign5100_e6844: f64 = (locals.var_inv_nfin * p.p1343);
        let assign5100_e6845: f64 = (assign5100_e6841 + assign5100_e6844);
        let assign5100_e6848: f64 = (locals.var_inv_lnfin * p.p1344);
        let assign5100_e6849: f64 = (assign5100_e6845 + assign5100_e6848);
        let assign5100_e6852: f64 = (locals.var_inv_w * p.p1345);
        let assign5100_e6853: f64 = (assign5100_e6849 + assign5100_e6852);
        let assign5100_e6856: f64 = (locals.var_inv_wl * p.p1346);
        let assign5100_e6857: f64 = (assign5100_e6853 + assign5100_e6856);
        locals.var_ctatd_i = assign5100_e6857;
        locals.var_ctatd_i_rv = 0.0;

        let assign5110_e6861: f64 = (locals.var_inv_l * p.p1348);
        let assign5110_e6862: f64 = (p.p1347 + assign5110_e6861);
        let assign5110_e6865: f64 = (locals.var_inv_nfin * p.p1349);
        let assign5110_e6866: f64 = (assign5110_e6862 + assign5110_e6865);
        let assign5110_e6869: f64 = (locals.var_inv_lnfin * p.p1350);
        let assign5110_e6870: f64 = (assign5110_e6866 + assign5110_e6869);
        let assign5110_e6873: f64 = (locals.var_inv_w * p.p1351);
        let assign5110_e6874: f64 = (assign5110_e6870 + assign5110_e6873);
        let assign5110_e6877: f64 = (locals.var_inv_wl * p.p1352);
        let assign5110_e6878: f64 = (assign5110_e6874 + assign5110_e6877);
        locals.var_dtatd_i = assign5110_e6878;
        locals.var_dtatd_i_rv = 0.0;

        let assign5120_e6882: f64 = (locals.var_inv_l * p.p1300);
        let assign5120_e6883: f64 = (p.p1299 + assign5120_e6882);
        let assign5120_e6886: f64 = (locals.var_inv_nfin * p.p1301);
        let assign5120_e6887: f64 = (assign5120_e6883 + assign5120_e6886);
        let assign5120_e6890: f64 = (locals.var_inv_lnfin * p.p1302);
        let assign5120_e6891: f64 = (assign5120_e6887 + assign5120_e6890);
        let assign5120_e6894: f64 = (locals.var_inv_w * p.p1303);
        let assign5120_e6895: f64 = (assign5120_e6891 + assign5120_e6894);
        let assign5120_e6898: f64 = (locals.var_inv_wl * p.p1304);
        let assign5120_e6899: f64 = (assign5120_e6895 + assign5120_e6898);
        locals.var_agisl_i = assign5120_e6899;
        locals.var_agisl_i_rv = 0.0;

    }
}
