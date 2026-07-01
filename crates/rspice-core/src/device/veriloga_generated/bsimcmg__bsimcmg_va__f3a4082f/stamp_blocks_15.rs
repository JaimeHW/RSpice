#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_110(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27350_e47027, assign27350_e47027_d_n0, assign27350_e47027_d_n2, assign27350_e47027_d_n3, assign27350_e47027_d_n4, assign27350_e47027_d_n5, assign27350_e47027_d_n6, assign27350_e47027_d_n7, assign27350_e47027_d_n8, assign27350_e47027_d_n9, assign27350_e47027_d_n10, assign27350_e47027_d_n11, assign27350_e47027_d_n13, assign27350_e47027_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27350_e46978: f64 = (locals.var_sii1_i * locals.var_vgsfbeff);
        let assign27350_e46980: f64 = (-10000.0);
        let assign27350_e46982: f64 = (assign27350_e46980 * p.p1441);
        let (assign27350_e47023, assign27350_e47023_d_n0, assign27350_e47023_d_n2, assign27350_e47023_d_n3, assign27350_e47023_d_n4, assign27350_e47023_d_n5, assign27350_e47023_d_n6, assign27350_e47023_d_n7, assign27350_e47023_d_n8, assign27350_e47023_d_n9, assign27350_e47023_d_n10, assign27350_e47023_d_n11, assign27350_e47023_d_n13, assign27350_e47023_d_n14,) = {
            if (!(assign27350_e46978 < assign27350_e46982)) {
                let assign27350_e46988: f64 = (locals.var_sii1_i * locals.var_vgsfbeff);
                let assign27350_e46991: f64 = (locals.var_sii1_i * locals.var_vgsfbeff);
                let assign27350_e46994: f64 = (locals.var_sii1_i * locals.var_vgsfbeff);
                let assign27350_e46995: f64 = (assign27350_e46991 * assign27350_e46994);
                let assign27350_e46998: f64 = (4.0 * p.p1441);
                let assign27350_e47000: f64 = (assign27350_e46998 * p.p1441);
                let assign27350_e47001: f64 = (assign27350_e46995 + assign27350_e47000);
                let assign27350_e47002: f64 = (assign27350_e47001).sqrt();
                let assign27350_e47003: f64 = (assign27350_e46988 + assign27350_e47002);
                let assign27350_e47004: f64 = (0.5 * assign27350_e47003);
                (assign27350_e47004, (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn0) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn0) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn0))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn2) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn2) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn2))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn3) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn3) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn3))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn4) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn4) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn4))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn5) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn5) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn5))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn6) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn6) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn6))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn7) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn7) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn7))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn8) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn8) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn8))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn9) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn9) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn9))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn10) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn10) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn10))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn11) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn11) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn11))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn13) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn13) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn13))) / (2.0 * assign27350_e47002)))), (0.5 * ((locals.var_sii1_i * locals.var_vgsfbeff_dn14) + ((((locals.var_sii1_i * locals.var_vgsfbeff_dn14) * assign27350_e46994) + (assign27350_e46991 * (locals.var_sii1_i * locals.var_vgsfbeff_dn14))) / (2.0 * assign27350_e47002)))),)
            } else {
                let assign27350_e47007: f64 = (locals.var_sii1_i * locals.var_vgsfbeff);
                let assign27350_e47009: f64 = (-10000.0);
                let assign27350_e47011: f64 = (assign27350_e47009 * p.p1441);
                let (assign27350_e47022, assign27350_e47022_d_n0, assign27350_e47022_d_n2, assign27350_e47022_d_n3, assign27350_e47022_d_n4, assign27350_e47022_d_n5, assign27350_e47022_d_n6, assign27350_e47022_d_n7, assign27350_e47022_d_n8, assign27350_e47022_d_n9, assign27350_e47022_d_n10, assign27350_e47022_d_n11, assign27350_e47022_d_n13, assign27350_e47022_d_n14,) = {
                    if (assign27350_e47007 < assign27350_e47011) {
                        let assign27350_e47014: f64 = (-p.p1441);
                        let assign27350_e47016: f64 = (assign27350_e47014 * p.p1441);
                        let assign27350_e47019: f64 = (locals.var_sii1_i * locals.var_vgsfbeff);
                        let assign27350_e47020: f64 = (assign27350_e47016 / assign27350_e47019);
                        (assign27350_e47020, (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn0)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn2)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn3)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn4)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn5)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn6)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn7)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn8)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn9)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn10)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn11)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn13)) / (assign27350_e47019 * assign27350_e47019))), (-((assign27350_e47016 * (locals.var_sii1_i * locals.var_vgsfbeff_dn14)) / (assign27350_e47019 * assign27350_e47019))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign27350_e47022, assign27350_e47022_d_n0, assign27350_e47022_d_n2, assign27350_e47022_d_n3, assign27350_e47022_d_n4, assign27350_e47022_d_n5, assign27350_e47022_d_n6, assign27350_e47022_d_n7, assign27350_e47022_d_n8, assign27350_e47022_d_n9, assign27350_e47022_d_n10, assign27350_e47022_d_n11, assign27350_e47022_d_n13, assign27350_e47022_d_n14,)
            }
        };
        let assign27350_e47024: f64 = (1.0 + assign27350_e47023);
        let assign27350_e47025: f64 = (1.0 / assign27350_e47024);
        (assign27350_e47025, (-(assign27350_e47023_d_n0 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n2 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n3 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n4 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n5 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n6 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n7 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n8 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n9 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n10 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n11 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n13 / (assign27350_e47024 * assign27350_e47024))), (-(assign27350_e47023_d_n14 / (assign27350_e47024 * assign27350_e47024))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27350_e47027;
        locals.var_t0_dn0 = assign27350_e47027_d_n0;
        locals.var_t0_dn2 = assign27350_e47027_d_n2;
        locals.var_t0_dn3 = assign27350_e47027_d_n3;
        locals.var_t0_dn4 = assign27350_e47027_d_n4;
        locals.var_t0_dn5 = assign27350_e47027_d_n5;
        locals.var_t0_dn6 = assign27350_e47027_d_n6;
        locals.var_t0_dn7 = assign27350_e47027_d_n7;
        locals.var_t0_dn8 = assign27350_e47027_d_n8;
        locals.var_t0_dn9 = assign27350_e47027_d_n9;
        locals.var_t0_dn10 = assign27350_e47027_d_n10;
        locals.var_t0_dn11 = assign27350_e47027_d_n11;
        locals.var_t0_dn13 = assign27350_e47027_d_n13;
        locals.var_t0_dn14 = assign27350_e47027_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27360_e47039, assign27360_e47039_d_n0, assign27360_e47039_d_n2, assign27360_e47039_d_n3, assign27360_e47039_d_n4, assign27360_e47039_d_n5, assign27360_e47039_d_n6, assign27360_e47039_d_n7, assign27360_e47039_d_n8, assign27360_e47039_d_n9, assign27360_e47039_d_n10, assign27360_e47039_d_n11, assign27360_e47039_d_n13, assign27360_e47039_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27360_e47037: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign27360_e47037, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27360_e47039;
        locals.var_t3_dn0 = assign27360_e47039_d_n0;
        locals.var_t3_dn2 = assign27360_e47039_d_n2;
        locals.var_t3_dn3 = assign27360_e47039_d_n3;
        locals.var_t3_dn4 = assign27360_e47039_d_n4;
        locals.var_t3_dn5 = assign27360_e47039_d_n5;
        locals.var_t3_dn6 = assign27360_e47039_d_n6;
        locals.var_t3_dn7 = assign27360_e47039_d_n7;
        locals.var_t3_dn8 = assign27360_e47039_d_n8;
        locals.var_t3_dn9 = assign27360_e47039_d_n9;
        locals.var_t3_dn10 = assign27360_e47039_d_n10;
        locals.var_t3_dn11 = assign27360_e47039_d_n11;
        locals.var_t3_dn13 = assign27360_e47039_d_n13;
        locals.var_t3_dn14 = assign27360_e47039_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27370_e47096, assign27370_e47096_d_n0, assign27370_e47096_d_n2, assign27370_e47096_d_n3, assign27370_e47096_d_n4, assign27370_e47096_d_n5, assign27370_e47096_d_n6, assign27370_e47096_d_n7, assign27370_e47096_d_n8, assign27370_e47096_d_n9, assign27370_e47096_d_n10, assign27370_e47096_d_n11, assign27370_e47096_d_n13, assign27370_e47096_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27370_e47049: f64 = (locals.var_vgsfbeff * locals.var_t3);
        let assign27370_e47051: f64 = (-10000.0);
        let assign27370_e47053: f64 = (assign27370_e47051 * p.p1442);
        let (assign27370_e47094, assign27370_e47094_d_n0, assign27370_e47094_d_n2, assign27370_e47094_d_n3, assign27370_e47094_d_n4, assign27370_e47094_d_n5, assign27370_e47094_d_n6, assign27370_e47094_d_n7, assign27370_e47094_d_n8, assign27370_e47094_d_n9, assign27370_e47094_d_n10, assign27370_e47094_d_n11, assign27370_e47094_d_n13, assign27370_e47094_d_n14,) = {
            if (!(assign27370_e47049 < assign27370_e47053)) {
                let assign27370_e47059: f64 = (locals.var_vgsfbeff * locals.var_t3);
                let assign27370_e47062: f64 = (locals.var_vgsfbeff * locals.var_t3);
                let assign27370_e47065: f64 = (locals.var_vgsfbeff * locals.var_t3);
                let assign27370_e47066: f64 = (assign27370_e47062 * assign27370_e47065);
                let assign27370_e47069: f64 = (4.0 * p.p1442);
                let assign27370_e47071: f64 = (assign27370_e47069 * p.p1442);
                let assign27370_e47072: f64 = (assign27370_e47066 + assign27370_e47071);
                let assign27370_e47073: f64 = (assign27370_e47072).sqrt();
                let assign27370_e47074: f64 = (assign27370_e47059 + assign27370_e47073);
                let assign27370_e47075: f64 = (0.5 * assign27370_e47074);
                (assign27370_e47075, (0.5 * (((locals.var_vgsfbeff_dn0 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn0)) + (((((locals.var_vgsfbeff_dn0 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn0)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn0 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn0)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn2 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn2)) + (((((locals.var_vgsfbeff_dn2 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn2)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn2 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn2)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn3 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn3)) + (((((locals.var_vgsfbeff_dn3 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn3)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn3 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn3)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn4 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn4)) + (((((locals.var_vgsfbeff_dn4 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn4)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn4 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn4)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn5 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn5)) + (((((locals.var_vgsfbeff_dn5 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn5)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn5 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn5)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn6 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn6)) + (((((locals.var_vgsfbeff_dn6 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn6)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn6 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn6)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn7 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn7)) + (((((locals.var_vgsfbeff_dn7 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn7)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn7 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn7)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn8 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn8)) + (((((locals.var_vgsfbeff_dn8 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn8)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn8 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn8)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn9 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn9)) + (((((locals.var_vgsfbeff_dn9 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn9)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn9 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn9)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn10 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn10)) + (((((locals.var_vgsfbeff_dn10 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn10)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn10 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn10)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn11 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn11)) + (((((locals.var_vgsfbeff_dn11 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn11)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn11 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn11)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn13 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn13)) + (((((locals.var_vgsfbeff_dn13 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn13)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn13 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn13)))) / (2.0 * assign27370_e47073)))), (0.5 * (((locals.var_vgsfbeff_dn14 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn14)) + (((((locals.var_vgsfbeff_dn14 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn14)) * assign27370_e47065) + (assign27370_e47062 * ((locals.var_vgsfbeff_dn14 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn14)))) / (2.0 * assign27370_e47073)))),)
            } else {
                let assign27370_e47078: f64 = (locals.var_vgsfbeff * locals.var_t3);
                let assign27370_e47080: f64 = (-10000.0);
                let assign27370_e47082: f64 = (assign27370_e47080 * p.p1442);
                let (assign27370_e47093, assign27370_e47093_d_n0, assign27370_e47093_d_n2, assign27370_e47093_d_n3, assign27370_e47093_d_n4, assign27370_e47093_d_n5, assign27370_e47093_d_n6, assign27370_e47093_d_n7, assign27370_e47093_d_n8, assign27370_e47093_d_n9, assign27370_e47093_d_n10, assign27370_e47093_d_n11, assign27370_e47093_d_n13, assign27370_e47093_d_n14,) = {
                    if (assign27370_e47078 < assign27370_e47082) {
                        let assign27370_e47085: f64 = (-p.p1442);
                        let assign27370_e47087: f64 = (assign27370_e47085 * p.p1442);
                        let assign27370_e47090: f64 = (locals.var_vgsfbeff * locals.var_t3);
                        let assign27370_e47091: f64 = (assign27370_e47087 / assign27370_e47090);
                        (assign27370_e47091, (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn0 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn0))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn2 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn2))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn3 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn3))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn4 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn4))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn5 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn5))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn6 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn6))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn7 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn7))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn8 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn8))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn9 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn9))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn10 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn10))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn11 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn11))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn13 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn13))) / (assign27370_e47090 * assign27370_e47090))), (-((assign27370_e47087 * ((locals.var_vgsfbeff_dn14 * locals.var_t3) + (locals.var_vgsfbeff * locals.var_t3_dn14))) / (assign27370_e47090 * assign27370_e47090))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign27370_e47093, assign27370_e47093_d_n0, assign27370_e47093_d_n2, assign27370_e47093_d_n3, assign27370_e47093_d_n4, assign27370_e47093_d_n5, assign27370_e47093_d_n6, assign27370_e47093_d_n7, assign27370_e47093_d_n8, assign27370_e47093_d_n9, assign27370_e47093_d_n10, assign27370_e47093_d_n11, assign27370_e47093_d_n13, assign27370_e47093_d_n14,)
            }
        };
        (assign27370_e47094, assign27370_e47094_d_n0, assign27370_e47094_d_n2, assign27370_e47094_d_n3, assign27370_e47094_d_n4, assign27370_e47094_d_n5, assign27370_e47094_d_n6, assign27370_e47094_d_n7, assign27370_e47094_d_n8, assign27370_e47094_d_n9, assign27370_e47094_d_n10, assign27370_e47094_d_n11, assign27370_e47094_d_n13, assign27370_e47094_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27370_e47096;
        locals.var_t2_dn0 = assign27370_e47096_d_n0;
        locals.var_t2_dn2 = assign27370_e47096_d_n2;
        locals.var_t2_dn3 = assign27370_e47096_d_n3;
        locals.var_t2_dn4 = assign27370_e47096_d_n4;
        locals.var_t2_dn5 = assign27370_e47096_d_n5;
        locals.var_t2_dn6 = assign27370_e47096_d_n6;
        locals.var_t2_dn7 = assign27370_e47096_d_n7;
        locals.var_t2_dn8 = assign27370_e47096_d_n8;
        locals.var_t2_dn9 = assign27370_e47096_d_n9;
        locals.var_t2_dn10 = assign27370_e47096_d_n10;
        locals.var_t2_dn11 = assign27370_e47096_d_n11;
        locals.var_t2_dn13 = assign27370_e47096_d_n13;
        locals.var_t2_dn14 = assign27370_e47096_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27380_e47112, assign27380_e47112_d_n0, assign27380_e47112_d_n2, assign27380_e47112_d_n3, assign27380_e47112_d_n4, assign27380_e47112_d_n5, assign27380_e47112_d_n6, assign27380_e47112_d_n7, assign27380_e47112_d_n8, assign27380_e47112_d_n9, assign27380_e47112_d_n10, assign27380_e47112_d_n11, assign27380_e47112_d_n13, assign27380_e47112_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27380_e47108: f64 = (locals.var_siid_i * locals.var_vds);
        let assign27380_e47109: f64 = (1.0 + assign27380_e47108);
        let assign27380_e47110: f64 = (1.0 / assign27380_e47109);
        (assign27380_e47110, 0.0, 0.0, 0.0, 0.0, (-((locals.var_siid_i * locals.var_vds_dn5) / (assign27380_e47109 * assign27380_e47109))), (-((locals.var_siid_i * locals.var_vds_dn6) / (assign27380_e47109 * assign27380_e47109))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27380_e47112;
        locals.var_t3_dn0 = assign27380_e47112_d_n0;
        locals.var_t3_dn2 = assign27380_e47112_d_n2;
        locals.var_t3_dn3 = assign27380_e47112_d_n3;
        locals.var_t3_dn4 = assign27380_e47112_d_n4;
        locals.var_t3_dn5 = assign27380_e47112_d_n5;
        locals.var_t3_dn6 = assign27380_e47112_d_n6;
        locals.var_t3_dn7 = assign27380_e47112_d_n7;
        locals.var_t3_dn8 = assign27380_e47112_d_n8;
        locals.var_t3_dn9 = assign27380_e47112_d_n9;
        locals.var_t3_dn10 = assign27380_e47112_d_n10;
        locals.var_t3_dn11 = assign27380_e47112_d_n11;
        locals.var_t3_dn13 = assign27380_e47112_d_n13;
        locals.var_t3_dn14 = assign27380_e47112_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27390_e47126, assign27390_e47126_d_n0, assign27390_e47126_d_n2, assign27390_e47126_d_n3, assign27390_e47126_d_n4, assign27390_e47126_d_n5, assign27390_e47126_d_n6, assign27390_e47126_d_n7, assign27390_e47126_d_n8, assign27390_e47126_d_n9, assign27390_e47126_d_n10, assign27390_e47126_d_n11, assign27390_e47126_d_n13, assign27390_e47126_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27390_e47122: f64 = (locals.var_t1 * locals.var_t2);
        let assign27390_e47124: f64 = (assign27390_e47122 * locals.var_t3);
        (assign27390_e47124, ((((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn0)), ((((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn2)), ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn11)), ((((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn13)), ((((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)) * locals.var_t3) + (assign27390_e47122 * locals.var_t3_dn14)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn0, locals.var_vgsstep_dn2, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11, locals.var_vgsstep_dn13, locals.var_vgsstep_dn14,)
    }
};
        locals.var_vgsstep = assign27390_e47126;
        locals.var_vgsstep_dn0 = assign27390_e47126_d_n0;
        locals.var_vgsstep_dn2 = assign27390_e47126_d_n2;
        locals.var_vgsstep_dn3 = assign27390_e47126_d_n3;
        locals.var_vgsstep_dn4 = assign27390_e47126_d_n4;
        locals.var_vgsstep_dn5 = assign27390_e47126_d_n5;
        locals.var_vgsstep_dn6 = assign27390_e47126_d_n6;
        locals.var_vgsstep_dn7 = assign27390_e47126_d_n7;
        locals.var_vgsstep_dn8 = assign27390_e47126_d_n8;
        locals.var_vgsstep_dn9 = assign27390_e47126_d_n9;
        locals.var_vgsstep_dn10 = assign27390_e47126_d_n10;
        locals.var_vgsstep_dn11 = assign27390_e47126_d_n11;
        locals.var_vgsstep_dn13 = assign27390_e47126_d_n13;
        locals.var_vgsstep_dn14 = assign27390_e47126_d_n14;
        locals.var_vgsstep_rv = 0.0;

        let (assign27400_e47142, assign27400_e47142_d_n0, assign27400_e47142_d_n2, assign27400_e47142_d_n3, assign27400_e47142_d_n4, assign27400_e47142_d_n5, assign27400_e47142_d_n6, assign27400_e47142_d_n7, assign27400_e47142_d_n8, assign27400_e47142_d_n9, assign27400_e47142_d_n10, assign27400_e47142_d_n11, assign27400_e47142_d_n13, assign27400_e47142_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27400_e47138: f64 = (locals.var_lii_i / locals.var_leff_1);
        let assign27400_e47139: f64 = (1.0 - assign27400_e47138);
        let assign27400_e47140: f64 = (locals.var_vgsstep * assign27400_e47139);
        (assign27400_e47140, ((locals.var_vgsstep_dn0 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn2 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn3 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn4 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn5 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn6 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn7 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn8 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn9 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn10 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn11 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn13 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1)))))), ((locals.var_vgsstep_dn14 * assign27400_e47139) + (locals.var_vgsstep * (-(-((locals.var_lii_i * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1)))))),)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn0, locals.var_vdsatii_dn2, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11, locals.var_vdsatii_dn13, locals.var_vdsatii_dn14,)
    }
};
        locals.var_vdsatii = assign27400_e47142;
        locals.var_vdsatii_dn0 = assign27400_e47142_d_n0;
        locals.var_vdsatii_dn2 = assign27400_e47142_d_n2;
        locals.var_vdsatii_dn3 = assign27400_e47142_d_n3;
        locals.var_vdsatii_dn4 = assign27400_e47142_d_n4;
        locals.var_vdsatii_dn5 = assign27400_e47142_d_n5;
        locals.var_vdsatii_dn6 = assign27400_e47142_d_n6;
        locals.var_vdsatii_dn7 = assign27400_e47142_d_n7;
        locals.var_vdsatii_dn8 = assign27400_e47142_d_n8;
        locals.var_vdsatii_dn9 = assign27400_e47142_d_n9;
        locals.var_vdsatii_dn10 = assign27400_e47142_d_n10;
        locals.var_vdsatii_dn11 = assign27400_e47142_d_n11;
        locals.var_vdsatii_dn13 = assign27400_e47142_d_n13;
        locals.var_vdsatii_dn14 = assign27400_e47142_d_n14;
        locals.var_vdsatii_rv = 0.0;

        let (assign27410_e47154, assign27410_e47154_d_n0, assign27410_e47154_d_n2, assign27410_e47154_d_n3, assign27410_e47154_d_n4, assign27410_e47154_d_n5, assign27410_e47154_d_n6, assign27410_e47154_d_n7, assign27410_e47154_d_n8, assign27410_e47154_d_n9, assign27410_e47154_d_n10, assign27410_e47154_d_n11, assign27410_e47154_d_n13, assign27410_e47154_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27410_e47152: f64 = (locals.var_vds - locals.var_vdsatii);
        (assign27410_e47152, (-locals.var_vdsatii_dn0), (-locals.var_vdsatii_dn2), (-locals.var_vdsatii_dn3), (-locals.var_vdsatii_dn4), (locals.var_vds_dn5 - locals.var_vdsatii_dn5), (locals.var_vds_dn6 - locals.var_vdsatii_dn6), (-locals.var_vdsatii_dn7), (-locals.var_vdsatii_dn8), (-locals.var_vdsatii_dn9), (-locals.var_vdsatii_dn10), (-locals.var_vdsatii_dn11), (-locals.var_vdsatii_dn13), (-locals.var_vdsatii_dn14),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn0, locals.var_vdiff_dn2, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11, locals.var_vdiff_dn13, locals.var_vdiff_dn14,)
    }
};
        locals.var_vdiff = assign27410_e47154;
        locals.var_vdiff_dn0 = assign27410_e47154_d_n0;
        locals.var_vdiff_dn2 = assign27410_e47154_d_n2;
        locals.var_vdiff_dn3 = assign27410_e47154_d_n3;
        locals.var_vdiff_dn4 = assign27410_e47154_d_n4;
        locals.var_vdiff_dn5 = assign27410_e47154_d_n5;
        locals.var_vdiff_dn6 = assign27410_e47154_d_n6;
        locals.var_vdiff_dn7 = assign27410_e47154_d_n7;
        locals.var_vdiff_dn8 = assign27410_e47154_d_n8;
        locals.var_vdiff_dn9 = assign27410_e47154_d_n9;
        locals.var_vdiff_dn10 = assign27410_e47154_d_n10;
        locals.var_vdiff_dn11 = assign27410_e47154_d_n11;
        locals.var_vdiff_dn13 = assign27410_e47154_d_n13;
        locals.var_vdiff_dn14 = assign27410_e47154_d_n14;
        locals.var_vdiff_rv = 0.0;

        let (assign27420_e47174, assign27420_e47174_d_n0, assign27420_e47174_d_n2, assign27420_e47174_d_n3, assign27420_e47174_d_n4, assign27420_e47174_d_n5, assign27420_e47174_d_n6, assign27420_e47174_d_n7, assign27420_e47174_d_n8, assign27420_e47174_d_n9, assign27420_e47174_d_n10, assign27420_e47174_d_n11, assign27420_e47174_d_n13, assign27420_e47174_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27420_e47165: f64 = (locals.var_betaii1_i * locals.var_vdiff);
        let assign27420_e47166: f64 = (locals.var_betaii2_i + assign27420_e47165);
        let assign27420_e47169: f64 = (locals.var_betaii0_i * locals.var_vdiff);
        let assign27420_e47171: f64 = (assign27420_e47169 * locals.var_vdiff);
        let assign27420_e47172: f64 = (assign27420_e47166 + assign27420_e47171);
        (assign27420_e47172, ((locals.var_betaii1_i * locals.var_vdiff_dn0) + (((locals.var_betaii0_i * locals.var_vdiff_dn0) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn0))), ((locals.var_betaii1_i * locals.var_vdiff_dn2) + (((locals.var_betaii0_i * locals.var_vdiff_dn2) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn2))), ((locals.var_betaii1_i * locals.var_vdiff_dn3) + (((locals.var_betaii0_i * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn3))), ((locals.var_betaii1_i * locals.var_vdiff_dn4) + (((locals.var_betaii0_i * locals.var_vdiff_dn4) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn4))), ((locals.var_betaii1_i * locals.var_vdiff_dn5) + (((locals.var_betaii0_i * locals.var_vdiff_dn5) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn5))), ((locals.var_betaii1_i * locals.var_vdiff_dn6) + (((locals.var_betaii0_i * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn6))), ((locals.var_betaii1_i * locals.var_vdiff_dn7) + (((locals.var_betaii0_i * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn7))), ((locals.var_betaii1_i * locals.var_vdiff_dn8) + (((locals.var_betaii0_i * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn8))), ((locals.var_betaii1_i * locals.var_vdiff_dn9) + (((locals.var_betaii0_i * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn9))), ((locals.var_betaii1_i * locals.var_vdiff_dn10) + (((locals.var_betaii0_i * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn10))), ((locals.var_betaii1_i * locals.var_vdiff_dn11) + (((locals.var_betaii0_i * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn11))), ((locals.var_betaii1_i * locals.var_vdiff_dn13) + (((locals.var_betaii0_i * locals.var_vdiff_dn13) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn13))), ((locals.var_betaii1_i * locals.var_vdiff_dn14) + (((locals.var_betaii0_i * locals.var_vdiff_dn14) * locals.var_vdiff) + (assign27420_e47169 * locals.var_vdiff_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27420_e47174;
        locals.var_t0_dn0 = assign27420_e47174_d_n0;
        locals.var_t0_dn2 = assign27420_e47174_d_n2;
        locals.var_t0_dn3 = assign27420_e47174_d_n3;
        locals.var_t0_dn4 = assign27420_e47174_d_n4;
        locals.var_t0_dn5 = assign27420_e47174_d_n5;
        locals.var_t0_dn6 = assign27420_e47174_d_n6;
        locals.var_t0_dn7 = assign27420_e47174_d_n7;
        locals.var_t0_dn8 = assign27420_e47174_d_n8;
        locals.var_t0_dn9 = assign27420_e47174_d_n9;
        locals.var_t0_dn10 = assign27420_e47174_d_n10;
        locals.var_t0_dn11 = assign27420_e47174_d_n11;
        locals.var_t0_dn13 = assign27420_e47174_d_n13;
        locals.var_t0_dn14 = assign27420_e47174_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27430_e47189, assign27430_e47189_d_n0, assign27430_e47189_d_n2, assign27430_e47189_d_n3, assign27430_e47189_d_n4, assign27430_e47189_d_n5, assign27430_e47189_d_n6, assign27430_e47189_d_n7, assign27430_e47189_d_n8, assign27430_e47189_d_n9, assign27430_e47189_d_n10, assign27430_e47189_d_n11, assign27430_e47189_d_n13, assign27430_e47189_d_n14,) = {
    if (((locals.var_guard440 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign27430_e47184: f64 = (locals.var_t0 * locals.var_t0);
        let assign27430_e47186: f64 = (assign27430_e47184 + 1e-10);
        let assign27430_e47187: f64 = (assign27430_e47186).sqrt();
        (assign27430_e47187, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign27430_e47187)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign27430_e47187)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27430_e47189;
        locals.var_t1_dn0 = assign27430_e47189_d_n0;
        locals.var_t1_dn2 = assign27430_e47189_d_n2;
        locals.var_t1_dn3 = assign27430_e47189_d_n3;
        locals.var_t1_dn4 = assign27430_e47189_d_n4;
        locals.var_t1_dn5 = assign27430_e47189_d_n5;
        locals.var_t1_dn6 = assign27430_e47189_d_n6;
        locals.var_t1_dn7 = assign27430_e47189_d_n7;
        locals.var_t1_dn8 = assign27430_e47189_d_n8;
        locals.var_t1_dn9 = assign27430_e47189_d_n9;
        locals.var_t1_dn10 = assign27430_e47189_d_n10;
        locals.var_t1_dn11 = assign27430_e47189_d_n11;
        locals.var_t1_dn13 = assign27430_e47189_d_n13;
        locals.var_t1_dn14 = assign27430_e47189_d_n14;
        locals.var_t1_rv = 0.0;

        let assign27460_e47265: f64 = if p.p69 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard444 = assign27460_e47265;
        locals.var_guard444_rv = 0.0;

        let (assign27470_e47275, assign27470_e47275_d_n0, assign27470_e47275_d_n2, assign27470_e47275_d_n3, assign27470_e47275_d_n4, assign27470_e47275_d_n5, assign27470_e47275_d_n6, assign27470_e47275_d_n7, assign27470_e47275_d_n8, assign27470_e47275_d_n9, assign27470_e47275_d_n10, assign27470_e47275_d_n11, assign27470_e47275_d_n13, assign27470_e47275_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27470_e47269: f64 = (locals.var_qia - locals.var_eigbinv_i);
        let assign27470_e47271: f64 = (assign27470_e47269 / locals.var_nigbinv_i);
        let assign27470_e47273: f64 = (assign27470_e47271 / locals.var_vtm);
        (assign27470_e47273, ((locals.var_qia_dn0 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn2 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn3 / locals.var_nigbinv_i) / locals.var_vtm), ((((locals.var_qia_dn4 / locals.var_nigbinv_i) * locals.var_vtm) - (assign27470_e47271 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), ((locals.var_qia_dn5 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn6 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn7 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn8 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn9 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn10 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn11 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn13 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn14 / locals.var_nigbinv_i) / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27470_e47275;
        locals.var_t1_dn0 = assign27470_e47275_d_n0;
        locals.var_t1_dn2 = assign27470_e47275_d_n2;
        locals.var_t1_dn3 = assign27470_e47275_d_n3;
        locals.var_t1_dn4 = assign27470_e47275_d_n4;
        locals.var_t1_dn5 = assign27470_e47275_d_n5;
        locals.var_t1_dn6 = assign27470_e47275_d_n6;
        locals.var_t1_dn7 = assign27470_e47275_d_n7;
        locals.var_t1_dn8 = assign27470_e47275_d_n8;
        locals.var_t1_dn9 = assign27470_e47275_d_n9;
        locals.var_t1_dn10 = assign27470_e47275_d_n10;
        locals.var_t1_dn11 = assign27470_e47275_d_n11;
        locals.var_t1_dn13 = assign27470_e47275_d_n13;
        locals.var_t1_dn14 = assign27470_e47275_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign27490_e47364, assign27490_e47364_d_n0, assign27490_e47364_d_n2, assign27490_e47364_d_n3, assign27490_e47364_d_n4, assign27490_e47364_d_n5, assign27490_e47364_d_n6, assign27490_e47364_d_n7, assign27490_e47364_d_n8, assign27490_e47364_d_n9, assign27490_e47364_d_n10, assign27490_e47364_d_n11, assign27490_e47364_d_n13, assign27490_e47364_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27490_e47319: f64 = (-p.p1110);
        let assign27490_e47324: f64 = (locals.var_bigbinv_i * locals.var_qia);
        let assign27490_e47325: f64 = (locals.var_aigbinv_t - assign27490_e47324);
        let assign27490_e47327: f64 = (-p.p1110);
        let assign27490_e47328: f64 = (assign27490_e47325 - assign27490_e47327);
        let assign27490_e47330: f64 = (assign27490_e47328 - 1e-6);
        let assign27490_e47334: f64 = (locals.var_bigbinv_i * locals.var_qia);
        let assign27490_e47335: f64 = (locals.var_aigbinv_t - assign27490_e47334);
        let assign27490_e47337: f64 = (-p.p1110);
        let assign27490_e47338: f64 = (assign27490_e47335 - assign27490_e47337);
        let assign27490_e47340: f64 = (assign27490_e47338 - 1e-6);
        let assign27490_e47344: f64 = (locals.var_bigbinv_i * locals.var_qia);
        let assign27490_e47345: f64 = (locals.var_aigbinv_t - assign27490_e47344);
        let assign27490_e47347: f64 = (-p.p1110);
        let assign27490_e47348: f64 = (assign27490_e47345 - assign27490_e47347);
        let assign27490_e47350: f64 = (assign27490_e47348 - 1e-6);
        let assign27490_e47351: f64 = (assign27490_e47340 * assign27490_e47350);
        let assign27490_e47354: f64 = (-p.p1110);
        let assign27490_e47355: f64 = (4.0 * assign27490_e47354);
        let assign27490_e47357: f64 = (assign27490_e47355 * 1e-6);
        let assign27490_e47358: f64 = (assign27490_e47351 - assign27490_e47357);
        let assign27490_e47359: f64 = (assign27490_e47358).sqrt();
        let assign27490_e47360: f64 = (assign27490_e47330 + assign27490_e47359);
        let assign27490_e47361: f64 = (0.5 * assign27490_e47360);
        let assign27490_e47362: f64 = (assign27490_e47319 + assign27490_e47361);
        (assign27490_e47362, (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn0)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn0)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn0)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn2)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn2)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn2)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn3)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn3)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn3)))) / (2.0 * assign27490_e47359)))), (0.5 * ((locals.var_aigbinv_t_dn4 - (locals.var_bigbinv_i * locals.var_qia_dn4)) + ((((locals.var_aigbinv_t_dn4 - (locals.var_bigbinv_i * locals.var_qia_dn4)) * assign27490_e47350) + (assign27490_e47340 * (locals.var_aigbinv_t_dn4 - (locals.var_bigbinv_i * locals.var_qia_dn4)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn5)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn5)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn5)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn6)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn6)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn6)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn7)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn7)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn7)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn8)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn8)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn8)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn9)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn9)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn9)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn10)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn10)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn10)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn11)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn11)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn11)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn13)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn13)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn13)))) / (2.0 * assign27490_e47359)))), (0.5 * ((-(locals.var_bigbinv_i * locals.var_qia_dn14)) + ((((-(locals.var_bigbinv_i * locals.var_qia_dn14)) * assign27490_e47350) + (assign27490_e47340 * (-(locals.var_bigbinv_i * locals.var_qia_dn14)))) / (2.0 * assign27490_e47359)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27490_e47364;
        locals.var_t2_dn0 = assign27490_e47364_d_n0;
        locals.var_t2_dn2 = assign27490_e47364_d_n2;
        locals.var_t2_dn3 = assign27490_e47364_d_n3;
        locals.var_t2_dn4 = assign27490_e47364_d_n4;
        locals.var_t2_dn5 = assign27490_e47364_d_n5;
        locals.var_t2_dn6 = assign27490_e47364_d_n6;
        locals.var_t2_dn7 = assign27490_e47364_d_n7;
        locals.var_t2_dn8 = assign27490_e47364_d_n8;
        locals.var_t2_dn9 = assign27490_e47364_d_n9;
        locals.var_t2_dn10 = assign27490_e47364_d_n10;
        locals.var_t2_dn11 = assign27490_e47364_d_n11;
        locals.var_t2_dn13 = assign27490_e47364_d_n13;
        locals.var_t2_dn14 = assign27490_e47364_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27500_e47372, assign27500_e47372_d_n0, assign27500_e47372_d_n2, assign27500_e47372_d_n3, assign27500_e47372_d_n4, assign27500_e47372_d_n5, assign27500_e47372_d_n6, assign27500_e47372_d_n7, assign27500_e47372_d_n8, assign27500_e47372_d_n9, assign27500_e47372_d_n10, assign27500_e47372_d_n11, assign27500_e47372_d_n13, assign27500_e47372_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27500_e47369: f64 = (locals.var_cigbinv_i * locals.var_qia);
        let assign27500_e47370: f64 = (1.0 + assign27500_e47369);
        (assign27500_e47370, (locals.var_cigbinv_i * locals.var_qia_dn0), (locals.var_cigbinv_i * locals.var_qia_dn2), (locals.var_cigbinv_i * locals.var_qia_dn3), (locals.var_cigbinv_i * locals.var_qia_dn4), (locals.var_cigbinv_i * locals.var_qia_dn5), (locals.var_cigbinv_i * locals.var_qia_dn6), (locals.var_cigbinv_i * locals.var_qia_dn7), (locals.var_cigbinv_i * locals.var_qia_dn8), (locals.var_cigbinv_i * locals.var_qia_dn9), (locals.var_cigbinv_i * locals.var_qia_dn10), (locals.var_cigbinv_i * locals.var_qia_dn11), (locals.var_cigbinv_i * locals.var_qia_dn13), (locals.var_cigbinv_i * locals.var_qia_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27500_e47372;
        locals.var_t3_dn0 = assign27500_e47372_d_n0;
        locals.var_t3_dn2 = assign27500_e47372_d_n2;
        locals.var_t3_dn3 = assign27500_e47372_d_n3;
        locals.var_t3_dn4 = assign27500_e47372_d_n4;
        locals.var_t3_dn5 = assign27500_e47372_d_n5;
        locals.var_t3_dn6 = assign27500_e47372_d_n6;
        locals.var_t3_dn7 = assign27500_e47372_d_n7;
        locals.var_t3_dn8 = assign27500_e47372_d_n8;
        locals.var_t3_dn9 = assign27500_e47372_d_n9;
        locals.var_t3_dn10 = assign27500_e47372_d_n10;
        locals.var_t3_dn11 = assign27500_e47372_d_n11;
        locals.var_t3_dn13 = assign27500_e47372_d_n13;
        locals.var_t3_dn14 = assign27500_e47372_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27510_e47383, assign27510_e47383_d_n0, assign27510_e47383_d_n2, assign27510_e47383_d_n3, assign27510_e47383_d_n4, assign27510_e47383_d_n5, assign27510_e47383_d_n6, assign27510_e47383_d_n7, assign27510_e47383_d_n8, assign27510_e47383_d_n9, assign27510_e47383_d_n10, assign27510_e47383_d_n11, assign27510_e47383_d_n13, assign27510_e47383_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27510_e47375: f64 = (-982222000000.0);
        let assign27510_e47377: f64 = (assign27510_e47375 * p.p1109);
        let assign27510_e47379: f64 = (assign27510_e47377 * locals.var_t2);
        let assign27510_e47381: f64 = (assign27510_e47379 * locals.var_t3);
        (assign27510_e47381, (((assign27510_e47377 * locals.var_t2_dn0) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn0)), (((assign27510_e47377 * locals.var_t2_dn2) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn2)), (((assign27510_e47377 * locals.var_t2_dn3) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn3)), (((assign27510_e47377 * locals.var_t2_dn4) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn4)), (((assign27510_e47377 * locals.var_t2_dn5) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn5)), (((assign27510_e47377 * locals.var_t2_dn6) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn6)), (((assign27510_e47377 * locals.var_t2_dn7) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn7)), (((assign27510_e47377 * locals.var_t2_dn8) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn8)), (((assign27510_e47377 * locals.var_t2_dn9) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn9)), (((assign27510_e47377 * locals.var_t2_dn10) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn10)), (((assign27510_e47377 * locals.var_t2_dn11) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn11)), (((assign27510_e47377 * locals.var_t2_dn13) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn13)), (((assign27510_e47377 * locals.var_t2_dn14) * locals.var_t3) + (assign27510_e47379 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign27510_e47383;
        locals.var_t4_dn0 = assign27510_e47383_d_n0;
        locals.var_t4_dn2 = assign27510_e47383_d_n2;
        locals.var_t4_dn3 = assign27510_e47383_d_n3;
        locals.var_t4_dn4 = assign27510_e47383_d_n4;
        locals.var_t4_dn5 = assign27510_e47383_d_n5;
        locals.var_t4_dn6 = assign27510_e47383_d_n6;
        locals.var_t4_dn7 = assign27510_e47383_d_n7;
        locals.var_t4_dn8 = assign27510_e47383_d_n8;
        locals.var_t4_dn9 = assign27510_e47383_d_n9;
        locals.var_t4_dn10 = assign27510_e47383_d_n10;
        locals.var_t4_dn11 = assign27510_e47383_d_n11;
        locals.var_t4_dn13 = assign27510_e47383_d_n13;
        locals.var_t4_dn14 = assign27510_e47383_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign27520_e47388, assign27520_e47388_d_n0, assign27520_e47388_d_n2, assign27520_e47388_d_n3, assign27520_e47388_d_n4, assign27520_e47388_d_n5, assign27520_e47388_d_n6, assign27520_e47388_d_n7, assign27520_e47388_d_n8, assign27520_e47388_d_n9, assign27520_e47388_d_n10, assign27520_e47388_d_n11, assign27520_e47388_d_n13, assign27520_e47388_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27520_e47386: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27520_e47386, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign27520_e47388;
        locals.var_t5_dn0 = assign27520_e47388_d_n0;
        locals.var_t5_dn2 = assign27520_e47388_d_n2;
        locals.var_t5_dn3 = assign27520_e47388_d_n3;
        locals.var_t5_dn4 = assign27520_e47388_d_n4;
        locals.var_t5_dn5 = assign27520_e47388_d_n5;
        locals.var_t5_dn6 = assign27520_e47388_d_n6;
        locals.var_t5_dn7 = assign27520_e47388_d_n7;
        locals.var_t5_dn8 = assign27520_e47388_d_n8;
        locals.var_t5_dn9 = assign27520_e47388_d_n9;
        locals.var_t5_dn10 = assign27520_e47388_d_n10;
        locals.var_t5_dn11 = assign27520_e47388_d_n11;
        locals.var_t5_dn13 = assign27520_e47388_d_n13;
        locals.var_t5_dn14 = assign27520_e47388_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign27530_e47392, assign27530_e47392_d_n0, assign27530_e47392_d_n2, assign27530_e47392_d_n3, assign27530_e47392_d_n4, assign27530_e47392_d_n5, assign27530_e47392_d_n6, assign27530_e47392_d_n7, assign27530_e47392_d_n8, assign27530_e47392_d_n9, assign27530_e47392_d_n10, assign27530_e47392_d_n11, assign27530_e47392_d_n13, assign27530_e47392_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        (3.75956e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign27530_e47392;
        locals.var_t6_dn0 = assign27530_e47392_d_n0;
        locals.var_t6_dn2 = assign27530_e47392_d_n2;
        locals.var_t6_dn3 = assign27530_e47392_d_n3;
        locals.var_t6_dn4 = assign27530_e47392_d_n4;
        locals.var_t6_dn5 = assign27530_e47392_d_n5;
        locals.var_t6_dn6 = assign27530_e47392_d_n6;
        locals.var_t6_dn7 = assign27530_e47392_d_n7;
        locals.var_t6_dn8 = assign27530_e47392_d_n8;
        locals.var_t6_dn9 = assign27530_e47392_d_n9;
        locals.var_t6_dn10 = assign27530_e47392_d_n10;
        locals.var_t6_dn11 = assign27530_e47392_d_n11;
        locals.var_t6_dn13 = assign27530_e47392_d_n13;
        locals.var_t6_dn14 = assign27530_e47392_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign27560_e47426, assign27560_e47426_d_n0, assign27560_e47426_d_n2, assign27560_e47426_d_n3, assign27560_e47426_d_n4, assign27560_e47426_d_n5, assign27560_e47426_d_n6, assign27560_e47426_d_n7, assign27560_e47426_d_n8, assign27560_e47426_d_n9, assign27560_e47426_d_n10, assign27560_e47426_d_n11, assign27560_e47426_d_n13, assign27560_e47426_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27560_e47421: f64 = (locals.var_eg / 2.0);
        let assign27560_e47422: f64 = (locals.var_deltaphi - assign27560_e47421);
        let assign27560_e47424: f64 = (assign27560_e47422 - locals.var_phib);
        (assign27560_e47424, (locals.var_deltaphi_dn0 - locals.var_phib_dn0), (locals.var_deltaphi_dn2 - locals.var_phib_dn2), (locals.var_deltaphi_dn3 - locals.var_phib_dn3), ((locals.var_deltaphi_dn4 - (locals.var_eg_dn4 / 2.0)) - locals.var_phib_dn4), (locals.var_deltaphi_dn5 - locals.var_phib_dn5), (locals.var_deltaphi_dn6 - locals.var_phib_dn6), (locals.var_deltaphi_dn7 - locals.var_phib_dn7), (locals.var_deltaphi_dn8 - locals.var_phib_dn8), (locals.var_deltaphi_dn9 - locals.var_phib_dn9), (locals.var_deltaphi_dn10 - locals.var_phib_dn10), (locals.var_deltaphi_dn11 - locals.var_phib_dn11), (locals.var_deltaphi_dn13 - locals.var_phib_dn13), (locals.var_deltaphi_dn14 - locals.var_phib_dn14),)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn0, locals.var_vfbzb_dn2, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn13, locals.var_vfbzb_dn14,)
    }
};
        locals.var_vfbzb = assign27560_e47426;
        locals.var_vfbzb_dn0 = assign27560_e47426_d_n0;
        locals.var_vfbzb_dn2 = assign27560_e47426_d_n2;
        locals.var_vfbzb_dn3 = assign27560_e47426_d_n3;
        locals.var_vfbzb_dn4 = assign27560_e47426_d_n4;
        locals.var_vfbzb_dn5 = assign27560_e47426_d_n5;
        locals.var_vfbzb_dn6 = assign27560_e47426_d_n6;
        locals.var_vfbzb_dn7 = assign27560_e47426_d_n7;
        locals.var_vfbzb_dn8 = assign27560_e47426_d_n8;
        locals.var_vfbzb_dn9 = assign27560_e47426_d_n9;
        locals.var_vfbzb_dn10 = assign27560_e47426_d_n10;
        locals.var_vfbzb_dn11 = assign27560_e47426_d_n11;
        locals.var_vfbzb_dn13 = assign27560_e47426_d_n13;
        locals.var_vfbzb_dn14 = assign27560_e47426_d_n14;
        locals.var_vfbzb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_111(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27570_e47432, assign27570_e47432_d_n0, assign27570_e47432_d_n2, assign27570_e47432_d_n3, assign27570_e47432_d_n4, assign27570_e47432_d_n5, assign27570_e47432_d_n6, assign27570_e47432_d_n7, assign27570_e47432_d_n8, assign27570_e47432_d_n9, assign27570_e47432_d_n10, assign27570_e47432_d_n11, assign27570_e47432_d_n13, assign27570_e47432_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27570_e47430: f64 = (locals.var_vfbzb - locals.var_vge);
        (assign27570_e47430, locals.var_vfbzb_dn0, locals.var_vfbzb_dn2, (locals.var_vfbzb_dn3 - locals.var_vge_dn3), locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, (locals.var_vfbzb_dn11 - locals.var_vge_dn11), locals.var_vfbzb_dn13, locals.var_vfbzb_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27570_e47432;
        locals.var_t0_dn0 = assign27570_e47432_d_n0;
        locals.var_t0_dn2 = assign27570_e47432_d_n2;
        locals.var_t0_dn3 = assign27570_e47432_d_n3;
        locals.var_t0_dn4 = assign27570_e47432_d_n4;
        locals.var_t0_dn5 = assign27570_e47432_d_n5;
        locals.var_t0_dn6 = assign27570_e47432_d_n6;
        locals.var_t0_dn7 = assign27570_e47432_d_n7;
        locals.var_t0_dn8 = assign27570_e47432_d_n8;
        locals.var_t0_dn9 = assign27570_e47432_d_n9;
        locals.var_t0_dn10 = assign27570_e47432_d_n10;
        locals.var_t0_dn11 = assign27570_e47432_d_n11;
        locals.var_t0_dn13 = assign27570_e47432_d_n13;
        locals.var_t0_dn14 = assign27570_e47432_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27580_e47440, assign27580_e47440_d_n0, assign27580_e47440_d_n2, assign27580_e47440_d_n3, assign27580_e47440_d_n4, assign27580_e47440_d_n5, assign27580_e47440_d_n6, assign27580_e47440_d_n7, assign27580_e47440_d_n8, assign27580_e47440_d_n9, assign27580_e47440_d_n10, assign27580_e47440_d_n11, assign27580_e47440_d_n13, assign27580_e47440_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27580_e47436: f64 = (locals.var_t0 / locals.var_nigbacc_i);
        let assign27580_e47438: f64 = (assign27580_e47436 / locals.var_vtm);
        (assign27580_e47438, ((locals.var_t0_dn0 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn2 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn3 / locals.var_nigbacc_i) / locals.var_vtm), ((((locals.var_t0_dn4 / locals.var_nigbacc_i) * locals.var_vtm) - (assign27580_e47436 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), ((locals.var_t0_dn5 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn6 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn7 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn8 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn9 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn10 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn11 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn13 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn14 / locals.var_nigbacc_i) / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27580_e47440;
        locals.var_t1_dn0 = assign27580_e47440_d_n0;
        locals.var_t1_dn2 = assign27580_e47440_d_n2;
        locals.var_t1_dn3 = assign27580_e47440_d_n3;
        locals.var_t1_dn4 = assign27580_e47440_d_n4;
        locals.var_t1_dn5 = assign27580_e47440_d_n5;
        locals.var_t1_dn6 = assign27580_e47440_d_n6;
        locals.var_t1_dn7 = assign27580_e47440_d_n7;
        locals.var_t1_dn8 = assign27580_e47440_d_n8;
        locals.var_t1_dn9 = assign27580_e47440_d_n9;
        locals.var_t1_dn10 = assign27580_e47440_d_n10;
        locals.var_t1_dn11 = assign27580_e47440_d_n11;
        locals.var_t1_dn13 = assign27580_e47440_d_n13;
        locals.var_t1_dn14 = assign27580_e47440_d_n14;
        locals.var_t1_rv = 0.0;

        let assign27600_e47484: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign27600_e47484;
        locals.var_guard445_rv = 0.0;

        let (assign27610_e47490, assign27610_e47490_d_n0, assign27610_e47490_d_n2, assign27610_e47490_d_n3, assign27610_e47490_d_n4, assign27610_e47490_d_n5, assign27610_e47490_d_n6, assign27610_e47490_d_n7, assign27610_e47490_d_n8, assign27610_e47490_d_n9, assign27610_e47490_d_n10, assign27610_e47490_d_n11, assign27610_e47490_d_n13, assign27610_e47490_d_n14,) = {
    if ((locals.var_guard444 != 0.0) && (locals.var_guard445 != 0.0)) {
        (locals.var_qi_acc_for_qm, locals.var_qi_acc_for_qm_dn0, locals.var_qi_acc_for_qm_dn2, locals.var_qi_acc_for_qm_dn3, locals.var_qi_acc_for_qm_dn4, locals.var_qi_acc_for_qm_dn5, locals.var_qi_acc_for_qm_dn6, locals.var_qi_acc_for_qm_dn7, locals.var_qi_acc_for_qm_dn8, locals.var_qi_acc_for_qm_dn9, locals.var_qi_acc_for_qm_dn10, locals.var_qi_acc_for_qm_dn11, locals.var_qi_acc_for_qm_dn13, locals.var_qi_acc_for_qm_dn14,)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn0, locals.var_voxacc_dn2, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn13, locals.var_voxacc_dn14,)
    }
};
        locals.var_voxacc = assign27610_e47490;
        locals.var_voxacc_dn0 = assign27610_e47490_d_n0;
        locals.var_voxacc_dn2 = assign27610_e47490_d_n2;
        locals.var_voxacc_dn3 = assign27610_e47490_d_n3;
        locals.var_voxacc_dn4 = assign27610_e47490_d_n4;
        locals.var_voxacc_dn5 = assign27610_e47490_d_n5;
        locals.var_voxacc_dn6 = assign27610_e47490_d_n6;
        locals.var_voxacc_dn7 = assign27610_e47490_d_n7;
        locals.var_voxacc_dn8 = assign27610_e47490_d_n8;
        locals.var_voxacc_dn9 = assign27610_e47490_d_n9;
        locals.var_voxacc_dn10 = assign27610_e47490_d_n10;
        locals.var_voxacc_dn11 = assign27610_e47490_d_n11;
        locals.var_voxacc_dn13 = assign27610_e47490_d_n13;
        locals.var_voxacc_dn14 = assign27610_e47490_d_n14;
        locals.var_voxacc_rv = 0.0;

        let assign27620_e47493: f64 = if locals.var_vfbzb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign27620_e47493;
        locals.var_guard446_rv = 0.0;

        let (assign27630_e47519, assign27630_e47519_d_n0, assign27630_e47519_d_n2, assign27630_e47519_d_n3, assign27630_e47519_d_n4, assign27630_e47519_d_n5, assign27630_e47519_d_n6, assign27630_e47519_d_n7, assign27630_e47519_d_n8, assign27630_e47519_d_n9, assign27630_e47519_d_n10, assign27630_e47519_d_n11, assign27630_e47519_d_n13, assign27630_e47519_d_n14,) = {
    if (((locals.var_guard444 != 0.0) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 != 0.0)) {
        let assign27630_e47503: f64 = (locals.var_t0 - 0.02);
        let assign27630_e47506: f64 = (locals.var_t0 - 0.02);
        let assign27630_e47509: f64 = (locals.var_t0 - 0.02);
        let assign27630_e47510: f64 = (assign27630_e47506 * assign27630_e47509);
        let assign27630_e47513: f64 = (0.08 * locals.var_vfbzb);
        let assign27630_e47514: f64 = (assign27630_e47510 - assign27630_e47513);
        let assign27630_e47515: f64 = (assign27630_e47514).sqrt();
        let assign27630_e47516: f64 = (assign27630_e47503 + assign27630_e47515);
        let assign27630_e47517: f64 = (0.5 * assign27630_e47516);
        (assign27630_e47517, (0.5 * (locals.var_t0_dn0 + ((((locals.var_t0_dn0 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn0)) - (0.08 * locals.var_vfbzb_dn0)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn2 + ((((locals.var_t0_dn2 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn2)) - (0.08 * locals.var_vfbzb_dn2)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn3 + ((((locals.var_t0_dn3 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn3)) - (0.08 * locals.var_vfbzb_dn3)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn4 + ((((locals.var_t0_dn4 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn4)) - (0.08 * locals.var_vfbzb_dn4)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn5 + ((((locals.var_t0_dn5 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn5)) - (0.08 * locals.var_vfbzb_dn5)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn6 + ((((locals.var_t0_dn6 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn6)) - (0.08 * locals.var_vfbzb_dn6)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn7 + ((((locals.var_t0_dn7 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn7)) - (0.08 * locals.var_vfbzb_dn7)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn8 + ((((locals.var_t0_dn8 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn8)) - (0.08 * locals.var_vfbzb_dn8)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn9 + ((((locals.var_t0_dn9 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn9)) - (0.08 * locals.var_vfbzb_dn9)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn10 + ((((locals.var_t0_dn10 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn10)) - (0.08 * locals.var_vfbzb_dn10)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn11 + ((((locals.var_t0_dn11 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn11)) - (0.08 * locals.var_vfbzb_dn11)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn13 + ((((locals.var_t0_dn13 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn13)) - (0.08 * locals.var_vfbzb_dn13)) / (2.0 * assign27630_e47515)))), (0.5 * (locals.var_t0_dn14 + ((((locals.var_t0_dn14 * assign27630_e47509) + (assign27630_e47506 * locals.var_t0_dn14)) - (0.08 * locals.var_vfbzb_dn14)) / (2.0 * assign27630_e47515)))),)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn0, locals.var_voxacc_dn2, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn13, locals.var_voxacc_dn14,)
    }
};
        locals.var_voxacc = assign27630_e47519;
        locals.var_voxacc_dn0 = assign27630_e47519_d_n0;
        locals.var_voxacc_dn2 = assign27630_e47519_d_n2;
        locals.var_voxacc_dn3 = assign27630_e47519_d_n3;
        locals.var_voxacc_dn4 = assign27630_e47519_d_n4;
        locals.var_voxacc_dn5 = assign27630_e47519_d_n5;
        locals.var_voxacc_dn6 = assign27630_e47519_d_n6;
        locals.var_voxacc_dn7 = assign27630_e47519_d_n7;
        locals.var_voxacc_dn8 = assign27630_e47519_d_n8;
        locals.var_voxacc_dn9 = assign27630_e47519_d_n9;
        locals.var_voxacc_dn10 = assign27630_e47519_d_n10;
        locals.var_voxacc_dn11 = assign27630_e47519_d_n11;
        locals.var_voxacc_dn13 = assign27630_e47519_d_n13;
        locals.var_voxacc_dn14 = assign27630_e47519_d_n14;
        locals.var_voxacc_rv = 0.0;

        let (assign27640_e47546, assign27640_e47546_d_n0, assign27640_e47546_d_n2, assign27640_e47546_d_n3, assign27640_e47546_d_n4, assign27640_e47546_d_n5, assign27640_e47546_d_n6, assign27640_e47546_d_n7, assign27640_e47546_d_n8, assign27640_e47546_d_n9, assign27640_e47546_d_n10, assign27640_e47546_d_n11, assign27640_e47546_d_n13, assign27640_e47546_d_n14,) = {
    if (((locals.var_guard444 != 0.0) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign27640_e47530: f64 = (locals.var_t0 - 0.02);
        let assign27640_e47533: f64 = (locals.var_t0 - 0.02);
        let assign27640_e47536: f64 = (locals.var_t0 - 0.02);
        let assign27640_e47537: f64 = (assign27640_e47533 * assign27640_e47536);
        let assign27640_e47540: f64 = (0.08 * locals.var_vfbzb);
        let assign27640_e47541: f64 = (assign27640_e47537 + assign27640_e47540);
        let assign27640_e47542: f64 = (assign27640_e47541).sqrt();
        let assign27640_e47543: f64 = (assign27640_e47530 + assign27640_e47542);
        let assign27640_e47544: f64 = (0.5 * assign27640_e47543);
        (assign27640_e47544, (0.5 * (locals.var_t0_dn0 + ((((locals.var_t0_dn0 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn0)) + (0.08 * locals.var_vfbzb_dn0)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn2 + ((((locals.var_t0_dn2 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn2)) + (0.08 * locals.var_vfbzb_dn2)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn3 + ((((locals.var_t0_dn3 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn3)) + (0.08 * locals.var_vfbzb_dn3)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn4 + ((((locals.var_t0_dn4 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn4)) + (0.08 * locals.var_vfbzb_dn4)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn5 + ((((locals.var_t0_dn5 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn5)) + (0.08 * locals.var_vfbzb_dn5)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn6 + ((((locals.var_t0_dn6 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn6)) + (0.08 * locals.var_vfbzb_dn6)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn7 + ((((locals.var_t0_dn7 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn7)) + (0.08 * locals.var_vfbzb_dn7)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn8 + ((((locals.var_t0_dn8 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn8)) + (0.08 * locals.var_vfbzb_dn8)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn9 + ((((locals.var_t0_dn9 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn9)) + (0.08 * locals.var_vfbzb_dn9)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn10 + ((((locals.var_t0_dn10 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn10)) + (0.08 * locals.var_vfbzb_dn10)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn11 + ((((locals.var_t0_dn11 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn11)) + (0.08 * locals.var_vfbzb_dn11)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn13 + ((((locals.var_t0_dn13 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn13)) + (0.08 * locals.var_vfbzb_dn13)) / (2.0 * assign27640_e47542)))), (0.5 * (locals.var_t0_dn14 + ((((locals.var_t0_dn14 * assign27640_e47536) + (assign27640_e47533 * locals.var_t0_dn14)) + (0.08 * locals.var_vfbzb_dn14)) / (2.0 * assign27640_e47542)))),)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn0, locals.var_voxacc_dn2, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn13, locals.var_voxacc_dn14,)
    }
};
        locals.var_voxacc = assign27640_e47546;
        locals.var_voxacc_dn0 = assign27640_e47546_d_n0;
        locals.var_voxacc_dn2 = assign27640_e47546_d_n2;
        locals.var_voxacc_dn3 = assign27640_e47546_d_n3;
        locals.var_voxacc_dn4 = assign27640_e47546_d_n4;
        locals.var_voxacc_dn5 = assign27640_e47546_d_n5;
        locals.var_voxacc_dn6 = assign27640_e47546_d_n6;
        locals.var_voxacc_dn7 = assign27640_e47546_d_n7;
        locals.var_voxacc_dn8 = assign27640_e47546_d_n8;
        locals.var_voxacc_dn9 = assign27640_e47546_d_n9;
        locals.var_voxacc_dn10 = assign27640_e47546_d_n10;
        locals.var_voxacc_dn11 = assign27640_e47546_d_n11;
        locals.var_voxacc_dn13 = assign27640_e47546_d_n13;
        locals.var_voxacc_dn14 = assign27640_e47546_d_n14;
        locals.var_voxacc_rv = 0.0;

        let (assign27650_e47594, assign27650_e47594_d_n0, assign27650_e47594_d_n2, assign27650_e47594_d_n3, assign27650_e47594_d_n4, assign27650_e47594_d_n5, assign27650_e47594_d_n6, assign27650_e47594_d_n7, assign27650_e47594_d_n8, assign27650_e47594_d_n9, assign27650_e47594_d_n10, assign27650_e47594_d_n11, assign27650_e47594_d_n13, assign27650_e47594_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27650_e47549: f64 = (-p.p1111);
        let assign27650_e47554: f64 = (locals.var_bigbacc_i * locals.var_voxacc);
        let assign27650_e47555: f64 = (locals.var_aigbacc_t - assign27650_e47554);
        let assign27650_e47557: f64 = (-p.p1111);
        let assign27650_e47558: f64 = (assign27650_e47555 - assign27650_e47557);
        let assign27650_e47560: f64 = (assign27650_e47558 - 1e-6);
        let assign27650_e47564: f64 = (locals.var_bigbacc_i * locals.var_voxacc);
        let assign27650_e47565: f64 = (locals.var_aigbacc_t - assign27650_e47564);
        let assign27650_e47567: f64 = (-p.p1111);
        let assign27650_e47568: f64 = (assign27650_e47565 - assign27650_e47567);
        let assign27650_e47570: f64 = (assign27650_e47568 - 1e-6);
        let assign27650_e47574: f64 = (locals.var_bigbacc_i * locals.var_voxacc);
        let assign27650_e47575: f64 = (locals.var_aigbacc_t - assign27650_e47574);
        let assign27650_e47577: f64 = (-p.p1111);
        let assign27650_e47578: f64 = (assign27650_e47575 - assign27650_e47577);
        let assign27650_e47580: f64 = (assign27650_e47578 - 1e-6);
        let assign27650_e47581: f64 = (assign27650_e47570 * assign27650_e47580);
        let assign27650_e47584: f64 = (-p.p1111);
        let assign27650_e47585: f64 = (4.0 * assign27650_e47584);
        let assign27650_e47587: f64 = (assign27650_e47585 * 1e-6);
        let assign27650_e47588: f64 = (assign27650_e47581 - assign27650_e47587);
        let assign27650_e47589: f64 = (assign27650_e47588).sqrt();
        let assign27650_e47590: f64 = (assign27650_e47560 + assign27650_e47589);
        let assign27650_e47591: f64 = (0.5 * assign27650_e47590);
        let assign27650_e47592: f64 = (assign27650_e47549 + assign27650_e47591);
        (assign27650_e47592, (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn0)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn0)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn0)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn2)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn2)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn2)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn3)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn3)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn3)))) / (2.0 * assign27650_e47589)))), (0.5 * ((locals.var_aigbacc_t_dn4 - (locals.var_bigbacc_i * locals.var_voxacc_dn4)) + ((((locals.var_aigbacc_t_dn4 - (locals.var_bigbacc_i * locals.var_voxacc_dn4)) * assign27650_e47580) + (assign27650_e47570 * (locals.var_aigbacc_t_dn4 - (locals.var_bigbacc_i * locals.var_voxacc_dn4)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn5)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn5)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn5)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn6)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn6)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn6)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn7)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn7)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn7)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn8)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn8)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn8)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn9)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn9)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn9)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn10)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn10)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn10)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn11)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn11)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn11)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn13)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn13)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn13)))) / (2.0 * assign27650_e47589)))), (0.5 * ((-(locals.var_bigbacc_i * locals.var_voxacc_dn14)) + ((((-(locals.var_bigbacc_i * locals.var_voxacc_dn14)) * assign27650_e47580) + (assign27650_e47570 * (-(locals.var_bigbacc_i * locals.var_voxacc_dn14)))) / (2.0 * assign27650_e47589)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27650_e47594;
        locals.var_t2_dn0 = assign27650_e47594_d_n0;
        locals.var_t2_dn2 = assign27650_e47594_d_n2;
        locals.var_t2_dn3 = assign27650_e47594_d_n3;
        locals.var_t2_dn4 = assign27650_e47594_d_n4;
        locals.var_t2_dn5 = assign27650_e47594_d_n5;
        locals.var_t2_dn6 = assign27650_e47594_d_n6;
        locals.var_t2_dn7 = assign27650_e47594_d_n7;
        locals.var_t2_dn8 = assign27650_e47594_d_n8;
        locals.var_t2_dn9 = assign27650_e47594_d_n9;
        locals.var_t2_dn10 = assign27650_e47594_d_n10;
        locals.var_t2_dn11 = assign27650_e47594_d_n11;
        locals.var_t2_dn13 = assign27650_e47594_d_n13;
        locals.var_t2_dn14 = assign27650_e47594_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27660_e47602, assign27660_e47602_d_n0, assign27660_e47602_d_n2, assign27660_e47602_d_n3, assign27660_e47602_d_n4, assign27660_e47602_d_n5, assign27660_e47602_d_n6, assign27660_e47602_d_n7, assign27660_e47602_d_n8, assign27660_e47602_d_n9, assign27660_e47602_d_n10, assign27660_e47602_d_n11, assign27660_e47602_d_n13, assign27660_e47602_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27660_e47599: f64 = (locals.var_cigbacc_i * locals.var_voxacc);
        let assign27660_e47600: f64 = (1.0 + assign27660_e47599);
        (assign27660_e47600, (locals.var_cigbacc_i * locals.var_voxacc_dn0), (locals.var_cigbacc_i * locals.var_voxacc_dn2), (locals.var_cigbacc_i * locals.var_voxacc_dn3), (locals.var_cigbacc_i * locals.var_voxacc_dn4), (locals.var_cigbacc_i * locals.var_voxacc_dn5), (locals.var_cigbacc_i * locals.var_voxacc_dn6), (locals.var_cigbacc_i * locals.var_voxacc_dn7), (locals.var_cigbacc_i * locals.var_voxacc_dn8), (locals.var_cigbacc_i * locals.var_voxacc_dn9), (locals.var_cigbacc_i * locals.var_voxacc_dn10), (locals.var_cigbacc_i * locals.var_voxacc_dn11), (locals.var_cigbacc_i * locals.var_voxacc_dn13), (locals.var_cigbacc_i * locals.var_voxacc_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27660_e47602;
        locals.var_t3_dn0 = assign27660_e47602_d_n0;
        locals.var_t3_dn2 = assign27660_e47602_d_n2;
        locals.var_t3_dn3 = assign27660_e47602_d_n3;
        locals.var_t3_dn4 = assign27660_e47602_d_n4;
        locals.var_t3_dn5 = assign27660_e47602_d_n5;
        locals.var_t3_dn6 = assign27660_e47602_d_n6;
        locals.var_t3_dn7 = assign27660_e47602_d_n7;
        locals.var_t3_dn8 = assign27660_e47602_d_n8;
        locals.var_t3_dn9 = assign27660_e47602_d_n9;
        locals.var_t3_dn10 = assign27660_e47602_d_n10;
        locals.var_t3_dn11 = assign27660_e47602_d_n11;
        locals.var_t3_dn13 = assign27660_e47602_d_n13;
        locals.var_t3_dn14 = assign27660_e47602_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27670_e47613, assign27670_e47613_d_n0, assign27670_e47613_d_n2, assign27670_e47613_d_n3, assign27670_e47613_d_n4, assign27670_e47613_d_n5, assign27670_e47613_d_n6, assign27670_e47613_d_n7, assign27670_e47613_d_n8, assign27670_e47613_d_n9, assign27670_e47613_d_n10, assign27670_e47613_d_n11, assign27670_e47613_d_n13, assign27670_e47613_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27670_e47605: f64 = (-745669000000.0);
        let assign27670_e47607: f64 = (assign27670_e47605 * p.p1109);
        let assign27670_e47609: f64 = (assign27670_e47607 * locals.var_t2);
        let assign27670_e47611: f64 = (assign27670_e47609 * locals.var_t3);
        (assign27670_e47611, (((assign27670_e47607 * locals.var_t2_dn0) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn0)), (((assign27670_e47607 * locals.var_t2_dn2) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn2)), (((assign27670_e47607 * locals.var_t2_dn3) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn3)), (((assign27670_e47607 * locals.var_t2_dn4) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn4)), (((assign27670_e47607 * locals.var_t2_dn5) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn5)), (((assign27670_e47607 * locals.var_t2_dn6) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn6)), (((assign27670_e47607 * locals.var_t2_dn7) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn7)), (((assign27670_e47607 * locals.var_t2_dn8) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn8)), (((assign27670_e47607 * locals.var_t2_dn9) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn9)), (((assign27670_e47607 * locals.var_t2_dn10) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn10)), (((assign27670_e47607 * locals.var_t2_dn11) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn11)), (((assign27670_e47607 * locals.var_t2_dn13) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn13)), (((assign27670_e47607 * locals.var_t2_dn14) * locals.var_t3) + (assign27670_e47609 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign27670_e47613;
        locals.var_t4_dn0 = assign27670_e47613_d_n0;
        locals.var_t4_dn2 = assign27670_e47613_d_n2;
        locals.var_t4_dn3 = assign27670_e47613_d_n3;
        locals.var_t4_dn4 = assign27670_e47613_d_n4;
        locals.var_t4_dn5 = assign27670_e47613_d_n5;
        locals.var_t4_dn6 = assign27670_e47613_d_n6;
        locals.var_t4_dn7 = assign27670_e47613_d_n7;
        locals.var_t4_dn8 = assign27670_e47613_d_n8;
        locals.var_t4_dn9 = assign27670_e47613_d_n9;
        locals.var_t4_dn10 = assign27670_e47613_d_n10;
        locals.var_t4_dn11 = assign27670_e47613_d_n11;
        locals.var_t4_dn13 = assign27670_e47613_d_n13;
        locals.var_t4_dn14 = assign27670_e47613_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign27680_e47618, assign27680_e47618_d_n0, assign27680_e47618_d_n2, assign27680_e47618_d_n3, assign27680_e47618_d_n4, assign27680_e47618_d_n5, assign27680_e47618_d_n6, assign27680_e47618_d_n7, assign27680_e47618_d_n8, assign27680_e47618_d_n9, assign27680_e47618_d_n10, assign27680_e47618_d_n11, assign27680_e47618_d_n13, assign27680_e47618_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        let assign27680_e47616: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27680_e47616, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign27680_e47618;
        locals.var_t5_dn0 = assign27680_e47618_d_n0;
        locals.var_t5_dn2 = assign27680_e47618_d_n2;
        locals.var_t5_dn3 = assign27680_e47618_d_n3;
        locals.var_t5_dn4 = assign27680_e47618_d_n4;
        locals.var_t5_dn5 = assign27680_e47618_d_n5;
        locals.var_t5_dn6 = assign27680_e47618_d_n6;
        locals.var_t5_dn7 = assign27680_e47618_d_n7;
        locals.var_t5_dn8 = assign27680_e47618_d_n8;
        locals.var_t5_dn9 = assign27680_e47618_d_n9;
        locals.var_t5_dn10 = assign27680_e47618_d_n10;
        locals.var_t5_dn11 = assign27680_e47618_d_n11;
        locals.var_t5_dn13 = assign27680_e47618_d_n13;
        locals.var_t5_dn14 = assign27680_e47618_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign27690_e47622, assign27690_e47622_d_n0, assign27690_e47622_d_n2, assign27690_e47622_d_n3, assign27690_e47622_d_n4, assign27690_e47622_d_n5, assign27690_e47622_d_n6, assign27690_e47622_d_n7, assign27690_e47622_d_n8, assign27690_e47622_d_n9, assign27690_e47622_d_n10, assign27690_e47622_d_n11, assign27690_e47622_d_n13, assign27690_e47622_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        (4.97232e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign27690_e47622;
        locals.var_t6_dn0 = assign27690_e47622_d_n0;
        locals.var_t6_dn2 = assign27690_e47622_d_n2;
        locals.var_t6_dn3 = assign27690_e47622_d_n3;
        locals.var_t6_dn4 = assign27690_e47622_d_n4;
        locals.var_t6_dn5 = assign27690_e47622_d_n5;
        locals.var_t6_dn6 = assign27690_e47622_d_n6;
        locals.var_t6_dn7 = assign27690_e47622_d_n7;
        locals.var_t6_dn8 = assign27690_e47622_d_n8;
        locals.var_t6_dn9 = assign27690_e47622_d_n9;
        locals.var_t6_dn10 = assign27690_e47622_d_n10;
        locals.var_t6_dn11 = assign27690_e47622_d_n11;
        locals.var_t6_dn13 = assign27690_e47622_d_n13;
        locals.var_t6_dn14 = assign27690_e47622_d_n14;
        locals.var_t6_rv = 0.0;

        let assign27720_e47649: f64 = if p.p68 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign27720_e47649;
        locals.var_guard447_rv = 0.0;

        let (assign27730_e47697, assign27730_e47697_d_n0, assign27730_e47697_d_n2, assign27730_e47697_d_n3, assign27730_e47697_d_n4, assign27730_e47697_d_n5, assign27730_e47697_d_n6, assign27730_e47697_d_n7, assign27730_e47697_d_n8, assign27730_e47697_d_n9, assign27730_e47697_d_n10, assign27730_e47697_d_n11, assign27730_e47697_d_n13, assign27730_e47697_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27730_e47652: f64 = (-p.p1112);
        let assign27730_e47657: f64 = (locals.var_bigc_i * locals.var_qia);
        let assign27730_e47658: f64 = (locals.var_aigc_t - assign27730_e47657);
        let assign27730_e47660: f64 = (-p.p1112);
        let assign27730_e47661: f64 = (assign27730_e47658 - assign27730_e47660);
        let assign27730_e47663: f64 = (assign27730_e47661 - 1e-6);
        let assign27730_e47667: f64 = (locals.var_bigc_i * locals.var_qia);
        let assign27730_e47668: f64 = (locals.var_aigc_t - assign27730_e47667);
        let assign27730_e47670: f64 = (-p.p1112);
        let assign27730_e47671: f64 = (assign27730_e47668 - assign27730_e47670);
        let assign27730_e47673: f64 = (assign27730_e47671 - 1e-6);
        let assign27730_e47677: f64 = (locals.var_bigc_i * locals.var_qia);
        let assign27730_e47678: f64 = (locals.var_aigc_t - assign27730_e47677);
        let assign27730_e47680: f64 = (-p.p1112);
        let assign27730_e47681: f64 = (assign27730_e47678 - assign27730_e47680);
        let assign27730_e47683: f64 = (assign27730_e47681 - 1e-6);
        let assign27730_e47684: f64 = (assign27730_e47673 * assign27730_e47683);
        let assign27730_e47687: f64 = (-p.p1112);
        let assign27730_e47688: f64 = (4.0 * assign27730_e47687);
        let assign27730_e47690: f64 = (assign27730_e47688 * 1e-6);
        let assign27730_e47691: f64 = (assign27730_e47684 - assign27730_e47690);
        let assign27730_e47692: f64 = (assign27730_e47691).sqrt();
        let assign27730_e47693: f64 = (assign27730_e47663 + assign27730_e47692);
        let assign27730_e47694: f64 = (0.5 * assign27730_e47693);
        let assign27730_e47695: f64 = (assign27730_e47652 + assign27730_e47694);
        (assign27730_e47695, (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn0)) + ((((-(locals.var_bigc_i * locals.var_qia_dn0)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn0)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn2)) + ((((-(locals.var_bigc_i * locals.var_qia_dn2)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn2)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn3)) + ((((-(locals.var_bigc_i * locals.var_qia_dn3)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn3)))) / (2.0 * assign27730_e47692)))), (0.5 * ((locals.var_aigc_t_dn4 - (locals.var_bigc_i * locals.var_qia_dn4)) + ((((locals.var_aigc_t_dn4 - (locals.var_bigc_i * locals.var_qia_dn4)) * assign27730_e47683) + (assign27730_e47673 * (locals.var_aigc_t_dn4 - (locals.var_bigc_i * locals.var_qia_dn4)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn5)) + ((((-(locals.var_bigc_i * locals.var_qia_dn5)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn5)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn6)) + ((((-(locals.var_bigc_i * locals.var_qia_dn6)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn6)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn7)) + ((((-(locals.var_bigc_i * locals.var_qia_dn7)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn7)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn8)) + ((((-(locals.var_bigc_i * locals.var_qia_dn8)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn8)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn9)) + ((((-(locals.var_bigc_i * locals.var_qia_dn9)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn9)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn10)) + ((((-(locals.var_bigc_i * locals.var_qia_dn10)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn10)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn11)) + ((((-(locals.var_bigc_i * locals.var_qia_dn11)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn11)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn13)) + ((((-(locals.var_bigc_i * locals.var_qia_dn13)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn13)))) / (2.0 * assign27730_e47692)))), (0.5 * ((-(locals.var_bigc_i * locals.var_qia_dn14)) + ((((-(locals.var_bigc_i * locals.var_qia_dn14)) * assign27730_e47683) + (assign27730_e47673 * (-(locals.var_bigc_i * locals.var_qia_dn14)))) / (2.0 * assign27730_e47692)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27730_e47697;
        locals.var_t1_dn0 = assign27730_e47697_d_n0;
        locals.var_t1_dn2 = assign27730_e47697_d_n2;
        locals.var_t1_dn3 = assign27730_e47697_d_n3;
        locals.var_t1_dn4 = assign27730_e47697_d_n4;
        locals.var_t1_dn5 = assign27730_e47697_d_n5;
        locals.var_t1_dn6 = assign27730_e47697_d_n6;
        locals.var_t1_dn7 = assign27730_e47697_d_n7;
        locals.var_t1_dn8 = assign27730_e47697_d_n8;
        locals.var_t1_dn9 = assign27730_e47697_d_n9;
        locals.var_t1_dn10 = assign27730_e47697_d_n10;
        locals.var_t1_dn11 = assign27730_e47697_d_n11;
        locals.var_t1_dn13 = assign27730_e47697_d_n13;
        locals.var_t1_dn14 = assign27730_e47697_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign27740_e47705, assign27740_e47705_d_n0, assign27740_e47705_d_n2, assign27740_e47705_d_n3, assign27740_e47705_d_n4, assign27740_e47705_d_n5, assign27740_e47705_d_n6, assign27740_e47705_d_n7, assign27740_e47705_d_n8, assign27740_e47705_d_n9, assign27740_e47705_d_n10, assign27740_e47705_d_n11, assign27740_e47705_d_n13, assign27740_e47705_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27740_e47702: f64 = (locals.var_cigc_i * locals.var_qia);
        let assign27740_e47703: f64 = (1.0 + assign27740_e47702);
        (assign27740_e47703, (locals.var_cigc_i * locals.var_qia_dn0), (locals.var_cigc_i * locals.var_qia_dn2), (locals.var_cigc_i * locals.var_qia_dn3), (locals.var_cigc_i * locals.var_qia_dn4), (locals.var_cigc_i * locals.var_qia_dn5), (locals.var_cigc_i * locals.var_qia_dn6), (locals.var_cigc_i * locals.var_qia_dn7), (locals.var_cigc_i * locals.var_qia_dn8), (locals.var_cigc_i * locals.var_qia_dn9), (locals.var_cigc_i * locals.var_qia_dn10), (locals.var_cigc_i * locals.var_qia_dn11), (locals.var_cigc_i * locals.var_qia_dn13), (locals.var_cigc_i * locals.var_qia_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27740_e47705;
        locals.var_t2_dn0 = assign27740_e47705_d_n0;
        locals.var_t2_dn2 = assign27740_e47705_d_n2;
        locals.var_t2_dn3 = assign27740_e47705_d_n3;
        locals.var_t2_dn4 = assign27740_e47705_d_n4;
        locals.var_t2_dn5 = assign27740_e47705_d_n5;
        locals.var_t2_dn6 = assign27740_e47705_d_n6;
        locals.var_t2_dn7 = assign27740_e47705_d_n7;
        locals.var_t2_dn8 = assign27740_e47705_d_n8;
        locals.var_t2_dn9 = assign27740_e47705_d_n9;
        locals.var_t2_dn10 = assign27740_e47705_d_n10;
        locals.var_t2_dn11 = assign27740_e47705_d_n11;
        locals.var_t2_dn13 = assign27740_e47705_d_n13;
        locals.var_t2_dn14 = assign27740_e47705_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27750_e47716, assign27750_e47716_d_n0, assign27750_e47716_d_n2, assign27750_e47716_d_n3, assign27750_e47716_d_n4, assign27750_e47716_d_n5, assign27750_e47716_d_n6, assign27750_e47716_d_n7, assign27750_e47716_d_n8, assign27750_e47716_d_n9, assign27750_e47716_d_n10, assign27750_e47716_d_n11, assign27750_e47716_d_n13, assign27750_e47716_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27750_e47708: f64 = (-locals.var_bechvb);
        let assign27750_e47710: f64 = (assign27750_e47708 * p.p1109);
        let assign27750_e47712: f64 = (assign27750_e47710 * locals.var_t1);
        let assign27750_e47714: f64 = (assign27750_e47712 * locals.var_t2);
        (assign27750_e47714, (((assign27750_e47710 * locals.var_t1_dn0) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn0)), (((assign27750_e47710 * locals.var_t1_dn2) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn2)), (((assign27750_e47710 * locals.var_t1_dn3) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn3)), (((assign27750_e47710 * locals.var_t1_dn4) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn4)), (((assign27750_e47710 * locals.var_t1_dn5) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn5)), (((assign27750_e47710 * locals.var_t1_dn6) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn6)), (((assign27750_e47710 * locals.var_t1_dn7) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn7)), (((assign27750_e47710 * locals.var_t1_dn8) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn8)), (((assign27750_e47710 * locals.var_t1_dn9) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn9)), (((assign27750_e47710 * locals.var_t1_dn10) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn10)), (((assign27750_e47710 * locals.var_t1_dn11) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn11)), (((assign27750_e47710 * locals.var_t1_dn13) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn13)), (((assign27750_e47710 * locals.var_t1_dn14) * locals.var_t2) + (assign27750_e47712 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27750_e47716;
        locals.var_t3_dn0 = assign27750_e47716_d_n0;
        locals.var_t3_dn2 = assign27750_e47716_d_n2;
        locals.var_t3_dn3 = assign27750_e47716_d_n3;
        locals.var_t3_dn4 = assign27750_e47716_d_n4;
        locals.var_t3_dn5 = assign27750_e47716_d_n5;
        locals.var_t3_dn6 = assign27750_e47716_d_n6;
        locals.var_t3_dn7 = assign27750_e47716_d_n7;
        locals.var_t3_dn8 = assign27750_e47716_d_n8;
        locals.var_t3_dn9 = assign27750_e47716_d_n9;
        locals.var_t3_dn10 = assign27750_e47716_d_n10;
        locals.var_t3_dn11 = assign27750_e47716_d_n11;
        locals.var_t3_dn13 = assign27750_e47716_d_n13;
        locals.var_t3_dn14 = assign27750_e47716_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27760_e47723, assign27760_e47723_d_n0, assign27760_e47723_d_n2, assign27760_e47723_d_n3, assign27760_e47723_d_n4, assign27760_e47723_d_n5, assign27760_e47723_d_n6, assign27760_e47723_d_n7, assign27760_e47723_d_n8, assign27760_e47723_d_n9, assign27760_e47723_d_n10, assign27760_e47723_d_n11, assign27760_e47723_d_n13, assign27760_e47723_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27760_e47720: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign27760_e47721: f64 = (locals.var_qia * assign27760_e47720);
        (assign27760_e47721, ((locals.var_qia_dn0 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0))), ((locals.var_qia_dn2 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2))), ((locals.var_qia_dn3 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3))), ((locals.var_qia_dn4 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4))), ((locals.var_qia_dn5 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5))), ((locals.var_qia_dn6 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6))), ((locals.var_qia_dn7 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7))), ((locals.var_qia_dn8 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8))), ((locals.var_qia_dn9 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9))), ((locals.var_qia_dn10 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10))), ((locals.var_qia_dn11 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11))), ((locals.var_qia_dn13 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13))), ((locals.var_qia_dn14 * assign27760_e47720) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign27760_e47723;
        locals.var_t4_dn0 = assign27760_e47723_d_n0;
        locals.var_t4_dn2 = assign27760_e47723_d_n2;
        locals.var_t4_dn3 = assign27760_e47723_d_n3;
        locals.var_t4_dn4 = assign27760_e47723_d_n4;
        locals.var_t4_dn5 = assign27760_e47723_d_n5;
        locals.var_t4_dn6 = assign27760_e47723_d_n6;
        locals.var_t4_dn7 = assign27760_e47723_d_n7;
        locals.var_t4_dn8 = assign27760_e47723_d_n8;
        locals.var_t4_dn9 = assign27760_e47723_d_n9;
        locals.var_t4_dn10 = assign27760_e47723_d_n10;
        locals.var_t4_dn11 = assign27760_e47723_d_n11;
        locals.var_t4_dn13 = assign27760_e47723_d_n13;
        locals.var_t4_dn14 = assign27760_e47723_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign27770_e47737, assign27770_e47737_d_n0, assign27770_e47737_d_n2, assign27770_e47737_d_n3, assign27770_e47737_d_n4, assign27770_e47737_d_n5, assign27770_e47737_d_n6, assign27770_e47737_d_n7, assign27770_e47737_d_n8, assign27770_e47737_d_n9, assign27770_e47737_d_n10, assign27770_e47737_d_n11, assign27770_e47737_d_n13, assign27770_e47737_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27770_e47728: f64 = (0.5 * locals.var_vdsx);
        let assign27770_e47729: f64 = (locals.var_vge + assign27770_e47728);
        let assign27770_e47733: f64 = (locals.var_ves_jct + locals.var_ved_jct);
        let assign27770_e47734: f64 = (0.5 * assign27770_e47733);
        let assign27770_e47735: f64 = (assign27770_e47729 + assign27770_e47734);
        (assign27770_e47735, 0.0, 0.0, (locals.var_vge_dn3 + (0.5 * (locals.var_ves_jct_dn3 + locals.var_ved_jct_dn3))), 0.0, ((0.5 * locals.var_vdsx_dn5) + (0.5 * locals.var_ved_jct_dn5)), ((0.5 * locals.var_vdsx_dn6) + (0.5 * locals.var_ves_jct_dn6)), 0.0, 0.0, 0.0, 0.0, locals.var_vge_dn11, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign27770_e47737;
        locals.var_t5_dn0 = assign27770_e47737_d_n0;
        locals.var_t5_dn2 = assign27770_e47737_d_n2;
        locals.var_t5_dn3 = assign27770_e47737_d_n3;
        locals.var_t5_dn4 = assign27770_e47737_d_n4;
        locals.var_t5_dn5 = assign27770_e47737_d_n5;
        locals.var_t5_dn6 = assign27770_e47737_d_n6;
        locals.var_t5_dn7 = assign27770_e47737_d_n7;
        locals.var_t5_dn8 = assign27770_e47737_d_n8;
        locals.var_t5_dn9 = assign27770_e47737_d_n9;
        locals.var_t5_dn10 = assign27770_e47737_d_n10;
        locals.var_t5_dn11 = assign27770_e47737_d_n11;
        locals.var_t5_dn13 = assign27770_e47737_d_n13;
        locals.var_t5_dn14 = assign27770_e47737_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign27790_e47766, assign27790_e47766_d_n0, assign27790_e47766_d_n2, assign27790_e47766_d_n3, assign27790_e47766_d_n4, assign27790_e47766_d_n5, assign27790_e47766_d_n6, assign27790_e47766_d_n7, assign27790_e47766_d_n8, assign27790_e47766_d_n9, assign27790_e47766_d_n10, assign27790_e47766_d_n11, assign27790_e47766_d_n13, assign27790_e47766_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27790_e47759: f64 = (locals.var_vdseff_1 * locals.var_vdseff_1);
        let assign27790_e47761: f64 = (assign27790_e47759 + 0.01);
        let assign27790_e47762: f64 = (assign27790_e47761).sqrt();
        let assign27790_e47764: f64 = (assign27790_e47762 - 0.1);
        (assign27790_e47764, (((locals.var_vdseff_1_dn0 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn0)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn2 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn2)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn3 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn3)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn4 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn4)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn5 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn5)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn6 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn6)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn7 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn7)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn8 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn8)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn9 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn9)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn10 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn10)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn11 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn11)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn13 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn13)) / (2.0 * assign27790_e47762)), (((locals.var_vdseff_1_dn14 * locals.var_vdseff_1) + (locals.var_vdseff_1 * locals.var_vdseff_1_dn14)) / (2.0 * assign27790_e47762)),)
    } else {
        (locals.var_vdseffx, locals.var_vdseffx_dn0, locals.var_vdseffx_dn2, locals.var_vdseffx_dn3, locals.var_vdseffx_dn4, locals.var_vdseffx_dn5, locals.var_vdseffx_dn6, locals.var_vdseffx_dn7, locals.var_vdseffx_dn8, locals.var_vdseffx_dn9, locals.var_vdseffx_dn10, locals.var_vdseffx_dn11, locals.var_vdseffx_dn13, locals.var_vdseffx_dn14,)
    }
};
        locals.var_vdseffx = assign27790_e47766;
        locals.var_vdseffx_dn0 = assign27790_e47766_d_n0;
        locals.var_vdseffx_dn2 = assign27790_e47766_d_n2;
        locals.var_vdseffx_dn3 = assign27790_e47766_d_n3;
        locals.var_vdseffx_dn4 = assign27790_e47766_d_n4;
        locals.var_vdseffx_dn5 = assign27790_e47766_d_n5;
        locals.var_vdseffx_dn6 = assign27790_e47766_d_n6;
        locals.var_vdseffx_dn7 = assign27790_e47766_d_n7;
        locals.var_vdseffx_dn8 = assign27790_e47766_d_n8;
        locals.var_vdseffx_dn9 = assign27790_e47766_d_n9;
        locals.var_vdseffx_dn10 = assign27790_e47766_d_n10;
        locals.var_vdseffx_dn11 = assign27790_e47766_d_n11;
        locals.var_vdseffx_dn13 = assign27790_e47766_d_n13;
        locals.var_vdseffx_dn14 = assign27790_e47766_d_n14;
        locals.var_vdseffx_rv = 0.0;

        let (assign27800_e47772, assign27800_e47772_d_n0, assign27800_e47772_d_n2, assign27800_e47772_d_n3, assign27800_e47772_d_n4, assign27800_e47772_d_n5, assign27800_e47772_d_n6, assign27800_e47772_d_n7, assign27800_e47772_d_n8, assign27800_e47772_d_n9, assign27800_e47772_d_n10, assign27800_e47772_d_n11, assign27800_e47772_d_n13, assign27800_e47772_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27800_e47770: f64 = (locals.var_pigcd_i * locals.var_vdseffx);
        (assign27800_e47770, (locals.var_pigcd_i * locals.var_vdseffx_dn0), (locals.var_pigcd_i * locals.var_vdseffx_dn2), (locals.var_pigcd_i * locals.var_vdseffx_dn3), (locals.var_pigcd_i * locals.var_vdseffx_dn4), (locals.var_pigcd_i * locals.var_vdseffx_dn5), (locals.var_pigcd_i * locals.var_vdseffx_dn6), (locals.var_pigcd_i * locals.var_vdseffx_dn7), (locals.var_pigcd_i * locals.var_vdseffx_dn8), (locals.var_pigcd_i * locals.var_vdseffx_dn9), (locals.var_pigcd_i * locals.var_vdseffx_dn10), (locals.var_pigcd_i * locals.var_vdseffx_dn11), (locals.var_pigcd_i * locals.var_vdseffx_dn13), (locals.var_pigcd_i * locals.var_vdseffx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27800_e47772;
        locals.var_t1_dn0 = assign27800_e47772_d_n0;
        locals.var_t1_dn2 = assign27800_e47772_d_n2;
        locals.var_t1_dn3 = assign27800_e47772_d_n3;
        locals.var_t1_dn4 = assign27800_e47772_d_n4;
        locals.var_t1_dn5 = assign27800_e47772_d_n5;
        locals.var_t1_dn6 = assign27800_e47772_d_n6;
        locals.var_t1_dn7 = assign27800_e47772_d_n7;
        locals.var_t1_dn8 = assign27800_e47772_d_n8;
        locals.var_t1_dn9 = assign27800_e47772_d_n9;
        locals.var_t1_dn10 = assign27800_e47772_d_n10;
        locals.var_t1_dn11 = assign27800_e47772_d_n11;
        locals.var_t1_dn13 = assign27800_e47772_d_n13;
        locals.var_t1_dn14 = assign27800_e47772_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27810_e47778, assign27810_e47778_d_n0, assign27810_e47778_d_n2, assign27810_e47778_d_n3, assign27810_e47778_d_n4, assign27810_e47778_d_n5, assign27810_e47778_d_n6, assign27810_e47778_d_n7, assign27810_e47778_d_n8, assign27810_e47778_d_n9, assign27810_e47778_d_n10, assign27810_e47778_d_n11, assign27810_e47778_d_n13, assign27810_e47778_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27810_e47775: f64 = (-locals.var_t1);
        let assign27810_e47776: f64 = { let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27810_e47776, ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13)), ({ let limited_exp_arg = assign27810_e47775; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14)),)
    } else {
        (locals.var_t1_exp, locals.var_t1_exp_dn0, locals.var_t1_exp_dn2, locals.var_t1_exp_dn3, locals.var_t1_exp_dn4, locals.var_t1_exp_dn5, locals.var_t1_exp_dn6, locals.var_t1_exp_dn7, locals.var_t1_exp_dn8, locals.var_t1_exp_dn9, locals.var_t1_exp_dn10, locals.var_t1_exp_dn11, locals.var_t1_exp_dn13, locals.var_t1_exp_dn14,)
    }
};
        locals.var_t1_exp = assign27810_e47778;
        locals.var_t1_exp_dn0 = assign27810_e47778_d_n0;
        locals.var_t1_exp_dn2 = assign27810_e47778_d_n2;
        locals.var_t1_exp_dn3 = assign27810_e47778_d_n3;
        locals.var_t1_exp_dn4 = assign27810_e47778_d_n4;
        locals.var_t1_exp_dn5 = assign27810_e47778_d_n5;
        locals.var_t1_exp_dn6 = assign27810_e47778_d_n6;
        locals.var_t1_exp_dn7 = assign27810_e47778_d_n7;
        locals.var_t1_exp_dn8 = assign27810_e47778_d_n8;
        locals.var_t1_exp_dn9 = assign27810_e47778_d_n9;
        locals.var_t1_exp_dn10 = assign27810_e47778_d_n10;
        locals.var_t1_exp_dn11 = assign27810_e47778_d_n11;
        locals.var_t1_exp_dn13 = assign27810_e47778_d_n13;
        locals.var_t1_exp_dn14 = assign27810_e47778_d_n14;
        locals.var_t1_exp_rv = 0.0;

        let (assign27820_e47788, assign27820_e47788_d_n0, assign27820_e47788_d_n2, assign27820_e47788_d_n3, assign27820_e47788_d_n4, assign27820_e47788_d_n5, assign27820_e47788_d_n6, assign27820_e47788_d_n7, assign27820_e47788_d_n8, assign27820_e47788_d_n9, assign27820_e47788_d_n10, assign27820_e47788_d_n11, assign27820_e47788_d_n13, assign27820_e47788_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27820_e47782: f64 = (locals.var_t1 + locals.var_t1_exp);
        let assign27820_e47784: f64 = (assign27820_e47782 - 1.0);
        let assign27820_e47786: f64 = (assign27820_e47784 + 0.0001);
        (assign27820_e47786, (locals.var_t1_dn0 + locals.var_t1_exp_dn0), (locals.var_t1_dn2 + locals.var_t1_exp_dn2), (locals.var_t1_dn3 + locals.var_t1_exp_dn3), (locals.var_t1_dn4 + locals.var_t1_exp_dn4), (locals.var_t1_dn5 + locals.var_t1_exp_dn5), (locals.var_t1_dn6 + locals.var_t1_exp_dn6), (locals.var_t1_dn7 + locals.var_t1_exp_dn7), (locals.var_t1_dn8 + locals.var_t1_exp_dn8), (locals.var_t1_dn9 + locals.var_t1_exp_dn9), (locals.var_t1_dn10 + locals.var_t1_exp_dn10), (locals.var_t1_dn11 + locals.var_t1_exp_dn11), (locals.var_t1_dn13 + locals.var_t1_exp_dn13), (locals.var_t1_dn14 + locals.var_t1_exp_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27820_e47788;
        locals.var_t3_dn0 = assign27820_e47788_d_n0;
        locals.var_t3_dn2 = assign27820_e47788_d_n2;
        locals.var_t3_dn3 = assign27820_e47788_d_n3;
        locals.var_t3_dn4 = assign27820_e47788_d_n4;
        locals.var_t3_dn5 = assign27820_e47788_d_n5;
        locals.var_t3_dn6 = assign27820_e47788_d_n6;
        locals.var_t3_dn7 = assign27820_e47788_d_n7;
        locals.var_t3_dn8 = assign27820_e47788_d_n8;
        locals.var_t3_dn9 = assign27820_e47788_d_n9;
        locals.var_t3_dn10 = assign27820_e47788_d_n10;
        locals.var_t3_dn11 = assign27820_e47788_d_n11;
        locals.var_t3_dn13 = assign27820_e47788_d_n13;
        locals.var_t3_dn14 = assign27820_e47788_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27830_e47800, assign27830_e47800_d_n0, assign27830_e47800_d_n2, assign27830_e47800_d_n3, assign27830_e47800_d_n4, assign27830_e47800_d_n5, assign27830_e47800_d_n6, assign27830_e47800_d_n7, assign27830_e47800_d_n8, assign27830_e47800_d_n9, assign27830_e47800_d_n10, assign27830_e47800_d_n11, assign27830_e47800_d_n13, assign27830_e47800_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27830_e47793: f64 = (locals.var_t1 + 1.0);
        let assign27830_e47795: f64 = (assign27830_e47793 * locals.var_t1_exp);
        let assign27830_e47796: f64 = (1.0 - assign27830_e47795);
        let assign27830_e47798: f64 = (assign27830_e47796 + 0.0001);
        (assign27830_e47798, (-((locals.var_t1_dn0 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn0))), (-((locals.var_t1_dn2 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn2))), (-((locals.var_t1_dn3 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn3))), (-((locals.var_t1_dn4 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn4))), (-((locals.var_t1_dn5 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn5))), (-((locals.var_t1_dn6 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn6))), (-((locals.var_t1_dn7 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn7))), (-((locals.var_t1_dn8 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn8))), (-((locals.var_t1_dn9 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn9))), (-((locals.var_t1_dn10 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn10))), (-((locals.var_t1_dn11 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn11))), (-((locals.var_t1_dn13 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn13))), (-((locals.var_t1_dn14 * locals.var_t1_exp) + (assign27830_e47793 * locals.var_t1_exp_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign27830_e47800;
        locals.var_t4_dn0 = assign27830_e47800_d_n0;
        locals.var_t4_dn2 = assign27830_e47800_d_n2;
        locals.var_t4_dn3 = assign27830_e47800_d_n3;
        locals.var_t4_dn4 = assign27830_e47800_d_n4;
        locals.var_t4_dn5 = assign27830_e47800_d_n5;
        locals.var_t4_dn6 = assign27830_e47800_d_n6;
        locals.var_t4_dn7 = assign27830_e47800_d_n7;
        locals.var_t4_dn8 = assign27830_e47800_d_n8;
        locals.var_t4_dn9 = assign27830_e47800_d_n9;
        locals.var_t4_dn10 = assign27830_e47800_d_n10;
        locals.var_t4_dn11 = assign27830_e47800_d_n11;
        locals.var_t4_dn13 = assign27830_e47800_d_n13;
        locals.var_t4_dn14 = assign27830_e47800_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign27840_e47808, assign27840_e47808_d_n0, assign27840_e47808_d_n2, assign27840_e47808_d_n3, assign27840_e47808_d_n4, assign27840_e47808_d_n5, assign27840_e47808_d_n6, assign27840_e47808_d_n7, assign27840_e47808_d_n8, assign27840_e47808_d_n9, assign27840_e47808_d_n10, assign27840_e47808_d_n11, assign27840_e47808_d_n13, assign27840_e47808_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27840_e47804: f64 = (locals.var_t1 * locals.var_t1);
        let assign27840_e47806: f64 = (assign27840_e47804 + 0.0002);
        (assign27840_e47806, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign27840_e47808;
        locals.var_t5_dn0 = assign27840_e47808_d_n0;
        locals.var_t5_dn2 = assign27840_e47808_d_n2;
        locals.var_t5_dn3 = assign27840_e47808_d_n3;
        locals.var_t5_dn4 = assign27840_e47808_d_n4;
        locals.var_t5_dn5 = assign27840_e47808_d_n5;
        locals.var_t5_dn6 = assign27840_e47808_d_n6;
        locals.var_t5_dn7 = assign27840_e47808_d_n7;
        locals.var_t5_dn8 = assign27840_e47808_d_n8;
        locals.var_t5_dn9 = assign27840_e47808_d_n9;
        locals.var_t5_dn10 = assign27840_e47808_d_n10;
        locals.var_t5_dn11 = assign27840_e47808_d_n11;
        locals.var_t5_dn13 = assign27840_e47808_d_n13;
        locals.var_t5_dn14 = assign27840_e47808_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign27870_e47830, assign27870_e47830_d_n0, assign27870_e47830_d_n2, assign27870_e47830_d_n3, assign27870_e47830_d_n4, assign27870_e47830_d_n5, assign27870_e47830_d_n6, assign27870_e47830_d_n7, assign27870_e47830_d_n8, assign27870_e47830_d_n9, assign27870_e47830_d_n10, assign27870_e47830_d_n11, assign27870_e47830_d_n13, assign27870_e47830_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27870_e47828: f64 = (locals.var_vgs_noswap - locals.var_vfbsd_v);
        (assign27870_e47828, (-locals.var_vfbsd_v_dn0), (-locals.var_vfbsd_v_dn2), (-locals.var_vfbsd_v_dn3), (-locals.var_vfbsd_v_dn4), (-locals.var_vfbsd_v_dn5), (locals.var_vgs_noswap_dn6 - locals.var_vfbsd_v_dn6), (-locals.var_vfbsd_v_dn7), (-locals.var_vfbsd_v_dn8), (-locals.var_vfbsd_v_dn9), (-locals.var_vfbsd_v_dn10), (locals.var_vgs_noswap_dn11 - locals.var_vfbsd_v_dn11), (-locals.var_vfbsd_v_dn13), (-locals.var_vfbsd_v_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27870_e47830;
        locals.var_t0_dn0 = assign27870_e47830_d_n0;
        locals.var_t0_dn2 = assign27870_e47830_d_n2;
        locals.var_t0_dn3 = assign27870_e47830_d_n3;
        locals.var_t0_dn4 = assign27870_e47830_d_n4;
        locals.var_t0_dn5 = assign27870_e47830_d_n5;
        locals.var_t0_dn6 = assign27870_e47830_d_n6;
        locals.var_t0_dn7 = assign27870_e47830_d_n7;
        locals.var_t0_dn8 = assign27870_e47830_d_n8;
        locals.var_t0_dn9 = assign27870_e47830_d_n9;
        locals.var_t0_dn10 = assign27870_e47830_d_n10;
        locals.var_t0_dn11 = assign27870_e47830_d_n11;
        locals.var_t0_dn13 = assign27870_e47830_d_n13;
        locals.var_t0_dn14 = assign27870_e47830_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27880_e47839, assign27880_e47839_d_n0, assign27880_e47839_d_n2, assign27880_e47839_d_n3, assign27880_e47839_d_n4, assign27880_e47839_d_n5, assign27880_e47839_d_n6, assign27880_e47839_d_n7, assign27880_e47839_d_n8, assign27880_e47839_d_n9, assign27880_e47839_d_n10, assign27880_e47839_d_n11, assign27880_e47839_d_n13, assign27880_e47839_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27880_e47834: f64 = (locals.var_t0 * locals.var_t0);
        let assign27880_e47836: f64 = (assign27880_e47834 + 0.0001);
        let assign27880_e47837: f64 = (assign27880_e47836).sqrt();
        (assign27880_e47837, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign27880_e47837)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign27880_e47837)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn0, locals.var_vgs_eff_dn2, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11, locals.var_vgs_eff_dn13, locals.var_vgs_eff_dn14,)
    }
};
        locals.var_vgs_eff = assign27880_e47839;
        locals.var_vgs_eff_dn0 = assign27880_e47839_d_n0;
        locals.var_vgs_eff_dn2 = assign27880_e47839_d_n2;
        locals.var_vgs_eff_dn3 = assign27880_e47839_d_n3;
        locals.var_vgs_eff_dn4 = assign27880_e47839_d_n4;
        locals.var_vgs_eff_dn5 = assign27880_e47839_d_n5;
        locals.var_vgs_eff_dn6 = assign27880_e47839_d_n6;
        locals.var_vgs_eff_dn7 = assign27880_e47839_d_n7;
        locals.var_vgs_eff_dn8 = assign27880_e47839_d_n8;
        locals.var_vgs_eff_dn9 = assign27880_e47839_d_n9;
        locals.var_vgs_eff_dn10 = assign27880_e47839_d_n10;
        locals.var_vgs_eff_dn11 = assign27880_e47839_d_n11;
        locals.var_vgs_eff_dn13 = assign27880_e47839_d_n13;
        locals.var_vgs_eff_dn14 = assign27880_e47839_d_n14;
        locals.var_vgs_eff_rv = 0.0;

        let assign27890_e47842: f64 = if p.p82 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign27890_e47842;
        locals.var_guard448_rv = 0.0;

        let (assign27900_e47907, assign27900_e47907_d_n0, assign27900_e47907_d_n2, assign27900_e47907_d_n3, assign27900_e47907_d_n4, assign27900_e47907_d_n5, assign27900_e47907_d_n6, assign27900_e47907_d_n7, assign27900_e47907_d_n8, assign27900_e47907_d_n9, assign27900_e47907_d_n10, assign27900_e47907_d_n11, assign27900_e47907_d_n13, assign27900_e47907_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign27900_e47849: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign27900_e47850: f64 = (locals.var_aigs_t - assign27900_e47849);
        let assign27900_e47852: f64 = (-10000.0);
        let assign27900_e47854: f64 = (assign27900_e47852 * 1e-6);
        let (assign27900_e47905, assign27900_e47905_d_n0, assign27900_e47905_d_n2, assign27900_e47905_d_n3, assign27900_e47905_d_n4, assign27900_e47905_d_n5, assign27900_e47905_d_n6, assign27900_e47905_d_n7, assign27900_e47905_d_n8, assign27900_e47905_d_n9, assign27900_e47905_d_n10, assign27900_e47905_d_n11, assign27900_e47905_d_n13, assign27900_e47905_d_n14,) = {
            if (!(assign27900_e47850 < assign27900_e47854)) {
                let assign27900_e47861: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign27900_e47862: f64 = (locals.var_aigs_t - assign27900_e47861);
                let assign27900_e47866: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign27900_e47867: f64 = (locals.var_aigs_t - assign27900_e47866);
                let assign27900_e47871: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign27900_e47872: f64 = (locals.var_aigs_t - assign27900_e47871);
                let assign27900_e47873: f64 = (assign27900_e47867 * assign27900_e47872);
                let assign27900_e47876: f64 = (4.0 * 1e-6);
                let assign27900_e47878: f64 = (assign27900_e47876 * 1e-6);
                let assign27900_e47879: f64 = (assign27900_e47873 + assign27900_e47878);
                let assign27900_e47880: f64 = (assign27900_e47879).sqrt();
                let assign27900_e47881: f64 = (assign27900_e47862 + assign27900_e47880);
                let assign27900_e47882: f64 = (0.5 * assign27900_e47881);
                (assign27900_e47882, (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn0)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn0)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn0)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn2)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn2)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn2)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)))) / (2.0 * assign27900_e47880)))), (0.5 * ((locals.var_aigs_t_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) + ((((locals.var_aigs_t_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) * assign27900_e47872) + (assign27900_e47867 * (locals.var_aigs_t_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn5)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn5)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn5)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn13)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn13)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn13)))) / (2.0 * assign27900_e47880)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn14)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn14)) * assign27900_e47872) + (assign27900_e47867 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn14)))) / (2.0 * assign27900_e47880)))),)
            } else {
                let assign27900_e47886: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign27900_e47887: f64 = (locals.var_aigs_t - assign27900_e47886);
                let assign27900_e47889: f64 = (-10000.0);
                let assign27900_e47891: f64 = (assign27900_e47889 * 1e-6);
                let (assign27900_e47904, assign27900_e47904_d_n0, assign27900_e47904_d_n2, assign27900_e47904_d_n3, assign27900_e47904_d_n4, assign27900_e47904_d_n5, assign27900_e47904_d_n6, assign27900_e47904_d_n7, assign27900_e47904_d_n8, assign27900_e47904_d_n9, assign27900_e47904_d_n10, assign27900_e47904_d_n11, assign27900_e47904_d_n13, assign27900_e47904_d_n14,) = {
                    if (assign27900_e47887 < assign27900_e47891) {
                        let assign27900_e47894: f64 = (-1e-6);
                        let assign27900_e47896: f64 = (assign27900_e47894 * 1e-6);
                        let assign27900_e47900: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                        let assign27900_e47901: f64 = (locals.var_aigs_t - assign27900_e47900);
                        let assign27900_e47902: f64 = (assign27900_e47896 / assign27900_e47901);
                        (assign27900_e47902, (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn0))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn2))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn3))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (locals.var_aigs_t_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn5))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn6))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn7))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn8))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn9))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn10))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn11))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn13))) / (assign27900_e47901 * assign27900_e47901))), (-((assign27900_e47896 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn14))) / (assign27900_e47901 * assign27900_e47901))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign27900_e47904, assign27900_e47904_d_n0, assign27900_e47904_d_n2, assign27900_e47904_d_n3, assign27900_e47904_d_n4, assign27900_e47904_d_n5, assign27900_e47904_d_n6, assign27900_e47904_d_n7, assign27900_e47904_d_n8, assign27900_e47904_d_n9, assign27900_e47904_d_n10, assign27900_e47904_d_n11, assign27900_e47904_d_n13, assign27900_e47904_d_n14,)
            }
        };
        (assign27900_e47905, assign27900_e47905_d_n0, assign27900_e47905_d_n2, assign27900_e47905_d_n3, assign27900_e47905_d_n4, assign27900_e47905_d_n5, assign27900_e47905_d_n6, assign27900_e47905_d_n7, assign27900_e47905_d_n8, assign27900_e47905_d_n9, assign27900_e47905_d_n10, assign27900_e47905_d_n11, assign27900_e47905_d_n13, assign27900_e47905_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27900_e47907;
        locals.var_t1_dn0 = assign27900_e47907_d_n0;
        locals.var_t1_dn2 = assign27900_e47907_d_n2;
        locals.var_t1_dn3 = assign27900_e47907_d_n3;
        locals.var_t1_dn4 = assign27900_e47907_d_n4;
        locals.var_t1_dn5 = assign27900_e47907_d_n5;
        locals.var_t1_dn6 = assign27900_e47907_d_n6;
        locals.var_t1_dn7 = assign27900_e47907_d_n7;
        locals.var_t1_dn8 = assign27900_e47907_d_n8;
        locals.var_t1_dn9 = assign27900_e47907_d_n9;
        locals.var_t1_dn10 = assign27900_e47907_d_n10;
        locals.var_t1_dn11 = assign27900_e47907_d_n11;
        locals.var_t1_dn13 = assign27900_e47907_d_n13;
        locals.var_t1_dn14 = assign27900_e47907_d_n14;
        locals.var_t1_rv = 0.0;

        let assign27910_e47910: f64 = if locals.var_cigs_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard449 = assign27910_e47910;
        locals.var_guard449_rv = 0.0;

        let (assign27920_e47918,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard449 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigs_i,)
    }
};
        locals.var_cigs_i = assign27920_e47918;
        locals.var_cigs_i_rv = 0.0;

        let (assign27930_e47929, assign27930_e47929_d_n0, assign27930_e47929_d_n2, assign27930_e47929_d_n3, assign27930_e47929_d_n4, assign27930_e47929_d_n5, assign27930_e47929_d_n6, assign27930_e47929_d_n7, assign27930_e47929_d_n8, assign27930_e47929_d_n9, assign27930_e47929_d_n10, assign27930_e47929_d_n11, assign27930_e47929_d_n13, assign27930_e47929_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 == 0.0)) {
        let assign27930_e47926: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign27930_e47927: f64 = (locals.var_aigs_t - assign27930_e47926);
        (assign27930_e47927, (-(locals.var_bigs_i * locals.var_vgs_eff_dn0)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn2)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)), (locals.var_aigs_t_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn5)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn13)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27930_e47929;
        locals.var_t1_dn0 = assign27930_e47929_d_n0;
        locals.var_t1_dn2 = assign27930_e47929_d_n2;
        locals.var_t1_dn3 = assign27930_e47929_d_n3;
        locals.var_t1_dn4 = assign27930_e47929_d_n4;
        locals.var_t1_dn5 = assign27930_e47929_d_n5;
        locals.var_t1_dn6 = assign27930_e47929_d_n6;
        locals.var_t1_dn7 = assign27930_e47929_d_n7;
        locals.var_t1_dn8 = assign27930_e47929_d_n8;
        locals.var_t1_dn9 = assign27930_e47929_d_n9;
        locals.var_t1_dn10 = assign27930_e47929_d_n10;
        locals.var_t1_dn11 = assign27930_e47929_d_n11;
        locals.var_t1_dn13 = assign27930_e47929_d_n13;
        locals.var_t1_dn14 = assign27930_e47929_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign27940_e47937, assign27940_e47937_d_n0, assign27940_e47937_d_n2, assign27940_e47937_d_n3, assign27940_e47937_d_n4, assign27940_e47937_d_n5, assign27940_e47937_d_n6, assign27940_e47937_d_n7, assign27940_e47937_d_n8, assign27940_e47937_d_n9, assign27940_e47937_d_n10, assign27940_e47937_d_n11, assign27940_e47937_d_n13, assign27940_e47937_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27940_e47934: f64 = (locals.var_cigs_i * locals.var_vgs_eff);
        let assign27940_e47935: f64 = (1.0 + assign27940_e47934);
        (assign27940_e47935, (locals.var_cigs_i * locals.var_vgs_eff_dn0), (locals.var_cigs_i * locals.var_vgs_eff_dn2), (locals.var_cigs_i * locals.var_vgs_eff_dn3), (locals.var_cigs_i * locals.var_vgs_eff_dn4), (locals.var_cigs_i * locals.var_vgs_eff_dn5), (locals.var_cigs_i * locals.var_vgs_eff_dn6), (locals.var_cigs_i * locals.var_vgs_eff_dn7), (locals.var_cigs_i * locals.var_vgs_eff_dn8), (locals.var_cigs_i * locals.var_vgs_eff_dn9), (locals.var_cigs_i * locals.var_vgs_eff_dn10), (locals.var_cigs_i * locals.var_vgs_eff_dn11), (locals.var_cigs_i * locals.var_vgs_eff_dn13), (locals.var_cigs_i * locals.var_vgs_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27940_e47937;
        locals.var_t2_dn0 = assign27940_e47937_d_n0;
        locals.var_t2_dn2 = assign27940_e47937_d_n2;
        locals.var_t2_dn3 = assign27940_e47937_d_n3;
        locals.var_t2_dn4 = assign27940_e47937_d_n4;
        locals.var_t2_dn5 = assign27940_e47937_d_n5;
        locals.var_t2_dn6 = assign27940_e47937_d_n6;
        locals.var_t2_dn7 = assign27940_e47937_d_n7;
        locals.var_t2_dn8 = assign27940_e47937_d_n8;
        locals.var_t2_dn9 = assign27940_e47937_d_n9;
        locals.var_t2_dn10 = assign27940_e47937_d_n10;
        locals.var_t2_dn11 = assign27940_e47937_d_n11;
        locals.var_t2_dn13 = assign27940_e47937_d_n13;
        locals.var_t2_dn14 = assign27940_e47937_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27950_e47950, assign27950_e47950_d_n0, assign27950_e47950_d_n2, assign27950_e47950_d_n3, assign27950_e47950_d_n4, assign27950_e47950_d_n5, assign27950_e47950_d_n6, assign27950_e47950_d_n7, assign27950_e47950_d_n8, assign27950_e47950_d_n9, assign27950_e47950_d_n10, assign27950_e47950_d_n11, assign27950_e47950_d_n13, assign27950_e47950_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27950_e47940: f64 = (-locals.var_bechvb);
        let assign27950_e47942: f64 = (assign27950_e47940 * p.p1109);
        let assign27950_e47944: f64 = (assign27950_e47942 * locals.var_poxedge_i);
        let assign27950_e47946: f64 = (assign27950_e47944 * locals.var_t1);
        let assign27950_e47948: f64 = (assign27950_e47946 * locals.var_t2);
        (assign27950_e47948, (((assign27950_e47944 * locals.var_t1_dn0) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn0)), (((assign27950_e47944 * locals.var_t1_dn2) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn2)), (((assign27950_e47944 * locals.var_t1_dn3) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn3)), (((assign27950_e47944 * locals.var_t1_dn4) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn4)), (((assign27950_e47944 * locals.var_t1_dn5) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn5)), (((assign27950_e47944 * locals.var_t1_dn6) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn6)), (((assign27950_e47944 * locals.var_t1_dn7) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn7)), (((assign27950_e47944 * locals.var_t1_dn8) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn8)), (((assign27950_e47944 * locals.var_t1_dn9) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn9)), (((assign27950_e47944 * locals.var_t1_dn10) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn10)), (((assign27950_e47944 * locals.var_t1_dn11) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn11)), (((assign27950_e47944 * locals.var_t1_dn13) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn13)), (((assign27950_e47944 * locals.var_t1_dn14) * locals.var_t2) + (assign27950_e47946 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27950_e47950;
        locals.var_t3_dn0 = assign27950_e47950_d_n0;
        locals.var_t3_dn2 = assign27950_e47950_d_n2;
        locals.var_t3_dn3 = assign27950_e47950_d_n3;
        locals.var_t3_dn4 = assign27950_e47950_d_n4;
        locals.var_t3_dn5 = assign27950_e47950_d_n5;
        locals.var_t3_dn6 = assign27950_e47950_d_n6;
        locals.var_t3_dn7 = assign27950_e47950_d_n7;
        locals.var_t3_dn8 = assign27950_e47950_d_n8;
        locals.var_t3_dn9 = assign27950_e47950_d_n9;
        locals.var_t3_dn10 = assign27950_e47950_d_n10;
        locals.var_t3_dn11 = assign27950_e47950_d_n11;
        locals.var_t3_dn13 = assign27950_e47950_d_n13;
        locals.var_t3_dn14 = assign27950_e47950_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27960_e47955, assign27960_e47955_d_n0, assign27960_e47955_d_n2, assign27960_e47955_d_n3, assign27960_e47955_d_n4, assign27960_e47955_d_n5, assign27960_e47955_d_n6, assign27960_e47955_d_n7, assign27960_e47955_d_n8, assign27960_e47955_d_n9, assign27960_e47955_d_n10, assign27960_e47955_d_n11, assign27960_e47955_d_n13, assign27960_e47955_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign27960_e47953: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27960_e47953, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign27960_e47955;
        locals.var_t4_dn0 = assign27960_e47955_d_n0;
        locals.var_t4_dn2 = assign27960_e47955_d_n2;
        locals.var_t4_dn3 = assign27960_e47955_d_n3;
        locals.var_t4_dn4 = assign27960_e47955_d_n4;
        locals.var_t4_dn5 = assign27960_e47955_d_n5;
        locals.var_t4_dn6 = assign27960_e47955_d_n6;
        locals.var_t4_dn7 = assign27960_e47955_d_n7;
        locals.var_t4_dn8 = assign27960_e47955_d_n8;
        locals.var_t4_dn9 = assign27960_e47955_d_n9;
        locals.var_t4_dn10 = assign27960_e47955_d_n10;
        locals.var_t4_dn11 = assign27960_e47955_d_n11;
        locals.var_t4_dn13 = assign27960_e47955_d_n13;
        locals.var_t4_dn14 = assign27960_e47955_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign28000_e47993, assign28000_e47993_d_n0, assign28000_e47993_d_n2, assign28000_e47993_d_n3, assign28000_e47993_d_n4, assign28000_e47993_d_n5, assign28000_e47993_d_n6, assign28000_e47993_d_n7, assign28000_e47993_d_n8, assign28000_e47993_d_n9, assign28000_e47993_d_n10, assign28000_e47993_d_n11, assign28000_e47993_d_n13, assign28000_e47993_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign28000_e47991: f64 = (locals.var_vgd_noswap - locals.var_vfbsd_v);
        (assign28000_e47991, (-locals.var_vfbsd_v_dn0), (-locals.var_vfbsd_v_dn2), (-locals.var_vfbsd_v_dn3), (-locals.var_vfbsd_v_dn4), (locals.var_vgd_noswap_dn5 - locals.var_vfbsd_v_dn5), (-locals.var_vfbsd_v_dn6), (-locals.var_vfbsd_v_dn7), (-locals.var_vfbsd_v_dn8), (-locals.var_vfbsd_v_dn9), (-locals.var_vfbsd_v_dn10), (locals.var_vgd_noswap_dn11 - locals.var_vfbsd_v_dn11), (-locals.var_vfbsd_v_dn13), (-locals.var_vfbsd_v_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28000_e47993;
        locals.var_t0_dn0 = assign28000_e47993_d_n0;
        locals.var_t0_dn2 = assign28000_e47993_d_n2;
        locals.var_t0_dn3 = assign28000_e47993_d_n3;
        locals.var_t0_dn4 = assign28000_e47993_d_n4;
        locals.var_t0_dn5 = assign28000_e47993_d_n5;
        locals.var_t0_dn6 = assign28000_e47993_d_n6;
        locals.var_t0_dn7 = assign28000_e47993_d_n7;
        locals.var_t0_dn8 = assign28000_e47993_d_n8;
        locals.var_t0_dn9 = assign28000_e47993_d_n9;
        locals.var_t0_dn10 = assign28000_e47993_d_n10;
        locals.var_t0_dn11 = assign28000_e47993_d_n11;
        locals.var_t0_dn13 = assign28000_e47993_d_n13;
        locals.var_t0_dn14 = assign28000_e47993_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign28010_e48002, assign28010_e48002_d_n0, assign28010_e48002_d_n2, assign28010_e48002_d_n3, assign28010_e48002_d_n4, assign28010_e48002_d_n5, assign28010_e48002_d_n6, assign28010_e48002_d_n7, assign28010_e48002_d_n8, assign28010_e48002_d_n9, assign28010_e48002_d_n10, assign28010_e48002_d_n11, assign28010_e48002_d_n13, assign28010_e48002_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign28010_e47997: f64 = (locals.var_t0 * locals.var_t0);
        let assign28010_e47999: f64 = (assign28010_e47997 + 0.0001);
        let assign28010_e48000: f64 = (assign28010_e47999).sqrt();
        (assign28010_e48000, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign28010_e48000)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign28010_e48000)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn0, locals.var_vgd_eff_dn2, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11, locals.var_vgd_eff_dn13, locals.var_vgd_eff_dn14,)
    }
};
        locals.var_vgd_eff = assign28010_e48002;
        locals.var_vgd_eff_dn0 = assign28010_e48002_d_n0;
        locals.var_vgd_eff_dn2 = assign28010_e48002_d_n2;
        locals.var_vgd_eff_dn3 = assign28010_e48002_d_n3;
        locals.var_vgd_eff_dn4 = assign28010_e48002_d_n4;
        locals.var_vgd_eff_dn5 = assign28010_e48002_d_n5;
        locals.var_vgd_eff_dn6 = assign28010_e48002_d_n6;
        locals.var_vgd_eff_dn7 = assign28010_e48002_d_n7;
        locals.var_vgd_eff_dn8 = assign28010_e48002_d_n8;
        locals.var_vgd_eff_dn9 = assign28010_e48002_d_n9;
        locals.var_vgd_eff_dn10 = assign28010_e48002_d_n10;
        locals.var_vgd_eff_dn11 = assign28010_e48002_d_n11;
        locals.var_vgd_eff_dn13 = assign28010_e48002_d_n13;
        locals.var_vgd_eff_dn14 = assign28010_e48002_d_n14;
        locals.var_vgd_eff_rv = 0.0;

        let assign28020_e48005: f64 = if p.p82 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard451 = assign28020_e48005;
        locals.var_guard451_rv = 0.0;

        let (assign28030_e48070, assign28030_e48070_d_n0, assign28030_e48070_d_n2, assign28030_e48070_d_n3, assign28030_e48070_d_n4, assign28030_e48070_d_n5, assign28030_e48070_d_n6, assign28030_e48070_d_n7, assign28030_e48070_d_n8, assign28030_e48070_d_n9, assign28030_e48070_d_n10, assign28030_e48070_d_n11, assign28030_e48070_d_n13, assign28030_e48070_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard451 != 0.0)) {
        let assign28030_e48012: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign28030_e48013: f64 = (locals.var_aigd_t - assign28030_e48012);
        let assign28030_e48015: f64 = (-10000.0);
        let assign28030_e48017: f64 = (assign28030_e48015 * 1e-6);
        let (assign28030_e48068, assign28030_e48068_d_n0, assign28030_e48068_d_n2, assign28030_e48068_d_n3, assign28030_e48068_d_n4, assign28030_e48068_d_n5, assign28030_e48068_d_n6, assign28030_e48068_d_n7, assign28030_e48068_d_n8, assign28030_e48068_d_n9, assign28030_e48068_d_n10, assign28030_e48068_d_n11, assign28030_e48068_d_n13, assign28030_e48068_d_n14,) = {
            if (!(assign28030_e48013 < assign28030_e48017)) {
                let assign28030_e48024: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign28030_e48025: f64 = (locals.var_aigd_t - assign28030_e48024);
                let assign28030_e48029: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign28030_e48030: f64 = (locals.var_aigd_t - assign28030_e48029);
                let assign28030_e48034: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign28030_e48035: f64 = (locals.var_aigd_t - assign28030_e48034);
                let assign28030_e48036: f64 = (assign28030_e48030 * assign28030_e48035);
                let assign28030_e48039: f64 = (4.0 * 1e-6);
                let assign28030_e48041: f64 = (assign28030_e48039 * 1e-6);
                let assign28030_e48042: f64 = (assign28030_e48036 + assign28030_e48041);
                let assign28030_e48043: f64 = (assign28030_e48042).sqrt();
                let assign28030_e48044: f64 = (assign28030_e48025 + assign28030_e48043);
                let assign28030_e48045: f64 = (0.5 * assign28030_e48044);
                (assign28030_e48045, (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn0)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn0)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn0)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn2)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn2)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn2)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)))) / (2.0 * assign28030_e48043)))), (0.5 * ((locals.var_aigd_t_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) + ((((locals.var_aigd_t_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) * assign28030_e48035) + (assign28030_e48030 * (locals.var_aigd_t_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn5)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn5)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn5)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn13)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn13)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn13)))) / (2.0 * assign28030_e48043)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn14)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn14)) * assign28030_e48035) + (assign28030_e48030 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn14)))) / (2.0 * assign28030_e48043)))),)
            } else {
                let assign28030_e48049: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign28030_e48050: f64 = (locals.var_aigd_t - assign28030_e48049);
                let assign28030_e48052: f64 = (-10000.0);
                let assign28030_e48054: f64 = (assign28030_e48052 * 1e-6);
                let (assign28030_e48067, assign28030_e48067_d_n0, assign28030_e48067_d_n2, assign28030_e48067_d_n3, assign28030_e48067_d_n4, assign28030_e48067_d_n5, assign28030_e48067_d_n6, assign28030_e48067_d_n7, assign28030_e48067_d_n8, assign28030_e48067_d_n9, assign28030_e48067_d_n10, assign28030_e48067_d_n11, assign28030_e48067_d_n13, assign28030_e48067_d_n14,) = {
                    if (assign28030_e48050 < assign28030_e48054) {
                        let assign28030_e48057: f64 = (-1e-6);
                        let assign28030_e48059: f64 = (assign28030_e48057 * 1e-6);
                        let assign28030_e48063: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                        let assign28030_e48064: f64 = (locals.var_aigd_t - assign28030_e48063);
                        let assign28030_e48065: f64 = (assign28030_e48059 / assign28030_e48064);
                        (assign28030_e48065, (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn0))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn2))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn3))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (locals.var_aigd_t_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn5))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn6))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn7))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn8))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn9))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn10))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn11))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn13))) / (assign28030_e48064 * assign28030_e48064))), (-((assign28030_e48059 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn14))) / (assign28030_e48064 * assign28030_e48064))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28030_e48067, assign28030_e48067_d_n0, assign28030_e48067_d_n2, assign28030_e48067_d_n3, assign28030_e48067_d_n4, assign28030_e48067_d_n5, assign28030_e48067_d_n6, assign28030_e48067_d_n7, assign28030_e48067_d_n8, assign28030_e48067_d_n9, assign28030_e48067_d_n10, assign28030_e48067_d_n11, assign28030_e48067_d_n13, assign28030_e48067_d_n14,)
            }
        };
        (assign28030_e48068, assign28030_e48068_d_n0, assign28030_e48068_d_n2, assign28030_e48068_d_n3, assign28030_e48068_d_n4, assign28030_e48068_d_n5, assign28030_e48068_d_n6, assign28030_e48068_d_n7, assign28030_e48068_d_n8, assign28030_e48068_d_n9, assign28030_e48068_d_n10, assign28030_e48068_d_n11, assign28030_e48068_d_n13, assign28030_e48068_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28030_e48070;
        locals.var_t1_dn0 = assign28030_e48070_d_n0;
        locals.var_t1_dn2 = assign28030_e48070_d_n2;
        locals.var_t1_dn3 = assign28030_e48070_d_n3;
        locals.var_t1_dn4 = assign28030_e48070_d_n4;
        locals.var_t1_dn5 = assign28030_e48070_d_n5;
        locals.var_t1_dn6 = assign28030_e48070_d_n6;
        locals.var_t1_dn7 = assign28030_e48070_d_n7;
        locals.var_t1_dn8 = assign28030_e48070_d_n8;
        locals.var_t1_dn9 = assign28030_e48070_d_n9;
        locals.var_t1_dn10 = assign28030_e48070_d_n10;
        locals.var_t1_dn11 = assign28030_e48070_d_n11;
        locals.var_t1_dn13 = assign28030_e48070_d_n13;
        locals.var_t1_dn14 = assign28030_e48070_d_n14;
        locals.var_t1_rv = 0.0;

        let assign28040_e48073: f64 = if locals.var_cigd_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign28040_e48073;
        locals.var_guard452_rv = 0.0;

        let (assign28050_e48081,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard451 != 0.0)) && (locals.var_guard452 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigd_i,)
    }
};
        locals.var_cigd_i = assign28050_e48081;
        locals.var_cigd_i_rv = 0.0;

        let (assign28060_e48092, assign28060_e48092_d_n0, assign28060_e48092_d_n2, assign28060_e48092_d_n3, assign28060_e48092_d_n4, assign28060_e48092_d_n5, assign28060_e48092_d_n6, assign28060_e48092_d_n7, assign28060_e48092_d_n8, assign28060_e48092_d_n9, assign28060_e48092_d_n10, assign28060_e48092_d_n11, assign28060_e48092_d_n13, assign28060_e48092_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard451 == 0.0)) {
        let assign28060_e48089: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign28060_e48090: f64 = (locals.var_aigd_t - assign28060_e48089);
        (assign28060_e48090, (-(locals.var_bigd_i * locals.var_vgd_eff_dn0)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn2)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)), (locals.var_aigd_t_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn5)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn13)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28060_e48092;
        locals.var_t1_dn0 = assign28060_e48092_d_n0;
        locals.var_t1_dn2 = assign28060_e48092_d_n2;
        locals.var_t1_dn3 = assign28060_e48092_d_n3;
        locals.var_t1_dn4 = assign28060_e48092_d_n4;
        locals.var_t1_dn5 = assign28060_e48092_d_n5;
        locals.var_t1_dn6 = assign28060_e48092_d_n6;
        locals.var_t1_dn7 = assign28060_e48092_d_n7;
        locals.var_t1_dn8 = assign28060_e48092_d_n8;
        locals.var_t1_dn9 = assign28060_e48092_d_n9;
        locals.var_t1_dn10 = assign28060_e48092_d_n10;
        locals.var_t1_dn11 = assign28060_e48092_d_n11;
        locals.var_t1_dn13 = assign28060_e48092_d_n13;
        locals.var_t1_dn14 = assign28060_e48092_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28070_e48100, assign28070_e48100_d_n0, assign28070_e48100_d_n2, assign28070_e48100_d_n3, assign28070_e48100_d_n4, assign28070_e48100_d_n5, assign28070_e48100_d_n6, assign28070_e48100_d_n7, assign28070_e48100_d_n8, assign28070_e48100_d_n9, assign28070_e48100_d_n10, assign28070_e48100_d_n11, assign28070_e48100_d_n13, assign28070_e48100_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign28070_e48097: f64 = (locals.var_cigd_i * locals.var_vgd_eff);
        let assign28070_e48098: f64 = (1.0 + assign28070_e48097);
        (assign28070_e48098, (locals.var_cigd_i * locals.var_vgd_eff_dn0), (locals.var_cigd_i * locals.var_vgd_eff_dn2), (locals.var_cigd_i * locals.var_vgd_eff_dn3), (locals.var_cigd_i * locals.var_vgd_eff_dn4), (locals.var_cigd_i * locals.var_vgd_eff_dn5), (locals.var_cigd_i * locals.var_vgd_eff_dn6), (locals.var_cigd_i * locals.var_vgd_eff_dn7), (locals.var_cigd_i * locals.var_vgd_eff_dn8), (locals.var_cigd_i * locals.var_vgd_eff_dn9), (locals.var_cigd_i * locals.var_vgd_eff_dn10), (locals.var_cigd_i * locals.var_vgd_eff_dn11), (locals.var_cigd_i * locals.var_vgd_eff_dn13), (locals.var_cigd_i * locals.var_vgd_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28070_e48100;
        locals.var_t2_dn0 = assign28070_e48100_d_n0;
        locals.var_t2_dn2 = assign28070_e48100_d_n2;
        locals.var_t2_dn3 = assign28070_e48100_d_n3;
        locals.var_t2_dn4 = assign28070_e48100_d_n4;
        locals.var_t2_dn5 = assign28070_e48100_d_n5;
        locals.var_t2_dn6 = assign28070_e48100_d_n6;
        locals.var_t2_dn7 = assign28070_e48100_d_n7;
        locals.var_t2_dn8 = assign28070_e48100_d_n8;
        locals.var_t2_dn9 = assign28070_e48100_d_n9;
        locals.var_t2_dn10 = assign28070_e48100_d_n10;
        locals.var_t2_dn11 = assign28070_e48100_d_n11;
        locals.var_t2_dn13 = assign28070_e48100_d_n13;
        locals.var_t2_dn14 = assign28070_e48100_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28080_e48113, assign28080_e48113_d_n0, assign28080_e48113_d_n2, assign28080_e48113_d_n3, assign28080_e48113_d_n4, assign28080_e48113_d_n5, assign28080_e48113_d_n6, assign28080_e48113_d_n7, assign28080_e48113_d_n8, assign28080_e48113_d_n9, assign28080_e48113_d_n10, assign28080_e48113_d_n11, assign28080_e48113_d_n13, assign28080_e48113_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign28080_e48103: f64 = (-locals.var_bechvb);
        let assign28080_e48105: f64 = (assign28080_e48103 * p.p1109);
        let assign28080_e48107: f64 = (assign28080_e48105 * locals.var_poxedge_i);
        let assign28080_e48109: f64 = (assign28080_e48107 * locals.var_t1);
        let assign28080_e48111: f64 = (assign28080_e48109 * locals.var_t2);
        (assign28080_e48111, (((assign28080_e48107 * locals.var_t1_dn0) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn0)), (((assign28080_e48107 * locals.var_t1_dn2) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn2)), (((assign28080_e48107 * locals.var_t1_dn3) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn3)), (((assign28080_e48107 * locals.var_t1_dn4) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn4)), (((assign28080_e48107 * locals.var_t1_dn5) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn5)), (((assign28080_e48107 * locals.var_t1_dn6) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn6)), (((assign28080_e48107 * locals.var_t1_dn7) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn7)), (((assign28080_e48107 * locals.var_t1_dn8) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn8)), (((assign28080_e48107 * locals.var_t1_dn9) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn9)), (((assign28080_e48107 * locals.var_t1_dn10) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn10)), (((assign28080_e48107 * locals.var_t1_dn11) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn11)), (((assign28080_e48107 * locals.var_t1_dn13) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn13)), (((assign28080_e48107 * locals.var_t1_dn14) * locals.var_t2) + (assign28080_e48109 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign28080_e48113;
        locals.var_t3_dn0 = assign28080_e48113_d_n0;
        locals.var_t3_dn2 = assign28080_e48113_d_n2;
        locals.var_t3_dn3 = assign28080_e48113_d_n3;
        locals.var_t3_dn4 = assign28080_e48113_d_n4;
        locals.var_t3_dn5 = assign28080_e48113_d_n5;
        locals.var_t3_dn6 = assign28080_e48113_d_n6;
        locals.var_t3_dn7 = assign28080_e48113_d_n7;
        locals.var_t3_dn8 = assign28080_e48113_d_n8;
        locals.var_t3_dn9 = assign28080_e48113_d_n9;
        locals.var_t3_dn10 = assign28080_e48113_d_n10;
        locals.var_t3_dn11 = assign28080_e48113_d_n11;
        locals.var_t3_dn13 = assign28080_e48113_d_n13;
        locals.var_t3_dn14 = assign28080_e48113_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign28090_e48118, assign28090_e48118_d_n0, assign28090_e48118_d_n2, assign28090_e48118_d_n3, assign28090_e48118_d_n4, assign28090_e48118_d_n5, assign28090_e48118_d_n6, assign28090_e48118_d_n7, assign28090_e48118_d_n8, assign28090_e48118_d_n9, assign28090_e48118_d_n10, assign28090_e48118_d_n11, assign28090_e48118_d_n13, assign28090_e48118_d_n14,) = {
    if (locals.var_guard447 != 0.0) {
        let assign28090_e48116: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign28090_e48116, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28090_e48118;
        locals.var_t4_dn0 = assign28090_e48118_d_n0;
        locals.var_t4_dn2 = assign28090_e48118_d_n2;
        locals.var_t4_dn3 = assign28090_e48118_d_n3;
        locals.var_t4_dn4 = assign28090_e48118_d_n4;
        locals.var_t4_dn5 = assign28090_e48118_d_n5;
        locals.var_t4_dn6 = assign28090_e48118_d_n6;
        locals.var_t4_dn7 = assign28090_e48118_d_n7;
        locals.var_t4_dn8 = assign28090_e48118_d_n8;
        locals.var_t4_dn9 = assign28090_e48118_d_n9;
        locals.var_t4_dn10 = assign28090_e48118_d_n10;
        locals.var_t4_dn11 = assign28090_e48118_d_n11;
        locals.var_t4_dn13 = assign28090_e48118_d_n13;
        locals.var_t4_dn14 = assign28090_e48118_d_n14;
        locals.var_t4_rv = 0.0;

        let assign28130_e48153: f64 = if p.p70 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign28130_e48153;
        locals.var_guard454_rv = 0.0;

        let (assign28140_e48159, assign28140_e48159_d_n0, assign28140_e48159_d_n2, assign28140_e48159_d_n3, assign28140_e48159_d_n4, assign28140_e48159_d_n5, assign28140_e48159_d_n6, assign28140_e48159_d_n7, assign28140_e48159_d_n8, assign28140_e48159_d_n9, assign28140_e48159_d_n10, assign28140_e48159_d_n11, assign28140_e48159_d_n13, assign28140_e48159_d_n14,) = {
    if (locals.var_guard454 != 0.0) {
        let assign28140_e48157: f64 = (locals.var_epsratio * p.p89);
        (assign28140_e48157, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28140_e48159;
        locals.var_t0_dn0 = assign28140_e48159_d_n0;
        locals.var_t0_dn2 = assign28140_e48159_d_n2;
        locals.var_t0_dn3 = assign28140_e48159_d_n3;
        locals.var_t0_dn4 = assign28140_e48159_d_n4;
        locals.var_t0_dn5 = assign28140_e48159_d_n5;
        locals.var_t0_dn6 = assign28140_e48159_d_n6;
        locals.var_t0_dn7 = assign28140_e48159_d_n7;
        locals.var_t0_dn8 = assign28140_e48159_d_n8;
        locals.var_t0_dn9 = assign28140_e48159_d_n9;
        locals.var_t0_dn10 = assign28140_e48159_d_n10;
        locals.var_t0_dn11 = assign28140_e48159_d_n11;
        locals.var_t0_dn13 = assign28140_e48159_d_n13;
        locals.var_t0_dn14 = assign28140_e48159_d_n14;
        locals.var_t0_rv = 0.0;

        let assign28150_e48166: f64 = if ((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard455 = assign28150_e48166;
        locals.var_guard455_rv = 0.0;

        let (assign28160_e48172, assign28160_e48172_d_n0, assign28160_e48172_d_n2, assign28160_e48172_d_n3, assign28160_e48172_d_n4, assign28160_e48172_d_n5, assign28160_e48172_d_n6, assign28160_e48172_d_n7, assign28160_e48172_d_n8, assign28160_e48172_d_n9, assign28160_e48172_d_n10, assign28160_e48172_d_n11, assign28160_e48172_d_n13, assign28160_e48172_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard455 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28160_e48172;
        locals.var_t6_dn0 = assign28160_e48172_d_n0;
        locals.var_t6_dn2 = assign28160_e48172_d_n2;
        locals.var_t6_dn3 = assign28160_e48172_d_n3;
        locals.var_t6_dn4 = assign28160_e48172_d_n4;
        locals.var_t6_dn5 = assign28160_e48172_d_n5;
        locals.var_t6_dn6 = assign28160_e48172_d_n6;
        locals.var_t6_dn7 = assign28160_e48172_d_n7;
        locals.var_t6_dn8 = assign28160_e48172_d_n8;
        locals.var_t6_dn9 = assign28160_e48172_d_n9;
        locals.var_t6_dn10 = assign28160_e48172_d_n10;
        locals.var_t6_dn11 = assign28160_e48172_d_n11;
        locals.var_t6_dn13 = assign28160_e48172_d_n13;
        locals.var_t6_dn14 = assign28160_e48172_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign28170_e48186, assign28170_e48186_d_n0, assign28170_e48186_d_n2, assign28170_e48186_d_n3, assign28170_e48186_d_n4, assign28170_e48186_d_n5, assign28170_e48186_d_n6, assign28170_e48186_d_n7, assign28170_e48186_d_n8, assign28170_e48186_d_n9, assign28170_e48186_d_n10, assign28170_e48186_d_n11, assign28170_e48186_d_n13, assign28170_e48186_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) {
        let assign28170_e48178: f64 = (-locals.var_vgd_noswap);
        let assign28170_e48180: f64 = (assign28170_e48178 - locals.var_egidl_i);
        let assign28170_e48182: f64 = (assign28170_e48180 + locals.var_vfbsd_v);
        let assign28170_e48184: f64 = (assign28170_e48182 / locals.var_t0);
        (assign28170_e48184, (((locals.var_vfbsd_v_dn0 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn2 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn3 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn4 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgd_noswap_dn5) + locals.var_vfbsd_v_dn5) * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn6 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn7 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn8 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn9 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn10 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgd_noswap_dn11) + locals.var_vfbsd_v_dn11) * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn13 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn14 * locals.var_t0) - (assign28170_e48182 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28170_e48186;
        locals.var_t1_dn0 = assign28170_e48186_d_n0;
        locals.var_t1_dn2 = assign28170_e48186_d_n2;
        locals.var_t1_dn3 = assign28170_e48186_d_n3;
        locals.var_t1_dn4 = assign28170_e48186_d_n4;
        locals.var_t1_dn5 = assign28170_e48186_d_n5;
        locals.var_t1_dn6 = assign28170_e48186_d_n6;
        locals.var_t1_dn7 = assign28170_e48186_d_n7;
        locals.var_t1_dn8 = assign28170_e48186_d_n8;
        locals.var_t1_dn9 = assign28170_e48186_d_n9;
        locals.var_t1_dn10 = assign28170_e48186_d_n10;
        locals.var_t1_dn11 = assign28170_e48186_d_n11;
        locals.var_t1_dn13 = assign28170_e48186_d_n13;
        locals.var_t1_dn14 = assign28170_e48186_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28180_e48228, assign28180_e48228_d_n0, assign28180_e48228_d_n2, assign28180_e48228_d_n3, assign28180_e48228_d_n4, assign28180_e48228_d_n5, assign28180_e48228_d_n6, assign28180_e48228_d_n7, assign28180_e48228_d_n8, assign28180_e48228_d_n9, assign28180_e48228_d_n10, assign28180_e48228_d_n11, assign28180_e48228_d_n13, assign28180_e48228_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) {
        let assign28180_e48193: f64 = (-10000.0);
        let assign28180_e48195: f64 = (assign28180_e48193 * 0.01);
        let (assign28180_e48226, assign28180_e48226_d_n0, assign28180_e48226_d_n2, assign28180_e48226_d_n3, assign28180_e48226_d_n4, assign28180_e48226_d_n5, assign28180_e48226_d_n6, assign28180_e48226_d_n7, assign28180_e48226_d_n8, assign28180_e48226_d_n9, assign28180_e48226_d_n10, assign28180_e48226_d_n11, assign28180_e48226_d_n13, assign28180_e48226_d_n14,) = {
            if (!(locals.var_t1 < assign28180_e48195)) {
                let assign28180_e48202: f64 = (locals.var_t1 * locals.var_t1);
                let assign28180_e48205: f64 = (4.0 * 0.01);
                let assign28180_e48207: f64 = (assign28180_e48205 * 0.01);
                let assign28180_e48208: f64 = (assign28180_e48202 + assign28180_e48207);
                let assign28180_e48209: f64 = (assign28180_e48208).sqrt();
                let assign28180_e48210: f64 = (locals.var_t1 + assign28180_e48209);
                let assign28180_e48211: f64 = (0.5 * assign28180_e48210);
                (assign28180_e48211, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign28180_e48209)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign28180_e48209)))),)
            } else {
                let assign28180_e48214: f64 = (-10000.0);
                let assign28180_e48216: f64 = (assign28180_e48214 * 0.01);
                let (assign28180_e48225, assign28180_e48225_d_n0, assign28180_e48225_d_n2, assign28180_e48225_d_n3, assign28180_e48225_d_n4, assign28180_e48225_d_n5, assign28180_e48225_d_n6, assign28180_e48225_d_n7, assign28180_e48225_d_n8, assign28180_e48225_d_n9, assign28180_e48225_d_n10, assign28180_e48225_d_n11, assign28180_e48225_d_n13, assign28180_e48225_d_n14,) = {
                    if (locals.var_t1 < assign28180_e48216) {
                        let assign28180_e48219: f64 = (-0.01);
                        let assign28180_e48221: f64 = (assign28180_e48219 * 0.01);
                        let assign28180_e48223: f64 = (assign28180_e48221 / locals.var_t1);
                        (assign28180_e48223, (-((assign28180_e48221 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign28180_e48221 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28180_e48225, assign28180_e48225_d_n0, assign28180_e48225_d_n2, assign28180_e48225_d_n3, assign28180_e48225_d_n4, assign28180_e48225_d_n5, assign28180_e48225_d_n6, assign28180_e48225_d_n7, assign28180_e48225_d_n8, assign28180_e48225_d_n9, assign28180_e48225_d_n10, assign28180_e48225_d_n11, assign28180_e48225_d_n13, assign28180_e48225_d_n14,)
            }
        };
        (assign28180_e48226, assign28180_e48226_d_n0, assign28180_e48226_d_n2, assign28180_e48226_d_n3, assign28180_e48226_d_n4, assign28180_e48226_d_n5, assign28180_e48226_d_n6, assign28180_e48226_d_n7, assign28180_e48226_d_n8, assign28180_e48226_d_n9, assign28180_e48226_d_n10, assign28180_e48226_d_n11, assign28180_e48226_d_n13, assign28180_e48226_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28180_e48228;
        locals.var_t1_dn0 = assign28180_e48228_d_n0;
        locals.var_t1_dn2 = assign28180_e48228_d_n2;
        locals.var_t1_dn3 = assign28180_e48228_d_n3;
        locals.var_t1_dn4 = assign28180_e48228_d_n4;
        locals.var_t1_dn5 = assign28180_e48228_d_n5;
        locals.var_t1_dn6 = assign28180_e48228_d_n6;
        locals.var_t1_dn7 = assign28180_e48228_d_n7;
        locals.var_t1_dn8 = assign28180_e48228_d_n8;
        locals.var_t1_dn9 = assign28180_e48228_d_n9;
        locals.var_t1_dn10 = assign28180_e48228_d_n10;
        locals.var_t1_dn11 = assign28180_e48228_d_n11;
        locals.var_t1_dn13 = assign28180_e48228_d_n13;
        locals.var_t1_dn14 = assign28180_e48228_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28190_e48239, assign28190_e48239_d_n0, assign28190_e48239_d_n2, assign28190_e48239_d_n3, assign28190_e48239_d_n4, assign28190_e48239_d_n5, assign28190_e48239_d_n6, assign28190_e48239_d_n7, assign28190_e48239_d_n8, assign28190_e48239_d_n9, assign28190_e48239_d_n10, assign28190_e48239_d_n11, assign28190_e48239_d_n13, assign28190_e48239_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) {
        let assign28190_e48236: f64 = (locals.var_t1 + 0.001);
        let assign28190_e48237: f64 = (locals.var_bgidl_t / assign28190_e48236);
        (assign28190_e48237, (-((locals.var_bgidl_t * locals.var_t1_dn0) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn2) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign28190_e48236 * assign28190_e48236))), (((locals.var_bgidl_t_dn4 * assign28190_e48236) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign28190_e48236 * assign28190_e48236)), (-((locals.var_bgidl_t * locals.var_t1_dn5) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn13) / (assign28190_e48236 * assign28190_e48236))), (-((locals.var_bgidl_t * locals.var_t1_dn14) / (assign28190_e48236 * assign28190_e48236))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28190_e48239;
        locals.var_t2_dn0 = assign28190_e48239_d_n0;
        locals.var_t2_dn2 = assign28190_e48239_d_n2;
        locals.var_t2_dn3 = assign28190_e48239_d_n3;
        locals.var_t2_dn4 = assign28190_e48239_d_n4;
        locals.var_t2_dn5 = assign28190_e48239_d_n5;
        locals.var_t2_dn6 = assign28190_e48239_d_n6;
        locals.var_t2_dn7 = assign28190_e48239_d_n7;
        locals.var_t2_dn8 = assign28190_e48239_d_n8;
        locals.var_t2_dn9 = assign28190_e48239_d_n9;
        locals.var_t2_dn10 = assign28190_e48239_d_n10;
        locals.var_t2_dn11 = assign28190_e48239_d_n11;
        locals.var_t2_dn13 = assign28190_e48239_d_n13;
        locals.var_t2_dn14 = assign28190_e48239_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28200_e48248, assign28200_e48248_d_n0, assign28200_e48248_d_n2, assign28200_e48248_d_n3, assign28200_e48248_d_n4, assign28200_e48248_d_n5, assign28200_e48248_d_n6, assign28200_e48248_d_n7, assign28200_e48248_d_n8, assign28200_e48248_d_n9, assign28200_e48248_d_n10, assign28200_e48248_d_n11, assign28200_e48248_d_n13, assign28200_e48248_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) {
        let assign28200_e48246: f64 = (locals.var_t1).powf(locals.var_pgidl_i);
        (assign28200_e48246, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn0)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn2)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn3)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn4)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn5)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn6)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn7)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn8)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn9)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn10)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn11)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn13)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidl_i) as f64).is_finite() && ((locals.var_pgidl_i) as f64).fract() == 0.0 { if locals.var_pgidl_i == 0.0 { 0.0 } else { (locals.var_pgidl_i * ((locals.var_t1).powf(locals.var_pgidl_i - 1.0) * locals.var_t1_dn14)) } } else { (assign28200_e48246 * (locals.var_pgidl_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign28200_e48248;
        locals.var_t3_dn0 = assign28200_e48248_d_n0;
        locals.var_t3_dn2 = assign28200_e48248_d_n2;
        locals.var_t3_dn3 = assign28200_e48248_d_n3;
        locals.var_t3_dn4 = assign28200_e48248_d_n4;
        locals.var_t3_dn5 = assign28200_e48248_d_n5;
        locals.var_t3_dn6 = assign28200_e48248_d_n6;
        locals.var_t3_dn7 = assign28200_e48248_d_n7;
        locals.var_t3_dn8 = assign28200_e48248_d_n8;
        locals.var_t3_dn9 = assign28200_e48248_d_n9;
        locals.var_t3_dn10 = assign28200_e48248_d_n10;
        locals.var_t3_dn11 = assign28200_e48248_d_n11;
        locals.var_t3_dn13 = assign28200_e48248_d_n13;
        locals.var_t3_dn14 = assign28200_e48248_d_n14;
        locals.var_t3_rv = 0.0;

        let assign28210_e48251: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard456 = assign28210_e48251;
        locals.var_guard456_rv = 0.0;

        let (assign28220_e48265, assign28220_e48265_d_n0, assign28220_e48265_d_n2, assign28220_e48265_d_n3, assign28220_e48265_d_n4, assign28220_e48265_d_n5, assign28220_e48265_d_n6, assign28220_e48265_d_n7, assign28220_e48265_d_n8, assign28220_e48265_d_n9, assign28220_e48265_d_n10, assign28220_e48265_d_n11, assign28220_e48265_d_n13, assign28220_e48265_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign28220_e48259: f64 = (-locals.var_ved_jct);
        let assign28220_e48261: f64 = (assign28220_e48259 * locals.var_ved_jct);
        let assign28220_e48263: f64 = (assign28220_e48261 * locals.var_ved_jct);
        (assign28220_e48263, 0.0, 0.0, (((((-locals.var_ved_jct_dn3) * locals.var_ved_jct) + (assign28220_e48259 * locals.var_ved_jct_dn3)) * locals.var_ved_jct) + (assign28220_e48261 * locals.var_ved_jct_dn3)), 0.0, (((((-locals.var_ved_jct_dn5) * locals.var_ved_jct) + (assign28220_e48259 * locals.var_ved_jct_dn5)) * locals.var_ved_jct) + (assign28220_e48261 * locals.var_ved_jct_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28220_e48265;
        locals.var_t4_dn0 = assign28220_e48265_d_n0;
        locals.var_t4_dn2 = assign28220_e48265_d_n2;
        locals.var_t4_dn3 = assign28220_e48265_d_n3;
        locals.var_t4_dn4 = assign28220_e48265_d_n4;
        locals.var_t4_dn5 = assign28220_e48265_d_n5;
        locals.var_t4_dn6 = assign28220_e48265_d_n6;
        locals.var_t4_dn7 = assign28220_e48265_d_n7;
        locals.var_t4_dn8 = assign28220_e48265_d_n8;
        locals.var_t4_dn9 = assign28220_e48265_d_n9;
        locals.var_t4_dn10 = assign28220_e48265_d_n10;
        locals.var_t4_dn11 = assign28220_e48265_d_n11;
        locals.var_t4_dn13 = assign28220_e48265_d_n13;
        locals.var_t4_dn14 = assign28220_e48265_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign28230_e48279, assign28230_e48279_d_n0, assign28230_e48279_d_n2, assign28230_e48279_d_n3, assign28230_e48279_d_n4, assign28230_e48279_d_n5, assign28230_e48279_d_n6, assign28230_e48279_d_n7, assign28230_e48279_d_n8, assign28230_e48279_d_n9, assign28230_e48279_d_n10, assign28230_e48279_d_n11, assign28230_e48279_d_n13, assign28230_e48279_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign28230_e48274: f64 = (locals.var_t4).abs();
        let assign28230_e48275: f64 = (locals.var_cgidl_i + assign28230_e48274);
        let assign28230_e48277: f64 = (assign28230_e48275 + 1e-5);
        (assign28230_e48277, if locals.var_t4 >= 0.0 { locals.var_t4_dn0 } else { (-locals.var_t4_dn0) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn2 } else { (-locals.var_t4_dn2) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn3 } else { (-locals.var_t4_dn3) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn4 } else { (-locals.var_t4_dn4) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn5 } else { (-locals.var_t4_dn5) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn6 } else { (-locals.var_t4_dn6) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn7 } else { (-locals.var_t4_dn7) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn8 } else { (-locals.var_t4_dn8) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn9 } else { (-locals.var_t4_dn9) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn10 } else { (-locals.var_t4_dn10) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn11 } else { (-locals.var_t4_dn11) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn13 } else { (-locals.var_t4_dn13) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn14 } else { (-locals.var_t4_dn14) },)
    } else {
        (locals.var_t4a, locals.var_t4a_dn0, locals.var_t4a_dn2, locals.var_t4a_dn3, locals.var_t4a_dn4, locals.var_t4a_dn5, locals.var_t4a_dn6, locals.var_t4a_dn7, locals.var_t4a_dn8, locals.var_t4a_dn9, locals.var_t4a_dn10, locals.var_t4a_dn11, locals.var_t4a_dn13, locals.var_t4a_dn14,)
    }
};
        locals.var_t4a = assign28230_e48279;
        locals.var_t4a_dn0 = assign28230_e48279_d_n0;
        locals.var_t4a_dn2 = assign28230_e48279_d_n2;
        locals.var_t4a_dn3 = assign28230_e48279_d_n3;
        locals.var_t4a_dn4 = assign28230_e48279_d_n4;
        locals.var_t4a_dn5 = assign28230_e48279_d_n5;
        locals.var_t4a_dn6 = assign28230_e48279_d_n6;
        locals.var_t4a_dn7 = assign28230_e48279_d_n7;
        locals.var_t4a_dn8 = assign28230_e48279_d_n8;
        locals.var_t4a_dn9 = assign28230_e48279_d_n9;
        locals.var_t4a_dn10 = assign28230_e48279_d_n10;
        locals.var_t4a_dn11 = assign28230_e48279_d_n11;
        locals.var_t4a_dn13 = assign28230_e48279_d_n13;
        locals.var_t4a_dn14 = assign28230_e48279_d_n14;
        locals.var_t4a_rv = 0.0;

        let (assign28240_e48337, assign28240_e48337_d_n0, assign28240_e48337_d_n2, assign28240_e48337_d_n3, assign28240_e48337_d_n4, assign28240_e48337_d_n5, assign28240_e48337_d_n6, assign28240_e48337_d_n7, assign28240_e48337_d_n8, assign28240_e48337_d_n9, assign28240_e48337_d_n10, assign28240_e48337_d_n11, assign28240_e48337_d_n13, assign28240_e48337_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign28240_e48288: f64 = (locals.var_t4 / locals.var_t4a);
        let assign28240_e48290: f64 = (-10000.0);
        let assign28240_e48292: f64 = (assign28240_e48290 * 1e-6);
        let (assign28240_e48333, assign28240_e48333_d_n0, assign28240_e48333_d_n2, assign28240_e48333_d_n3, assign28240_e48333_d_n4, assign28240_e48333_d_n5, assign28240_e48333_d_n6, assign28240_e48333_d_n7, assign28240_e48333_d_n8, assign28240_e48333_d_n9, assign28240_e48333_d_n10, assign28240_e48333_d_n11, assign28240_e48333_d_n13, assign28240_e48333_d_n14,) = {
            if (!(assign28240_e48288 < assign28240_e48292)) {
                let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4a;
                let assign28240_e48298: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28240_e48301: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28240_e48304: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28240_e48305: f64 = (assign28240_e48301 * assign28240_e48304);
                let assign28240_e48308: f64 = (4.0 * 1e-6);
                let assign28240_e48310: f64 = (assign28240_e48308 * 1e-6);
                let assign28240_e48311: f64 = (assign28240_e48305 + assign28240_e48310);
                let assign28240_e48312: f64 = (assign28240_e48311).sqrt();
                let assign28240_e48313: f64 = (assign28240_e48298 + assign28240_e48312);
                let assign28240_e48314: f64 = (0.5 * assign28240_e48313);
                (assign28240_e48314, (0.5 * ((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))), (0.5 * ((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) * assign28240_e48304) + (assign28240_e48301 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28240_e48312)))),)
            } else {
                let assign28240_e48317: f64 = (locals.var_t4 / locals.var_t4a);
                let assign28240_e48319: f64 = (-10000.0);
                let assign28240_e48321: f64 = (assign28240_e48319 * 1e-6);
                let (assign28240_e48332, assign28240_e48332_d_n0, assign28240_e48332_d_n2, assign28240_e48332_d_n3, assign28240_e48332_d_n4, assign28240_e48332_d_n5, assign28240_e48332_d_n6, assign28240_e48332_d_n7, assign28240_e48332_d_n8, assign28240_e48332_d_n9, assign28240_e48332_d_n10, assign28240_e48332_d_n11, assign28240_e48332_d_n13, assign28240_e48332_d_n14,) = {
                    if (assign28240_e48317 < assign28240_e48321) {
                        let assign28240_e48324: f64 = (-1e-6);
                        let assign28240_e48326: f64 = (assign28240_e48324 * 1e-6);
                        let assign28240_e48329: f64 = (locals.var_t4 / locals.var_t4a);
                        let assign28240_e48330: f64 = (assign28240_e48326 / assign28240_e48329);
                        (assign28240_e48330, (-((assign28240_e48326 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))), (-((assign28240_e48326 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a))) / (assign28240_e48329 * assign28240_e48329))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28240_e48332, assign28240_e48332_d_n0, assign28240_e48332_d_n2, assign28240_e48332_d_n3, assign28240_e48332_d_n4, assign28240_e48332_d_n5, assign28240_e48332_d_n6, assign28240_e48332_d_n7, assign28240_e48332_d_n8, assign28240_e48332_d_n9, assign28240_e48332_d_n10, assign28240_e48332_d_n11, assign28240_e48332_d_n13, assign28240_e48332_d_n14,)
            }
        };
        let assign28240_e48335: f64 = (assign28240_e48333 - 1e-6);
        (assign28240_e48335, assign28240_e48333_d_n0, assign28240_e48333_d_n2, assign28240_e48333_d_n3, assign28240_e48333_d_n4, assign28240_e48333_d_n5, assign28240_e48333_d_n6, assign28240_e48333_d_n7, assign28240_e48333_d_n8, assign28240_e48333_d_n9, assign28240_e48333_d_n10, assign28240_e48333_d_n11, assign28240_e48333_d_n13, assign28240_e48333_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign28240_e48337;
        locals.var_t5_dn0 = assign28240_e48337_d_n0;
        locals.var_t5_dn2 = assign28240_e48337_d_n2;
        locals.var_t5_dn3 = assign28240_e48337_d_n3;
        locals.var_t5_dn4 = assign28240_e48337_d_n4;
        locals.var_t5_dn5 = assign28240_e48337_d_n5;
        locals.var_t5_dn6 = assign28240_e48337_d_n6;
        locals.var_t5_dn7 = assign28240_e48337_d_n7;
        locals.var_t5_dn8 = assign28240_e48337_d_n8;
        locals.var_t5_dn9 = assign28240_e48337_d_n9;
        locals.var_t5_dn10 = assign28240_e48337_d_n10;
        locals.var_t5_dn11 = assign28240_e48337_d_n11;
        locals.var_t5_dn13 = assign28240_e48337_d_n13;
        locals.var_t5_dn14 = assign28240_e48337_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign28250_e48356, assign28250_e48356_d_n0, assign28250_e48356_d_n2, assign28250_e48356_d_n3, assign28250_e48356_d_n4, assign28250_e48356_d_n5, assign28250_e48356_d_n6, assign28250_e48356_d_n7, assign28250_e48356_d_n8, assign28250_e48356_d_n9, assign28250_e48356_d_n10, assign28250_e48356_d_n11, assign28250_e48356_d_n13, assign28250_e48356_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign28250_e48346: f64 = (locals.var_agidl_i * locals.var_weff0);
        let assign28250_e48348: f64 = (assign28250_e48346 * locals.var_t3);
        let assign28250_e48350: f64 = (-locals.var_t2);
        let assign28250_e48351: f64 = { let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28250_e48352: f64 = (assign28250_e48348 * assign28250_e48351);
        let assign28250_e48354: f64 = (assign28250_e48352 * locals.var_t5);
        (assign28250_e48354, (((((assign28250_e48346 * locals.var_t3_dn0) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn0)), (((((assign28250_e48346 * locals.var_t3_dn2) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn2)), (((((assign28250_e48346 * locals.var_t3_dn3) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn3)), (((((assign28250_e48346 * locals.var_t3_dn4) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn4)), (((((assign28250_e48346 * locals.var_t3_dn5) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn5)), (((((assign28250_e48346 * locals.var_t3_dn6) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn6)), (((((assign28250_e48346 * locals.var_t3_dn7) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn7)), (((((assign28250_e48346 * locals.var_t3_dn8) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn8)), (((((assign28250_e48346 * locals.var_t3_dn9) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn9)), (((((assign28250_e48346 * locals.var_t3_dn10) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn10)), (((((assign28250_e48346 * locals.var_t3_dn11) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn11)), (((((assign28250_e48346 * locals.var_t3_dn13) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn13)), (((((assign28250_e48346 * locals.var_t3_dn14) * assign28250_e48351) + (assign28250_e48348 * ({ let limited_exp_arg = assign28250_e48350; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_t5) + (assign28250_e48352 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28250_e48356;
        locals.var_t6_dn0 = assign28250_e48356_d_n0;
        locals.var_t6_dn2 = assign28250_e48356_d_n2;
        locals.var_t6_dn3 = assign28250_e48356_d_n3;
        locals.var_t6_dn4 = assign28250_e48356_d_n4;
        locals.var_t6_dn5 = assign28250_e48356_d_n5;
        locals.var_t6_dn6 = assign28250_e48356_d_n6;
        locals.var_t6_dn7 = assign28250_e48356_d_n7;
        locals.var_t6_dn8 = assign28250_e48356_d_n8;
        locals.var_t6_dn9 = assign28250_e48356_d_n9;
        locals.var_t6_dn10 = assign28250_e48356_d_n10;
        locals.var_t6_dn11 = assign28250_e48356_d_n11;
        locals.var_t6_dn13 = assign28250_e48356_d_n13;
        locals.var_t6_dn14 = assign28250_e48356_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign28260_e48376, assign28260_e48376_d_n0, assign28260_e48376_d_n2, assign28260_e48376_d_n3, assign28260_e48376_d_n4, assign28260_e48376_d_n5, assign28260_e48376_d_n6, assign28260_e48376_d_n7, assign28260_e48376_d_n8, assign28260_e48376_d_n9, assign28260_e48376_d_n10, assign28260_e48376_d_n11, assign28260_e48376_d_n13, assign28260_e48376_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
        let assign28260_e48366: f64 = (locals.var_agidl_i * locals.var_weff0);
        let assign28260_e48368: f64 = (assign28260_e48366 * locals.var_t3);
        let assign28260_e48370: f64 = (-locals.var_t2);
        let assign28260_e48371: f64 = { let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28260_e48372: f64 = (assign28260_e48368 * assign28260_e48371);
        let assign28260_e48374: f64 = (assign28260_e48372 * locals.var_vds_noswap);
        (assign28260_e48374, ((((assign28260_e48366 * locals.var_t3_dn0) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn2) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn3) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn4) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_vds_noswap), (((((assign28260_e48366 * locals.var_t3_dn5) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_vds_noswap) + (assign28260_e48372 * locals.var_vds_noswap_dn5)), (((((assign28260_e48366 * locals.var_t3_dn6) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_vds_noswap) + (assign28260_e48372 * locals.var_vds_noswap_dn6)), ((((assign28260_e48366 * locals.var_t3_dn7) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn8) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn9) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn10) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn11) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn13) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_vds_noswap), ((((assign28260_e48366 * locals.var_t3_dn14) * assign28260_e48371) + (assign28260_e48368 * ({ let limited_exp_arg = assign28260_e48370; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_vds_noswap),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28260_e48376;
        locals.var_t6_dn0 = assign28260_e48376_d_n0;
        locals.var_t6_dn2 = assign28260_e48376_d_n2;
        locals.var_t6_dn3 = assign28260_e48376_d_n3;
        locals.var_t6_dn4 = assign28260_e48376_d_n4;
        locals.var_t6_dn5 = assign28260_e48376_d_n5;
        locals.var_t6_dn6 = assign28260_e48376_d_n6;
        locals.var_t6_dn7 = assign28260_e48376_d_n7;
        locals.var_t6_dn8 = assign28260_e48376_d_n8;
        locals.var_t6_dn9 = assign28260_e48376_d_n9;
        locals.var_t6_dn10 = assign28260_e48376_d_n10;
        locals.var_t6_dn11 = assign28260_e48376_d_n11;
        locals.var_t6_dn13 = assign28260_e48376_d_n13;
        locals.var_t6_dn14 = assign28260_e48376_d_n14;
        locals.var_t6_rv = 0.0;

        let assign28270_e48383: f64 = if ((p.p70 == 3.0) && (locals.var_atatd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard457 = assign28270_e48383;
        locals.var_guard457_rv = 0.0;

        let assign28280_e48386: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign28280_e48386;
        locals.var_guard458_rv = 0.0;

        let (assign28290_e48467, assign28290_e48467_d_n4,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign28290_e48396: f64 = (locals.var_ttat_i * locals.var_deltemp);
        let assign28290_e48397: f64 = (1.0 + assign28290_e48396);
        let assign28290_e48399: f64 = (assign28290_e48397 - 1e-6);
        let assign28290_e48401: f64 = (-10000.0);
        let assign28290_e48403: f64 = (assign28290_e48401 * 0.001);
        let (assign28290_e48464, assign28290_e48464_d_n4,) = {
            if (!(assign28290_e48399 < assign28290_e48403)) {
                let assign28290_e48410: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28290_e48411: f64 = (1.0 + assign28290_e48410);
                let assign28290_e48413: f64 = (assign28290_e48411 - 1e-6);
                let assign28290_e48417: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28290_e48418: f64 = (1.0 + assign28290_e48417);
                let assign28290_e48420: f64 = (assign28290_e48418 - 1e-6);
                let assign28290_e48424: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28290_e48425: f64 = (1.0 + assign28290_e48424);
                let assign28290_e48427: f64 = (assign28290_e48425 - 1e-6);
                let assign28290_e48428: f64 = (assign28290_e48420 * assign28290_e48427);
                let assign28290_e48431: f64 = (4.0 * 0.001);
                let assign28290_e48433: f64 = (assign28290_e48431 * 0.001);
                let assign28290_e48434: f64 = (assign28290_e48428 + assign28290_e48433);
                let assign28290_e48435: f64 = (assign28290_e48434).sqrt();
                let assign28290_e48436: f64 = (assign28290_e48413 + assign28290_e48435);
                let assign28290_e48437: f64 = (0.5 * assign28290_e48436);
                (assign28290_e48437, (0.5 * ((locals.var_ttat_i * locals.var_deltemp_dn4) + ((((locals.var_ttat_i * locals.var_deltemp_dn4) * assign28290_e48427) + (assign28290_e48420 * (locals.var_ttat_i * locals.var_deltemp_dn4))) / (2.0 * assign28290_e48435)))),)
            } else {
                let assign28290_e48441: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28290_e48442: f64 = (1.0 + assign28290_e48441);
                let assign28290_e48444: f64 = (assign28290_e48442 - 1e-6);
                let assign28290_e48446: f64 = (-10000.0);
                let assign28290_e48448: f64 = (assign28290_e48446 * 0.001);
                let (assign28290_e48463, assign28290_e48463_d_n4,) = {
                    if (assign28290_e48444 < assign28290_e48448) {
                        let assign28290_e48451: f64 = (-0.001);
                        let assign28290_e48453: f64 = (assign28290_e48451 * 0.001);
                        let assign28290_e48457: f64 = (locals.var_ttat_i * locals.var_deltemp);
                        let assign28290_e48458: f64 = (1.0 + assign28290_e48457);
                        let assign28290_e48460: f64 = (assign28290_e48458 - 1e-6);
                        let assign28290_e48461: f64 = (assign28290_e48453 / assign28290_e48460);
                        (assign28290_e48461, (-((assign28290_e48453 * (locals.var_ttat_i * locals.var_deltemp_dn4)) / (assign28290_e48460 * assign28290_e48460))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign28290_e48463, assign28290_e48463_d_n4,)
            }
        };
        let assign28290_e48465: f64 = (locals.var_ctatd_i * assign28290_e48464);
        (assign28290_e48465, (locals.var_ctatd_i * assign28290_e48464_d_n4),)
    } else {
        (locals.var_ctatd_t, locals.var_ctatd_t_dn4,)
    }
};
        locals.var_ctatd_t = assign28290_e48467;
        locals.var_ctatd_t_dn4 = assign28290_e48467_d_n4;
        locals.var_ctatd_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28300_e48489, assign28300_e48489_d_n0, assign28300_e48489_d_n2, assign28300_e48489_d_n3, assign28300_e48489_d_n4, assign28300_e48489_d_n5, assign28300_e48489_d_n6, assign28300_e48489_d_n7, assign28300_e48489_d_n8, assign28300_e48489_d_n9, assign28300_e48489_d_n10, assign28300_e48489_d_n11, assign28300_e48489_d_n13, assign28300_e48489_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign28300_e48475: f64 = (locals.var_btatd_i * locals.var_vgd_noswap);
        let assign28300_e48477: f64 = (assign28300_e48475 * locals.var_vgd_noswap);
        let assign28300_e48480: f64 = (locals.var_ctatd_t * locals.var_vgd_noswap);
        let assign28300_e48481: f64 = (assign28300_e48477 - assign28300_e48480);
        let assign28300_e48483: f64 = (assign28300_e48481 - locals.var_dtatd_i);
        let assign28300_e48485: f64 = (assign28300_e48483 + locals.var_vfbsd_v);
        let assign28300_e48487: f64 = (assign28300_e48485 / locals.var_vtm);
        (assign28300_e48487, (locals.var_vfbsd_v_dn0 / locals.var_vtm), (locals.var_vfbsd_v_dn2 / locals.var_vtm), (locals.var_vfbsd_v_dn3 / locals.var_vtm), (((((-(locals.var_ctatd_t_dn4 * locals.var_vgd_noswap)) + locals.var_vfbsd_v_dn4) * locals.var_vtm) - (assign28300_e48485 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), ((((((locals.var_btatd_i * locals.var_vgd_noswap_dn5) * locals.var_vgd_noswap) + (assign28300_e48475 * locals.var_vgd_noswap_dn5)) - (locals.var_ctatd_t * locals.var_vgd_noswap_dn5)) + locals.var_vfbsd_v_dn5) / locals.var_vtm), (locals.var_vfbsd_v_dn6 / locals.var_vtm), (locals.var_vfbsd_v_dn7 / locals.var_vtm), (locals.var_vfbsd_v_dn8 / locals.var_vtm), (locals.var_vfbsd_v_dn9 / locals.var_vtm), (locals.var_vfbsd_v_dn10 / locals.var_vtm), ((((((locals.var_btatd_i * locals.var_vgd_noswap_dn11) * locals.var_vgd_noswap) + (assign28300_e48475 * locals.var_vgd_noswap_dn11)) - (locals.var_ctatd_t * locals.var_vgd_noswap_dn11)) + locals.var_vfbsd_v_dn11) / locals.var_vtm), (locals.var_vfbsd_v_dn13 / locals.var_vtm), (locals.var_vfbsd_v_dn14 / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28300_e48489;
        locals.var_t1_dn0 = assign28300_e48489_d_n0;
        locals.var_t1_dn2 = assign28300_e48489_d_n2;
        locals.var_t1_dn3 = assign28300_e48489_d_n3;
        locals.var_t1_dn4 = assign28300_e48489_d_n4;
        locals.var_t1_dn5 = assign28300_e48489_d_n5;
        locals.var_t1_dn6 = assign28300_e48489_d_n6;
        locals.var_t1_dn7 = assign28300_e48489_d_n7;
        locals.var_t1_dn8 = assign28300_e48489_d_n8;
        locals.var_t1_dn9 = assign28300_e48489_d_n9;
        locals.var_t1_dn10 = assign28300_e48489_d_n10;
        locals.var_t1_dn11 = assign28300_e48489_d_n11;
        locals.var_t1_dn13 = assign28300_e48489_d_n13;
        locals.var_t1_dn14 = assign28300_e48489_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28310_e48504, assign28310_e48504_d_n0, assign28310_e48504_d_n2, assign28310_e48504_d_n3, assign28310_e48504_d_n4, assign28310_e48504_d_n5, assign28310_e48504_d_n6, assign28310_e48504_d_n7, assign28310_e48504_d_n8, assign28310_e48504_d_n9, assign28310_e48504_d_n10, assign28310_e48504_d_n11, assign28310_e48504_d_n13, assign28310_e48504_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign28310_e48497: f64 = (locals.var_atatd_i * locals.var_weff0);
        let assign28310_e48499: f64 = (assign28310_e48497 * locals.var_ni);
        let assign28310_e48501: f64 = { let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28310_e48502: f64 = (assign28310_e48499 * assign28310_e48501);
        (assign28310_e48502, (((assign28310_e48497 * locals.var_ni_dn0) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn0))), (((assign28310_e48497 * locals.var_ni_dn2) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn2))), (((assign28310_e48497 * locals.var_ni_dn3) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn3))), (((assign28310_e48497 * locals.var_ni_dn4) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn4))), (((assign28310_e48497 * locals.var_ni_dn5) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn5))), (((assign28310_e48497 * locals.var_ni_dn6) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn6))), (((assign28310_e48497 * locals.var_ni_dn7) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn7))), (((assign28310_e48497 * locals.var_ni_dn8) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn8))), (((assign28310_e48497 * locals.var_ni_dn9) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn9))), (((assign28310_e48497 * locals.var_ni_dn10) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn10))), (((assign28310_e48497 * locals.var_ni_dn11) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn11))), (((assign28310_e48497 * locals.var_ni_dn13) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn13))), (((assign28310_e48497 * locals.var_ni_dn14) * assign28310_e48501) + (assign28310_e48499 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28310_e48504;
        locals.var_t2_dn0 = assign28310_e48504_d_n0;
        locals.var_t2_dn2 = assign28310_e48504_d_n2;
        locals.var_t2_dn3 = assign28310_e48504_d_n3;
        locals.var_t2_dn4 = assign28310_e48504_d_n4;
        locals.var_t2_dn5 = assign28310_e48504_d_n5;
        locals.var_t2_dn6 = assign28310_e48504_d_n6;
        locals.var_t2_dn7 = assign28310_e48504_d_n7;
        locals.var_t2_dn8 = assign28310_e48504_d_n8;
        locals.var_t2_dn9 = assign28310_e48504_d_n9;
        locals.var_t2_dn10 = assign28310_e48504_d_n10;
        locals.var_t2_dn11 = assign28310_e48504_d_n11;
        locals.var_t2_dn13 = assign28310_e48504_d_n13;
        locals.var_t2_dn14 = assign28310_e48504_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28320_e48517, assign28320_e48517_d_n0, assign28320_e48517_d_n2, assign28320_e48517_d_n3, assign28320_e48517_d_n4, assign28320_e48517_d_n5, assign28320_e48517_d_n6, assign28320_e48517_d_n7, assign28320_e48517_d_n8, assign28320_e48517_d_n9, assign28320_e48517_d_n10, assign28320_e48517_d_n11, assign28320_e48517_d_n13, assign28320_e48517_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign28320_e48511: f64 = (-locals.var_ved_jct);
        let assign28320_e48513: f64 = (assign28320_e48511 * locals.var_ved_jct);
        let assign28320_e48515: f64 = (assign28320_e48513 * locals.var_ved_jct);
        (assign28320_e48515, 0.0, 0.0, (((((-locals.var_ved_jct_dn3) * locals.var_ved_jct) + (assign28320_e48511 * locals.var_ved_jct_dn3)) * locals.var_ved_jct) + (assign28320_e48513 * locals.var_ved_jct_dn3)), 0.0, (((((-locals.var_ved_jct_dn5) * locals.var_ved_jct) + (assign28320_e48511 * locals.var_ved_jct_dn5)) * locals.var_ved_jct) + (assign28320_e48513 * locals.var_ved_jct_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28320_e48517;
        locals.var_t4_dn0 = assign28320_e48517_d_n0;
        locals.var_t4_dn2 = assign28320_e48517_d_n2;
        locals.var_t4_dn3 = assign28320_e48517_d_n3;
        locals.var_t4_dn4 = assign28320_e48517_d_n4;
        locals.var_t4_dn5 = assign28320_e48517_d_n5;
        locals.var_t4_dn6 = assign28320_e48517_d_n6;
        locals.var_t4_dn7 = assign28320_e48517_d_n7;
        locals.var_t4_dn8 = assign28320_e48517_d_n8;
        locals.var_t4_dn9 = assign28320_e48517_d_n9;
        locals.var_t4_dn10 = assign28320_e48517_d_n10;
        locals.var_t4_dn11 = assign28320_e48517_d_n11;
        locals.var_t4_dn13 = assign28320_e48517_d_n13;
        locals.var_t4_dn14 = assign28320_e48517_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign28330_e48530, assign28330_e48530_d_n0, assign28330_e48530_d_n2, assign28330_e48530_d_n3, assign28330_e48530_d_n4, assign28330_e48530_d_n5, assign28330_e48530_d_n6, assign28330_e48530_d_n7, assign28330_e48530_d_n8, assign28330_e48530_d_n9, assign28330_e48530_d_n10, assign28330_e48530_d_n11, assign28330_e48530_d_n13, assign28330_e48530_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign28330_e48525: f64 = (locals.var_t4).abs();
        let assign28330_e48526: f64 = (locals.var_cgidl_i + assign28330_e48525);
        let assign28330_e48528: f64 = (assign28330_e48526 + 1e-5);
        (assign28330_e48528, if locals.var_t4 >= 0.0 { locals.var_t4_dn0 } else { (-locals.var_t4_dn0) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn2 } else { (-locals.var_t4_dn2) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn3 } else { (-locals.var_t4_dn3) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn4 } else { (-locals.var_t4_dn4) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn5 } else { (-locals.var_t4_dn5) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn6 } else { (-locals.var_t4_dn6) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn7 } else { (-locals.var_t4_dn7) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn8 } else { (-locals.var_t4_dn8) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn9 } else { (-locals.var_t4_dn9) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn10 } else { (-locals.var_t4_dn10) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn11 } else { (-locals.var_t4_dn11) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn13 } else { (-locals.var_t4_dn13) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn14 } else { (-locals.var_t4_dn14) },)
    } else {
        (locals.var_t4a, locals.var_t4a_dn0, locals.var_t4a_dn2, locals.var_t4a_dn3, locals.var_t4a_dn4, locals.var_t4a_dn5, locals.var_t4a_dn6, locals.var_t4a_dn7, locals.var_t4a_dn8, locals.var_t4a_dn9, locals.var_t4a_dn10, locals.var_t4a_dn11, locals.var_t4a_dn13, locals.var_t4a_dn14,)
    }
};
        locals.var_t4a = assign28330_e48530;
        locals.var_t4a_dn0 = assign28330_e48530_d_n0;
        locals.var_t4a_dn2 = assign28330_e48530_d_n2;
        locals.var_t4a_dn3 = assign28330_e48530_d_n3;
        locals.var_t4a_dn4 = assign28330_e48530_d_n4;
        locals.var_t4a_dn5 = assign28330_e48530_d_n5;
        locals.var_t4a_dn6 = assign28330_e48530_d_n6;
        locals.var_t4a_dn7 = assign28330_e48530_d_n7;
        locals.var_t4a_dn8 = assign28330_e48530_d_n8;
        locals.var_t4a_dn9 = assign28330_e48530_d_n9;
        locals.var_t4a_dn10 = assign28330_e48530_d_n10;
        locals.var_t4a_dn11 = assign28330_e48530_d_n11;
        locals.var_t4a_dn13 = assign28330_e48530_d_n13;
        locals.var_t4a_dn14 = assign28330_e48530_d_n14;
        locals.var_t4a_rv = 0.0;

        let (assign28340_e48587, assign28340_e48587_d_n0, assign28340_e48587_d_n2, assign28340_e48587_d_n3, assign28340_e48587_d_n4, assign28340_e48587_d_n5, assign28340_e48587_d_n6, assign28340_e48587_d_n7, assign28340_e48587_d_n8, assign28340_e48587_d_n9, assign28340_e48587_d_n10, assign28340_e48587_d_n11, assign28340_e48587_d_n13, assign28340_e48587_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign28340_e48538: f64 = (locals.var_t4 / locals.var_t4a);
        let assign28340_e48540: f64 = (-10000.0);
        let assign28340_e48542: f64 = (assign28340_e48540 * 1e-6);
        let (assign28340_e48583, assign28340_e48583_d_n0, assign28340_e48583_d_n2, assign28340_e48583_d_n3, assign28340_e48583_d_n4, assign28340_e48583_d_n5, assign28340_e48583_d_n6, assign28340_e48583_d_n7, assign28340_e48583_d_n8, assign28340_e48583_d_n9, assign28340_e48583_d_n10, assign28340_e48583_d_n11, assign28340_e48583_d_n13, assign28340_e48583_d_n14,) = {
            if (!(assign28340_e48538 < assign28340_e48542)) {
                let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4a;
                let assign28340_e48548: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28340_e48551: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28340_e48554: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28340_e48555: f64 = (assign28340_e48551 * assign28340_e48554);
                let assign28340_e48558: f64 = (4.0 * 1e-6);
                let assign28340_e48560: f64 = (assign28340_e48558 * 1e-6);
                let assign28340_e48561: f64 = (assign28340_e48555 + assign28340_e48560);
                let assign28340_e48562: f64 = (assign28340_e48561).sqrt();
                let assign28340_e48563: f64 = (assign28340_e48548 + assign28340_e48562);
                let assign28340_e48564: f64 = (0.5 * assign28340_e48563);
                (assign28340_e48564, (0.5 * ((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))), (0.5 * ((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) * assign28340_e48554) + (assign28340_e48551 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28340_e48562)))),)
            } else {
                let assign28340_e48567: f64 = (locals.var_t4 / locals.var_t4a);
                let assign28340_e48569: f64 = (-10000.0);
                let assign28340_e48571: f64 = (assign28340_e48569 * 1e-6);
                let (assign28340_e48582, assign28340_e48582_d_n0, assign28340_e48582_d_n2, assign28340_e48582_d_n3, assign28340_e48582_d_n4, assign28340_e48582_d_n5, assign28340_e48582_d_n6, assign28340_e48582_d_n7, assign28340_e48582_d_n8, assign28340_e48582_d_n9, assign28340_e48582_d_n10, assign28340_e48582_d_n11, assign28340_e48582_d_n13, assign28340_e48582_d_n14,) = {
                    if (assign28340_e48567 < assign28340_e48571) {
                        let assign28340_e48574: f64 = (-1e-6);
                        let assign28340_e48576: f64 = (assign28340_e48574 * 1e-6);
                        let assign28340_e48579: f64 = (locals.var_t4 / locals.var_t4a);
                        let assign28340_e48580: f64 = (assign28340_e48576 / assign28340_e48579);
                        (assign28340_e48580, (-((assign28340_e48576 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))), (-((assign28340_e48576 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a))) / (assign28340_e48579 * assign28340_e48579))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28340_e48582, assign28340_e48582_d_n0, assign28340_e48582_d_n2, assign28340_e48582_d_n3, assign28340_e48582_d_n4, assign28340_e48582_d_n5, assign28340_e48582_d_n6, assign28340_e48582_d_n7, assign28340_e48582_d_n8, assign28340_e48582_d_n9, assign28340_e48582_d_n10, assign28340_e48582_d_n11, assign28340_e48582_d_n13, assign28340_e48582_d_n14,)
            }
        };
        let assign28340_e48585: f64 = (assign28340_e48583 - 1e-6);
        (assign28340_e48585, assign28340_e48583_d_n0, assign28340_e48583_d_n2, assign28340_e48583_d_n3, assign28340_e48583_d_n4, assign28340_e48583_d_n5, assign28340_e48583_d_n6, assign28340_e48583_d_n7, assign28340_e48583_d_n8, assign28340_e48583_d_n9, assign28340_e48583_d_n10, assign28340_e48583_d_n11, assign28340_e48583_d_n13, assign28340_e48583_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign28340_e48587;
        locals.var_t5_dn0 = assign28340_e48587_d_n0;
        locals.var_t5_dn2 = assign28340_e48587_d_n2;
        locals.var_t5_dn3 = assign28340_e48587_d_n3;
        locals.var_t5_dn4 = assign28340_e48587_d_n4;
        locals.var_t5_dn5 = assign28340_e48587_d_n5;
        locals.var_t5_dn6 = assign28340_e48587_d_n6;
        locals.var_t5_dn7 = assign28340_e48587_d_n7;
        locals.var_t5_dn8 = assign28340_e48587_d_n8;
        locals.var_t5_dn9 = assign28340_e48587_d_n9;
        locals.var_t5_dn10 = assign28340_e48587_d_n10;
        locals.var_t5_dn11 = assign28340_e48587_d_n11;
        locals.var_t5_dn13 = assign28340_e48587_d_n13;
        locals.var_t5_dn14 = assign28340_e48587_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign28350_e48599, assign28350_e48599_d_n0, assign28350_e48599_d_n2, assign28350_e48599_d_n3, assign28350_e48599_d_n4, assign28350_e48599_d_n5, assign28350_e48599_d_n6, assign28350_e48599_d_n7, assign28350_e48599_d_n8, assign28350_e48599_d_n9, assign28350_e48599_d_n10, assign28350_e48599_d_n11, assign28350_e48599_d_n13, assign28350_e48599_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 != 0.0)) {
        let assign28350_e48596: f64 = (locals.var_t2 * locals.var_t5);
        let assign28350_e48597: f64 = (locals.var_t6 + assign28350_e48596);
        (assign28350_e48597, (locals.var_t6_dn0 + ((locals.var_t2_dn0 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn0))), (locals.var_t6_dn2 + ((locals.var_t2_dn2 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn2))), (locals.var_t6_dn3 + ((locals.var_t2_dn3 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn3))), (locals.var_t6_dn4 + ((locals.var_t2_dn4 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn4))), (locals.var_t6_dn5 + ((locals.var_t2_dn5 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn5))), (locals.var_t6_dn6 + ((locals.var_t2_dn6 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn6))), (locals.var_t6_dn7 + ((locals.var_t2_dn7 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn7))), (locals.var_t6_dn8 + ((locals.var_t2_dn8 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn8))), (locals.var_t6_dn9 + ((locals.var_t2_dn9 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn9))), (locals.var_t6_dn10 + ((locals.var_t2_dn10 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn10))), (locals.var_t6_dn11 + ((locals.var_t2_dn11 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn11))), (locals.var_t6_dn13 + ((locals.var_t2_dn13 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn13))), (locals.var_t6_dn14 + ((locals.var_t2_dn14 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28350_e48599;
        locals.var_t6_dn0 = assign28350_e48599_d_n0;
        locals.var_t6_dn2 = assign28350_e48599_d_n2;
        locals.var_t6_dn3 = assign28350_e48599_d_n3;
        locals.var_t6_dn4 = assign28350_e48599_d_n4;
        locals.var_t6_dn5 = assign28350_e48599_d_n5;
        locals.var_t6_dn6 = assign28350_e48599_d_n6;
        locals.var_t6_dn7 = assign28350_e48599_d_n7;
        locals.var_t6_dn8 = assign28350_e48599_d_n8;
        locals.var_t6_dn9 = assign28350_e48599_d_n9;
        locals.var_t6_dn10 = assign28350_e48599_d_n10;
        locals.var_t6_dn11 = assign28350_e48599_d_n11;
        locals.var_t6_dn13 = assign28350_e48599_d_n13;
        locals.var_t6_dn14 = assign28350_e48599_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign28360_e48681, assign28360_e48681_d_n4,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign28360_e48610: f64 = (locals.var_ttat_i * locals.var_deltemp);
        let assign28360_e48611: f64 = (1.0 + assign28360_e48610);
        let assign28360_e48613: f64 = (assign28360_e48611 - 1e-6);
        let assign28360_e48615: f64 = (-10000.0);
        let assign28360_e48617: f64 = (assign28360_e48615 * 0.001);
        let (assign28360_e48678, assign28360_e48678_d_n4,) = {
            if (!(assign28360_e48613 < assign28360_e48617)) {
                let assign28360_e48624: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28360_e48625: f64 = (1.0 + assign28360_e48624);
                let assign28360_e48627: f64 = (assign28360_e48625 - 1e-6);
                let assign28360_e48631: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28360_e48632: f64 = (1.0 + assign28360_e48631);
                let assign28360_e48634: f64 = (assign28360_e48632 - 1e-6);
                let assign28360_e48638: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28360_e48639: f64 = (1.0 + assign28360_e48638);
                let assign28360_e48641: f64 = (assign28360_e48639 - 1e-6);
                let assign28360_e48642: f64 = (assign28360_e48634 * assign28360_e48641);
                let assign28360_e48645: f64 = (4.0 * 0.001);
                let assign28360_e48647: f64 = (assign28360_e48645 * 0.001);
                let assign28360_e48648: f64 = (assign28360_e48642 + assign28360_e48647);
                let assign28360_e48649: f64 = (assign28360_e48648).sqrt();
                let assign28360_e48650: f64 = (assign28360_e48627 + assign28360_e48649);
                let assign28360_e48651: f64 = (0.5 * assign28360_e48650);
                (assign28360_e48651, (0.5 * ((locals.var_ttat_i * locals.var_deltemp_dn4) + ((((locals.var_ttat_i * locals.var_deltemp_dn4) * assign28360_e48641) + (assign28360_e48634 * (locals.var_ttat_i * locals.var_deltemp_dn4))) / (2.0 * assign28360_e48649)))),)
            } else {
                let assign28360_e48655: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28360_e48656: f64 = (1.0 + assign28360_e48655);
                let assign28360_e48658: f64 = (assign28360_e48656 - 1e-6);
                let assign28360_e48660: f64 = (-10000.0);
                let assign28360_e48662: f64 = (assign28360_e48660 * 0.001);
                let (assign28360_e48677, assign28360_e48677_d_n4,) = {
                    if (assign28360_e48658 < assign28360_e48662) {
                        let assign28360_e48665: f64 = (-0.001);
                        let assign28360_e48667: f64 = (assign28360_e48665 * 0.001);
                        let assign28360_e48671: f64 = (locals.var_ttat_i * locals.var_deltemp);
                        let assign28360_e48672: f64 = (1.0 + assign28360_e48671);
                        let assign28360_e48674: f64 = (assign28360_e48672 - 1e-6);
                        let assign28360_e48675: f64 = (assign28360_e48667 / assign28360_e48674);
                        (assign28360_e48675, (-((assign28360_e48667 * (locals.var_ttat_i * locals.var_deltemp_dn4)) / (assign28360_e48674 * assign28360_e48674))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign28360_e48677, assign28360_e48677_d_n4,)
            }
        };
        let assign28360_e48679: f64 = (locals.var_ctatd_i * assign28360_e48678);
        (assign28360_e48679, (locals.var_ctatd_i * assign28360_e48678_d_n4),)
    } else {
        (locals.var_ctatd_t, locals.var_ctatd_t_dn4,)
    }
};
        locals.var_ctatd_t = assign28360_e48681;
        locals.var_ctatd_t_dn4 = assign28360_e48681_d_n4;
        locals.var_ctatd_t_rv = 0.0;

        let (assign28370_e48704, assign28370_e48704_d_n0, assign28370_e48704_d_n2, assign28370_e48704_d_n3, assign28370_e48704_d_n4, assign28370_e48704_d_n5, assign28370_e48704_d_n6, assign28370_e48704_d_n7, assign28370_e48704_d_n8, assign28370_e48704_d_n9, assign28370_e48704_d_n10, assign28370_e48704_d_n11, assign28370_e48704_d_n13, assign28370_e48704_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign28370_e48690: f64 = (locals.var_btatd_i * locals.var_vgd_noswap);
        let assign28370_e48692: f64 = (assign28370_e48690 * locals.var_vgd_noswap);
        let assign28370_e48695: f64 = (locals.var_ctatd_t * locals.var_vgd_noswap);
        let assign28370_e48696: f64 = (assign28370_e48692 - assign28370_e48695);
        let assign28370_e48698: f64 = (assign28370_e48696 - locals.var_dtatd_i);
        let assign28370_e48700: f64 = (assign28370_e48698 + locals.var_vfbsd_v);
        let assign28370_e48702: f64 = (assign28370_e48700 / locals.var_vtm);
        (assign28370_e48702, (locals.var_vfbsd_v_dn0 / locals.var_vtm), (locals.var_vfbsd_v_dn2 / locals.var_vtm), (locals.var_vfbsd_v_dn3 / locals.var_vtm), (((((-(locals.var_ctatd_t_dn4 * locals.var_vgd_noswap)) + locals.var_vfbsd_v_dn4) * locals.var_vtm) - (assign28370_e48700 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), ((((((locals.var_btatd_i * locals.var_vgd_noswap_dn5) * locals.var_vgd_noswap) + (assign28370_e48690 * locals.var_vgd_noswap_dn5)) - (locals.var_ctatd_t * locals.var_vgd_noswap_dn5)) + locals.var_vfbsd_v_dn5) / locals.var_vtm), (locals.var_vfbsd_v_dn6 / locals.var_vtm), (locals.var_vfbsd_v_dn7 / locals.var_vtm), (locals.var_vfbsd_v_dn8 / locals.var_vtm), (locals.var_vfbsd_v_dn9 / locals.var_vtm), (locals.var_vfbsd_v_dn10 / locals.var_vtm), ((((((locals.var_btatd_i * locals.var_vgd_noswap_dn11) * locals.var_vgd_noswap) + (assign28370_e48690 * locals.var_vgd_noswap_dn11)) - (locals.var_ctatd_t * locals.var_vgd_noswap_dn11)) + locals.var_vfbsd_v_dn11) / locals.var_vtm), (locals.var_vfbsd_v_dn13 / locals.var_vtm), (locals.var_vfbsd_v_dn14 / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28370_e48704;
        locals.var_t1_dn0 = assign28370_e48704_d_n0;
        locals.var_t1_dn2 = assign28370_e48704_d_n2;
        locals.var_t1_dn3 = assign28370_e48704_d_n3;
        locals.var_t1_dn4 = assign28370_e48704_d_n4;
        locals.var_t1_dn5 = assign28370_e48704_d_n5;
        locals.var_t1_dn6 = assign28370_e48704_d_n6;
        locals.var_t1_dn7 = assign28370_e48704_d_n7;
        locals.var_t1_dn8 = assign28370_e48704_d_n8;
        locals.var_t1_dn9 = assign28370_e48704_d_n9;
        locals.var_t1_dn10 = assign28370_e48704_d_n10;
        locals.var_t1_dn11 = assign28370_e48704_d_n11;
        locals.var_t1_dn13 = assign28370_e48704_d_n13;
        locals.var_t1_dn14 = assign28370_e48704_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28380_e48720, assign28380_e48720_d_n0, assign28380_e48720_d_n2, assign28380_e48720_d_n3, assign28380_e48720_d_n4, assign28380_e48720_d_n5, assign28380_e48720_d_n6, assign28380_e48720_d_n7, assign28380_e48720_d_n8, assign28380_e48720_d_n9, assign28380_e48720_d_n10, assign28380_e48720_d_n11, assign28380_e48720_d_n13, assign28380_e48720_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign28380_e48713: f64 = (locals.var_atatd_i * locals.var_weff0);
        let assign28380_e48715: f64 = (assign28380_e48713 * locals.var_ni);
        let assign28380_e48717: f64 = { let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28380_e48718: f64 = (assign28380_e48715 * assign28380_e48717);
        (assign28380_e48718, (((assign28380_e48713 * locals.var_ni_dn0) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn0))), (((assign28380_e48713 * locals.var_ni_dn2) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn2))), (((assign28380_e48713 * locals.var_ni_dn3) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn3))), (((assign28380_e48713 * locals.var_ni_dn4) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn4))), (((assign28380_e48713 * locals.var_ni_dn5) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn5))), (((assign28380_e48713 * locals.var_ni_dn6) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn6))), (((assign28380_e48713 * locals.var_ni_dn7) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn7))), (((assign28380_e48713 * locals.var_ni_dn8) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn8))), (((assign28380_e48713 * locals.var_ni_dn9) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn9))), (((assign28380_e48713 * locals.var_ni_dn10) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn10))), (((assign28380_e48713 * locals.var_ni_dn11) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn11))), (((assign28380_e48713 * locals.var_ni_dn13) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn13))), (((assign28380_e48713 * locals.var_ni_dn14) * assign28380_e48717) + (assign28380_e48715 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28380_e48720;
        locals.var_t2_dn0 = assign28380_e48720_d_n0;
        locals.var_t2_dn2 = assign28380_e48720_d_n2;
        locals.var_t2_dn3 = assign28380_e48720_d_n3;
        locals.var_t2_dn4 = assign28380_e48720_d_n4;
        locals.var_t2_dn5 = assign28380_e48720_d_n5;
        locals.var_t2_dn6 = assign28380_e48720_d_n6;
        locals.var_t2_dn7 = assign28380_e48720_d_n7;
        locals.var_t2_dn8 = assign28380_e48720_d_n8;
        locals.var_t2_dn9 = assign28380_e48720_d_n9;
        locals.var_t2_dn10 = assign28380_e48720_d_n10;
        locals.var_t2_dn11 = assign28380_e48720_d_n11;
        locals.var_t2_dn13 = assign28380_e48720_d_n13;
        locals.var_t2_dn14 = assign28380_e48720_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28390_e48733, assign28390_e48733_d_n0, assign28390_e48733_d_n2, assign28390_e48733_d_n3, assign28390_e48733_d_n4, assign28390_e48733_d_n5, assign28390_e48733_d_n6, assign28390_e48733_d_n7, assign28390_e48733_d_n8, assign28390_e48733_d_n9, assign28390_e48733_d_n10, assign28390_e48733_d_n11, assign28390_e48733_d_n13, assign28390_e48733_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard457 != 0.0)) && (locals.var_guard458 == 0.0)) {
        let assign28390_e48730: f64 = (locals.var_t2 * locals.var_vds_noswap);
        let assign28390_e48731: f64 = (locals.var_t6 + assign28390_e48730);
        (assign28390_e48731, (locals.var_t6_dn0 + (locals.var_t2_dn0 * locals.var_vds_noswap)), (locals.var_t6_dn2 + (locals.var_t2_dn2 * locals.var_vds_noswap)), (locals.var_t6_dn3 + (locals.var_t2_dn3 * locals.var_vds_noswap)), (locals.var_t6_dn4 + (locals.var_t2_dn4 * locals.var_vds_noswap)), (locals.var_t6_dn5 + ((locals.var_t2_dn5 * locals.var_vds_noswap) + (locals.var_t2 * locals.var_vds_noswap_dn5))), (locals.var_t6_dn6 + ((locals.var_t2_dn6 * locals.var_vds_noswap) + (locals.var_t2 * locals.var_vds_noswap_dn6))), (locals.var_t6_dn7 + (locals.var_t2_dn7 * locals.var_vds_noswap)), (locals.var_t6_dn8 + (locals.var_t2_dn8 * locals.var_vds_noswap)), (locals.var_t6_dn9 + (locals.var_t2_dn9 * locals.var_vds_noswap)), (locals.var_t6_dn10 + (locals.var_t2_dn10 * locals.var_vds_noswap)), (locals.var_t6_dn11 + (locals.var_t2_dn11 * locals.var_vds_noswap)), (locals.var_t6_dn13 + (locals.var_t2_dn13 * locals.var_vds_noswap)), (locals.var_t6_dn14 + (locals.var_t2_dn14 * locals.var_vds_noswap)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28390_e48733;
        locals.var_t6_dn0 = assign28390_e48733_d_n0;
        locals.var_t6_dn2 = assign28390_e48733_d_n2;
        locals.var_t6_dn3 = assign28390_e48733_d_n3;
        locals.var_t6_dn4 = assign28390_e48733_d_n4;
        locals.var_t6_dn5 = assign28390_e48733_d_n5;
        locals.var_t6_dn6 = assign28390_e48733_d_n6;
        locals.var_t6_dn7 = assign28390_e48733_d_n7;
        locals.var_t6_dn8 = assign28390_e48733_d_n8;
        locals.var_t6_dn9 = assign28390_e48733_d_n9;
        locals.var_t6_dn10 = assign28390_e48733_d_n10;
        locals.var_t6_dn11 = assign28390_e48733_d_n11;
        locals.var_t6_dn13 = assign28390_e48733_d_n13;
        locals.var_t6_dn14 = assign28390_e48733_d_n14;
        locals.var_t6_rv = 0.0;

        let assign28400_e48756: f64 = if (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        locals.var_guard459 = assign28400_e48756;
        locals.var_guard459_rv = 0.0;

        let (assign28410_e48835, assign28410_e48835_d_n4,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) {
        let assign28410_e48764: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign28410_e48765: f64 = (1.0 + assign28410_e48764);
        let assign28410_e48767: f64 = (assign28410_e48765 - 1e-6);
        let assign28410_e48769: f64 = (-10000.0);
        let assign28410_e48771: f64 = (assign28410_e48769 * 0.001);
        let (assign28410_e48832, assign28410_e48832_d_n4,) = {
            if (!(assign28410_e48767 < assign28410_e48771)) {
                let assign28410_e48778: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28410_e48779: f64 = (1.0 + assign28410_e48778);
                let assign28410_e48781: f64 = (assign28410_e48779 - 1e-6);
                let assign28410_e48785: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28410_e48786: f64 = (1.0 + assign28410_e48785);
                let assign28410_e48788: f64 = (assign28410_e48786 - 1e-6);
                let assign28410_e48792: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28410_e48793: f64 = (1.0 + assign28410_e48792);
                let assign28410_e48795: f64 = (assign28410_e48793 - 1e-6);
                let assign28410_e48796: f64 = (assign28410_e48788 * assign28410_e48795);
                let assign28410_e48799: f64 = (4.0 * 0.001);
                let assign28410_e48801: f64 = (assign28410_e48799 * 0.001);
                let assign28410_e48802: f64 = (assign28410_e48796 + assign28410_e48801);
                let assign28410_e48803: f64 = (assign28410_e48802).sqrt();
                let assign28410_e48804: f64 = (assign28410_e48781 + assign28410_e48803);
                let assign28410_e48805: f64 = (0.5 * assign28410_e48804);
                (assign28410_e48805, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign28410_e48795) + (assign28410_e48788 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign28410_e48803)))),)
            } else {
                let assign28410_e48809: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28410_e48810: f64 = (1.0 + assign28410_e48809);
                let assign28410_e48812: f64 = (assign28410_e48810 - 1e-6);
                let assign28410_e48814: f64 = (-10000.0);
                let assign28410_e48816: f64 = (assign28410_e48814 * 0.001);
                let (assign28410_e48831, assign28410_e48831_d_n4,) = {
                    if (assign28410_e48812 < assign28410_e48816) {
                        let assign28410_e48819: f64 = (-0.001);
                        let assign28410_e48821: f64 = (assign28410_e48819 * 0.001);
                        let assign28410_e48825: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                        let assign28410_e48826: f64 = (1.0 + assign28410_e48825);
                        let assign28410_e48828: f64 = (assign28410_e48826 - 1e-6);
                        let assign28410_e48829: f64 = (assign28410_e48821 / assign28410_e48828);
                        (assign28410_e48829, (-((assign28410_e48821 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign28410_e48828 * assign28410_e48828))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign28410_e48831, assign28410_e48831_d_n4,)
            }
        };
        let assign28410_e48833: f64 = (locals.var_bgidlb_i * assign28410_e48832);
        (assign28410_e48833, (locals.var_bgidlb_i * assign28410_e48832_d_n4),)
    } else {
        (locals.var_bgidlb_t, locals.var_bgidlb_t_dn4,)
    }
};
        locals.var_bgidlb_t = assign28410_e48835;
        locals.var_bgidlb_t_dn4 = assign28410_e48835_d_n4;
        locals.var_bgidlb_t_rv = 0.0;

        let assign28420_e48842: f64 = if ((locals.var_agidlb_i <= 0.0) || (locals.var_bgidlb_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard460 = assign28420_e48842;
        locals.var_guard460_rv = 0.0;

        let (assign28430_e48850, assign28430_e48850_d_n0, assign28430_e48850_d_n2, assign28430_e48850_d_n3, assign28430_e48850_d_n4, assign28430_e48850_d_n5, assign28430_e48850_d_n6, assign28430_e48850_d_n7, assign28430_e48850_d_n8, assign28430_e48850_d_n9, assign28430_e48850_d_n10, assign28430_e48850_d_n11, assign28430_e48850_d_n13, assign28430_e48850_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign28430_e48850;
        locals.var_t7_dn0 = assign28430_e48850_d_n0;
        locals.var_t7_dn2 = assign28430_e48850_d_n2;
        locals.var_t7_dn3 = assign28430_e48850_d_n3;
        locals.var_t7_dn4 = assign28430_e48850_d_n4;
        locals.var_t7_dn5 = assign28430_e48850_d_n5;
        locals.var_t7_dn6 = assign28430_e48850_d_n6;
        locals.var_t7_dn7 = assign28430_e48850_d_n7;
        locals.var_t7_dn8 = assign28430_e48850_d_n8;
        locals.var_t7_dn9 = assign28430_e48850_d_n9;
        locals.var_t7_dn10 = assign28430_e48850_d_n10;
        locals.var_t7_dn11 = assign28430_e48850_d_n11;
        locals.var_t7_dn13 = assign28430_e48850_d_n13;
        locals.var_t7_dn14 = assign28430_e48850_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign28440_e48866, assign28440_e48866_d_n0, assign28440_e48866_d_n2, assign28440_e48866_d_n3, assign28440_e48866_d_n4, assign28440_e48866_d_n5, assign28440_e48866_d_n6, assign28440_e48866_d_n7, assign28440_e48866_d_n8, assign28440_e48866_d_n9, assign28440_e48866_d_n10, assign28440_e48866_d_n11, assign28440_e48866_d_n13, assign28440_e48866_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28440_e48858: f64 = (-locals.var_vgd_noswap);
        let assign28440_e48860: f64 = (assign28440_e48858 - locals.var_egidlb_i);
        let assign28440_e48862: f64 = (assign28440_e48860 + locals.var_vfbsd_v);
        let assign28440_e48864: f64 = (assign28440_e48862 / locals.var_t0);
        (assign28440_e48864, (((locals.var_vfbsd_v_dn0 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn2 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn3 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn4 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgd_noswap_dn5) + locals.var_vfbsd_v_dn5) * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn6 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn7 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn8 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn9 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn10 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgd_noswap_dn11) + locals.var_vfbsd_v_dn11) * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn13 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn14 * locals.var_t0) - (assign28440_e48862 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28440_e48866;
        locals.var_t1_dn0 = assign28440_e48866_d_n0;
        locals.var_t1_dn2 = assign28440_e48866_d_n2;
        locals.var_t1_dn3 = assign28440_e48866_d_n3;
        locals.var_t1_dn4 = assign28440_e48866_d_n4;
        locals.var_t1_dn5 = assign28440_e48866_d_n5;
        locals.var_t1_dn6 = assign28440_e48866_d_n6;
        locals.var_t1_dn7 = assign28440_e48866_d_n7;
        locals.var_t1_dn8 = assign28440_e48866_d_n8;
        locals.var_t1_dn9 = assign28440_e48866_d_n9;
        locals.var_t1_dn10 = assign28440_e48866_d_n10;
        locals.var_t1_dn11 = assign28440_e48866_d_n11;
        locals.var_t1_dn13 = assign28440_e48866_d_n13;
        locals.var_t1_dn14 = assign28440_e48866_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28450_e48910, assign28450_e48910_d_n0, assign28450_e48910_d_n2, assign28450_e48910_d_n3, assign28450_e48910_d_n4, assign28450_e48910_d_n5, assign28450_e48910_d_n6, assign28450_e48910_d_n7, assign28450_e48910_d_n8, assign28450_e48910_d_n9, assign28450_e48910_d_n10, assign28450_e48910_d_n11, assign28450_e48910_d_n13, assign28450_e48910_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28450_e48875: f64 = (-10000.0);
        let assign28450_e48877: f64 = (assign28450_e48875 * 0.01);
        let (assign28450_e48908, assign28450_e48908_d_n0, assign28450_e48908_d_n2, assign28450_e48908_d_n3, assign28450_e48908_d_n4, assign28450_e48908_d_n5, assign28450_e48908_d_n6, assign28450_e48908_d_n7, assign28450_e48908_d_n8, assign28450_e48908_d_n9, assign28450_e48908_d_n10, assign28450_e48908_d_n11, assign28450_e48908_d_n13, assign28450_e48908_d_n14,) = {
            if (!(locals.var_t1 < assign28450_e48877)) {
                let assign28450_e48884: f64 = (locals.var_t1 * locals.var_t1);
                let assign28450_e48887: f64 = (4.0 * 0.01);
                let assign28450_e48889: f64 = (assign28450_e48887 * 0.01);
                let assign28450_e48890: f64 = (assign28450_e48884 + assign28450_e48889);
                let assign28450_e48891: f64 = (assign28450_e48890).sqrt();
                let assign28450_e48892: f64 = (locals.var_t1 + assign28450_e48891);
                let assign28450_e48893: f64 = (0.5 * assign28450_e48892);
                (assign28450_e48893, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign28450_e48891)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign28450_e48891)))),)
            } else {
                let assign28450_e48896: f64 = (-10000.0);
                let assign28450_e48898: f64 = (assign28450_e48896 * 0.01);
                let (assign28450_e48907, assign28450_e48907_d_n0, assign28450_e48907_d_n2, assign28450_e48907_d_n3, assign28450_e48907_d_n4, assign28450_e48907_d_n5, assign28450_e48907_d_n6, assign28450_e48907_d_n7, assign28450_e48907_d_n8, assign28450_e48907_d_n9, assign28450_e48907_d_n10, assign28450_e48907_d_n11, assign28450_e48907_d_n13, assign28450_e48907_d_n14,) = {
                    if (locals.var_t1 < assign28450_e48898) {
                        let assign28450_e48901: f64 = (-0.01);
                        let assign28450_e48903: f64 = (assign28450_e48901 * 0.01);
                        let assign28450_e48905: f64 = (assign28450_e48903 / locals.var_t1);
                        (assign28450_e48905, (-((assign28450_e48903 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign28450_e48903 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28450_e48907, assign28450_e48907_d_n0, assign28450_e48907_d_n2, assign28450_e48907_d_n3, assign28450_e48907_d_n4, assign28450_e48907_d_n5, assign28450_e48907_d_n6, assign28450_e48907_d_n7, assign28450_e48907_d_n8, assign28450_e48907_d_n9, assign28450_e48907_d_n10, assign28450_e48907_d_n11, assign28450_e48907_d_n13, assign28450_e48907_d_n14,)
            }
        };
        (assign28450_e48908, assign28450_e48908_d_n0, assign28450_e48908_d_n2, assign28450_e48908_d_n3, assign28450_e48908_d_n4, assign28450_e48908_d_n5, assign28450_e48908_d_n6, assign28450_e48908_d_n7, assign28450_e48908_d_n8, assign28450_e48908_d_n9, assign28450_e48908_d_n10, assign28450_e48908_d_n11, assign28450_e48908_d_n13, assign28450_e48908_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28450_e48910;
        locals.var_t1_dn0 = assign28450_e48910_d_n0;
        locals.var_t1_dn2 = assign28450_e48910_d_n2;
        locals.var_t1_dn3 = assign28450_e48910_d_n3;
        locals.var_t1_dn4 = assign28450_e48910_d_n4;
        locals.var_t1_dn5 = assign28450_e48910_d_n5;
        locals.var_t1_dn6 = assign28450_e48910_d_n6;
        locals.var_t1_dn7 = assign28450_e48910_d_n7;
        locals.var_t1_dn8 = assign28450_e48910_d_n8;
        locals.var_t1_dn9 = assign28450_e48910_d_n9;
        locals.var_t1_dn10 = assign28450_e48910_d_n10;
        locals.var_t1_dn11 = assign28450_e48910_d_n11;
        locals.var_t1_dn13 = assign28450_e48910_d_n13;
        locals.var_t1_dn14 = assign28450_e48910_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_115(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28460_e48923, assign28460_e48923_d_n0, assign28460_e48923_d_n2, assign28460_e48923_d_n3, assign28460_e48923_d_n4, assign28460_e48923_d_n5, assign28460_e48923_d_n6, assign28460_e48923_d_n7, assign28460_e48923_d_n8, assign28460_e48923_d_n9, assign28460_e48923_d_n10, assign28460_e48923_d_n11, assign28460_e48923_d_n13, assign28460_e48923_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28460_e48920: f64 = (locals.var_t1 + 0.001);
        let assign28460_e48921: f64 = (locals.var_bgidlb_t / assign28460_e48920);
        (assign28460_e48921, (-((locals.var_bgidlb_t * locals.var_t1_dn0) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn2) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn3) / (assign28460_e48920 * assign28460_e48920))), (((locals.var_bgidlb_t_dn4 * assign28460_e48920) - (locals.var_bgidlb_t * locals.var_t1_dn4)) / (assign28460_e48920 * assign28460_e48920)), (-((locals.var_bgidlb_t * locals.var_t1_dn5) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn6) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn7) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn8) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn9) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn10) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn11) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn13) / (assign28460_e48920 * assign28460_e48920))), (-((locals.var_bgidlb_t * locals.var_t1_dn14) / (assign28460_e48920 * assign28460_e48920))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28460_e48923;
        locals.var_t2_dn0 = assign28460_e48923_d_n0;
        locals.var_t2_dn2 = assign28460_e48923_d_n2;
        locals.var_t2_dn3 = assign28460_e48923_d_n3;
        locals.var_t2_dn4 = assign28460_e48923_d_n4;
        locals.var_t2_dn5 = assign28460_e48923_d_n5;
        locals.var_t2_dn6 = assign28460_e48923_d_n6;
        locals.var_t2_dn7 = assign28460_e48923_d_n7;
        locals.var_t2_dn8 = assign28460_e48923_d_n8;
        locals.var_t2_dn9 = assign28460_e48923_d_n9;
        locals.var_t2_dn10 = assign28460_e48923_d_n10;
        locals.var_t2_dn11 = assign28460_e48923_d_n11;
        locals.var_t2_dn13 = assign28460_e48923_d_n13;
        locals.var_t2_dn14 = assign28460_e48923_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28470_e48934, assign28470_e48934_d_n0, assign28470_e48934_d_n2, assign28470_e48934_d_n3, assign28470_e48934_d_n4, assign28470_e48934_d_n5, assign28470_e48934_d_n6, assign28470_e48934_d_n7, assign28470_e48934_d_n8, assign28470_e48934_d_n9, assign28470_e48934_d_n10, assign28470_e48934_d_n11, assign28470_e48934_d_n13, assign28470_e48934_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28470_e48932: f64 = (locals.var_t1).powf(locals.var_pgidlb_i);
        (assign28470_e48932, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn0)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn2)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn3)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn4)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn5)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn6)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn7)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn8)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn9)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn10)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn11)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn13)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgidlb_i) as f64).is_finite() && ((locals.var_pgidlb_i) as f64).fract() == 0.0 { if locals.var_pgidlb_i == 0.0 { 0.0 } else { (locals.var_pgidlb_i * ((locals.var_t1).powf(locals.var_pgidlb_i - 1.0) * locals.var_t1_dn14)) } } else { (assign28470_e48932 * (locals.var_pgidlb_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign28470_e48934;
        locals.var_t3_dn0 = assign28470_e48934_d_n0;
        locals.var_t3_dn2 = assign28470_e48934_d_n2;
        locals.var_t3_dn3 = assign28470_e48934_d_n3;
        locals.var_t3_dn4 = assign28470_e48934_d_n4;
        locals.var_t3_dn5 = assign28470_e48934_d_n5;
        locals.var_t3_dn6 = assign28470_e48934_d_n6;
        locals.var_t3_dn7 = assign28470_e48934_d_n7;
        locals.var_t3_dn8 = assign28470_e48934_d_n8;
        locals.var_t3_dn9 = assign28470_e48934_d_n9;
        locals.var_t3_dn10 = assign28470_e48934_d_n10;
        locals.var_t3_dn11 = assign28470_e48934_d_n11;
        locals.var_t3_dn13 = assign28470_e48934_d_n13;
        locals.var_t3_dn14 = assign28470_e48934_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign28480_e48948, assign28480_e48948_d_n0, assign28480_e48948_d_n2, assign28480_e48948_d_n3, assign28480_e48948_d_n4, assign28480_e48948_d_n5, assign28480_e48948_d_n6, assign28480_e48948_d_n7, assign28480_e48948_d_n8, assign28480_e48948_d_n9, assign28480_e48948_d_n10, assign28480_e48948_d_n11, assign28480_e48948_d_n13, assign28480_e48948_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28480_e48942: f64 = (-locals.var_ved_jct);
        let assign28480_e48944: f64 = (assign28480_e48942 * locals.var_ved_jct);
        let assign28480_e48946: f64 = (assign28480_e48944 * locals.var_ved_jct);
        (assign28480_e48946, 0.0, 0.0, (((((-locals.var_ved_jct_dn3) * locals.var_ved_jct) + (assign28480_e48942 * locals.var_ved_jct_dn3)) * locals.var_ved_jct) + (assign28480_e48944 * locals.var_ved_jct_dn3)), 0.0, (((((-locals.var_ved_jct_dn5) * locals.var_ved_jct) + (assign28480_e48942 * locals.var_ved_jct_dn5)) * locals.var_ved_jct) + (assign28480_e48944 * locals.var_ved_jct_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28480_e48948;
        locals.var_t4_dn0 = assign28480_e48948_d_n0;
        locals.var_t4_dn2 = assign28480_e48948_d_n2;
        locals.var_t4_dn3 = assign28480_e48948_d_n3;
        locals.var_t4_dn4 = assign28480_e48948_d_n4;
        locals.var_t4_dn5 = assign28480_e48948_d_n5;
        locals.var_t4_dn6 = assign28480_e48948_d_n6;
        locals.var_t4_dn7 = assign28480_e48948_d_n7;
        locals.var_t4_dn8 = assign28480_e48948_d_n8;
        locals.var_t4_dn9 = assign28480_e48948_d_n9;
        locals.var_t4_dn10 = assign28480_e48948_d_n10;
        locals.var_t4_dn11 = assign28480_e48948_d_n11;
        locals.var_t4_dn13 = assign28480_e48948_d_n13;
        locals.var_t4_dn14 = assign28480_e48948_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign28490_e48962, assign28490_e48962_d_n0, assign28490_e48962_d_n2, assign28490_e48962_d_n3, assign28490_e48962_d_n4, assign28490_e48962_d_n5, assign28490_e48962_d_n6, assign28490_e48962_d_n7, assign28490_e48962_d_n8, assign28490_e48962_d_n9, assign28490_e48962_d_n10, assign28490_e48962_d_n11, assign28490_e48962_d_n13, assign28490_e48962_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28490_e48957: f64 = (locals.var_t4).abs();
        let assign28490_e48958: f64 = (locals.var_cgidlb_i + assign28490_e48957);
        let assign28490_e48960: f64 = (assign28490_e48958 + 1e-5);
        (assign28490_e48960, if locals.var_t4 >= 0.0 { locals.var_t4_dn0 } else { (-locals.var_t4_dn0) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn2 } else { (-locals.var_t4_dn2) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn3 } else { (-locals.var_t4_dn3) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn4 } else { (-locals.var_t4_dn4) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn5 } else { (-locals.var_t4_dn5) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn6 } else { (-locals.var_t4_dn6) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn7 } else { (-locals.var_t4_dn7) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn8 } else { (-locals.var_t4_dn8) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn9 } else { (-locals.var_t4_dn9) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn10 } else { (-locals.var_t4_dn10) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn11 } else { (-locals.var_t4_dn11) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn13 } else { (-locals.var_t4_dn13) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn14 } else { (-locals.var_t4_dn14) },)
    } else {
        (locals.var_t4a, locals.var_t4a_dn0, locals.var_t4a_dn2, locals.var_t4a_dn3, locals.var_t4a_dn4, locals.var_t4a_dn5, locals.var_t4a_dn6, locals.var_t4a_dn7, locals.var_t4a_dn8, locals.var_t4a_dn9, locals.var_t4a_dn10, locals.var_t4a_dn11, locals.var_t4a_dn13, locals.var_t4a_dn14,)
    }
};
        locals.var_t4a = assign28490_e48962;
        locals.var_t4a_dn0 = assign28490_e48962_d_n0;
        locals.var_t4a_dn2 = assign28490_e48962_d_n2;
        locals.var_t4a_dn3 = assign28490_e48962_d_n3;
        locals.var_t4a_dn4 = assign28490_e48962_d_n4;
        locals.var_t4a_dn5 = assign28490_e48962_d_n5;
        locals.var_t4a_dn6 = assign28490_e48962_d_n6;
        locals.var_t4a_dn7 = assign28490_e48962_d_n7;
        locals.var_t4a_dn8 = assign28490_e48962_d_n8;
        locals.var_t4a_dn9 = assign28490_e48962_d_n9;
        locals.var_t4a_dn10 = assign28490_e48962_d_n10;
        locals.var_t4a_dn11 = assign28490_e48962_d_n11;
        locals.var_t4a_dn13 = assign28490_e48962_d_n13;
        locals.var_t4a_dn14 = assign28490_e48962_d_n14;
        locals.var_t4a_rv = 0.0;

        let (assign28500_e49020, assign28500_e49020_d_n0, assign28500_e49020_d_n2, assign28500_e49020_d_n3, assign28500_e49020_d_n4, assign28500_e49020_d_n5, assign28500_e49020_d_n6, assign28500_e49020_d_n7, assign28500_e49020_d_n8, assign28500_e49020_d_n9, assign28500_e49020_d_n10, assign28500_e49020_d_n11, assign28500_e49020_d_n13, assign28500_e49020_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28500_e48971: f64 = (locals.var_t4 / locals.var_t4a);
        let assign28500_e48973: f64 = (-10000.0);
        let assign28500_e48975: f64 = (assign28500_e48973 * 1e-6);
        let (assign28500_e49016, assign28500_e49016_d_n0, assign28500_e49016_d_n2, assign28500_e49016_d_n3, assign28500_e49016_d_n4, assign28500_e49016_d_n5, assign28500_e49016_d_n6, assign28500_e49016_d_n7, assign28500_e49016_d_n8, assign28500_e49016_d_n9, assign28500_e49016_d_n10, assign28500_e49016_d_n11, assign28500_e49016_d_n13, assign28500_e49016_d_n14,) = {
            if (!(assign28500_e48971 < assign28500_e48975)) {
                let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4a;
                let assign28500_e48981: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28500_e48984: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28500_e48987: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28500_e48988: f64 = (assign28500_e48984 * assign28500_e48987);
                let assign28500_e48991: f64 = (4.0 * 1e-6);
                let assign28500_e48993: f64 = (assign28500_e48991 * 1e-6);
                let assign28500_e48994: f64 = (assign28500_e48988 + assign28500_e48993);
                let assign28500_e48995: f64 = (assign28500_e48994).sqrt();
                let assign28500_e48996: f64 = (assign28500_e48981 + assign28500_e48995);
                let assign28500_e48997: f64 = (0.5 * assign28500_e48996);
                (assign28500_e48997, (0.5 * ((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))), (0.5 * ((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) * assign28500_e48987) + (assign28500_e48984 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28500_e48995)))),)
            } else {
                let assign28500_e49000: f64 = (locals.var_t4 / locals.var_t4a);
                let assign28500_e49002: f64 = (-10000.0);
                let assign28500_e49004: f64 = (assign28500_e49002 * 1e-6);
                let (assign28500_e49015, assign28500_e49015_d_n0, assign28500_e49015_d_n2, assign28500_e49015_d_n3, assign28500_e49015_d_n4, assign28500_e49015_d_n5, assign28500_e49015_d_n6, assign28500_e49015_d_n7, assign28500_e49015_d_n8, assign28500_e49015_d_n9, assign28500_e49015_d_n10, assign28500_e49015_d_n11, assign28500_e49015_d_n13, assign28500_e49015_d_n14,) = {
                    if (assign28500_e49000 < assign28500_e49004) {
                        let assign28500_e49007: f64 = (-1e-6);
                        let assign28500_e49009: f64 = (assign28500_e49007 * 1e-6);
                        let assign28500_e49012: f64 = (locals.var_t4 / locals.var_t4a);
                        let assign28500_e49013: f64 = (assign28500_e49009 / assign28500_e49012);
                        (assign28500_e49013, (-((assign28500_e49009 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))), (-((assign28500_e49009 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a))) / (assign28500_e49012 * assign28500_e49012))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28500_e49015, assign28500_e49015_d_n0, assign28500_e49015_d_n2, assign28500_e49015_d_n3, assign28500_e49015_d_n4, assign28500_e49015_d_n5, assign28500_e49015_d_n6, assign28500_e49015_d_n7, assign28500_e49015_d_n8, assign28500_e49015_d_n9, assign28500_e49015_d_n10, assign28500_e49015_d_n11, assign28500_e49015_d_n13, assign28500_e49015_d_n14,)
            }
        };
        let assign28500_e49018: f64 = (assign28500_e49016 - 1e-6);
        (assign28500_e49018, assign28500_e49016_d_n0, assign28500_e49016_d_n2, assign28500_e49016_d_n3, assign28500_e49016_d_n4, assign28500_e49016_d_n5, assign28500_e49016_d_n6, assign28500_e49016_d_n7, assign28500_e49016_d_n8, assign28500_e49016_d_n9, assign28500_e49016_d_n10, assign28500_e49016_d_n11, assign28500_e49016_d_n13, assign28500_e49016_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign28500_e49020;
        locals.var_t5_dn0 = assign28500_e49020_d_n0;
        locals.var_t5_dn2 = assign28500_e49020_d_n2;
        locals.var_t5_dn3 = assign28500_e49020_d_n3;
        locals.var_t5_dn4 = assign28500_e49020_d_n4;
        locals.var_t5_dn5 = assign28500_e49020_d_n5;
        locals.var_t5_dn6 = assign28500_e49020_d_n6;
        locals.var_t5_dn7 = assign28500_e49020_d_n7;
        locals.var_t5_dn8 = assign28500_e49020_d_n8;
        locals.var_t5_dn9 = assign28500_e49020_d_n9;
        locals.var_t5_dn10 = assign28500_e49020_d_n10;
        locals.var_t5_dn11 = assign28500_e49020_d_n11;
        locals.var_t5_dn13 = assign28500_e49020_d_n13;
        locals.var_t5_dn14 = assign28500_e49020_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign28510_e49039, assign28510_e49039_d_n0, assign28510_e49039_d_n2, assign28510_e49039_d_n3, assign28510_e49039_d_n4, assign28510_e49039_d_n5, assign28510_e49039_d_n6, assign28510_e49039_d_n7, assign28510_e49039_d_n8, assign28510_e49039_d_n9, assign28510_e49039_d_n10, assign28510_e49039_d_n11, assign28510_e49039_d_n13, assign28510_e49039_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard459 != 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign28510_e49029: f64 = (locals.var_agidlb_i * locals.var_weffb);
        let assign28510_e49031: f64 = (assign28510_e49029 * locals.var_t3);
        let assign28510_e49033: f64 = (-locals.var_t2);
        let assign28510_e49034: f64 = { let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28510_e49035: f64 = (assign28510_e49031 * assign28510_e49034);
        let assign28510_e49037: f64 = (assign28510_e49035 * locals.var_t5);
        (assign28510_e49037, (((((assign28510_e49029 * locals.var_t3_dn0) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn0)), (((((assign28510_e49029 * locals.var_t3_dn2) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn2)), (((((assign28510_e49029 * locals.var_t3_dn3) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn3)), (((((assign28510_e49029 * locals.var_t3_dn4) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn4)), (((((assign28510_e49029 * locals.var_t3_dn5) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn5)), (((((assign28510_e49029 * locals.var_t3_dn6) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn6)), (((((assign28510_e49029 * locals.var_t3_dn7) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn7)), (((((assign28510_e49029 * locals.var_t3_dn8) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn8)), (((((assign28510_e49029 * locals.var_t3_dn9) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn9)), (((((assign28510_e49029 * locals.var_t3_dn10) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn10)), (((((assign28510_e49029 * locals.var_t3_dn11) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn11)), (((((assign28510_e49029 * locals.var_t3_dn13) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn13)), (((((assign28510_e49029 * locals.var_t3_dn14) * assign28510_e49034) + (assign28510_e49031 * ({ let limited_exp_arg = assign28510_e49033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_t5) + (assign28510_e49035 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign28510_e49039;
        locals.var_t7_dn0 = assign28510_e49039_d_n0;
        locals.var_t7_dn2 = assign28510_e49039_d_n2;
        locals.var_t7_dn3 = assign28510_e49039_d_n3;
        locals.var_t7_dn4 = assign28510_e49039_d_n4;
        locals.var_t7_dn5 = assign28510_e49039_d_n5;
        locals.var_t7_dn6 = assign28510_e49039_d_n6;
        locals.var_t7_dn7 = assign28510_e49039_d_n7;
        locals.var_t7_dn8 = assign28510_e49039_d_n8;
        locals.var_t7_dn9 = assign28510_e49039_d_n9;
        locals.var_t7_dn10 = assign28510_e49039_d_n10;
        locals.var_t7_dn11 = assign28510_e49039_d_n11;
        locals.var_t7_dn13 = assign28510_e49039_d_n13;
        locals.var_t7_dn14 = assign28510_e49039_d_n14;
        locals.var_t7_rv = 0.0;

        let assign28570_e49075: f64 = if ((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard462 = assign28570_e49075;
        locals.var_guard462_rv = 0.0;

        let (assign28580_e49081, assign28580_e49081_d_n0, assign28580_e49081_d_n2, assign28580_e49081_d_n3, assign28580_e49081_d_n4, assign28580_e49081_d_n5, assign28580_e49081_d_n6, assign28580_e49081_d_n7, assign28580_e49081_d_n8, assign28580_e49081_d_n9, assign28580_e49081_d_n10, assign28580_e49081_d_n11, assign28580_e49081_d_n13, assign28580_e49081_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard462 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28580_e49081;
        locals.var_t6_dn0 = assign28580_e49081_d_n0;
        locals.var_t6_dn2 = assign28580_e49081_d_n2;
        locals.var_t6_dn3 = assign28580_e49081_d_n3;
        locals.var_t6_dn4 = assign28580_e49081_d_n4;
        locals.var_t6_dn5 = assign28580_e49081_d_n5;
        locals.var_t6_dn6 = assign28580_e49081_d_n6;
        locals.var_t6_dn7 = assign28580_e49081_d_n7;
        locals.var_t6_dn8 = assign28580_e49081_d_n8;
        locals.var_t6_dn9 = assign28580_e49081_d_n9;
        locals.var_t6_dn10 = assign28580_e49081_d_n10;
        locals.var_t6_dn11 = assign28580_e49081_d_n11;
        locals.var_t6_dn13 = assign28580_e49081_d_n13;
        locals.var_t6_dn14 = assign28580_e49081_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign28590_e49095, assign28590_e49095_d_n0, assign28590_e49095_d_n2, assign28590_e49095_d_n3, assign28590_e49095_d_n4, assign28590_e49095_d_n5, assign28590_e49095_d_n6, assign28590_e49095_d_n7, assign28590_e49095_d_n8, assign28590_e49095_d_n9, assign28590_e49095_d_n10, assign28590_e49095_d_n11, assign28590_e49095_d_n13, assign28590_e49095_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) {
        let assign28590_e49087: f64 = (-locals.var_vgs_noswap);
        let assign28590_e49089: f64 = (assign28590_e49087 - locals.var_egisl_i);
        let assign28590_e49091: f64 = (assign28590_e49089 + locals.var_vfbsd_v);
        let assign28590_e49093: f64 = (assign28590_e49091 / locals.var_t0);
        (assign28590_e49093, (((locals.var_vfbsd_v_dn0 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn2 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn3 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn4 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn5 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgs_noswap_dn6) + locals.var_vfbsd_v_dn6) * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn7 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn8 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn9 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn10 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgs_noswap_dn11) + locals.var_vfbsd_v_dn11) * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn13 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn14 * locals.var_t0) - (assign28590_e49091 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28590_e49095;
        locals.var_t1_dn0 = assign28590_e49095_d_n0;
        locals.var_t1_dn2 = assign28590_e49095_d_n2;
        locals.var_t1_dn3 = assign28590_e49095_d_n3;
        locals.var_t1_dn4 = assign28590_e49095_d_n4;
        locals.var_t1_dn5 = assign28590_e49095_d_n5;
        locals.var_t1_dn6 = assign28590_e49095_d_n6;
        locals.var_t1_dn7 = assign28590_e49095_d_n7;
        locals.var_t1_dn8 = assign28590_e49095_d_n8;
        locals.var_t1_dn9 = assign28590_e49095_d_n9;
        locals.var_t1_dn10 = assign28590_e49095_d_n10;
        locals.var_t1_dn11 = assign28590_e49095_d_n11;
        locals.var_t1_dn13 = assign28590_e49095_d_n13;
        locals.var_t1_dn14 = assign28590_e49095_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28600_e49137, assign28600_e49137_d_n0, assign28600_e49137_d_n2, assign28600_e49137_d_n3, assign28600_e49137_d_n4, assign28600_e49137_d_n5, assign28600_e49137_d_n6, assign28600_e49137_d_n7, assign28600_e49137_d_n8, assign28600_e49137_d_n9, assign28600_e49137_d_n10, assign28600_e49137_d_n11, assign28600_e49137_d_n13, assign28600_e49137_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) {
        let assign28600_e49102: f64 = (-10000.0);
        let assign28600_e49104: f64 = (assign28600_e49102 * 0.01);
        let (assign28600_e49135, assign28600_e49135_d_n0, assign28600_e49135_d_n2, assign28600_e49135_d_n3, assign28600_e49135_d_n4, assign28600_e49135_d_n5, assign28600_e49135_d_n6, assign28600_e49135_d_n7, assign28600_e49135_d_n8, assign28600_e49135_d_n9, assign28600_e49135_d_n10, assign28600_e49135_d_n11, assign28600_e49135_d_n13, assign28600_e49135_d_n14,) = {
            if (!(locals.var_t1 < assign28600_e49104)) {
                let assign28600_e49111: f64 = (locals.var_t1 * locals.var_t1);
                let assign28600_e49114: f64 = (4.0 * 0.01);
                let assign28600_e49116: f64 = (assign28600_e49114 * 0.01);
                let assign28600_e49117: f64 = (assign28600_e49111 + assign28600_e49116);
                let assign28600_e49118: f64 = (assign28600_e49117).sqrt();
                let assign28600_e49119: f64 = (locals.var_t1 + assign28600_e49118);
                let assign28600_e49120: f64 = (0.5 * assign28600_e49119);
                (assign28600_e49120, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign28600_e49118)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign28600_e49118)))),)
            } else {
                let assign28600_e49123: f64 = (-10000.0);
                let assign28600_e49125: f64 = (assign28600_e49123 * 0.01);
                let (assign28600_e49134, assign28600_e49134_d_n0, assign28600_e49134_d_n2, assign28600_e49134_d_n3, assign28600_e49134_d_n4, assign28600_e49134_d_n5, assign28600_e49134_d_n6, assign28600_e49134_d_n7, assign28600_e49134_d_n8, assign28600_e49134_d_n9, assign28600_e49134_d_n10, assign28600_e49134_d_n11, assign28600_e49134_d_n13, assign28600_e49134_d_n14,) = {
                    if (locals.var_t1 < assign28600_e49125) {
                        let assign28600_e49128: f64 = (-0.01);
                        let assign28600_e49130: f64 = (assign28600_e49128 * 0.01);
                        let assign28600_e49132: f64 = (assign28600_e49130 / locals.var_t1);
                        (assign28600_e49132, (-((assign28600_e49130 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign28600_e49130 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28600_e49134, assign28600_e49134_d_n0, assign28600_e49134_d_n2, assign28600_e49134_d_n3, assign28600_e49134_d_n4, assign28600_e49134_d_n5, assign28600_e49134_d_n6, assign28600_e49134_d_n7, assign28600_e49134_d_n8, assign28600_e49134_d_n9, assign28600_e49134_d_n10, assign28600_e49134_d_n11, assign28600_e49134_d_n13, assign28600_e49134_d_n14,)
            }
        };
        (assign28600_e49135, assign28600_e49135_d_n0, assign28600_e49135_d_n2, assign28600_e49135_d_n3, assign28600_e49135_d_n4, assign28600_e49135_d_n5, assign28600_e49135_d_n6, assign28600_e49135_d_n7, assign28600_e49135_d_n8, assign28600_e49135_d_n9, assign28600_e49135_d_n10, assign28600_e49135_d_n11, assign28600_e49135_d_n13, assign28600_e49135_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28600_e49137;
        locals.var_t1_dn0 = assign28600_e49137_d_n0;
        locals.var_t1_dn2 = assign28600_e49137_d_n2;
        locals.var_t1_dn3 = assign28600_e49137_d_n3;
        locals.var_t1_dn4 = assign28600_e49137_d_n4;
        locals.var_t1_dn5 = assign28600_e49137_d_n5;
        locals.var_t1_dn6 = assign28600_e49137_d_n6;
        locals.var_t1_dn7 = assign28600_e49137_d_n7;
        locals.var_t1_dn8 = assign28600_e49137_d_n8;
        locals.var_t1_dn9 = assign28600_e49137_d_n9;
        locals.var_t1_dn10 = assign28600_e49137_d_n10;
        locals.var_t1_dn11 = assign28600_e49137_d_n11;
        locals.var_t1_dn13 = assign28600_e49137_d_n13;
        locals.var_t1_dn14 = assign28600_e49137_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28610_e49148, assign28610_e49148_d_n0, assign28610_e49148_d_n2, assign28610_e49148_d_n3, assign28610_e49148_d_n4, assign28610_e49148_d_n5, assign28610_e49148_d_n6, assign28610_e49148_d_n7, assign28610_e49148_d_n8, assign28610_e49148_d_n9, assign28610_e49148_d_n10, assign28610_e49148_d_n11, assign28610_e49148_d_n13, assign28610_e49148_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) {
        let assign28610_e49145: f64 = (locals.var_t1 + 0.001);
        let assign28610_e49146: f64 = (locals.var_bgisl_t / assign28610_e49145);
        (assign28610_e49146, (-((locals.var_bgisl_t * locals.var_t1_dn0) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn2) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign28610_e49145 * assign28610_e49145))), (((locals.var_bgisl_t_dn4 * assign28610_e49145) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign28610_e49145 * assign28610_e49145)), (-((locals.var_bgisl_t * locals.var_t1_dn5) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn13) / (assign28610_e49145 * assign28610_e49145))), (-((locals.var_bgisl_t * locals.var_t1_dn14) / (assign28610_e49145 * assign28610_e49145))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28610_e49148;
        locals.var_t2_dn0 = assign28610_e49148_d_n0;
        locals.var_t2_dn2 = assign28610_e49148_d_n2;
        locals.var_t2_dn3 = assign28610_e49148_d_n3;
        locals.var_t2_dn4 = assign28610_e49148_d_n4;
        locals.var_t2_dn5 = assign28610_e49148_d_n5;
        locals.var_t2_dn6 = assign28610_e49148_d_n6;
        locals.var_t2_dn7 = assign28610_e49148_d_n7;
        locals.var_t2_dn8 = assign28610_e49148_d_n8;
        locals.var_t2_dn9 = assign28610_e49148_d_n9;
        locals.var_t2_dn10 = assign28610_e49148_d_n10;
        locals.var_t2_dn11 = assign28610_e49148_d_n11;
        locals.var_t2_dn13 = assign28610_e49148_d_n13;
        locals.var_t2_dn14 = assign28610_e49148_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28620_e49157, assign28620_e49157_d_n0, assign28620_e49157_d_n2, assign28620_e49157_d_n3, assign28620_e49157_d_n4, assign28620_e49157_d_n5, assign28620_e49157_d_n6, assign28620_e49157_d_n7, assign28620_e49157_d_n8, assign28620_e49157_d_n9, assign28620_e49157_d_n10, assign28620_e49157_d_n11, assign28620_e49157_d_n13, assign28620_e49157_d_n14,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) {
        let assign28620_e49155: f64 = (locals.var_t1).powf(locals.var_pgisl_i);
        (assign28620_e49155, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn0)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn2)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn3)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn4)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn5)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn6)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn7)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn8)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn9)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn10)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn11)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn13)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgisl_i) as f64).is_finite() && ((locals.var_pgisl_i) as f64).fract() == 0.0 { if locals.var_pgisl_i == 0.0 { 0.0 } else { (locals.var_pgisl_i * ((locals.var_t1).powf(locals.var_pgisl_i - 1.0) * locals.var_t1_dn14)) } } else { (assign28620_e49155 * (locals.var_pgisl_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign28620_e49157;
        locals.var_t3_dn0 = assign28620_e49157_d_n0;
        locals.var_t3_dn2 = assign28620_e49157_d_n2;
        locals.var_t3_dn3 = assign28620_e49157_d_n3;
        locals.var_t3_dn4 = assign28620_e49157_d_n4;
        locals.var_t3_dn5 = assign28620_e49157_d_n5;
        locals.var_t3_dn6 = assign28620_e49157_d_n6;
        locals.var_t3_dn7 = assign28620_e49157_d_n7;
        locals.var_t3_dn8 = assign28620_e49157_d_n8;
        locals.var_t3_dn9 = assign28620_e49157_d_n9;
        locals.var_t3_dn10 = assign28620_e49157_d_n10;
        locals.var_t3_dn11 = assign28620_e49157_d_n11;
        locals.var_t3_dn13 = assign28620_e49157_d_n13;
        locals.var_t3_dn14 = assign28620_e49157_d_n14;
        locals.var_t3_rv = 0.0;

        let assign28630_e49160: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign28630_e49160;
        locals.var_guard463_rv = 0.0;

        let (assign28640_e49174, assign28640_e49174_d_n0, assign28640_e49174_d_n2, assign28640_e49174_d_n3, assign28640_e49174_d_n4, assign28640_e49174_d_n5, assign28640_e49174_d_n6, assign28640_e49174_d_n7, assign28640_e49174_d_n8, assign28640_e49174_d_n9, assign28640_e49174_d_n10, assign28640_e49174_d_n11, assign28640_e49174_d_n13, assign28640_e49174_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) && (locals.var_guard463 != 0.0)) {
        let assign28640_e49168: f64 = (-locals.var_ves_jct);
        let assign28640_e49170: f64 = (assign28640_e49168 * locals.var_ves_jct);
        let assign28640_e49172: f64 = (assign28640_e49170 * locals.var_ves_jct);
        (assign28640_e49172, 0.0, 0.0, (((((-locals.var_ves_jct_dn3) * locals.var_ves_jct) + (assign28640_e49168 * locals.var_ves_jct_dn3)) * locals.var_ves_jct) + (assign28640_e49170 * locals.var_ves_jct_dn3)), 0.0, 0.0, (((((-locals.var_ves_jct_dn6) * locals.var_ves_jct) + (assign28640_e49168 * locals.var_ves_jct_dn6)) * locals.var_ves_jct) + (assign28640_e49170 * locals.var_ves_jct_dn6)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28640_e49174;
        locals.var_t4_dn0 = assign28640_e49174_d_n0;
        locals.var_t4_dn2 = assign28640_e49174_d_n2;
        locals.var_t4_dn3 = assign28640_e49174_d_n3;
        locals.var_t4_dn4 = assign28640_e49174_d_n4;
        locals.var_t4_dn5 = assign28640_e49174_d_n5;
        locals.var_t4_dn6 = assign28640_e49174_d_n6;
        locals.var_t4_dn7 = assign28640_e49174_d_n7;
        locals.var_t4_dn8 = assign28640_e49174_d_n8;
        locals.var_t4_dn9 = assign28640_e49174_d_n9;
        locals.var_t4_dn10 = assign28640_e49174_d_n10;
        locals.var_t4_dn11 = assign28640_e49174_d_n11;
        locals.var_t4_dn13 = assign28640_e49174_d_n13;
        locals.var_t4_dn14 = assign28640_e49174_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign28650_e49188, assign28650_e49188_d_n0, assign28650_e49188_d_n2, assign28650_e49188_d_n3, assign28650_e49188_d_n4, assign28650_e49188_d_n5, assign28650_e49188_d_n6, assign28650_e49188_d_n7, assign28650_e49188_d_n8, assign28650_e49188_d_n9, assign28650_e49188_d_n10, assign28650_e49188_d_n11, assign28650_e49188_d_n13, assign28650_e49188_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) && (locals.var_guard463 != 0.0)) {
        let assign28650_e49183: f64 = (locals.var_t4).abs();
        let assign28650_e49184: f64 = (locals.var_cgisl_i + assign28650_e49183);
        let assign28650_e49186: f64 = (assign28650_e49184 + 1e-5);
        (assign28650_e49186, if locals.var_t4 >= 0.0 { locals.var_t4_dn0 } else { (-locals.var_t4_dn0) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn2 } else { (-locals.var_t4_dn2) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn3 } else { (-locals.var_t4_dn3) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn4 } else { (-locals.var_t4_dn4) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn5 } else { (-locals.var_t4_dn5) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn6 } else { (-locals.var_t4_dn6) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn7 } else { (-locals.var_t4_dn7) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn8 } else { (-locals.var_t4_dn8) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn9 } else { (-locals.var_t4_dn9) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn10 } else { (-locals.var_t4_dn10) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn11 } else { (-locals.var_t4_dn11) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn13 } else { (-locals.var_t4_dn13) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn14 } else { (-locals.var_t4_dn14) },)
    } else {
        (locals.var_t4a, locals.var_t4a_dn0, locals.var_t4a_dn2, locals.var_t4a_dn3, locals.var_t4a_dn4, locals.var_t4a_dn5, locals.var_t4a_dn6, locals.var_t4a_dn7, locals.var_t4a_dn8, locals.var_t4a_dn9, locals.var_t4a_dn10, locals.var_t4a_dn11, locals.var_t4a_dn13, locals.var_t4a_dn14,)
    }
};
        locals.var_t4a = assign28650_e49188;
        locals.var_t4a_dn0 = assign28650_e49188_d_n0;
        locals.var_t4a_dn2 = assign28650_e49188_d_n2;
        locals.var_t4a_dn3 = assign28650_e49188_d_n3;
        locals.var_t4a_dn4 = assign28650_e49188_d_n4;
        locals.var_t4a_dn5 = assign28650_e49188_d_n5;
        locals.var_t4a_dn6 = assign28650_e49188_d_n6;
        locals.var_t4a_dn7 = assign28650_e49188_d_n7;
        locals.var_t4a_dn8 = assign28650_e49188_d_n8;
        locals.var_t4a_dn9 = assign28650_e49188_d_n9;
        locals.var_t4a_dn10 = assign28650_e49188_d_n10;
        locals.var_t4a_dn11 = assign28650_e49188_d_n11;
        locals.var_t4a_dn13 = assign28650_e49188_d_n13;
        locals.var_t4a_dn14 = assign28650_e49188_d_n14;
        locals.var_t4a_rv = 0.0;

        let (assign28660_e49246, assign28660_e49246_d_n0, assign28660_e49246_d_n2, assign28660_e49246_d_n3, assign28660_e49246_d_n4, assign28660_e49246_d_n5, assign28660_e49246_d_n6, assign28660_e49246_d_n7, assign28660_e49246_d_n8, assign28660_e49246_d_n9, assign28660_e49246_d_n10, assign28660_e49246_d_n11, assign28660_e49246_d_n13, assign28660_e49246_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) && (locals.var_guard463 != 0.0)) {
        let assign28660_e49197: f64 = (locals.var_t4 / locals.var_t4a);
        let assign28660_e49199: f64 = (-10000.0);
        let assign28660_e49201: f64 = (assign28660_e49199 * 1e-6);
        let (assign28660_e49242, assign28660_e49242_d_n0, assign28660_e49242_d_n2, assign28660_e49242_d_n3, assign28660_e49242_d_n4, assign28660_e49242_d_n5, assign28660_e49242_d_n6, assign28660_e49242_d_n7, assign28660_e49242_d_n8, assign28660_e49242_d_n9, assign28660_e49242_d_n10, assign28660_e49242_d_n11, assign28660_e49242_d_n13, assign28660_e49242_d_n14,) = {
            if (!(assign28660_e49197 < assign28660_e49201)) {
                let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4a;
                let assign28660_e49207: f64 = (locals.var_t4 * __rspice_inv_cse_1);
                let assign28660_e49210: f64 = (locals.var_t4 * __rspice_inv_cse_1);
                let assign28660_e49213: f64 = (locals.var_t4 * __rspice_inv_cse_1);
                let assign28660_e49214: f64 = (assign28660_e49210 * assign28660_e49213);
                let assign28660_e49217: f64 = (4.0 * 1e-6);
                let assign28660_e49219: f64 = (assign28660_e49217 * 1e-6);
                let assign28660_e49220: f64 = (assign28660_e49214 + assign28660_e49219);
                let assign28660_e49221: f64 = (assign28660_e49220).sqrt();
                let assign28660_e49222: f64 = (assign28660_e49207 + assign28660_e49221);
                let assign28660_e49223: f64 = (0.5 * assign28660_e49222);
                (assign28660_e49223, (0.5 * ((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))), (0.5 * ((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) * assign28660_e49213) + (assign28660_e49210 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28660_e49221)))),)
            } else {
                let assign28660_e49226: f64 = (locals.var_t4 / locals.var_t4a);
                let assign28660_e49228: f64 = (-10000.0);
                let assign28660_e49230: f64 = (assign28660_e49228 * 1e-6);
                let (assign28660_e49241, assign28660_e49241_d_n0, assign28660_e49241_d_n2, assign28660_e49241_d_n3, assign28660_e49241_d_n4, assign28660_e49241_d_n5, assign28660_e49241_d_n6, assign28660_e49241_d_n7, assign28660_e49241_d_n8, assign28660_e49241_d_n9, assign28660_e49241_d_n10, assign28660_e49241_d_n11, assign28660_e49241_d_n13, assign28660_e49241_d_n14,) = {
                    if (assign28660_e49226 < assign28660_e49230) {
                        let assign28660_e49233: f64 = (-1e-6);
                        let assign28660_e49235: f64 = (assign28660_e49233 * 1e-6);
                        let assign28660_e49238: f64 = (locals.var_t4 / locals.var_t4a);
                        let assign28660_e49239: f64 = (assign28660_e49235 / assign28660_e49238);
                        (assign28660_e49239, (-((assign28660_e49235 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))), (-((assign28660_e49235 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a))) / (assign28660_e49238 * assign28660_e49238))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28660_e49241, assign28660_e49241_d_n0, assign28660_e49241_d_n2, assign28660_e49241_d_n3, assign28660_e49241_d_n4, assign28660_e49241_d_n5, assign28660_e49241_d_n6, assign28660_e49241_d_n7, assign28660_e49241_d_n8, assign28660_e49241_d_n9, assign28660_e49241_d_n10, assign28660_e49241_d_n11, assign28660_e49241_d_n13, assign28660_e49241_d_n14,)
            }
        };
        let assign28660_e49244: f64 = (assign28660_e49242 - 1e-6);
        (assign28660_e49244, assign28660_e49242_d_n0, assign28660_e49242_d_n2, assign28660_e49242_d_n3, assign28660_e49242_d_n4, assign28660_e49242_d_n5, assign28660_e49242_d_n6, assign28660_e49242_d_n7, assign28660_e49242_d_n8, assign28660_e49242_d_n9, assign28660_e49242_d_n10, assign28660_e49242_d_n11, assign28660_e49242_d_n13, assign28660_e49242_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign28660_e49246;
        locals.var_t5_dn0 = assign28660_e49246_d_n0;
        locals.var_t5_dn2 = assign28660_e49246_d_n2;
        locals.var_t5_dn3 = assign28660_e49246_d_n3;
        locals.var_t5_dn4 = assign28660_e49246_d_n4;
        locals.var_t5_dn5 = assign28660_e49246_d_n5;
        locals.var_t5_dn6 = assign28660_e49246_d_n6;
        locals.var_t5_dn7 = assign28660_e49246_d_n7;
        locals.var_t5_dn8 = assign28660_e49246_d_n8;
        locals.var_t5_dn9 = assign28660_e49246_d_n9;
        locals.var_t5_dn10 = assign28660_e49246_d_n10;
        locals.var_t5_dn11 = assign28660_e49246_d_n11;
        locals.var_t5_dn13 = assign28660_e49246_d_n13;
        locals.var_t5_dn14 = assign28660_e49246_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign28670_e49265, assign28670_e49265_d_n0, assign28670_e49265_d_n2, assign28670_e49265_d_n3, assign28670_e49265_d_n4, assign28670_e49265_d_n5, assign28670_e49265_d_n6, assign28670_e49265_d_n7, assign28670_e49265_d_n8, assign28670_e49265_d_n9, assign28670_e49265_d_n10, assign28670_e49265_d_n11, assign28670_e49265_d_n13, assign28670_e49265_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) && (locals.var_guard463 != 0.0)) {
        let assign28670_e49255: f64 = (locals.var_agisl_i * locals.var_weff0);
        let assign28670_e49257: f64 = (assign28670_e49255 * locals.var_t3);
        let assign28670_e49259: f64 = (-locals.var_t2);
        let assign28670_e49260: f64 = { let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28670_e49261: f64 = (assign28670_e49257 * assign28670_e49260);
        let assign28670_e49263: f64 = (assign28670_e49261 * locals.var_t5);
        (assign28670_e49263, (((((assign28670_e49255 * locals.var_t3_dn0) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn0)), (((((assign28670_e49255 * locals.var_t3_dn2) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn2)), (((((assign28670_e49255 * locals.var_t3_dn3) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn3)), (((((assign28670_e49255 * locals.var_t3_dn4) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn4)), (((((assign28670_e49255 * locals.var_t3_dn5) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn5)), (((((assign28670_e49255 * locals.var_t3_dn6) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn6)), (((((assign28670_e49255 * locals.var_t3_dn7) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn7)), (((((assign28670_e49255 * locals.var_t3_dn8) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn8)), (((((assign28670_e49255 * locals.var_t3_dn9) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn9)), (((((assign28670_e49255 * locals.var_t3_dn10) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn10)), (((((assign28670_e49255 * locals.var_t3_dn11) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn11)), (((((assign28670_e49255 * locals.var_t3_dn13) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn13)), (((((assign28670_e49255 * locals.var_t3_dn14) * assign28670_e49260) + (assign28670_e49257 * ({ let limited_exp_arg = assign28670_e49259; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_t5) + (assign28670_e49261 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28670_e49265;
        locals.var_t6_dn0 = assign28670_e49265_d_n0;
        locals.var_t6_dn2 = assign28670_e49265_d_n2;
        locals.var_t6_dn3 = assign28670_e49265_d_n3;
        locals.var_t6_dn4 = assign28670_e49265_d_n4;
        locals.var_t6_dn5 = assign28670_e49265_d_n5;
        locals.var_t6_dn6 = assign28670_e49265_d_n6;
        locals.var_t6_dn7 = assign28670_e49265_d_n7;
        locals.var_t6_dn8 = assign28670_e49265_d_n8;
        locals.var_t6_dn9 = assign28670_e49265_d_n9;
        locals.var_t6_dn10 = assign28670_e49265_d_n10;
        locals.var_t6_dn11 = assign28670_e49265_d_n11;
        locals.var_t6_dn13 = assign28670_e49265_d_n13;
        locals.var_t6_dn14 = assign28670_e49265_d_n14;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28680_e49286, assign28680_e49286_d_n0, assign28680_e49286_d_n2, assign28680_e49286_d_n3, assign28680_e49286_d_n4, assign28680_e49286_d_n5, assign28680_e49286_d_n6, assign28680_e49286_d_n7, assign28680_e49286_d_n8, assign28680_e49286_d_n9, assign28680_e49286_d_n10, assign28680_e49286_d_n11, assign28680_e49286_d_n13, assign28680_e49286_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard462 == 0.0)) && (locals.var_guard463 == 0.0)) {
        let assign28680_e49275: f64 = (locals.var_agisl_i * locals.var_weff0);
        let assign28680_e49277: f64 = (assign28680_e49275 * locals.var_t3);
        let assign28680_e49279: f64 = (-locals.var_t2);
        let assign28680_e49280: f64 = { let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28680_e49281: f64 = (assign28680_e49277 * assign28680_e49280);
        let assign28680_e49283: f64 = (-locals.var_vds_noswap);
        let assign28680_e49284: f64 = (assign28680_e49281 * assign28680_e49283);
        (assign28680_e49284, ((((assign28680_e49275 * locals.var_t3_dn0) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn2) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn3) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn4) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign28680_e49283), (((((assign28680_e49275 * locals.var_t3_dn5) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign28680_e49283) + (assign28680_e49281 * (-locals.var_vds_noswap_dn5))), (((((assign28680_e49275 * locals.var_t3_dn6) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign28680_e49283) + (assign28680_e49281 * (-locals.var_vds_noswap_dn6))), ((((assign28680_e49275 * locals.var_t3_dn7) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn8) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn9) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn10) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn11) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn13) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * assign28680_e49283), ((((assign28680_e49275 * locals.var_t3_dn14) * assign28680_e49280) + (assign28680_e49277 * ({ let limited_exp_arg = assign28680_e49279; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * assign28680_e49283),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28680_e49286;
        locals.var_t6_dn0 = assign28680_e49286_d_n0;
        locals.var_t6_dn2 = assign28680_e49286_d_n2;
        locals.var_t6_dn3 = assign28680_e49286_d_n3;
        locals.var_t6_dn4 = assign28680_e49286_d_n4;
        locals.var_t6_dn5 = assign28680_e49286_d_n5;
        locals.var_t6_dn6 = assign28680_e49286_d_n6;
        locals.var_t6_dn7 = assign28680_e49286_d_n7;
        locals.var_t6_dn8 = assign28680_e49286_d_n8;
        locals.var_t6_dn9 = assign28680_e49286_d_n9;
        locals.var_t6_dn10 = assign28680_e49286_d_n10;
        locals.var_t6_dn11 = assign28680_e49286_d_n11;
        locals.var_t6_dn13 = assign28680_e49286_d_n13;
        locals.var_t6_dn14 = assign28680_e49286_d_n14;
        locals.var_t6_rv = 0.0;

        let assign28690_e49293: f64 = if ((p.p70 == 3.0) && (locals.var_atats_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard464 = assign28690_e49293;
        locals.var_guard464_rv = 0.0;

        let assign28700_e49296: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign28700_e49296;
        locals.var_guard465_rv = 0.0;

        let (assign28710_e49377, assign28710_e49377_d_n4,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 != 0.0)) {
        let assign28710_e49306: f64 = (locals.var_ttat_i * locals.var_deltemp);
        let assign28710_e49307: f64 = (1.0 + assign28710_e49306);
        let assign28710_e49309: f64 = (assign28710_e49307 - 1e-6);
        let assign28710_e49311: f64 = (-10000.0);
        let assign28710_e49313: f64 = (assign28710_e49311 * 0.001);
        let (assign28710_e49374, assign28710_e49374_d_n4,) = {
            if (!(assign28710_e49309 < assign28710_e49313)) {
                let assign28710_e49320: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28710_e49321: f64 = (1.0 + assign28710_e49320);
                let assign28710_e49323: f64 = (assign28710_e49321 - 1e-6);
                let assign28710_e49327: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28710_e49328: f64 = (1.0 + assign28710_e49327);
                let assign28710_e49330: f64 = (assign28710_e49328 - 1e-6);
                let assign28710_e49334: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28710_e49335: f64 = (1.0 + assign28710_e49334);
                let assign28710_e49337: f64 = (assign28710_e49335 - 1e-6);
                let assign28710_e49338: f64 = (assign28710_e49330 * assign28710_e49337);
                let assign28710_e49341: f64 = (4.0 * 0.001);
                let assign28710_e49343: f64 = (assign28710_e49341 * 0.001);
                let assign28710_e49344: f64 = (assign28710_e49338 + assign28710_e49343);
                let assign28710_e49345: f64 = (assign28710_e49344).sqrt();
                let assign28710_e49346: f64 = (assign28710_e49323 + assign28710_e49345);
                let assign28710_e49347: f64 = (0.5 * assign28710_e49346);
                (assign28710_e49347, (0.5 * ((locals.var_ttat_i * locals.var_deltemp_dn4) + ((((locals.var_ttat_i * locals.var_deltemp_dn4) * assign28710_e49337) + (assign28710_e49330 * (locals.var_ttat_i * locals.var_deltemp_dn4))) / (2.0 * assign28710_e49345)))),)
            } else {
                let assign28710_e49351: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28710_e49352: f64 = (1.0 + assign28710_e49351);
                let assign28710_e49354: f64 = (assign28710_e49352 - 1e-6);
                let assign28710_e49356: f64 = (-10000.0);
                let assign28710_e49358: f64 = (assign28710_e49356 * 0.001);
                let (assign28710_e49373, assign28710_e49373_d_n4,) = {
                    if (assign28710_e49354 < assign28710_e49358) {
                        let assign28710_e49361: f64 = (-0.001);
                        let assign28710_e49363: f64 = (assign28710_e49361 * 0.001);
                        let assign28710_e49367: f64 = (locals.var_ttat_i * locals.var_deltemp);
                        let assign28710_e49368: f64 = (1.0 + assign28710_e49367);
                        let assign28710_e49370: f64 = (assign28710_e49368 - 1e-6);
                        let assign28710_e49371: f64 = (assign28710_e49363 / assign28710_e49370);
                        (assign28710_e49371, (-((assign28710_e49363 * (locals.var_ttat_i * locals.var_deltemp_dn4)) / (assign28710_e49370 * assign28710_e49370))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign28710_e49373, assign28710_e49373_d_n4,)
            }
        };
        let assign28710_e49375: f64 = (locals.var_ctats_i * assign28710_e49374);
        (assign28710_e49375, (locals.var_ctats_i * assign28710_e49374_d_n4),)
    } else {
        (locals.var_ctats_t, locals.var_ctats_t_dn4,)
    }
};
        locals.var_ctats_t = assign28710_e49377;
        locals.var_ctats_t_dn4 = assign28710_e49377_d_n4;
        locals.var_ctats_t_rv = 0.0;

        let (assign28720_e49399, assign28720_e49399_d_n0, assign28720_e49399_d_n2, assign28720_e49399_d_n3, assign28720_e49399_d_n4, assign28720_e49399_d_n5, assign28720_e49399_d_n6, assign28720_e49399_d_n7, assign28720_e49399_d_n8, assign28720_e49399_d_n9, assign28720_e49399_d_n10, assign28720_e49399_d_n11, assign28720_e49399_d_n13, assign28720_e49399_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 != 0.0)) {
        let assign28720_e49385: f64 = (locals.var_btats_i * locals.var_vgs_noswap);
        let assign28720_e49387: f64 = (assign28720_e49385 * locals.var_vgs_noswap);
        let assign28720_e49390: f64 = (locals.var_ctats_t * locals.var_vgs_noswap);
        let assign28720_e49391: f64 = (assign28720_e49387 - assign28720_e49390);
        let assign28720_e49393: f64 = (assign28720_e49391 - locals.var_dtats_i);
        let assign28720_e49395: f64 = (assign28720_e49393 + locals.var_vfbsd_v);
        let assign28720_e49397: f64 = (assign28720_e49395 / locals.var_vtm);
        (assign28720_e49397, (locals.var_vfbsd_v_dn0 / locals.var_vtm), (locals.var_vfbsd_v_dn2 / locals.var_vtm), (locals.var_vfbsd_v_dn3 / locals.var_vtm), (((((-(locals.var_ctats_t_dn4 * locals.var_vgs_noswap)) + locals.var_vfbsd_v_dn4) * locals.var_vtm) - (assign28720_e49395 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), (locals.var_vfbsd_v_dn5 / locals.var_vtm), ((((((locals.var_btats_i * locals.var_vgs_noswap_dn6) * locals.var_vgs_noswap) + (assign28720_e49385 * locals.var_vgs_noswap_dn6)) - (locals.var_ctats_t * locals.var_vgs_noswap_dn6)) + locals.var_vfbsd_v_dn6) / locals.var_vtm), (locals.var_vfbsd_v_dn7 / locals.var_vtm), (locals.var_vfbsd_v_dn8 / locals.var_vtm), (locals.var_vfbsd_v_dn9 / locals.var_vtm), (locals.var_vfbsd_v_dn10 / locals.var_vtm), ((((((locals.var_btats_i * locals.var_vgs_noswap_dn11) * locals.var_vgs_noswap) + (assign28720_e49385 * locals.var_vgs_noswap_dn11)) - (locals.var_ctats_t * locals.var_vgs_noswap_dn11)) + locals.var_vfbsd_v_dn11) / locals.var_vtm), (locals.var_vfbsd_v_dn13 / locals.var_vtm), (locals.var_vfbsd_v_dn14 / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28720_e49399;
        locals.var_t1_dn0 = assign28720_e49399_d_n0;
        locals.var_t1_dn2 = assign28720_e49399_d_n2;
        locals.var_t1_dn3 = assign28720_e49399_d_n3;
        locals.var_t1_dn4 = assign28720_e49399_d_n4;
        locals.var_t1_dn5 = assign28720_e49399_d_n5;
        locals.var_t1_dn6 = assign28720_e49399_d_n6;
        locals.var_t1_dn7 = assign28720_e49399_d_n7;
        locals.var_t1_dn8 = assign28720_e49399_d_n8;
        locals.var_t1_dn9 = assign28720_e49399_d_n9;
        locals.var_t1_dn10 = assign28720_e49399_d_n10;
        locals.var_t1_dn11 = assign28720_e49399_d_n11;
        locals.var_t1_dn13 = assign28720_e49399_d_n13;
        locals.var_t1_dn14 = assign28720_e49399_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28730_e49414, assign28730_e49414_d_n0, assign28730_e49414_d_n2, assign28730_e49414_d_n3, assign28730_e49414_d_n4, assign28730_e49414_d_n5, assign28730_e49414_d_n6, assign28730_e49414_d_n7, assign28730_e49414_d_n8, assign28730_e49414_d_n9, assign28730_e49414_d_n10, assign28730_e49414_d_n11, assign28730_e49414_d_n13, assign28730_e49414_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 != 0.0)) {
        let assign28730_e49407: f64 = (locals.var_atats_i * locals.var_weff0);
        let assign28730_e49409: f64 = (assign28730_e49407 * locals.var_ni);
        let assign28730_e49411: f64 = { let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28730_e49412: f64 = (assign28730_e49409 * assign28730_e49411);
        (assign28730_e49412, (((assign28730_e49407 * locals.var_ni_dn0) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn0))), (((assign28730_e49407 * locals.var_ni_dn2) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn2))), (((assign28730_e49407 * locals.var_ni_dn3) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn3))), (((assign28730_e49407 * locals.var_ni_dn4) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn4))), (((assign28730_e49407 * locals.var_ni_dn5) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn5))), (((assign28730_e49407 * locals.var_ni_dn6) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn6))), (((assign28730_e49407 * locals.var_ni_dn7) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn7))), (((assign28730_e49407 * locals.var_ni_dn8) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn8))), (((assign28730_e49407 * locals.var_ni_dn9) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn9))), (((assign28730_e49407 * locals.var_ni_dn10) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn10))), (((assign28730_e49407 * locals.var_ni_dn11) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn11))), (((assign28730_e49407 * locals.var_ni_dn13) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn13))), (((assign28730_e49407 * locals.var_ni_dn14) * assign28730_e49411) + (assign28730_e49409 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28730_e49414;
        locals.var_t2_dn0 = assign28730_e49414_d_n0;
        locals.var_t2_dn2 = assign28730_e49414_d_n2;
        locals.var_t2_dn3 = assign28730_e49414_d_n3;
        locals.var_t2_dn4 = assign28730_e49414_d_n4;
        locals.var_t2_dn5 = assign28730_e49414_d_n5;
        locals.var_t2_dn6 = assign28730_e49414_d_n6;
        locals.var_t2_dn7 = assign28730_e49414_d_n7;
        locals.var_t2_dn8 = assign28730_e49414_d_n8;
        locals.var_t2_dn9 = assign28730_e49414_d_n9;
        locals.var_t2_dn10 = assign28730_e49414_d_n10;
        locals.var_t2_dn11 = assign28730_e49414_d_n11;
        locals.var_t2_dn13 = assign28730_e49414_d_n13;
        locals.var_t2_dn14 = assign28730_e49414_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28740_e49427, assign28740_e49427_d_n0, assign28740_e49427_d_n2, assign28740_e49427_d_n3, assign28740_e49427_d_n4, assign28740_e49427_d_n5, assign28740_e49427_d_n6, assign28740_e49427_d_n7, assign28740_e49427_d_n8, assign28740_e49427_d_n9, assign28740_e49427_d_n10, assign28740_e49427_d_n11, assign28740_e49427_d_n13, assign28740_e49427_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 != 0.0)) {
        let assign28740_e49421: f64 = (-locals.var_ves_jct);
        let assign28740_e49423: f64 = (assign28740_e49421 * locals.var_ves_jct);
        let assign28740_e49425: f64 = (assign28740_e49423 * locals.var_ves_jct);
        (assign28740_e49425, 0.0, 0.0, (((((-locals.var_ves_jct_dn3) * locals.var_ves_jct) + (assign28740_e49421 * locals.var_ves_jct_dn3)) * locals.var_ves_jct) + (assign28740_e49423 * locals.var_ves_jct_dn3)), 0.0, 0.0, (((((-locals.var_ves_jct_dn6) * locals.var_ves_jct) + (assign28740_e49421 * locals.var_ves_jct_dn6)) * locals.var_ves_jct) + (assign28740_e49423 * locals.var_ves_jct_dn6)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28740_e49427;
        locals.var_t4_dn0 = assign28740_e49427_d_n0;
        locals.var_t4_dn2 = assign28740_e49427_d_n2;
        locals.var_t4_dn3 = assign28740_e49427_d_n3;
        locals.var_t4_dn4 = assign28740_e49427_d_n4;
        locals.var_t4_dn5 = assign28740_e49427_d_n5;
        locals.var_t4_dn6 = assign28740_e49427_d_n6;
        locals.var_t4_dn7 = assign28740_e49427_d_n7;
        locals.var_t4_dn8 = assign28740_e49427_d_n8;
        locals.var_t4_dn9 = assign28740_e49427_d_n9;
        locals.var_t4_dn10 = assign28740_e49427_d_n10;
        locals.var_t4_dn11 = assign28740_e49427_d_n11;
        locals.var_t4_dn13 = assign28740_e49427_d_n13;
        locals.var_t4_dn14 = assign28740_e49427_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign28750_e49440, assign28750_e49440_d_n0, assign28750_e49440_d_n2, assign28750_e49440_d_n3, assign28750_e49440_d_n4, assign28750_e49440_d_n5, assign28750_e49440_d_n6, assign28750_e49440_d_n7, assign28750_e49440_d_n8, assign28750_e49440_d_n9, assign28750_e49440_d_n10, assign28750_e49440_d_n11, assign28750_e49440_d_n13, assign28750_e49440_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 != 0.0)) {
        let assign28750_e49435: f64 = (locals.var_t4).abs();
        let assign28750_e49436: f64 = (locals.var_cgisl_i + assign28750_e49435);
        let assign28750_e49438: f64 = (assign28750_e49436 + 1e-5);
        (assign28750_e49438, if locals.var_t4 >= 0.0 { locals.var_t4_dn0 } else { (-locals.var_t4_dn0) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn2 } else { (-locals.var_t4_dn2) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn3 } else { (-locals.var_t4_dn3) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn4 } else { (-locals.var_t4_dn4) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn5 } else { (-locals.var_t4_dn5) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn6 } else { (-locals.var_t4_dn6) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn7 } else { (-locals.var_t4_dn7) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn8 } else { (-locals.var_t4_dn8) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn9 } else { (-locals.var_t4_dn9) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn10 } else { (-locals.var_t4_dn10) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn11 } else { (-locals.var_t4_dn11) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn13 } else { (-locals.var_t4_dn13) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn14 } else { (-locals.var_t4_dn14) },)
    } else {
        (locals.var_t4a, locals.var_t4a_dn0, locals.var_t4a_dn2, locals.var_t4a_dn3, locals.var_t4a_dn4, locals.var_t4a_dn5, locals.var_t4a_dn6, locals.var_t4a_dn7, locals.var_t4a_dn8, locals.var_t4a_dn9, locals.var_t4a_dn10, locals.var_t4a_dn11, locals.var_t4a_dn13, locals.var_t4a_dn14,)
    }
};
        locals.var_t4a = assign28750_e49440;
        locals.var_t4a_dn0 = assign28750_e49440_d_n0;
        locals.var_t4a_dn2 = assign28750_e49440_d_n2;
        locals.var_t4a_dn3 = assign28750_e49440_d_n3;
        locals.var_t4a_dn4 = assign28750_e49440_d_n4;
        locals.var_t4a_dn5 = assign28750_e49440_d_n5;
        locals.var_t4a_dn6 = assign28750_e49440_d_n6;
        locals.var_t4a_dn7 = assign28750_e49440_d_n7;
        locals.var_t4a_dn8 = assign28750_e49440_d_n8;
        locals.var_t4a_dn9 = assign28750_e49440_d_n9;
        locals.var_t4a_dn10 = assign28750_e49440_d_n10;
        locals.var_t4a_dn11 = assign28750_e49440_d_n11;
        locals.var_t4a_dn13 = assign28750_e49440_d_n13;
        locals.var_t4a_dn14 = assign28750_e49440_d_n14;
        locals.var_t4a_rv = 0.0;

        let (assign28760_e49497, assign28760_e49497_d_n0, assign28760_e49497_d_n2, assign28760_e49497_d_n3, assign28760_e49497_d_n4, assign28760_e49497_d_n5, assign28760_e49497_d_n6, assign28760_e49497_d_n7, assign28760_e49497_d_n8, assign28760_e49497_d_n9, assign28760_e49497_d_n10, assign28760_e49497_d_n11, assign28760_e49497_d_n13, assign28760_e49497_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 != 0.0)) {
        let assign28760_e49448: f64 = (locals.var_t4 / locals.var_t4a);
        let assign28760_e49450: f64 = (-10000.0);
        let assign28760_e49452: f64 = (assign28760_e49450 * 1e-6);
        let (assign28760_e49493, assign28760_e49493_d_n0, assign28760_e49493_d_n2, assign28760_e49493_d_n3, assign28760_e49493_d_n4, assign28760_e49493_d_n5, assign28760_e49493_d_n6, assign28760_e49493_d_n7, assign28760_e49493_d_n8, assign28760_e49493_d_n9, assign28760_e49493_d_n10, assign28760_e49493_d_n11, assign28760_e49493_d_n13, assign28760_e49493_d_n14,) = {
            if (!(assign28760_e49448 < assign28760_e49452)) {
                let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4a;
                let assign28760_e49458: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28760_e49461: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28760_e49464: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28760_e49465: f64 = (assign28760_e49461 * assign28760_e49464);
                let assign28760_e49468: f64 = (4.0 * 1e-6);
                let assign28760_e49470: f64 = (assign28760_e49468 * 1e-6);
                let assign28760_e49471: f64 = (assign28760_e49465 + assign28760_e49470);
                let assign28760_e49472: f64 = (assign28760_e49471).sqrt();
                let assign28760_e49473: f64 = (assign28760_e49458 + assign28760_e49472);
                let assign28760_e49474: f64 = (0.5 * assign28760_e49473);
                (assign28760_e49474, (0.5 * ((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))), (0.5 * ((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) * assign28760_e49464) + (assign28760_e49461 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28760_e49472)))),)
            } else {
                let assign28760_e49477: f64 = (locals.var_t4 / locals.var_t4a);
                let assign28760_e49479: f64 = (-10000.0);
                let assign28760_e49481: f64 = (assign28760_e49479 * 1e-6);
                let (assign28760_e49492, assign28760_e49492_d_n0, assign28760_e49492_d_n2, assign28760_e49492_d_n3, assign28760_e49492_d_n4, assign28760_e49492_d_n5, assign28760_e49492_d_n6, assign28760_e49492_d_n7, assign28760_e49492_d_n8, assign28760_e49492_d_n9, assign28760_e49492_d_n10, assign28760_e49492_d_n11, assign28760_e49492_d_n13, assign28760_e49492_d_n14,) = {
                    if (assign28760_e49477 < assign28760_e49481) {
                        let assign28760_e49484: f64 = (-1e-6);
                        let assign28760_e49486: f64 = (assign28760_e49484 * 1e-6);
                        let assign28760_e49489: f64 = (locals.var_t4 / locals.var_t4a);
                        let assign28760_e49490: f64 = (assign28760_e49486 / assign28760_e49489);
                        (assign28760_e49490, (-((assign28760_e49486 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))), (-((assign28760_e49486 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a))) / (assign28760_e49489 * assign28760_e49489))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28760_e49492, assign28760_e49492_d_n0, assign28760_e49492_d_n2, assign28760_e49492_d_n3, assign28760_e49492_d_n4, assign28760_e49492_d_n5, assign28760_e49492_d_n6, assign28760_e49492_d_n7, assign28760_e49492_d_n8, assign28760_e49492_d_n9, assign28760_e49492_d_n10, assign28760_e49492_d_n11, assign28760_e49492_d_n13, assign28760_e49492_d_n14,)
            }
        };
        let assign28760_e49495: f64 = (assign28760_e49493 - 1e-6);
        (assign28760_e49495, assign28760_e49493_d_n0, assign28760_e49493_d_n2, assign28760_e49493_d_n3, assign28760_e49493_d_n4, assign28760_e49493_d_n5, assign28760_e49493_d_n6, assign28760_e49493_d_n7, assign28760_e49493_d_n8, assign28760_e49493_d_n9, assign28760_e49493_d_n10, assign28760_e49493_d_n11, assign28760_e49493_d_n13, assign28760_e49493_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign28760_e49497;
        locals.var_t5_dn0 = assign28760_e49497_d_n0;
        locals.var_t5_dn2 = assign28760_e49497_d_n2;
        locals.var_t5_dn3 = assign28760_e49497_d_n3;
        locals.var_t5_dn4 = assign28760_e49497_d_n4;
        locals.var_t5_dn5 = assign28760_e49497_d_n5;
        locals.var_t5_dn6 = assign28760_e49497_d_n6;
        locals.var_t5_dn7 = assign28760_e49497_d_n7;
        locals.var_t5_dn8 = assign28760_e49497_d_n8;
        locals.var_t5_dn9 = assign28760_e49497_d_n9;
        locals.var_t5_dn10 = assign28760_e49497_d_n10;
        locals.var_t5_dn11 = assign28760_e49497_d_n11;
        locals.var_t5_dn13 = assign28760_e49497_d_n13;
        locals.var_t5_dn14 = assign28760_e49497_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign28770_e49509, assign28770_e49509_d_n0, assign28770_e49509_d_n2, assign28770_e49509_d_n3, assign28770_e49509_d_n4, assign28770_e49509_d_n5, assign28770_e49509_d_n6, assign28770_e49509_d_n7, assign28770_e49509_d_n8, assign28770_e49509_d_n9, assign28770_e49509_d_n10, assign28770_e49509_d_n11, assign28770_e49509_d_n13, assign28770_e49509_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 != 0.0)) {
        let assign28770_e49506: f64 = (locals.var_t2 * locals.var_t5);
        let assign28770_e49507: f64 = (locals.var_t6 + assign28770_e49506);
        (assign28770_e49507, (locals.var_t6_dn0 + ((locals.var_t2_dn0 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn0))), (locals.var_t6_dn2 + ((locals.var_t2_dn2 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn2))), (locals.var_t6_dn3 + ((locals.var_t2_dn3 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn3))), (locals.var_t6_dn4 + ((locals.var_t2_dn4 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn4))), (locals.var_t6_dn5 + ((locals.var_t2_dn5 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn5))), (locals.var_t6_dn6 + ((locals.var_t2_dn6 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn6))), (locals.var_t6_dn7 + ((locals.var_t2_dn7 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn7))), (locals.var_t6_dn8 + ((locals.var_t2_dn8 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn8))), (locals.var_t6_dn9 + ((locals.var_t2_dn9 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn9))), (locals.var_t6_dn10 + ((locals.var_t2_dn10 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn10))), (locals.var_t6_dn11 + ((locals.var_t2_dn11 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn11))), (locals.var_t6_dn13 + ((locals.var_t2_dn13 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn13))), (locals.var_t6_dn14 + ((locals.var_t2_dn14 * locals.var_t5) + (locals.var_t2 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28770_e49509;
        locals.var_t6_dn0 = assign28770_e49509_d_n0;
        locals.var_t6_dn2 = assign28770_e49509_d_n2;
        locals.var_t6_dn3 = assign28770_e49509_d_n3;
        locals.var_t6_dn4 = assign28770_e49509_d_n4;
        locals.var_t6_dn5 = assign28770_e49509_d_n5;
        locals.var_t6_dn6 = assign28770_e49509_d_n6;
        locals.var_t6_dn7 = assign28770_e49509_d_n7;
        locals.var_t6_dn8 = assign28770_e49509_d_n8;
        locals.var_t6_dn9 = assign28770_e49509_d_n9;
        locals.var_t6_dn10 = assign28770_e49509_d_n10;
        locals.var_t6_dn11 = assign28770_e49509_d_n11;
        locals.var_t6_dn13 = assign28770_e49509_d_n13;
        locals.var_t6_dn14 = assign28770_e49509_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign28780_e49591, assign28780_e49591_d_n4,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 == 0.0)) {
        let assign28780_e49520: f64 = (locals.var_ttat_i * locals.var_deltemp);
        let assign28780_e49521: f64 = (1.0 + assign28780_e49520);
        let assign28780_e49523: f64 = (assign28780_e49521 - 1e-6);
        let assign28780_e49525: f64 = (-10000.0);
        let assign28780_e49527: f64 = (assign28780_e49525 * 0.001);
        let (assign28780_e49588, assign28780_e49588_d_n4,) = {
            if (!(assign28780_e49523 < assign28780_e49527)) {
                let assign28780_e49534: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28780_e49535: f64 = (1.0 + assign28780_e49534);
                let assign28780_e49537: f64 = (assign28780_e49535 - 1e-6);
                let assign28780_e49541: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28780_e49542: f64 = (1.0 + assign28780_e49541);
                let assign28780_e49544: f64 = (assign28780_e49542 - 1e-6);
                let assign28780_e49548: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28780_e49549: f64 = (1.0 + assign28780_e49548);
                let assign28780_e49551: f64 = (assign28780_e49549 - 1e-6);
                let assign28780_e49552: f64 = (assign28780_e49544 * assign28780_e49551);
                let assign28780_e49555: f64 = (4.0 * 0.001);
                let assign28780_e49557: f64 = (assign28780_e49555 * 0.001);
                let assign28780_e49558: f64 = (assign28780_e49552 + assign28780_e49557);
                let assign28780_e49559: f64 = (assign28780_e49558).sqrt();
                let assign28780_e49560: f64 = (assign28780_e49537 + assign28780_e49559);
                let assign28780_e49561: f64 = (0.5 * assign28780_e49560);
                (assign28780_e49561, (0.5 * ((locals.var_ttat_i * locals.var_deltemp_dn4) + ((((locals.var_ttat_i * locals.var_deltemp_dn4) * assign28780_e49551) + (assign28780_e49544 * (locals.var_ttat_i * locals.var_deltemp_dn4))) / (2.0 * assign28780_e49559)))),)
            } else {
                let assign28780_e49565: f64 = (locals.var_ttat_i * locals.var_deltemp);
                let assign28780_e49566: f64 = (1.0 + assign28780_e49565);
                let assign28780_e49568: f64 = (assign28780_e49566 - 1e-6);
                let assign28780_e49570: f64 = (-10000.0);
                let assign28780_e49572: f64 = (assign28780_e49570 * 0.001);
                let (assign28780_e49587, assign28780_e49587_d_n4,) = {
                    if (assign28780_e49568 < assign28780_e49572) {
                        let assign28780_e49575: f64 = (-0.001);
                        let assign28780_e49577: f64 = (assign28780_e49575 * 0.001);
                        let assign28780_e49581: f64 = (locals.var_ttat_i * locals.var_deltemp);
                        let assign28780_e49582: f64 = (1.0 + assign28780_e49581);
                        let assign28780_e49584: f64 = (assign28780_e49582 - 1e-6);
                        let assign28780_e49585: f64 = (assign28780_e49577 / assign28780_e49584);
                        (assign28780_e49585, (-((assign28780_e49577 * (locals.var_ttat_i * locals.var_deltemp_dn4)) / (assign28780_e49584 * assign28780_e49584))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign28780_e49587, assign28780_e49587_d_n4,)
            }
        };
        let assign28780_e49589: f64 = (locals.var_ctats_i * assign28780_e49588);
        (assign28780_e49589, (locals.var_ctats_i * assign28780_e49588_d_n4),)
    } else {
        (locals.var_ctats_t, locals.var_ctats_t_dn4,)
    }
};
        locals.var_ctats_t = assign28780_e49591;
        locals.var_ctats_t_dn4 = assign28780_e49591_d_n4;
        locals.var_ctats_t_rv = 0.0;

        let (assign28790_e49614, assign28790_e49614_d_n0, assign28790_e49614_d_n2, assign28790_e49614_d_n3, assign28790_e49614_d_n4, assign28790_e49614_d_n5, assign28790_e49614_d_n6, assign28790_e49614_d_n7, assign28790_e49614_d_n8, assign28790_e49614_d_n9, assign28790_e49614_d_n10, assign28790_e49614_d_n11, assign28790_e49614_d_n13, assign28790_e49614_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 == 0.0)) {
        let assign28790_e49600: f64 = (locals.var_btats_i * locals.var_vgs_noswap);
        let assign28790_e49602: f64 = (assign28790_e49600 * locals.var_vgs_noswap);
        let assign28790_e49605: f64 = (locals.var_ctats_t * locals.var_vgs_noswap);
        let assign28790_e49606: f64 = (assign28790_e49602 - assign28790_e49605);
        let assign28790_e49608: f64 = (assign28790_e49606 - locals.var_dtats_i);
        let assign28790_e49610: f64 = (assign28790_e49608 + locals.var_vfbsd_v);
        let assign28790_e49612: f64 = (assign28790_e49610 / locals.var_vtm);
        (assign28790_e49612, (locals.var_vfbsd_v_dn0 / locals.var_vtm), (locals.var_vfbsd_v_dn2 / locals.var_vtm), (locals.var_vfbsd_v_dn3 / locals.var_vtm), (((((-(locals.var_ctats_t_dn4 * locals.var_vgs_noswap)) + locals.var_vfbsd_v_dn4) * locals.var_vtm) - (assign28790_e49610 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), (locals.var_vfbsd_v_dn5 / locals.var_vtm), ((((((locals.var_btats_i * locals.var_vgs_noswap_dn6) * locals.var_vgs_noswap) + (assign28790_e49600 * locals.var_vgs_noswap_dn6)) - (locals.var_ctats_t * locals.var_vgs_noswap_dn6)) + locals.var_vfbsd_v_dn6) / locals.var_vtm), (locals.var_vfbsd_v_dn7 / locals.var_vtm), (locals.var_vfbsd_v_dn8 / locals.var_vtm), (locals.var_vfbsd_v_dn9 / locals.var_vtm), (locals.var_vfbsd_v_dn10 / locals.var_vtm), ((((((locals.var_btats_i * locals.var_vgs_noswap_dn11) * locals.var_vgs_noswap) + (assign28790_e49600 * locals.var_vgs_noswap_dn11)) - (locals.var_ctats_t * locals.var_vgs_noswap_dn11)) + locals.var_vfbsd_v_dn11) / locals.var_vtm), (locals.var_vfbsd_v_dn13 / locals.var_vtm), (locals.var_vfbsd_v_dn14 / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28790_e49614;
        locals.var_t1_dn0 = assign28790_e49614_d_n0;
        locals.var_t1_dn2 = assign28790_e49614_d_n2;
        locals.var_t1_dn3 = assign28790_e49614_d_n3;
        locals.var_t1_dn4 = assign28790_e49614_d_n4;
        locals.var_t1_dn5 = assign28790_e49614_d_n5;
        locals.var_t1_dn6 = assign28790_e49614_d_n6;
        locals.var_t1_dn7 = assign28790_e49614_d_n7;
        locals.var_t1_dn8 = assign28790_e49614_d_n8;
        locals.var_t1_dn9 = assign28790_e49614_d_n9;
        locals.var_t1_dn10 = assign28790_e49614_d_n10;
        locals.var_t1_dn11 = assign28790_e49614_d_n11;
        locals.var_t1_dn13 = assign28790_e49614_d_n13;
        locals.var_t1_dn14 = assign28790_e49614_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28800_e49630, assign28800_e49630_d_n0, assign28800_e49630_d_n2, assign28800_e49630_d_n3, assign28800_e49630_d_n4, assign28800_e49630_d_n5, assign28800_e49630_d_n6, assign28800_e49630_d_n7, assign28800_e49630_d_n8, assign28800_e49630_d_n9, assign28800_e49630_d_n10, assign28800_e49630_d_n11, assign28800_e49630_d_n13, assign28800_e49630_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 == 0.0)) {
        let assign28800_e49623: f64 = (locals.var_atats_i * locals.var_weff0);
        let assign28800_e49625: f64 = (assign28800_e49623 * locals.var_ni);
        let assign28800_e49627: f64 = { let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28800_e49628: f64 = (assign28800_e49625 * assign28800_e49627);
        (assign28800_e49628, (((assign28800_e49623 * locals.var_ni_dn0) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn0))), (((assign28800_e49623 * locals.var_ni_dn2) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn2))), (((assign28800_e49623 * locals.var_ni_dn3) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn3))), (((assign28800_e49623 * locals.var_ni_dn4) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn4))), (((assign28800_e49623 * locals.var_ni_dn5) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn5))), (((assign28800_e49623 * locals.var_ni_dn6) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn6))), (((assign28800_e49623 * locals.var_ni_dn7) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn7))), (((assign28800_e49623 * locals.var_ni_dn8) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn8))), (((assign28800_e49623 * locals.var_ni_dn9) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn9))), (((assign28800_e49623 * locals.var_ni_dn10) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn10))), (((assign28800_e49623 * locals.var_ni_dn11) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn11))), (((assign28800_e49623 * locals.var_ni_dn13) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn13))), (((assign28800_e49623 * locals.var_ni_dn14) * assign28800_e49627) + (assign28800_e49625 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28800_e49630;
        locals.var_t2_dn0 = assign28800_e49630_d_n0;
        locals.var_t2_dn2 = assign28800_e49630_d_n2;
        locals.var_t2_dn3 = assign28800_e49630_d_n3;
        locals.var_t2_dn4 = assign28800_e49630_d_n4;
        locals.var_t2_dn5 = assign28800_e49630_d_n5;
        locals.var_t2_dn6 = assign28800_e49630_d_n6;
        locals.var_t2_dn7 = assign28800_e49630_d_n7;
        locals.var_t2_dn8 = assign28800_e49630_d_n8;
        locals.var_t2_dn9 = assign28800_e49630_d_n9;
        locals.var_t2_dn10 = assign28800_e49630_d_n10;
        locals.var_t2_dn11 = assign28800_e49630_d_n11;
        locals.var_t2_dn13 = assign28800_e49630_d_n13;
        locals.var_t2_dn14 = assign28800_e49630_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28810_e49644, assign28810_e49644_d_n0, assign28810_e49644_d_n2, assign28810_e49644_d_n3, assign28810_e49644_d_n4, assign28810_e49644_d_n5, assign28810_e49644_d_n6, assign28810_e49644_d_n7, assign28810_e49644_d_n8, assign28810_e49644_d_n9, assign28810_e49644_d_n10, assign28810_e49644_d_n11, assign28810_e49644_d_n13, assign28810_e49644_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard464 != 0.0)) && (locals.var_guard465 == 0.0)) {
        let assign28810_e49640: f64 = (-locals.var_vds_noswap);
        let assign28810_e49641: f64 = (locals.var_t2 * assign28810_e49640);
        let assign28810_e49642: f64 = (locals.var_t6 + assign28810_e49641);
        (assign28810_e49642, (locals.var_t6_dn0 + (locals.var_t2_dn0 * assign28810_e49640)), (locals.var_t6_dn2 + (locals.var_t2_dn2 * assign28810_e49640)), (locals.var_t6_dn3 + (locals.var_t2_dn3 * assign28810_e49640)), (locals.var_t6_dn4 + (locals.var_t2_dn4 * assign28810_e49640)), (locals.var_t6_dn5 + ((locals.var_t2_dn5 * assign28810_e49640) + (locals.var_t2 * (-locals.var_vds_noswap_dn5)))), (locals.var_t6_dn6 + ((locals.var_t2_dn6 * assign28810_e49640) + (locals.var_t2 * (-locals.var_vds_noswap_dn6)))), (locals.var_t6_dn7 + (locals.var_t2_dn7 * assign28810_e49640)), (locals.var_t6_dn8 + (locals.var_t2_dn8 * assign28810_e49640)), (locals.var_t6_dn9 + (locals.var_t2_dn9 * assign28810_e49640)), (locals.var_t6_dn10 + (locals.var_t2_dn10 * assign28810_e49640)), (locals.var_t6_dn11 + (locals.var_t2_dn11 * assign28810_e49640)), (locals.var_t6_dn13 + (locals.var_t2_dn13 * assign28810_e49640)), (locals.var_t6_dn14 + (locals.var_t2_dn14 * assign28810_e49640)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign28810_e49644;
        locals.var_t6_dn0 = assign28810_e49644_d_n0;
        locals.var_t6_dn2 = assign28810_e49644_d_n2;
        locals.var_t6_dn3 = assign28810_e49644_d_n3;
        locals.var_t6_dn4 = assign28810_e49644_d_n4;
        locals.var_t6_dn5 = assign28810_e49644_d_n5;
        locals.var_t6_dn6 = assign28810_e49644_d_n6;
        locals.var_t6_dn7 = assign28810_e49644_d_n7;
        locals.var_t6_dn8 = assign28810_e49644_d_n8;
        locals.var_t6_dn9 = assign28810_e49644_d_n9;
        locals.var_t6_dn10 = assign28810_e49644_d_n10;
        locals.var_t6_dn11 = assign28810_e49644_d_n11;
        locals.var_t6_dn13 = assign28810_e49644_d_n13;
        locals.var_t6_dn14 = assign28810_e49644_d_n14;
        locals.var_t6_rv = 0.0;

        let assign28820_e49667: f64 = if (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        locals.var_guard466 = assign28820_e49667;
        locals.var_guard466_rv = 0.0;

        let (assign28830_e49746, assign28830_e49746_d_n4,) = {
    if ((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) {
        let assign28830_e49675: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign28830_e49676: f64 = (1.0 + assign28830_e49675);
        let assign28830_e49678: f64 = (assign28830_e49676 - 1e-6);
        let assign28830_e49680: f64 = (-10000.0);
        let assign28830_e49682: f64 = (assign28830_e49680 * 0.001);
        let (assign28830_e49743, assign28830_e49743_d_n4,) = {
            if (!(assign28830_e49678 < assign28830_e49682)) {
                let assign28830_e49689: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28830_e49690: f64 = (1.0 + assign28830_e49689);
                let assign28830_e49692: f64 = (assign28830_e49690 - 1e-6);
                let assign28830_e49696: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28830_e49697: f64 = (1.0 + assign28830_e49696);
                let assign28830_e49699: f64 = (assign28830_e49697 - 1e-6);
                let assign28830_e49703: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28830_e49704: f64 = (1.0 + assign28830_e49703);
                let assign28830_e49706: f64 = (assign28830_e49704 - 1e-6);
                let assign28830_e49707: f64 = (assign28830_e49699 * assign28830_e49706);
                let assign28830_e49710: f64 = (4.0 * 0.001);
                let assign28830_e49712: f64 = (assign28830_e49710 * 0.001);
                let assign28830_e49713: f64 = (assign28830_e49707 + assign28830_e49712);
                let assign28830_e49714: f64 = (assign28830_e49713).sqrt();
                let assign28830_e49715: f64 = (assign28830_e49692 + assign28830_e49714);
                let assign28830_e49716: f64 = (0.5 * assign28830_e49715);
                (assign28830_e49716, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign28830_e49706) + (assign28830_e49699 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign28830_e49714)))),)
            } else {
                let assign28830_e49720: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign28830_e49721: f64 = (1.0 + assign28830_e49720);
                let assign28830_e49723: f64 = (assign28830_e49721 - 1e-6);
                let assign28830_e49725: f64 = (-10000.0);
                let assign28830_e49727: f64 = (assign28830_e49725 * 0.001);
                let (assign28830_e49742, assign28830_e49742_d_n4,) = {
                    if (assign28830_e49723 < assign28830_e49727) {
                        let assign28830_e49730: f64 = (-0.001);
                        let assign28830_e49732: f64 = (assign28830_e49730 * 0.001);
                        let assign28830_e49736: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                        let assign28830_e49737: f64 = (1.0 + assign28830_e49736);
                        let assign28830_e49739: f64 = (assign28830_e49737 - 1e-6);
                        let assign28830_e49740: f64 = (assign28830_e49732 / assign28830_e49739);
                        (assign28830_e49740, (-((assign28830_e49732 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign28830_e49739 * assign28830_e49739))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign28830_e49742, assign28830_e49742_d_n4,)
            }
        };
        let assign28830_e49744: f64 = (locals.var_bgislb_i * assign28830_e49743);
        (assign28830_e49744, (locals.var_bgislb_i * assign28830_e49743_d_n4),)
    } else {
        (locals.var_bgislb_t, locals.var_bgislb_t_dn4,)
    }
};
        locals.var_bgislb_t = assign28830_e49746;
        locals.var_bgislb_t_dn4 = assign28830_e49746_d_n4;
        locals.var_bgislb_t_rv = 0.0;

        let assign28840_e49753: f64 = if ((locals.var_agislb_i <= 0.0) || (locals.var_bgislb_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard467 = assign28840_e49753;
        locals.var_guard467_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28850_e49761, assign28850_e49761_d_n0, assign28850_e49761_d_n2, assign28850_e49761_d_n3, assign28850_e49761_d_n4, assign28850_e49761_d_n5, assign28850_e49761_d_n6, assign28850_e49761_d_n7, assign28850_e49761_d_n8, assign28850_e49761_d_n9, assign28850_e49761_d_n10, assign28850_e49761_d_n11, assign28850_e49761_d_n13, assign28850_e49761_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign28850_e49761;
        locals.var_t7_dn0 = assign28850_e49761_d_n0;
        locals.var_t7_dn2 = assign28850_e49761_d_n2;
        locals.var_t7_dn3 = assign28850_e49761_d_n3;
        locals.var_t7_dn4 = assign28850_e49761_d_n4;
        locals.var_t7_dn5 = assign28850_e49761_d_n5;
        locals.var_t7_dn6 = assign28850_e49761_d_n6;
        locals.var_t7_dn7 = assign28850_e49761_d_n7;
        locals.var_t7_dn8 = assign28850_e49761_d_n8;
        locals.var_t7_dn9 = assign28850_e49761_d_n9;
        locals.var_t7_dn10 = assign28850_e49761_d_n10;
        locals.var_t7_dn11 = assign28850_e49761_d_n11;
        locals.var_t7_dn13 = assign28850_e49761_d_n13;
        locals.var_t7_dn14 = assign28850_e49761_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign28860_e49777, assign28860_e49777_d_n0, assign28860_e49777_d_n2, assign28860_e49777_d_n3, assign28860_e49777_d_n4, assign28860_e49777_d_n5, assign28860_e49777_d_n6, assign28860_e49777_d_n7, assign28860_e49777_d_n8, assign28860_e49777_d_n9, assign28860_e49777_d_n10, assign28860_e49777_d_n11, assign28860_e49777_d_n13, assign28860_e49777_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28860_e49769: f64 = (-locals.var_vgs_noswap);
        let assign28860_e49771: f64 = (assign28860_e49769 - locals.var_egislb_i);
        let assign28860_e49773: f64 = (assign28860_e49771 + locals.var_vfbsd_v);
        let assign28860_e49775: f64 = (assign28860_e49773 / locals.var_t0);
        (assign28860_e49775, (((locals.var_vfbsd_v_dn0 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn2 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn3 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn4 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn5 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgs_noswap_dn6) + locals.var_vfbsd_v_dn6) * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn7 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn8 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn9 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn10 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((-locals.var_vgs_noswap_dn11) + locals.var_vfbsd_v_dn11) * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn13 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsd_v_dn14 * locals.var_t0) - (assign28860_e49773 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28860_e49777;
        locals.var_t1_dn0 = assign28860_e49777_d_n0;
        locals.var_t1_dn2 = assign28860_e49777_d_n2;
        locals.var_t1_dn3 = assign28860_e49777_d_n3;
        locals.var_t1_dn4 = assign28860_e49777_d_n4;
        locals.var_t1_dn5 = assign28860_e49777_d_n5;
        locals.var_t1_dn6 = assign28860_e49777_d_n6;
        locals.var_t1_dn7 = assign28860_e49777_d_n7;
        locals.var_t1_dn8 = assign28860_e49777_d_n8;
        locals.var_t1_dn9 = assign28860_e49777_d_n9;
        locals.var_t1_dn10 = assign28860_e49777_d_n10;
        locals.var_t1_dn11 = assign28860_e49777_d_n11;
        locals.var_t1_dn13 = assign28860_e49777_d_n13;
        locals.var_t1_dn14 = assign28860_e49777_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28870_e49821, assign28870_e49821_d_n0, assign28870_e49821_d_n2, assign28870_e49821_d_n3, assign28870_e49821_d_n4, assign28870_e49821_d_n5, assign28870_e49821_d_n6, assign28870_e49821_d_n7, assign28870_e49821_d_n8, assign28870_e49821_d_n9, assign28870_e49821_d_n10, assign28870_e49821_d_n11, assign28870_e49821_d_n13, assign28870_e49821_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28870_e49786: f64 = (-10000.0);
        let assign28870_e49788: f64 = (assign28870_e49786 * 0.01);
        let (assign28870_e49819, assign28870_e49819_d_n0, assign28870_e49819_d_n2, assign28870_e49819_d_n3, assign28870_e49819_d_n4, assign28870_e49819_d_n5, assign28870_e49819_d_n6, assign28870_e49819_d_n7, assign28870_e49819_d_n8, assign28870_e49819_d_n9, assign28870_e49819_d_n10, assign28870_e49819_d_n11, assign28870_e49819_d_n13, assign28870_e49819_d_n14,) = {
            if (!(locals.var_t1 < assign28870_e49788)) {
                let assign28870_e49795: f64 = (locals.var_t1 * locals.var_t1);
                let assign28870_e49798: f64 = (4.0 * 0.01);
                let assign28870_e49800: f64 = (assign28870_e49798 * 0.01);
                let assign28870_e49801: f64 = (assign28870_e49795 + assign28870_e49800);
                let assign28870_e49802: f64 = (assign28870_e49801).sqrt();
                let assign28870_e49803: f64 = (locals.var_t1 + assign28870_e49802);
                let assign28870_e49804: f64 = (0.5 * assign28870_e49803);
                (assign28870_e49804, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign28870_e49802)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign28870_e49802)))),)
            } else {
                let assign28870_e49807: f64 = (-10000.0);
                let assign28870_e49809: f64 = (assign28870_e49807 * 0.01);
                let (assign28870_e49818, assign28870_e49818_d_n0, assign28870_e49818_d_n2, assign28870_e49818_d_n3, assign28870_e49818_d_n4, assign28870_e49818_d_n5, assign28870_e49818_d_n6, assign28870_e49818_d_n7, assign28870_e49818_d_n8, assign28870_e49818_d_n9, assign28870_e49818_d_n10, assign28870_e49818_d_n11, assign28870_e49818_d_n13, assign28870_e49818_d_n14,) = {
                    if (locals.var_t1 < assign28870_e49809) {
                        let assign28870_e49812: f64 = (-0.01);
                        let assign28870_e49814: f64 = (assign28870_e49812 * 0.01);
                        let assign28870_e49816: f64 = (assign28870_e49814 / locals.var_t1);
                        (assign28870_e49816, (-((assign28870_e49814 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign28870_e49814 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28870_e49818, assign28870_e49818_d_n0, assign28870_e49818_d_n2, assign28870_e49818_d_n3, assign28870_e49818_d_n4, assign28870_e49818_d_n5, assign28870_e49818_d_n6, assign28870_e49818_d_n7, assign28870_e49818_d_n8, assign28870_e49818_d_n9, assign28870_e49818_d_n10, assign28870_e49818_d_n11, assign28870_e49818_d_n13, assign28870_e49818_d_n14,)
            }
        };
        (assign28870_e49819, assign28870_e49819_d_n0, assign28870_e49819_d_n2, assign28870_e49819_d_n3, assign28870_e49819_d_n4, assign28870_e49819_d_n5, assign28870_e49819_d_n6, assign28870_e49819_d_n7, assign28870_e49819_d_n8, assign28870_e49819_d_n9, assign28870_e49819_d_n10, assign28870_e49819_d_n11, assign28870_e49819_d_n13, assign28870_e49819_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28870_e49821;
        locals.var_t1_dn0 = assign28870_e49821_d_n0;
        locals.var_t1_dn2 = assign28870_e49821_d_n2;
        locals.var_t1_dn3 = assign28870_e49821_d_n3;
        locals.var_t1_dn4 = assign28870_e49821_d_n4;
        locals.var_t1_dn5 = assign28870_e49821_d_n5;
        locals.var_t1_dn6 = assign28870_e49821_d_n6;
        locals.var_t1_dn7 = assign28870_e49821_d_n7;
        locals.var_t1_dn8 = assign28870_e49821_d_n8;
        locals.var_t1_dn9 = assign28870_e49821_d_n9;
        locals.var_t1_dn10 = assign28870_e49821_d_n10;
        locals.var_t1_dn11 = assign28870_e49821_d_n11;
        locals.var_t1_dn13 = assign28870_e49821_d_n13;
        locals.var_t1_dn14 = assign28870_e49821_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign28880_e49834, assign28880_e49834_d_n0, assign28880_e49834_d_n2, assign28880_e49834_d_n3, assign28880_e49834_d_n4, assign28880_e49834_d_n5, assign28880_e49834_d_n6, assign28880_e49834_d_n7, assign28880_e49834_d_n8, assign28880_e49834_d_n9, assign28880_e49834_d_n10, assign28880_e49834_d_n11, assign28880_e49834_d_n13, assign28880_e49834_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28880_e49831: f64 = (locals.var_t1 + 0.001);
        let assign28880_e49832: f64 = (locals.var_bgislb_t / assign28880_e49831);
        (assign28880_e49832, (-((locals.var_bgislb_t * locals.var_t1_dn0) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn2) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn3) / (assign28880_e49831 * assign28880_e49831))), (((locals.var_bgislb_t_dn4 * assign28880_e49831) - (locals.var_bgislb_t * locals.var_t1_dn4)) / (assign28880_e49831 * assign28880_e49831)), (-((locals.var_bgislb_t * locals.var_t1_dn5) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn6) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn7) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn8) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn9) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn10) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn11) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn13) / (assign28880_e49831 * assign28880_e49831))), (-((locals.var_bgislb_t * locals.var_t1_dn14) / (assign28880_e49831 * assign28880_e49831))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28880_e49834;
        locals.var_t2_dn0 = assign28880_e49834_d_n0;
        locals.var_t2_dn2 = assign28880_e49834_d_n2;
        locals.var_t2_dn3 = assign28880_e49834_d_n3;
        locals.var_t2_dn4 = assign28880_e49834_d_n4;
        locals.var_t2_dn5 = assign28880_e49834_d_n5;
        locals.var_t2_dn6 = assign28880_e49834_d_n6;
        locals.var_t2_dn7 = assign28880_e49834_d_n7;
        locals.var_t2_dn8 = assign28880_e49834_d_n8;
        locals.var_t2_dn9 = assign28880_e49834_d_n9;
        locals.var_t2_dn10 = assign28880_e49834_d_n10;
        locals.var_t2_dn11 = assign28880_e49834_d_n11;
        locals.var_t2_dn13 = assign28880_e49834_d_n13;
        locals.var_t2_dn14 = assign28880_e49834_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign28890_e49845, assign28890_e49845_d_n0, assign28890_e49845_d_n2, assign28890_e49845_d_n3, assign28890_e49845_d_n4, assign28890_e49845_d_n5, assign28890_e49845_d_n6, assign28890_e49845_d_n7, assign28890_e49845_d_n8, assign28890_e49845_d_n9, assign28890_e49845_d_n10, assign28890_e49845_d_n11, assign28890_e49845_d_n13, assign28890_e49845_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28890_e49843: f64 = (locals.var_t1).powf(locals.var_pgislb_i);
        (assign28890_e49843, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn0)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn2)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn3)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn4)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn5)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn6)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn7)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn8)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn9)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn10)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn11)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn13)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_pgislb_i) as f64).is_finite() && ((locals.var_pgislb_i) as f64).fract() == 0.0 { if locals.var_pgislb_i == 0.0 { 0.0 } else { (locals.var_pgislb_i * ((locals.var_t1).powf(locals.var_pgislb_i - 1.0) * locals.var_t1_dn14)) } } else { (assign28890_e49843 * (locals.var_pgislb_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign28890_e49845;
        locals.var_t3_dn0 = assign28890_e49845_d_n0;
        locals.var_t3_dn2 = assign28890_e49845_d_n2;
        locals.var_t3_dn3 = assign28890_e49845_d_n3;
        locals.var_t3_dn4 = assign28890_e49845_d_n4;
        locals.var_t3_dn5 = assign28890_e49845_d_n5;
        locals.var_t3_dn6 = assign28890_e49845_d_n6;
        locals.var_t3_dn7 = assign28890_e49845_d_n7;
        locals.var_t3_dn8 = assign28890_e49845_d_n8;
        locals.var_t3_dn9 = assign28890_e49845_d_n9;
        locals.var_t3_dn10 = assign28890_e49845_d_n10;
        locals.var_t3_dn11 = assign28890_e49845_d_n11;
        locals.var_t3_dn13 = assign28890_e49845_d_n13;
        locals.var_t3_dn14 = assign28890_e49845_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign28900_e49859, assign28900_e49859_d_n0, assign28900_e49859_d_n2, assign28900_e49859_d_n3, assign28900_e49859_d_n4, assign28900_e49859_d_n5, assign28900_e49859_d_n6, assign28900_e49859_d_n7, assign28900_e49859_d_n8, assign28900_e49859_d_n9, assign28900_e49859_d_n10, assign28900_e49859_d_n11, assign28900_e49859_d_n13, assign28900_e49859_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28900_e49853: f64 = (-locals.var_ves_jct);
        let assign28900_e49855: f64 = (assign28900_e49853 * locals.var_ves_jct);
        let assign28900_e49857: f64 = (assign28900_e49855 * locals.var_ves_jct);
        (assign28900_e49857, 0.0, 0.0, (((((-locals.var_ves_jct_dn3) * locals.var_ves_jct) + (assign28900_e49853 * locals.var_ves_jct_dn3)) * locals.var_ves_jct) + (assign28900_e49855 * locals.var_ves_jct_dn3)), 0.0, 0.0, (((((-locals.var_ves_jct_dn6) * locals.var_ves_jct) + (assign28900_e49853 * locals.var_ves_jct_dn6)) * locals.var_ves_jct) + (assign28900_e49855 * locals.var_ves_jct_dn6)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign28900_e49859;
        locals.var_t4_dn0 = assign28900_e49859_d_n0;
        locals.var_t4_dn2 = assign28900_e49859_d_n2;
        locals.var_t4_dn3 = assign28900_e49859_d_n3;
        locals.var_t4_dn4 = assign28900_e49859_d_n4;
        locals.var_t4_dn5 = assign28900_e49859_d_n5;
        locals.var_t4_dn6 = assign28900_e49859_d_n6;
        locals.var_t4_dn7 = assign28900_e49859_d_n7;
        locals.var_t4_dn8 = assign28900_e49859_d_n8;
        locals.var_t4_dn9 = assign28900_e49859_d_n9;
        locals.var_t4_dn10 = assign28900_e49859_d_n10;
        locals.var_t4_dn11 = assign28900_e49859_d_n11;
        locals.var_t4_dn13 = assign28900_e49859_d_n13;
        locals.var_t4_dn14 = assign28900_e49859_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign28910_e49873, assign28910_e49873_d_n0, assign28910_e49873_d_n2, assign28910_e49873_d_n3, assign28910_e49873_d_n4, assign28910_e49873_d_n5, assign28910_e49873_d_n6, assign28910_e49873_d_n7, assign28910_e49873_d_n8, assign28910_e49873_d_n9, assign28910_e49873_d_n10, assign28910_e49873_d_n11, assign28910_e49873_d_n13, assign28910_e49873_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28910_e49868: f64 = (locals.var_t4).abs();
        let assign28910_e49869: f64 = (locals.var_cgislb_i + assign28910_e49868);
        let assign28910_e49871: f64 = (assign28910_e49869 + 1e-5);
        (assign28910_e49871, if locals.var_t4 >= 0.0 { locals.var_t4_dn0 } else { (-locals.var_t4_dn0) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn2 } else { (-locals.var_t4_dn2) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn3 } else { (-locals.var_t4_dn3) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn4 } else { (-locals.var_t4_dn4) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn5 } else { (-locals.var_t4_dn5) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn6 } else { (-locals.var_t4_dn6) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn7 } else { (-locals.var_t4_dn7) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn8 } else { (-locals.var_t4_dn8) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn9 } else { (-locals.var_t4_dn9) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn10 } else { (-locals.var_t4_dn10) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn11 } else { (-locals.var_t4_dn11) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn13 } else { (-locals.var_t4_dn13) }, if locals.var_t4 >= 0.0 { locals.var_t4_dn14 } else { (-locals.var_t4_dn14) },)
    } else {
        (locals.var_t4a, locals.var_t4a_dn0, locals.var_t4a_dn2, locals.var_t4a_dn3, locals.var_t4a_dn4, locals.var_t4a_dn5, locals.var_t4a_dn6, locals.var_t4a_dn7, locals.var_t4a_dn8, locals.var_t4a_dn9, locals.var_t4a_dn10, locals.var_t4a_dn11, locals.var_t4a_dn13, locals.var_t4a_dn14,)
    }
};
        locals.var_t4a = assign28910_e49873;
        locals.var_t4a_dn0 = assign28910_e49873_d_n0;
        locals.var_t4a_dn2 = assign28910_e49873_d_n2;
        locals.var_t4a_dn3 = assign28910_e49873_d_n3;
        locals.var_t4a_dn4 = assign28910_e49873_d_n4;
        locals.var_t4a_dn5 = assign28910_e49873_d_n5;
        locals.var_t4a_dn6 = assign28910_e49873_d_n6;
        locals.var_t4a_dn7 = assign28910_e49873_d_n7;
        locals.var_t4a_dn8 = assign28910_e49873_d_n8;
        locals.var_t4a_dn9 = assign28910_e49873_d_n9;
        locals.var_t4a_dn10 = assign28910_e49873_d_n10;
        locals.var_t4a_dn11 = assign28910_e49873_d_n11;
        locals.var_t4a_dn13 = assign28910_e49873_d_n13;
        locals.var_t4a_dn14 = assign28910_e49873_d_n14;
        locals.var_t4a_rv = 0.0;

        let (assign28920_e49931, assign28920_e49931_d_n0, assign28920_e49931_d_n2, assign28920_e49931_d_n3, assign28920_e49931_d_n4, assign28920_e49931_d_n5, assign28920_e49931_d_n6, assign28920_e49931_d_n7, assign28920_e49931_d_n8, assign28920_e49931_d_n9, assign28920_e49931_d_n10, assign28920_e49931_d_n11, assign28920_e49931_d_n13, assign28920_e49931_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28920_e49882: f64 = (locals.var_t4 / locals.var_t4a);
        let assign28920_e49884: f64 = (-10000.0);
        let assign28920_e49886: f64 = (assign28920_e49884 * 1e-6);
        let (assign28920_e49927, assign28920_e49927_d_n0, assign28920_e49927_d_n2, assign28920_e49927_d_n3, assign28920_e49927_d_n4, assign28920_e49927_d_n5, assign28920_e49927_d_n6, assign28920_e49927_d_n7, assign28920_e49927_d_n8, assign28920_e49927_d_n9, assign28920_e49927_d_n10, assign28920_e49927_d_n11, assign28920_e49927_d_n13, assign28920_e49927_d_n14,) = {
            if (!(assign28920_e49882 < assign28920_e49886)) {
                let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4a;
                let assign28920_e49892: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28920_e49895: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28920_e49898: f64 = (locals.var_t4 * __rspice_inv_cse_0);
                let assign28920_e49899: f64 = (assign28920_e49895 * assign28920_e49898);
                let assign28920_e49902: f64 = (4.0 * 1e-6);
                let assign28920_e49904: f64 = (assign28920_e49902 * 1e-6);
                let assign28920_e49905: f64 = (assign28920_e49899 + assign28920_e49904);
                let assign28920_e49906: f64 = (assign28920_e49905).sqrt();
                let assign28920_e49907: f64 = (assign28920_e49892 + assign28920_e49906);
                let assign28920_e49908: f64 = (0.5 * assign28920_e49907);
                (assign28920_e49908, (0.5 * ((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))), (0.5 * ((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) + ((((((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)) * assign28920_e49898) + (assign28920_e49895 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a)))) / (2.0 * assign28920_e49906)))),)
            } else {
                let assign28920_e49911: f64 = (locals.var_t4 / locals.var_t4a);
                let assign28920_e49913: f64 = (-10000.0);
                let assign28920_e49915: f64 = (assign28920_e49913 * 1e-6);
                let (assign28920_e49926, assign28920_e49926_d_n0, assign28920_e49926_d_n2, assign28920_e49926_d_n3, assign28920_e49926_d_n4, assign28920_e49926_d_n5, assign28920_e49926_d_n6, assign28920_e49926_d_n7, assign28920_e49926_d_n8, assign28920_e49926_d_n9, assign28920_e49926_d_n10, assign28920_e49926_d_n11, assign28920_e49926_d_n13, assign28920_e49926_d_n14,) = {
                    if (assign28920_e49911 < assign28920_e49915) {
                        let assign28920_e49918: f64 = (-1e-6);
                        let assign28920_e49920: f64 = (assign28920_e49918 * 1e-6);
                        let assign28920_e49923: f64 = (locals.var_t4 / locals.var_t4a);
                        let assign28920_e49924: f64 = (assign28920_e49920 / assign28920_e49923);
                        (assign28920_e49924, (-((assign28920_e49920 * (((locals.var_t4_dn0 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn0)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn2 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn2)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn3 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn3)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn4 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn4)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn5 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn5)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn6 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn6)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn7 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn7)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn8 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn8)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn9 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn9)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn10 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn10)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn11 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn11)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn13 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn13)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))), (-((assign28920_e49920 * (((locals.var_t4_dn14 * locals.var_t4a) - (locals.var_t4 * locals.var_t4a_dn14)) / (locals.var_t4a * locals.var_t4a))) / (assign28920_e49923 * assign28920_e49923))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign28920_e49926, assign28920_e49926_d_n0, assign28920_e49926_d_n2, assign28920_e49926_d_n3, assign28920_e49926_d_n4, assign28920_e49926_d_n5, assign28920_e49926_d_n6, assign28920_e49926_d_n7, assign28920_e49926_d_n8, assign28920_e49926_d_n9, assign28920_e49926_d_n10, assign28920_e49926_d_n11, assign28920_e49926_d_n13, assign28920_e49926_d_n14,)
            }
        };
        let assign28920_e49929: f64 = (assign28920_e49927 - 1e-6);
        (assign28920_e49929, assign28920_e49927_d_n0, assign28920_e49927_d_n2, assign28920_e49927_d_n3, assign28920_e49927_d_n4, assign28920_e49927_d_n5, assign28920_e49927_d_n6, assign28920_e49927_d_n7, assign28920_e49927_d_n8, assign28920_e49927_d_n9, assign28920_e49927_d_n10, assign28920_e49927_d_n11, assign28920_e49927_d_n13, assign28920_e49927_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign28920_e49931;
        locals.var_t5_dn0 = assign28920_e49931_d_n0;
        locals.var_t5_dn2 = assign28920_e49931_d_n2;
        locals.var_t5_dn3 = assign28920_e49931_d_n3;
        locals.var_t5_dn4 = assign28920_e49931_d_n4;
        locals.var_t5_dn5 = assign28920_e49931_d_n5;
        locals.var_t5_dn6 = assign28920_e49931_d_n6;
        locals.var_t5_dn7 = assign28920_e49931_d_n7;
        locals.var_t5_dn8 = assign28920_e49931_d_n8;
        locals.var_t5_dn9 = assign28920_e49931_d_n9;
        locals.var_t5_dn10 = assign28920_e49931_d_n10;
        locals.var_t5_dn11 = assign28920_e49931_d_n11;
        locals.var_t5_dn13 = assign28920_e49931_d_n13;
        locals.var_t5_dn14 = assign28920_e49931_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign28930_e49950, assign28930_e49950_d_n0, assign28930_e49950_d_n2, assign28930_e49950_d_n3, assign28930_e49950_d_n4, assign28930_e49950_d_n5, assign28930_e49950_d_n6, assign28930_e49950_d_n7, assign28930_e49950_d_n8, assign28930_e49950_d_n9, assign28930_e49950_d_n10, assign28930_e49950_d_n11, assign28930_e49950_d_n13, assign28930_e49950_d_n14,) = {
    if (((locals.var_guard454 != 0.0) && (locals.var_guard466 != 0.0)) && (locals.var_guard467 == 0.0)) {
        let assign28930_e49940: f64 = (locals.var_agislb_i * locals.var_weffb);
        let assign28930_e49942: f64 = (assign28930_e49940 * locals.var_t3);
        let assign28930_e49944: f64 = (-locals.var_t2);
        let assign28930_e49945: f64 = { let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign28930_e49946: f64 = (assign28930_e49942 * assign28930_e49945);
        let assign28930_e49948: f64 = (assign28930_e49946 * locals.var_t5);
        (assign28930_e49948, (((((assign28930_e49940 * locals.var_t3_dn0) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn0)), (((((assign28930_e49940 * locals.var_t3_dn2) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn2)), (((((assign28930_e49940 * locals.var_t3_dn3) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn3)), (((((assign28930_e49940 * locals.var_t3_dn4) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn4)), (((((assign28930_e49940 * locals.var_t3_dn5) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn5)), (((((assign28930_e49940 * locals.var_t3_dn6) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn6)), (((((assign28930_e49940 * locals.var_t3_dn7) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn7)), (((((assign28930_e49940 * locals.var_t3_dn8) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn8)), (((((assign28930_e49940 * locals.var_t3_dn9) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn9)), (((((assign28930_e49940 * locals.var_t3_dn10) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn10)), (((((assign28930_e49940 * locals.var_t3_dn11) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn11)), (((((assign28930_e49940 * locals.var_t3_dn13) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn13)), (((((assign28930_e49940 * locals.var_t3_dn14) * assign28930_e49945) + (assign28930_e49942 * ({ let limited_exp_arg = assign28930_e49944; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_t5) + (assign28930_e49946 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign28930_e49950;
        locals.var_t7_dn0 = assign28930_e49950_d_n0;
        locals.var_t7_dn2 = assign28930_e49950_d_n2;
        locals.var_t7_dn3 = assign28930_e49950_d_n3;
        locals.var_t7_dn4 = assign28930_e49950_d_n4;
        locals.var_t7_dn5 = assign28930_e49950_d_n5;
        locals.var_t7_dn6 = assign28930_e49950_d_n6;
        locals.var_t7_dn7 = assign28930_e49950_d_n7;
        locals.var_t7_dn8 = assign28930_e49950_d_n8;
        locals.var_t7_dn9 = assign28930_e49950_d_n9;
        locals.var_t7_dn10 = assign28930_e49950_d_n10;
        locals.var_t7_dn11 = assign28930_e49950_d_n11;
        locals.var_t7_dn13 = assign28930_e49950_d_n13;
        locals.var_t7_dn14 = assign28930_e49950_d_n14;
        locals.var_t7_rv = 0.0;

        let assign28990_e49982: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign28990_e49982;
        locals.var_guard469_rv = 0.0;

        let assign29000_e49985: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard470 = assign29000_e49985;
        locals.var_guard470_rv = 0.0;

        let assign29010_e49988: f64 = if locals.var_ves_jct < locals.var_vjsmrev { 1.0 } else { 0.0 };
        locals.var_guard471 = assign29010_e49988;
        locals.var_guard471_rv = 0.0;

        let (assign29020_e49998, assign29020_e49998_d_n0, assign29020_e49998_d_n2, assign29020_e49998_d_n3, assign29020_e49998_d_n4, assign29020_e49998_d_n5, assign29020_e49998_d_n6, assign29020_e49998_d_n7, assign29020_e49998_d_n8, assign29020_e49998_d_n9, assign29020_e49998_d_n10, assign29020_e49998_d_n11, assign29020_e49998_d_n13, assign29020_e49998_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign29020_e49996: f64 = (locals.var_ves_jct / locals.var_nvtms);
        (assign29020_e49996, 0.0, 0.0, (locals.var_ves_jct_dn3 / locals.var_nvtms), (-((locals.var_ves_jct * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms))), 0.0, (locals.var_ves_jct_dn6 / locals.var_nvtms), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29020_e49998;
        locals.var_t0_dn0 = assign29020_e49998_d_n0;
        locals.var_t0_dn2 = assign29020_e49998_d_n2;
        locals.var_t0_dn3 = assign29020_e49998_d_n3;
        locals.var_t0_dn4 = assign29020_e49998_d_n4;
        locals.var_t0_dn5 = assign29020_e49998_d_n5;
        locals.var_t0_dn6 = assign29020_e49998_d_n6;
        locals.var_t0_dn7 = assign29020_e49998_d_n7;
        locals.var_t0_dn8 = assign29020_e49998_d_n8;
        locals.var_t0_dn9 = assign29020_e49998_d_n9;
        locals.var_t0_dn10 = assign29020_e49998_d_n10;
        locals.var_t0_dn11 = assign29020_e49998_d_n11;
        locals.var_t0_dn13 = assign29020_e49998_d_n13;
        locals.var_t0_dn14 = assign29020_e49998_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29030_e50009, assign29030_e50009_d_n0, assign29030_e50009_d_n2, assign29030_e50009_d_n3, assign29030_e50009_d_n4, assign29030_e50009_d_n5, assign29030_e50009_d_n6, assign29030_e50009_d_n7, assign29030_e50009_d_n8, assign29030_e50009_d_n9, assign29030_e50009_d_n10, assign29030_e50009_d_n11, assign29030_e50009_d_n13, assign29030_e50009_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign29030_e50005: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29030_e50007: f64 = (assign29030_e50005 - 1.0);
        (assign29030_e50007, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29030_e50009;
        locals.var_t1_dn0 = assign29030_e50009_d_n0;
        locals.var_t1_dn2 = assign29030_e50009_d_n2;
        locals.var_t1_dn3 = assign29030_e50009_d_n3;
        locals.var_t1_dn4 = assign29030_e50009_d_n4;
        locals.var_t1_dn5 = assign29030_e50009_d_n5;
        locals.var_t1_dn6 = assign29030_e50009_d_n6;
        locals.var_t1_dn7 = assign29030_e50009_d_n7;
        locals.var_t1_dn8 = assign29030_e50009_d_n8;
        locals.var_t1_dn9 = assign29030_e50009_d_n9;
        locals.var_t1_dn10 = assign29030_e50009_d_n10;
        locals.var_t1_dn11 = assign29030_e50009_d_n11;
        locals.var_t1_dn13 = assign29030_e50009_d_n13;
        locals.var_t1_dn14 = assign29030_e50009_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29040_e50023, assign29040_e50023_d_n0, assign29040_e50023_d_n2, assign29040_e50023_d_n3, assign29040_e50023_d_n4, assign29040_e50023_d_n5, assign29040_e50023_d_n6, assign29040_e50023_d_n7, assign29040_e50023_d_n8, assign29040_e50023_d_n9, assign29040_e50023_d_n10, assign29040_e50023_d_n11, assign29040_e50023_d_n13, assign29040_e50023_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        let assign29040_e50019: f64 = (locals.var_ves_jct - locals.var_vjsmrev);
        let assign29040_e50020: f64 = (locals.var_sslprev * assign29040_e50019);
        let assign29040_e50021: f64 = (locals.var_ivjsmrev + assign29040_e50020);
        (assign29040_e50021, (locals.var_ivjsmrev_dn0 + ((locals.var_sslprev_dn0 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn0)))), (locals.var_ivjsmrev_dn2 + ((locals.var_sslprev_dn2 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn2)))), (locals.var_ivjsmrev_dn3 + ((locals.var_sslprev_dn3 * assign29040_e50019) + (locals.var_sslprev * (locals.var_ves_jct_dn3 - locals.var_vjsmrev_dn3)))), (locals.var_ivjsmrev_dn4 + ((locals.var_sslprev_dn4 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn4)))), (locals.var_ivjsmrev_dn5 + ((locals.var_sslprev_dn5 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn5)))), (locals.var_ivjsmrev_dn6 + ((locals.var_sslprev_dn6 * assign29040_e50019) + (locals.var_sslprev * (locals.var_ves_jct_dn6 - locals.var_vjsmrev_dn6)))), (locals.var_ivjsmrev_dn7 + ((locals.var_sslprev_dn7 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn7)))), (locals.var_ivjsmrev_dn8 + ((locals.var_sslprev_dn8 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn8)))), (locals.var_ivjsmrev_dn9 + ((locals.var_sslprev_dn9 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn9)))), (locals.var_ivjsmrev_dn10 + ((locals.var_sslprev_dn10 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn10)))), (locals.var_ivjsmrev_dn11 + ((locals.var_sslprev_dn11 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn11)))), (locals.var_ivjsmrev_dn13 + ((locals.var_sslprev_dn13 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn13)))), (locals.var_ivjsmrev_dn14 + ((locals.var_sslprev_dn14 * assign29040_e50019) + (locals.var_sslprev * (-locals.var_vjsmrev_dn14)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29040_e50023;
        locals.var_t2_dn0 = assign29040_e50023_d_n0;
        locals.var_t2_dn2 = assign29040_e50023_d_n2;
        locals.var_t2_dn3 = assign29040_e50023_d_n3;
        locals.var_t2_dn4 = assign29040_e50023_d_n4;
        locals.var_t2_dn5 = assign29040_e50023_d_n5;
        locals.var_t2_dn6 = assign29040_e50023_d_n6;
        locals.var_t2_dn7 = assign29040_e50023_d_n7;
        locals.var_t2_dn8 = assign29040_e50023_d_n8;
        locals.var_t2_dn9 = assign29040_e50023_d_n9;
        locals.var_t2_dn10 = assign29040_e50023_d_n10;
        locals.var_t2_dn11 = assign29040_e50023_d_n11;
        locals.var_t2_dn13 = assign29040_e50023_d_n13;
        locals.var_t2_dn14 = assign29040_e50023_d_n14;
        locals.var_t2_rv = 0.0;

        let assign29060_e50036: f64 = if locals.var_ves_jct <= locals.var_vjsmfwd { 1.0 } else { 0.0 };
        locals.var_guard472 = assign29060_e50036;
        locals.var_guard472_rv = 0.0;

        let (assign29070_e50049, assign29070_e50049_d_n0, assign29070_e50049_d_n2, assign29070_e50049_d_n3, assign29070_e50049_d_n4, assign29070_e50049_d_n5, assign29070_e50049_d_n6, assign29070_e50049_d_n7, assign29070_e50049_d_n8, assign29070_e50049_d_n9, assign29070_e50049_d_n10, assign29070_e50049_d_n11, assign29070_e50049_d_n13, assign29070_e50049_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign29070_e50047: f64 = (locals.var_ves_jct / locals.var_nvtms);
        (assign29070_e50047, 0.0, 0.0, (locals.var_ves_jct_dn3 / locals.var_nvtms), (-((locals.var_ves_jct * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms))), 0.0, (locals.var_ves_jct_dn6 / locals.var_nvtms), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29070_e50049;
        locals.var_t0_dn0 = assign29070_e50049_d_n0;
        locals.var_t0_dn2 = assign29070_e50049_d_n2;
        locals.var_t0_dn3 = assign29070_e50049_d_n3;
        locals.var_t0_dn4 = assign29070_e50049_d_n4;
        locals.var_t0_dn5 = assign29070_e50049_d_n5;
        locals.var_t0_dn6 = assign29070_e50049_d_n6;
        locals.var_t0_dn7 = assign29070_e50049_d_n7;
        locals.var_t0_dn8 = assign29070_e50049_d_n8;
        locals.var_t0_dn9 = assign29070_e50049_d_n9;
        locals.var_t0_dn10 = assign29070_e50049_d_n10;
        locals.var_t0_dn11 = assign29070_e50049_d_n11;
        locals.var_t0_dn13 = assign29070_e50049_d_n13;
        locals.var_t0_dn14 = assign29070_e50049_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29080_e50064, assign29080_e50064_d_n0, assign29080_e50064_d_n2, assign29080_e50064_d_n3, assign29080_e50064_d_n4, assign29080_e50064_d_n5, assign29080_e50064_d_n6, assign29080_e50064_d_n7, assign29080_e50064_d_n8, assign29080_e50064_d_n9, assign29080_e50064_d_n10, assign29080_e50064_d_n11, assign29080_e50064_d_n13, assign29080_e50064_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign29080_e50060: f64 = (p.p1626 + locals.var_ves_jct);
        let assign29080_e50062: f64 = (assign29080_e50060 / locals.var_nvtms);
        (assign29080_e50062, 0.0, 0.0, (locals.var_ves_jct_dn3 / locals.var_nvtms), (-((assign29080_e50060 * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms))), 0.0, (locals.var_ves_jct_dn6 / locals.var_nvtms), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29080_e50064;
        locals.var_t1_dn0 = assign29080_e50064_d_n0;
        locals.var_t1_dn2 = assign29080_e50064_d_n2;
        locals.var_t1_dn3 = assign29080_e50064_d_n3;
        locals.var_t1_dn4 = assign29080_e50064_d_n4;
        locals.var_t1_dn5 = assign29080_e50064_d_n5;
        locals.var_t1_dn6 = assign29080_e50064_d_n6;
        locals.var_t1_dn7 = assign29080_e50064_d_n7;
        locals.var_t1_dn8 = assign29080_e50064_d_n8;
        locals.var_t1_dn9 = assign29080_e50064_d_n9;
        locals.var_t1_dn10 = assign29080_e50064_d_n10;
        locals.var_t1_dn11 = assign29080_e50064_d_n11;
        locals.var_t1_dn13 = assign29080_e50064_d_n13;
        locals.var_t1_dn14 = assign29080_e50064_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29090_e50077, assign29090_e50077_d_n0, assign29090_e50077_d_n2, assign29090_e50077_d_n3, assign29090_e50077_d_n4, assign29090_e50077_d_n5, assign29090_e50077_d_n6, assign29090_e50077_d_n7, assign29090_e50077_d_n8, assign29090_e50077_d_n9, assign29090_e50077_d_n10, assign29090_e50077_d_n11, assign29090_e50077_d_n13, assign29090_e50077_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        let assign29090_e50074: f64 = (-locals.var_t1);
        let assign29090_e50075: f64 = { let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign29090_e50075, ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13)), ({ let limited_exp_arg = assign29090_e50074; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29090_e50077;
        locals.var_t2_dn0 = assign29090_e50077_d_n0;
        locals.var_t2_dn2 = assign29090_e50077_d_n2;
        locals.var_t2_dn3 = assign29090_e50077_d_n3;
        locals.var_t2_dn4 = assign29090_e50077_d_n4;
        locals.var_t2_dn5 = assign29090_e50077_d_n5;
        locals.var_t2_dn6 = assign29090_e50077_d_n6;
        locals.var_t2_dn7 = assign29090_e50077_d_n7;
        locals.var_t2_dn8 = assign29090_e50077_d_n8;
        locals.var_t2_dn9 = assign29090_e50077_d_n9;
        locals.var_t2_dn10 = assign29090_e50077_d_n10;
        locals.var_t2_dn11 = assign29090_e50077_d_n11;
        locals.var_t2_dn13 = assign29090_e50077_d_n13;
        locals.var_t2_dn14 = assign29090_e50077_d_n14;
        locals.var_t2_rv = 0.0;

        let assign29130_e50127: f64 = if locals.var_jtss_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign29130_e50127;
        locals.var_guard473_rv = 0.0;

        let assign29140_e50130: f64 = (p.p1643 - locals.var_ves_jct);
        let assign29140_e50133: f64 = (p.p1643 * 0.001);
        let assign29140_e50134: f64 = if assign29140_e50130 < assign29140_e50133 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign29140_e50134;
        locals.var_guard474_rv = 0.0;

        let (assign29150_e50147, assign29150_e50147_d_n0, assign29150_e50147_d_n2, assign29150_e50147_d_n3, assign29150_e50147_d_n4, assign29150_e50147_d_n5, assign29150_e50147_d_n6, assign29150_e50147_d_n7, assign29150_e50147_d_n8, assign29150_e50147_d_n9, assign29150_e50147_d_n10, assign29150_e50147_d_n11, assign29150_e50147_d_n13, assign29150_e50147_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign29150_e50141: f64 = (-locals.var_ves_jct);
        let assign29150_e50143: f64 = (assign29150_e50141 / locals.var_vtm0);
        let assign29150_e50145: f64 = (assign29150_e50143 / locals.var_njts_t);
        (assign29150_e50145, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njts_t), (-((assign29150_e50143 * locals.var_njts_t_dn4) / (locals.var_njts_t * locals.var_njts_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29150_e50147;
        locals.var_t0_dn0 = assign29150_e50147_d_n0;
        locals.var_t0_dn2 = assign29150_e50147_d_n2;
        locals.var_t0_dn3 = assign29150_e50147_d_n3;
        locals.var_t0_dn4 = assign29150_e50147_d_n4;
        locals.var_t0_dn5 = assign29150_e50147_d_n5;
        locals.var_t0_dn6 = assign29150_e50147_d_n6;
        locals.var_t0_dn7 = assign29150_e50147_d_n7;
        locals.var_t0_dn8 = assign29150_e50147_d_n8;
        locals.var_t0_dn9 = assign29150_e50147_d_n9;
        locals.var_t0_dn10 = assign29150_e50147_d_n10;
        locals.var_t0_dn11 = assign29150_e50147_d_n11;
        locals.var_t0_dn13 = assign29150_e50147_d_n13;
        locals.var_t0_dn14 = assign29150_e50147_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29160_e50160, assign29160_e50160_d_n0, assign29160_e50160_d_n2, assign29160_e50160_d_n3, assign29160_e50160_d_n4, assign29160_e50160_d_n5, assign29160_e50160_d_n6, assign29160_e50160_d_n7, assign29160_e50160_d_n8, assign29160_e50160_d_n9, assign29160_e50160_d_n10, assign29160_e50160_d_n11, assign29160_e50160_d_n13, assign29160_e50160_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign29160_e50155: f64 = (locals.var_t0 * 1000.0);
        let assign29160_e50156: f64 = { let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29160_e50158: f64 = (assign29160_e50156 - 1.0);
        (assign29160_e50158, ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29160_e50155; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29160_e50160;
        locals.var_t1_dn0 = assign29160_e50160_d_n0;
        locals.var_t1_dn2 = assign29160_e50160_d_n2;
        locals.var_t1_dn3 = assign29160_e50160_d_n3;
        locals.var_t1_dn4 = assign29160_e50160_d_n4;
        locals.var_t1_dn5 = assign29160_e50160_d_n5;
        locals.var_t1_dn6 = assign29160_e50160_d_n6;
        locals.var_t1_dn7 = assign29160_e50160_d_n7;
        locals.var_t1_dn8 = assign29160_e50160_d_n8;
        locals.var_t1_dn9 = assign29160_e50160_d_n9;
        locals.var_t1_dn10 = assign29160_e50160_d_n10;
        locals.var_t1_dn11 = assign29160_e50160_d_n11;
        locals.var_t1_dn13 = assign29160_e50160_d_n13;
        locals.var_t1_dn14 = assign29160_e50160_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29180_e50188, assign29180_e50188_d_n0, assign29180_e50188_d_n2, assign29180_e50188_d_n3, assign29180_e50188_d_n4, assign29180_e50188_d_n5, assign29180_e50188_d_n6, assign29180_e50188_d_n7, assign29180_e50188_d_n8, assign29180_e50188_d_n9, assign29180_e50188_d_n10, assign29180_e50188_d_n11, assign29180_e50188_d_n13, assign29180_e50188_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 == 0.0)) {
        let assign29180_e50182: f64 = (-locals.var_ves_jct);
        let assign29180_e50184: f64 = (assign29180_e50182 / locals.var_vtm0);
        let assign29180_e50186: f64 = (assign29180_e50184 / locals.var_njts_t);
        (assign29180_e50186, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njts_t), (-((assign29180_e50184 * locals.var_njts_t_dn4) / (locals.var_njts_t * locals.var_njts_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29180_e50188;
        locals.var_t0_dn0 = assign29180_e50188_d_n0;
        locals.var_t0_dn2 = assign29180_e50188_d_n2;
        locals.var_t0_dn3 = assign29180_e50188_d_n3;
        locals.var_t0_dn4 = assign29180_e50188_d_n4;
        locals.var_t0_dn5 = assign29180_e50188_d_n5;
        locals.var_t0_dn6 = assign29180_e50188_d_n6;
        locals.var_t0_dn7 = assign29180_e50188_d_n7;
        locals.var_t0_dn8 = assign29180_e50188_d_n8;
        locals.var_t0_dn9 = assign29180_e50188_d_n9;
        locals.var_t0_dn10 = assign29180_e50188_d_n10;
        locals.var_t0_dn11 = assign29180_e50188_d_n11;
        locals.var_t0_dn13 = assign29180_e50188_d_n13;
        locals.var_t0_dn14 = assign29180_e50188_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29190_e50206, assign29190_e50206_d_n0, assign29190_e50206_d_n2, assign29190_e50206_d_n3, assign29190_e50206_d_n4, assign29190_e50206_d_n5, assign29190_e50206_d_n6, assign29190_e50206_d_n7, assign29190_e50206_d_n8, assign29190_e50206_d_n9, assign29190_e50206_d_n10, assign29190_e50206_d_n11, assign29190_e50206_d_n13, assign29190_e50206_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 == 0.0)) {
        let assign29190_e50197: f64 = (locals.var_t0 * p.p1643);
        let assign29190_e50200: f64 = (p.p1643 - locals.var_ves_jct);
        let assign29190_e50201: f64 = (assign29190_e50197 / assign29190_e50200);
        let assign29190_e50202: f64 = { let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29190_e50204: f64 = (assign29190_e50202 - 1.0);
        (assign29190_e50204, ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1643) * assign29190_e50200) - (assign29190_e50197 * (-locals.var_ves_jct_dn3))) / (assign29190_e50200 * assign29190_e50200))), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn6 * p.p1643) * assign29190_e50200) - (assign29190_e50197 * (-locals.var_ves_jct_dn6))) / (assign29190_e50200 * assign29190_e50200))), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1643) / assign29190_e50200)), ({ let limited_exp_arg = assign29190_e50201; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1643) / assign29190_e50200)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29190_e50206;
        locals.var_t1_dn0 = assign29190_e50206_d_n0;
        locals.var_t1_dn2 = assign29190_e50206_d_n2;
        locals.var_t1_dn3 = assign29190_e50206_d_n3;
        locals.var_t1_dn4 = assign29190_e50206_d_n4;
        locals.var_t1_dn5 = assign29190_e50206_d_n5;
        locals.var_t1_dn6 = assign29190_e50206_d_n6;
        locals.var_t1_dn7 = assign29190_e50206_d_n7;
        locals.var_t1_dn8 = assign29190_e50206_d_n8;
        locals.var_t1_dn9 = assign29190_e50206_d_n9;
        locals.var_t1_dn10 = assign29190_e50206_d_n10;
        locals.var_t1_dn11 = assign29190_e50206_d_n11;
        locals.var_t1_dn13 = assign29190_e50206_d_n13;
        locals.var_t1_dn14 = assign29190_e50206_d_n14;
        locals.var_t1_rv = 0.0;

        let assign29210_e50224: f64 = if locals.var_jtssws_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign29210_e50224;
        locals.var_guard475_rv = 0.0;

        let assign29220_e50227: f64 = (p.p1645 - locals.var_ves_jct);
        let assign29220_e50230: f64 = (p.p1645 * 0.001);
        let assign29220_e50231: f64 = if assign29220_e50227 < assign29220_e50230 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign29220_e50231;
        locals.var_guard476_rv = 0.0;

        let (assign29230_e50244, assign29230_e50244_d_n0, assign29230_e50244_d_n2, assign29230_e50244_d_n3, assign29230_e50244_d_n4, assign29230_e50244_d_n5, assign29230_e50244_d_n6, assign29230_e50244_d_n7, assign29230_e50244_d_n8, assign29230_e50244_d_n9, assign29230_e50244_d_n10, assign29230_e50244_d_n11, assign29230_e50244_d_n13, assign29230_e50244_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign29230_e50238: f64 = (-locals.var_ves_jct);
        let assign29230_e50240: f64 = (assign29230_e50238 / locals.var_vtm0);
        let assign29230_e50242: f64 = (assign29230_e50240 / locals.var_njtssw_t);
        (assign29230_e50242, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtssw_t), (-((assign29230_e50240 * locals.var_njtssw_t_dn4) / (locals.var_njtssw_t * locals.var_njtssw_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29230_e50244;
        locals.var_t0_dn0 = assign29230_e50244_d_n0;
        locals.var_t0_dn2 = assign29230_e50244_d_n2;
        locals.var_t0_dn3 = assign29230_e50244_d_n3;
        locals.var_t0_dn4 = assign29230_e50244_d_n4;
        locals.var_t0_dn5 = assign29230_e50244_d_n5;
        locals.var_t0_dn6 = assign29230_e50244_d_n6;
        locals.var_t0_dn7 = assign29230_e50244_d_n7;
        locals.var_t0_dn8 = assign29230_e50244_d_n8;
        locals.var_t0_dn9 = assign29230_e50244_d_n9;
        locals.var_t0_dn10 = assign29230_e50244_d_n10;
        locals.var_t0_dn11 = assign29230_e50244_d_n11;
        locals.var_t0_dn13 = assign29230_e50244_d_n13;
        locals.var_t0_dn14 = assign29230_e50244_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29240_e50257, assign29240_e50257_d_n0, assign29240_e50257_d_n2, assign29240_e50257_d_n3, assign29240_e50257_d_n4, assign29240_e50257_d_n5, assign29240_e50257_d_n6, assign29240_e50257_d_n7, assign29240_e50257_d_n8, assign29240_e50257_d_n9, assign29240_e50257_d_n10, assign29240_e50257_d_n11, assign29240_e50257_d_n13, assign29240_e50257_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign29240_e50252: f64 = (locals.var_t0 * 1000.0);
        let assign29240_e50253: f64 = { let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29240_e50255: f64 = (assign29240_e50253 - 1.0);
        (assign29240_e50255, ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29240_e50252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29240_e50257;
        locals.var_t1_dn0 = assign29240_e50257_d_n0;
        locals.var_t1_dn2 = assign29240_e50257_d_n2;
        locals.var_t1_dn3 = assign29240_e50257_d_n3;
        locals.var_t1_dn4 = assign29240_e50257_d_n4;
        locals.var_t1_dn5 = assign29240_e50257_d_n5;
        locals.var_t1_dn6 = assign29240_e50257_d_n6;
        locals.var_t1_dn7 = assign29240_e50257_d_n7;
        locals.var_t1_dn8 = assign29240_e50257_d_n8;
        locals.var_t1_dn9 = assign29240_e50257_d_n9;
        locals.var_t1_dn10 = assign29240_e50257_d_n10;
        locals.var_t1_dn11 = assign29240_e50257_d_n11;
        locals.var_t1_dn13 = assign29240_e50257_d_n13;
        locals.var_t1_dn14 = assign29240_e50257_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29260_e50285, assign29260_e50285_d_n0, assign29260_e50285_d_n2, assign29260_e50285_d_n3, assign29260_e50285_d_n4, assign29260_e50285_d_n5, assign29260_e50285_d_n6, assign29260_e50285_d_n7, assign29260_e50285_d_n8, assign29260_e50285_d_n9, assign29260_e50285_d_n10, assign29260_e50285_d_n11, assign29260_e50285_d_n13, assign29260_e50285_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 == 0.0)) {
        let assign29260_e50279: f64 = (-locals.var_ves_jct);
        let assign29260_e50281: f64 = (assign29260_e50279 / locals.var_vtm0);
        let assign29260_e50283: f64 = (assign29260_e50281 / locals.var_njtssw_t);
        (assign29260_e50283, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtssw_t), (-((assign29260_e50281 * locals.var_njtssw_t_dn4) / (locals.var_njtssw_t * locals.var_njtssw_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29260_e50285;
        locals.var_t0_dn0 = assign29260_e50285_d_n0;
        locals.var_t0_dn2 = assign29260_e50285_d_n2;
        locals.var_t0_dn3 = assign29260_e50285_d_n3;
        locals.var_t0_dn4 = assign29260_e50285_d_n4;
        locals.var_t0_dn5 = assign29260_e50285_d_n5;
        locals.var_t0_dn6 = assign29260_e50285_d_n6;
        locals.var_t0_dn7 = assign29260_e50285_d_n7;
        locals.var_t0_dn8 = assign29260_e50285_d_n8;
        locals.var_t0_dn9 = assign29260_e50285_d_n9;
        locals.var_t0_dn10 = assign29260_e50285_d_n10;
        locals.var_t0_dn11 = assign29260_e50285_d_n11;
        locals.var_t0_dn13 = assign29260_e50285_d_n13;
        locals.var_t0_dn14 = assign29260_e50285_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29270_e50303, assign29270_e50303_d_n0, assign29270_e50303_d_n2, assign29270_e50303_d_n3, assign29270_e50303_d_n4, assign29270_e50303_d_n5, assign29270_e50303_d_n6, assign29270_e50303_d_n7, assign29270_e50303_d_n8, assign29270_e50303_d_n9, assign29270_e50303_d_n10, assign29270_e50303_d_n11, assign29270_e50303_d_n13, assign29270_e50303_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 == 0.0)) {
        let assign29270_e50294: f64 = (locals.var_t0 * p.p1645);
        let assign29270_e50297: f64 = (p.p1645 - locals.var_ves_jct);
        let assign29270_e50298: f64 = (assign29270_e50294 / assign29270_e50297);
        let assign29270_e50299: f64 = { let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29270_e50301: f64 = (assign29270_e50299 - 1.0);
        (assign29270_e50301, ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1645) * assign29270_e50297) - (assign29270_e50294 * (-locals.var_ves_jct_dn3))) / (assign29270_e50297 * assign29270_e50297))), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn6 * p.p1645) * assign29270_e50297) - (assign29270_e50294 * (-locals.var_ves_jct_dn6))) / (assign29270_e50297 * assign29270_e50297))), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1645) / assign29270_e50297)), ({ let limited_exp_arg = assign29270_e50298; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1645) / assign29270_e50297)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29270_e50303;
        locals.var_t1_dn0 = assign29270_e50303_d_n0;
        locals.var_t1_dn2 = assign29270_e50303_d_n2;
        locals.var_t1_dn3 = assign29270_e50303_d_n3;
        locals.var_t1_dn4 = assign29270_e50303_d_n4;
        locals.var_t1_dn5 = assign29270_e50303_d_n5;
        locals.var_t1_dn6 = assign29270_e50303_d_n6;
        locals.var_t1_dn7 = assign29270_e50303_d_n7;
        locals.var_t1_dn8 = assign29270_e50303_d_n8;
        locals.var_t1_dn9 = assign29270_e50303_d_n9;
        locals.var_t1_dn10 = assign29270_e50303_d_n10;
        locals.var_t1_dn11 = assign29270_e50303_d_n11;
        locals.var_t1_dn13 = assign29270_e50303_d_n13;
        locals.var_t1_dn14 = assign29270_e50303_d_n14;
        locals.var_t1_rv = 0.0;

        let assign29290_e50321: f64 = if locals.var_jtsswgs_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign29290_e50321;
        locals.var_guard477_rv = 0.0;

        let assign29300_e50324: f64 = (p.p1647 - locals.var_ves_jct);
        let assign29300_e50327: f64 = (p.p1647 * 0.001);
        let assign29300_e50328: f64 = if assign29300_e50324 < assign29300_e50327 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign29300_e50328;
        locals.var_guard478_rv = 0.0;

        let (assign29310_e50341, assign29310_e50341_d_n0, assign29310_e50341_d_n2, assign29310_e50341_d_n3, assign29310_e50341_d_n4, assign29310_e50341_d_n5, assign29310_e50341_d_n6, assign29310_e50341_d_n7, assign29310_e50341_d_n8, assign29310_e50341_d_n9, assign29310_e50341_d_n10, assign29310_e50341_d_n11, assign29310_e50341_d_n13, assign29310_e50341_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign29310_e50335: f64 = (-locals.var_ves_jct);
        let assign29310_e50337: f64 = (assign29310_e50335 / locals.var_vtm0);
        let assign29310_e50339: f64 = (assign29310_e50337 / locals.var_njtsswg_t);
        (assign29310_e50339, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtsswg_t), (-((assign29310_e50337 * locals.var_njtsswg_t_dn4) / (locals.var_njtsswg_t * locals.var_njtsswg_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29310_e50341;
        locals.var_t0_dn0 = assign29310_e50341_d_n0;
        locals.var_t0_dn2 = assign29310_e50341_d_n2;
        locals.var_t0_dn3 = assign29310_e50341_d_n3;
        locals.var_t0_dn4 = assign29310_e50341_d_n4;
        locals.var_t0_dn5 = assign29310_e50341_d_n5;
        locals.var_t0_dn6 = assign29310_e50341_d_n6;
        locals.var_t0_dn7 = assign29310_e50341_d_n7;
        locals.var_t0_dn8 = assign29310_e50341_d_n8;
        locals.var_t0_dn9 = assign29310_e50341_d_n9;
        locals.var_t0_dn10 = assign29310_e50341_d_n10;
        locals.var_t0_dn11 = assign29310_e50341_d_n11;
        locals.var_t0_dn13 = assign29310_e50341_d_n13;
        locals.var_t0_dn14 = assign29310_e50341_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29320_e50354, assign29320_e50354_d_n0, assign29320_e50354_d_n2, assign29320_e50354_d_n3, assign29320_e50354_d_n4, assign29320_e50354_d_n5, assign29320_e50354_d_n6, assign29320_e50354_d_n7, assign29320_e50354_d_n8, assign29320_e50354_d_n9, assign29320_e50354_d_n10, assign29320_e50354_d_n11, assign29320_e50354_d_n13, assign29320_e50354_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign29320_e50349: f64 = (locals.var_t0 * 1000.0);
        let assign29320_e50350: f64 = { let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29320_e50352: f64 = (assign29320_e50350 - 1.0);
        (assign29320_e50352, ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29320_e50349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29320_e50354;
        locals.var_t1_dn0 = assign29320_e50354_d_n0;
        locals.var_t1_dn2 = assign29320_e50354_d_n2;
        locals.var_t1_dn3 = assign29320_e50354_d_n3;
        locals.var_t1_dn4 = assign29320_e50354_d_n4;
        locals.var_t1_dn5 = assign29320_e50354_d_n5;
        locals.var_t1_dn6 = assign29320_e50354_d_n6;
        locals.var_t1_dn7 = assign29320_e50354_d_n7;
        locals.var_t1_dn8 = assign29320_e50354_d_n8;
        locals.var_t1_dn9 = assign29320_e50354_d_n9;
        locals.var_t1_dn10 = assign29320_e50354_d_n10;
        locals.var_t1_dn11 = assign29320_e50354_d_n11;
        locals.var_t1_dn13 = assign29320_e50354_d_n13;
        locals.var_t1_dn14 = assign29320_e50354_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29340_e50384, assign29340_e50384_d_n0, assign29340_e50384_d_n2, assign29340_e50384_d_n3, assign29340_e50384_d_n4, assign29340_e50384_d_n5, assign29340_e50384_d_n6, assign29340_e50384_d_n7, assign29340_e50384_d_n8, assign29340_e50384_d_n9, assign29340_e50384_d_n10, assign29340_e50384_d_n11, assign29340_e50384_d_n13, assign29340_e50384_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 == 0.0)) {
        let assign29340_e50378: f64 = (-locals.var_ves_jct);
        let assign29340_e50380: f64 = (assign29340_e50378 / locals.var_vtm0);
        let assign29340_e50382: f64 = (assign29340_e50380 / locals.var_njtsswg_t);
        (assign29340_e50382, 0.0, 0.0, (((-locals.var_ves_jct_dn3) / locals.var_vtm0) / locals.var_njtsswg_t), (-((assign29340_e50380 * locals.var_njtsswg_t_dn4) / (locals.var_njtsswg_t * locals.var_njtsswg_t))), 0.0, (((-locals.var_ves_jct_dn6) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29340_e50384;
        locals.var_t0_dn0 = assign29340_e50384_d_n0;
        locals.var_t0_dn2 = assign29340_e50384_d_n2;
        locals.var_t0_dn3 = assign29340_e50384_d_n3;
        locals.var_t0_dn4 = assign29340_e50384_d_n4;
        locals.var_t0_dn5 = assign29340_e50384_d_n5;
        locals.var_t0_dn6 = assign29340_e50384_d_n6;
        locals.var_t0_dn7 = assign29340_e50384_d_n7;
        locals.var_t0_dn8 = assign29340_e50384_d_n8;
        locals.var_t0_dn9 = assign29340_e50384_d_n9;
        locals.var_t0_dn10 = assign29340_e50384_d_n10;
        locals.var_t0_dn11 = assign29340_e50384_d_n11;
        locals.var_t0_dn13 = assign29340_e50384_d_n13;
        locals.var_t0_dn14 = assign29340_e50384_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29350_e50402, assign29350_e50402_d_n0, assign29350_e50402_d_n2, assign29350_e50402_d_n3, assign29350_e50402_d_n4, assign29350_e50402_d_n5, assign29350_e50402_d_n6, assign29350_e50402_d_n7, assign29350_e50402_d_n8, assign29350_e50402_d_n9, assign29350_e50402_d_n10, assign29350_e50402_d_n11, assign29350_e50402_d_n13, assign29350_e50402_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 == 0.0)) {
        let assign29350_e50393: f64 = (locals.var_t0 * p.p1647);
        let assign29350_e50396: f64 = (p.p1647 - locals.var_ves_jct);
        let assign29350_e50397: f64 = (assign29350_e50393 / assign29350_e50396);
        let assign29350_e50398: f64 = { let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29350_e50400: f64 = (assign29350_e50398 - 1.0);
        (assign29350_e50400, ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1647) * assign29350_e50396) - (assign29350_e50393 * (-locals.var_ves_jct_dn3))) / (assign29350_e50396 * assign29350_e50396))), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn6 * p.p1647) * assign29350_e50396) - (assign29350_e50393 * (-locals.var_ves_jct_dn6))) / (assign29350_e50396 * assign29350_e50396))), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1647) / assign29350_e50396)), ({ let limited_exp_arg = assign29350_e50397; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1647) / assign29350_e50396)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29350_e50402;
        locals.var_t1_dn0 = assign29350_e50402_d_n0;
        locals.var_t1_dn2 = assign29350_e50402_d_n2;
        locals.var_t1_dn3 = assign29350_e50402_d_n3;
        locals.var_t1_dn4 = assign29350_e50402_d_n4;
        locals.var_t1_dn5 = assign29350_e50402_d_n5;
        locals.var_t1_dn6 = assign29350_e50402_d_n6;
        locals.var_t1_dn7 = assign29350_e50402_d_n7;
        locals.var_t1_dn8 = assign29350_e50402_d_n8;
        locals.var_t1_dn9 = assign29350_e50402_d_n9;
        locals.var_t1_dn10 = assign29350_e50402_d_n10;
        locals.var_t1_dn11 = assign29350_e50402_d_n11;
        locals.var_t1_dn13 = assign29350_e50402_d_n13;
        locals.var_t1_dn14 = assign29350_e50402_d_n14;
        locals.var_t1_rv = 0.0;

        let assign29370_e50422: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign29370_e50422;
        locals.var_guard479_rv = 0.0;

        let assign29380_e50425: f64 = if locals.var_ved_jct < locals.var_vjdmrev { 1.0 } else { 0.0 };
        locals.var_guard480 = assign29380_e50425;
        locals.var_guard480_rv = 0.0;

        let (assign29390_e50435, assign29390_e50435_d_n0, assign29390_e50435_d_n2, assign29390_e50435_d_n3, assign29390_e50435_d_n4, assign29390_e50435_d_n5, assign29390_e50435_d_n6, assign29390_e50435_d_n7, assign29390_e50435_d_n8, assign29390_e50435_d_n9, assign29390_e50435_d_n10, assign29390_e50435_d_n11, assign29390_e50435_d_n13, assign29390_e50435_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29390_e50433: f64 = (locals.var_ved_jct / locals.var_nvtmd);
        (assign29390_e50433, 0.0, 0.0, (locals.var_ved_jct_dn3 / locals.var_nvtmd), (-((locals.var_ved_jct * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), (locals.var_ved_jct_dn5 / locals.var_nvtmd), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29390_e50435;
        locals.var_t0_dn0 = assign29390_e50435_d_n0;
        locals.var_t0_dn2 = assign29390_e50435_d_n2;
        locals.var_t0_dn3 = assign29390_e50435_d_n3;
        locals.var_t0_dn4 = assign29390_e50435_d_n4;
        locals.var_t0_dn5 = assign29390_e50435_d_n5;
        locals.var_t0_dn6 = assign29390_e50435_d_n6;
        locals.var_t0_dn7 = assign29390_e50435_d_n7;
        locals.var_t0_dn8 = assign29390_e50435_d_n8;
        locals.var_t0_dn9 = assign29390_e50435_d_n9;
        locals.var_t0_dn10 = assign29390_e50435_d_n10;
        locals.var_t0_dn11 = assign29390_e50435_d_n11;
        locals.var_t0_dn13 = assign29390_e50435_d_n13;
        locals.var_t0_dn14 = assign29390_e50435_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29400_e50446, assign29400_e50446_d_n0, assign29400_e50446_d_n2, assign29400_e50446_d_n3, assign29400_e50446_d_n4, assign29400_e50446_d_n5, assign29400_e50446_d_n6, assign29400_e50446_d_n7, assign29400_e50446_d_n8, assign29400_e50446_d_n9, assign29400_e50446_d_n10, assign29400_e50446_d_n11, assign29400_e50446_d_n13, assign29400_e50446_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29400_e50442: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29400_e50444: f64 = (assign29400_e50442 - 1.0);
        (assign29400_e50444, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29400_e50446;
        locals.var_t1_dn0 = assign29400_e50446_d_n0;
        locals.var_t1_dn2 = assign29400_e50446_d_n2;
        locals.var_t1_dn3 = assign29400_e50446_d_n3;
        locals.var_t1_dn4 = assign29400_e50446_d_n4;
        locals.var_t1_dn5 = assign29400_e50446_d_n5;
        locals.var_t1_dn6 = assign29400_e50446_d_n6;
        locals.var_t1_dn7 = assign29400_e50446_d_n7;
        locals.var_t1_dn8 = assign29400_e50446_d_n8;
        locals.var_t1_dn9 = assign29400_e50446_d_n9;
        locals.var_t1_dn10 = assign29400_e50446_d_n10;
        locals.var_t1_dn11 = assign29400_e50446_d_n11;
        locals.var_t1_dn13 = assign29400_e50446_d_n13;
        locals.var_t1_dn14 = assign29400_e50446_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29410_e50460, assign29410_e50460_d_n0, assign29410_e50460_d_n2, assign29410_e50460_d_n3, assign29410_e50460_d_n4, assign29410_e50460_d_n5, assign29410_e50460_d_n6, assign29410_e50460_d_n7, assign29410_e50460_d_n8, assign29410_e50460_d_n9, assign29410_e50460_d_n10, assign29410_e50460_d_n11, assign29410_e50460_d_n13, assign29410_e50460_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 != 0.0)) {
        let assign29410_e50456: f64 = (locals.var_ved_jct - locals.var_vjdmrev);
        let assign29410_e50457: f64 = (locals.var_dslprev * assign29410_e50456);
        let assign29410_e50458: f64 = (locals.var_ivjdmrev + assign29410_e50457);
        (assign29410_e50458, (locals.var_ivjdmrev_dn0 + ((locals.var_dslprev_dn0 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn0)))), (locals.var_ivjdmrev_dn2 + ((locals.var_dslprev_dn2 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn2)))), (locals.var_ivjdmrev_dn3 + ((locals.var_dslprev_dn3 * assign29410_e50456) + (locals.var_dslprev * (locals.var_ved_jct_dn3 - locals.var_vjdmrev_dn3)))), (locals.var_ivjdmrev_dn4 + ((locals.var_dslprev_dn4 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn4)))), (locals.var_ivjdmrev_dn5 + ((locals.var_dslprev_dn5 * assign29410_e50456) + (locals.var_dslprev * (locals.var_ved_jct_dn5 - locals.var_vjdmrev_dn5)))), (locals.var_ivjdmrev_dn6 + ((locals.var_dslprev_dn6 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn6)))), (locals.var_ivjdmrev_dn7 + ((locals.var_dslprev_dn7 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn7)))), (locals.var_ivjdmrev_dn8 + ((locals.var_dslprev_dn8 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn8)))), (locals.var_ivjdmrev_dn9 + ((locals.var_dslprev_dn9 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn9)))), (locals.var_ivjdmrev_dn10 + ((locals.var_dslprev_dn10 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn10)))), (locals.var_ivjdmrev_dn11 + ((locals.var_dslprev_dn11 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn11)))), (locals.var_ivjdmrev_dn13 + ((locals.var_dslprev_dn13 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn13)))), (locals.var_ivjdmrev_dn14 + ((locals.var_dslprev_dn14 * assign29410_e50456) + (locals.var_dslprev * (-locals.var_vjdmrev_dn14)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29410_e50460;
        locals.var_t2_dn0 = assign29410_e50460_d_n0;
        locals.var_t2_dn2 = assign29410_e50460_d_n2;
        locals.var_t2_dn3 = assign29410_e50460_d_n3;
        locals.var_t2_dn4 = assign29410_e50460_d_n4;
        locals.var_t2_dn5 = assign29410_e50460_d_n5;
        locals.var_t2_dn6 = assign29410_e50460_d_n6;
        locals.var_t2_dn7 = assign29410_e50460_d_n7;
        locals.var_t2_dn8 = assign29410_e50460_d_n8;
        locals.var_t2_dn9 = assign29410_e50460_d_n9;
        locals.var_t2_dn10 = assign29410_e50460_d_n10;
        locals.var_t2_dn11 = assign29410_e50460_d_n11;
        locals.var_t2_dn13 = assign29410_e50460_d_n13;
        locals.var_t2_dn14 = assign29410_e50460_d_n14;
        locals.var_t2_rv = 0.0;

        let assign29430_e50473: f64 = if locals.var_ved_jct <= locals.var_vjdmfwd { 1.0 } else { 0.0 };
        locals.var_guard481 = assign29430_e50473;
        locals.var_guard481_rv = 0.0;

        let (assign29440_e50486, assign29440_e50486_d_n0, assign29440_e50486_d_n2, assign29440_e50486_d_n3, assign29440_e50486_d_n4, assign29440_e50486_d_n5, assign29440_e50486_d_n6, assign29440_e50486_d_n7, assign29440_e50486_d_n8, assign29440_e50486_d_n9, assign29440_e50486_d_n10, assign29440_e50486_d_n11, assign29440_e50486_d_n13, assign29440_e50486_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign29440_e50484: f64 = (locals.var_ved_jct / locals.var_nvtmd);
        (assign29440_e50484, 0.0, 0.0, (locals.var_ved_jct_dn3 / locals.var_nvtmd), (-((locals.var_ved_jct * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), (locals.var_ved_jct_dn5 / locals.var_nvtmd), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29440_e50486;
        locals.var_t0_dn0 = assign29440_e50486_d_n0;
        locals.var_t0_dn2 = assign29440_e50486_d_n2;
        locals.var_t0_dn3 = assign29440_e50486_d_n3;
        locals.var_t0_dn4 = assign29440_e50486_d_n4;
        locals.var_t0_dn5 = assign29440_e50486_d_n5;
        locals.var_t0_dn6 = assign29440_e50486_d_n6;
        locals.var_t0_dn7 = assign29440_e50486_d_n7;
        locals.var_t0_dn8 = assign29440_e50486_d_n8;
        locals.var_t0_dn9 = assign29440_e50486_d_n9;
        locals.var_t0_dn10 = assign29440_e50486_d_n10;
        locals.var_t0_dn11 = assign29440_e50486_d_n11;
        locals.var_t0_dn13 = assign29440_e50486_d_n13;
        locals.var_t0_dn14 = assign29440_e50486_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29450_e50501, assign29450_e50501_d_n0, assign29450_e50501_d_n2, assign29450_e50501_d_n3, assign29450_e50501_d_n4, assign29450_e50501_d_n5, assign29450_e50501_d_n6, assign29450_e50501_d_n7, assign29450_e50501_d_n8, assign29450_e50501_d_n9, assign29450_e50501_d_n10, assign29450_e50501_d_n11, assign29450_e50501_d_n13, assign29450_e50501_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign29450_e50497: f64 = (p.p1627 + locals.var_ved_jct);
        let assign29450_e50499: f64 = (assign29450_e50497 / locals.var_nvtmd);
        (assign29450_e50499, 0.0, 0.0, (locals.var_ved_jct_dn3 / locals.var_nvtmd), (-((assign29450_e50497 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), (locals.var_ved_jct_dn5 / locals.var_nvtmd), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29450_e50501;
        locals.var_t1_dn0 = assign29450_e50501_d_n0;
        locals.var_t1_dn2 = assign29450_e50501_d_n2;
        locals.var_t1_dn3 = assign29450_e50501_d_n3;
        locals.var_t1_dn4 = assign29450_e50501_d_n4;
        locals.var_t1_dn5 = assign29450_e50501_d_n5;
        locals.var_t1_dn6 = assign29450_e50501_d_n6;
        locals.var_t1_dn7 = assign29450_e50501_d_n7;
        locals.var_t1_dn8 = assign29450_e50501_d_n8;
        locals.var_t1_dn9 = assign29450_e50501_d_n9;
        locals.var_t1_dn10 = assign29450_e50501_d_n10;
        locals.var_t1_dn11 = assign29450_e50501_d_n11;
        locals.var_t1_dn13 = assign29450_e50501_d_n13;
        locals.var_t1_dn14 = assign29450_e50501_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29460_e50514, assign29460_e50514_d_n0, assign29460_e50514_d_n2, assign29460_e50514_d_n3, assign29460_e50514_d_n4, assign29460_e50514_d_n5, assign29460_e50514_d_n6, assign29460_e50514_d_n7, assign29460_e50514_d_n8, assign29460_e50514_d_n9, assign29460_e50514_d_n10, assign29460_e50514_d_n11, assign29460_e50514_d_n13, assign29460_e50514_d_n14,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard479 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign29460_e50511: f64 = (-locals.var_t1);
        let assign29460_e50512: f64 = { let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign29460_e50512, ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13)), ({ let limited_exp_arg = assign29460_e50511; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29460_e50514;
        locals.var_t2_dn0 = assign29460_e50514_d_n0;
        locals.var_t2_dn2 = assign29460_e50514_d_n2;
        locals.var_t2_dn3 = assign29460_e50514_d_n3;
        locals.var_t2_dn4 = assign29460_e50514_d_n4;
        locals.var_t2_dn5 = assign29460_e50514_d_n5;
        locals.var_t2_dn6 = assign29460_e50514_d_n6;
        locals.var_t2_dn7 = assign29460_e50514_d_n7;
        locals.var_t2_dn8 = assign29460_e50514_d_n8;
        locals.var_t2_dn9 = assign29460_e50514_d_n9;
        locals.var_t2_dn10 = assign29460_e50514_d_n10;
        locals.var_t2_dn11 = assign29460_e50514_d_n11;
        locals.var_t2_dn13 = assign29460_e50514_d_n13;
        locals.var_t2_dn14 = assign29460_e50514_d_n14;
        locals.var_t2_rv = 0.0;

        let assign29500_e50564: f64 = if locals.var_jtsd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign29500_e50564;
        locals.var_guard482_rv = 0.0;

        let assign29510_e50567: f64 = (p.p1644 - locals.var_ved_jct);
        let assign29510_e50570: f64 = (p.p1644 * 0.001);
        let assign29510_e50571: f64 = if assign29510_e50567 < assign29510_e50570 { 1.0 } else { 0.0 };
        locals.var_guard483 = assign29510_e50571;
        locals.var_guard483_rv = 0.0;

        let (assign29520_e50584, assign29520_e50584_d_n0, assign29520_e50584_d_n2, assign29520_e50584_d_n3, assign29520_e50584_d_n4, assign29520_e50584_d_n5, assign29520_e50584_d_n6, assign29520_e50584_d_n7, assign29520_e50584_d_n8, assign29520_e50584_d_n9, assign29520_e50584_d_n10, assign29520_e50584_d_n11, assign29520_e50584_d_n13, assign29520_e50584_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29520_e50578: f64 = (-locals.var_ved_jct);
        let assign29520_e50580: f64 = (assign29520_e50578 / locals.var_vtm0);
        let assign29520_e50582: f64 = (assign29520_e50580 / locals.var_njtsd_t);
        (assign29520_e50582, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsd_t), (-((assign29520_e50580 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29520_e50584;
        locals.var_t0_dn0 = assign29520_e50584_d_n0;
        locals.var_t0_dn2 = assign29520_e50584_d_n2;
        locals.var_t0_dn3 = assign29520_e50584_d_n3;
        locals.var_t0_dn4 = assign29520_e50584_d_n4;
        locals.var_t0_dn5 = assign29520_e50584_d_n5;
        locals.var_t0_dn6 = assign29520_e50584_d_n6;
        locals.var_t0_dn7 = assign29520_e50584_d_n7;
        locals.var_t0_dn8 = assign29520_e50584_d_n8;
        locals.var_t0_dn9 = assign29520_e50584_d_n9;
        locals.var_t0_dn10 = assign29520_e50584_d_n10;
        locals.var_t0_dn11 = assign29520_e50584_d_n11;
        locals.var_t0_dn13 = assign29520_e50584_d_n13;
        locals.var_t0_dn14 = assign29520_e50584_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29530_e50597, assign29530_e50597_d_n0, assign29530_e50597_d_n2, assign29530_e50597_d_n3, assign29530_e50597_d_n4, assign29530_e50597_d_n5, assign29530_e50597_d_n6, assign29530_e50597_d_n7, assign29530_e50597_d_n8, assign29530_e50597_d_n9, assign29530_e50597_d_n10, assign29530_e50597_d_n11, assign29530_e50597_d_n13, assign29530_e50597_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign29530_e50592: f64 = (locals.var_t0 * 1000.0);
        let assign29530_e50593: f64 = { let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29530_e50595: f64 = (assign29530_e50593 - 1.0);
        (assign29530_e50595, ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29530_e50592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29530_e50597;
        locals.var_t1_dn0 = assign29530_e50597_d_n0;
        locals.var_t1_dn2 = assign29530_e50597_d_n2;
        locals.var_t1_dn3 = assign29530_e50597_d_n3;
        locals.var_t1_dn4 = assign29530_e50597_d_n4;
        locals.var_t1_dn5 = assign29530_e50597_d_n5;
        locals.var_t1_dn6 = assign29530_e50597_d_n6;
        locals.var_t1_dn7 = assign29530_e50597_d_n7;
        locals.var_t1_dn8 = assign29530_e50597_d_n8;
        locals.var_t1_dn9 = assign29530_e50597_d_n9;
        locals.var_t1_dn10 = assign29530_e50597_d_n10;
        locals.var_t1_dn11 = assign29530_e50597_d_n11;
        locals.var_t1_dn13 = assign29530_e50597_d_n13;
        locals.var_t1_dn14 = assign29530_e50597_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29550_e50625, assign29550_e50625_d_n0, assign29550_e50625_d_n2, assign29550_e50625_d_n3, assign29550_e50625_d_n4, assign29550_e50625_d_n5, assign29550_e50625_d_n6, assign29550_e50625_d_n7, assign29550_e50625_d_n8, assign29550_e50625_d_n9, assign29550_e50625_d_n10, assign29550_e50625_d_n11, assign29550_e50625_d_n13, assign29550_e50625_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 == 0.0)) {
        let assign29550_e50619: f64 = (-locals.var_ved_jct);
        let assign29550_e50621: f64 = (assign29550_e50619 / locals.var_vtm0);
        let assign29550_e50623: f64 = (assign29550_e50621 / locals.var_njtsd_t);
        (assign29550_e50623, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsd_t), (-((assign29550_e50621 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29550_e50625;
        locals.var_t0_dn0 = assign29550_e50625_d_n0;
        locals.var_t0_dn2 = assign29550_e50625_d_n2;
        locals.var_t0_dn3 = assign29550_e50625_d_n3;
        locals.var_t0_dn4 = assign29550_e50625_d_n4;
        locals.var_t0_dn5 = assign29550_e50625_d_n5;
        locals.var_t0_dn6 = assign29550_e50625_d_n6;
        locals.var_t0_dn7 = assign29550_e50625_d_n7;
        locals.var_t0_dn8 = assign29550_e50625_d_n8;
        locals.var_t0_dn9 = assign29550_e50625_d_n9;
        locals.var_t0_dn10 = assign29550_e50625_d_n10;
        locals.var_t0_dn11 = assign29550_e50625_d_n11;
        locals.var_t0_dn13 = assign29550_e50625_d_n13;
        locals.var_t0_dn14 = assign29550_e50625_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29560_e50643, assign29560_e50643_d_n0, assign29560_e50643_d_n2, assign29560_e50643_d_n3, assign29560_e50643_d_n4, assign29560_e50643_d_n5, assign29560_e50643_d_n6, assign29560_e50643_d_n7, assign29560_e50643_d_n8, assign29560_e50643_d_n9, assign29560_e50643_d_n10, assign29560_e50643_d_n11, assign29560_e50643_d_n13, assign29560_e50643_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 == 0.0)) {
        let assign29560_e50634: f64 = (locals.var_t0 * p.p1644);
        let assign29560_e50637: f64 = (p.p1644 - locals.var_ved_jct);
        let assign29560_e50638: f64 = (assign29560_e50634 / assign29560_e50637);
        let assign29560_e50639: f64 = { let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29560_e50641: f64 = (assign29560_e50639 - 1.0);
        (assign29560_e50641, ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1644) * assign29560_e50637) - (assign29560_e50634 * (-locals.var_ved_jct_dn3))) / (assign29560_e50637 * assign29560_e50637))), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p1644) * assign29560_e50637) - (assign29560_e50634 * (-locals.var_ved_jct_dn5))) / (assign29560_e50637 * assign29560_e50637))), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1644) / assign29560_e50637)), ({ let limited_exp_arg = assign29560_e50638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1644) / assign29560_e50637)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29560_e50643;
        locals.var_t1_dn0 = assign29560_e50643_d_n0;
        locals.var_t1_dn2 = assign29560_e50643_d_n2;
        locals.var_t1_dn3 = assign29560_e50643_d_n3;
        locals.var_t1_dn4 = assign29560_e50643_d_n4;
        locals.var_t1_dn5 = assign29560_e50643_d_n5;
        locals.var_t1_dn6 = assign29560_e50643_d_n6;
        locals.var_t1_dn7 = assign29560_e50643_d_n7;
        locals.var_t1_dn8 = assign29560_e50643_d_n8;
        locals.var_t1_dn9 = assign29560_e50643_d_n9;
        locals.var_t1_dn10 = assign29560_e50643_d_n10;
        locals.var_t1_dn11 = assign29560_e50643_d_n11;
        locals.var_t1_dn13 = assign29560_e50643_d_n13;
        locals.var_t1_dn14 = assign29560_e50643_d_n14;
        locals.var_t1_rv = 0.0;

        let assign29580_e50661: f64 = if locals.var_jtsswd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard484 = assign29580_e50661;
        locals.var_guard484_rv = 0.0;

        let assign29590_e50664: f64 = (p.p1646 - locals.var_ved_jct);
        let assign29590_e50667: f64 = (p.p1646 * 0.001);
        let assign29590_e50668: f64 = if assign29590_e50664 < assign29590_e50667 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign29590_e50668;
        locals.var_guard485_rv = 0.0;

        let (assign29600_e50681, assign29600_e50681_d_n0, assign29600_e50681_d_n2, assign29600_e50681_d_n3, assign29600_e50681_d_n4, assign29600_e50681_d_n5, assign29600_e50681_d_n6, assign29600_e50681_d_n7, assign29600_e50681_d_n8, assign29600_e50681_d_n9, assign29600_e50681_d_n10, assign29600_e50681_d_n11, assign29600_e50681_d_n13, assign29600_e50681_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign29600_e50675: f64 = (-locals.var_ved_jct);
        let assign29600_e50677: f64 = (assign29600_e50675 / locals.var_vtm0);
        let assign29600_e50679: f64 = (assign29600_e50677 / locals.var_njtsswd_t);
        (assign29600_e50679, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswd_t), (-((assign29600_e50677 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29600_e50681;
        locals.var_t0_dn0 = assign29600_e50681_d_n0;
        locals.var_t0_dn2 = assign29600_e50681_d_n2;
        locals.var_t0_dn3 = assign29600_e50681_d_n3;
        locals.var_t0_dn4 = assign29600_e50681_d_n4;
        locals.var_t0_dn5 = assign29600_e50681_d_n5;
        locals.var_t0_dn6 = assign29600_e50681_d_n6;
        locals.var_t0_dn7 = assign29600_e50681_d_n7;
        locals.var_t0_dn8 = assign29600_e50681_d_n8;
        locals.var_t0_dn9 = assign29600_e50681_d_n9;
        locals.var_t0_dn10 = assign29600_e50681_d_n10;
        locals.var_t0_dn11 = assign29600_e50681_d_n11;
        locals.var_t0_dn13 = assign29600_e50681_d_n13;
        locals.var_t0_dn14 = assign29600_e50681_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29610_e50694, assign29610_e50694_d_n0, assign29610_e50694_d_n2, assign29610_e50694_d_n3, assign29610_e50694_d_n4, assign29610_e50694_d_n5, assign29610_e50694_d_n6, assign29610_e50694_d_n7, assign29610_e50694_d_n8, assign29610_e50694_d_n9, assign29610_e50694_d_n10, assign29610_e50694_d_n11, assign29610_e50694_d_n13, assign29610_e50694_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign29610_e50689: f64 = (locals.var_t0 * 1000.0);
        let assign29610_e50690: f64 = { let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29610_e50692: f64 = (assign29610_e50690 - 1.0);
        (assign29610_e50692, ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29610_e50689; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29610_e50694;
        locals.var_t1_dn0 = assign29610_e50694_d_n0;
        locals.var_t1_dn2 = assign29610_e50694_d_n2;
        locals.var_t1_dn3 = assign29610_e50694_d_n3;
        locals.var_t1_dn4 = assign29610_e50694_d_n4;
        locals.var_t1_dn5 = assign29610_e50694_d_n5;
        locals.var_t1_dn6 = assign29610_e50694_d_n6;
        locals.var_t1_dn7 = assign29610_e50694_d_n7;
        locals.var_t1_dn8 = assign29610_e50694_d_n8;
        locals.var_t1_dn9 = assign29610_e50694_d_n9;
        locals.var_t1_dn10 = assign29610_e50694_d_n10;
        locals.var_t1_dn11 = assign29610_e50694_d_n11;
        locals.var_t1_dn13 = assign29610_e50694_d_n13;
        locals.var_t1_dn14 = assign29610_e50694_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29630_e50722, assign29630_e50722_d_n0, assign29630_e50722_d_n2, assign29630_e50722_d_n3, assign29630_e50722_d_n4, assign29630_e50722_d_n5, assign29630_e50722_d_n6, assign29630_e50722_d_n7, assign29630_e50722_d_n8, assign29630_e50722_d_n9, assign29630_e50722_d_n10, assign29630_e50722_d_n11, assign29630_e50722_d_n13, assign29630_e50722_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign29630_e50716: f64 = (-locals.var_ved_jct);
        let assign29630_e50718: f64 = (assign29630_e50716 / locals.var_vtm0);
        let assign29630_e50720: f64 = (assign29630_e50718 / locals.var_njtsswd_t);
        (assign29630_e50720, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswd_t), (-((assign29630_e50718 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29630_e50722;
        locals.var_t0_dn0 = assign29630_e50722_d_n0;
        locals.var_t0_dn2 = assign29630_e50722_d_n2;
        locals.var_t0_dn3 = assign29630_e50722_d_n3;
        locals.var_t0_dn4 = assign29630_e50722_d_n4;
        locals.var_t0_dn5 = assign29630_e50722_d_n5;
        locals.var_t0_dn6 = assign29630_e50722_d_n6;
        locals.var_t0_dn7 = assign29630_e50722_d_n7;
        locals.var_t0_dn8 = assign29630_e50722_d_n8;
        locals.var_t0_dn9 = assign29630_e50722_d_n9;
        locals.var_t0_dn10 = assign29630_e50722_d_n10;
        locals.var_t0_dn11 = assign29630_e50722_d_n11;
        locals.var_t0_dn13 = assign29630_e50722_d_n13;
        locals.var_t0_dn14 = assign29630_e50722_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29640_e50740, assign29640_e50740_d_n0, assign29640_e50740_d_n2, assign29640_e50740_d_n3, assign29640_e50740_d_n4, assign29640_e50740_d_n5, assign29640_e50740_d_n6, assign29640_e50740_d_n7, assign29640_e50740_d_n8, assign29640_e50740_d_n9, assign29640_e50740_d_n10, assign29640_e50740_d_n11, assign29640_e50740_d_n13, assign29640_e50740_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard484 != 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign29640_e50731: f64 = (locals.var_t0 * p.p1646);
        let assign29640_e50734: f64 = (p.p1646 - locals.var_ved_jct);
        let assign29640_e50735: f64 = (assign29640_e50731 / assign29640_e50734);
        let assign29640_e50736: f64 = { let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29640_e50738: f64 = (assign29640_e50736 - 1.0);
        (assign29640_e50738, ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1646) * assign29640_e50734) - (assign29640_e50731 * (-locals.var_ved_jct_dn3))) / (assign29640_e50734 * assign29640_e50734))), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p1646) * assign29640_e50734) - (assign29640_e50731 * (-locals.var_ved_jct_dn5))) / (assign29640_e50734 * assign29640_e50734))), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1646) / assign29640_e50734)), ({ let limited_exp_arg = assign29640_e50735; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1646) / assign29640_e50734)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29640_e50740;
        locals.var_t1_dn0 = assign29640_e50740_d_n0;
        locals.var_t1_dn2 = assign29640_e50740_d_n2;
        locals.var_t1_dn3 = assign29640_e50740_d_n3;
        locals.var_t1_dn4 = assign29640_e50740_d_n4;
        locals.var_t1_dn5 = assign29640_e50740_d_n5;
        locals.var_t1_dn6 = assign29640_e50740_d_n6;
        locals.var_t1_dn7 = assign29640_e50740_d_n7;
        locals.var_t1_dn8 = assign29640_e50740_d_n8;
        locals.var_t1_dn9 = assign29640_e50740_d_n9;
        locals.var_t1_dn10 = assign29640_e50740_d_n10;
        locals.var_t1_dn11 = assign29640_e50740_d_n11;
        locals.var_t1_dn13 = assign29640_e50740_d_n13;
        locals.var_t1_dn14 = assign29640_e50740_d_n14;
        locals.var_t1_rv = 0.0;

        let assign29660_e50758: f64 = if locals.var_jtsswgd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign29660_e50758;
        locals.var_guard486_rv = 0.0;

        let assign29670_e50761: f64 = (p.p1648 - locals.var_ved_jct);
        let assign29670_e50764: f64 = (p.p1648 * 0.001);
        let assign29670_e50765: f64 = if assign29670_e50761 < assign29670_e50764 { 1.0 } else { 0.0 };
        locals.var_guard487 = assign29670_e50765;
        locals.var_guard487_rv = 0.0;

        let (assign29680_e50778, assign29680_e50778_d_n0, assign29680_e50778_d_n2, assign29680_e50778_d_n3, assign29680_e50778_d_n4, assign29680_e50778_d_n5, assign29680_e50778_d_n6, assign29680_e50778_d_n7, assign29680_e50778_d_n8, assign29680_e50778_d_n9, assign29680_e50778_d_n10, assign29680_e50778_d_n11, assign29680_e50778_d_n13, assign29680_e50778_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 != 0.0)) {
        let assign29680_e50772: f64 = (-locals.var_ved_jct);
        let assign29680_e50774: f64 = (assign29680_e50772 / locals.var_vtm0);
        let assign29680_e50776: f64 = (assign29680_e50774 / locals.var_njtsswgd_t);
        (assign29680_e50776, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswgd_t), (-((assign29680_e50774 * locals.var_njtsswgd_t_dn4) / (locals.var_njtsswgd_t * locals.var_njtsswgd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29680_e50778;
        locals.var_t0_dn0 = assign29680_e50778_d_n0;
        locals.var_t0_dn2 = assign29680_e50778_d_n2;
        locals.var_t0_dn3 = assign29680_e50778_d_n3;
        locals.var_t0_dn4 = assign29680_e50778_d_n4;
        locals.var_t0_dn5 = assign29680_e50778_d_n5;
        locals.var_t0_dn6 = assign29680_e50778_d_n6;
        locals.var_t0_dn7 = assign29680_e50778_d_n7;
        locals.var_t0_dn8 = assign29680_e50778_d_n8;
        locals.var_t0_dn9 = assign29680_e50778_d_n9;
        locals.var_t0_dn10 = assign29680_e50778_d_n10;
        locals.var_t0_dn11 = assign29680_e50778_d_n11;
        locals.var_t0_dn13 = assign29680_e50778_d_n13;
        locals.var_t0_dn14 = assign29680_e50778_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29690_e50791, assign29690_e50791_d_n0, assign29690_e50791_d_n2, assign29690_e50791_d_n3, assign29690_e50791_d_n4, assign29690_e50791_d_n5, assign29690_e50791_d_n6, assign29690_e50791_d_n7, assign29690_e50791_d_n8, assign29690_e50791_d_n9, assign29690_e50791_d_n10, assign29690_e50791_d_n11, assign29690_e50791_d_n13, assign29690_e50791_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 != 0.0)) {
        let assign29690_e50786: f64 = (locals.var_t0 * 1000.0);
        let assign29690_e50787: f64 = { let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29690_e50789: f64 = (assign29690_e50787 - 1.0);
        (assign29690_e50789, ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign29690_e50786; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29690_e50791;
        locals.var_t1_dn0 = assign29690_e50791_d_n0;
        locals.var_t1_dn2 = assign29690_e50791_d_n2;
        locals.var_t1_dn3 = assign29690_e50791_d_n3;
        locals.var_t1_dn4 = assign29690_e50791_d_n4;
        locals.var_t1_dn5 = assign29690_e50791_d_n5;
        locals.var_t1_dn6 = assign29690_e50791_d_n6;
        locals.var_t1_dn7 = assign29690_e50791_d_n7;
        locals.var_t1_dn8 = assign29690_e50791_d_n8;
        locals.var_t1_dn9 = assign29690_e50791_d_n9;
        locals.var_t1_dn10 = assign29690_e50791_d_n10;
        locals.var_t1_dn11 = assign29690_e50791_d_n11;
        locals.var_t1_dn13 = assign29690_e50791_d_n13;
        locals.var_t1_dn14 = assign29690_e50791_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign29710_e50821, assign29710_e50821_d_n0, assign29710_e50821_d_n2, assign29710_e50821_d_n3, assign29710_e50821_d_n4, assign29710_e50821_d_n5, assign29710_e50821_d_n6, assign29710_e50821_d_n7, assign29710_e50821_d_n8, assign29710_e50821_d_n9, assign29710_e50821_d_n10, assign29710_e50821_d_n11, assign29710_e50821_d_n13, assign29710_e50821_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign29710_e50815: f64 = (-locals.var_ved_jct);
        let assign29710_e50817: f64 = (assign29710_e50815 / locals.var_vtm0);
        let assign29710_e50819: f64 = (assign29710_e50817 / locals.var_njtsswgd_t);
        (assign29710_e50819, 0.0, 0.0, (((-locals.var_ved_jct_dn3) / locals.var_vtm0) / locals.var_njtsswgd_t), (-((assign29710_e50817 * locals.var_njtsswgd_t_dn4) / (locals.var_njtsswgd_t * locals.var_njtsswgd_t))), (((-locals.var_ved_jct_dn5) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29710_e50821;
        locals.var_t0_dn0 = assign29710_e50821_d_n0;
        locals.var_t0_dn2 = assign29710_e50821_d_n2;
        locals.var_t0_dn3 = assign29710_e50821_d_n3;
        locals.var_t0_dn4 = assign29710_e50821_d_n4;
        locals.var_t0_dn5 = assign29710_e50821_d_n5;
        locals.var_t0_dn6 = assign29710_e50821_d_n6;
        locals.var_t0_dn7 = assign29710_e50821_d_n7;
        locals.var_t0_dn8 = assign29710_e50821_d_n8;
        locals.var_t0_dn9 = assign29710_e50821_d_n9;
        locals.var_t0_dn10 = assign29710_e50821_d_n10;
        locals.var_t0_dn11 = assign29710_e50821_d_n11;
        locals.var_t0_dn13 = assign29710_e50821_d_n13;
        locals.var_t0_dn14 = assign29710_e50821_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign29720_e50839, assign29720_e50839_d_n0, assign29720_e50839_d_n2, assign29720_e50839_d_n3, assign29720_e50839_d_n4, assign29720_e50839_d_n5, assign29720_e50839_d_n6, assign29720_e50839_d_n7, assign29720_e50839_d_n8, assign29720_e50839_d_n9, assign29720_e50839_d_n10, assign29720_e50839_d_n11, assign29720_e50839_d_n13, assign29720_e50839_d_n14,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign29720_e50830: f64 = (locals.var_t0 * p.p1648);
        let assign29720_e50833: f64 = (p.p1648 - locals.var_ved_jct);
        let assign29720_e50834: f64 = (assign29720_e50830 / assign29720_e50833);
        let assign29720_e50835: f64 = { let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign29720_e50837: f64 = (assign29720_e50835 - 1.0);
        (assign29720_e50837, ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn3 * p.p1648) * assign29720_e50833) - (assign29720_e50830 * (-locals.var_ved_jct_dn3))) / (assign29720_e50833 * assign29720_e50833))), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p1648) * assign29720_e50833) - (assign29720_e50830 * (-locals.var_ved_jct_dn5))) / (assign29720_e50833 * assign29720_e50833))), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p1648) / assign29720_e50833)), ({ let limited_exp_arg = assign29720_e50834; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p1648) / assign29720_e50833)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign29720_e50839;
        locals.var_t1_dn0 = assign29720_e50839_d_n0;
        locals.var_t1_dn2 = assign29720_e50839_d_n2;
        locals.var_t1_dn3 = assign29720_e50839_d_n3;
        locals.var_t1_dn4 = assign29720_e50839_d_n4;
        locals.var_t1_dn5 = assign29720_e50839_d_n5;
        locals.var_t1_dn6 = assign29720_e50839_d_n6;
        locals.var_t1_dn7 = assign29720_e50839_d_n7;
        locals.var_t1_dn8 = assign29720_e50839_d_n8;
        locals.var_t1_dn9 = assign29720_e50839_d_n9;
        locals.var_t1_dn10 = assign29720_e50839_d_n10;
        locals.var_t1_dn11 = assign29720_e50839_d_n11;
        locals.var_t1_dn13 = assign29720_e50839_d_n13;
        locals.var_t1_dn14 = assign29720_e50839_d_n14;
        locals.var_t1_rv = 0.0;

        let assign29740_e50859: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard496 = assign29740_e50859;
        locals.var_guard496_rv = 0.0;

        let (assign29750_e50867, assign29750_e50867_d_n3, assign29750_e50867_d_n4, assign29750_e50867_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) {
        let assign29750_e50865: f64 = (locals.var_ves_jct / locals.var_pbs_t);
        (assign29750_e50865, (locals.var_ves_jct_dn3 / locals.var_pbs_t), (-((locals.var_ves_jct * locals.var_pbs_t_dn4) / (locals.var_pbs_t * locals.var_pbs_t))), (locals.var_ves_jct_dn6 / locals.var_pbs_t),)
    } else {
        (locals.var_t1__blk488, locals.var_t1__blk488_dn3, locals.var_t1__blk488_dn4, locals.var_t1__blk488_dn6,)
    }
};
        locals.var_t1__blk488 = assign29750_e50867;
        locals.var_t1__blk488_dn3 = assign29750_e50867_d_n3;
        locals.var_t1__blk488_dn4 = assign29750_e50867_d_n4;
        locals.var_t1__blk488_dn6 = assign29750_e50867_d_n6;
        locals.var_t1__blk488_rv = 0.0;

        let assign29760_e50870: f64 = if locals.var_t1__blk488 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard497 = assign29760_e50870;
        locals.var_guard497_rv = 0.0;

        let assign29770_e50873: f64 = if p.p1602 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign29770_e50873;
        locals.var_guard498_rv = 0.0;

        let assign29780_e50876: f64 = if locals.var_ves_jct > locals.var_vec1s { 1.0 } else { 0.0 };
        locals.var_guard499 = assign29780_e50876;
        locals.var_guard499_rv = 0.0;

        let (assign29790_e50890, assign29790_e50890_d_n3, assign29790_e50890_d_n4, assign29790_e50890_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) {
        let assign29790_e50888: f64 = (1.0 - locals.var_t1__blk488);
        (assign29790_e50888, (-locals.var_t1__blk488_dn3), (-locals.var_t1__blk488_dn4), (-locals.var_t1__blk488_dn6),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign29790_e50890;
        locals.var_arg_dn3 = assign29790_e50890_d_n3;
        locals.var_arg_dn4 = assign29790_e50890_d_n4;
        locals.var_arg_dn6 = assign29790_e50890_d_n6;
        locals.var_arg_rv = 0.0;

        let assign29800_e50893: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign29800_e50893;
        locals.var_guard500_rv = 0.0;

        let assign29810_e50896: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign29810_e50896;
        locals.var_guard501_rv = 0.0;

        let (assign29820_e50915, assign29820_e50915_d_n3, assign29820_e50915_d_n4, assign29820_e50915_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 != 0.0)) {
        let assign29820_e50912: f64 = (locals.var_arg).sqrt();
        let assign29820_e50913: f64 = (1.0 / assign29820_e50912);
        (assign29820_e50913, (-((locals.var_arg_dn3 / (2.0 * assign29820_e50912)) / (assign29820_e50912 * assign29820_e50912))), (-((locals.var_arg_dn4 / (2.0 * assign29820_e50912)) / (assign29820_e50912 * assign29820_e50912))), (-((locals.var_arg_dn6 / (2.0 * assign29820_e50912)) / (assign29820_e50912 * assign29820_e50912))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29820_e50915;
        locals.var_sarg_dn3 = assign29820_e50915_d_n3;
        locals.var_sarg_dn4 = assign29820_e50915_d_n4;
        locals.var_sarg_dn6 = assign29820_e50915_d_n6;
        locals.var_sarg_rv = 0.0;

        let (assign29830_e50935, assign29830_e50935_d_n3, assign29830_e50935_d_n4, assign29830_e50935_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 == 0.0)) {
        let assign29830_e50932: f64 = (-p.p1596);
        let assign29830_e50933: f64 = (locals.var_arg).powf(assign29830_e50932);
        (assign29830_e50933, if 0.0 == 0.0 && ((assign29830_e50932) as f64).is_finite() && ((assign29830_e50932) as f64).fract() == 0.0 { if assign29830_e50932 == 0.0 { 0.0 } else { (assign29830_e50932 * ((locals.var_arg).powf(assign29830_e50932 - 1.0) * locals.var_arg_dn3)) } } else { (assign29830_e50933 * (assign29830_e50932 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29830_e50932) as f64).is_finite() && ((assign29830_e50932) as f64).fract() == 0.0 { if assign29830_e50932 == 0.0 { 0.0 } else { (assign29830_e50932 * ((locals.var_arg).powf(assign29830_e50932 - 1.0) * locals.var_arg_dn4)) } } else { (assign29830_e50933 * (assign29830_e50932 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29830_e50932) as f64).is_finite() && ((assign29830_e50932) as f64).fract() == 0.0 { if assign29830_e50932 == 0.0 { 0.0 } else { (assign29830_e50932 * ((locals.var_arg).powf(assign29830_e50932 - 1.0) * locals.var_arg_dn6)) } } else { (assign29830_e50933 * (assign29830_e50932 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29830_e50935;
        locals.var_sarg_dn3 = assign29830_e50935_d_n3;
        locals.var_sarg_dn4 = assign29830_e50935_d_n4;
        locals.var_sarg_dn6 = assign29830_e50935_d_n6;
        locals.var_sarg_rv = 0.0;

        let (assign29840_e50961, assign29840_e50961_d_n3, assign29840_e50961_d_n4, assign29840_e50961_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 != 0.0)) {
        let assign29840_e50949: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign29840_e50953: f64 = (locals.var_arg * locals.var_sarg);
        let assign29840_e50954: f64 = (1.0 - assign29840_e50953);
        let assign29840_e50955: f64 = (assign29840_e50949 * assign29840_e50954);
        let assign29840_e50958: f64 = (1.0 - p.p1596);
        let assign29840_e50959: f64 = (assign29840_e50955 / assign29840_e50958);
        (assign29840_e50959, ((assign29840_e50949 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign29840_e50958), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign29840_e50954) + (assign29840_e50949 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign29840_e50958), ((assign29840_e50949 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign29840_e50958),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29840_e50961;
        locals.var_qesj1_dn3 = assign29840_e50961_d_n3;
        locals.var_qesj1_dn4 = assign29840_e50961_d_n4;
        locals.var_qesj1_dn6 = assign29840_e50961_d_n6;
        locals.var_qesj1_rv = 0.0;

        let (assign29850_e50994, assign29850_e50994_d_n3, assign29850_e50994_d_n4, assign29850_e50994_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 == 0.0)) {
        let assign29850_e50975: f64 = (-locals.var_pbs_t);
        let assign29850_e50977: f64 = (assign29850_e50975 * locals.var_czbs);
        let (assign29850_e50991, assign29850_e50991_d_n3, assign29850_e50991_d_n4, assign29850_e50991_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign29850_e50983: f64 = (-87.498233534);
                (assign29850_e50983, 0.0, 0.0, 0.0,)
            } else {
                let (assign29850_e50990, assign29850_e50990_d_n3, assign29850_e50990_d_n4, assign29850_e50990_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign29850_e50988: f64 = (locals.var_arg).ln();
                        (assign29850_e50988, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign29850_e50990, assign29850_e50990_d_n3, assign29850_e50990_d_n4, assign29850_e50990_d_n6,)
            }
        };
        let assign29850_e50992: f64 = (assign29850_e50977 * assign29850_e50991);
        (assign29850_e50992, (assign29850_e50977 * assign29850_e50991_d_n3), (((((-locals.var_pbs_t_dn4) * locals.var_czbs) + (assign29850_e50975 * locals.var_czbs_dn4)) * assign29850_e50991) + (assign29850_e50977 * assign29850_e50991_d_n4)), (assign29850_e50977 * assign29850_e50991_d_n6),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29850_e50994;
        locals.var_qesj1_dn3 = assign29850_e50994_d_n3;
        locals.var_qesj1_dn4 = assign29850_e50994_d_n4;
        locals.var_qesj1_dn6 = assign29850_e50994_d_n6;
        locals.var_qesj1_rv = 0.0;

        let (assign29860_e51011, assign29860_e51011_d_n3, assign29860_e51011_d_n4, assign29860_e51011_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) {
        let assign29860_e51008: f64 = (locals.var_vec1s / locals.var_pbs_t);
        let assign29860_e51009: f64 = (1.0 - assign29860_e51008);
        (assign29860_e51009, 0.0, (-(((locals.var_vec1s_dn4 * locals.var_pbs_t) - (locals.var_vec1s * locals.var_pbs_t_dn4)) / (locals.var_pbs_t * locals.var_pbs_t))), 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign29860_e51011;
        locals.var_arg_dn3 = assign29860_e51011_d_n3;
        locals.var_arg_dn4 = assign29860_e51011_d_n4;
        locals.var_arg_dn6 = assign29860_e51011_d_n6;
        locals.var_arg_rv = 0.0;

        let assign29870_e51014: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard502 = assign29870_e51014;
        locals.var_guard502_rv = 0.0;

        let assign29880_e51017: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard503 = assign29880_e51017;
        locals.var_guard503_rv = 0.0;

        let (assign29890_e51037, assign29890_e51037_d_n3, assign29890_e51037_d_n4, assign29890_e51037_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 != 0.0)) && (locals.var_guard503 != 0.0)) {
        let assign29890_e51034: f64 = (locals.var_arg).sqrt();
        let assign29890_e51035: f64 = (1.0 / assign29890_e51034);
        (assign29890_e51035, (-((locals.var_arg_dn3 / (2.0 * assign29890_e51034)) / (assign29890_e51034 * assign29890_e51034))), (-((locals.var_arg_dn4 / (2.0 * assign29890_e51034)) / (assign29890_e51034 * assign29890_e51034))), (-((locals.var_arg_dn6 / (2.0 * assign29890_e51034)) / (assign29890_e51034 * assign29890_e51034))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29890_e51037;
        locals.var_sarg_dn3 = assign29890_e51037_d_n3;
        locals.var_sarg_dn4 = assign29890_e51037_d_n4;
        locals.var_sarg_dn6 = assign29890_e51037_d_n6;
        locals.var_sarg_rv = 0.0;

        let (assign29900_e51058, assign29900_e51058_d_n3, assign29900_e51058_d_n4, assign29900_e51058_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 != 0.0)) && (locals.var_guard503 == 0.0)) {
        let assign29900_e51055: f64 = (-p.p1596);
        let assign29900_e51056: f64 = (locals.var_arg).powf(assign29900_e51055);
        (assign29900_e51056, if 0.0 == 0.0 && ((assign29900_e51055) as f64).is_finite() && ((assign29900_e51055) as f64).fract() == 0.0 { if assign29900_e51055 == 0.0 { 0.0 } else { (assign29900_e51055 * ((locals.var_arg).powf(assign29900_e51055 - 1.0) * locals.var_arg_dn3)) } } else { (assign29900_e51056 * (assign29900_e51055 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29900_e51055) as f64).is_finite() && ((assign29900_e51055) as f64).fract() == 0.0 { if assign29900_e51055 == 0.0 { 0.0 } else { (assign29900_e51055 * ((locals.var_arg).powf(assign29900_e51055 - 1.0) * locals.var_arg_dn4)) } } else { (assign29900_e51056 * (assign29900_e51055 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29900_e51055) as f64).is_finite() && ((assign29900_e51055) as f64).fract() == 0.0 { if assign29900_e51055 == 0.0 { 0.0 } else { (assign29900_e51055 * ((locals.var_arg).powf(assign29900_e51055 - 1.0) * locals.var_arg_dn6)) } } else { (assign29900_e51056 * (assign29900_e51055 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29900_e51058;
        locals.var_sarg_dn3 = assign29900_e51058_d_n3;
        locals.var_sarg_dn4 = assign29900_e51058_d_n4;
        locals.var_sarg_dn6 = assign29900_e51058_d_n6;
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29910_e51085, assign29910_e51085_d_n3, assign29910_e51085_d_n4, assign29910_e51085_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 != 0.0)) {
        let assign29910_e51073: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign29910_e51077: f64 = (locals.var_arg * locals.var_sarg);
        let assign29910_e51078: f64 = (1.0 - assign29910_e51077);
        let assign29910_e51079: f64 = (assign29910_e51073 * assign29910_e51078);
        let assign29910_e51082: f64 = (1.0 - p.p1596);
        let assign29910_e51083: f64 = (assign29910_e51079 / assign29910_e51082);
        (assign29910_e51083, ((assign29910_e51073 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign29910_e51082), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign29910_e51078) + (assign29910_e51073 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign29910_e51082), ((assign29910_e51073 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign29910_e51082),)
    } else {
        (locals.var_qec, locals.var_qec_dn3, locals.var_qec_dn4, locals.var_qec_dn6,)
    }
};
        locals.var_qec = assign29910_e51085;
        locals.var_qec_dn3 = assign29910_e51085_d_n3;
        locals.var_qec_dn4 = assign29910_e51085_d_n4;
        locals.var_qec_dn6 = assign29910_e51085_d_n6;
        locals.var_qec_rv = 0.0;

        let (assign29920_e51119, assign29920_e51119_d_n3, assign29920_e51119_d_n4, assign29920_e51119_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard502 == 0.0)) {
        let assign29920_e51100: f64 = (-locals.var_pbs_t);
        let assign29920_e51102: f64 = (assign29920_e51100 * locals.var_czbs);
        let (assign29920_e51116, assign29920_e51116_d_n3, assign29920_e51116_d_n4, assign29920_e51116_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign29920_e51108: f64 = (-87.498233534);
                (assign29920_e51108, 0.0, 0.0, 0.0,)
            } else {
                let (assign29920_e51115, assign29920_e51115_d_n3, assign29920_e51115_d_n4, assign29920_e51115_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign29920_e51113: f64 = (locals.var_arg).ln();
                        (assign29920_e51113, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign29920_e51115, assign29920_e51115_d_n3, assign29920_e51115_d_n4, assign29920_e51115_d_n6,)
            }
        };
        let assign29920_e51117: f64 = (assign29920_e51102 * assign29920_e51116);
        (assign29920_e51117, (assign29920_e51102 * assign29920_e51116_d_n3), (((((-locals.var_pbs_t_dn4) * locals.var_czbs) + (assign29920_e51100 * locals.var_czbs_dn4)) * assign29920_e51116) + (assign29920_e51102 * assign29920_e51116_d_n4)), (assign29920_e51102 * assign29920_e51116_d_n6),)
    } else {
        (locals.var_qec, locals.var_qec_dn3, locals.var_qec_dn4, locals.var_qec_dn6,)
    }
};
        locals.var_qec = assign29920_e51119;
        locals.var_qec_dn3 = assign29920_e51119_d_n3;
        locals.var_qec_dn4 = assign29920_e51119_d_n4;
        locals.var_qec_dn6 = assign29920_e51119_d_n6;
        locals.var_qec_rv = 0.0;

        let (assign29930_e51138, assign29930_e51138_d_n3, assign29930_e51138_d_n4, assign29930_e51138_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) {
        let assign29930_e51133: f64 = (locals.var_ves_jct - locals.var_vec1s);
        let assign29930_e51135: f64 = (assign29930_e51133 / locals.var_pb21s);
        let assign29930_e51136: f64 = (1.0 - assign29930_e51135);
        (assign29930_e51136, (-(locals.var_ves_jct_dn3 / locals.var_pb21s)), (-((((-locals.var_vec1s_dn4) * locals.var_pb21s) - (assign29930_e51133 * locals.var_pb21s_dn4)) / (locals.var_pb21s * locals.var_pb21s))), (-(locals.var_ves_jct_dn6 / locals.var_pb21s)),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign29930_e51138;
        locals.var_arg_dn3 = assign29930_e51138_d_n3;
        locals.var_arg_dn4 = assign29930_e51138_d_n4;
        locals.var_arg_dn6 = assign29930_e51138_d_n6;
        locals.var_arg_rv = 0.0;

        let assign29940_e51141: f64 = if p.p1608 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard504 = assign29940_e51141;
        locals.var_guard504_rv = 0.0;

        let assign29950_e51144: f64 = if p.p1608 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard505 = assign29950_e51144;
        locals.var_guard505_rv = 0.0;

        let (assign29960_e51164, assign29960_e51164_d_n3, assign29960_e51164_d_n4, assign29960_e51164_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 != 0.0)) && (locals.var_guard505 != 0.0)) {
        let assign29960_e51161: f64 = (locals.var_arg).sqrt();
        let assign29960_e51162: f64 = (1.0 / assign29960_e51161);
        (assign29960_e51162, (-((locals.var_arg_dn3 / (2.0 * assign29960_e51161)) / (assign29960_e51161 * assign29960_e51161))), (-((locals.var_arg_dn4 / (2.0 * assign29960_e51161)) / (assign29960_e51161 * assign29960_e51161))), (-((locals.var_arg_dn6 / (2.0 * assign29960_e51161)) / (assign29960_e51161 * assign29960_e51161))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29960_e51164;
        locals.var_sarg_dn3 = assign29960_e51164_d_n3;
        locals.var_sarg_dn4 = assign29960_e51164_d_n4;
        locals.var_sarg_dn6 = assign29960_e51164_d_n6;
        locals.var_sarg_rv = 0.0;

        let (assign29970_e51185, assign29970_e51185_d_n3, assign29970_e51185_d_n4, assign29970_e51185_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 != 0.0)) && (locals.var_guard505 == 0.0)) {
        let assign29970_e51182: f64 = (-p.p1608);
        let assign29970_e51183: f64 = (locals.var_arg).powf(assign29970_e51182);
        (assign29970_e51183, if 0.0 == 0.0 && ((assign29970_e51182) as f64).is_finite() && ((assign29970_e51182) as f64).fract() == 0.0 { if assign29970_e51182 == 0.0 { 0.0 } else { (assign29970_e51182 * ((locals.var_arg).powf(assign29970_e51182 - 1.0) * locals.var_arg_dn3)) } } else { (assign29970_e51183 * (assign29970_e51182 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29970_e51182) as f64).is_finite() && ((assign29970_e51182) as f64).fract() == 0.0 { if assign29970_e51182 == 0.0 { 0.0 } else { (assign29970_e51182 * ((locals.var_arg).powf(assign29970_e51182 - 1.0) * locals.var_arg_dn4)) } } else { (assign29970_e51183 * (assign29970_e51182 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign29970_e51182) as f64).is_finite() && ((assign29970_e51182) as f64).fract() == 0.0 { if assign29970_e51182 == 0.0 { 0.0 } else { (assign29970_e51182 * ((locals.var_arg).powf(assign29970_e51182 - 1.0) * locals.var_arg_dn6)) } } else { (assign29970_e51183 * (assign29970_e51182 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign29970_e51185;
        locals.var_sarg_dn3 = assign29970_e51185_d_n3;
        locals.var_sarg_dn4 = assign29970_e51185_d_n4;
        locals.var_sarg_dn6 = assign29970_e51185_d_n6;
        locals.var_sarg_rv = 0.0;

        let (assign29980_e51216, assign29980_e51216_d_n3, assign29980_e51216_d_n4, assign29980_e51216_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 != 0.0)) {
        let assign29980_e51201: f64 = (p.p1602 * locals.var_pb21s);
        let assign29980_e51203: f64 = (assign29980_e51201 * locals.var_czbs);
        let assign29980_e51207: f64 = (locals.var_arg * locals.var_sarg);
        let assign29980_e51208: f64 = (1.0 - assign29980_e51207);
        let assign29980_e51209: f64 = (assign29980_e51203 * assign29980_e51208);
        let assign29980_e51212: f64 = (1.0 - p.p1608);
        let assign29980_e51213: f64 = (assign29980_e51209 / assign29980_e51212);
        let assign29980_e51214: f64 = (locals.var_qec + assign29980_e51213);
        (assign29980_e51214, (locals.var_qec_dn3 + ((assign29980_e51203 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign29980_e51212)), (locals.var_qec_dn4 + ((((((p.p1602 * locals.var_pb21s_dn4) * locals.var_czbs) + (assign29980_e51201 * locals.var_czbs_dn4)) * assign29980_e51208) + (assign29980_e51203 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign29980_e51212)), (locals.var_qec_dn6 + ((assign29980_e51203 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign29980_e51212)),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29980_e51216;
        locals.var_qesj1_dn3 = assign29980_e51216_d_n3;
        locals.var_qesj1_dn4 = assign29980_e51216_d_n4;
        locals.var_qesj1_dn6 = assign29980_e51216_d_n6;
        locals.var_qesj1_rv = 0.0;

        let (assign29990_e51253, assign29990_e51253_d_n3, assign29990_e51253_d_n4, assign29990_e51253_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) && (locals.var_guard499 == 0.0)) && (locals.var_guard504 == 0.0)) {
        let assign29990_e51233: f64 = (p.p1602 * locals.var_pb21s);
        let assign29990_e51235: f64 = (assign29990_e51233 * locals.var_czbs);
        let (assign29990_e51249, assign29990_e51249_d_n3, assign29990_e51249_d_n4, assign29990_e51249_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign29990_e51241: f64 = (-87.498233534);
                (assign29990_e51241, 0.0, 0.0, 0.0,)
            } else {
                let (assign29990_e51248, assign29990_e51248_d_n3, assign29990_e51248_d_n4, assign29990_e51248_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign29990_e51246: f64 = (locals.var_arg).ln();
                        (assign29990_e51246, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign29990_e51248, assign29990_e51248_d_n3, assign29990_e51248_d_n4, assign29990_e51248_d_n6,)
            }
        };
        let assign29990_e51250: f64 = (assign29990_e51235 * assign29990_e51249);
        let assign29990_e51251: f64 = (locals.var_qec - assign29990_e51250);
        (assign29990_e51251, (locals.var_qec_dn3 - (assign29990_e51235 * assign29990_e51249_d_n3)), (locals.var_qec_dn4 - (((((p.p1602 * locals.var_pb21s_dn4) * locals.var_czbs) + (assign29990_e51233 * locals.var_czbs_dn4)) * assign29990_e51249) + (assign29990_e51235 * assign29990_e51249_d_n4))), (locals.var_qec_dn6 - (assign29990_e51235 * assign29990_e51249_d_n6)),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign29990_e51253;
        locals.var_qesj1_dn3 = assign29990_e51253_d_n3;
        locals.var_qesj1_dn4 = assign29990_e51253_d_n4;
        locals.var_qesj1_dn6 = assign29990_e51253_d_n6;
        locals.var_qesj1_rv = 0.0;

        let (assign30000_e51266, assign30000_e51266_d_n3, assign30000_e51266_d_n4, assign30000_e51266_d_n6,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) {
        let assign30000_e51264: f64 = (1.0 - locals.var_t1__blk488);
        (assign30000_e51264, (-locals.var_t1__blk488_dn3), (-locals.var_t1__blk488_dn4), (-locals.var_t1__blk488_dn6),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn6,)
    }
};
        locals.var_arg = assign30000_e51266;
        locals.var_arg_dn3 = assign30000_e51266_d_n3;
        locals.var_arg_dn4 = assign30000_e51266_d_n4;
        locals.var_arg_dn6 = assign30000_e51266_d_n6;
        locals.var_arg_rv = 0.0;

        let assign30010_e51269: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard506 = assign30010_e51269;
        locals.var_guard506_rv = 0.0;

        let assign30020_e51272: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard507 = assign30020_e51272;
        locals.var_guard507_rv = 0.0;

        let (assign30030_e51290, assign30030_e51290_d_n3, assign30030_e51290_d_n4, assign30030_e51290_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 != 0.0)) && (locals.var_guard507 != 0.0)) {
        let assign30030_e51287: f64 = (locals.var_arg).sqrt();
        let assign30030_e51288: f64 = (1.0 / assign30030_e51287);
        (assign30030_e51288, (-((locals.var_arg_dn3 / (2.0 * assign30030_e51287)) / (assign30030_e51287 * assign30030_e51287))), (-((locals.var_arg_dn4 / (2.0 * assign30030_e51287)) / (assign30030_e51287 * assign30030_e51287))), (-((locals.var_arg_dn6 / (2.0 * assign30030_e51287)) / (assign30030_e51287 * assign30030_e51287))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign30030_e51290;
        locals.var_sarg_dn3 = assign30030_e51290_d_n3;
        locals.var_sarg_dn4 = assign30030_e51290_d_n4;
        locals.var_sarg_dn6 = assign30030_e51290_d_n6;
        locals.var_sarg_rv = 0.0;

        let (assign30040_e51309, assign30040_e51309_d_n3, assign30040_e51309_d_n4, assign30040_e51309_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 != 0.0)) && (locals.var_guard507 == 0.0)) {
        let assign30040_e51306: f64 = (-p.p1596);
        let assign30040_e51307: f64 = (locals.var_arg).powf(assign30040_e51306);
        (assign30040_e51307, if 0.0 == 0.0 && ((assign30040_e51306) as f64).is_finite() && ((assign30040_e51306) as f64).fract() == 0.0 { if assign30040_e51306 == 0.0 { 0.0 } else { (assign30040_e51306 * ((locals.var_arg).powf(assign30040_e51306 - 1.0) * locals.var_arg_dn3)) } } else { (assign30040_e51307 * (assign30040_e51306 * (locals.var_arg_dn3 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign30040_e51306) as f64).is_finite() && ((assign30040_e51306) as f64).fract() == 0.0 { if assign30040_e51306 == 0.0 { 0.0 } else { (assign30040_e51306 * ((locals.var_arg).powf(assign30040_e51306 - 1.0) * locals.var_arg_dn4)) } } else { (assign30040_e51307 * (assign30040_e51306 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign30040_e51306) as f64).is_finite() && ((assign30040_e51306) as f64).fract() == 0.0 { if assign30040_e51306 == 0.0 { 0.0 } else { (assign30040_e51306 * ((locals.var_arg).powf(assign30040_e51306 - 1.0) * locals.var_arg_dn6)) } } else { (assign30040_e51307 * (assign30040_e51306 * (locals.var_arg_dn6 / locals.var_arg))) },)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn6,)
    }
};
        locals.var_sarg = assign30040_e51309;
        locals.var_sarg_dn3 = assign30040_e51309_d_n3;
        locals.var_sarg_dn4 = assign30040_e51309_d_n4;
        locals.var_sarg_dn6 = assign30040_e51309_d_n6;
        locals.var_sarg_rv = 0.0;

        let (assign30050_e51334, assign30050_e51334_d_n3, assign30050_e51334_d_n4, assign30050_e51334_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 != 0.0)) {
        let assign30050_e51322: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign30050_e51326: f64 = (locals.var_arg * locals.var_sarg);
        let assign30050_e51327: f64 = (1.0 - assign30050_e51326);
        let assign30050_e51328: f64 = (assign30050_e51322 * assign30050_e51327);
        let assign30050_e51331: f64 = (1.0 - p.p1596);
        let assign30050_e51332: f64 = (assign30050_e51328 / assign30050_e51331);
        (assign30050_e51332, ((assign30050_e51322 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign30050_e51331), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign30050_e51327) + (assign30050_e51322 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign30050_e51331), ((assign30050_e51322 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign30050_e51331),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30050_e51334;
        locals.var_qesj1_dn3 = assign30050_e51334_d_n3;
        locals.var_qesj1_dn4 = assign30050_e51334_d_n4;
        locals.var_qesj1_dn6 = assign30050_e51334_d_n6;
        locals.var_qesj1_rv = 0.0;

        let (assign30060_e51366, assign30060_e51366_d_n3, assign30060_e51366_d_n4, assign30060_e51366_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard506 == 0.0)) {
        let assign30060_e51347: f64 = (-locals.var_pbs_t);
        let assign30060_e51349: f64 = (assign30060_e51347 * locals.var_czbs);
        let (assign30060_e51363, assign30060_e51363_d_n3, assign30060_e51363_d_n4, assign30060_e51363_d_n6,) = {
            if (!(locals.var_arg > 1e-38)) {
                let assign30060_e51355: f64 = (-87.498233534);
                (assign30060_e51355, 0.0, 0.0, 0.0,)
            } else {
                let (assign30060_e51362, assign30060_e51362_d_n3, assign30060_e51362_d_n4, assign30060_e51362_d_n6,) = {
                    if (locals.var_arg > 1e-38) {
                        let assign30060_e51360: f64 = (locals.var_arg).ln();
                        (assign30060_e51360, (locals.var_arg_dn3 / locals.var_arg), (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30060_e51362, assign30060_e51362_d_n3, assign30060_e51362_d_n4, assign30060_e51362_d_n6,)
            }
        };
        let assign30060_e51364: f64 = (assign30060_e51349 * assign30060_e51363);
        (assign30060_e51364, (assign30060_e51349 * assign30060_e51363_d_n3), (((((-locals.var_pbs_t_dn4) * locals.var_czbs) + (assign30060_e51347 * locals.var_czbs_dn4)) * assign30060_e51363) + (assign30060_e51349 * assign30060_e51363_d_n4)), (assign30060_e51349 * assign30060_e51363_d_n6),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30060_e51366;
        locals.var_qesj1_dn3 = assign30060_e51366_d_n3;
        locals.var_qesj1_dn4 = assign30060_e51366_d_n4;
        locals.var_qesj1_dn6 = assign30060_e51366_d_n6;
        locals.var_qesj1_rv = 0.0;

        let assign30070_e51369: f64 = if p.p1596 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign30070_e51369;
        locals.var_guard508_rv = 0.0;

        let assign30080_e51372: f64 = if p.p1596 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard509 = assign30080_e51372;
        locals.var_guard509_rv = 0.0;

        let (assign30090_e51388,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) && (locals.var_guard509 != 0.0)) {
        let assign30090_e51385: f64 = (0.1_f64).sqrt();
        let assign30090_e51386: f64 = (1.0 / assign30090_e51385);
        (assign30090_e51386,)
    } else {
        (locals.var_t2__blk489,)
    }
};
        locals.var_t2__blk489 = assign30090_e51388;
        locals.var_t2__blk489_rv = 0.0;

        let (assign30100_e51405,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) && (locals.var_guard509 == 0.0)) {
        let assign30100_e51402: f64 = (-p.p1596);
        let assign30100_e51403: f64 = (0.1_f64).powf(assign30100_e51402);
        (assign30100_e51403,)
    } else {
        (locals.var_t2__blk489,)
    }
};
        locals.var_t2__blk489 = assign30100_e51405;
        locals.var_t2__blk489_rv = 0.0;

        let (assign30110_e51420,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) {
        let assign30110_e51417: f64 = (1.0 - p.p1596);
        let assign30110_e51418: f64 = (1.0 / assign30110_e51417);
        (assign30110_e51418,)
    } else {
        (locals.var_t3__blk490,)
    }
};
        locals.var_t3__blk490 = assign30110_e51420;
        locals.var_t3__blk490_rv = 0.0;

        let (assign30120_e51443,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 != 0.0)) {
        let assign30120_e51433: f64 = (0.05 * p.p1596);
        let assign30120_e51436: f64 = (1.0 + p.p1596);
        let assign30120_e51437: f64 = (assign30120_e51433 * assign30120_e51436);
        let assign30120_e51439: f64 = (assign30120_e51437 * locals.var_t2__blk489);
        let assign30120_e51440: f64 = (1.0 - assign30120_e51439);
        let assign30120_e51441: f64 = (locals.var_t3__blk490 * assign30120_e51440);
        (assign30120_e51441,)
    } else {
        (locals.var_t5__blk492,)
    }
};
        locals.var_t5__blk492 = assign30120_e51443;
        locals.var_t5__blk492_rv = 0.0;

        let (assign30130_e51455,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk489,)
    }
};
        locals.var_t2__blk489 = assign30130_e51455;
        locals.var_t2__blk489_rv = 0.0;

        let (assign30140_e51470,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard508 == 0.0)) {
        let assign30140_e51467: f64 = (0.1_f64).ln();
        let assign30140_e51468: f64 = (1.5 - assign30140_e51467);
        (assign30140_e51468,)
    } else {
        (locals.var_t5__blk492,)
    }
};
        locals.var_t5__blk492 = assign30140_e51470;
        locals.var_t5__blk492_rv = 0.0;

        let (assign30150_e51495, assign30150_e51495_d_n3, assign30150_e51495_d_n4, assign30150_e51495_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) {
        let assign30150_e51480: f64 = (locals.var_t1__blk488 - 1.0);
        let assign30150_e51481: f64 = (locals.var_t2__blk489 * assign30150_e51480);
        let assign30150_e51484: f64 = (5.0 * p.p1596);
        let assign30150_e51487: f64 = (locals.var_t1__blk488 - 1.0);
        let assign30150_e51488: f64 = (assign30150_e51484 * assign30150_e51487);
        let assign30150_e51491: f64 = (1.0 + p.p1596);
        let assign30150_e51492: f64 = (assign30150_e51488 + assign30150_e51491);
        let assign30150_e51493: f64 = (assign30150_e51481 * assign30150_e51492);
        (assign30150_e51493, (((locals.var_t2__blk489 * locals.var_t1__blk488_dn3) * assign30150_e51492) + (assign30150_e51481 * (assign30150_e51484 * locals.var_t1__blk488_dn3))), (((locals.var_t2__blk489 * locals.var_t1__blk488_dn4) * assign30150_e51492) + (assign30150_e51481 * (assign30150_e51484 * locals.var_t1__blk488_dn4))), (((locals.var_t2__blk489 * locals.var_t1__blk488_dn6) * assign30150_e51492) + (assign30150_e51481 * (assign30150_e51484 * locals.var_t1__blk488_dn6))),)
    } else {
        (locals.var_t4__blk491, locals.var_t4__blk491_dn3, locals.var_t4__blk491_dn4, locals.var_t4__blk491_dn6,)
    }
};
        locals.var_t4__blk491 = assign30150_e51495;
        locals.var_t4__blk491_dn3 = assign30150_e51495_d_n3;
        locals.var_t4__blk491_dn4 = assign30150_e51495_d_n4;
        locals.var_t4__blk491_dn6 = assign30150_e51495_d_n6;
        locals.var_t4__blk491_rv = 0.0;

        let (assign30160_e51510, assign30160_e51510_d_n3, assign30160_e51510_d_n4, assign30160_e51510_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard496 != 0.0)) && (locals.var_guard497 == 0.0)) {
        let assign30160_e51504: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign30160_e51507: f64 = (locals.var_t4__blk491 + locals.var_t5__blk492);
        let assign30160_e51508: f64 = (assign30160_e51504 * assign30160_e51507);
        (assign30160_e51508, (assign30160_e51504 * locals.var_t4__blk491_dn3), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign30160_e51507) + (assign30160_e51504 * locals.var_t4__blk491_dn4)), (assign30160_e51504 * locals.var_t4__blk491_dn6),)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30160_e51510;
        locals.var_qesj1_dn3 = assign30160_e51510_d_n3;
        locals.var_qesj1_dn4 = assign30160_e51510_d_n4;
        locals.var_qesj1_dn6 = assign30160_e51510_d_n6;
        locals.var_qesj1_rv = 0.0;

        let (assign30170_e51517, assign30170_e51517_d_n3, assign30170_e51517_d_n4, assign30170_e51517_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard496 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qesj1, locals.var_qesj1_dn3, locals.var_qesj1_dn4, locals.var_qesj1_dn6,)
    }
};
        locals.var_qesj1 = assign30170_e51517;
        locals.var_qesj1_dn3 = assign30170_e51517_d_n3;
        locals.var_qesj1_dn4 = assign30170_e51517_d_n4;
        locals.var_qesj1_dn6 = assign30170_e51517_d_n6;
        locals.var_qesj1_rv = 0.0;

        let assign30180_e51520: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard518 = assign30180_e51520;
        locals.var_guard518_rv = 0.0;

        let (assign30190_e51528, assign30190_e51528_d_n3, assign30190_e51528_d_n4, assign30190_e51528_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) {
        let assign30190_e51526: f64 = (locals.var_ves_jct / locals.var_pbsws_t);
        (assign30190_e51526, (locals.var_ves_jct_dn3 / locals.var_pbsws_t), (-((locals.var_ves_jct * locals.var_pbsws_t_dn4) / (locals.var_pbsws_t * locals.var_pbsws_t))), (locals.var_ves_jct_dn6 / locals.var_pbsws_t),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn3, locals.var_t1__blk510_dn4, locals.var_t1__blk510_dn6,)
    }
};
        locals.var_t1__blk510 = assign30190_e51528;
        locals.var_t1__blk510_dn3 = assign30190_e51528_d_n3;
        locals.var_t1__blk510_dn4 = assign30190_e51528_d_n4;
        locals.var_t1__blk510_dn6 = assign30190_e51528_d_n6;
        locals.var_t1__blk510_rv = 0.0;

        let assign30200_e51531: f64 = if locals.var_t1__blk510 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard519 = assign30200_e51531;
        locals.var_guard519_rv = 0.0;

        let assign30210_e51534: f64 = if p.p1604 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard520 = assign30210_e51534;
        locals.var_guard520_rv = 0.0;

        let assign30220_e51537: f64 = if locals.var_ves_jct > locals.var_vec2s { 1.0 } else { 0.0 };
        locals.var_guard521 = assign30220_e51537;
        locals.var_guard521_rv = 0.0;

        let (assign30230_e51551, assign30230_e51551_d_n3, assign30230_e51551_d_n4, assign30230_e51551_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign30230_e51549: f64 = (1.0 - locals.var_t1__blk510);
        (assign30230_e51549, (-locals.var_t1__blk510_dn3), (-locals.var_t1__blk510_dn4), (-locals.var_t1__blk510_dn6),)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30230_e51551;
        locals.var_arg__blk515_dn3 = assign30230_e51551_d_n3;
        locals.var_arg__blk515_dn4 = assign30230_e51551_d_n4;
        locals.var_arg__blk515_dn6 = assign30230_e51551_d_n6;
        locals.var_arg__blk515_rv = 0.0;

        let assign30240_e51554: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard522 = assign30240_e51554;
        locals.var_guard522_rv = 0.0;

        let assign30250_e51557: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard523 = assign30250_e51557;
        locals.var_guard523_rv = 0.0;

        let (assign30260_e51576, assign30260_e51576_d_n3, assign30260_e51576_d_n4, assign30260_e51576_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 != 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign30260_e51573: f64 = (locals.var_arg__blk515).sqrt();
        let assign30260_e51574: f64 = (1.0 / assign30260_e51573);
        (assign30260_e51574, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30260_e51573)) / (assign30260_e51573 * assign30260_e51573))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30260_e51573)) / (assign30260_e51573 * assign30260_e51573))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30260_e51573)) / (assign30260_e51573 * assign30260_e51573))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30260_e51576;
        locals.var_sarg__blk516_dn3 = assign30260_e51576_d_n3;
        locals.var_sarg__blk516_dn4 = assign30260_e51576_d_n4;
        locals.var_sarg__blk516_dn6 = assign30260_e51576_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30270_e51596, assign30270_e51596_d_n3, assign30270_e51596_d_n4, assign30270_e51596_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 != 0.0)) && (locals.var_guard523 == 0.0)) {
        let assign30270_e51593: f64 = (-p.p1598);
        let assign30270_e51594: f64 = (locals.var_arg__blk515).powf(assign30270_e51593);
        (assign30270_e51594, if 0.0 == 0.0 && ((assign30270_e51593) as f64).is_finite() && ((assign30270_e51593) as f64).fract() == 0.0 { if assign30270_e51593 == 0.0 { 0.0 } else { (assign30270_e51593 * ((locals.var_arg__blk515).powf(assign30270_e51593 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30270_e51594 * (assign30270_e51593 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30270_e51593) as f64).is_finite() && ((assign30270_e51593) as f64).fract() == 0.0 { if assign30270_e51593 == 0.0 { 0.0 } else { (assign30270_e51593 * ((locals.var_arg__blk515).powf(assign30270_e51593 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30270_e51594 * (assign30270_e51593 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30270_e51593) as f64).is_finite() && ((assign30270_e51593) as f64).fract() == 0.0 { if assign30270_e51593 == 0.0 { 0.0 } else { (assign30270_e51593 * ((locals.var_arg__blk515).powf(assign30270_e51593 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30270_e51594 * (assign30270_e51593 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30270_e51596;
        locals.var_sarg__blk516_dn3 = assign30270_e51596_d_n3;
        locals.var_sarg__blk516_dn4 = assign30270_e51596_d_n4;
        locals.var_sarg__blk516_dn6 = assign30270_e51596_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30280_e51622, assign30280_e51622_d_n3, assign30280_e51622_d_n4, assign30280_e51622_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 != 0.0)) {
        let assign30280_e51610: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30280_e51614: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30280_e51615: f64 = (1.0 - assign30280_e51614);
        let assign30280_e51616: f64 = (assign30280_e51610 * assign30280_e51615);
        let assign30280_e51619: f64 = (1.0 - p.p1598);
        let assign30280_e51620: f64 = (assign30280_e51616 / assign30280_e51619);
        (assign30280_e51620, ((assign30280_e51610 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30280_e51619), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30280_e51615) + (assign30280_e51610 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30280_e51619), ((assign30280_e51610 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30280_e51619),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30280_e51622;
        locals.var_qesj2_dn3 = assign30280_e51622_d_n3;
        locals.var_qesj2_dn4 = assign30280_e51622_d_n4;
        locals.var_qesj2_dn6 = assign30280_e51622_d_n6;
        locals.var_qesj2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_121(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30290_e51655, assign30290_e51655_d_n3, assign30290_e51655_d_n4, assign30290_e51655_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 == 0.0)) {
        let assign30290_e51636: f64 = (-locals.var_pbsws_t);
        let assign30290_e51638: f64 = (assign30290_e51636 * locals.var_czbssw);
        let (assign30290_e51652, assign30290_e51652_d_n3, assign30290_e51652_d_n4, assign30290_e51652_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30290_e51644: f64 = (-87.498233534);
                (assign30290_e51644, 0.0, 0.0, 0.0,)
            } else {
                let (assign30290_e51651, assign30290_e51651_d_n3, assign30290_e51651_d_n4, assign30290_e51651_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30290_e51649: f64 = (locals.var_arg__blk515).ln();
                        (assign30290_e51649, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30290_e51651, assign30290_e51651_d_n3, assign30290_e51651_d_n4, assign30290_e51651_d_n6,)
            }
        };
        let assign30290_e51653: f64 = (assign30290_e51638 * assign30290_e51652);
        (assign30290_e51653, (assign30290_e51638 * assign30290_e51652_d_n3), (((((-locals.var_pbsws_t_dn4) * locals.var_czbssw) + (assign30290_e51636 * locals.var_czbssw_dn4)) * assign30290_e51652) + (assign30290_e51638 * assign30290_e51652_d_n4)), (assign30290_e51638 * assign30290_e51652_d_n6),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30290_e51655;
        locals.var_qesj2_dn3 = assign30290_e51655_d_n3;
        locals.var_qesj2_dn4 = assign30290_e51655_d_n4;
        locals.var_qesj2_dn6 = assign30290_e51655_d_n6;
        locals.var_qesj2_rv = 0.0;

        let (assign30300_e51672, assign30300_e51672_d_n3, assign30300_e51672_d_n4, assign30300_e51672_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) {
        let assign30300_e51669: f64 = (locals.var_vec2s / locals.var_pbsws_t);
        let assign30300_e51670: f64 = (1.0 - assign30300_e51669);
        (assign30300_e51670, 0.0, (-(((locals.var_vec2s_dn4 * locals.var_pbsws_t) - (locals.var_vec2s * locals.var_pbsws_t_dn4)) / (locals.var_pbsws_t * locals.var_pbsws_t))), 0.0,)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30300_e51672;
        locals.var_arg__blk515_dn3 = assign30300_e51672_d_n3;
        locals.var_arg__blk515_dn4 = assign30300_e51672_d_n4;
        locals.var_arg__blk515_dn6 = assign30300_e51672_d_n6;
        locals.var_arg__blk515_rv = 0.0;

        let assign30310_e51675: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard524 = assign30310_e51675;
        locals.var_guard524_rv = 0.0;

        let assign30320_e51678: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign30320_e51678;
        locals.var_guard525_rv = 0.0;

        let (assign30330_e51698, assign30330_e51698_d_n3, assign30330_e51698_d_n4, assign30330_e51698_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 != 0.0)) && (locals.var_guard525 != 0.0)) {
        let assign30330_e51695: f64 = (locals.var_arg__blk515).sqrt();
        let assign30330_e51696: f64 = (1.0 / assign30330_e51695);
        (assign30330_e51696, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30330_e51695)) / (assign30330_e51695 * assign30330_e51695))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30330_e51695)) / (assign30330_e51695 * assign30330_e51695))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30330_e51695)) / (assign30330_e51695 * assign30330_e51695))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30330_e51698;
        locals.var_sarg__blk516_dn3 = assign30330_e51698_d_n3;
        locals.var_sarg__blk516_dn4 = assign30330_e51698_d_n4;
        locals.var_sarg__blk516_dn6 = assign30330_e51698_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30340_e51719, assign30340_e51719_d_n3, assign30340_e51719_d_n4, assign30340_e51719_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 != 0.0)) && (locals.var_guard525 == 0.0)) {
        let assign30340_e51716: f64 = (-p.p1598);
        let assign30340_e51717: f64 = (locals.var_arg__blk515).powf(assign30340_e51716);
        (assign30340_e51717, if 0.0 == 0.0 && ((assign30340_e51716) as f64).is_finite() && ((assign30340_e51716) as f64).fract() == 0.0 { if assign30340_e51716 == 0.0 { 0.0 } else { (assign30340_e51716 * ((locals.var_arg__blk515).powf(assign30340_e51716 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30340_e51717 * (assign30340_e51716 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30340_e51716) as f64).is_finite() && ((assign30340_e51716) as f64).fract() == 0.0 { if assign30340_e51716 == 0.0 { 0.0 } else { (assign30340_e51716 * ((locals.var_arg__blk515).powf(assign30340_e51716 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30340_e51717 * (assign30340_e51716 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30340_e51716) as f64).is_finite() && ((assign30340_e51716) as f64).fract() == 0.0 { if assign30340_e51716 == 0.0 { 0.0 } else { (assign30340_e51716 * ((locals.var_arg__blk515).powf(assign30340_e51716 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30340_e51717 * (assign30340_e51716 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30340_e51719;
        locals.var_sarg__blk516_dn3 = assign30340_e51719_d_n3;
        locals.var_sarg__blk516_dn4 = assign30340_e51719_d_n4;
        locals.var_sarg__blk516_dn6 = assign30340_e51719_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30350_e51746, assign30350_e51746_d_n3, assign30350_e51746_d_n4, assign30350_e51746_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 != 0.0)) {
        let assign30350_e51734: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30350_e51738: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30350_e51739: f64 = (1.0 - assign30350_e51738);
        let assign30350_e51740: f64 = (assign30350_e51734 * assign30350_e51739);
        let assign30350_e51743: f64 = (1.0 - p.p1598);
        let assign30350_e51744: f64 = (assign30350_e51740 / assign30350_e51743);
        (assign30350_e51744, ((assign30350_e51734 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30350_e51743), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30350_e51739) + (assign30350_e51734 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30350_e51743), ((assign30350_e51734 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30350_e51743),)
    } else {
        (locals.var_qec__blk517, locals.var_qec__blk517_dn3, locals.var_qec__blk517_dn4, locals.var_qec__blk517_dn6,)
    }
};
        locals.var_qec__blk517 = assign30350_e51746;
        locals.var_qec__blk517_dn3 = assign30350_e51746_d_n3;
        locals.var_qec__blk517_dn4 = assign30350_e51746_d_n4;
        locals.var_qec__blk517_dn6 = assign30350_e51746_d_n6;
        locals.var_qec__blk517_rv = 0.0;

        let (assign30360_e51780, assign30360_e51780_d_n3, assign30360_e51780_d_n4, assign30360_e51780_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign30360_e51761: f64 = (-locals.var_pbsws_t);
        let assign30360_e51763: f64 = (assign30360_e51761 * locals.var_czbssw);
        let (assign30360_e51777, assign30360_e51777_d_n3, assign30360_e51777_d_n4, assign30360_e51777_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30360_e51769: f64 = (-87.498233534);
                (assign30360_e51769, 0.0, 0.0, 0.0,)
            } else {
                let (assign30360_e51776, assign30360_e51776_d_n3, assign30360_e51776_d_n4, assign30360_e51776_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30360_e51774: f64 = (locals.var_arg__blk515).ln();
                        (assign30360_e51774, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30360_e51776, assign30360_e51776_d_n3, assign30360_e51776_d_n4, assign30360_e51776_d_n6,)
            }
        };
        let assign30360_e51778: f64 = (assign30360_e51763 * assign30360_e51777);
        (assign30360_e51778, (assign30360_e51763 * assign30360_e51777_d_n3), (((((-locals.var_pbsws_t_dn4) * locals.var_czbssw) + (assign30360_e51761 * locals.var_czbssw_dn4)) * assign30360_e51777) + (assign30360_e51763 * assign30360_e51777_d_n4)), (assign30360_e51763 * assign30360_e51777_d_n6),)
    } else {
        (locals.var_qec__blk517, locals.var_qec__blk517_dn3, locals.var_qec__blk517_dn4, locals.var_qec__blk517_dn6,)
    }
};
        locals.var_qec__blk517 = assign30360_e51780;
        locals.var_qec__blk517_dn3 = assign30360_e51780_d_n3;
        locals.var_qec__blk517_dn4 = assign30360_e51780_d_n4;
        locals.var_qec__blk517_dn6 = assign30360_e51780_d_n6;
        locals.var_qec__blk517_rv = 0.0;

        let (assign30370_e51799, assign30370_e51799_d_n3, assign30370_e51799_d_n4, assign30370_e51799_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) {
        let assign30370_e51794: f64 = (locals.var_ves_jct - locals.var_vec2s);
        let assign30370_e51796: f64 = (assign30370_e51794 / locals.var_pb22s);
        let assign30370_e51797: f64 = (1.0 - assign30370_e51796);
        (assign30370_e51797, (-(locals.var_ves_jct_dn3 / locals.var_pb22s)), (-((((-locals.var_vec2s_dn4) * locals.var_pb22s) - (assign30370_e51794 * locals.var_pb22s_dn4)) / (locals.var_pb22s * locals.var_pb22s))), (-(locals.var_ves_jct_dn6 / locals.var_pb22s)),)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30370_e51799;
        locals.var_arg__blk515_dn3 = assign30370_e51799_d_n3;
        locals.var_arg__blk515_dn4 = assign30370_e51799_d_n4;
        locals.var_arg__blk515_dn6 = assign30370_e51799_d_n6;
        locals.var_arg__blk515_rv = 0.0;

        let assign30380_e51802: f64 = if p.p1610 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard526 = assign30380_e51802;
        locals.var_guard526_rv = 0.0;

        let assign30390_e51805: f64 = if p.p1610 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign30390_e51805;
        locals.var_guard527_rv = 0.0;

        let (assign30400_e51825, assign30400_e51825_d_n3, assign30400_e51825_d_n4, assign30400_e51825_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 != 0.0)) {
        let assign30400_e51822: f64 = (locals.var_arg__blk515).sqrt();
        let assign30400_e51823: f64 = (1.0 / assign30400_e51822);
        (assign30400_e51823, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30400_e51822)) / (assign30400_e51822 * assign30400_e51822))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30400_e51822)) / (assign30400_e51822 * assign30400_e51822))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30400_e51822)) / (assign30400_e51822 * assign30400_e51822))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30400_e51825;
        locals.var_sarg__blk516_dn3 = assign30400_e51825_d_n3;
        locals.var_sarg__blk516_dn4 = assign30400_e51825_d_n4;
        locals.var_sarg__blk516_dn6 = assign30400_e51825_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30410_e51846, assign30410_e51846_d_n3, assign30410_e51846_d_n4, assign30410_e51846_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 == 0.0)) {
        let assign30410_e51843: f64 = (-p.p1610);
        let assign30410_e51844: f64 = (locals.var_arg__blk515).powf(assign30410_e51843);
        (assign30410_e51844, if 0.0 == 0.0 && ((assign30410_e51843) as f64).is_finite() && ((assign30410_e51843) as f64).fract() == 0.0 { if assign30410_e51843 == 0.0 { 0.0 } else { (assign30410_e51843 * ((locals.var_arg__blk515).powf(assign30410_e51843 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30410_e51844 * (assign30410_e51843 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30410_e51843) as f64).is_finite() && ((assign30410_e51843) as f64).fract() == 0.0 { if assign30410_e51843 == 0.0 { 0.0 } else { (assign30410_e51843 * ((locals.var_arg__blk515).powf(assign30410_e51843 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30410_e51844 * (assign30410_e51843 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30410_e51843) as f64).is_finite() && ((assign30410_e51843) as f64).fract() == 0.0 { if assign30410_e51843 == 0.0 { 0.0 } else { (assign30410_e51843 * ((locals.var_arg__blk515).powf(assign30410_e51843 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30410_e51844 * (assign30410_e51843 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30410_e51846;
        locals.var_sarg__blk516_dn3 = assign30410_e51846_d_n3;
        locals.var_sarg__blk516_dn4 = assign30410_e51846_d_n4;
        locals.var_sarg__blk516_dn6 = assign30410_e51846_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30420_e51877, assign30420_e51877_d_n3, assign30420_e51877_d_n4, assign30420_e51877_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign30420_e51862: f64 = (p.p1604 * locals.var_pb22s);
        let assign30420_e51864: f64 = (assign30420_e51862 * locals.var_czbssw);
        let assign30420_e51868: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30420_e51869: f64 = (1.0 - assign30420_e51868);
        let assign30420_e51870: f64 = (assign30420_e51864 * assign30420_e51869);
        let assign30420_e51873: f64 = (1.0 - p.p1610);
        let assign30420_e51874: f64 = (assign30420_e51870 / assign30420_e51873);
        let assign30420_e51875: f64 = (locals.var_qec__blk517 + assign30420_e51874);
        (assign30420_e51875, (locals.var_qec__blk517_dn3 + ((assign30420_e51864 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30420_e51873)), (locals.var_qec__blk517_dn4 + ((((((p.p1604 * locals.var_pb22s_dn4) * locals.var_czbssw) + (assign30420_e51862 * locals.var_czbssw_dn4)) * assign30420_e51869) + (assign30420_e51864 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30420_e51873)), (locals.var_qec__blk517_dn6 + ((assign30420_e51864 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30420_e51873)),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30420_e51877;
        locals.var_qesj2_dn3 = assign30420_e51877_d_n3;
        locals.var_qesj2_dn4 = assign30420_e51877_d_n4;
        locals.var_qesj2_dn6 = assign30420_e51877_d_n6;
        locals.var_qesj2_rv = 0.0;

        let (assign30430_e51914, assign30430_e51914_d_n3, assign30430_e51914_d_n4, assign30430_e51914_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 != 0.0)) && (locals.var_guard521 == 0.0)) && (locals.var_guard526 == 0.0)) {
        let assign30430_e51894: f64 = (p.p1604 * locals.var_pb22s);
        let assign30430_e51896: f64 = (assign30430_e51894 * locals.var_czbssw);
        let (assign30430_e51910, assign30430_e51910_d_n3, assign30430_e51910_d_n4, assign30430_e51910_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30430_e51902: f64 = (-87.498233534);
                (assign30430_e51902, 0.0, 0.0, 0.0,)
            } else {
                let (assign30430_e51909, assign30430_e51909_d_n3, assign30430_e51909_d_n4, assign30430_e51909_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30430_e51907: f64 = (locals.var_arg__blk515).ln();
                        (assign30430_e51907, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30430_e51909, assign30430_e51909_d_n3, assign30430_e51909_d_n4, assign30430_e51909_d_n6,)
            }
        };
        let assign30430_e51911: f64 = (assign30430_e51896 * assign30430_e51910);
        let assign30430_e51912: f64 = (locals.var_qec__blk517 - assign30430_e51911);
        (assign30430_e51912, (locals.var_qec__blk517_dn3 - (assign30430_e51896 * assign30430_e51910_d_n3)), (locals.var_qec__blk517_dn4 - (((((p.p1604 * locals.var_pb22s_dn4) * locals.var_czbssw) + (assign30430_e51894 * locals.var_czbssw_dn4)) * assign30430_e51910) + (assign30430_e51896 * assign30430_e51910_d_n4))), (locals.var_qec__blk517_dn6 - (assign30430_e51896 * assign30430_e51910_d_n6)),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30430_e51914;
        locals.var_qesj2_dn3 = assign30430_e51914_d_n3;
        locals.var_qesj2_dn4 = assign30430_e51914_d_n4;
        locals.var_qesj2_dn6 = assign30430_e51914_d_n6;
        locals.var_qesj2_rv = 0.0;

        let (assign30440_e51927, assign30440_e51927_d_n3, assign30440_e51927_d_n4, assign30440_e51927_d_n6,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign30440_e51925: f64 = (1.0 - locals.var_t1__blk510);
        (assign30440_e51925, (-locals.var_t1__blk510_dn3), (-locals.var_t1__blk510_dn4), (-locals.var_t1__blk510_dn6),)
    } else {
        (locals.var_arg__blk515, locals.var_arg__blk515_dn3, locals.var_arg__blk515_dn4, locals.var_arg__blk515_dn6,)
    }
};
        locals.var_arg__blk515 = assign30440_e51927;
        locals.var_arg__blk515_dn3 = assign30440_e51927_d_n3;
        locals.var_arg__blk515_dn4 = assign30440_e51927_d_n4;
        locals.var_arg__blk515_dn6 = assign30440_e51927_d_n6;
        locals.var_arg__blk515_rv = 0.0;

        let assign30450_e51930: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard528 = assign30450_e51930;
        locals.var_guard528_rv = 0.0;

        let assign30460_e51933: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign30460_e51933;
        locals.var_guard529_rv = 0.0;

        let (assign30470_e51951, assign30470_e51951_d_n3, assign30470_e51951_d_n4, assign30470_e51951_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 != 0.0)) {
        let assign30470_e51948: f64 = (locals.var_arg__blk515).sqrt();
        let assign30470_e51949: f64 = (1.0 / assign30470_e51948);
        (assign30470_e51949, (-((locals.var_arg__blk515_dn3 / (2.0 * assign30470_e51948)) / (assign30470_e51948 * assign30470_e51948))), (-((locals.var_arg__blk515_dn4 / (2.0 * assign30470_e51948)) / (assign30470_e51948 * assign30470_e51948))), (-((locals.var_arg__blk515_dn6 / (2.0 * assign30470_e51948)) / (assign30470_e51948 * assign30470_e51948))),)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30470_e51951;
        locals.var_sarg__blk516_dn3 = assign30470_e51951_d_n3;
        locals.var_sarg__blk516_dn4 = assign30470_e51951_d_n4;
        locals.var_sarg__blk516_dn6 = assign30470_e51951_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30480_e51970, assign30480_e51970_d_n3, assign30480_e51970_d_n4, assign30480_e51970_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 == 0.0)) {
        let assign30480_e51967: f64 = (-p.p1598);
        let assign30480_e51968: f64 = (locals.var_arg__blk515).powf(assign30480_e51967);
        (assign30480_e51968, if 0.0 == 0.0 && ((assign30480_e51967) as f64).is_finite() && ((assign30480_e51967) as f64).fract() == 0.0 { if assign30480_e51967 == 0.0 { 0.0 } else { (assign30480_e51967 * ((locals.var_arg__blk515).powf(assign30480_e51967 - 1.0) * locals.var_arg__blk515_dn3)) } } else { (assign30480_e51968 * (assign30480_e51967 * (locals.var_arg__blk515_dn3 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30480_e51967) as f64).is_finite() && ((assign30480_e51967) as f64).fract() == 0.0 { if assign30480_e51967 == 0.0 { 0.0 } else { (assign30480_e51967 * ((locals.var_arg__blk515).powf(assign30480_e51967 - 1.0) * locals.var_arg__blk515_dn4)) } } else { (assign30480_e51968 * (assign30480_e51967 * (locals.var_arg__blk515_dn4 / locals.var_arg__blk515))) }, if 0.0 == 0.0 && ((assign30480_e51967) as f64).is_finite() && ((assign30480_e51967) as f64).fract() == 0.0 { if assign30480_e51967 == 0.0 { 0.0 } else { (assign30480_e51967 * ((locals.var_arg__blk515).powf(assign30480_e51967 - 1.0) * locals.var_arg__blk515_dn6)) } } else { (assign30480_e51968 * (assign30480_e51967 * (locals.var_arg__blk515_dn6 / locals.var_arg__blk515))) },)
    } else {
        (locals.var_sarg__blk516, locals.var_sarg__blk516_dn3, locals.var_sarg__blk516_dn4, locals.var_sarg__blk516_dn6,)
    }
};
        locals.var_sarg__blk516 = assign30480_e51970;
        locals.var_sarg__blk516_dn3 = assign30480_e51970_d_n3;
        locals.var_sarg__blk516_dn4 = assign30480_e51970_d_n4;
        locals.var_sarg__blk516_dn6 = assign30480_e51970_d_n6;
        locals.var_sarg__blk516_rv = 0.0;

        let (assign30490_e51995, assign30490_e51995_d_n3, assign30490_e51995_d_n4, assign30490_e51995_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 != 0.0)) {
        let assign30490_e51983: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30490_e51987: f64 = (locals.var_arg__blk515 * locals.var_sarg__blk516);
        let assign30490_e51988: f64 = (1.0 - assign30490_e51987);
        let assign30490_e51989: f64 = (assign30490_e51983 * assign30490_e51988);
        let assign30490_e51992: f64 = (1.0 - p.p1598);
        let assign30490_e51993: f64 = (assign30490_e51989 / assign30490_e51992);
        (assign30490_e51993, ((assign30490_e51983 * (-((locals.var_arg__blk515_dn3 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn3)))) / assign30490_e51992), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30490_e51988) + (assign30490_e51983 * (-((locals.var_arg__blk515_dn4 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn4))))) / assign30490_e51992), ((assign30490_e51983 * (-((locals.var_arg__blk515_dn6 * locals.var_sarg__blk516) + (locals.var_arg__blk515 * locals.var_sarg__blk516_dn6)))) / assign30490_e51992),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30490_e51995;
        locals.var_qesj2_dn3 = assign30490_e51995_d_n3;
        locals.var_qesj2_dn4 = assign30490_e51995_d_n4;
        locals.var_qesj2_dn6 = assign30490_e51995_d_n6;
        locals.var_qesj2_rv = 0.0;

        let (assign30500_e52027, assign30500_e52027_d_n3, assign30500_e52027_d_n4, assign30500_e52027_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 != 0.0)) && (locals.var_guard520 == 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign30500_e52008: f64 = (-locals.var_pbsws_t);
        let assign30500_e52010: f64 = (assign30500_e52008 * locals.var_czbssw);
        let (assign30500_e52024, assign30500_e52024_d_n3, assign30500_e52024_d_n4, assign30500_e52024_d_n6,) = {
            if (!(locals.var_arg__blk515 > 1e-38)) {
                let assign30500_e52016: f64 = (-87.498233534);
                (assign30500_e52016, 0.0, 0.0, 0.0,)
            } else {
                let (assign30500_e52023, assign30500_e52023_d_n3, assign30500_e52023_d_n4, assign30500_e52023_d_n6,) = {
                    if (locals.var_arg__blk515 > 1e-38) {
                        let assign30500_e52021: f64 = (locals.var_arg__blk515).ln();
                        (assign30500_e52021, (locals.var_arg__blk515_dn3 / locals.var_arg__blk515), (locals.var_arg__blk515_dn4 / locals.var_arg__blk515), (locals.var_arg__blk515_dn6 / locals.var_arg__blk515),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30500_e52023, assign30500_e52023_d_n3, assign30500_e52023_d_n4, assign30500_e52023_d_n6,)
            }
        };
        let assign30500_e52025: f64 = (assign30500_e52010 * assign30500_e52024);
        (assign30500_e52025, (assign30500_e52010 * assign30500_e52024_d_n3), (((((-locals.var_pbsws_t_dn4) * locals.var_czbssw) + (assign30500_e52008 * locals.var_czbssw_dn4)) * assign30500_e52024) + (assign30500_e52010 * assign30500_e52024_d_n4)), (assign30500_e52010 * assign30500_e52024_d_n6),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30500_e52027;
        locals.var_qesj2_dn3 = assign30500_e52027_d_n3;
        locals.var_qesj2_dn4 = assign30500_e52027_d_n4;
        locals.var_qesj2_dn6 = assign30500_e52027_d_n6;
        locals.var_qesj2_rv = 0.0;

        let assign30510_e52030: f64 = if p.p1598 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign30510_e52030;
        locals.var_guard530_rv = 0.0;

        let assign30520_e52033: f64 = if p.p1598 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign30520_e52033;
        locals.var_guard531_rv = 0.0;

        let (assign30530_e52049,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) && (locals.var_guard531 != 0.0)) {
        let assign30530_e52046: f64 = (0.1_f64).sqrt();
        let assign30530_e52047: f64 = (1.0 / assign30530_e52046);
        (assign30530_e52047,)
    } else {
        (locals.var_t2__blk511,)
    }
};
        locals.var_t2__blk511 = assign30530_e52049;
        locals.var_t2__blk511_rv = 0.0;

        let (assign30540_e52066,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) && (locals.var_guard531 == 0.0)) {
        let assign30540_e52063: f64 = (-p.p1598);
        let assign30540_e52064: f64 = (0.1_f64).powf(assign30540_e52063);
        (assign30540_e52064,)
    } else {
        (locals.var_t2__blk511,)
    }
};
        locals.var_t2__blk511 = assign30540_e52066;
        locals.var_t2__blk511_rv = 0.0;

        let (assign30550_e52081,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) {
        let assign30550_e52078: f64 = (1.0 - p.p1598);
        let assign30550_e52079: f64 = (1.0 / assign30550_e52078);
        (assign30550_e52079,)
    } else {
        (locals.var_t3__blk512,)
    }
};
        locals.var_t3__blk512 = assign30550_e52081;
        locals.var_t3__blk512_rv = 0.0;

        let (assign30560_e52104,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 != 0.0)) {
        let assign30560_e52094: f64 = (0.05 * p.p1598);
        let assign30560_e52097: f64 = (1.0 + p.p1598);
        let assign30560_e52098: f64 = (assign30560_e52094 * assign30560_e52097);
        let assign30560_e52100: f64 = (assign30560_e52098 * locals.var_t2__blk511);
        let assign30560_e52101: f64 = (1.0 - assign30560_e52100);
        let assign30560_e52102: f64 = (locals.var_t3__blk512 * assign30560_e52101);
        (assign30560_e52102,)
    } else {
        (locals.var_t5__blk514,)
    }
};
        locals.var_t5__blk514 = assign30560_e52104;
        locals.var_t5__blk514_rv = 0.0;

        let (assign30570_e52116,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk511,)
    }
};
        locals.var_t2__blk511 = assign30570_e52116;
        locals.var_t2__blk511_rv = 0.0;

        let (assign30580_e52131,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) && (locals.var_guard530 == 0.0)) {
        let assign30580_e52128: f64 = (0.1_f64).ln();
        let assign30580_e52129: f64 = (1.5 - assign30580_e52128);
        (assign30580_e52129,)
    } else {
        (locals.var_t5__blk514,)
    }
};
        locals.var_t5__blk514 = assign30580_e52131;
        locals.var_t5__blk514_rv = 0.0;

        let (assign30590_e52156, assign30590_e52156_d_n3, assign30590_e52156_d_n4, assign30590_e52156_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) {
        let assign30590_e52141: f64 = (locals.var_t1__blk510 - 1.0);
        let assign30590_e52142: f64 = (locals.var_t2__blk511 * assign30590_e52141);
        let assign30590_e52145: f64 = (5.0 * p.p1598);
        let assign30590_e52148: f64 = (locals.var_t1__blk510 - 1.0);
        let assign30590_e52149: f64 = (assign30590_e52145 * assign30590_e52148);
        let assign30590_e52152: f64 = (1.0 + p.p1598);
        let assign30590_e52153: f64 = (assign30590_e52149 + assign30590_e52152);
        let assign30590_e52154: f64 = (assign30590_e52142 * assign30590_e52153);
        (assign30590_e52154, (((locals.var_t2__blk511 * locals.var_t1__blk510_dn3) * assign30590_e52153) + (assign30590_e52142 * (assign30590_e52145 * locals.var_t1__blk510_dn3))), (((locals.var_t2__blk511 * locals.var_t1__blk510_dn4) * assign30590_e52153) + (assign30590_e52142 * (assign30590_e52145 * locals.var_t1__blk510_dn4))), (((locals.var_t2__blk511 * locals.var_t1__blk510_dn6) * assign30590_e52153) + (assign30590_e52142 * (assign30590_e52145 * locals.var_t1__blk510_dn6))),)
    } else {
        (locals.var_t4__blk513, locals.var_t4__blk513_dn3, locals.var_t4__blk513_dn4, locals.var_t4__blk513_dn6,)
    }
};
        locals.var_t4__blk513 = assign30590_e52156;
        locals.var_t4__blk513_dn3 = assign30590_e52156_d_n3;
        locals.var_t4__blk513_dn4 = assign30590_e52156_d_n4;
        locals.var_t4__blk513_dn6 = assign30590_e52156_d_n6;
        locals.var_t4__blk513_rv = 0.0;

        let (assign30600_e52171, assign30600_e52171_d_n3, assign30600_e52171_d_n4, assign30600_e52171_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard518 != 0.0)) && (locals.var_guard519 == 0.0)) {
        let assign30600_e52165: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign30600_e52168: f64 = (locals.var_t4__blk513 + locals.var_t5__blk514);
        let assign30600_e52169: f64 = (assign30600_e52165 * assign30600_e52168);
        (assign30600_e52169, (assign30600_e52165 * locals.var_t4__blk513_dn3), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign30600_e52168) + (assign30600_e52165 * locals.var_t4__blk513_dn4)), (assign30600_e52165 * locals.var_t4__blk513_dn6),)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30600_e52171;
        locals.var_qesj2_dn3 = assign30600_e52171_d_n3;
        locals.var_qesj2_dn4 = assign30600_e52171_d_n4;
        locals.var_qesj2_dn6 = assign30600_e52171_d_n6;
        locals.var_qesj2_rv = 0.0;

        let (assign30610_e52178, assign30610_e52178_d_n3, assign30610_e52178_d_n4, assign30610_e52178_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard518 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qesj2, locals.var_qesj2_dn3, locals.var_qesj2_dn4, locals.var_qesj2_dn6,)
    }
};
        locals.var_qesj2 = assign30610_e52178;
        locals.var_qesj2_dn3 = assign30610_e52178_d_n3;
        locals.var_qesj2_dn4 = assign30610_e52178_d_n4;
        locals.var_qesj2_dn6 = assign30610_e52178_d_n6;
        locals.var_qesj2_rv = 0.0;

        let assign30620_e52181: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign30620_e52181;
        locals.var_guard540_rv = 0.0;

        let (assign30630_e52189, assign30630_e52189_d_n3, assign30630_e52189_d_n4, assign30630_e52189_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) {
        let assign30630_e52187: f64 = (locals.var_ves_jct / locals.var_pbswgs_t);
        (assign30630_e52187, (locals.var_ves_jct_dn3 / locals.var_pbswgs_t), (-((locals.var_ves_jct * locals.var_pbswgs_t_dn4) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), (locals.var_ves_jct_dn6 / locals.var_pbswgs_t),)
    } else {
        (locals.var_t1__blk532, locals.var_t1__blk532_dn3, locals.var_t1__blk532_dn4, locals.var_t1__blk532_dn6,)
    }
};
        locals.var_t1__blk532 = assign30630_e52189;
        locals.var_t1__blk532_dn3 = assign30630_e52189_d_n3;
        locals.var_t1__blk532_dn4 = assign30630_e52189_d_n4;
        locals.var_t1__blk532_dn6 = assign30630_e52189_d_n6;
        locals.var_t1__blk532_rv = 0.0;

        let assign30640_e52192: f64 = if locals.var_t1__blk532 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign30640_e52192;
        locals.var_guard541_rv = 0.0;

        let assign30650_e52195: f64 = if p.p1606 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign30650_e52195;
        locals.var_guard542_rv = 0.0;

        let assign30660_e52198: f64 = if locals.var_ves_jct > locals.var_vec3s { 1.0 } else { 0.0 };
        locals.var_guard543 = assign30660_e52198;
        locals.var_guard543_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_122(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30670_e52212, assign30670_e52212_d_n3, assign30670_e52212_d_n4, assign30670_e52212_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) {
        let assign30670_e52210: f64 = (1.0 - locals.var_t1__blk532);
        (assign30670_e52210, (-locals.var_t1__blk532_dn3), (-locals.var_t1__blk532_dn4), (-locals.var_t1__blk532_dn6),)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30670_e52212;
        locals.var_arg__blk537_dn3 = assign30670_e52212_d_n3;
        locals.var_arg__blk537_dn4 = assign30670_e52212_d_n4;
        locals.var_arg__blk537_dn6 = assign30670_e52212_d_n6;
        locals.var_arg__blk537_rv = 0.0;

        let assign30680_e52215: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign30680_e52215;
        locals.var_guard544_rv = 0.0;

        let assign30690_e52218: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign30690_e52218;
        locals.var_guard545_rv = 0.0;

        let (assign30700_e52237, assign30700_e52237_d_n3, assign30700_e52237_d_n4, assign30700_e52237_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 != 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign30700_e52234: f64 = (locals.var_arg__blk537).sqrt();
        let assign30700_e52235: f64 = (1.0 / assign30700_e52234);
        (assign30700_e52235, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30700_e52234)) / (assign30700_e52234 * assign30700_e52234))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30700_e52234)) / (assign30700_e52234 * assign30700_e52234))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30700_e52234)) / (assign30700_e52234 * assign30700_e52234))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30700_e52237;
        locals.var_sarg__blk538_dn3 = assign30700_e52237_d_n3;
        locals.var_sarg__blk538_dn4 = assign30700_e52237_d_n4;
        locals.var_sarg__blk538_dn6 = assign30700_e52237_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30710_e52257, assign30710_e52257_d_n3, assign30710_e52257_d_n4, assign30710_e52257_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 != 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign30710_e52254: f64 = (-p.p1600);
        let assign30710_e52255: f64 = (locals.var_arg__blk537).powf(assign30710_e52254);
        (assign30710_e52255, if 0.0 == 0.0 && ((assign30710_e52254) as f64).is_finite() && ((assign30710_e52254) as f64).fract() == 0.0 { if assign30710_e52254 == 0.0 { 0.0 } else { (assign30710_e52254 * ((locals.var_arg__blk537).powf(assign30710_e52254 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30710_e52255 * (assign30710_e52254 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30710_e52254) as f64).is_finite() && ((assign30710_e52254) as f64).fract() == 0.0 { if assign30710_e52254 == 0.0 { 0.0 } else { (assign30710_e52254 * ((locals.var_arg__blk537).powf(assign30710_e52254 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30710_e52255 * (assign30710_e52254 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30710_e52254) as f64).is_finite() && ((assign30710_e52254) as f64).fract() == 0.0 { if assign30710_e52254 == 0.0 { 0.0 } else { (assign30710_e52254 * ((locals.var_arg__blk537).powf(assign30710_e52254 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30710_e52255 * (assign30710_e52254 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30710_e52257;
        locals.var_sarg__blk538_dn3 = assign30710_e52257_d_n3;
        locals.var_sarg__blk538_dn4 = assign30710_e52257_d_n4;
        locals.var_sarg__blk538_dn6 = assign30710_e52257_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30720_e52283, assign30720_e52283_d_n3, assign30720_e52283_d_n4, assign30720_e52283_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 != 0.0)) {
        let assign30720_e52271: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign30720_e52275: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30720_e52276: f64 = (1.0 - assign30720_e52275);
        let assign30720_e52277: f64 = (assign30720_e52271 * assign30720_e52276);
        let assign30720_e52280: f64 = (1.0 - p.p1600);
        let assign30720_e52281: f64 = (assign30720_e52277 / assign30720_e52280);
        (assign30720_e52281, ((assign30720_e52271 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30720_e52280), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign30720_e52276) + (assign30720_e52271 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30720_e52280), ((assign30720_e52271 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30720_e52280),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30720_e52283;
        locals.var_qesj3_dn3 = assign30720_e52283_d_n3;
        locals.var_qesj3_dn4 = assign30720_e52283_d_n4;
        locals.var_qesj3_dn6 = assign30720_e52283_d_n6;
        locals.var_qesj3_rv = 0.0;

        let (assign30730_e52316, assign30730_e52316_d_n3, assign30730_e52316_d_n4, assign30730_e52316_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 != 0.0)) && (locals.var_guard544 == 0.0)) {
        let assign30730_e52297: f64 = (-locals.var_pbswgs_t);
        let assign30730_e52299: f64 = (assign30730_e52297 * locals.var_czbsswg);
        let (assign30730_e52313, assign30730_e52313_d_n3, assign30730_e52313_d_n4, assign30730_e52313_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30730_e52305: f64 = (-87.498233534);
                (assign30730_e52305, 0.0, 0.0, 0.0,)
            } else {
                let (assign30730_e52312, assign30730_e52312_d_n3, assign30730_e52312_d_n4, assign30730_e52312_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30730_e52310: f64 = (locals.var_arg__blk537).ln();
                        (assign30730_e52310, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30730_e52312, assign30730_e52312_d_n3, assign30730_e52312_d_n4, assign30730_e52312_d_n6,)
            }
        };
        let assign30730_e52314: f64 = (assign30730_e52299 * assign30730_e52313);
        (assign30730_e52314, (assign30730_e52299 * assign30730_e52313_d_n3), (((((-locals.var_pbswgs_t_dn4) * locals.var_czbsswg) + (assign30730_e52297 * locals.var_czbsswg_dn4)) * assign30730_e52313) + (assign30730_e52299 * assign30730_e52313_d_n4)), (assign30730_e52299 * assign30730_e52313_d_n6),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30730_e52316;
        locals.var_qesj3_dn3 = assign30730_e52316_d_n3;
        locals.var_qesj3_dn4 = assign30730_e52316_d_n4;
        locals.var_qesj3_dn6 = assign30730_e52316_d_n6;
        locals.var_qesj3_rv = 0.0;

        let (assign30740_e52333, assign30740_e52333_d_n3, assign30740_e52333_d_n4, assign30740_e52333_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) {
        let assign30740_e52330: f64 = (locals.var_vec3s / locals.var_pbswgs_t);
        let assign30740_e52331: f64 = (1.0 - assign30740_e52330);
        (assign30740_e52331, 0.0, (-(((locals.var_vec3s_dn4 * locals.var_pbswgs_t) - (locals.var_vec3s * locals.var_pbswgs_t_dn4)) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), 0.0,)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30740_e52333;
        locals.var_arg__blk537_dn3 = assign30740_e52333_d_n3;
        locals.var_arg__blk537_dn4 = assign30740_e52333_d_n4;
        locals.var_arg__blk537_dn6 = assign30740_e52333_d_n6;
        locals.var_arg__blk537_rv = 0.0;

        let assign30750_e52336: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign30750_e52336;
        locals.var_guard546_rv = 0.0;

        let assign30760_e52339: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign30760_e52339;
        locals.var_guard547_rv = 0.0;

        let (assign30770_e52359, assign30770_e52359_d_n3, assign30770_e52359_d_n4, assign30770_e52359_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 != 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign30770_e52356: f64 = (locals.var_arg__blk537).sqrt();
        let assign30770_e52357: f64 = (1.0 / assign30770_e52356);
        (assign30770_e52357, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30770_e52356)) / (assign30770_e52356 * assign30770_e52356))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30770_e52356)) / (assign30770_e52356 * assign30770_e52356))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30770_e52356)) / (assign30770_e52356 * assign30770_e52356))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30770_e52359;
        locals.var_sarg__blk538_dn3 = assign30770_e52359_d_n3;
        locals.var_sarg__blk538_dn4 = assign30770_e52359_d_n4;
        locals.var_sarg__blk538_dn6 = assign30770_e52359_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30780_e52380, assign30780_e52380_d_n3, assign30780_e52380_d_n4, assign30780_e52380_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 != 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign30780_e52377: f64 = (-p.p1600);
        let assign30780_e52378: f64 = (locals.var_arg__blk537).powf(assign30780_e52377);
        (assign30780_e52378, if 0.0 == 0.0 && ((assign30780_e52377) as f64).is_finite() && ((assign30780_e52377) as f64).fract() == 0.0 { if assign30780_e52377 == 0.0 { 0.0 } else { (assign30780_e52377 * ((locals.var_arg__blk537).powf(assign30780_e52377 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30780_e52378 * (assign30780_e52377 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30780_e52377) as f64).is_finite() && ((assign30780_e52377) as f64).fract() == 0.0 { if assign30780_e52377 == 0.0 { 0.0 } else { (assign30780_e52377 * ((locals.var_arg__blk537).powf(assign30780_e52377 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30780_e52378 * (assign30780_e52377 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30780_e52377) as f64).is_finite() && ((assign30780_e52377) as f64).fract() == 0.0 { if assign30780_e52377 == 0.0 { 0.0 } else { (assign30780_e52377 * ((locals.var_arg__blk537).powf(assign30780_e52377 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30780_e52378 * (assign30780_e52377 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30780_e52380;
        locals.var_sarg__blk538_dn3 = assign30780_e52380_d_n3;
        locals.var_sarg__blk538_dn4 = assign30780_e52380_d_n4;
        locals.var_sarg__blk538_dn6 = assign30780_e52380_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30790_e52407, assign30790_e52407_d_n3, assign30790_e52407_d_n4, assign30790_e52407_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 != 0.0)) {
        let assign30790_e52395: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign30790_e52399: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30790_e52400: f64 = (1.0 - assign30790_e52399);
        let assign30790_e52401: f64 = (assign30790_e52395 * assign30790_e52400);
        let assign30790_e52404: f64 = (1.0 - p.p1600);
        let assign30790_e52405: f64 = (assign30790_e52401 / assign30790_e52404);
        (assign30790_e52405, ((assign30790_e52395 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30790_e52404), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign30790_e52400) + (assign30790_e52395 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30790_e52404), ((assign30790_e52395 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30790_e52404),)
    } else {
        (locals.var_qec__blk539, locals.var_qec__blk539_dn3, locals.var_qec__blk539_dn4, locals.var_qec__blk539_dn6,)
    }
};
        locals.var_qec__blk539 = assign30790_e52407;
        locals.var_qec__blk539_dn3 = assign30790_e52407_d_n3;
        locals.var_qec__blk539_dn4 = assign30790_e52407_d_n4;
        locals.var_qec__blk539_dn6 = assign30790_e52407_d_n6;
        locals.var_qec__blk539_rv = 0.0;

        let (assign30800_e52441, assign30800_e52441_d_n3, assign30800_e52441_d_n4, assign30800_e52441_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard546 == 0.0)) {
        let assign30800_e52422: f64 = (-locals.var_pbswgs_t);
        let assign30800_e52424: f64 = (assign30800_e52422 * locals.var_czbsswg);
        let (assign30800_e52438, assign30800_e52438_d_n3, assign30800_e52438_d_n4, assign30800_e52438_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30800_e52430: f64 = (-87.498233534);
                (assign30800_e52430, 0.0, 0.0, 0.0,)
            } else {
                let (assign30800_e52437, assign30800_e52437_d_n3, assign30800_e52437_d_n4, assign30800_e52437_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30800_e52435: f64 = (locals.var_arg__blk537).ln();
                        (assign30800_e52435, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30800_e52437, assign30800_e52437_d_n3, assign30800_e52437_d_n4, assign30800_e52437_d_n6,)
            }
        };
        let assign30800_e52439: f64 = (assign30800_e52424 * assign30800_e52438);
        (assign30800_e52439, (assign30800_e52424 * assign30800_e52438_d_n3), (((((-locals.var_pbswgs_t_dn4) * locals.var_czbsswg) + (assign30800_e52422 * locals.var_czbsswg_dn4)) * assign30800_e52438) + (assign30800_e52424 * assign30800_e52438_d_n4)), (assign30800_e52424 * assign30800_e52438_d_n6),)
    } else {
        (locals.var_qec__blk539, locals.var_qec__blk539_dn3, locals.var_qec__blk539_dn4, locals.var_qec__blk539_dn6,)
    }
};
        locals.var_qec__blk539 = assign30800_e52441;
        locals.var_qec__blk539_dn3 = assign30800_e52441_d_n3;
        locals.var_qec__blk539_dn4 = assign30800_e52441_d_n4;
        locals.var_qec__blk539_dn6 = assign30800_e52441_d_n6;
        locals.var_qec__blk539_rv = 0.0;

        let (assign30810_e52460, assign30810_e52460_d_n3, assign30810_e52460_d_n4, assign30810_e52460_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) {
        let assign30810_e52455: f64 = (locals.var_ves_jct - locals.var_vec3s);
        let assign30810_e52457: f64 = (assign30810_e52455 / locals.var_pb23s);
        let assign30810_e52458: f64 = (1.0 - assign30810_e52457);
        (assign30810_e52458, (-(locals.var_ves_jct_dn3 / locals.var_pb23s)), (-((((-locals.var_vec3s_dn4) * locals.var_pb23s) - (assign30810_e52455 * locals.var_pb23s_dn4)) / (locals.var_pb23s * locals.var_pb23s))), (-(locals.var_ves_jct_dn6 / locals.var_pb23s)),)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30810_e52460;
        locals.var_arg__blk537_dn3 = assign30810_e52460_d_n3;
        locals.var_arg__blk537_dn4 = assign30810_e52460_d_n4;
        locals.var_arg__blk537_dn6 = assign30810_e52460_d_n6;
        locals.var_arg__blk537_rv = 0.0;

        let assign30820_e52463: f64 = if p.p1612 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign30820_e52463;
        locals.var_guard548_rv = 0.0;

        let assign30830_e52466: f64 = if p.p1612 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign30830_e52466;
        locals.var_guard549_rv = 0.0;

        let (assign30840_e52486, assign30840_e52486_d_n3, assign30840_e52486_d_n4, assign30840_e52486_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 != 0.0)) && (locals.var_guard549 != 0.0)) {
        let assign30840_e52483: f64 = (locals.var_arg__blk537).sqrt();
        let assign30840_e52484: f64 = (1.0 / assign30840_e52483);
        (assign30840_e52484, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30840_e52483)) / (assign30840_e52483 * assign30840_e52483))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30840_e52483)) / (assign30840_e52483 * assign30840_e52483))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30840_e52483)) / (assign30840_e52483 * assign30840_e52483))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30840_e52486;
        locals.var_sarg__blk538_dn3 = assign30840_e52486_d_n3;
        locals.var_sarg__blk538_dn4 = assign30840_e52486_d_n4;
        locals.var_sarg__blk538_dn6 = assign30840_e52486_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30850_e52507, assign30850_e52507_d_n3, assign30850_e52507_d_n4, assign30850_e52507_d_n6,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 != 0.0)) && (locals.var_guard549 == 0.0)) {
        let assign30850_e52504: f64 = (-p.p1612);
        let assign30850_e52505: f64 = (locals.var_arg__blk537).powf(assign30850_e52504);
        (assign30850_e52505, if 0.0 == 0.0 && ((assign30850_e52504) as f64).is_finite() && ((assign30850_e52504) as f64).fract() == 0.0 { if assign30850_e52504 == 0.0 { 0.0 } else { (assign30850_e52504 * ((locals.var_arg__blk537).powf(assign30850_e52504 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30850_e52505 * (assign30850_e52504 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30850_e52504) as f64).is_finite() && ((assign30850_e52504) as f64).fract() == 0.0 { if assign30850_e52504 == 0.0 { 0.0 } else { (assign30850_e52504 * ((locals.var_arg__blk537).powf(assign30850_e52504 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30850_e52505 * (assign30850_e52504 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30850_e52504) as f64).is_finite() && ((assign30850_e52504) as f64).fract() == 0.0 { if assign30850_e52504 == 0.0 { 0.0 } else { (assign30850_e52504 * ((locals.var_arg__blk537).powf(assign30850_e52504 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30850_e52505 * (assign30850_e52504 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30850_e52507;
        locals.var_sarg__blk538_dn3 = assign30850_e52507_d_n3;
        locals.var_sarg__blk538_dn4 = assign30850_e52507_d_n4;
        locals.var_sarg__blk538_dn6 = assign30850_e52507_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30860_e52538, assign30860_e52538_d_n3, assign30860_e52538_d_n4, assign30860_e52538_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 != 0.0)) {
        let assign30860_e52523: f64 = (p.p1606 * locals.var_pb23s);
        let assign30860_e52525: f64 = (assign30860_e52523 * locals.var_czbsswg);
        let assign30860_e52529: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30860_e52530: f64 = (1.0 - assign30860_e52529);
        let assign30860_e52531: f64 = (assign30860_e52525 * assign30860_e52530);
        let assign30860_e52534: f64 = (1.0 - p.p1612);
        let assign30860_e52535: f64 = (assign30860_e52531 / assign30860_e52534);
        let assign30860_e52536: f64 = (locals.var_qec__blk539 + assign30860_e52535);
        (assign30860_e52536, (locals.var_qec__blk539_dn3 + ((assign30860_e52525 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30860_e52534)), (locals.var_qec__blk539_dn4 + ((((((p.p1606 * locals.var_pb23s_dn4) * locals.var_czbsswg) + (assign30860_e52523 * locals.var_czbsswg_dn4)) * assign30860_e52530) + (assign30860_e52525 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30860_e52534)), (locals.var_qec__blk539_dn6 + ((assign30860_e52525 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30860_e52534)),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30860_e52538;
        locals.var_qesj3_dn3 = assign30860_e52538_d_n3;
        locals.var_qesj3_dn4 = assign30860_e52538_d_n4;
        locals.var_qesj3_dn6 = assign30860_e52538_d_n6;
        locals.var_qesj3_rv = 0.0;

        let (assign30870_e52575, assign30870_e52575_d_n3, assign30870_e52575_d_n4, assign30870_e52575_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 != 0.0)) && (locals.var_guard543 == 0.0)) && (locals.var_guard548 == 0.0)) {
        let assign30870_e52555: f64 = (p.p1606 * locals.var_pb23s);
        let assign30870_e52557: f64 = (assign30870_e52555 * locals.var_czbsswg);
        let (assign30870_e52571, assign30870_e52571_d_n3, assign30870_e52571_d_n4, assign30870_e52571_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30870_e52563: f64 = (-87.498233534);
                (assign30870_e52563, 0.0, 0.0, 0.0,)
            } else {
                let (assign30870_e52570, assign30870_e52570_d_n3, assign30870_e52570_d_n4, assign30870_e52570_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30870_e52568: f64 = (locals.var_arg__blk537).ln();
                        (assign30870_e52568, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30870_e52570, assign30870_e52570_d_n3, assign30870_e52570_d_n4, assign30870_e52570_d_n6,)
            }
        };
        let assign30870_e52572: f64 = (assign30870_e52557 * assign30870_e52571);
        let assign30870_e52573: f64 = (locals.var_qec__blk539 - assign30870_e52572);
        (assign30870_e52573, (locals.var_qec__blk539_dn3 - (assign30870_e52557 * assign30870_e52571_d_n3)), (locals.var_qec__blk539_dn4 - (((((p.p1606 * locals.var_pb23s_dn4) * locals.var_czbsswg) + (assign30870_e52555 * locals.var_czbsswg_dn4)) * assign30870_e52571) + (assign30870_e52557 * assign30870_e52571_d_n4))), (locals.var_qec__blk539_dn6 - (assign30870_e52557 * assign30870_e52571_d_n6)),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30870_e52575;
        locals.var_qesj3_dn3 = assign30870_e52575_d_n3;
        locals.var_qesj3_dn4 = assign30870_e52575_d_n4;
        locals.var_qesj3_dn6 = assign30870_e52575_d_n6;
        locals.var_qesj3_rv = 0.0;

        let (assign30880_e52588, assign30880_e52588_d_n3, assign30880_e52588_d_n4, assign30880_e52588_d_n6,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) {
        let assign30880_e52586: f64 = (1.0 - locals.var_t1__blk532);
        (assign30880_e52586, (-locals.var_t1__blk532_dn3), (-locals.var_t1__blk532_dn4), (-locals.var_t1__blk532_dn6),)
    } else {
        (locals.var_arg__blk537, locals.var_arg__blk537_dn3, locals.var_arg__blk537_dn4, locals.var_arg__blk537_dn6,)
    }
};
        locals.var_arg__blk537 = assign30880_e52588;
        locals.var_arg__blk537_dn3 = assign30880_e52588_d_n3;
        locals.var_arg__blk537_dn4 = assign30880_e52588_d_n4;
        locals.var_arg__blk537_dn6 = assign30880_e52588_d_n6;
        locals.var_arg__blk537_rv = 0.0;

        let assign30890_e52591: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign30890_e52591;
        locals.var_guard550_rv = 0.0;

        let assign30900_e52594: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign30900_e52594;
        locals.var_guard551_rv = 0.0;

        let (assign30910_e52612, assign30910_e52612_d_n3, assign30910_e52612_d_n4, assign30910_e52612_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 != 0.0)) && (locals.var_guard551 != 0.0)) {
        let assign30910_e52609: f64 = (locals.var_arg__blk537).sqrt();
        let assign30910_e52610: f64 = (1.0 / assign30910_e52609);
        (assign30910_e52610, (-((locals.var_arg__blk537_dn3 / (2.0 * assign30910_e52609)) / (assign30910_e52609 * assign30910_e52609))), (-((locals.var_arg__blk537_dn4 / (2.0 * assign30910_e52609)) / (assign30910_e52609 * assign30910_e52609))), (-((locals.var_arg__blk537_dn6 / (2.0 * assign30910_e52609)) / (assign30910_e52609 * assign30910_e52609))),)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30910_e52612;
        locals.var_sarg__blk538_dn3 = assign30910_e52612_d_n3;
        locals.var_sarg__blk538_dn4 = assign30910_e52612_d_n4;
        locals.var_sarg__blk538_dn6 = assign30910_e52612_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30920_e52631, assign30920_e52631_d_n3, assign30920_e52631_d_n4, assign30920_e52631_d_n6,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 != 0.0)) && (locals.var_guard551 == 0.0)) {
        let assign30920_e52628: f64 = (-p.p1600);
        let assign30920_e52629: f64 = (locals.var_arg__blk537).powf(assign30920_e52628);
        (assign30920_e52629, if 0.0 == 0.0 && ((assign30920_e52628) as f64).is_finite() && ((assign30920_e52628) as f64).fract() == 0.0 { if assign30920_e52628 == 0.0 { 0.0 } else { (assign30920_e52628 * ((locals.var_arg__blk537).powf(assign30920_e52628 - 1.0) * locals.var_arg__blk537_dn3)) } } else { (assign30920_e52629 * (assign30920_e52628 * (locals.var_arg__blk537_dn3 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30920_e52628) as f64).is_finite() && ((assign30920_e52628) as f64).fract() == 0.0 { if assign30920_e52628 == 0.0 { 0.0 } else { (assign30920_e52628 * ((locals.var_arg__blk537).powf(assign30920_e52628 - 1.0) * locals.var_arg__blk537_dn4)) } } else { (assign30920_e52629 * (assign30920_e52628 * (locals.var_arg__blk537_dn4 / locals.var_arg__blk537))) }, if 0.0 == 0.0 && ((assign30920_e52628) as f64).is_finite() && ((assign30920_e52628) as f64).fract() == 0.0 { if assign30920_e52628 == 0.0 { 0.0 } else { (assign30920_e52628 * ((locals.var_arg__blk537).powf(assign30920_e52628 - 1.0) * locals.var_arg__blk537_dn6)) } } else { (assign30920_e52629 * (assign30920_e52628 * (locals.var_arg__blk537_dn6 / locals.var_arg__blk537))) },)
    } else {
        (locals.var_sarg__blk538, locals.var_sarg__blk538_dn3, locals.var_sarg__blk538_dn4, locals.var_sarg__blk538_dn6,)
    }
};
        locals.var_sarg__blk538 = assign30920_e52631;
        locals.var_sarg__blk538_dn3 = assign30920_e52631_d_n3;
        locals.var_sarg__blk538_dn4 = assign30920_e52631_d_n4;
        locals.var_sarg__blk538_dn6 = assign30920_e52631_d_n6;
        locals.var_sarg__blk538_rv = 0.0;

        let (assign30930_e52656, assign30930_e52656_d_n3, assign30930_e52656_d_n4, assign30930_e52656_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 != 0.0)) {
        let assign30930_e52644: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign30930_e52648: f64 = (locals.var_arg__blk537 * locals.var_sarg__blk538);
        let assign30930_e52649: f64 = (1.0 - assign30930_e52648);
        let assign30930_e52650: f64 = (assign30930_e52644 * assign30930_e52649);
        let assign30930_e52653: f64 = (1.0 - p.p1600);
        let assign30930_e52654: f64 = (assign30930_e52650 / assign30930_e52653);
        (assign30930_e52654, ((assign30930_e52644 * (-((locals.var_arg__blk537_dn3 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn3)))) / assign30930_e52653), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign30930_e52649) + (assign30930_e52644 * (-((locals.var_arg__blk537_dn4 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn4))))) / assign30930_e52653), ((assign30930_e52644 * (-((locals.var_arg__blk537_dn6 * locals.var_sarg__blk538) + (locals.var_arg__blk537 * locals.var_sarg__blk538_dn6)))) / assign30930_e52653),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30930_e52656;
        locals.var_qesj3_dn3 = assign30930_e52656_d_n3;
        locals.var_qesj3_dn4 = assign30930_e52656_d_n4;
        locals.var_qesj3_dn6 = assign30930_e52656_d_n6;
        locals.var_qesj3_rv = 0.0;

        let (assign30940_e52688, assign30940_e52688_d_n3, assign30940_e52688_d_n4, assign30940_e52688_d_n6,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) && (locals.var_guard542 == 0.0)) && (locals.var_guard550 == 0.0)) {
        let assign30940_e52669: f64 = (-locals.var_pbswgs_t);
        let assign30940_e52671: f64 = (assign30940_e52669 * locals.var_czbsswg);
        let (assign30940_e52685, assign30940_e52685_d_n3, assign30940_e52685_d_n4, assign30940_e52685_d_n6,) = {
            if (!(locals.var_arg__blk537 > 1e-38)) {
                let assign30940_e52677: f64 = (-87.498233534);
                (assign30940_e52677, 0.0, 0.0, 0.0,)
            } else {
                let (assign30940_e52684, assign30940_e52684_d_n3, assign30940_e52684_d_n4, assign30940_e52684_d_n6,) = {
                    if (locals.var_arg__blk537 > 1e-38) {
                        let assign30940_e52682: f64 = (locals.var_arg__blk537).ln();
                        (assign30940_e52682, (locals.var_arg__blk537_dn3 / locals.var_arg__blk537), (locals.var_arg__blk537_dn4 / locals.var_arg__blk537), (locals.var_arg__blk537_dn6 / locals.var_arg__blk537),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign30940_e52684, assign30940_e52684_d_n3, assign30940_e52684_d_n4, assign30940_e52684_d_n6,)
            }
        };
        let assign30940_e52686: f64 = (assign30940_e52671 * assign30940_e52685);
        (assign30940_e52686, (assign30940_e52671 * assign30940_e52685_d_n3), (((((-locals.var_pbswgs_t_dn4) * locals.var_czbsswg) + (assign30940_e52669 * locals.var_czbsswg_dn4)) * assign30940_e52685) + (assign30940_e52671 * assign30940_e52685_d_n4)), (assign30940_e52671 * assign30940_e52685_d_n6),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign30940_e52688;
        locals.var_qesj3_dn3 = assign30940_e52688_d_n3;
        locals.var_qesj3_dn4 = assign30940_e52688_d_n4;
        locals.var_qesj3_dn6 = assign30940_e52688_d_n6;
        locals.var_qesj3_rv = 0.0;

        let assign30950_e52691: f64 = if p.p1600 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign30950_e52691;
        locals.var_guard552_rv = 0.0;

        let assign30960_e52694: f64 = if p.p1600 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign30960_e52694;
        locals.var_guard553_rv = 0.0;

        let (assign30970_e52710,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) && (locals.var_guard553 != 0.0)) {
        let assign30970_e52707: f64 = (0.1_f64).sqrt();
        let assign30970_e52708: f64 = (1.0 / assign30970_e52707);
        (assign30970_e52708,)
    } else {
        (locals.var_t2__blk533,)
    }
};
        locals.var_t2__blk533 = assign30970_e52710;
        locals.var_t2__blk533_rv = 0.0;

        let (assign30980_e52727,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) && (locals.var_guard553 == 0.0)) {
        let assign30980_e52724: f64 = (-p.p1600);
        let assign30980_e52725: f64 = (0.1_f64).powf(assign30980_e52724);
        (assign30980_e52725,)
    } else {
        (locals.var_t2__blk533,)
    }
};
        locals.var_t2__blk533 = assign30980_e52727;
        locals.var_t2__blk533_rv = 0.0;

        let (assign30990_e52742,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) {
        let assign30990_e52739: f64 = (1.0 - p.p1600);
        let assign30990_e52740: f64 = (1.0 / assign30990_e52739);
        (assign30990_e52740,)
    } else {
        (locals.var_t3__blk534,)
    }
};
        locals.var_t3__blk534 = assign30990_e52742;
        locals.var_t3__blk534_rv = 0.0;

        let (assign31000_e52765,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 != 0.0)) {
        let assign31000_e52755: f64 = (0.05 * p.p1600);
        let assign31000_e52758: f64 = (1.0 + p.p1600);
        let assign31000_e52759: f64 = (assign31000_e52755 * assign31000_e52758);
        let assign31000_e52761: f64 = (assign31000_e52759 * locals.var_t2__blk533);
        let assign31000_e52762: f64 = (1.0 - assign31000_e52761);
        let assign31000_e52763: f64 = (locals.var_t3__blk534 * assign31000_e52762);
        (assign31000_e52763,)
    } else {
        (locals.var_t5__blk536,)
    }
};
        locals.var_t5__blk536 = assign31000_e52765;
        locals.var_t5__blk536_rv = 0.0;

        let (assign31010_e52777,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk533,)
    }
};
        locals.var_t2__blk533 = assign31010_e52777;
        locals.var_t2__blk533_rv = 0.0;

        let (assign31020_e52792,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign31020_e52789: f64 = (0.1_f64).ln();
        let assign31020_e52790: f64 = (1.5 - assign31020_e52789);
        (assign31020_e52790,)
    } else {
        (locals.var_t5__blk536,)
    }
};
        locals.var_t5__blk536 = assign31020_e52792;
        locals.var_t5__blk536_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_123(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31030_e52817, assign31030_e52817_d_n3, assign31030_e52817_d_n4, assign31030_e52817_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) {
        let assign31030_e52802: f64 = (locals.var_t1__blk532 - 1.0);
        let assign31030_e52803: f64 = (locals.var_t2__blk533 * assign31030_e52802);
        let assign31030_e52806: f64 = (5.0 * p.p1600);
        let assign31030_e52809: f64 = (locals.var_t1__blk532 - 1.0);
        let assign31030_e52810: f64 = (assign31030_e52806 * assign31030_e52809);
        let assign31030_e52813: f64 = (1.0 + p.p1600);
        let assign31030_e52814: f64 = (assign31030_e52810 + assign31030_e52813);
        let assign31030_e52815: f64 = (assign31030_e52803 * assign31030_e52814);
        (assign31030_e52815, (((locals.var_t2__blk533 * locals.var_t1__blk532_dn3) * assign31030_e52814) + (assign31030_e52803 * (assign31030_e52806 * locals.var_t1__blk532_dn3))), (((locals.var_t2__blk533 * locals.var_t1__blk532_dn4) * assign31030_e52814) + (assign31030_e52803 * (assign31030_e52806 * locals.var_t1__blk532_dn4))), (((locals.var_t2__blk533 * locals.var_t1__blk532_dn6) * assign31030_e52814) + (assign31030_e52803 * (assign31030_e52806 * locals.var_t1__blk532_dn6))),)
    } else {
        (locals.var_t4__blk535, locals.var_t4__blk535_dn3, locals.var_t4__blk535_dn4, locals.var_t4__blk535_dn6,)
    }
};
        locals.var_t4__blk535 = assign31030_e52817;
        locals.var_t4__blk535_dn3 = assign31030_e52817_d_n3;
        locals.var_t4__blk535_dn4 = assign31030_e52817_d_n4;
        locals.var_t4__blk535_dn6 = assign31030_e52817_d_n6;
        locals.var_t4__blk535_rv = 0.0;

        let (assign31040_e52832, assign31040_e52832_d_n3, assign31040_e52832_d_n4, assign31040_e52832_d_n6,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 == 0.0)) {
        let assign31040_e52826: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign31040_e52829: f64 = (locals.var_t4__blk535 + locals.var_t5__blk536);
        let assign31040_e52830: f64 = (assign31040_e52826 * assign31040_e52829);
        (assign31040_e52830, (assign31040_e52826 * locals.var_t4__blk535_dn3), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign31040_e52829) + (assign31040_e52826 * locals.var_t4__blk535_dn4)), (assign31040_e52826 * locals.var_t4__blk535_dn6),)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign31040_e52832;
        locals.var_qesj3_dn3 = assign31040_e52832_d_n3;
        locals.var_qesj3_dn4 = assign31040_e52832_d_n4;
        locals.var_qesj3_dn6 = assign31040_e52832_d_n6;
        locals.var_qesj3_rv = 0.0;

        let (assign31050_e52839, assign31050_e52839_d_n3, assign31050_e52839_d_n4, assign31050_e52839_d_n6,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard540 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qesj3, locals.var_qesj3_dn3, locals.var_qesj3_dn4, locals.var_qesj3_dn6,)
    }
};
        locals.var_qesj3 = assign31050_e52839;
        locals.var_qesj3_dn3 = assign31050_e52839_d_n3;
        locals.var_qesj3_dn4 = assign31050_e52839_d_n4;
        locals.var_qesj3_dn6 = assign31050_e52839_d_n6;
        locals.var_qesj3_rv = 0.0;

        let (assign31060_e52847, assign31060_e52847_d_n3, assign31060_e52847_d_n4, assign31060_e52847_d_n6,) = {
    if (locals.var_guard469 != 0.0) {
        let assign31060_e52843: f64 = (locals.var_qesj1 + locals.var_qesj2);
        let assign31060_e52845: f64 = (assign31060_e52843 + locals.var_qesj3);
        (assign31060_e52845, ((locals.var_qesj1_dn3 + locals.var_qesj2_dn3) + locals.var_qesj3_dn3), ((locals.var_qesj1_dn4 + locals.var_qesj2_dn4) + locals.var_qesj3_dn4), ((locals.var_qesj1_dn6 + locals.var_qesj2_dn6) + locals.var_qesj3_dn6),)
    } else {
        (locals.var_qesj, locals.var_qesj_dn3, locals.var_qesj_dn4, locals.var_qesj_dn6,)
    }
};
        locals.var_qesj = assign31060_e52847;
        locals.var_qesj_dn3 = assign31060_e52847_d_n3;
        locals.var_qesj_dn4 = assign31060_e52847_d_n4;
        locals.var_qesj_dn6 = assign31060_e52847_d_n6;
        locals.var_qesj_rv = 0.0;

        let assign31070_e52850: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign31070_e52850;
        locals.var_guard562_rv = 0.0;

        let (assign31080_e52858, assign31080_e52858_d_n3, assign31080_e52858_d_n4, assign31080_e52858_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) {
        let assign31080_e52856: f64 = (locals.var_ved_jct / locals.var_pbd_t);
        (assign31080_e52856, (locals.var_ved_jct_dn3 / locals.var_pbd_t), (-((locals.var_ved_jct * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), (locals.var_ved_jct_dn5 / locals.var_pbd_t),)
    } else {
        (locals.var_t1__blk554, locals.var_t1__blk554_dn3, locals.var_t1__blk554_dn4, locals.var_t1__blk554_dn5,)
    }
};
        locals.var_t1__blk554 = assign31080_e52858;
        locals.var_t1__blk554_dn3 = assign31080_e52858_d_n3;
        locals.var_t1__blk554_dn4 = assign31080_e52858_d_n4;
        locals.var_t1__blk554_dn5 = assign31080_e52858_d_n5;
        locals.var_t1__blk554_rv = 0.0;

        let assign31090_e52861: f64 = if locals.var_t1__blk554 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign31090_e52861;
        locals.var_guard563_rv = 0.0;

        let assign31100_e52864: f64 = if p.p1603 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard564 = assign31100_e52864;
        locals.var_guard564_rv = 0.0;

        let assign31110_e52867: f64 = if locals.var_ved_jct > locals.var_vec1d { 1.0 } else { 0.0 };
        locals.var_guard565 = assign31110_e52867;
        locals.var_guard565_rv = 0.0;

        let (assign31120_e52881, assign31120_e52881_d_n3, assign31120_e52881_d_n4, assign31120_e52881_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) {
        let assign31120_e52879: f64 = (1.0 - locals.var_t1__blk554);
        (assign31120_e52879, (-locals.var_t1__blk554_dn3), (-locals.var_t1__blk554_dn4), (-locals.var_t1__blk554_dn5),)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31120_e52881;
        locals.var_arg__blk559_dn3 = assign31120_e52881_d_n3;
        locals.var_arg__blk559_dn4 = assign31120_e52881_d_n4;
        locals.var_arg__blk559_dn5 = assign31120_e52881_d_n5;
        locals.var_arg__blk559_rv = 0.0;

        let assign31130_e52884: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign31130_e52884;
        locals.var_guard566_rv = 0.0;

        let assign31140_e52887: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign31140_e52887;
        locals.var_guard567_rv = 0.0;

        let (assign31150_e52906, assign31150_e52906_d_n3, assign31150_e52906_d_n4, assign31150_e52906_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) {
        let assign31150_e52903: f64 = (locals.var_arg__blk559).sqrt();
        let assign31150_e52904: f64 = (1.0 / assign31150_e52903);
        (assign31150_e52904, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31150_e52903)) / (assign31150_e52903 * assign31150_e52903))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31150_e52903)) / (assign31150_e52903 * assign31150_e52903))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31150_e52903)) / (assign31150_e52903 * assign31150_e52903))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31150_e52906;
        locals.var_sarg__blk560_dn3 = assign31150_e52906_d_n3;
        locals.var_sarg__blk560_dn4 = assign31150_e52906_d_n4;
        locals.var_sarg__blk560_dn5 = assign31150_e52906_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31160_e52926, assign31160_e52926_d_n3, assign31160_e52926_d_n4, assign31160_e52926_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 == 0.0)) {
        let assign31160_e52923: f64 = (-p.p1597);
        let assign31160_e52924: f64 = (locals.var_arg__blk559).powf(assign31160_e52923);
        (assign31160_e52924, if 0.0 == 0.0 && ((assign31160_e52923) as f64).is_finite() && ((assign31160_e52923) as f64).fract() == 0.0 { if assign31160_e52923 == 0.0 { 0.0 } else { (assign31160_e52923 * ((locals.var_arg__blk559).powf(assign31160_e52923 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31160_e52924 * (assign31160_e52923 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31160_e52923) as f64).is_finite() && ((assign31160_e52923) as f64).fract() == 0.0 { if assign31160_e52923 == 0.0 { 0.0 } else { (assign31160_e52923 * ((locals.var_arg__blk559).powf(assign31160_e52923 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31160_e52924 * (assign31160_e52923 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31160_e52923) as f64).is_finite() && ((assign31160_e52923) as f64).fract() == 0.0 { if assign31160_e52923 == 0.0 { 0.0 } else { (assign31160_e52923 * ((locals.var_arg__blk559).powf(assign31160_e52923 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31160_e52924 * (assign31160_e52923 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31160_e52926;
        locals.var_sarg__blk560_dn3 = assign31160_e52926_d_n3;
        locals.var_sarg__blk560_dn4 = assign31160_e52926_d_n4;
        locals.var_sarg__blk560_dn5 = assign31160_e52926_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31170_e52952, assign31170_e52952_d_n3, assign31170_e52952_d_n4, assign31170_e52952_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign31170_e52940: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31170_e52944: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31170_e52945: f64 = (1.0 - assign31170_e52944);
        let assign31170_e52946: f64 = (assign31170_e52940 * assign31170_e52945);
        let assign31170_e52949: f64 = (1.0 - p.p1597);
        let assign31170_e52950: f64 = (assign31170_e52946 / assign31170_e52949);
        (assign31170_e52950, ((assign31170_e52940 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31170_e52949), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31170_e52945) + (assign31170_e52940 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31170_e52949), ((assign31170_e52940 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31170_e52949),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31170_e52952;
        locals.var_qedj1_dn3 = assign31170_e52952_d_n3;
        locals.var_qedj1_dn4 = assign31170_e52952_d_n4;
        locals.var_qedj1_dn5 = assign31170_e52952_d_n5;
        locals.var_qedj1_rv = 0.0;

        let (assign31180_e52985, assign31180_e52985_d_n3, assign31180_e52985_d_n4, assign31180_e52985_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 != 0.0)) && (locals.var_guard566 == 0.0)) {
        let assign31180_e52966: f64 = (-locals.var_pbd_t);
        let assign31180_e52968: f64 = (assign31180_e52966 * locals.var_czbd);
        let (assign31180_e52982, assign31180_e52982_d_n3, assign31180_e52982_d_n4, assign31180_e52982_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31180_e52974: f64 = (-87.498233534);
                (assign31180_e52974, 0.0, 0.0, 0.0,)
            } else {
                let (assign31180_e52981, assign31180_e52981_d_n3, assign31180_e52981_d_n4, assign31180_e52981_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31180_e52979: f64 = (locals.var_arg__blk559).ln();
                        (assign31180_e52979, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31180_e52981, assign31180_e52981_d_n3, assign31180_e52981_d_n4, assign31180_e52981_d_n5,)
            }
        };
        let assign31180_e52983: f64 = (assign31180_e52968 * assign31180_e52982);
        (assign31180_e52983, (assign31180_e52968 * assign31180_e52982_d_n3), (((((-locals.var_pbd_t_dn4) * locals.var_czbd) + (assign31180_e52966 * locals.var_czbd_dn4)) * assign31180_e52982) + (assign31180_e52968 * assign31180_e52982_d_n4)), (assign31180_e52968 * assign31180_e52982_d_n5),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31180_e52985;
        locals.var_qedj1_dn3 = assign31180_e52985_d_n3;
        locals.var_qedj1_dn4 = assign31180_e52985_d_n4;
        locals.var_qedj1_dn5 = assign31180_e52985_d_n5;
        locals.var_qedj1_rv = 0.0;

        let (assign31190_e53002, assign31190_e53002_d_n3, assign31190_e53002_d_n4, assign31190_e53002_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) {
        let assign31190_e52999: f64 = (locals.var_vec1d / locals.var_pbd_t);
        let assign31190_e53000: f64 = (1.0 - assign31190_e52999);
        (assign31190_e53000, 0.0, (-(((locals.var_vec1d_dn4 * locals.var_pbd_t) - (locals.var_vec1d * locals.var_pbd_t_dn4)) / (locals.var_pbd_t * locals.var_pbd_t))), 0.0,)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31190_e53002;
        locals.var_arg__blk559_dn3 = assign31190_e53002_d_n3;
        locals.var_arg__blk559_dn4 = assign31190_e53002_d_n4;
        locals.var_arg__blk559_dn5 = assign31190_e53002_d_n5;
        locals.var_arg__blk559_rv = 0.0;

        let assign31200_e53005: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign31200_e53005;
        locals.var_guard568_rv = 0.0;

        let assign31210_e53008: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign31210_e53008;
        locals.var_guard569_rv = 0.0;

        let (assign31220_e53028, assign31220_e53028_d_n3, assign31220_e53028_d_n4, assign31220_e53028_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 != 0.0)) {
        let assign31220_e53025: f64 = (locals.var_arg__blk559).sqrt();
        let assign31220_e53026: f64 = (1.0 / assign31220_e53025);
        (assign31220_e53026, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31220_e53025)) / (assign31220_e53025 * assign31220_e53025))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31220_e53025)) / (assign31220_e53025 * assign31220_e53025))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31220_e53025)) / (assign31220_e53025 * assign31220_e53025))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31220_e53028;
        locals.var_sarg__blk560_dn3 = assign31220_e53028_d_n3;
        locals.var_sarg__blk560_dn4 = assign31220_e53028_d_n4;
        locals.var_sarg__blk560_dn5 = assign31220_e53028_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31230_e53049, assign31230_e53049_d_n3, assign31230_e53049_d_n4, assign31230_e53049_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 != 0.0)) && (locals.var_guard569 == 0.0)) {
        let assign31230_e53046: f64 = (-p.p1597);
        let assign31230_e53047: f64 = (locals.var_arg__blk559).powf(assign31230_e53046);
        (assign31230_e53047, if 0.0 == 0.0 && ((assign31230_e53046) as f64).is_finite() && ((assign31230_e53046) as f64).fract() == 0.0 { if assign31230_e53046 == 0.0 { 0.0 } else { (assign31230_e53046 * ((locals.var_arg__blk559).powf(assign31230_e53046 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31230_e53047 * (assign31230_e53046 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31230_e53046) as f64).is_finite() && ((assign31230_e53046) as f64).fract() == 0.0 { if assign31230_e53046 == 0.0 { 0.0 } else { (assign31230_e53046 * ((locals.var_arg__blk559).powf(assign31230_e53046 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31230_e53047 * (assign31230_e53046 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31230_e53046) as f64).is_finite() && ((assign31230_e53046) as f64).fract() == 0.0 { if assign31230_e53046 == 0.0 { 0.0 } else { (assign31230_e53046 * ((locals.var_arg__blk559).powf(assign31230_e53046 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31230_e53047 * (assign31230_e53046 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31230_e53049;
        locals.var_sarg__blk560_dn3 = assign31230_e53049_d_n3;
        locals.var_sarg__blk560_dn4 = assign31230_e53049_d_n4;
        locals.var_sarg__blk560_dn5 = assign31230_e53049_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31240_e53076, assign31240_e53076_d_n3, assign31240_e53076_d_n4, assign31240_e53076_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign31240_e53064: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31240_e53068: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31240_e53069: f64 = (1.0 - assign31240_e53068);
        let assign31240_e53070: f64 = (assign31240_e53064 * assign31240_e53069);
        let assign31240_e53073: f64 = (1.0 - p.p1597);
        let assign31240_e53074: f64 = (assign31240_e53070 / assign31240_e53073);
        (assign31240_e53074, ((assign31240_e53064 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31240_e53073), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31240_e53069) + (assign31240_e53064 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31240_e53073), ((assign31240_e53064 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31240_e53073),)
    } else {
        (locals.var_qec__blk561, locals.var_qec__blk561_dn3, locals.var_qec__blk561_dn4, locals.var_qec__blk561_dn5,)
    }
};
        locals.var_qec__blk561 = assign31240_e53076;
        locals.var_qec__blk561_dn3 = assign31240_e53076_d_n3;
        locals.var_qec__blk561_dn4 = assign31240_e53076_d_n4;
        locals.var_qec__blk561_dn5 = assign31240_e53076_d_n5;
        locals.var_qec__blk561_rv = 0.0;

        let (assign31250_e53110, assign31250_e53110_d_n3, assign31250_e53110_d_n4, assign31250_e53110_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard568 == 0.0)) {
        let assign31250_e53091: f64 = (-locals.var_pbd_t);
        let assign31250_e53093: f64 = (assign31250_e53091 * locals.var_czbd);
        let (assign31250_e53107, assign31250_e53107_d_n3, assign31250_e53107_d_n4, assign31250_e53107_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31250_e53099: f64 = (-87.498233534);
                (assign31250_e53099, 0.0, 0.0, 0.0,)
            } else {
                let (assign31250_e53106, assign31250_e53106_d_n3, assign31250_e53106_d_n4, assign31250_e53106_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31250_e53104: f64 = (locals.var_arg__blk559).ln();
                        (assign31250_e53104, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31250_e53106, assign31250_e53106_d_n3, assign31250_e53106_d_n4, assign31250_e53106_d_n5,)
            }
        };
        let assign31250_e53108: f64 = (assign31250_e53093 * assign31250_e53107);
        (assign31250_e53108, (assign31250_e53093 * assign31250_e53107_d_n3), (((((-locals.var_pbd_t_dn4) * locals.var_czbd) + (assign31250_e53091 * locals.var_czbd_dn4)) * assign31250_e53107) + (assign31250_e53093 * assign31250_e53107_d_n4)), (assign31250_e53093 * assign31250_e53107_d_n5),)
    } else {
        (locals.var_qec__blk561, locals.var_qec__blk561_dn3, locals.var_qec__blk561_dn4, locals.var_qec__blk561_dn5,)
    }
};
        locals.var_qec__blk561 = assign31250_e53110;
        locals.var_qec__blk561_dn3 = assign31250_e53110_d_n3;
        locals.var_qec__blk561_dn4 = assign31250_e53110_d_n4;
        locals.var_qec__blk561_dn5 = assign31250_e53110_d_n5;
        locals.var_qec__blk561_rv = 0.0;

        let (assign31260_e53129, assign31260_e53129_d_n3, assign31260_e53129_d_n4, assign31260_e53129_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) {
        let assign31260_e53124: f64 = (locals.var_ved_jct - locals.var_vec1d);
        let assign31260_e53126: f64 = (assign31260_e53124 / locals.var_pb21d);
        let assign31260_e53127: f64 = (1.0 - assign31260_e53126);
        (assign31260_e53127, (-(locals.var_ved_jct_dn3 / locals.var_pb21d)), (-((((-locals.var_vec1d_dn4) * locals.var_pb21d) - (assign31260_e53124 * locals.var_pb21d_dn4)) / (locals.var_pb21d * locals.var_pb21d))), (-(locals.var_ved_jct_dn5 / locals.var_pb21d)),)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31260_e53129;
        locals.var_arg__blk559_dn3 = assign31260_e53129_d_n3;
        locals.var_arg__blk559_dn4 = assign31260_e53129_d_n4;
        locals.var_arg__blk559_dn5 = assign31260_e53129_d_n5;
        locals.var_arg__blk559_rv = 0.0;

        let assign31270_e53132: f64 = if p.p1609 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign31270_e53132;
        locals.var_guard570_rv = 0.0;

        let assign31280_e53135: f64 = if p.p1609 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign31280_e53135;
        locals.var_guard571_rv = 0.0;

        let (assign31290_e53155, assign31290_e53155_d_n3, assign31290_e53155_d_n4, assign31290_e53155_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 != 0.0)) {
        let assign31290_e53152: f64 = (locals.var_arg__blk559).sqrt();
        let assign31290_e53153: f64 = (1.0 / assign31290_e53152);
        (assign31290_e53153, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31290_e53152)) / (assign31290_e53152 * assign31290_e53152))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31290_e53152)) / (assign31290_e53152 * assign31290_e53152))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31290_e53152)) / (assign31290_e53152 * assign31290_e53152))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31290_e53155;
        locals.var_sarg__blk560_dn3 = assign31290_e53155_d_n3;
        locals.var_sarg__blk560_dn4 = assign31290_e53155_d_n4;
        locals.var_sarg__blk560_dn5 = assign31290_e53155_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31300_e53176, assign31300_e53176_d_n3, assign31300_e53176_d_n4, assign31300_e53176_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 != 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign31300_e53173: f64 = (-p.p1609);
        let assign31300_e53174: f64 = (locals.var_arg__blk559).powf(assign31300_e53173);
        (assign31300_e53174, if 0.0 == 0.0 && ((assign31300_e53173) as f64).is_finite() && ((assign31300_e53173) as f64).fract() == 0.0 { if assign31300_e53173 == 0.0 { 0.0 } else { (assign31300_e53173 * ((locals.var_arg__blk559).powf(assign31300_e53173 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31300_e53174 * (assign31300_e53173 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31300_e53173) as f64).is_finite() && ((assign31300_e53173) as f64).fract() == 0.0 { if assign31300_e53173 == 0.0 { 0.0 } else { (assign31300_e53173 * ((locals.var_arg__blk559).powf(assign31300_e53173 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31300_e53174 * (assign31300_e53173 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31300_e53173) as f64).is_finite() && ((assign31300_e53173) as f64).fract() == 0.0 { if assign31300_e53173 == 0.0 { 0.0 } else { (assign31300_e53173 * ((locals.var_arg__blk559).powf(assign31300_e53173 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31300_e53174 * (assign31300_e53173 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31300_e53176;
        locals.var_sarg__blk560_dn3 = assign31300_e53176_d_n3;
        locals.var_sarg__blk560_dn4 = assign31300_e53176_d_n4;
        locals.var_sarg__blk560_dn5 = assign31300_e53176_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31310_e53207, assign31310_e53207_d_n3, assign31310_e53207_d_n4, assign31310_e53207_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 != 0.0)) {
        let assign31310_e53192: f64 = (p.p1603 * locals.var_pb21d);
        let assign31310_e53194: f64 = (assign31310_e53192 * locals.var_czbd);
        let assign31310_e53198: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31310_e53199: f64 = (1.0 - assign31310_e53198);
        let assign31310_e53200: f64 = (assign31310_e53194 * assign31310_e53199);
        let assign31310_e53203: f64 = (1.0 - p.p1609);
        let assign31310_e53204: f64 = (assign31310_e53200 / assign31310_e53203);
        let assign31310_e53205: f64 = (locals.var_qec__blk561 + assign31310_e53204);
        (assign31310_e53205, (locals.var_qec__blk561_dn3 + ((assign31310_e53194 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31310_e53203)), (locals.var_qec__blk561_dn4 + ((((((p.p1603 * locals.var_pb21d_dn4) * locals.var_czbd) + (assign31310_e53192 * locals.var_czbd_dn4)) * assign31310_e53199) + (assign31310_e53194 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31310_e53203)), (locals.var_qec__blk561_dn5 + ((assign31310_e53194 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31310_e53203)),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31310_e53207;
        locals.var_qedj1_dn3 = assign31310_e53207_d_n3;
        locals.var_qedj1_dn4 = assign31310_e53207_d_n4;
        locals.var_qedj1_dn5 = assign31310_e53207_d_n5;
        locals.var_qedj1_rv = 0.0;

        let (assign31320_e53244, assign31320_e53244_d_n3, assign31320_e53244_d_n4, assign31320_e53244_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 != 0.0)) && (locals.var_guard565 == 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign31320_e53224: f64 = (p.p1603 * locals.var_pb21d);
        let assign31320_e53226: f64 = (assign31320_e53224 * locals.var_czbd);
        let (assign31320_e53240, assign31320_e53240_d_n3, assign31320_e53240_d_n4, assign31320_e53240_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31320_e53232: f64 = (-87.498233534);
                (assign31320_e53232, 0.0, 0.0, 0.0,)
            } else {
                let (assign31320_e53239, assign31320_e53239_d_n3, assign31320_e53239_d_n4, assign31320_e53239_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31320_e53237: f64 = (locals.var_arg__blk559).ln();
                        (assign31320_e53237, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31320_e53239, assign31320_e53239_d_n3, assign31320_e53239_d_n4, assign31320_e53239_d_n5,)
            }
        };
        let assign31320_e53241: f64 = (assign31320_e53226 * assign31320_e53240);
        let assign31320_e53242: f64 = (locals.var_qec__blk561 - assign31320_e53241);
        (assign31320_e53242, (locals.var_qec__blk561_dn3 - (assign31320_e53226 * assign31320_e53240_d_n3)), (locals.var_qec__blk561_dn4 - (((((p.p1603 * locals.var_pb21d_dn4) * locals.var_czbd) + (assign31320_e53224 * locals.var_czbd_dn4)) * assign31320_e53240) + (assign31320_e53226 * assign31320_e53240_d_n4))), (locals.var_qec__blk561_dn5 - (assign31320_e53226 * assign31320_e53240_d_n5)),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31320_e53244;
        locals.var_qedj1_dn3 = assign31320_e53244_d_n3;
        locals.var_qedj1_dn4 = assign31320_e53244_d_n4;
        locals.var_qedj1_dn5 = assign31320_e53244_d_n5;
        locals.var_qedj1_rv = 0.0;

        let (assign31330_e53257, assign31330_e53257_d_n3, assign31330_e53257_d_n4, assign31330_e53257_d_n5,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign31330_e53255: f64 = (1.0 - locals.var_t1__blk554);
        (assign31330_e53255, (-locals.var_t1__blk554_dn3), (-locals.var_t1__blk554_dn4), (-locals.var_t1__blk554_dn5),)
    } else {
        (locals.var_arg__blk559, locals.var_arg__blk559_dn3, locals.var_arg__blk559_dn4, locals.var_arg__blk559_dn5,)
    }
};
        locals.var_arg__blk559 = assign31330_e53257;
        locals.var_arg__blk559_dn3 = assign31330_e53257_d_n3;
        locals.var_arg__blk559_dn4 = assign31330_e53257_d_n4;
        locals.var_arg__blk559_dn5 = assign31330_e53257_d_n5;
        locals.var_arg__blk559_rv = 0.0;

        let assign31340_e53260: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign31340_e53260;
        locals.var_guard572_rv = 0.0;

        let assign31350_e53263: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign31350_e53263;
        locals.var_guard573_rv = 0.0;

        let (assign31360_e53281, assign31360_e53281_d_n3, assign31360_e53281_d_n4, assign31360_e53281_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 != 0.0)) {
        let assign31360_e53278: f64 = (locals.var_arg__blk559).sqrt();
        let assign31360_e53279: f64 = (1.0 / assign31360_e53278);
        (assign31360_e53279, (-((locals.var_arg__blk559_dn3 / (2.0 * assign31360_e53278)) / (assign31360_e53278 * assign31360_e53278))), (-((locals.var_arg__blk559_dn4 / (2.0 * assign31360_e53278)) / (assign31360_e53278 * assign31360_e53278))), (-((locals.var_arg__blk559_dn5 / (2.0 * assign31360_e53278)) / (assign31360_e53278 * assign31360_e53278))),)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31360_e53281;
        locals.var_sarg__blk560_dn3 = assign31360_e53281_d_n3;
        locals.var_sarg__blk560_dn4 = assign31360_e53281_d_n4;
        locals.var_sarg__blk560_dn5 = assign31360_e53281_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31370_e53300, assign31370_e53300_d_n3, assign31370_e53300_d_n4, assign31370_e53300_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 == 0.0)) {
        let assign31370_e53297: f64 = (-p.p1597);
        let assign31370_e53298: f64 = (locals.var_arg__blk559).powf(assign31370_e53297);
        (assign31370_e53298, if 0.0 == 0.0 && ((assign31370_e53297) as f64).is_finite() && ((assign31370_e53297) as f64).fract() == 0.0 { if assign31370_e53297 == 0.0 { 0.0 } else { (assign31370_e53297 * ((locals.var_arg__blk559).powf(assign31370_e53297 - 1.0) * locals.var_arg__blk559_dn3)) } } else { (assign31370_e53298 * (assign31370_e53297 * (locals.var_arg__blk559_dn3 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31370_e53297) as f64).is_finite() && ((assign31370_e53297) as f64).fract() == 0.0 { if assign31370_e53297 == 0.0 { 0.0 } else { (assign31370_e53297 * ((locals.var_arg__blk559).powf(assign31370_e53297 - 1.0) * locals.var_arg__blk559_dn4)) } } else { (assign31370_e53298 * (assign31370_e53297 * (locals.var_arg__blk559_dn4 / locals.var_arg__blk559))) }, if 0.0 == 0.0 && ((assign31370_e53297) as f64).is_finite() && ((assign31370_e53297) as f64).fract() == 0.0 { if assign31370_e53297 == 0.0 { 0.0 } else { (assign31370_e53297 * ((locals.var_arg__blk559).powf(assign31370_e53297 - 1.0) * locals.var_arg__blk559_dn5)) } } else { (assign31370_e53298 * (assign31370_e53297 * (locals.var_arg__blk559_dn5 / locals.var_arg__blk559))) },)
    } else {
        (locals.var_sarg__blk560, locals.var_sarg__blk560_dn3, locals.var_sarg__blk560_dn4, locals.var_sarg__blk560_dn5,)
    }
};
        locals.var_sarg__blk560 = assign31370_e53300;
        locals.var_sarg__blk560_dn3 = assign31370_e53300_d_n3;
        locals.var_sarg__blk560_dn4 = assign31370_e53300_d_n4;
        locals.var_sarg__blk560_dn5 = assign31370_e53300_d_n5;
        locals.var_sarg__blk560_rv = 0.0;

        let (assign31380_e53325, assign31380_e53325_d_n3, assign31380_e53325_d_n4, assign31380_e53325_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 != 0.0)) {
        let assign31380_e53313: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31380_e53317: f64 = (locals.var_arg__blk559 * locals.var_sarg__blk560);
        let assign31380_e53318: f64 = (1.0 - assign31380_e53317);
        let assign31380_e53319: f64 = (assign31380_e53313 * assign31380_e53318);
        let assign31380_e53322: f64 = (1.0 - p.p1597);
        let assign31380_e53323: f64 = (assign31380_e53319 / assign31380_e53322);
        (assign31380_e53323, ((assign31380_e53313 * (-((locals.var_arg__blk559_dn3 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn3)))) / assign31380_e53322), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31380_e53318) + (assign31380_e53313 * (-((locals.var_arg__blk559_dn4 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn4))))) / assign31380_e53322), ((assign31380_e53313 * (-((locals.var_arg__blk559_dn5 * locals.var_sarg__blk560) + (locals.var_arg__blk559 * locals.var_sarg__blk560_dn5)))) / assign31380_e53322),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31380_e53325;
        locals.var_qedj1_dn3 = assign31380_e53325_d_n3;
        locals.var_qedj1_dn4 = assign31380_e53325_d_n4;
        locals.var_qedj1_dn5 = assign31380_e53325_d_n5;
        locals.var_qedj1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_124(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31390_e53357, assign31390_e53357_d_n3, assign31390_e53357_d_n4, assign31390_e53357_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 != 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard572 == 0.0)) {
        let assign31390_e53338: f64 = (-locals.var_pbd_t);
        let assign31390_e53340: f64 = (assign31390_e53338 * locals.var_czbd);
        let (assign31390_e53354, assign31390_e53354_d_n3, assign31390_e53354_d_n4, assign31390_e53354_d_n5,) = {
            if (!(locals.var_arg__blk559 > 1e-38)) {
                let assign31390_e53346: f64 = (-87.498233534);
                (assign31390_e53346, 0.0, 0.0, 0.0,)
            } else {
                let (assign31390_e53353, assign31390_e53353_d_n3, assign31390_e53353_d_n4, assign31390_e53353_d_n5,) = {
                    if (locals.var_arg__blk559 > 1e-38) {
                        let assign31390_e53351: f64 = (locals.var_arg__blk559).ln();
                        (assign31390_e53351, (locals.var_arg__blk559_dn3 / locals.var_arg__blk559), (locals.var_arg__blk559_dn4 / locals.var_arg__blk559), (locals.var_arg__blk559_dn5 / locals.var_arg__blk559),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31390_e53353, assign31390_e53353_d_n3, assign31390_e53353_d_n4, assign31390_e53353_d_n5,)
            }
        };
        let assign31390_e53355: f64 = (assign31390_e53340 * assign31390_e53354);
        (assign31390_e53355, (assign31390_e53340 * assign31390_e53354_d_n3), (((((-locals.var_pbd_t_dn4) * locals.var_czbd) + (assign31390_e53338 * locals.var_czbd_dn4)) * assign31390_e53354) + (assign31390_e53340 * assign31390_e53354_d_n4)), (assign31390_e53340 * assign31390_e53354_d_n5),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31390_e53357;
        locals.var_qedj1_dn3 = assign31390_e53357_d_n3;
        locals.var_qedj1_dn4 = assign31390_e53357_d_n4;
        locals.var_qedj1_dn5 = assign31390_e53357_d_n5;
        locals.var_qedj1_rv = 0.0;

        let assign31400_e53360: f64 = if p.p1597 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign31400_e53360;
        locals.var_guard574_rv = 0.0;

        let assign31410_e53363: f64 = if p.p1597 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign31410_e53363;
        locals.var_guard575_rv = 0.0;

        let (assign31420_e53379,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 != 0.0)) {
        let assign31420_e53376: f64 = (0.1_f64).sqrt();
        let assign31420_e53377: f64 = (1.0 / assign31420_e53376);
        (assign31420_e53377,)
    } else {
        (locals.var_t2__blk555,)
    }
};
        locals.var_t2__blk555 = assign31420_e53379;
        locals.var_t2__blk555_rv = 0.0;

        let (assign31430_e53396,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 == 0.0)) {
        let assign31430_e53393: f64 = (-p.p1597);
        let assign31430_e53394: f64 = (0.1_f64).powf(assign31430_e53393);
        (assign31430_e53394,)
    } else {
        (locals.var_t2__blk555,)
    }
};
        locals.var_t2__blk555 = assign31430_e53396;
        locals.var_t2__blk555_rv = 0.0;

        let (assign31440_e53411,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) {
        let assign31440_e53408: f64 = (1.0 - p.p1597);
        let assign31440_e53409: f64 = (1.0 / assign31440_e53408);
        (assign31440_e53409,)
    } else {
        (locals.var_t3__blk556,)
    }
};
        locals.var_t3__blk556 = assign31440_e53411;
        locals.var_t3__blk556_rv = 0.0;

        let (assign31450_e53434,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 != 0.0)) {
        let assign31450_e53424: f64 = (0.05 * p.p1597);
        let assign31450_e53427: f64 = (1.0 + p.p1597);
        let assign31450_e53428: f64 = (assign31450_e53424 * assign31450_e53427);
        let assign31450_e53430: f64 = (assign31450_e53428 * locals.var_t2__blk555);
        let assign31450_e53431: f64 = (1.0 - assign31450_e53430);
        let assign31450_e53432: f64 = (locals.var_t3__blk556 * assign31450_e53431);
        (assign31450_e53432,)
    } else {
        (locals.var_t5__blk558,)
    }
};
        locals.var_t5__blk558 = assign31450_e53434;
        locals.var_t5__blk558_rv = 0.0;

        let (assign31460_e53446,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk555,)
    }
};
        locals.var_t2__blk555 = assign31460_e53446;
        locals.var_t2__blk555_rv = 0.0;

        let (assign31470_e53461,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard574 == 0.0)) {
        let assign31470_e53458: f64 = (0.1_f64).ln();
        let assign31470_e53459: f64 = (1.5 - assign31470_e53458);
        (assign31470_e53459,)
    } else {
        (locals.var_t5__blk558,)
    }
};
        locals.var_t5__blk558 = assign31470_e53461;
        locals.var_t5__blk558_rv = 0.0;

        let (assign31480_e53486, assign31480_e53486_d_n3, assign31480_e53486_d_n4, assign31480_e53486_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let assign31480_e53471: f64 = (locals.var_t1__blk554 - 1.0);
        let assign31480_e53472: f64 = (locals.var_t2__blk555 * assign31480_e53471);
        let assign31480_e53475: f64 = (5.0 * p.p1597);
        let assign31480_e53478: f64 = (locals.var_t1__blk554 - 1.0);
        let assign31480_e53479: f64 = (assign31480_e53475 * assign31480_e53478);
        let assign31480_e53482: f64 = (1.0 + p.p1597);
        let assign31480_e53483: f64 = (assign31480_e53479 + assign31480_e53482);
        let assign31480_e53484: f64 = (assign31480_e53472 * assign31480_e53483);
        (assign31480_e53484, (((locals.var_t2__blk555 * locals.var_t1__blk554_dn3) * assign31480_e53483) + (assign31480_e53472 * (assign31480_e53475 * locals.var_t1__blk554_dn3))), (((locals.var_t2__blk555 * locals.var_t1__blk554_dn4) * assign31480_e53483) + (assign31480_e53472 * (assign31480_e53475 * locals.var_t1__blk554_dn4))), (((locals.var_t2__blk555 * locals.var_t1__blk554_dn5) * assign31480_e53483) + (assign31480_e53472 * (assign31480_e53475 * locals.var_t1__blk554_dn5))),)
    } else {
        (locals.var_t4__blk557, locals.var_t4__blk557_dn3, locals.var_t4__blk557_dn4, locals.var_t4__blk557_dn5,)
    }
};
        locals.var_t4__blk557 = assign31480_e53486;
        locals.var_t4__blk557_dn3 = assign31480_e53486_d_n3;
        locals.var_t4__blk557_dn4 = assign31480_e53486_d_n4;
        locals.var_t4__blk557_dn5 = assign31480_e53486_d_n5;
        locals.var_t4__blk557_rv = 0.0;

        let (assign31490_e53501, assign31490_e53501_d_n3, assign31490_e53501_d_n4, assign31490_e53501_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard562 != 0.0)) && (locals.var_guard563 == 0.0)) {
        let assign31490_e53495: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign31490_e53498: f64 = (locals.var_t4__blk557 + locals.var_t5__blk558);
        let assign31490_e53499: f64 = (assign31490_e53495 * assign31490_e53498);
        (assign31490_e53499, (assign31490_e53495 * locals.var_t4__blk557_dn3), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign31490_e53498) + (assign31490_e53495 * locals.var_t4__blk557_dn4)), (assign31490_e53495 * locals.var_t4__blk557_dn5),)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31490_e53501;
        locals.var_qedj1_dn3 = assign31490_e53501_d_n3;
        locals.var_qedj1_dn4 = assign31490_e53501_d_n4;
        locals.var_qedj1_dn5 = assign31490_e53501_d_n5;
        locals.var_qedj1_rv = 0.0;

        let (assign31500_e53508, assign31500_e53508_d_n3, assign31500_e53508_d_n4, assign31500_e53508_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard562 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qedj1, locals.var_qedj1_dn3, locals.var_qedj1_dn4, locals.var_qedj1_dn5,)
    }
};
        locals.var_qedj1 = assign31500_e53508;
        locals.var_qedj1_dn3 = assign31500_e53508_d_n3;
        locals.var_qedj1_dn4 = assign31500_e53508_d_n4;
        locals.var_qedj1_dn5 = assign31500_e53508_d_n5;
        locals.var_qedj1_rv = 0.0;

        let assign31510_e53511: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign31510_e53511;
        locals.var_guard584_rv = 0.0;

        let (assign31520_e53519, assign31520_e53519_d_n3, assign31520_e53519_d_n4, assign31520_e53519_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign31520_e53517: f64 = (locals.var_ved_jct / locals.var_pbswd_t);
        (assign31520_e53517, (locals.var_ved_jct_dn3 / locals.var_pbswd_t), (-((locals.var_ved_jct * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), (locals.var_ved_jct_dn5 / locals.var_pbswd_t),)
    } else {
        (locals.var_t1__blk576, locals.var_t1__blk576_dn3, locals.var_t1__blk576_dn4, locals.var_t1__blk576_dn5,)
    }
};
        locals.var_t1__blk576 = assign31520_e53519;
        locals.var_t1__blk576_dn3 = assign31520_e53519_d_n3;
        locals.var_t1__blk576_dn4 = assign31520_e53519_d_n4;
        locals.var_t1__blk576_dn5 = assign31520_e53519_d_n5;
        locals.var_t1__blk576_rv = 0.0;

        let assign31530_e53522: f64 = if locals.var_t1__blk576 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign31530_e53522;
        locals.var_guard585_rv = 0.0;

        let assign31540_e53525: f64 = if p.p1605 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign31540_e53525;
        locals.var_guard586_rv = 0.0;

        let assign31550_e53528: f64 = if locals.var_ved_jct > locals.var_vec2d { 1.0 } else { 0.0 };
        locals.var_guard587 = assign31550_e53528;
        locals.var_guard587_rv = 0.0;

        let (assign31560_e53542, assign31560_e53542_d_n3, assign31560_e53542_d_n4, assign31560_e53542_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign31560_e53540: f64 = (1.0 - locals.var_t1__blk576);
        (assign31560_e53540, (-locals.var_t1__blk576_dn3), (-locals.var_t1__blk576_dn4), (-locals.var_t1__blk576_dn5),)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31560_e53542;
        locals.var_arg__blk581_dn3 = assign31560_e53542_d_n3;
        locals.var_arg__blk581_dn4 = assign31560_e53542_d_n4;
        locals.var_arg__blk581_dn5 = assign31560_e53542_d_n5;
        locals.var_arg__blk581_rv = 0.0;

        let assign31570_e53545: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign31570_e53545;
        locals.var_guard588_rv = 0.0;

        let assign31580_e53548: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign31580_e53548;
        locals.var_guard589_rv = 0.0;

        let (assign31590_e53567, assign31590_e53567_d_n3, assign31590_e53567_d_n4, assign31590_e53567_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) && (locals.var_guard589 != 0.0)) {
        let assign31590_e53564: f64 = (locals.var_arg__blk581).sqrt();
        let assign31590_e53565: f64 = (1.0 / assign31590_e53564);
        (assign31590_e53565, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31590_e53564)) / (assign31590_e53564 * assign31590_e53564))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31590_e53564)) / (assign31590_e53564 * assign31590_e53564))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31590_e53564)) / (assign31590_e53564 * assign31590_e53564))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31590_e53567;
        locals.var_sarg__blk582_dn3 = assign31590_e53567_d_n3;
        locals.var_sarg__blk582_dn4 = assign31590_e53567_d_n4;
        locals.var_sarg__blk582_dn5 = assign31590_e53567_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31600_e53587, assign31600_e53587_d_n3, assign31600_e53587_d_n4, assign31600_e53587_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) && (locals.var_guard589 == 0.0)) {
        let assign31600_e53584: f64 = (-p.p1599);
        let assign31600_e53585: f64 = (locals.var_arg__blk581).powf(assign31600_e53584);
        (assign31600_e53585, if 0.0 == 0.0 && ((assign31600_e53584) as f64).is_finite() && ((assign31600_e53584) as f64).fract() == 0.0 { if assign31600_e53584 == 0.0 { 0.0 } else { (assign31600_e53584 * ((locals.var_arg__blk581).powf(assign31600_e53584 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31600_e53585 * (assign31600_e53584 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31600_e53584) as f64).is_finite() && ((assign31600_e53584) as f64).fract() == 0.0 { if assign31600_e53584 == 0.0 { 0.0 } else { (assign31600_e53584 * ((locals.var_arg__blk581).powf(assign31600_e53584 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31600_e53585 * (assign31600_e53584 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31600_e53584) as f64).is_finite() && ((assign31600_e53584) as f64).fract() == 0.0 { if assign31600_e53584 == 0.0 { 0.0 } else { (assign31600_e53584 * ((locals.var_arg__blk581).powf(assign31600_e53584 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31600_e53585 * (assign31600_e53584 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31600_e53587;
        locals.var_sarg__blk582_dn3 = assign31600_e53587_d_n3;
        locals.var_sarg__blk582_dn4 = assign31600_e53587_d_n4;
        locals.var_sarg__blk582_dn5 = assign31600_e53587_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31610_e53613, assign31610_e53613_d_n3, assign31610_e53613_d_n4, assign31610_e53613_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) {
        let assign31610_e53601: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31610_e53605: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31610_e53606: f64 = (1.0 - assign31610_e53605);
        let assign31610_e53607: f64 = (assign31610_e53601 * assign31610_e53606);
        let assign31610_e53610: f64 = (1.0 - p.p1599);
        let assign31610_e53611: f64 = (assign31610_e53607 / assign31610_e53610);
        (assign31610_e53611, ((assign31610_e53601 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31610_e53610), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31610_e53606) + (assign31610_e53601 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31610_e53610), ((assign31610_e53601 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31610_e53610),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31610_e53613;
        locals.var_qedj2_dn3 = assign31610_e53613_d_n3;
        locals.var_qedj2_dn4 = assign31610_e53613_d_n4;
        locals.var_qedj2_dn5 = assign31610_e53613_d_n5;
        locals.var_qedj2_rv = 0.0;

        let (assign31620_e53646, assign31620_e53646_d_n3, assign31620_e53646_d_n4, assign31620_e53646_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 == 0.0)) {
        let assign31620_e53627: f64 = (-locals.var_pbswd_t);
        let assign31620_e53629: f64 = (assign31620_e53627 * locals.var_czbdsw);
        let (assign31620_e53643, assign31620_e53643_d_n3, assign31620_e53643_d_n4, assign31620_e53643_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31620_e53635: f64 = (-87.498233534);
                (assign31620_e53635, 0.0, 0.0, 0.0,)
            } else {
                let (assign31620_e53642, assign31620_e53642_d_n3, assign31620_e53642_d_n4, assign31620_e53642_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31620_e53640: f64 = (locals.var_arg__blk581).ln();
                        (assign31620_e53640, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31620_e53642, assign31620_e53642_d_n3, assign31620_e53642_d_n4, assign31620_e53642_d_n5,)
            }
        };
        let assign31620_e53644: f64 = (assign31620_e53629 * assign31620_e53643);
        (assign31620_e53644, (assign31620_e53629 * assign31620_e53643_d_n3), (((((-locals.var_pbswd_t_dn4) * locals.var_czbdsw) + (assign31620_e53627 * locals.var_czbdsw_dn4)) * assign31620_e53643) + (assign31620_e53629 * assign31620_e53643_d_n4)), (assign31620_e53629 * assign31620_e53643_d_n5),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31620_e53646;
        locals.var_qedj2_dn3 = assign31620_e53646_d_n3;
        locals.var_qedj2_dn4 = assign31620_e53646_d_n4;
        locals.var_qedj2_dn5 = assign31620_e53646_d_n5;
        locals.var_qedj2_rv = 0.0;

        let (assign31630_e53663, assign31630_e53663_d_n3, assign31630_e53663_d_n4, assign31630_e53663_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign31630_e53660: f64 = (locals.var_vec2d / locals.var_pbswd_t);
        let assign31630_e53661: f64 = (1.0 - assign31630_e53660);
        (assign31630_e53661, 0.0, (-(((locals.var_vec2d_dn4 * locals.var_pbswd_t) - (locals.var_vec2d * locals.var_pbswd_t_dn4)) / (locals.var_pbswd_t * locals.var_pbswd_t))), 0.0,)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31630_e53663;
        locals.var_arg__blk581_dn3 = assign31630_e53663_d_n3;
        locals.var_arg__blk581_dn4 = assign31630_e53663_d_n4;
        locals.var_arg__blk581_dn5 = assign31630_e53663_d_n5;
        locals.var_arg__blk581_rv = 0.0;

        let assign31640_e53666: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign31640_e53666;
        locals.var_guard590_rv = 0.0;

        let assign31650_e53669: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign31650_e53669;
        locals.var_guard591_rv = 0.0;

        let (assign31660_e53689, assign31660_e53689_d_n3, assign31660_e53689_d_n4, assign31660_e53689_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign31660_e53686: f64 = (locals.var_arg__blk581).sqrt();
        let assign31660_e53687: f64 = (1.0 / assign31660_e53686);
        (assign31660_e53687, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31660_e53686)) / (assign31660_e53686 * assign31660_e53686))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31660_e53686)) / (assign31660_e53686 * assign31660_e53686))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31660_e53686)) / (assign31660_e53686 * assign31660_e53686))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31660_e53689;
        locals.var_sarg__blk582_dn3 = assign31660_e53689_d_n3;
        locals.var_sarg__blk582_dn4 = assign31660_e53689_d_n4;
        locals.var_sarg__blk582_dn5 = assign31660_e53689_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31670_e53710, assign31670_e53710_d_n3, assign31670_e53710_d_n4, assign31670_e53710_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard591 == 0.0)) {
        let assign31670_e53707: f64 = (-p.p1599);
        let assign31670_e53708: f64 = (locals.var_arg__blk581).powf(assign31670_e53707);
        (assign31670_e53708, if 0.0 == 0.0 && ((assign31670_e53707) as f64).is_finite() && ((assign31670_e53707) as f64).fract() == 0.0 { if assign31670_e53707 == 0.0 { 0.0 } else { (assign31670_e53707 * ((locals.var_arg__blk581).powf(assign31670_e53707 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31670_e53708 * (assign31670_e53707 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31670_e53707) as f64).is_finite() && ((assign31670_e53707) as f64).fract() == 0.0 { if assign31670_e53707 == 0.0 { 0.0 } else { (assign31670_e53707 * ((locals.var_arg__blk581).powf(assign31670_e53707 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31670_e53708 * (assign31670_e53707 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31670_e53707) as f64).is_finite() && ((assign31670_e53707) as f64).fract() == 0.0 { if assign31670_e53707 == 0.0 { 0.0 } else { (assign31670_e53707 * ((locals.var_arg__blk581).powf(assign31670_e53707 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31670_e53708 * (assign31670_e53707 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31670_e53710;
        locals.var_sarg__blk582_dn3 = assign31670_e53710_d_n3;
        locals.var_sarg__blk582_dn4 = assign31670_e53710_d_n4;
        locals.var_sarg__blk582_dn5 = assign31670_e53710_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31680_e53737, assign31680_e53737_d_n3, assign31680_e53737_d_n4, assign31680_e53737_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign31680_e53725: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31680_e53729: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31680_e53730: f64 = (1.0 - assign31680_e53729);
        let assign31680_e53731: f64 = (assign31680_e53725 * assign31680_e53730);
        let assign31680_e53734: f64 = (1.0 - p.p1599);
        let assign31680_e53735: f64 = (assign31680_e53731 / assign31680_e53734);
        (assign31680_e53735, ((assign31680_e53725 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31680_e53734), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31680_e53730) + (assign31680_e53725 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31680_e53734), ((assign31680_e53725 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31680_e53734),)
    } else {
        (locals.var_qec__blk583, locals.var_qec__blk583_dn3, locals.var_qec__blk583_dn4, locals.var_qec__blk583_dn5,)
    }
};
        locals.var_qec__blk583 = assign31680_e53737;
        locals.var_qec__blk583_dn3 = assign31680_e53737_d_n3;
        locals.var_qec__blk583_dn4 = assign31680_e53737_d_n4;
        locals.var_qec__blk583_dn5 = assign31680_e53737_d_n5;
        locals.var_qec__blk583_rv = 0.0;

        let (assign31690_e53771, assign31690_e53771_d_n3, assign31690_e53771_d_n4, assign31690_e53771_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard590 == 0.0)) {
        let assign31690_e53752: f64 = (-locals.var_pbswd_t);
        let assign31690_e53754: f64 = (assign31690_e53752 * locals.var_czbdsw);
        let (assign31690_e53768, assign31690_e53768_d_n3, assign31690_e53768_d_n4, assign31690_e53768_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31690_e53760: f64 = (-87.498233534);
                (assign31690_e53760, 0.0, 0.0, 0.0,)
            } else {
                let (assign31690_e53767, assign31690_e53767_d_n3, assign31690_e53767_d_n4, assign31690_e53767_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31690_e53765: f64 = (locals.var_arg__blk581).ln();
                        (assign31690_e53765, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31690_e53767, assign31690_e53767_d_n3, assign31690_e53767_d_n4, assign31690_e53767_d_n5,)
            }
        };
        let assign31690_e53769: f64 = (assign31690_e53754 * assign31690_e53768);
        (assign31690_e53769, (assign31690_e53754 * assign31690_e53768_d_n3), (((((-locals.var_pbswd_t_dn4) * locals.var_czbdsw) + (assign31690_e53752 * locals.var_czbdsw_dn4)) * assign31690_e53768) + (assign31690_e53754 * assign31690_e53768_d_n4)), (assign31690_e53754 * assign31690_e53768_d_n5),)
    } else {
        (locals.var_qec__blk583, locals.var_qec__blk583_dn3, locals.var_qec__blk583_dn4, locals.var_qec__blk583_dn5,)
    }
};
        locals.var_qec__blk583 = assign31690_e53771;
        locals.var_qec__blk583_dn3 = assign31690_e53771_d_n3;
        locals.var_qec__blk583_dn4 = assign31690_e53771_d_n4;
        locals.var_qec__blk583_dn5 = assign31690_e53771_d_n5;
        locals.var_qec__blk583_rv = 0.0;

        let (assign31700_e53790, assign31700_e53790_d_n3, assign31700_e53790_d_n4, assign31700_e53790_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign31700_e53785: f64 = (locals.var_ved_jct - locals.var_vec2d);
        let assign31700_e53787: f64 = (assign31700_e53785 / locals.var_pb22d);
        let assign31700_e53788: f64 = (1.0 - assign31700_e53787);
        (assign31700_e53788, (-(locals.var_ved_jct_dn3 / locals.var_pb22d)), (-((((-locals.var_vec2d_dn4) * locals.var_pb22d) - (assign31700_e53785 * locals.var_pb22d_dn4)) / (locals.var_pb22d * locals.var_pb22d))), (-(locals.var_ved_jct_dn5 / locals.var_pb22d)),)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31700_e53790;
        locals.var_arg__blk581_dn3 = assign31700_e53790_d_n3;
        locals.var_arg__blk581_dn4 = assign31700_e53790_d_n4;
        locals.var_arg__blk581_dn5 = assign31700_e53790_d_n5;
        locals.var_arg__blk581_rv = 0.0;

        let assign31710_e53793: f64 = if p.p1611 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign31710_e53793;
        locals.var_guard592_rv = 0.0;

        let assign31720_e53796: f64 = if p.p1611 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign31720_e53796;
        locals.var_guard593_rv = 0.0;

        let (assign31730_e53816, assign31730_e53816_d_n3, assign31730_e53816_d_n4, assign31730_e53816_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign31730_e53813: f64 = (locals.var_arg__blk581).sqrt();
        let assign31730_e53814: f64 = (1.0 / assign31730_e53813);
        (assign31730_e53814, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31730_e53813)) / (assign31730_e53813 * assign31730_e53813))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31730_e53813)) / (assign31730_e53813 * assign31730_e53813))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31730_e53813)) / (assign31730_e53813 * assign31730_e53813))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31730_e53816;
        locals.var_sarg__blk582_dn3 = assign31730_e53816_d_n3;
        locals.var_sarg__blk582_dn4 = assign31730_e53816_d_n4;
        locals.var_sarg__blk582_dn5 = assign31730_e53816_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31740_e53837, assign31740_e53837_d_n3, assign31740_e53837_d_n4, assign31740_e53837_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign31740_e53834: f64 = (-p.p1611);
        let assign31740_e53835: f64 = (locals.var_arg__blk581).powf(assign31740_e53834);
        (assign31740_e53835, if 0.0 == 0.0 && ((assign31740_e53834) as f64).is_finite() && ((assign31740_e53834) as f64).fract() == 0.0 { if assign31740_e53834 == 0.0 { 0.0 } else { (assign31740_e53834 * ((locals.var_arg__blk581).powf(assign31740_e53834 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31740_e53835 * (assign31740_e53834 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31740_e53834) as f64).is_finite() && ((assign31740_e53834) as f64).fract() == 0.0 { if assign31740_e53834 == 0.0 { 0.0 } else { (assign31740_e53834 * ((locals.var_arg__blk581).powf(assign31740_e53834 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31740_e53835 * (assign31740_e53834 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31740_e53834) as f64).is_finite() && ((assign31740_e53834) as f64).fract() == 0.0 { if assign31740_e53834 == 0.0 { 0.0 } else { (assign31740_e53834 * ((locals.var_arg__blk581).powf(assign31740_e53834 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31740_e53835 * (assign31740_e53834 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31740_e53837;
        locals.var_sarg__blk582_dn3 = assign31740_e53837_d_n3;
        locals.var_sarg__blk582_dn4 = assign31740_e53837_d_n4;
        locals.var_sarg__blk582_dn5 = assign31740_e53837_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31750_e53868, assign31750_e53868_d_n3, assign31750_e53868_d_n4, assign31750_e53868_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 != 0.0)) {
        let assign31750_e53853: f64 = (p.p1605 * locals.var_pb22d);
        let assign31750_e53855: f64 = (assign31750_e53853 * locals.var_czbdsw);
        let assign31750_e53859: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31750_e53860: f64 = (1.0 - assign31750_e53859);
        let assign31750_e53861: f64 = (assign31750_e53855 * assign31750_e53860);
        let assign31750_e53864: f64 = (1.0 - p.p1611);
        let assign31750_e53865: f64 = (assign31750_e53861 / assign31750_e53864);
        let assign31750_e53866: f64 = (locals.var_qec__blk583 + assign31750_e53865);
        (assign31750_e53866, (locals.var_qec__blk583_dn3 + ((assign31750_e53855 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31750_e53864)), (locals.var_qec__blk583_dn4 + ((((((p.p1605 * locals.var_pb22d_dn4) * locals.var_czbdsw) + (assign31750_e53853 * locals.var_czbdsw_dn4)) * assign31750_e53860) + (assign31750_e53855 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31750_e53864)), (locals.var_qec__blk583_dn5 + ((assign31750_e53855 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31750_e53864)),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31750_e53868;
        locals.var_qedj2_dn3 = assign31750_e53868_d_n3;
        locals.var_qedj2_dn4 = assign31750_e53868_d_n4;
        locals.var_qedj2_dn5 = assign31750_e53868_d_n5;
        locals.var_qedj2_rv = 0.0;

        let (assign31760_e53905, assign31760_e53905_d_n3, assign31760_e53905_d_n4, assign31760_e53905_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) && (locals.var_guard592 == 0.0)) {
        let assign31760_e53885: f64 = (p.p1605 * locals.var_pb22d);
        let assign31760_e53887: f64 = (assign31760_e53885 * locals.var_czbdsw);
        let (assign31760_e53901, assign31760_e53901_d_n3, assign31760_e53901_d_n4, assign31760_e53901_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31760_e53893: f64 = (-87.498233534);
                (assign31760_e53893, 0.0, 0.0, 0.0,)
            } else {
                let (assign31760_e53900, assign31760_e53900_d_n3, assign31760_e53900_d_n4, assign31760_e53900_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31760_e53898: f64 = (locals.var_arg__blk581).ln();
                        (assign31760_e53898, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31760_e53900, assign31760_e53900_d_n3, assign31760_e53900_d_n4, assign31760_e53900_d_n5,)
            }
        };
        let assign31760_e53902: f64 = (assign31760_e53887 * assign31760_e53901);
        let assign31760_e53903: f64 = (locals.var_qec__blk583 - assign31760_e53902);
        (assign31760_e53903, (locals.var_qec__blk583_dn3 - (assign31760_e53887 * assign31760_e53901_d_n3)), (locals.var_qec__blk583_dn4 - (((((p.p1605 * locals.var_pb22d_dn4) * locals.var_czbdsw) + (assign31760_e53885 * locals.var_czbdsw_dn4)) * assign31760_e53901) + (assign31760_e53887 * assign31760_e53901_d_n4))), (locals.var_qec__blk583_dn5 - (assign31760_e53887 * assign31760_e53901_d_n5)),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31760_e53905;
        locals.var_qedj2_dn3 = assign31760_e53905_d_n3;
        locals.var_qedj2_dn4 = assign31760_e53905_d_n4;
        locals.var_qedj2_dn5 = assign31760_e53905_d_n5;
        locals.var_qedj2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_125(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31770_e53918, assign31770_e53918_d_n3, assign31770_e53918_d_n4, assign31770_e53918_d_n5,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign31770_e53916: f64 = (1.0 - locals.var_t1__blk576);
        (assign31770_e53916, (-locals.var_t1__blk576_dn3), (-locals.var_t1__blk576_dn4), (-locals.var_t1__blk576_dn5),)
    } else {
        (locals.var_arg__blk581, locals.var_arg__blk581_dn3, locals.var_arg__blk581_dn4, locals.var_arg__blk581_dn5,)
    }
};
        locals.var_arg__blk581 = assign31770_e53918;
        locals.var_arg__blk581_dn3 = assign31770_e53918_d_n3;
        locals.var_arg__blk581_dn4 = assign31770_e53918_d_n4;
        locals.var_arg__blk581_dn5 = assign31770_e53918_d_n5;
        locals.var_arg__blk581_rv = 0.0;

        let assign31780_e53921: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign31780_e53921;
        locals.var_guard594_rv = 0.0;

        let assign31790_e53924: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign31790_e53924;
        locals.var_guard595_rv = 0.0;

        let (assign31800_e53942, assign31800_e53942_d_n3, assign31800_e53942_d_n4, assign31800_e53942_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) {
        let assign31800_e53939: f64 = (locals.var_arg__blk581).sqrt();
        let assign31800_e53940: f64 = (1.0 / assign31800_e53939);
        (assign31800_e53940, (-((locals.var_arg__blk581_dn3 / (2.0 * assign31800_e53939)) / (assign31800_e53939 * assign31800_e53939))), (-((locals.var_arg__blk581_dn4 / (2.0 * assign31800_e53939)) / (assign31800_e53939 * assign31800_e53939))), (-((locals.var_arg__blk581_dn5 / (2.0 * assign31800_e53939)) / (assign31800_e53939 * assign31800_e53939))),)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31800_e53942;
        locals.var_sarg__blk582_dn3 = assign31800_e53942_d_n3;
        locals.var_sarg__blk582_dn4 = assign31800_e53942_d_n4;
        locals.var_sarg__blk582_dn5 = assign31800_e53942_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31810_e53961, assign31810_e53961_d_n3, assign31810_e53961_d_n4, assign31810_e53961_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 == 0.0)) {
        let assign31810_e53958: f64 = (-p.p1599);
        let assign31810_e53959: f64 = (locals.var_arg__blk581).powf(assign31810_e53958);
        (assign31810_e53959, if 0.0 == 0.0 && ((assign31810_e53958) as f64).is_finite() && ((assign31810_e53958) as f64).fract() == 0.0 { if assign31810_e53958 == 0.0 { 0.0 } else { (assign31810_e53958 * ((locals.var_arg__blk581).powf(assign31810_e53958 - 1.0) * locals.var_arg__blk581_dn3)) } } else { (assign31810_e53959 * (assign31810_e53958 * (locals.var_arg__blk581_dn3 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31810_e53958) as f64).is_finite() && ((assign31810_e53958) as f64).fract() == 0.0 { if assign31810_e53958 == 0.0 { 0.0 } else { (assign31810_e53958 * ((locals.var_arg__blk581).powf(assign31810_e53958 - 1.0) * locals.var_arg__blk581_dn4)) } } else { (assign31810_e53959 * (assign31810_e53958 * (locals.var_arg__blk581_dn4 / locals.var_arg__blk581))) }, if 0.0 == 0.0 && ((assign31810_e53958) as f64).is_finite() && ((assign31810_e53958) as f64).fract() == 0.0 { if assign31810_e53958 == 0.0 { 0.0 } else { (assign31810_e53958 * ((locals.var_arg__blk581).powf(assign31810_e53958 - 1.0) * locals.var_arg__blk581_dn5)) } } else { (assign31810_e53959 * (assign31810_e53958 * (locals.var_arg__blk581_dn5 / locals.var_arg__blk581))) },)
    } else {
        (locals.var_sarg__blk582, locals.var_sarg__blk582_dn3, locals.var_sarg__blk582_dn4, locals.var_sarg__blk582_dn5,)
    }
};
        locals.var_sarg__blk582 = assign31810_e53961;
        locals.var_sarg__blk582_dn3 = assign31810_e53961_d_n3;
        locals.var_sarg__blk582_dn4 = assign31810_e53961_d_n4;
        locals.var_sarg__blk582_dn5 = assign31810_e53961_d_n5;
        locals.var_sarg__blk582_rv = 0.0;

        let (assign31820_e53986, assign31820_e53986_d_n3, assign31820_e53986_d_n4, assign31820_e53986_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 != 0.0)) {
        let assign31820_e53974: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31820_e53978: f64 = (locals.var_arg__blk581 * locals.var_sarg__blk582);
        let assign31820_e53979: f64 = (1.0 - assign31820_e53978);
        let assign31820_e53980: f64 = (assign31820_e53974 * assign31820_e53979);
        let assign31820_e53983: f64 = (1.0 - p.p1599);
        let assign31820_e53984: f64 = (assign31820_e53980 / assign31820_e53983);
        (assign31820_e53984, ((assign31820_e53974 * (-((locals.var_arg__blk581_dn3 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn3)))) / assign31820_e53983), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31820_e53979) + (assign31820_e53974 * (-((locals.var_arg__blk581_dn4 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn4))))) / assign31820_e53983), ((assign31820_e53974 * (-((locals.var_arg__blk581_dn5 * locals.var_sarg__blk582) + (locals.var_arg__blk581 * locals.var_sarg__blk582_dn5)))) / assign31820_e53983),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31820_e53986;
        locals.var_qedj2_dn3 = assign31820_e53986_d_n3;
        locals.var_qedj2_dn4 = assign31820_e53986_d_n4;
        locals.var_qedj2_dn5 = assign31820_e53986_d_n5;
        locals.var_qedj2_rv = 0.0;

        let (assign31830_e54018, assign31830_e54018_d_n3, assign31830_e54018_d_n4, assign31830_e54018_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) && (locals.var_guard594 == 0.0)) {
        let assign31830_e53999: f64 = (-locals.var_pbswd_t);
        let assign31830_e54001: f64 = (assign31830_e53999 * locals.var_czbdsw);
        let (assign31830_e54015, assign31830_e54015_d_n3, assign31830_e54015_d_n4, assign31830_e54015_d_n5,) = {
            if (!(locals.var_arg__blk581 > 1e-38)) {
                let assign31830_e54007: f64 = (-87.498233534);
                (assign31830_e54007, 0.0, 0.0, 0.0,)
            } else {
                let (assign31830_e54014, assign31830_e54014_d_n3, assign31830_e54014_d_n4, assign31830_e54014_d_n5,) = {
                    if (locals.var_arg__blk581 > 1e-38) {
                        let assign31830_e54012: f64 = (locals.var_arg__blk581).ln();
                        (assign31830_e54012, (locals.var_arg__blk581_dn3 / locals.var_arg__blk581), (locals.var_arg__blk581_dn4 / locals.var_arg__blk581), (locals.var_arg__blk581_dn5 / locals.var_arg__blk581),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign31830_e54014, assign31830_e54014_d_n3, assign31830_e54014_d_n4, assign31830_e54014_d_n5,)
            }
        };
        let assign31830_e54016: f64 = (assign31830_e54001 * assign31830_e54015);
        (assign31830_e54016, (assign31830_e54001 * assign31830_e54015_d_n3), (((((-locals.var_pbswd_t_dn4) * locals.var_czbdsw) + (assign31830_e53999 * locals.var_czbdsw_dn4)) * assign31830_e54015) + (assign31830_e54001 * assign31830_e54015_d_n4)), (assign31830_e54001 * assign31830_e54015_d_n5),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31830_e54018;
        locals.var_qedj2_dn3 = assign31830_e54018_d_n3;
        locals.var_qedj2_dn4 = assign31830_e54018_d_n4;
        locals.var_qedj2_dn5 = assign31830_e54018_d_n5;
        locals.var_qedj2_rv = 0.0;

        let assign31840_e54021: f64 = if p.p1599 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign31840_e54021;
        locals.var_guard596_rv = 0.0;

        let assign31850_e54024: f64 = if p.p1599 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign31850_e54024;
        locals.var_guard597_rv = 0.0;

        let (assign31860_e54040,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) {
        let assign31860_e54037: f64 = (0.1_f64).sqrt();
        let assign31860_e54038: f64 = (1.0 / assign31860_e54037);
        (assign31860_e54038,)
    } else {
        (locals.var_t2__blk577,)
    }
};
        locals.var_t2__blk577 = assign31860_e54040;
        locals.var_t2__blk577_rv = 0.0;

        let (assign31870_e54057,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 == 0.0)) {
        let assign31870_e54054: f64 = (-p.p1599);
        let assign31870_e54055: f64 = (0.1_f64).powf(assign31870_e54054);
        (assign31870_e54055,)
    } else {
        (locals.var_t2__blk577,)
    }
};
        locals.var_t2__blk577 = assign31870_e54057;
        locals.var_t2__blk577_rv = 0.0;

        let (assign31880_e54072,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign31880_e54069: f64 = (1.0 - p.p1599);
        let assign31880_e54070: f64 = (1.0 / assign31880_e54069);
        (assign31880_e54070,)
    } else {
        (locals.var_t3__blk578,)
    }
};
        locals.var_t3__blk578 = assign31880_e54072;
        locals.var_t3__blk578_rv = 0.0;

        let (assign31890_e54095,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 != 0.0)) {
        let assign31890_e54085: f64 = (0.05 * p.p1599);
        let assign31890_e54088: f64 = (1.0 + p.p1599);
        let assign31890_e54089: f64 = (assign31890_e54085 * assign31890_e54088);
        let assign31890_e54091: f64 = (assign31890_e54089 * locals.var_t2__blk577);
        let assign31890_e54092: f64 = (1.0 - assign31890_e54091);
        let assign31890_e54093: f64 = (locals.var_t3__blk578 * assign31890_e54092);
        (assign31890_e54093,)
    } else {
        (locals.var_t5__blk580,)
    }
};
        locals.var_t5__blk580 = assign31890_e54095;
        locals.var_t5__blk580_rv = 0.0;

        let (assign31900_e54107,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk577,)
    }
};
        locals.var_t2__blk577 = assign31900_e54107;
        locals.var_t2__blk577_rv = 0.0;

        let (assign31910_e54122,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) && (locals.var_guard596 == 0.0)) {
        let assign31910_e54119: f64 = (0.1_f64).ln();
        let assign31910_e54120: f64 = (1.5 - assign31910_e54119);
        (assign31910_e54120,)
    } else {
        (locals.var_t5__blk580,)
    }
};
        locals.var_t5__blk580 = assign31910_e54122;
        locals.var_t5__blk580_rv = 0.0;

        let (assign31920_e54147, assign31920_e54147_d_n3, assign31920_e54147_d_n4, assign31920_e54147_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign31920_e54132: f64 = (locals.var_t1__blk576 - 1.0);
        let assign31920_e54133: f64 = (locals.var_t2__blk577 * assign31920_e54132);
        let assign31920_e54136: f64 = (5.0 * p.p1599);
        let assign31920_e54139: f64 = (locals.var_t1__blk576 - 1.0);
        let assign31920_e54140: f64 = (assign31920_e54136 * assign31920_e54139);
        let assign31920_e54143: f64 = (1.0 + p.p1599);
        let assign31920_e54144: f64 = (assign31920_e54140 + assign31920_e54143);
        let assign31920_e54145: f64 = (assign31920_e54133 * assign31920_e54144);
        (assign31920_e54145, (((locals.var_t2__blk577 * locals.var_t1__blk576_dn3) * assign31920_e54144) + (assign31920_e54133 * (assign31920_e54136 * locals.var_t1__blk576_dn3))), (((locals.var_t2__blk577 * locals.var_t1__blk576_dn4) * assign31920_e54144) + (assign31920_e54133 * (assign31920_e54136 * locals.var_t1__blk576_dn4))), (((locals.var_t2__blk577 * locals.var_t1__blk576_dn5) * assign31920_e54144) + (assign31920_e54133 * (assign31920_e54136 * locals.var_t1__blk576_dn5))),)
    } else {
        (locals.var_t4__blk579, locals.var_t4__blk579_dn3, locals.var_t4__blk579_dn4, locals.var_t4__blk579_dn5,)
    }
};
        locals.var_t4__blk579 = assign31920_e54147;
        locals.var_t4__blk579_dn3 = assign31920_e54147_d_n3;
        locals.var_t4__blk579_dn4 = assign31920_e54147_d_n4;
        locals.var_t4__blk579_dn5 = assign31920_e54147_d_n5;
        locals.var_t4__blk579_rv = 0.0;

        let (assign31930_e54162, assign31930_e54162_d_n3, assign31930_e54162_d_n4, assign31930_e54162_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard584 != 0.0)) && (locals.var_guard585 == 0.0)) {
        let assign31930_e54156: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign31930_e54159: f64 = (locals.var_t4__blk579 + locals.var_t5__blk580);
        let assign31930_e54160: f64 = (assign31930_e54156 * assign31930_e54159);
        (assign31930_e54160, (assign31930_e54156 * locals.var_t4__blk579_dn3), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign31930_e54159) + (assign31930_e54156 * locals.var_t4__blk579_dn4)), (assign31930_e54156 * locals.var_t4__blk579_dn5),)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31930_e54162;
        locals.var_qedj2_dn3 = assign31930_e54162_d_n3;
        locals.var_qedj2_dn4 = assign31930_e54162_d_n4;
        locals.var_qedj2_dn5 = assign31930_e54162_d_n5;
        locals.var_qedj2_rv = 0.0;

        let (assign31940_e54169, assign31940_e54169_d_n3, assign31940_e54169_d_n4, assign31940_e54169_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard584 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qedj2, locals.var_qedj2_dn3, locals.var_qedj2_dn4, locals.var_qedj2_dn5,)
    }
};
        locals.var_qedj2 = assign31940_e54169;
        locals.var_qedj2_dn3 = assign31940_e54169_d_n3;
        locals.var_qedj2_dn4 = assign31940_e54169_d_n4;
        locals.var_qedj2_dn5 = assign31940_e54169_d_n5;
        locals.var_qedj2_rv = 0.0;

        let assign31950_e54172: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard606 = assign31950_e54172;
        locals.var_guard606_rv = 0.0;

        let (assign31960_e54180, assign31960_e54180_d_n3, assign31960_e54180_d_n4, assign31960_e54180_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) {
        let assign31960_e54178: f64 = (locals.var_ved_jct / locals.var_pbswgd_t);
        (assign31960_e54178, (locals.var_ved_jct_dn3 / locals.var_pbswgd_t), (-((locals.var_ved_jct * locals.var_pbswgd_t_dn4) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (locals.var_ved_jct_dn5 / locals.var_pbswgd_t),)
    } else {
        (locals.var_t1__blk598, locals.var_t1__blk598_dn3, locals.var_t1__blk598_dn4, locals.var_t1__blk598_dn5,)
    }
};
        locals.var_t1__blk598 = assign31960_e54180;
        locals.var_t1__blk598_dn3 = assign31960_e54180_d_n3;
        locals.var_t1__blk598_dn4 = assign31960_e54180_d_n4;
        locals.var_t1__blk598_dn5 = assign31960_e54180_d_n5;
        locals.var_t1__blk598_rv = 0.0;

        let assign31970_e54183: f64 = if locals.var_t1__blk598 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign31970_e54183;
        locals.var_guard607_rv = 0.0;

        let assign31980_e54186: f64 = if p.p1607 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign31980_e54186;
        locals.var_guard608_rv = 0.0;

        let assign31990_e54189: f64 = if locals.var_ved_jct > locals.var_vec3d { 1.0 } else { 0.0 };
        locals.var_guard609 = assign31990_e54189;
        locals.var_guard609_rv = 0.0;

        let (assign32000_e54203, assign32000_e54203_d_n3, assign32000_e54203_d_n4, assign32000_e54203_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) {
        let assign32000_e54201: f64 = (1.0 - locals.var_t1__blk598);
        (assign32000_e54201, (-locals.var_t1__blk598_dn3), (-locals.var_t1__blk598_dn4), (-locals.var_t1__blk598_dn5),)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32000_e54203;
        locals.var_arg__blk603_dn3 = assign32000_e54203_d_n3;
        locals.var_arg__blk603_dn4 = assign32000_e54203_d_n4;
        locals.var_arg__blk603_dn5 = assign32000_e54203_d_n5;
        locals.var_arg__blk603_rv = 0.0;

        let assign32010_e54206: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard610 = assign32010_e54206;
        locals.var_guard610_rv = 0.0;

        let assign32020_e54209: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign32020_e54209;
        locals.var_guard611_rv = 0.0;

        let (assign32030_e54228, assign32030_e54228_d_n3, assign32030_e54228_d_n4, assign32030_e54228_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 != 0.0)) && (locals.var_guard611 != 0.0)) {
        let assign32030_e54225: f64 = (locals.var_arg__blk603).sqrt();
        let assign32030_e54226: f64 = (1.0 / assign32030_e54225);
        (assign32030_e54226, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32030_e54225)) / (assign32030_e54225 * assign32030_e54225))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32030_e54225)) / (assign32030_e54225 * assign32030_e54225))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32030_e54225)) / (assign32030_e54225 * assign32030_e54225))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32030_e54228;
        locals.var_sarg__blk604_dn3 = assign32030_e54228_d_n3;
        locals.var_sarg__blk604_dn4 = assign32030_e54228_d_n4;
        locals.var_sarg__blk604_dn5 = assign32030_e54228_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32040_e54248, assign32040_e54248_d_n3, assign32040_e54248_d_n4, assign32040_e54248_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 != 0.0)) && (locals.var_guard611 == 0.0)) {
        let assign32040_e54245: f64 = (-p.p1601);
        let assign32040_e54246: f64 = (locals.var_arg__blk603).powf(assign32040_e54245);
        (assign32040_e54246, if 0.0 == 0.0 && ((assign32040_e54245) as f64).is_finite() && ((assign32040_e54245) as f64).fract() == 0.0 { if assign32040_e54245 == 0.0 { 0.0 } else { (assign32040_e54245 * ((locals.var_arg__blk603).powf(assign32040_e54245 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32040_e54246 * (assign32040_e54245 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32040_e54245) as f64).is_finite() && ((assign32040_e54245) as f64).fract() == 0.0 { if assign32040_e54245 == 0.0 { 0.0 } else { (assign32040_e54245 * ((locals.var_arg__blk603).powf(assign32040_e54245 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32040_e54246 * (assign32040_e54245 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32040_e54245) as f64).is_finite() && ((assign32040_e54245) as f64).fract() == 0.0 { if assign32040_e54245 == 0.0 { 0.0 } else { (assign32040_e54245 * ((locals.var_arg__blk603).powf(assign32040_e54245 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32040_e54246 * (assign32040_e54245 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32040_e54248;
        locals.var_sarg__blk604_dn3 = assign32040_e54248_d_n3;
        locals.var_sarg__blk604_dn4 = assign32040_e54248_d_n4;
        locals.var_sarg__blk604_dn5 = assign32040_e54248_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32050_e54274, assign32050_e54274_d_n3, assign32050_e54274_d_n4, assign32050_e54274_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 != 0.0)) {
        let assign32050_e54262: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32050_e54266: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32050_e54267: f64 = (1.0 - assign32050_e54266);
        let assign32050_e54268: f64 = (assign32050_e54262 * assign32050_e54267);
        let assign32050_e54271: f64 = (1.0 - p.p1601);
        let assign32050_e54272: f64 = (assign32050_e54268 / assign32050_e54271);
        (assign32050_e54272, ((assign32050_e54262 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32050_e54271), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32050_e54267) + (assign32050_e54262 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32050_e54271), ((assign32050_e54262 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32050_e54271),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32050_e54274;
        locals.var_qedj3_dn3 = assign32050_e54274_d_n3;
        locals.var_qedj3_dn4 = assign32050_e54274_d_n4;
        locals.var_qedj3_dn5 = assign32050_e54274_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32060_e54307, assign32060_e54307_d_n3, assign32060_e54307_d_n4, assign32060_e54307_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 == 0.0)) {
        let assign32060_e54288: f64 = (-locals.var_pbswgd_t);
        let assign32060_e54290: f64 = (assign32060_e54288 * locals.var_czbdswg);
        let (assign32060_e54304, assign32060_e54304_d_n3, assign32060_e54304_d_n4, assign32060_e54304_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32060_e54296: f64 = (-87.498233534);
                (assign32060_e54296, 0.0, 0.0, 0.0,)
            } else {
                let (assign32060_e54303, assign32060_e54303_d_n3, assign32060_e54303_d_n4, assign32060_e54303_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32060_e54301: f64 = (locals.var_arg__blk603).ln();
                        (assign32060_e54301, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32060_e54303, assign32060_e54303_d_n3, assign32060_e54303_d_n4, assign32060_e54303_d_n5,)
            }
        };
        let assign32060_e54305: f64 = (assign32060_e54290 * assign32060_e54304);
        (assign32060_e54305, (assign32060_e54290 * assign32060_e54304_d_n3), (((((-locals.var_pbswgd_t_dn4) * locals.var_czbdswg) + (assign32060_e54288 * locals.var_czbdswg_dn4)) * assign32060_e54304) + (assign32060_e54290 * assign32060_e54304_d_n4)), (assign32060_e54290 * assign32060_e54304_d_n5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32060_e54307;
        locals.var_qedj3_dn3 = assign32060_e54307_d_n3;
        locals.var_qedj3_dn4 = assign32060_e54307_d_n4;
        locals.var_qedj3_dn5 = assign32060_e54307_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32070_e54324, assign32070_e54324_d_n3, assign32070_e54324_d_n4, assign32070_e54324_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) {
        let assign32070_e54321: f64 = (locals.var_vec3d / locals.var_pbswgd_t);
        let assign32070_e54322: f64 = (1.0 - assign32070_e54321);
        (assign32070_e54322, 0.0, (-(((locals.var_vec3d_dn4 * locals.var_pbswgd_t) - (locals.var_vec3d * locals.var_pbswgd_t_dn4)) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), 0.0,)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32070_e54324;
        locals.var_arg__blk603_dn3 = assign32070_e54324_d_n3;
        locals.var_arg__blk603_dn4 = assign32070_e54324_d_n4;
        locals.var_arg__blk603_dn5 = assign32070_e54324_d_n5;
        locals.var_arg__blk603_rv = 0.0;

        let assign32080_e54327: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign32080_e54327;
        locals.var_guard612_rv = 0.0;

        let assign32090_e54330: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign32090_e54330;
        locals.var_guard613_rv = 0.0;

        let (assign32100_e54350, assign32100_e54350_d_n3, assign32100_e54350_d_n4, assign32100_e54350_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 != 0.0)) && (locals.var_guard613 != 0.0)) {
        let assign32100_e54347: f64 = (locals.var_arg__blk603).sqrt();
        let assign32100_e54348: f64 = (1.0 / assign32100_e54347);
        (assign32100_e54348, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32100_e54347)) / (assign32100_e54347 * assign32100_e54347))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32100_e54347)) / (assign32100_e54347 * assign32100_e54347))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32100_e54347)) / (assign32100_e54347 * assign32100_e54347))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32100_e54350;
        locals.var_sarg__blk604_dn3 = assign32100_e54350_d_n3;
        locals.var_sarg__blk604_dn4 = assign32100_e54350_d_n4;
        locals.var_sarg__blk604_dn5 = assign32100_e54350_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32110_e54371, assign32110_e54371_d_n3, assign32110_e54371_d_n4, assign32110_e54371_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 != 0.0)) && (locals.var_guard613 == 0.0)) {
        let assign32110_e54368: f64 = (-p.p1601);
        let assign32110_e54369: f64 = (locals.var_arg__blk603).powf(assign32110_e54368);
        (assign32110_e54369, if 0.0 == 0.0 && ((assign32110_e54368) as f64).is_finite() && ((assign32110_e54368) as f64).fract() == 0.0 { if assign32110_e54368 == 0.0 { 0.0 } else { (assign32110_e54368 * ((locals.var_arg__blk603).powf(assign32110_e54368 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32110_e54369 * (assign32110_e54368 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32110_e54368) as f64).is_finite() && ((assign32110_e54368) as f64).fract() == 0.0 { if assign32110_e54368 == 0.0 { 0.0 } else { (assign32110_e54368 * ((locals.var_arg__blk603).powf(assign32110_e54368 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32110_e54369 * (assign32110_e54368 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32110_e54368) as f64).is_finite() && ((assign32110_e54368) as f64).fract() == 0.0 { if assign32110_e54368 == 0.0 { 0.0 } else { (assign32110_e54368 * ((locals.var_arg__blk603).powf(assign32110_e54368 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32110_e54369 * (assign32110_e54368 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32110_e54371;
        locals.var_sarg__blk604_dn3 = assign32110_e54371_d_n3;
        locals.var_sarg__blk604_dn4 = assign32110_e54371_d_n4;
        locals.var_sarg__blk604_dn5 = assign32110_e54371_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32120_e54398, assign32120_e54398_d_n3, assign32120_e54398_d_n4, assign32120_e54398_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign32120_e54386: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32120_e54390: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32120_e54391: f64 = (1.0 - assign32120_e54390);
        let assign32120_e54392: f64 = (assign32120_e54386 * assign32120_e54391);
        let assign32120_e54395: f64 = (1.0 - p.p1601);
        let assign32120_e54396: f64 = (assign32120_e54392 / assign32120_e54395);
        (assign32120_e54396, ((assign32120_e54386 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32120_e54395), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32120_e54391) + (assign32120_e54386 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32120_e54395), ((assign32120_e54386 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32120_e54395),)
    } else {
        (locals.var_qec__blk605, locals.var_qec__blk605_dn3, locals.var_qec__blk605_dn4, locals.var_qec__blk605_dn5,)
    }
};
        locals.var_qec__blk605 = assign32120_e54398;
        locals.var_qec__blk605_dn3 = assign32120_e54398_d_n3;
        locals.var_qec__blk605_dn4 = assign32120_e54398_d_n4;
        locals.var_qec__blk605_dn5 = assign32120_e54398_d_n5;
        locals.var_qec__blk605_rv = 0.0;

        let (assign32130_e54432, assign32130_e54432_d_n3, assign32130_e54432_d_n4, assign32130_e54432_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard612 == 0.0)) {
        let assign32130_e54413: f64 = (-locals.var_pbswgd_t);
        let assign32130_e54415: f64 = (assign32130_e54413 * locals.var_czbdswg);
        let (assign32130_e54429, assign32130_e54429_d_n3, assign32130_e54429_d_n4, assign32130_e54429_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32130_e54421: f64 = (-87.498233534);
                (assign32130_e54421, 0.0, 0.0, 0.0,)
            } else {
                let (assign32130_e54428, assign32130_e54428_d_n3, assign32130_e54428_d_n4, assign32130_e54428_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32130_e54426: f64 = (locals.var_arg__blk603).ln();
                        (assign32130_e54426, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32130_e54428, assign32130_e54428_d_n3, assign32130_e54428_d_n4, assign32130_e54428_d_n5,)
            }
        };
        let assign32130_e54430: f64 = (assign32130_e54415 * assign32130_e54429);
        (assign32130_e54430, (assign32130_e54415 * assign32130_e54429_d_n3), (((((-locals.var_pbswgd_t_dn4) * locals.var_czbdswg) + (assign32130_e54413 * locals.var_czbdswg_dn4)) * assign32130_e54429) + (assign32130_e54415 * assign32130_e54429_d_n4)), (assign32130_e54415 * assign32130_e54429_d_n5),)
    } else {
        (locals.var_qec__blk605, locals.var_qec__blk605_dn3, locals.var_qec__blk605_dn4, locals.var_qec__blk605_dn5,)
    }
};
        locals.var_qec__blk605 = assign32130_e54432;
        locals.var_qec__blk605_dn3 = assign32130_e54432_d_n3;
        locals.var_qec__blk605_dn4 = assign32130_e54432_d_n4;
        locals.var_qec__blk605_dn5 = assign32130_e54432_d_n5;
        locals.var_qec__blk605_rv = 0.0;

        let (assign32140_e54451, assign32140_e54451_d_n3, assign32140_e54451_d_n4, assign32140_e54451_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) {
        let assign32140_e54446: f64 = (locals.var_ved_jct - locals.var_vec3d);
        let assign32140_e54448: f64 = (assign32140_e54446 / locals.var_pb23d);
        let assign32140_e54449: f64 = (1.0 - assign32140_e54448);
        (assign32140_e54449, (-(locals.var_ved_jct_dn3 / locals.var_pb23d)), (-((((-locals.var_vec3d_dn4) * locals.var_pb23d) - (assign32140_e54446 * locals.var_pb23d_dn4)) / (locals.var_pb23d * locals.var_pb23d))), (-(locals.var_ved_jct_dn5 / locals.var_pb23d)),)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32140_e54451;
        locals.var_arg__blk603_dn3 = assign32140_e54451_d_n3;
        locals.var_arg__blk603_dn4 = assign32140_e54451_d_n4;
        locals.var_arg__blk603_dn5 = assign32140_e54451_d_n5;
        locals.var_arg__blk603_rv = 0.0;

        let assign32150_e54454: f64 = if p.p1613 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard614 = assign32150_e54454;
        locals.var_guard614_rv = 0.0;

        let assign32160_e54457: f64 = if p.p1613 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign32160_e54457;
        locals.var_guard615_rv = 0.0;

        let (assign32170_e54477, assign32170_e54477_d_n3, assign32170_e54477_d_n4, assign32170_e54477_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign32170_e54474: f64 = (locals.var_arg__blk603).sqrt();
        let assign32170_e54475: f64 = (1.0 / assign32170_e54474);
        (assign32170_e54475, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32170_e54474)) / (assign32170_e54474 * assign32170_e54474))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32170_e54474)) / (assign32170_e54474 * assign32170_e54474))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32170_e54474)) / (assign32170_e54474 * assign32170_e54474))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32170_e54477;
        locals.var_sarg__blk604_dn3 = assign32170_e54477_d_n3;
        locals.var_sarg__blk604_dn4 = assign32170_e54477_d_n4;
        locals.var_sarg__blk604_dn5 = assign32170_e54477_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

    }
}
