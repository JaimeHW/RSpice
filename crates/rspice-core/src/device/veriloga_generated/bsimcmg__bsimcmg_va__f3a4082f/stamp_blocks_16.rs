#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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

    pub(super) fn stamp_reactive_block_126(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (assign32180_e54498, assign32180_e54498_d_n3, assign32180_e54498_d_n4, assign32180_e54498_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign32180_e54495: f64 = (-p.p1613);
        let assign32180_e54496: f64 = (locals.var_arg__blk603).powf(assign32180_e54495);
        (assign32180_e54496, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32180_e54498;
        locals.var_sarg__blk604_dn3 = assign32180_e54498_d_n3;
        locals.var_sarg__blk604_dn4 = assign32180_e54498_d_n4;
        locals.var_sarg__blk604_dn5 = assign32180_e54498_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32190_e54529, assign32190_e54529_d_n3, assign32190_e54529_d_n4, assign32190_e54529_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) {
        let assign32190_e54514: f64 = (p.p1607 * locals.var_pb23d);
        let assign32190_e54516: f64 = (assign32190_e54514 * locals.var_czbdswg);
        let assign32190_e54520: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32190_e54521: f64 = (1.0 - assign32190_e54520);
        let assign32190_e54522: f64 = (assign32190_e54516 * assign32190_e54521);
        let assign32190_e54525: f64 = (1.0 - p.p1613);
        let assign32190_e54526: f64 = (assign32190_e54522 / assign32190_e54525);
        let assign32190_e54527: f64 = (locals.var_qec__blk605 + assign32190_e54526);
        (assign32190_e54527, (locals.var_qec__blk605_dn3 + ((assign32190_e54516 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32190_e54525)), (locals.var_qec__blk605_dn4 + ((((((p.p1607 * locals.var_pb23d_dn4) * locals.var_czbdswg) + (assign32190_e54514 * locals.var_czbdswg_dn4)) * assign32190_e54521) + (assign32190_e54516 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32190_e54525)), (locals.var_qec__blk605_dn5 + ((assign32190_e54516 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32190_e54525)),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32190_e54529;
        locals.var_qedj3_dn3 = assign32190_e54529_d_n3;
        locals.var_qedj3_dn4 = assign32190_e54529_d_n4;
        locals.var_qedj3_dn5 = assign32190_e54529_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32200_e54566, assign32200_e54566_d_n3, assign32200_e54566_d_n4, assign32200_e54566_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 == 0.0)) {
        let assign32200_e54546: f64 = (p.p1607 * locals.var_pb23d);
        let assign32200_e54548: f64 = (assign32200_e54546 * locals.var_czbdswg);
        let (assign32200_e54562, assign32200_e54562_d_n3, assign32200_e54562_d_n4, assign32200_e54562_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32200_e54554: f64 = (-87.498233534);
                (assign32200_e54554, 0.0, 0.0, 0.0,)
            } else {
                let (assign32200_e54561, assign32200_e54561_d_n3, assign32200_e54561_d_n4, assign32200_e54561_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32200_e54559: f64 = (locals.var_arg__blk603).ln();
                        (assign32200_e54559, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32200_e54561, assign32200_e54561_d_n3, assign32200_e54561_d_n4, assign32200_e54561_d_n5,)
            }
        };
        let assign32200_e54563: f64 = (assign32200_e54548 * assign32200_e54562);
        let assign32200_e54564: f64 = (locals.var_qec__blk605 - assign32200_e54563);
        (assign32200_e54564, (locals.var_qec__blk605_dn3 - (assign32200_e54548 * assign32200_e54562_d_n3)), (locals.var_qec__blk605_dn4 - (((((p.p1607 * locals.var_pb23d_dn4) * locals.var_czbdswg) + (assign32200_e54546 * locals.var_czbdswg_dn4)) * assign32200_e54562) + (assign32200_e54548 * assign32200_e54562_d_n4))), (locals.var_qec__blk605_dn5 - (assign32200_e54548 * assign32200_e54562_d_n5)),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32200_e54566;
        locals.var_qedj3_dn3 = assign32200_e54566_d_n3;
        locals.var_qedj3_dn4 = assign32200_e54566_d_n4;
        locals.var_qedj3_dn5 = assign32200_e54566_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32210_e54579, assign32210_e54579_d_n3, assign32210_e54579_d_n4, assign32210_e54579_d_n5,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) {
        let assign32210_e54577: f64 = (1.0 - locals.var_t1__blk598);
        (assign32210_e54577, (-locals.var_t1__blk598_dn3), (-locals.var_t1__blk598_dn4), (-locals.var_t1__blk598_dn5),)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32210_e54579;
        locals.var_arg__blk603_dn3 = assign32210_e54579_d_n3;
        locals.var_arg__blk603_dn4 = assign32210_e54579_d_n4;
        locals.var_arg__blk603_dn5 = assign32210_e54579_d_n5;
        locals.var_arg__blk603_rv = 0.0;

        let assign32220_e54582: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign32220_e54582;
        locals.var_guard616_rv = 0.0;

        let assign32230_e54585: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard617 = assign32230_e54585;
        locals.var_guard617_rv = 0.0;

        let (assign32240_e54603, assign32240_e54603_d_n3, assign32240_e54603_d_n4, assign32240_e54603_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 != 0.0)) {
        let assign32240_e54600: f64 = (locals.var_arg__blk603).sqrt();
        let assign32240_e54601: f64 = (1.0 / assign32240_e54600);
        (assign32240_e54601, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32240_e54603;
        locals.var_sarg__blk604_dn3 = assign32240_e54603_d_n3;
        locals.var_sarg__blk604_dn4 = assign32240_e54603_d_n4;
        locals.var_sarg__blk604_dn5 = assign32240_e54603_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32250_e54622, assign32250_e54622_d_n3, assign32250_e54622_d_n4, assign32250_e54622_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 == 0.0)) {
        let assign32250_e54619: f64 = (-p.p1601);
        let assign32250_e54620: f64 = (locals.var_arg__blk603).powf(assign32250_e54619);
        (assign32250_e54620, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32250_e54622;
        locals.var_sarg__blk604_dn3 = assign32250_e54622_d_n3;
        locals.var_sarg__blk604_dn4 = assign32250_e54622_d_n4;
        locals.var_sarg__blk604_dn5 = assign32250_e54622_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32260_e54647, assign32260_e54647_d_n3, assign32260_e54647_d_n4, assign32260_e54647_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) {
        let assign32260_e54635: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32260_e54639: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32260_e54640: f64 = (1.0 - assign32260_e54639);
        let assign32260_e54641: f64 = (assign32260_e54635 * assign32260_e54640);
        let assign32260_e54644: f64 = (1.0 - p.p1601);
        let assign32260_e54645: f64 = (assign32260_e54641 / assign32260_e54644);
        (assign32260_e54645, ((assign32260_e54635 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32260_e54644), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32260_e54640) + (assign32260_e54635 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32260_e54644), ((assign32260_e54635 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32260_e54644),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32260_e54647;
        locals.var_qedj3_dn3 = assign32260_e54647_d_n3;
        locals.var_qedj3_dn4 = assign32260_e54647_d_n4;
        locals.var_qedj3_dn5 = assign32260_e54647_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32270_e54679, assign32270_e54679_d_n3, assign32270_e54679_d_n4, assign32270_e54679_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign32270_e54660: f64 = (-locals.var_pbswgd_t);
        let assign32270_e54662: f64 = (assign32270_e54660 * locals.var_czbdswg);
        let (assign32270_e54676, assign32270_e54676_d_n3, assign32270_e54676_d_n4, assign32270_e54676_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32270_e54668: f64 = (-87.498233534);
                (assign32270_e54668, 0.0, 0.0, 0.0,)
            } else {
                let (assign32270_e54675, assign32270_e54675_d_n3, assign32270_e54675_d_n4, assign32270_e54675_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32270_e54673: f64 = (locals.var_arg__blk603).ln();
                        (assign32270_e54673, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32270_e54675, assign32270_e54675_d_n3, assign32270_e54675_d_n4, assign32270_e54675_d_n5,)
            }
        };
        let assign32270_e54677: f64 = (assign32270_e54662 * assign32270_e54676);
        (assign32270_e54677, (assign32270_e54662 * assign32270_e54676_d_n3), (((((-locals.var_pbswgd_t_dn4) * locals.var_czbdswg) + (assign32270_e54660 * locals.var_czbdswg_dn4)) * assign32270_e54676) + (assign32270_e54662 * assign32270_e54676_d_n4)), (assign32270_e54662 * assign32270_e54676_d_n5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32270_e54679;
        locals.var_qedj3_dn3 = assign32270_e54679_d_n3;
        locals.var_qedj3_dn4 = assign32270_e54679_d_n4;
        locals.var_qedj3_dn5 = assign32270_e54679_d_n5;
        locals.var_qedj3_rv = 0.0;

        let assign32280_e54682: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard618 = assign32280_e54682;
        locals.var_guard618_rv = 0.0;

        let assign32290_e54685: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard619 = assign32290_e54685;
        locals.var_guard619_rv = 0.0;

        let (assign32300_e54701,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign32300_e54698: f64 = (0.1_f64).sqrt();
        let assign32300_e54699: f64 = (1.0 / assign32300_e54698);
        (assign32300_e54699,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32300_e54701;
        locals.var_t2__blk599_rv = 0.0;

        let (assign32310_e54718,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) {
        let assign32310_e54715: f64 = (-p.p1601);
        let assign32310_e54716: f64 = (0.1_f64).powf(assign32310_e54715);
        (assign32310_e54716,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32310_e54718;
        locals.var_t2__blk599_rv = 0.0;

        let (assign32320_e54733,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) {
        let assign32320_e54730: f64 = (1.0 - p.p1601);
        let assign32320_e54731: f64 = (1.0 / assign32320_e54730);
        (assign32320_e54731,)
    } else {
        (locals.var_t3__blk600,)
    }
};
        locals.var_t3__blk600 = assign32320_e54733;
        locals.var_t3__blk600_rv = 0.0;

        let (assign32330_e54756,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) {
        let assign32330_e54746: f64 = (0.05 * p.p1601);
        let assign32330_e54749: f64 = (1.0 + p.p1601);
        let assign32330_e54750: f64 = (assign32330_e54746 * assign32330_e54749);
        let assign32330_e54752: f64 = (assign32330_e54750 * locals.var_t2__blk599);
        let assign32330_e54753: f64 = (1.0 - assign32330_e54752);
        let assign32330_e54754: f64 = (locals.var_t3__blk600 * assign32330_e54753);
        (assign32330_e54754,)
    } else {
        (locals.var_t5__blk602,)
    }
};
        locals.var_t5__blk602 = assign32330_e54756;
        locals.var_t5__blk602_rv = 0.0;

        let (assign32340_e54768,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32340_e54768;
        locals.var_t2__blk599_rv = 0.0;

        let (assign32350_e54783,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign32350_e54780: f64 = (0.1_f64).ln();
        let assign32350_e54781: f64 = (1.5 - assign32350_e54780);
        (assign32350_e54781,)
    } else {
        (locals.var_t5__blk602,)
    }
};
        locals.var_t5__blk602 = assign32350_e54783;
        locals.var_t5__blk602_rv = 0.0;

        let (assign32360_e54808, assign32360_e54808_d_n3, assign32360_e54808_d_n4, assign32360_e54808_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) {
        let assign32360_e54793: f64 = (locals.var_t1__blk598 - 1.0);
        let assign32360_e54794: f64 = (locals.var_t2__blk599 * assign32360_e54793);
        let assign32360_e54797: f64 = (5.0 * p.p1601);
        let assign32360_e54800: f64 = (locals.var_t1__blk598 - 1.0);
        let assign32360_e54801: f64 = (assign32360_e54797 * assign32360_e54800);
        let assign32360_e54804: f64 = (1.0 + p.p1601);
        let assign32360_e54805: f64 = (assign32360_e54801 + assign32360_e54804);
        let assign32360_e54806: f64 = (assign32360_e54794 * assign32360_e54805);
        (assign32360_e54806, (((locals.var_t2__blk599 * locals.var_t1__blk598_dn3) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn3))), (((locals.var_t2__blk599 * locals.var_t1__blk598_dn4) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn4))), (((locals.var_t2__blk599 * locals.var_t1__blk598_dn5) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn5))),)
    } else {
        (locals.var_t4__blk601, locals.var_t4__blk601_dn3, locals.var_t4__blk601_dn4, locals.var_t4__blk601_dn5,)
    }
};
        locals.var_t4__blk601 = assign32360_e54808;
        locals.var_t4__blk601_dn3 = assign32360_e54808_d_n3;
        locals.var_t4__blk601_dn4 = assign32360_e54808_d_n4;
        locals.var_t4__blk601_dn5 = assign32360_e54808_d_n5;
        locals.var_t4__blk601_rv = 0.0;

        let (assign32370_e54823, assign32370_e54823_d_n3, assign32370_e54823_d_n4, assign32370_e54823_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) {
        let assign32370_e54817: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32370_e54820: f64 = (locals.var_t4__blk601 + locals.var_t5__blk602);
        let assign32370_e54821: f64 = (assign32370_e54817 * assign32370_e54820);
        (assign32370_e54821, (assign32370_e54817 * locals.var_t4__blk601_dn3), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32370_e54820) + (assign32370_e54817 * locals.var_t4__blk601_dn4)), (assign32370_e54817 * locals.var_t4__blk601_dn5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32370_e54823;
        locals.var_qedj3_dn3 = assign32370_e54823_d_n3;
        locals.var_qedj3_dn4 = assign32370_e54823_d_n4;
        locals.var_qedj3_dn5 = assign32370_e54823_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32380_e54830, assign32380_e54830_d_n3, assign32380_e54830_d_n4, assign32380_e54830_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard606 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32380_e54830;
        locals.var_qedj3_dn3 = assign32380_e54830_d_n3;
        locals.var_qedj3_dn4 = assign32380_e54830_d_n4;
        locals.var_qedj3_dn5 = assign32380_e54830_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32390_e54838, assign32390_e54838_d_n3, assign32390_e54838_d_n4, assign32390_e54838_d_n5,) = {
    if (locals.var_guard469 != 0.0) {
        let assign32390_e54834: f64 = (locals.var_qedj1 + locals.var_qedj2);
        let assign32390_e54836: f64 = (assign32390_e54834 + locals.var_qedj3);
        (assign32390_e54836, ((locals.var_qedj1_dn3 + locals.var_qedj2_dn3) + locals.var_qedj3_dn3), ((locals.var_qedj1_dn4 + locals.var_qedj2_dn4) + locals.var_qedj3_dn4), ((locals.var_qedj1_dn5 + locals.var_qedj2_dn5) + locals.var_qedj3_dn5),)
    } else {
        (locals.var_qedj, locals.var_qedj_dn3, locals.var_qedj_dn4, locals.var_qedj_dn5,)
    }
};
        locals.var_qedj = assign32390_e54838;
        locals.var_qedj_dn3 = assign32390_e54838_d_n3;
        locals.var_qedj_dn4 = assign32390_e54838_d_n4;
        locals.var_qedj_dn5 = assign32390_e54838_d_n5;
        locals.var_qedj_rv = 0.0;

        let assign32400_e54842: f64 = (locals.var_csbox * locals.var_ves_jct);
        let assign32400_e54843: f64 = (locals.var_qesj + assign32400_e54842);
        locals.var_qes = assign32400_e54843;
        locals.var_qes_dn0 = (locals.var_csbox_dn0 * locals.var_ves_jct);
        locals.var_qes_dn2 = (locals.var_csbox_dn2 * locals.var_ves_jct);
        locals.var_qes_dn3 = (locals.var_qesj_dn3 + ((locals.var_csbox_dn3 * locals.var_ves_jct) + (locals.var_csbox * locals.var_ves_jct_dn3)));
        locals.var_qes_dn4 = (locals.var_qesj_dn4 + (locals.var_csbox_dn4 * locals.var_ves_jct));
        locals.var_qes_dn5 = (locals.var_csbox_dn5 * locals.var_ves_jct);
        locals.var_qes_dn6 = (locals.var_qesj_dn6 + ((locals.var_csbox_dn6 * locals.var_ves_jct) + (locals.var_csbox * locals.var_ves_jct_dn6)));
        locals.var_qes_dn7 = (locals.var_csbox_dn7 * locals.var_ves_jct);
        locals.var_qes_dn8 = (locals.var_csbox_dn8 * locals.var_ves_jct);
        locals.var_qes_dn9 = (locals.var_csbox_dn9 * locals.var_ves_jct);
        locals.var_qes_dn10 = (locals.var_csbox_dn10 * locals.var_ves_jct);
        locals.var_qes_dn11 = (locals.var_csbox_dn11 * locals.var_ves_jct);
        locals.var_qes_dn13 = (locals.var_csbox_dn13 * locals.var_ves_jct);
        locals.var_qes_dn14 = (locals.var_csbox_dn14 * locals.var_ves_jct);
        locals.var_qes_rv = 0.0;

        let assign32410_e54847: f64 = (locals.var_cdbox * locals.var_ved_jct);
        let assign32410_e54848: f64 = (locals.var_qedj + assign32410_e54847);
        locals.var_qed = assign32410_e54848;
        locals.var_qed_dn0 = (locals.var_cdbox_dn0 * locals.var_ved_jct);
        locals.var_qed_dn2 = (locals.var_cdbox_dn2 * locals.var_ved_jct);
        locals.var_qed_dn3 = (locals.var_qedj_dn3 + ((locals.var_cdbox_dn3 * locals.var_ved_jct) + (locals.var_cdbox * locals.var_ved_jct_dn3)));
        locals.var_qed_dn4 = (locals.var_qedj_dn4 + (locals.var_cdbox_dn4 * locals.var_ved_jct));
        locals.var_qed_dn5 = (locals.var_qedj_dn5 + ((locals.var_cdbox_dn5 * locals.var_ved_jct) + (locals.var_cdbox * locals.var_ved_jct_dn5)));
        locals.var_qed_dn6 = (locals.var_cdbox_dn6 * locals.var_ved_jct);
        locals.var_qed_dn7 = (locals.var_cdbox_dn7 * locals.var_ved_jct);
        locals.var_qed_dn8 = (locals.var_cdbox_dn8 * locals.var_ved_jct);
        locals.var_qed_dn9 = (locals.var_cdbox_dn9 * locals.var_ved_jct);
        locals.var_qed_dn10 = (locals.var_cdbox_dn10 * locals.var_ved_jct);
        locals.var_qed_dn11 = (locals.var_cdbox_dn11 * locals.var_ved_jct);
        locals.var_qed_dn13 = (locals.var_cdbox_dn13 * locals.var_ved_jct);
        locals.var_qed_dn14 = (locals.var_cdbox_dn14 * locals.var_ved_jct);
        locals.var_qed_rv = 0.0;

        let assign32420_e54851: f64 = (locals.var_cgbox * locals.var_devsign);
        let assign32420_e54853: f64 = (assign32420_e54851 * (nv3 - nv10));
        locals.var_qeg = assign32420_e54853;
        locals.var_qeg_dn0 = 0.0;
        locals.var_qeg_dn2 = 0.0;
        locals.var_qeg_dn3 = assign32420_e54851;
        locals.var_qeg_dn4 = 0.0;
        locals.var_qeg_dn5 = 0.0;
        locals.var_qeg_dn6 = 0.0;
        locals.var_qeg_dn7 = 0.0;
        locals.var_qeg_dn8 = 0.0;
        locals.var_qeg_dn9 = 0.0;
        locals.var_qeg_dn10 = (-assign32420_e54851);
        locals.var_qeg_dn11 = 0.0;
        locals.var_qeg_dn13 = 0.0;
        locals.var_qeg_dn14 = 0.0;
        locals.var_qeg_rv = 0.0;

        let assign32430_e54856: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign32430_e54856;
        locals.var_guard620_rv = 0.0;

        let (assign32440_e54862, assign32440_e54862_d_n0, assign32440_e54862_d_n2, assign32440_e54862_d_n3, assign32440_e54862_d_n4, assign32440_e54862_d_n5, assign32440_e54862_d_n6, assign32440_e54862_d_n7, assign32440_e54862_d_n8, assign32440_e54862_d_n9, assign32440_e54862_d_n10, assign32440_e54862_d_n11, assign32440_e54862_d_n13, assign32440_e54862_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32440_e54860: f64 = (locals.var_devsign * (nv10 - nv3));
        (assign32440_e54860, 0.0, 0.0, (-locals.var_devsign), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_devsign, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32440_e54862;
        locals.var_t2_dn0 = assign32440_e54862_d_n0;
        locals.var_t2_dn2 = assign32440_e54862_d_n2;
        locals.var_t2_dn3 = assign32440_e54862_d_n3;
        locals.var_t2_dn4 = assign32440_e54862_d_n4;
        locals.var_t2_dn5 = assign32440_e54862_d_n5;
        locals.var_t2_dn6 = assign32440_e54862_d_n6;
        locals.var_t2_dn7 = assign32440_e54862_d_n7;
        locals.var_t2_dn8 = assign32440_e54862_d_n8;
        locals.var_t2_dn9 = assign32440_e54862_d_n9;
        locals.var_t2_dn10 = assign32440_e54862_d_n10;
        locals.var_t2_dn11 = assign32440_e54862_d_n11;
        locals.var_t2_dn13 = assign32440_e54862_d_n13;
        locals.var_t2_dn14 = assign32440_e54862_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32450_e54876, assign32450_e54876_d_n0, assign32450_e54876_d_n2, assign32450_e54876_d_n3, assign32450_e54876_d_n4, assign32450_e54876_d_n5, assign32450_e54876_d_n6, assign32450_e54876_d_n7, assign32450_e54876_d_n8, assign32450_e54876_d_n9, assign32450_e54876_d_n10, assign32450_e54876_d_n11, assign32450_e54876_d_n13, assign32450_e54876_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32450_e54866: f64 = (locals.var_t2 - locals.var_deltaphi);
        let assign32450_e54869: f64 = (locals.var_eg / 2.0);
        let assign32450_e54870: f64 = (assign32450_e54866 + assign32450_e54869);
        let assign32450_e54872: f64 = (assign32450_e54870 + locals.var_phib);
        let assign32450_e54874: f64 = (assign32450_e54872 - p.p1529);
        (assign32450_e54874, ((locals.var_t2_dn0 - locals.var_deltaphi_dn0) + locals.var_phib_dn0), ((locals.var_t2_dn2 - locals.var_deltaphi_dn2) + locals.var_phib_dn2), ((locals.var_t2_dn3 - locals.var_deltaphi_dn3) + locals.var_phib_dn3), (((locals.var_t2_dn4 - locals.var_deltaphi_dn4) + (locals.var_eg_dn4 / 2.0)) + locals.var_phib_dn4), ((locals.var_t2_dn5 - locals.var_deltaphi_dn5) + locals.var_phib_dn5), ((locals.var_t2_dn6 - locals.var_deltaphi_dn6) + locals.var_phib_dn6), ((locals.var_t2_dn7 - locals.var_deltaphi_dn7) + locals.var_phib_dn7), ((locals.var_t2_dn8 - locals.var_deltaphi_dn8) + locals.var_phib_dn8), ((locals.var_t2_dn9 - locals.var_deltaphi_dn9) + locals.var_phib_dn9), ((locals.var_t2_dn10 - locals.var_deltaphi_dn10) + locals.var_phib_dn10), ((locals.var_t2_dn11 - locals.var_deltaphi_dn11) + locals.var_phib_dn11), ((locals.var_t2_dn13 - locals.var_deltaphi_dn13) + locals.var_phib_dn13), ((locals.var_t2_dn14 - locals.var_deltaphi_dn14) + locals.var_phib_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32450_e54876;
        locals.var_t3_dn0 = assign32450_e54876_d_n0;
        locals.var_t3_dn2 = assign32450_e54876_d_n2;
        locals.var_t3_dn3 = assign32450_e54876_d_n3;
        locals.var_t3_dn4 = assign32450_e54876_d_n4;
        locals.var_t3_dn5 = assign32450_e54876_d_n5;
        locals.var_t3_dn6 = assign32450_e54876_d_n6;
        locals.var_t3_dn7 = assign32450_e54876_d_n7;
        locals.var_t3_dn8 = assign32450_e54876_d_n8;
        locals.var_t3_dn9 = assign32450_e54876_d_n9;
        locals.var_t3_dn10 = assign32450_e54876_d_n10;
        locals.var_t3_dn11 = assign32450_e54876_d_n11;
        locals.var_t3_dn13 = assign32450_e54876_d_n13;
        locals.var_t3_dn14 = assign32450_e54876_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32460_e54882, assign32460_e54882_d_n0, assign32460_e54882_d_n2, assign32460_e54882_d_n3, assign32460_e54882_d_n4, assign32460_e54882_d_n5, assign32460_e54882_d_n6, assign32460_e54882_d_n7, assign32460_e54882_d_n8, assign32460_e54882_d_n9, assign32460_e54882_d_n10, assign32460_e54882_d_n11, assign32460_e54882_d_n13, assign32460_e54882_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32460_e54880: f64 = (locals.var_t3 + 0.02);
        (assign32460_e54880, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32460_e54882;
        locals.var_t0_dn0 = assign32460_e54882_d_n0;
        locals.var_t0_dn2 = assign32460_e54882_d_n2;
        locals.var_t0_dn3 = assign32460_e54882_d_n3;
        locals.var_t0_dn4 = assign32460_e54882_d_n4;
        locals.var_t0_dn5 = assign32460_e54882_d_n5;
        locals.var_t0_dn6 = assign32460_e54882_d_n6;
        locals.var_t0_dn7 = assign32460_e54882_d_n7;
        locals.var_t0_dn8 = assign32460_e54882_d_n8;
        locals.var_t0_dn9 = assign32460_e54882_d_n9;
        locals.var_t0_dn10 = assign32460_e54882_d_n10;
        locals.var_t0_dn11 = assign32460_e54882_d_n11;
        locals.var_t0_dn13 = assign32460_e54882_d_n13;
        locals.var_t0_dn14 = assign32460_e54882_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32470_e54897, assign32470_e54897_d_n0, assign32470_e54897_d_n2, assign32470_e54897_d_n3, assign32470_e54897_d_n4, assign32470_e54897_d_n5, assign32470_e54897_d_n6, assign32470_e54897_d_n7, assign32470_e54897_d_n8, assign32470_e54897_d_n9, assign32470_e54897_d_n10, assign32470_e54897_d_n11, assign32470_e54897_d_n13, assign32470_e54897_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32470_e54888: f64 = (locals.var_t0 * locals.var_t0);
        let assign32470_e54891: f64 = (4.0 * 0.02);
        let assign32470_e54892: f64 = (assign32470_e54888 + assign32470_e54891);
        let assign32470_e54893: f64 = (assign32470_e54892).sqrt();
        let assign32470_e54894: f64 = (locals.var_t0 + assign32470_e54893);
        let assign32470_e54895: f64 = (0.5 * assign32470_e54894);
        (assign32470_e54895, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign32470_e54893)))),)
    } else {
        (locals.var_vge_overlap, locals.var_vge_overlap_dn0, locals.var_vge_overlap_dn2, locals.var_vge_overlap_dn3, locals.var_vge_overlap_dn4, locals.var_vge_overlap_dn5, locals.var_vge_overlap_dn6, locals.var_vge_overlap_dn7, locals.var_vge_overlap_dn8, locals.var_vge_overlap_dn9, locals.var_vge_overlap_dn10, locals.var_vge_overlap_dn11, locals.var_vge_overlap_dn13, locals.var_vge_overlap_dn14,)
    }
};
        locals.var_vge_overlap = assign32470_e54897;
        locals.var_vge_overlap_dn0 = assign32470_e54897_d_n0;
        locals.var_vge_overlap_dn2 = assign32470_e54897_d_n2;
        locals.var_vge_overlap_dn3 = assign32470_e54897_d_n3;
        locals.var_vge_overlap_dn4 = assign32470_e54897_d_n4;
        locals.var_vge_overlap_dn5 = assign32470_e54897_d_n5;
        locals.var_vge_overlap_dn6 = assign32470_e54897_d_n6;
        locals.var_vge_overlap_dn7 = assign32470_e54897_d_n7;
        locals.var_vge_overlap_dn8 = assign32470_e54897_d_n8;
        locals.var_vge_overlap_dn9 = assign32470_e54897_d_n9;
        locals.var_vge_overlap_dn10 = assign32470_e54897_d_n10;
        locals.var_vge_overlap_dn11 = assign32470_e54897_d_n11;
        locals.var_vge_overlap_dn13 = assign32470_e54897_d_n13;
        locals.var_vge_overlap_dn14 = assign32470_e54897_d_n14;
        locals.var_vge_overlap_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_127(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32480_e54926, assign32480_e54926_d_n0, assign32480_e54926_d_n2, assign32480_e54926_d_n3, assign32480_e54926_d_n4, assign32480_e54926_d_n5, assign32480_e54926_d_n6, assign32480_e54926_d_n7, assign32480_e54926_d_n8, assign32480_e54926_d_n9, assign32480_e54926_d_n10, assign32480_e54926_d_n11, assign32480_e54926_d_n13, assign32480_e54926_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32480_e54902: f64 = (locals.var_nfintotal * locals.var_leffcv_1);
        let assign32480_e54906: f64 = (locals.var_t3 - locals.var_vge_overlap);
        let assign32480_e54909: f64 = (0.5 * locals.var_ckappab_i);
        let assign32480_e54913: f64 = (4.0 * locals.var_vge_overlap);
        let assign32480_e54915: f64 = (assign32480_e54913 / locals.var_ckappab_i);
        let assign32480_e54916: f64 = (1.0 + assign32480_e54915);
        let assign32480_e54917: f64 = (assign32480_e54916).sqrt();
        let assign32480_e54919: f64 = (assign32480_e54917 - 1.0);
        let assign32480_e54920: f64 = (assign32480_e54909 * assign32480_e54919);
        let assign32480_e54921: f64 = (assign32480_e54906 + assign32480_e54920);
        let assign32480_e54922: f64 = (locals.var_cgbl_i * assign32480_e54921);
        let assign32480_e54923: f64 = (assign32480_e54902 * assign32480_e54922);
        let assign32480_e54924: f64 = (locals.var_qeg - assign32480_e54923);
        (assign32480_e54924, (locals.var_qeg_dn0 - (((locals.var_nfintotal * locals.var_leffcv_1_dn0) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn0 - locals.var_vge_overlap_dn0) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn0) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn2 - (((locals.var_nfintotal * locals.var_leffcv_1_dn2) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn2 - locals.var_vge_overlap_dn2) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn2) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn3 - (((locals.var_nfintotal * locals.var_leffcv_1_dn3) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn3 - locals.var_vge_overlap_dn3) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn3) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn4 - (((locals.var_nfintotal * locals.var_leffcv_1_dn4) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn4 - locals.var_vge_overlap_dn4) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn4) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn5 - (((locals.var_nfintotal * locals.var_leffcv_1_dn5) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn5 - locals.var_vge_overlap_dn5) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn5) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn6 - (((locals.var_nfintotal * locals.var_leffcv_1_dn6) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn6 - locals.var_vge_overlap_dn6) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn6) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn7 - (((locals.var_nfintotal * locals.var_leffcv_1_dn7) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn7 - locals.var_vge_overlap_dn7) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn7) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn8 - (((locals.var_nfintotal * locals.var_leffcv_1_dn8) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn8 - locals.var_vge_overlap_dn8) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn8) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn9 - (((locals.var_nfintotal * locals.var_leffcv_1_dn9) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn9 - locals.var_vge_overlap_dn9) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn9) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn10 - (((locals.var_nfintotal * locals.var_leffcv_1_dn10) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn10 - locals.var_vge_overlap_dn10) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn10) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn11 - (((locals.var_nfintotal * locals.var_leffcv_1_dn11) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn11 - locals.var_vge_overlap_dn11) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn11) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn13 - (((locals.var_nfintotal * locals.var_leffcv_1_dn13) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn13 - locals.var_vge_overlap_dn13) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn13) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn14 - (((locals.var_nfintotal * locals.var_leffcv_1_dn14) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn14 - locals.var_vge_overlap_dn14) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn14) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))),)
    } else {
        (locals.var_qeg, locals.var_qeg_dn0, locals.var_qeg_dn2, locals.var_qeg_dn3, locals.var_qeg_dn4, locals.var_qeg_dn5, locals.var_qeg_dn6, locals.var_qeg_dn7, locals.var_qeg_dn8, locals.var_qeg_dn9, locals.var_qeg_dn10, locals.var_qeg_dn11, locals.var_qeg_dn13, locals.var_qeg_dn14,)
    }
};
        locals.var_qeg = assign32480_e54926;
        locals.var_qeg_dn0 = assign32480_e54926_d_n0;
        locals.var_qeg_dn2 = assign32480_e54926_d_n2;
        locals.var_qeg_dn3 = assign32480_e54926_d_n3;
        locals.var_qeg_dn4 = assign32480_e54926_d_n4;
        locals.var_qeg_dn5 = assign32480_e54926_d_n5;
        locals.var_qeg_dn6 = assign32480_e54926_d_n6;
        locals.var_qeg_dn7 = assign32480_e54926_d_n7;
        locals.var_qeg_dn8 = assign32480_e54926_d_n8;
        locals.var_qeg_dn9 = assign32480_e54926_d_n9;
        locals.var_qeg_dn10 = assign32480_e54926_d_n10;
        locals.var_qeg_dn11 = assign32480_e54926_d_n11;
        locals.var_qeg_dn13 = assign32480_e54926_d_n13;
        locals.var_qeg_dn14 = assign32480_e54926_d_n14;
        locals.var_qeg_rv = 0.0;

        let assign32490_e54931: f64 = (locals.var_bigen_i * locals.var_vds);
        let assign32490_e54933: f64 = (assign32490_e54931 * locals.var_vds);
        let assign32490_e54934: f64 = (locals.var_aigen_i + assign32490_e54933);
        let assign32490_e54935: f64 = (locals.var_vds * assign32490_e54934);
        locals.var_t1 = assign32490_e54935;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = ((locals.var_vds_dn5 * assign32490_e54934) + (locals.var_vds * (((locals.var_bigen_i * locals.var_vds_dn5) * locals.var_vds) + (assign32490_e54931 * locals.var_vds_dn5))));
        locals.var_t1_dn6 = ((locals.var_vds_dn6 * assign32490_e54934) + (locals.var_vds * (((locals.var_bigen_i * locals.var_vds_dn6) * locals.var_vds) + (assign32490_e54931 * locals.var_vds_dn6))));
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign32510_e54951: f64 = (locals.var_ueff * locals.var_coxeff);
        let assign32510_e54953: f64 = (assign32510_e54951 * locals.var_weff0);
        let assign32510_e54955: f64 = (assign32510_e54953 / locals.var_leff_1);
        locals.var_t0 = assign32510_e54955;
        locals.var_t0_dn0 = ((((((locals.var_ueff_dn0 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn0)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn0)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn2 = ((((((locals.var_ueff_dn2 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn2)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn2)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn3 = ((((((locals.var_ueff_dn3 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn3)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn3)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn4 = ((((((locals.var_ueff_dn4 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn4)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn4)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn5 = ((((((locals.var_ueff_dn5 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn5)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn5)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn6 = ((((((locals.var_ueff_dn6 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn6)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn6)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn7 = ((((((locals.var_ueff_dn7 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn7)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn7)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn8 = ((((((locals.var_ueff_dn8 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn8)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn8)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn9 = ((((((locals.var_ueff_dn9 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn9)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn9)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn10 = ((((((locals.var_ueff_dn10 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn10)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn10)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn11 = ((((((locals.var_ueff_dn11 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn11)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn11)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn13 = ((((((locals.var_ueff_dn13 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn13)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn13)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn14 = ((((((locals.var_ueff_dn14 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn14)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn14)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_rv = 0.0;

        let assign32730_e55066: f64 = (2.0 * locals.var_vsat_a);
        let assign32730_e55068: f64 = (assign32730_e55066 / locals.var_ueff);
        locals.var_esatnoi = assign32730_e55068;
        locals.var_esatnoi_dn0 = ((((2.0 * locals.var_vsat_a_dn0) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn0)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn2 = ((((2.0 * locals.var_vsat_a_dn2) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn2)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn3 = ((((2.0 * locals.var_vsat_a_dn3) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn3)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn4 = ((((2.0 * locals.var_vsat_a_dn4) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn4)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn5 = ((((2.0 * locals.var_vsat_a_dn5) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn5)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn6 = ((((2.0 * locals.var_vsat_a_dn6) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn6)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn7 = ((((2.0 * locals.var_vsat_a_dn7) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn7)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn8 = ((((2.0 * locals.var_vsat_a_dn8) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn8)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn9 = ((((2.0 * locals.var_vsat_a_dn9) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn9)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn10 = ((((2.0 * locals.var_vsat_a_dn10) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn10)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn11 = ((((2.0 * locals.var_vsat_a_dn11) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn11)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn13 = ((((2.0 * locals.var_vsat_a_dn13) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn13)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn14 = ((((2.0 * locals.var_vsat_a_dn14) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn14)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_rv = 0.0;

        let assign32740_e55079: f64 = if (((p.p1682 > 0.0) || (p.p1683 > 0.0)) || (p.p1684 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard624 = assign32740_e55079;
        locals.var_guard624_rv = 0.0;

        let (assign32750_e55087, assign32750_e55087_d_n0, assign32750_e55087_d_n2, assign32750_e55087_d_n3, assign32750_e55087_d_n4, assign32750_e55087_d_n5, assign32750_e55087_d_n6, assign32750_e55087_d_n7, assign32750_e55087_d_n8, assign32750_e55087_d_n9, assign32750_e55087_d_n10, assign32750_e55087_d_n11, assign32750_e55087_d_n13, assign32750_e55087_d_n14,) = {
    if (locals.var_guard624 != 0.0) {
        let assign32750_e55084: f64 = (2.0 * p.p1687);
        let assign32750_e55085: f64 = (locals.var_leff_1 - assign32750_e55084);
        (assign32750_e55085, locals.var_leff_1_dn0, locals.var_leff_1_dn2, locals.var_leff_1_dn3, locals.var_leff_1_dn4, locals.var_leff_1_dn5, locals.var_leff_1_dn6, locals.var_leff_1_dn7, locals.var_leff_1_dn8, locals.var_leff_1_dn9, locals.var_leff_1_dn10, locals.var_leff_1_dn11, locals.var_leff_1_dn13, locals.var_leff_1_dn14,)
    } else {
        (locals.var_leffnoi, locals.var_leffnoi_dn0, locals.var_leffnoi_dn2, locals.var_leffnoi_dn3, locals.var_leffnoi_dn4, locals.var_leffnoi_dn5, locals.var_leffnoi_dn6, locals.var_leffnoi_dn7, locals.var_leffnoi_dn8, locals.var_leffnoi_dn9, locals.var_leffnoi_dn10, locals.var_leffnoi_dn11, locals.var_leffnoi_dn13, locals.var_leffnoi_dn14,)
    }
};
        locals.var_leffnoi = assign32750_e55087;
        locals.var_leffnoi_dn0 = assign32750_e55087_d_n0;
        locals.var_leffnoi_dn2 = assign32750_e55087_d_n2;
        locals.var_leffnoi_dn3 = assign32750_e55087_d_n3;
        locals.var_leffnoi_dn4 = assign32750_e55087_d_n4;
        locals.var_leffnoi_dn5 = assign32750_e55087_d_n5;
        locals.var_leffnoi_dn6 = assign32750_e55087_d_n6;
        locals.var_leffnoi_dn7 = assign32750_e55087_d_n7;
        locals.var_leffnoi_dn8 = assign32750_e55087_d_n8;
        locals.var_leffnoi_dn9 = assign32750_e55087_d_n9;
        locals.var_leffnoi_dn10 = assign32750_e55087_d_n10;
        locals.var_leffnoi_dn11 = assign32750_e55087_d_n11;
        locals.var_leffnoi_dn13 = assign32750_e55087_d_n13;
        locals.var_leffnoi_dn14 = assign32750_e55087_d_n14;
        locals.var_leffnoi_rv = 0.0;

        let assign32760_e55090: f64 = if locals.var_leffnoi <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign32760_e55090;
        locals.var_guard625_rv = 0.0;

        let (assign32770_e55096, assign32770_e55096_d_n0, assign32770_e55096_d_n2, assign32770_e55096_d_n3, assign32770_e55096_d_n4, assign32770_e55096_d_n5, assign32770_e55096_d_n6, assign32770_e55096_d_n7, assign32770_e55096_d_n8, assign32770_e55096_d_n9, assign32770_e55096_d_n10, assign32770_e55096_d_n11, assign32770_e55096_d_n13, assign32770_e55096_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard625 != 0.0)) {
        (locals.var_leff_1, locals.var_leff_1_dn0, locals.var_leff_1_dn2, locals.var_leff_1_dn3, locals.var_leff_1_dn4, locals.var_leff_1_dn5, locals.var_leff_1_dn6, locals.var_leff_1_dn7, locals.var_leff_1_dn8, locals.var_leff_1_dn9, locals.var_leff_1_dn10, locals.var_leff_1_dn11, locals.var_leff_1_dn13, locals.var_leff_1_dn14,)
    } else {
        (locals.var_leffnoi, locals.var_leffnoi_dn0, locals.var_leffnoi_dn2, locals.var_leffnoi_dn3, locals.var_leffnoi_dn4, locals.var_leffnoi_dn5, locals.var_leffnoi_dn6, locals.var_leffnoi_dn7, locals.var_leffnoi_dn8, locals.var_leffnoi_dn9, locals.var_leffnoi_dn10, locals.var_leffnoi_dn11, locals.var_leffnoi_dn13, locals.var_leffnoi_dn14,)
    }
};
        locals.var_leffnoi = assign32770_e55096;
        locals.var_leffnoi_dn0 = assign32770_e55096_d_n0;
        locals.var_leffnoi_dn2 = assign32770_e55096_d_n2;
        locals.var_leffnoi_dn3 = assign32770_e55096_d_n3;
        locals.var_leffnoi_dn4 = assign32770_e55096_d_n4;
        locals.var_leffnoi_dn5 = assign32770_e55096_d_n5;
        locals.var_leffnoi_dn6 = assign32770_e55096_d_n6;
        locals.var_leffnoi_dn7 = assign32770_e55096_d_n7;
        locals.var_leffnoi_dn8 = assign32770_e55096_d_n8;
        locals.var_leffnoi_dn9 = assign32770_e55096_d_n9;
        locals.var_leffnoi_dn10 = assign32770_e55096_d_n10;
        locals.var_leffnoi_dn11 = assign32770_e55096_d_n11;
        locals.var_leffnoi_dn13 = assign32770_e55096_d_n13;
        locals.var_leffnoi_dn14 = assign32770_e55096_d_n14;
        locals.var_leffnoi_rv = 0.0;

        let assign32780_e55103: f64 = if ((p.p79 == 1.0) || (p.p79 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard626 = assign32780_e55103;
        locals.var_guard626_rv = 0.0;

        let (assign32790_e55111, assign32790_e55111_d_n0, assign32790_e55111_d_n2, assign32790_e55111_d_n3, assign32790_e55111_d_n4, assign32790_e55111_d_n5, assign32790_e55111_d_n6, assign32790_e55111_d_n7, assign32790_e55111_d_n8, assign32790_e55111_d_n9, assign32790_e55111_d_n10, assign32790_e55111_d_n11, assign32790_e55111_d_n13, assign32790_e55111_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32790_e55109: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign32790_e55109, ((locals.var_leffnoi_dn0 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn0)), ((locals.var_leffnoi_dn2 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn2)), ((locals.var_leffnoi_dn3 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn3)), ((locals.var_leffnoi_dn4 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn4)), ((locals.var_leffnoi_dn5 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn5)), ((locals.var_leffnoi_dn6 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn6)), ((locals.var_leffnoi_dn7 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn7)), ((locals.var_leffnoi_dn8 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn8)), ((locals.var_leffnoi_dn9 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn9)), ((locals.var_leffnoi_dn10 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn10)), ((locals.var_leffnoi_dn11 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn11)), ((locals.var_leffnoi_dn13 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn13)), ((locals.var_leffnoi_dn14 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn14)),)
    } else {
        (locals.var_leffnoisq, locals.var_leffnoisq_dn0, locals.var_leffnoisq_dn2, locals.var_leffnoisq_dn3, locals.var_leffnoisq_dn4, locals.var_leffnoisq_dn5, locals.var_leffnoisq_dn6, locals.var_leffnoisq_dn7, locals.var_leffnoisq_dn8, locals.var_leffnoisq_dn9, locals.var_leffnoisq_dn10, locals.var_leffnoisq_dn11, locals.var_leffnoisq_dn13, locals.var_leffnoisq_dn14,)
    }
};
        locals.var_leffnoisq = assign32790_e55111;
        locals.var_leffnoisq_dn0 = assign32790_e55111_d_n0;
        locals.var_leffnoisq_dn2 = assign32790_e55111_d_n2;
        locals.var_leffnoisq_dn3 = assign32790_e55111_d_n3;
        locals.var_leffnoisq_dn4 = assign32790_e55111_d_n4;
        locals.var_leffnoisq_dn5 = assign32790_e55111_d_n5;
        locals.var_leffnoisq_dn6 = assign32790_e55111_d_n6;
        locals.var_leffnoisq_dn7 = assign32790_e55111_d_n7;
        locals.var_leffnoisq_dn8 = assign32790_e55111_d_n8;
        locals.var_leffnoisq_dn9 = assign32790_e55111_d_n9;
        locals.var_leffnoisq_dn10 = assign32790_e55111_d_n10;
        locals.var_leffnoisq_dn11 = assign32790_e55111_d_n11;
        locals.var_leffnoisq_dn13 = assign32790_e55111_d_n13;
        locals.var_leffnoisq_dn14 = assign32790_e55111_d_n14;
        locals.var_leffnoisq_rv = 0.0;

        let assign32800_e55114: f64 = if p.p1681 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard627 = assign32800_e55114;
        locals.var_guard627_rv = 0.0;

        let (assign32810_e55128, assign32810_e55128_d_n0, assign32810_e55128_d_n2, assign32810_e55128_d_n3, assign32810_e55128_d_n4, assign32810_e55128_d_n5, assign32810_e55128_d_n6, assign32810_e55128_d_n7, assign32810_e55128_d_n8, assign32810_e55128_d_n9, assign32810_e55128_d_n10, assign32810_e55128_d_n11, assign32810_e55128_d_n13, assign32810_e55128_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign32810_e55122: f64 = (locals.var_diffvds / locals.var_litl);
        let assign32810_e55124: f64 = (assign32810_e55122 + p.p1681);
        let assign32810_e55126: f64 = (assign32810_e55124 / locals.var_esatnoi);
        (assign32810_e55126, ((((locals.var_diffvds_dn0 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn0)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn2 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn2)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn3 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn4 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn5 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn6 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn7 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn8 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn9 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn10 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn11 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn13 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn13)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn14 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn14)) / (locals.var_esatnoi * locals.var_esatnoi)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32810_e55128;
        locals.var_t0_dn0 = assign32810_e55128_d_n0;
        locals.var_t0_dn2 = assign32810_e55128_d_n2;
        locals.var_t0_dn3 = assign32810_e55128_d_n3;
        locals.var_t0_dn4 = assign32810_e55128_d_n4;
        locals.var_t0_dn5 = assign32810_e55128_d_n5;
        locals.var_t0_dn6 = assign32810_e55128_d_n6;
        locals.var_t0_dn7 = assign32810_e55128_d_n7;
        locals.var_t0_dn8 = assign32810_e55128_d_n8;
        locals.var_t0_dn9 = assign32810_e55128_d_n9;
        locals.var_t0_dn10 = assign32810_e55128_d_n10;
        locals.var_t0_dn11 = assign32810_e55128_d_n11;
        locals.var_t0_dn13 = assign32810_e55128_d_n13;
        locals.var_t0_dn14 = assign32810_e55128_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32820_e55151, assign32820_e55151_d_n0, assign32820_e55151_d_n2, assign32820_e55151_d_n3, assign32820_e55151_d_n4, assign32820_e55151_d_n5, assign32820_e55151_d_n6, assign32820_e55151_d_n7, assign32820_e55151_d_n8, assign32820_e55151_d_n9, assign32820_e55151_d_n10, assign32820_e55151_d_n11, assign32820_e55151_d_n13, assign32820_e55151_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let (assign32820_e55148, assign32820_e55148_d_n0, assign32820_e55148_d_n2, assign32820_e55148_d_n3, assign32820_e55148_d_n4, assign32820_e55148_d_n5, assign32820_e55148_d_n6, assign32820_e55148_d_n7, assign32820_e55148_d_n8, assign32820_e55148_d_n9, assign32820_e55148_d_n10, assign32820_e55148_d_n11, assign32820_e55148_d_n13, assign32820_e55148_d_n14,) = {
            if (!(locals.var_t0 > 1e-38)) {
                let assign32820_e55140: f64 = (-87.498233534);
                (assign32820_e55140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (assign32820_e55147, assign32820_e55147_d_n0, assign32820_e55147_d_n2, assign32820_e55147_d_n3, assign32820_e55147_d_n4, assign32820_e55147_d_n5, assign32820_e55147_d_n6, assign32820_e55147_d_n7, assign32820_e55147_d_n8, assign32820_e55147_d_n9, assign32820_e55147_d_n10, assign32820_e55147_d_n11, assign32820_e55147_d_n13, assign32820_e55147_d_n14,) = {
                    if (locals.var_t0 > 1e-38) {
                        let assign32820_e55145: f64 = (locals.var_t0).ln();
                        (assign32820_e55145, (locals.var_t0_dn0 / locals.var_t0), (locals.var_t0_dn2 / locals.var_t0), (locals.var_t0_dn3 / locals.var_t0), (locals.var_t0_dn4 / locals.var_t0), (locals.var_t0_dn5 / locals.var_t0), (locals.var_t0_dn6 / locals.var_t0), (locals.var_t0_dn7 / locals.var_t0), (locals.var_t0_dn8 / locals.var_t0), (locals.var_t0_dn9 / locals.var_t0), (locals.var_t0_dn10 / locals.var_t0), (locals.var_t0_dn11 / locals.var_t0), (locals.var_t0_dn13 / locals.var_t0), (locals.var_t0_dn14 / locals.var_t0),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32820_e55147, assign32820_e55147_d_n0, assign32820_e55147_d_n2, assign32820_e55147_d_n3, assign32820_e55147_d_n4, assign32820_e55147_d_n5, assign32820_e55147_d_n6, assign32820_e55147_d_n7, assign32820_e55147_d_n8, assign32820_e55147_d_n9, assign32820_e55147_d_n10, assign32820_e55147_d_n11, assign32820_e55147_d_n13, assign32820_e55147_d_n14,)
            }
        };
        let assign32820_e55149: f64 = (locals.var_litl * assign32820_e55148);
        (assign32820_e55149, (locals.var_litl * assign32820_e55148_d_n0), (locals.var_litl * assign32820_e55148_d_n2), (locals.var_litl * assign32820_e55148_d_n3), (locals.var_litl * assign32820_e55148_d_n4), (locals.var_litl * assign32820_e55148_d_n5), (locals.var_litl * assign32820_e55148_d_n6), (locals.var_litl * assign32820_e55148_d_n7), (locals.var_litl * assign32820_e55148_d_n8), (locals.var_litl * assign32820_e55148_d_n9), (locals.var_litl * assign32820_e55148_d_n10), (locals.var_litl * assign32820_e55148_d_n11), (locals.var_litl * assign32820_e55148_d_n13), (locals.var_litl * assign32820_e55148_d_n14),)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign32820_e55151;
        locals.var_delclm_dn0 = assign32820_e55151_d_n0;
        locals.var_delclm_dn2 = assign32820_e55151_d_n2;
        locals.var_delclm_dn3 = assign32820_e55151_d_n3;
        locals.var_delclm_dn4 = assign32820_e55151_d_n4;
        locals.var_delclm_dn5 = assign32820_e55151_d_n5;
        locals.var_delclm_dn6 = assign32820_e55151_d_n6;
        locals.var_delclm_dn7 = assign32820_e55151_d_n7;
        locals.var_delclm_dn8 = assign32820_e55151_d_n8;
        locals.var_delclm_dn9 = assign32820_e55151_d_n9;
        locals.var_delclm_dn10 = assign32820_e55151_d_n10;
        locals.var_delclm_dn11 = assign32820_e55151_d_n11;
        locals.var_delclm_dn13 = assign32820_e55151_d_n13;
        locals.var_delclm_dn14 = assign32820_e55151_d_n14;
        locals.var_delclm_rv = 0.0;

        let (assign32830_e55160, assign32830_e55160_d_n0, assign32830_e55160_d_n2, assign32830_e55160_d_n3, assign32830_e55160_d_n4, assign32830_e55160_d_n5, assign32830_e55160_d_n6, assign32830_e55160_d_n7, assign32830_e55160_d_n8, assign32830_e55160_d_n9, assign32830_e55160_d_n10, assign32830_e55160_d_n11, assign32830_e55160_d_n13, assign32830_e55160_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign32830_e55160;
        locals.var_delclm_dn0 = assign32830_e55160_d_n0;
        locals.var_delclm_dn2 = assign32830_e55160_d_n2;
        locals.var_delclm_dn3 = assign32830_e55160_d_n3;
        locals.var_delclm_dn4 = assign32830_e55160_d_n4;
        locals.var_delclm_dn5 = assign32830_e55160_d_n5;
        locals.var_delclm_dn6 = assign32830_e55160_d_n6;
        locals.var_delclm_dn7 = assign32830_e55160_d_n7;
        locals.var_delclm_dn8 = assign32830_e55160_d_n8;
        locals.var_delclm_dn9 = assign32830_e55160_d_n9;
        locals.var_delclm_dn10 = assign32830_e55160_d_n10;
        locals.var_delclm_dn11 = assign32830_e55160_d_n11;
        locals.var_delclm_dn13 = assign32830_e55160_d_n13;
        locals.var_delclm_dn14 = assign32830_e55160_d_n14;
        locals.var_delclm_rv = 0.0;

        let assign32840_e55163: f64 = if p.p79 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign32840_e55163;
        locals.var_guard628_rv = 0.0;

        let (assign32850_e55173, assign32850_e55173_d_n0, assign32850_e55173_d_n2, assign32850_e55173_d_n3, assign32850_e55173_d_n4, assign32850_e55173_d_n5, assign32850_e55173_d_n6, assign32850_e55173_d_n7, assign32850_e55173_d_n8, assign32850_e55173_d_n9, assign32850_e55173_d_n10, assign32850_e55173_d_n11, assign32850_e55173_d_n13, assign32850_e55173_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32850_e55171: f64 = (locals.var_qia2 / locals.var_qsref_i);
        (assign32850_e55171, (locals.var_qia2_dn0 / locals.var_qsref_i), (locals.var_qia2_dn2 / locals.var_qsref_i), (locals.var_qia2_dn3 / locals.var_qsref_i), (locals.var_qia2_dn4 / locals.var_qsref_i), (locals.var_qia2_dn5 / locals.var_qsref_i), (locals.var_qia2_dn6 / locals.var_qsref_i), (locals.var_qia2_dn7 / locals.var_qsref_i), (locals.var_qia2_dn8 / locals.var_qsref_i), (locals.var_qia2_dn9 / locals.var_qsref_i), (locals.var_qia2_dn10 / locals.var_qsref_i), (locals.var_qia2_dn11 / locals.var_qsref_i), (locals.var_qia2_dn13 / locals.var_qsref_i), (locals.var_qia2_dn14 / locals.var_qsref_i),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32850_e55173;
        locals.var_t1_dn0 = assign32850_e55173_d_n0;
        locals.var_t1_dn2 = assign32850_e55173_d_n2;
        locals.var_t1_dn3 = assign32850_e55173_d_n3;
        locals.var_t1_dn4 = assign32850_e55173_d_n4;
        locals.var_t1_dn5 = assign32850_e55173_d_n5;
        locals.var_t1_dn6 = assign32850_e55173_d_n6;
        locals.var_t1_dn7 = assign32850_e55173_d_n7;
        locals.var_t1_dn8 = assign32850_e55173_d_n8;
        locals.var_t1_dn9 = assign32850_e55173_d_n9;
        locals.var_t1_dn10 = assign32850_e55173_d_n10;
        locals.var_t1_dn11 = assign32850_e55173_d_n11;
        locals.var_t1_dn13 = assign32850_e55173_d_n13;
        locals.var_t1_dn14 = assign32850_e55173_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32860_e55185, assign32860_e55185_d_n0, assign32860_e55185_d_n2, assign32860_e55185_d_n3, assign32860_e55185_d_n4, assign32860_e55185_d_n5, assign32860_e55185_d_n6, assign32860_e55185_d_n7, assign32860_e55185_d_n8, assign32860_e55185_d_n9, assign32860_e55185_d_n10, assign32860_e55185_d_n11, assign32860_e55185_d_n13, assign32860_e55185_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32860_e55182: f64 = (locals.var_t1).powf(locals.var_mpower_i);
        let assign32860_e55183: f64 = (1.0 + assign32860_e55182);
        (assign32860_e55183, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn0)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn2)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn3)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn4)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn5)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn6)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn7)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn8)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn9)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn10)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn11)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn13)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn14)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32860_e55185;
        locals.var_t2_dn0 = assign32860_e55185_d_n0;
        locals.var_t2_dn2 = assign32860_e55185_d_n2;
        locals.var_t2_dn3 = assign32860_e55185_d_n3;
        locals.var_t2_dn4 = assign32860_e55185_d_n4;
        locals.var_t2_dn5 = assign32860_e55185_d_n5;
        locals.var_t2_dn6 = assign32860_e55185_d_n6;
        locals.var_t2_dn7 = assign32860_e55185_d_n7;
        locals.var_t2_dn8 = assign32860_e55185_d_n8;
        locals.var_t2_dn9 = assign32860_e55185_d_n9;
        locals.var_t2_dn10 = assign32860_e55185_d_n10;
        locals.var_t2_dn11 = assign32860_e55185_d_n11;
        locals.var_t2_dn13 = assign32860_e55185_d_n13;
        locals.var_t2_dn14 = assign32860_e55185_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32870_e55195, assign32870_e55195_d_n0, assign32870_e55195_d_n2, assign32870_e55195_d_n3, assign32870_e55195_d_n4, assign32870_e55195_d_n5, assign32870_e55195_d_n6, assign32870_e55195_d_n7, assign32870_e55195_d_n8, assign32870_e55195_d_n9, assign32870_e55195_d_n10, assign32870_e55195_d_n11, assign32870_e55195_d_n13, assign32870_e55195_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32870_e55193: f64 = (locals.var_noia2_i / locals.var_t2);
        (assign32870_e55193, (-((locals.var_noia2_i * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32870_e55195;
        locals.var_t3_dn0 = assign32870_e55195_d_n0;
        locals.var_t3_dn2 = assign32870_e55195_d_n2;
        locals.var_t3_dn3 = assign32870_e55195_d_n3;
        locals.var_t3_dn4 = assign32870_e55195_d_n4;
        locals.var_t3_dn5 = assign32870_e55195_d_n5;
        locals.var_t3_dn6 = assign32870_e55195_d_n6;
        locals.var_t3_dn7 = assign32870_e55195_d_n7;
        locals.var_t3_dn8 = assign32870_e55195_d_n8;
        locals.var_t3_dn9 = assign32870_e55195_d_n9;
        locals.var_t3_dn10 = assign32870_e55195_d_n10;
        locals.var_t3_dn11 = assign32870_e55195_d_n11;
        locals.var_t3_dn13 = assign32870_e55195_d_n13;
        locals.var_t3_dn14 = assign32870_e55195_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32880_e55205, assign32880_e55205_d_n0, assign32880_e55205_d_n2, assign32880_e55205_d_n3, assign32880_e55205_d_n4, assign32880_e55205_d_n5, assign32880_e55205_d_n6, assign32880_e55205_d_n7, assign32880_e55205_d_n8, assign32880_e55205_d_n9, assign32880_e55205_d_n10, assign32880_e55205_d_n11, assign32880_e55205_d_n13, assign32880_e55205_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32880_e55203: f64 = (locals.var_t3 / p.p1682);
        (assign32880_e55203, (locals.var_t3_dn0 / p.p1682), (locals.var_t3_dn2 / p.p1682), (locals.var_t3_dn3 / p.p1682), (locals.var_t3_dn4 / p.p1682), (locals.var_t3_dn5 / p.p1682), (locals.var_t3_dn6 / p.p1682), (locals.var_t3_dn7 / p.p1682), (locals.var_t3_dn8 / p.p1682), (locals.var_t3_dn9 / p.p1682), (locals.var_t3_dn10 / p.p1682), (locals.var_t3_dn11 / p.p1682), (locals.var_t3_dn13 / p.p1682), (locals.var_t3_dn14 / p.p1682),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32880_e55205;
        locals.var_t4_dn0 = assign32880_e55205_d_n0;
        locals.var_t4_dn2 = assign32880_e55205_d_n2;
        locals.var_t4_dn3 = assign32880_e55205_d_n3;
        locals.var_t4_dn4 = assign32880_e55205_d_n4;
        locals.var_t4_dn5 = assign32880_e55205_d_n5;
        locals.var_t4_dn6 = assign32880_e55205_d_n6;
        locals.var_t4_dn7 = assign32880_e55205_d_n7;
        locals.var_t4_dn8 = assign32880_e55205_d_n8;
        locals.var_t4_dn9 = assign32880_e55205_d_n9;
        locals.var_t4_dn10 = assign32880_e55205_d_n10;
        locals.var_t4_dn11 = assign32880_e55205_d_n11;
        locals.var_t4_dn13 = assign32880_e55205_d_n13;
        locals.var_t4_dn14 = assign32880_e55205_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32890_e55232, assign32890_e55232_d_n0, assign32890_e55232_d_n2, assign32890_e55232_d_n3, assign32890_e55232_d_n4, assign32890_e55232_d_n5, assign32890_e55232_d_n6, assign32890_e55232_d_n7, assign32890_e55232_d_n8, assign32890_e55232_d_n9, assign32890_e55232_d_n10, assign32890_e55232_d_n11, assign32890_e55232_d_n13, assign32890_e55232_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32890_e55214: f64 = (locals.var_t4 + 1.0);
        let assign32890_e55217: f64 = (locals.var_t4 - 1.0);
        let assign32890_e55220: f64 = (locals.var_t4 - 1.0);
        let assign32890_e55221: f64 = (assign32890_e55217 * assign32890_e55220);
        let assign32890_e55224: f64 = (0.25 * p.p1688);
        let assign32890_e55226: f64 = (assign32890_e55224 * p.p1688);
        let assign32890_e55227: f64 = (assign32890_e55221 + assign32890_e55226);
        let assign32890_e55228: f64 = (assign32890_e55227).sqrt();
        let assign32890_e55229: f64 = (assign32890_e55214 + assign32890_e55228);
        let assign32890_e55230: f64 = (0.5 * assign32890_e55229);
        (assign32890_e55230, (0.5 * (locals.var_t4_dn0 + (((locals.var_t4_dn0 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn0)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn2 + (((locals.var_t4_dn2 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn2)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn3)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn4)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn5)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn6)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn7)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn8)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn9)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn10)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn11)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn13 + (((locals.var_t4_dn13 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn13)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn14 + (((locals.var_t4_dn14 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn14)) / (2.0 * assign32890_e55228)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32890_e55232;
        locals.var_t5_dn0 = assign32890_e55232_d_n0;
        locals.var_t5_dn2 = assign32890_e55232_d_n2;
        locals.var_t5_dn3 = assign32890_e55232_d_n3;
        locals.var_t5_dn4 = assign32890_e55232_d_n4;
        locals.var_t5_dn5 = assign32890_e55232_d_n5;
        locals.var_t5_dn6 = assign32890_e55232_d_n6;
        locals.var_t5_dn7 = assign32890_e55232_d_n7;
        locals.var_t5_dn8 = assign32890_e55232_d_n8;
        locals.var_t5_dn9 = assign32890_e55232_d_n9;
        locals.var_t5_dn10 = assign32890_e55232_d_n10;
        locals.var_t5_dn11 = assign32890_e55232_d_n11;
        locals.var_t5_dn13 = assign32890_e55232_d_n13;
        locals.var_t5_dn14 = assign32890_e55232_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32900_e55242, assign32900_e55242_d_n0, assign32900_e55242_d_n2, assign32900_e55242_d_n3, assign32900_e55242_d_n4, assign32900_e55242_d_n5, assign32900_e55242_d_n6, assign32900_e55242_d_n7, assign32900_e55242_d_n8, assign32900_e55242_d_n9, assign32900_e55242_d_n10, assign32900_e55242_d_n11, assign32900_e55242_d_n13, assign32900_e55242_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32900_e55240: f64 = (p.p1682 * locals.var_t5);
        (assign32900_e55240, (p.p1682 * locals.var_t5_dn0), (p.p1682 * locals.var_t5_dn2), (p.p1682 * locals.var_t5_dn3), (p.p1682 * locals.var_t5_dn4), (p.p1682 * locals.var_t5_dn5), (p.p1682 * locals.var_t5_dn6), (p.p1682 * locals.var_t5_dn7), (p.p1682 * locals.var_t5_dn8), (p.p1682 * locals.var_t5_dn9), (p.p1682 * locals.var_t5_dn10), (p.p1682 * locals.var_t5_dn11), (p.p1682 * locals.var_t5_dn13), (p.p1682 * locals.var_t5_dn14),)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign32900_e55242;
        locals.var_noiaeff_dn0 = assign32900_e55242_d_n0;
        locals.var_noiaeff_dn2 = assign32900_e55242_d_n2;
        locals.var_noiaeff_dn3 = assign32900_e55242_d_n3;
        locals.var_noiaeff_dn4 = assign32900_e55242_d_n4;
        locals.var_noiaeff_dn5 = assign32900_e55242_d_n5;
        locals.var_noiaeff_dn6 = assign32900_e55242_d_n6;
        locals.var_noiaeff_dn7 = assign32900_e55242_d_n7;
        locals.var_noiaeff_dn8 = assign32900_e55242_d_n8;
        locals.var_noiaeff_dn9 = assign32900_e55242_d_n9;
        locals.var_noiaeff_dn10 = assign32900_e55242_d_n10;
        locals.var_noiaeff_dn11 = assign32900_e55242_d_n11;
        locals.var_noiaeff_dn13 = assign32900_e55242_d_n13;
        locals.var_noiaeff_dn14 = assign32900_e55242_d_n14;
        locals.var_noiaeff_rv = 0.0;

        let (assign32910_e55251, assign32910_e55251_d_n0, assign32910_e55251_d_n2, assign32910_e55251_d_n3, assign32910_e55251_d_n4, assign32910_e55251_d_n5, assign32910_e55251_d_n6, assign32910_e55251_d_n7, assign32910_e55251_d_n8, assign32910_e55251_d_n9, assign32910_e55251_d_n10, assign32910_e55251_d_n11, assign32910_e55251_d_n13, assign32910_e55251_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 == 0.0)) {
        (p.p1682, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign32910_e55251;
        locals.var_noiaeff_dn0 = assign32910_e55251_d_n0;
        locals.var_noiaeff_dn2 = assign32910_e55251_d_n2;
        locals.var_noiaeff_dn3 = assign32910_e55251_d_n3;
        locals.var_noiaeff_dn4 = assign32910_e55251_d_n4;
        locals.var_noiaeff_dn5 = assign32910_e55251_d_n5;
        locals.var_noiaeff_dn6 = assign32910_e55251_d_n6;
        locals.var_noiaeff_dn7 = assign32910_e55251_d_n7;
        locals.var_noiaeff_dn8 = assign32910_e55251_d_n8;
        locals.var_noiaeff_dn9 = assign32910_e55251_d_n9;
        locals.var_noiaeff_dn10 = assign32910_e55251_d_n10;
        locals.var_noiaeff_dn11 = assign32910_e55251_d_n11;
        locals.var_noiaeff_dn13 = assign32910_e55251_d_n13;
        locals.var_noiaeff_dn14 = assign32910_e55251_d_n14;
        locals.var_noiaeff_rv = 0.0;

        let (assign32920_e55268, assign32920_e55268_d_n0, assign32920_e55268_d_n2, assign32920_e55268_d_n3, assign32920_e55268_d_n4, assign32920_e55268_d_n5, assign32920_e55268_d_n6, assign32920_e55268_d_n7, assign32920_e55268_d_n8, assign32920_e55268_d_n9, assign32920_e55268_d_n10, assign32920_e55268_d_n11, assign32920_e55268_d_n13, assign32920_e55268_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32920_e55257: f64 = (1.60219e-19 * 1.60219e-19);
        let assign32920_e55259: f64 = (assign32920_e55257 * 1.60219e-19);
        let assign32920_e55261: f64 = (assign32920_e55259 * locals.var_vtm);
        let assign32920_e55263: f64 = (locals.var_ids_v).abs();
        let assign32920_e55264: f64 = (assign32920_e55261 * assign32920_e55263);
        let assign32920_e55266: f64 = (assign32920_e55264 * locals.var_ueff);
        (assign32920_e55266, (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn0 } else { (-locals.var_ids_v_dn0) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn0)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn2 } else { (-locals.var_ids_v_dn2) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn2)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn3 } else { (-locals.var_ids_v_dn3) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn3)), (((((assign32920_e55259 * locals.var_vtm_dn4) * assign32920_e55263) + (assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn4 } else { (-locals.var_ids_v_dn4) })) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn4)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn5 } else { (-locals.var_ids_v_dn5) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn5)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn6 } else { (-locals.var_ids_v_dn6) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn6)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn7 } else { (-locals.var_ids_v_dn7) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn7)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn8 } else { (-locals.var_ids_v_dn8) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn8)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn9 } else { (-locals.var_ids_v_dn9) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn9)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn10 } else { (-locals.var_ids_v_dn10) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn10)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn11 } else { (-locals.var_ids_v_dn11) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn11)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn13 } else { (-locals.var_ids_v_dn13) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn13)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn14 } else { (-locals.var_ids_v_dn14) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e55268;
        locals.var_t1_dn0 = assign32920_e55268_d_n0;
        locals.var_t1_dn2 = assign32920_e55268_d_n2;
        locals.var_t1_dn3 = assign32920_e55268_d_n3;
        locals.var_t1_dn4 = assign32920_e55268_d_n4;
        locals.var_t1_dn5 = assign32920_e55268_d_n5;
        locals.var_t1_dn6 = assign32920_e55268_d_n6;
        locals.var_t1_dn7 = assign32920_e55268_d_n7;
        locals.var_t1_dn8 = assign32920_e55268_d_n8;
        locals.var_t1_dn9 = assign32920_e55268_d_n9;
        locals.var_t1_dn10 = assign32920_e55268_d_n10;
        locals.var_t1_dn11 = assign32920_e55268_d_n11;
        locals.var_t1_dn13 = assign32920_e55268_d_n13;
        locals.var_t1_dn14 = assign32920_e55268_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32930_e55278, assign32930_e55278_d_n0, assign32930_e55278_d_n2, assign32930_e55278_d_n3, assign32930_e55278_d_n4, assign32930_e55278_d_n5, assign32930_e55278_d_n6, assign32930_e55278_d_n7, assign32930_e55278_d_n8, assign32930_e55278_d_n9, assign32930_e55278_d_n10, assign32930_e55278_d_n11, assign32930_e55278_d_n13, assign32930_e55278_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32930_e55274: f64 = (10000000000.0 * locals.var_coxeff);
        let assign32930_e55276: f64 = (assign32930_e55274 * locals.var_leffnoisq);
        (assign32930_e55276, (((10000000000.0 * locals.var_coxeff_dn0) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn0)), (((10000000000.0 * locals.var_coxeff_dn2) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn2)), (((10000000000.0 * locals.var_coxeff_dn3) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn3)), (((10000000000.0 * locals.var_coxeff_dn4) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn4)), (((10000000000.0 * locals.var_coxeff_dn5) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn5)), (((10000000000.0 * locals.var_coxeff_dn6) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn6)), (((10000000000.0 * locals.var_coxeff_dn7) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn7)), (((10000000000.0 * locals.var_coxeff_dn8) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn8)), (((10000000000.0 * locals.var_coxeff_dn9) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn9)), (((10000000000.0 * locals.var_coxeff_dn10) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn10)), (((10000000000.0 * locals.var_coxeff_dn11) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn11)), (((10000000000.0 * locals.var_coxeff_dn13) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn13)), (((10000000000.0 * locals.var_coxeff_dn14) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32930_e55278;
        locals.var_t2_dn0 = assign32930_e55278_d_n0;
        locals.var_t2_dn2 = assign32930_e55278_d_n2;
        locals.var_t2_dn3 = assign32930_e55278_d_n3;
        locals.var_t2_dn4 = assign32930_e55278_d_n4;
        locals.var_t2_dn5 = assign32930_e55278_d_n5;
        locals.var_t2_dn6 = assign32930_e55278_d_n6;
        locals.var_t2_dn7 = assign32930_e55278_d_n7;
        locals.var_t2_dn8 = assign32930_e55278_d_n8;
        locals.var_t2_dn9 = assign32930_e55278_d_n9;
        locals.var_t2_dn10 = assign32930_e55278_d_n10;
        locals.var_t2_dn11 = assign32930_e55278_d_n11;
        locals.var_t2_dn13 = assign32930_e55278_d_n13;
        locals.var_t2_dn14 = assign32930_e55278_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_128(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32940_e55288, assign32940_e55288_d_n0, assign32940_e55288_d_n2, assign32940_e55288_d_n3, assign32940_e55288_d_n4, assign32940_e55288_d_n5, assign32940_e55288_d_n6, assign32940_e55288_d_n7, assign32940_e55288_d_n8, assign32940_e55288_d_n9, assign32940_e55288_d_n10, assign32940_e55288_d_n11, assign32940_e55288_d_n13, assign32940_e55288_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32940_e55284: f64 = (locals.var_coxeff * locals.var_qis);
        let assign32940_e55286: f64 = (assign32940_e55284 / 1.60219e-19);
        (assign32940_e55286, (((locals.var_coxeff_dn0 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn0)) / 1.60219e-19), (((locals.var_coxeff_dn2 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn2)) / 1.60219e-19), (((locals.var_coxeff_dn3 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn8)) / 1.60219e-19), (((locals.var_coxeff_dn9 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn9)) / 1.60219e-19), (((locals.var_coxeff_dn10 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn10)) / 1.60219e-19), (((locals.var_coxeff_dn11 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn11)) / 1.60219e-19), (((locals.var_coxeff_dn13 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn13)) / 1.60219e-19), (((locals.var_coxeff_dn14 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn0, locals.var_n0_dn2, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn13, locals.var_n0_dn14,)
    }
};
        locals.var_n0 = assign32940_e55288;
        locals.var_n0_dn0 = assign32940_e55288_d_n0;
        locals.var_n0_dn2 = assign32940_e55288_d_n2;
        locals.var_n0_dn3 = assign32940_e55288_d_n3;
        locals.var_n0_dn4 = assign32940_e55288_d_n4;
        locals.var_n0_dn5 = assign32940_e55288_d_n5;
        locals.var_n0_dn6 = assign32940_e55288_d_n6;
        locals.var_n0_dn7 = assign32940_e55288_d_n7;
        locals.var_n0_dn8 = assign32940_e55288_d_n8;
        locals.var_n0_dn9 = assign32940_e55288_d_n9;
        locals.var_n0_dn10 = assign32940_e55288_d_n10;
        locals.var_n0_dn11 = assign32940_e55288_d_n11;
        locals.var_n0_dn13 = assign32940_e55288_d_n13;
        locals.var_n0_dn14 = assign32940_e55288_d_n14;
        locals.var_n0_rv = 0.0;

        let (assign32950_e55298, assign32950_e55298_d_n0, assign32950_e55298_d_n2, assign32950_e55298_d_n3, assign32950_e55298_d_n4, assign32950_e55298_d_n5, assign32950_e55298_d_n6, assign32950_e55298_d_n7, assign32950_e55298_d_n8, assign32950_e55298_d_n9, assign32950_e55298_d_n10, assign32950_e55298_d_n11, assign32950_e55298_d_n13, assign32950_e55298_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32950_e55294: f64 = (locals.var_coxeff * locals.var_qid);
        let assign32950_e55296: f64 = (assign32950_e55294 / 1.60219e-19);
        (assign32950_e55296, (((locals.var_coxeff_dn0 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn0)) / 1.60219e-19), (((locals.var_coxeff_dn2 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn2)) / 1.60219e-19), (((locals.var_coxeff_dn3 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn8)) / 1.60219e-19), (((locals.var_coxeff_dn9 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn9)) / 1.60219e-19), (((locals.var_coxeff_dn10 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn10)) / 1.60219e-19), (((locals.var_coxeff_dn11 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn11)) / 1.60219e-19), (((locals.var_coxeff_dn13 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn13)) / 1.60219e-19), (((locals.var_coxeff_dn14 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn0, locals.var_nl_dn2, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11, locals.var_nl_dn13, locals.var_nl_dn14,)
    }
};
        locals.var_nl = assign32950_e55298;
        locals.var_nl_dn0 = assign32950_e55298_d_n0;
        locals.var_nl_dn2 = assign32950_e55298_d_n2;
        locals.var_nl_dn3 = assign32950_e55298_d_n3;
        locals.var_nl_dn4 = assign32950_e55298_d_n4;
        locals.var_nl_dn5 = assign32950_e55298_d_n5;
        locals.var_nl_dn6 = assign32950_e55298_d_n6;
        locals.var_nl_dn7 = assign32950_e55298_d_n7;
        locals.var_nl_dn8 = assign32950_e55298_d_n8;
        locals.var_nl_dn9 = assign32950_e55298_d_n9;
        locals.var_nl_dn10 = assign32950_e55298_d_n10;
        locals.var_nl_dn11 = assign32950_e55298_d_n11;
        locals.var_nl_dn13 = assign32950_e55298_d_n13;
        locals.var_nl_dn14 = assign32950_e55298_d_n14;
        locals.var_nl_rv = 0.0;

        let (assign32960_e55310, assign32960_e55310_d_n0, assign32960_e55310_d_n2, assign32960_e55310_d_n3, assign32960_e55310_d_n4, assign32960_e55310_d_n5, assign32960_e55310_d_n6, assign32960_e55310_d_n7, assign32960_e55310_d_n8, assign32960_e55310_d_n9, assign32960_e55310_d_n10, assign32960_e55310_d_n11, assign32960_e55310_d_n13, assign32960_e55310_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32960_e55304: f64 = (locals.var_vtm / 1.60219e-19);
        let assign32960_e55307: f64 = (locals.var_coxeff + locals.var_cit_a);
        let assign32960_e55308: f64 = (assign32960_e55304 * assign32960_e55307);
        (assign32960_e55308, (assign32960_e55304 * (locals.var_coxeff_dn0 + locals.var_cit_a_dn0)), (assign32960_e55304 * (locals.var_coxeff_dn2 + locals.var_cit_a_dn2)), (assign32960_e55304 * (locals.var_coxeff_dn3 + locals.var_cit_a_dn3)), (((locals.var_vtm_dn4 / 1.60219e-19) * assign32960_e55307) + (assign32960_e55304 * (locals.var_coxeff_dn4 + locals.var_cit_a_dn4))), (assign32960_e55304 * (locals.var_coxeff_dn5 + locals.var_cit_a_dn5)), (assign32960_e55304 * (locals.var_coxeff_dn6 + locals.var_cit_a_dn6)), (assign32960_e55304 * (locals.var_coxeff_dn7 + locals.var_cit_a_dn7)), (assign32960_e55304 * (locals.var_coxeff_dn8 + locals.var_cit_a_dn8)), (assign32960_e55304 * (locals.var_coxeff_dn9 + locals.var_cit_a_dn9)), (assign32960_e55304 * (locals.var_coxeff_dn10 + locals.var_cit_a_dn10)), (assign32960_e55304 * (locals.var_coxeff_dn11 + locals.var_cit_a_dn11)), (assign32960_e55304 * (locals.var_coxeff_dn13 + locals.var_cit_a_dn13)), (assign32960_e55304 * (locals.var_coxeff_dn14 + locals.var_cit_a_dn14)),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn0, locals.var_nstar_dn2, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11, locals.var_nstar_dn13, locals.var_nstar_dn14,)
    }
};
        locals.var_nstar = assign32960_e55310;
        locals.var_nstar_dn0 = assign32960_e55310_d_n0;
        locals.var_nstar_dn2 = assign32960_e55310_d_n2;
        locals.var_nstar_dn3 = assign32960_e55310_d_n3;
        locals.var_nstar_dn4 = assign32960_e55310_d_n4;
        locals.var_nstar_dn5 = assign32960_e55310_d_n5;
        locals.var_nstar_dn6 = assign32960_e55310_d_n6;
        locals.var_nstar_dn7 = assign32960_e55310_d_n7;
        locals.var_nstar_dn8 = assign32960_e55310_d_n8;
        locals.var_nstar_dn9 = assign32960_e55310_d_n9;
        locals.var_nstar_dn10 = assign32960_e55310_d_n10;
        locals.var_nstar_dn11 = assign32960_e55310_d_n11;
        locals.var_nstar_dn13 = assign32960_e55310_d_n13;
        locals.var_nstar_dn14 = assign32960_e55310_d_n14;
        locals.var_nstar_rv = 0.0;

        let (assign32970_e55349, assign32970_e55349_d_n0, assign32970_e55349_d_n2, assign32970_e55349_d_n3, assign32970_e55349_d_n4, assign32970_e55349_d_n5, assign32970_e55349_d_n6, assign32970_e55349_d_n7, assign32970_e55349_d_n8, assign32970_e55349_d_n9, assign32970_e55349_d_n10, assign32970_e55349_d_n11, assign32970_e55349_d_n13, assign32970_e55349_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32970_e55317: f64 = (locals.var_n0 + locals.var_nstar);
        let assign32970_e55320: f64 = (locals.var_nl + locals.var_nstar);
        let assign32970_e55321: f64 = (assign32970_e55317 / assign32970_e55320);
        let (assign32970_e55346, assign32970_e55346_d_n0, assign32970_e55346_d_n2, assign32970_e55346_d_n3, assign32970_e55346_d_n4, assign32970_e55346_d_n5, assign32970_e55346_d_n6, assign32970_e55346_d_n7, assign32970_e55346_d_n8, assign32970_e55346_d_n9, assign32970_e55346_d_n10, assign32970_e55346_d_n11, assign32970_e55346_d_n13, assign32970_e55346_d_n14,) = {
            if (!(assign32970_e55321 > 1e-38)) {
                let assign32970_e55326: f64 = (-87.498233534);
                (assign32970_e55326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign32970_e55329: f64 = (locals.var_n0 + locals.var_nstar);
                let assign32970_e55332: f64 = (locals.var_nl + locals.var_nstar);
                let assign32970_e55333: f64 = (assign32970_e55329 / assign32970_e55332);
                let (assign32970_e55345, assign32970_e55345_d_n0, assign32970_e55345_d_n2, assign32970_e55345_d_n3, assign32970_e55345_d_n4, assign32970_e55345_d_n5, assign32970_e55345_d_n6, assign32970_e55345_d_n7, assign32970_e55345_d_n8, assign32970_e55345_d_n9, assign32970_e55345_d_n10, assign32970_e55345_d_n11, assign32970_e55345_d_n13, assign32970_e55345_d_n14,) = {
                    if (assign32970_e55333 > 1e-38) {
                        let assign32970_e55338: f64 = (locals.var_n0 + locals.var_nstar);
                        let assign32970_e55341: f64 = (locals.var_nl + locals.var_nstar);
                        let assign32970_e55342: f64 = (assign32970_e55338 / assign32970_e55341);
                        let assign32970_e55343: f64 = (assign32970_e55342).ln();
                        (assign32970_e55343, (((((locals.var_n0_dn0 + locals.var_nstar_dn0) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn0 + locals.var_nstar_dn0))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn2 + locals.var_nstar_dn2) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn2 + locals.var_nstar_dn2))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn13 + locals.var_nstar_dn13) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn13 + locals.var_nstar_dn13))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn14 + locals.var_nstar_dn14) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn14 + locals.var_nstar_dn14))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32970_e55345, assign32970_e55345_d_n0, assign32970_e55345_d_n2, assign32970_e55345_d_n3, assign32970_e55345_d_n4, assign32970_e55345_d_n5, assign32970_e55345_d_n6, assign32970_e55345_d_n7, assign32970_e55345_d_n8, assign32970_e55345_d_n9, assign32970_e55345_d_n10, assign32970_e55345_d_n11, assign32970_e55345_d_n13, assign32970_e55345_d_n14,)
            }
        };
        let assign32970_e55347: f64 = (locals.var_noiaeff * assign32970_e55346);
        (assign32970_e55347, ((locals.var_noiaeff_dn0 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n0)), ((locals.var_noiaeff_dn2 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n2)), ((locals.var_noiaeff_dn3 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n3)), ((locals.var_noiaeff_dn4 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n4)), ((locals.var_noiaeff_dn5 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n5)), ((locals.var_noiaeff_dn6 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n6)), ((locals.var_noiaeff_dn7 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n7)), ((locals.var_noiaeff_dn8 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n8)), ((locals.var_noiaeff_dn9 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n9)), ((locals.var_noiaeff_dn10 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n10)), ((locals.var_noiaeff_dn11 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n11)), ((locals.var_noiaeff_dn13 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n13)), ((locals.var_noiaeff_dn14 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32970_e55349;
        locals.var_t3_dn0 = assign32970_e55349_d_n0;
        locals.var_t3_dn2 = assign32970_e55349_d_n2;
        locals.var_t3_dn3 = assign32970_e55349_d_n3;
        locals.var_t3_dn4 = assign32970_e55349_d_n4;
        locals.var_t3_dn5 = assign32970_e55349_d_n5;
        locals.var_t3_dn6 = assign32970_e55349_d_n6;
        locals.var_t3_dn7 = assign32970_e55349_d_n7;
        locals.var_t3_dn8 = assign32970_e55349_d_n8;
        locals.var_t3_dn9 = assign32970_e55349_d_n9;
        locals.var_t3_dn10 = assign32970_e55349_d_n10;
        locals.var_t3_dn11 = assign32970_e55349_d_n11;
        locals.var_t3_dn13 = assign32970_e55349_d_n13;
        locals.var_t3_dn14 = assign32970_e55349_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32980_e55359, assign32980_e55359_d_n0, assign32980_e55359_d_n2, assign32980_e55359_d_n3, assign32980_e55359_d_n4, assign32980_e55359_d_n5, assign32980_e55359_d_n6, assign32980_e55359_d_n7, assign32980_e55359_d_n8, assign32980_e55359_d_n9, assign32980_e55359_d_n10, assign32980_e55359_d_n11, assign32980_e55359_d_n13, assign32980_e55359_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32980_e55356: f64 = (locals.var_n0 - locals.var_nl);
        let assign32980_e55357: f64 = (p.p1683 * assign32980_e55356);
        (assign32980_e55357, (p.p1683 * (locals.var_n0_dn0 - locals.var_nl_dn0)), (p.p1683 * (locals.var_n0_dn2 - locals.var_nl_dn2)), (p.p1683 * (locals.var_n0_dn3 - locals.var_nl_dn3)), (p.p1683 * (locals.var_n0_dn4 - locals.var_nl_dn4)), (p.p1683 * (locals.var_n0_dn5 - locals.var_nl_dn5)), (p.p1683 * (locals.var_n0_dn6 - locals.var_nl_dn6)), (p.p1683 * (locals.var_n0_dn7 - locals.var_nl_dn7)), (p.p1683 * (locals.var_n0_dn8 - locals.var_nl_dn8)), (p.p1683 * (locals.var_n0_dn9 - locals.var_nl_dn9)), (p.p1683 * (locals.var_n0_dn10 - locals.var_nl_dn10)), (p.p1683 * (locals.var_n0_dn11 - locals.var_nl_dn11)), (p.p1683 * (locals.var_n0_dn13 - locals.var_nl_dn13)), (p.p1683 * (locals.var_n0_dn14 - locals.var_nl_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32980_e55359;
        locals.var_t4_dn0 = assign32980_e55359_d_n0;
        locals.var_t4_dn2 = assign32980_e55359_d_n2;
        locals.var_t4_dn3 = assign32980_e55359_d_n3;
        locals.var_t4_dn4 = assign32980_e55359_d_n4;
        locals.var_t4_dn5 = assign32980_e55359_d_n5;
        locals.var_t4_dn6 = assign32980_e55359_d_n6;
        locals.var_t4_dn7 = assign32980_e55359_d_n7;
        locals.var_t4_dn8 = assign32980_e55359_d_n8;
        locals.var_t4_dn9 = assign32980_e55359_d_n9;
        locals.var_t4_dn10 = assign32980_e55359_d_n10;
        locals.var_t4_dn11 = assign32980_e55359_d_n11;
        locals.var_t4_dn13 = assign32980_e55359_d_n13;
        locals.var_t4_dn14 = assign32980_e55359_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32990_e55375, assign32990_e55375_d_n0, assign32990_e55375_d_n2, assign32990_e55375_d_n3, assign32990_e55375_d_n4, assign32990_e55375_d_n5, assign32990_e55375_d_n6, assign32990_e55375_d_n7, assign32990_e55375_d_n8, assign32990_e55375_d_n9, assign32990_e55375_d_n10, assign32990_e55375_d_n11, assign32990_e55375_d_n13, assign32990_e55375_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32990_e55365: f64 = (0.5 * p.p1684);
        let assign32990_e55368: f64 = (locals.var_n0 * locals.var_n0);
        let assign32990_e55371: f64 = (locals.var_nl * locals.var_nl);
        let assign32990_e55372: f64 = (assign32990_e55368 - assign32990_e55371);
        let assign32990_e55373: f64 = (assign32990_e55365 * assign32990_e55372);
        (assign32990_e55373, (assign32990_e55365 * (((locals.var_n0_dn0 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn0)) - ((locals.var_nl_dn0 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn0)))), (assign32990_e55365 * (((locals.var_n0_dn2 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn2)) - ((locals.var_nl_dn2 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn2)))), (assign32990_e55365 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign32990_e55365 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign32990_e55365 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign32990_e55365 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign32990_e55365 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign32990_e55365 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign32990_e55365 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign32990_e55365 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign32990_e55365 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))), (assign32990_e55365 * (((locals.var_n0_dn13 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn13)) - ((locals.var_nl_dn13 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn13)))), (assign32990_e55365 * (((locals.var_n0_dn14 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn14)) - ((locals.var_nl_dn14 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn14)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32990_e55375;
        locals.var_t5_dn0 = assign32990_e55375_d_n0;
        locals.var_t5_dn2 = assign32990_e55375_d_n2;
        locals.var_t5_dn3 = assign32990_e55375_d_n3;
        locals.var_t5_dn4 = assign32990_e55375_d_n4;
        locals.var_t5_dn5 = assign32990_e55375_d_n5;
        locals.var_t5_dn6 = assign32990_e55375_d_n6;
        locals.var_t5_dn7 = assign32990_e55375_d_n7;
        locals.var_t5_dn8 = assign32990_e55375_d_n8;
        locals.var_t5_dn9 = assign32990_e55375_d_n9;
        locals.var_t5_dn10 = assign32990_e55375_d_n10;
        locals.var_t5_dn11 = assign32990_e55375_d_n11;
        locals.var_t5_dn13 = assign32990_e55375_d_n13;
        locals.var_t5_dn14 = assign32990_e55375_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33000_e55387, assign33000_e55387_d_n0, assign33000_e55387_d_n2, assign33000_e55387_d_n3, assign33000_e55387_d_n4, assign33000_e55387_d_n5, assign33000_e55387_d_n6, assign33000_e55387_d_n7, assign33000_e55387_d_n8, assign33000_e55387_d_n9, assign33000_e55387_d_n10, assign33000_e55387_d_n11, assign33000_e55387_d_n13, assign33000_e55387_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33000_e55381: f64 = (1.60219e-19 * locals.var_vtm);
        let assign33000_e55383: f64 = (assign33000_e55381 * locals.var_ids_v);
        let assign33000_e55385: f64 = (assign33000_e55383 * locals.var_ids_v);
        (assign33000_e55385, (((assign33000_e55381 * locals.var_ids_v_dn0) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn0)), (((assign33000_e55381 * locals.var_ids_v_dn2) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn2)), (((assign33000_e55381 * locals.var_ids_v_dn3) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn3)), (((((1.60219e-19 * locals.var_vtm_dn4) * locals.var_ids_v) + (assign33000_e55381 * locals.var_ids_v_dn4)) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn4)), (((assign33000_e55381 * locals.var_ids_v_dn5) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn5)), (((assign33000_e55381 * locals.var_ids_v_dn6) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn6)), (((assign33000_e55381 * locals.var_ids_v_dn7) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn7)), (((assign33000_e55381 * locals.var_ids_v_dn8) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn8)), (((assign33000_e55381 * locals.var_ids_v_dn9) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn9)), (((assign33000_e55381 * locals.var_ids_v_dn10) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn10)), (((assign33000_e55381 * locals.var_ids_v_dn11) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn11)), (((assign33000_e55381 * locals.var_ids_v_dn13) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn13)), (((assign33000_e55381 * locals.var_ids_v_dn14) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33000_e55387;
        locals.var_t6_dn0 = assign33000_e55387_d_n0;
        locals.var_t6_dn2 = assign33000_e55387_d_n2;
        locals.var_t6_dn3 = assign33000_e55387_d_n3;
        locals.var_t6_dn4 = assign33000_e55387_d_n4;
        locals.var_t6_dn5 = assign33000_e55387_d_n5;
        locals.var_t6_dn6 = assign33000_e55387_d_n6;
        locals.var_t6_dn7 = assign33000_e55387_d_n7;
        locals.var_t6_dn8 = assign33000_e55387_d_n8;
        locals.var_t6_dn9 = assign33000_e55387_d_n9;
        locals.var_t6_dn10 = assign33000_e55387_d_n10;
        locals.var_t6_dn11 = assign33000_e55387_d_n11;
        locals.var_t6_dn13 = assign33000_e55387_d_n13;
        locals.var_t6_dn14 = assign33000_e55387_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign33010_e55399, assign33010_e55399_d_n0, assign33010_e55399_d_n2, assign33010_e55399_d_n3, assign33010_e55399_d_n4, assign33010_e55399_d_n5, assign33010_e55399_d_n6, assign33010_e55399_d_n7, assign33010_e55399_d_n8, assign33010_e55399_d_n9, assign33010_e55399_d_n10, assign33010_e55399_d_n11, assign33010_e55399_d_n13, assign33010_e55399_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33010_e55393: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign33010_e55395: f64 = (assign33010_e55393 * locals.var_weff0);
        let assign33010_e55397: f64 = (assign33010_e55395 * locals.var_nfintotal);
        (assign33010_e55397, (((10000000000.0 * locals.var_leffnoisq_dn0) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn2) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn3) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn4) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn5) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn6) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn7) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn8) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn9) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn10) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn11) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn13) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn14) * locals.var_weff0) * locals.var_nfintotal),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33010_e55399;
        locals.var_t7_dn0 = assign33010_e55399_d_n0;
        locals.var_t7_dn2 = assign33010_e55399_d_n2;
        locals.var_t7_dn3 = assign33010_e55399_d_n3;
        locals.var_t7_dn4 = assign33010_e55399_d_n4;
        locals.var_t7_dn5 = assign33010_e55399_d_n5;
        locals.var_t7_dn6 = assign33010_e55399_d_n6;
        locals.var_t7_dn7 = assign33010_e55399_d_n7;
        locals.var_t7_dn8 = assign33010_e55399_d_n8;
        locals.var_t7_dn9 = assign33010_e55399_d_n9;
        locals.var_t7_dn10 = assign33010_e55399_d_n10;
        locals.var_t7_dn11 = assign33010_e55399_d_n11;
        locals.var_t7_dn13 = assign33010_e55399_d_n13;
        locals.var_t7_dn14 = assign33010_e55399_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign33020_e55415, assign33020_e55415_d_n0, assign33020_e55415_d_n2, assign33020_e55415_d_n3, assign33020_e55415_d_n4, assign33020_e55415_d_n5, assign33020_e55415_d_n6, assign33020_e55415_d_n7, assign33020_e55415_d_n8, assign33020_e55415_d_n9, assign33020_e55415_d_n10, assign33020_e55415_d_n11, assign33020_e55415_d_n13, assign33020_e55415_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33020_e55406: f64 = (p.p1683 * locals.var_nl);
        let assign33020_e55407: f64 = (locals.var_noiaeff + assign33020_e55406);
        let assign33020_e55410: f64 = (p.p1684 * locals.var_nl);
        let assign33020_e55412: f64 = (assign33020_e55410 * locals.var_nl);
        let assign33020_e55413: f64 = (assign33020_e55407 + assign33020_e55412);
        (assign33020_e55413, ((locals.var_noiaeff_dn0 + (p.p1683 * locals.var_nl_dn0)) + (((p.p1684 * locals.var_nl_dn0) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn0))), ((locals.var_noiaeff_dn2 + (p.p1683 * locals.var_nl_dn2)) + (((p.p1684 * locals.var_nl_dn2) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn2))), ((locals.var_noiaeff_dn3 + (p.p1683 * locals.var_nl_dn3)) + (((p.p1684 * locals.var_nl_dn3) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn3))), ((locals.var_noiaeff_dn4 + (p.p1683 * locals.var_nl_dn4)) + (((p.p1684 * locals.var_nl_dn4) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn4))), ((locals.var_noiaeff_dn5 + (p.p1683 * locals.var_nl_dn5)) + (((p.p1684 * locals.var_nl_dn5) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn5))), ((locals.var_noiaeff_dn6 + (p.p1683 * locals.var_nl_dn6)) + (((p.p1684 * locals.var_nl_dn6) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn6))), ((locals.var_noiaeff_dn7 + (p.p1683 * locals.var_nl_dn7)) + (((p.p1684 * locals.var_nl_dn7) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn7))), ((locals.var_noiaeff_dn8 + (p.p1683 * locals.var_nl_dn8)) + (((p.p1684 * locals.var_nl_dn8) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn8))), ((locals.var_noiaeff_dn9 + (p.p1683 * locals.var_nl_dn9)) + (((p.p1684 * locals.var_nl_dn9) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn9))), ((locals.var_noiaeff_dn10 + (p.p1683 * locals.var_nl_dn10)) + (((p.p1684 * locals.var_nl_dn10) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn10))), ((locals.var_noiaeff_dn11 + (p.p1683 * locals.var_nl_dn11)) + (((p.p1684 * locals.var_nl_dn11) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn11))), ((locals.var_noiaeff_dn13 + (p.p1683 * locals.var_nl_dn13)) + (((p.p1684 * locals.var_nl_dn13) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn13))), ((locals.var_noiaeff_dn14 + (p.p1683 * locals.var_nl_dn14)) + (((p.p1684 * locals.var_nl_dn14) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn14))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign33020_e55415;
        locals.var_t8_dn0 = assign33020_e55415_d_n0;
        locals.var_t8_dn2 = assign33020_e55415_d_n2;
        locals.var_t8_dn3 = assign33020_e55415_d_n3;
        locals.var_t8_dn4 = assign33020_e55415_d_n4;
        locals.var_t8_dn5 = assign33020_e55415_d_n5;
        locals.var_t8_dn6 = assign33020_e55415_d_n6;
        locals.var_t8_dn7 = assign33020_e55415_d_n7;
        locals.var_t8_dn8 = assign33020_e55415_d_n8;
        locals.var_t8_dn9 = assign33020_e55415_d_n9;
        locals.var_t8_dn10 = assign33020_e55415_d_n10;
        locals.var_t8_dn11 = assign33020_e55415_d_n11;
        locals.var_t8_dn13 = assign33020_e55415_d_n13;
        locals.var_t8_dn14 = assign33020_e55415_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign33030_e55427, assign33030_e55427_d_n0, assign33030_e55427_d_n2, assign33030_e55427_d_n3, assign33030_e55427_d_n4, assign33030_e55427_d_n5, assign33030_e55427_d_n6, assign33030_e55427_d_n7, assign33030_e55427_d_n8, assign33030_e55427_d_n9, assign33030_e55427_d_n10, assign33030_e55427_d_n11, assign33030_e55427_d_n13, assign33030_e55427_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33030_e55421: f64 = (locals.var_nl + locals.var_nstar);
        let assign33030_e55424: f64 = (locals.var_nl + locals.var_nstar);
        let assign33030_e55425: f64 = (assign33030_e55421 * assign33030_e55424);
        (assign33030_e55425, (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn0 + locals.var_nstar_dn0))), (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn2 + locals.var_nstar_dn2))), (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn11 + locals.var_nstar_dn11))), (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn13 + locals.var_nstar_dn13))), (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn14 + locals.var_nstar_dn14))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn13, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign33030_e55427;
        locals.var_t9_dn0 = assign33030_e55427_d_n0;
        locals.var_t9_dn2 = assign33030_e55427_d_n2;
        locals.var_t9_dn3 = assign33030_e55427_d_n3;
        locals.var_t9_dn4 = assign33030_e55427_d_n4;
        locals.var_t9_dn5 = assign33030_e55427_d_n5;
        locals.var_t9_dn6 = assign33030_e55427_d_n6;
        locals.var_t9_dn7 = assign33030_e55427_d_n7;
        locals.var_t9_dn8 = assign33030_e55427_d_n8;
        locals.var_t9_dn9 = assign33030_e55427_d_n9;
        locals.var_t9_dn10 = assign33030_e55427_d_n10;
        locals.var_t9_dn11 = assign33030_e55427_d_n11;
        locals.var_t9_dn13 = assign33030_e55427_d_n13;
        locals.var_t9_dn14 = assign33030_e55427_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign33040_e55451, assign33040_e55451_d_n0, assign33040_e55451_d_n2, assign33040_e55451_d_n3, assign33040_e55451_d_n4, assign33040_e55451_d_n5, assign33040_e55451_d_n6, assign33040_e55451_d_n7, assign33040_e55451_d_n8, assign33040_e55451_d_n9, assign33040_e55451_d_n10, assign33040_e55451_d_n11, assign33040_e55451_d_n13, assign33040_e55451_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33040_e55433: f64 = (locals.var_t1 / locals.var_t2);
        let assign33040_e55436: f64 = (locals.var_t3 + locals.var_t4);
        let assign33040_e55438: f64 = (assign33040_e55436 + locals.var_t5);
        let assign33040_e55439: f64 = (assign33040_e55433 * assign33040_e55438);
        let assign33040_e55442: f64 = (locals.var_t6 / locals.var_t7);
        let assign33040_e55444: f64 = (assign33040_e55442 * locals.var_delclm);
        let assign33040_e55446: f64 = (assign33040_e55444 * locals.var_t8);
        let assign33040_e55448: f64 = (assign33040_e55446 / locals.var_t9);
        let assign33040_e55449: f64 = (assign33040_e55439 + assign33040_e55448);
        (assign33040_e55449, ((((((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn0 + locals.var_t4_dn0) + locals.var_t5_dn0))) + ((((((((((locals.var_t6_dn0 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn0)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn0)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn0)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn2 + locals.var_t4_dn2) + locals.var_t5_dn2))) + ((((((((((locals.var_t6_dn2 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn2)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn2)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn2)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn3 + locals.var_t4_dn3) + locals.var_t5_dn3))) + ((((((((((locals.var_t6_dn3 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn3)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn3)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn3)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn3)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn4 + locals.var_t4_dn4) + locals.var_t5_dn4))) + ((((((((((locals.var_t6_dn4 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn4)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn4)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn4)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn5 + locals.var_t4_dn5) + locals.var_t5_dn5))) + ((((((((((locals.var_t6_dn5 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn5)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn5)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn5)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn6 + locals.var_t4_dn6) + locals.var_t5_dn6))) + ((((((((((locals.var_t6_dn6 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn6)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn6)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn6)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn7 + locals.var_t4_dn7) + locals.var_t5_dn7))) + ((((((((((locals.var_t6_dn7 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn7)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn7)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn7)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn8 + locals.var_t4_dn8) + locals.var_t5_dn8))) + ((((((((((locals.var_t6_dn8 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn8)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn8)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn8)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn9 + locals.var_t4_dn9) + locals.var_t5_dn9))) + ((((((((((locals.var_t6_dn9 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn9)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn9)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn9)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn10 + locals.var_t4_dn10) + locals.var_t5_dn10))) + ((((((((((locals.var_t6_dn10 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn10)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn10)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn10)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn11 + locals.var_t4_dn11) + locals.var_t5_dn11))) + ((((((((((locals.var_t6_dn11 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn11)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn11)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn11)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn13 + locals.var_t4_dn13) + locals.var_t5_dn13))) + ((((((((((locals.var_t6_dn13 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn13)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn13)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn13)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn14 + locals.var_t4_dn14) + locals.var_t5_dn14))) + ((((((((((locals.var_t6_dn14 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn14)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn14)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn14)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn0, locals.var_ssi_dn2, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11, locals.var_ssi_dn13, locals.var_ssi_dn14,)
    }
};
        locals.var_ssi = assign33040_e55451;
        locals.var_ssi_dn0 = assign33040_e55451_d_n0;
        locals.var_ssi_dn2 = assign33040_e55451_d_n2;
        locals.var_ssi_dn3 = assign33040_e55451_d_n3;
        locals.var_ssi_dn4 = assign33040_e55451_d_n4;
        locals.var_ssi_dn5 = assign33040_e55451_d_n5;
        locals.var_ssi_dn6 = assign33040_e55451_d_n6;
        locals.var_ssi_dn7 = assign33040_e55451_d_n7;
        locals.var_ssi_dn8 = assign33040_e55451_d_n8;
        locals.var_ssi_dn9 = assign33040_e55451_d_n9;
        locals.var_ssi_dn10 = assign33040_e55451_d_n10;
        locals.var_ssi_dn11 = assign33040_e55451_d_n11;
        locals.var_ssi_dn13 = assign33040_e55451_d_n13;
        locals.var_ssi_dn14 = assign33040_e55451_d_n14;
        locals.var_ssi_rv = 0.0;

        let (assign33050_e55461, assign33050_e55461_d_n0, assign33050_e55461_d_n2, assign33050_e55461_d_n3, assign33050_e55461_d_n4, assign33050_e55461_d_n5, assign33050_e55461_d_n6, assign33050_e55461_d_n7, assign33050_e55461_d_n8, assign33050_e55461_d_n9, assign33050_e55461_d_n10, assign33050_e55461_d_n11, assign33050_e55461_d_n13, assign33050_e55461_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33050_e55457: f64 = (locals.var_noiaeff * 1.60219e-19);
        let assign33050_e55459: f64 = (assign33050_e55457 * locals.var_vtm);
        (assign33050_e55459, ((locals.var_noiaeff_dn0 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn2 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn3 * 1.60219e-19) * locals.var_vtm), (((locals.var_noiaeff_dn4 * 1.60219e-19) * locals.var_vtm) + (assign33050_e55457 * locals.var_vtm_dn4)), ((locals.var_noiaeff_dn5 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn6 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn7 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn8 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn9 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn10 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn11 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn13 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn14 * 1.60219e-19) * locals.var_vtm),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn13, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign33050_e55461;
        locals.var_t10_dn0 = assign33050_e55461_d_n0;
        locals.var_t10_dn2 = assign33050_e55461_d_n2;
        locals.var_t10_dn3 = assign33050_e55461_d_n3;
        locals.var_t10_dn4 = assign33050_e55461_d_n4;
        locals.var_t10_dn5 = assign33050_e55461_d_n5;
        locals.var_t10_dn6 = assign33050_e55461_d_n6;
        locals.var_t10_dn7 = assign33050_e55461_d_n7;
        locals.var_t10_dn8 = assign33050_e55461_d_n8;
        locals.var_t10_dn9 = assign33050_e55461_d_n9;
        locals.var_t10_dn10 = assign33050_e55461_d_n10;
        locals.var_t10_dn11 = assign33050_e55461_d_n11;
        locals.var_t10_dn13 = assign33050_e55461_d_n13;
        locals.var_t10_dn14 = assign33050_e55461_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign33060_e55477, assign33060_e55477_d_n0, assign33060_e55477_d_n2, assign33060_e55477_d_n3, assign33060_e55477_d_n4, assign33060_e55477_d_n5, assign33060_e55477_d_n6, assign33060_e55477_d_n7, assign33060_e55477_d_n8, assign33060_e55477_d_n9, assign33060_e55477_d_n10, assign33060_e55477_d_n11, assign33060_e55477_d_n13, assign33060_e55477_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33060_e55467: f64 = (locals.var_weff0 * locals.var_nfintotal);
        let assign33060_e55469: f64 = (assign33060_e55467 * locals.var_leffnoi);
        let assign33060_e55471: f64 = (assign33060_e55469 * 10000000000.0);
        let assign33060_e55473: f64 = (assign33060_e55471 * locals.var_nstar);
        let assign33060_e55475: f64 = (assign33060_e55473 * locals.var_nstar);
        (assign33060_e55475, ((((((assign33060_e55467 * locals.var_leffnoi_dn0) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn0)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn0)), ((((((assign33060_e55467 * locals.var_leffnoi_dn2) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn2)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn2)), ((((((assign33060_e55467 * locals.var_leffnoi_dn3) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn3)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn3)), ((((((assign33060_e55467 * locals.var_leffnoi_dn4) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn4)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn4)), ((((((assign33060_e55467 * locals.var_leffnoi_dn5) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn5)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn5)), ((((((assign33060_e55467 * locals.var_leffnoi_dn6) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn6)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn6)), ((((((assign33060_e55467 * locals.var_leffnoi_dn7) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn7)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn7)), ((((((assign33060_e55467 * locals.var_leffnoi_dn8) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn8)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn8)), ((((((assign33060_e55467 * locals.var_leffnoi_dn9) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn9)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn9)), ((((((assign33060_e55467 * locals.var_leffnoi_dn10) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn10)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn10)), ((((((assign33060_e55467 * locals.var_leffnoi_dn11) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn11)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn11)), ((((((assign33060_e55467 * locals.var_leffnoi_dn13) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn13)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn13)), ((((((assign33060_e55467 * locals.var_leffnoi_dn14) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn14)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn13, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign33060_e55477;
        locals.var_t11_dn0 = assign33060_e55477_d_n0;
        locals.var_t11_dn2 = assign33060_e55477_d_n2;
        locals.var_t11_dn3 = assign33060_e55477_d_n3;
        locals.var_t11_dn4 = assign33060_e55477_d_n4;
        locals.var_t11_dn5 = assign33060_e55477_d_n5;
        locals.var_t11_dn6 = assign33060_e55477_d_n6;
        locals.var_t11_dn7 = assign33060_e55477_d_n7;
        locals.var_t11_dn8 = assign33060_e55477_d_n8;
        locals.var_t11_dn9 = assign33060_e55477_d_n9;
        locals.var_t11_dn10 = assign33060_e55477_d_n10;
        locals.var_t11_dn11 = assign33060_e55477_d_n11;
        locals.var_t11_dn13 = assign33060_e55477_d_n13;
        locals.var_t11_dn14 = assign33060_e55477_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign33070_e55489, assign33070_e55489_d_n0, assign33070_e55489_d_n2, assign33070_e55489_d_n3, assign33070_e55489_d_n4, assign33070_e55489_d_n5, assign33070_e55489_d_n6, assign33070_e55489_d_n7, assign33070_e55489_d_n8, assign33070_e55489_d_n9, assign33070_e55489_d_n10, assign33070_e55489_d_n11, assign33070_e55489_d_n13, assign33070_e55489_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33070_e55483: f64 = (locals.var_t10 / locals.var_t11);
        let assign33070_e55485: f64 = (assign33070_e55483 * locals.var_ids_v);
        let assign33070_e55487: f64 = (assign33070_e55485 * locals.var_ids_v);
        (assign33070_e55487, (((((((locals.var_t10_dn0 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn0)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn0)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn0)), (((((((locals.var_t10_dn2 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn2)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn2)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn2)), (((((((locals.var_t10_dn3 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn3)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn3)), (((((((locals.var_t10_dn4 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn4)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn4)), (((((((locals.var_t10_dn5 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn5)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn5)), (((((((locals.var_t10_dn6 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn6)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn6)), (((((((locals.var_t10_dn7 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn7)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn7)), (((((((locals.var_t10_dn8 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn8)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn8)), (((((((locals.var_t10_dn9 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn9)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn9)), (((((((locals.var_t10_dn10 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn10)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn10)), (((((((locals.var_t10_dn11 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn11)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn11)), (((((((locals.var_t10_dn13 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn13)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn13)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn13)), (((((((locals.var_t10_dn14 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn14)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn14)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn14)),)
    } else {
        (locals.var_swi, locals.var_swi_dn0, locals.var_swi_dn2, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11, locals.var_swi_dn13, locals.var_swi_dn14,)
    }
};
        locals.var_swi = assign33070_e55489;
        locals.var_swi_dn0 = assign33070_e55489_d_n0;
        locals.var_swi_dn2 = assign33070_e55489_d_n2;
        locals.var_swi_dn3 = assign33070_e55489_d_n3;
        locals.var_swi_dn4 = assign33070_e55489_d_n4;
        locals.var_swi_dn5 = assign33070_e55489_d_n5;
        locals.var_swi_dn6 = assign33070_e55489_d_n6;
        locals.var_swi_dn7 = assign33070_e55489_d_n7;
        locals.var_swi_dn8 = assign33070_e55489_d_n8;
        locals.var_swi_dn9 = assign33070_e55489_d_n9;
        locals.var_swi_dn10 = assign33070_e55489_d_n10;
        locals.var_swi_dn11 = assign33070_e55489_d_n11;
        locals.var_swi_dn13 = assign33070_e55489_d_n13;
        locals.var_swi_dn14 = assign33070_e55489_d_n14;
        locals.var_swi_rv = 0.0;

        let (assign33080_e55497, assign33080_e55497_d_n0, assign33080_e55497_d_n2, assign33080_e55497_d_n3, assign33080_e55497_d_n4, assign33080_e55497_d_n5, assign33080_e55497_d_n6, assign33080_e55497_d_n7, assign33080_e55497_d_n8, assign33080_e55497_d_n9, assign33080_e55497_d_n10, assign33080_e55497_d_n11, assign33080_e55497_d_n13, assign33080_e55497_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33080_e55495: f64 = (locals.var_swi + locals.var_ssi);
        (assign33080_e55495, (locals.var_swi_dn0 + locals.var_ssi_dn0), (locals.var_swi_dn2 + locals.var_ssi_dn2), (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11), (locals.var_swi_dn13 + locals.var_ssi_dn13), (locals.var_swi_dn14 + locals.var_ssi_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33080_e55497;
        locals.var_t1_dn0 = assign33080_e55497_d_n0;
        locals.var_t1_dn2 = assign33080_e55497_d_n2;
        locals.var_t1_dn3 = assign33080_e55497_d_n3;
        locals.var_t1_dn4 = assign33080_e55497_d_n4;
        locals.var_t1_dn5 = assign33080_e55497_d_n5;
        locals.var_t1_dn6 = assign33080_e55497_d_n6;
        locals.var_t1_dn7 = assign33080_e55497_d_n7;
        locals.var_t1_dn8 = assign33080_e55497_d_n8;
        locals.var_t1_dn9 = assign33080_e55497_d_n9;
        locals.var_t1_dn10 = assign33080_e55497_d_n10;
        locals.var_t1_dn11 = assign33080_e55497_d_n11;
        locals.var_t1_dn13 = assign33080_e55497_d_n13;
        locals.var_t1_dn14 = assign33080_e55497_d_n14;
        locals.var_t1_rv = 0.0;

        let assign33120_e55524: f64 = if p.p79 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign33120_e55524;
        locals.var_guard630_rv = 0.0;

        let (assign33130_e55535, assign33130_e55535_d_n0, assign33130_e55535_d_n2, assign33130_e55535_d_n3, assign33130_e55535_d_n4, assign33130_e55535_d_n5, assign33130_e55535_d_n6, assign33130_e55535_d_n7, assign33130_e55535_d_n8, assign33130_e55535_d_n9, assign33130_e55535_d_n10, assign33130_e55535_d_n11, assign33130_e55535_d_n13, assign33130_e55535_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33130_e55533: f64 = (locals.var_qia2 / locals.var_qsref_i);
        (assign33130_e55533, (locals.var_qia2_dn0 / locals.var_qsref_i), (locals.var_qia2_dn2 / locals.var_qsref_i), (locals.var_qia2_dn3 / locals.var_qsref_i), (locals.var_qia2_dn4 / locals.var_qsref_i), (locals.var_qia2_dn5 / locals.var_qsref_i), (locals.var_qia2_dn6 / locals.var_qsref_i), (locals.var_qia2_dn7 / locals.var_qsref_i), (locals.var_qia2_dn8 / locals.var_qsref_i), (locals.var_qia2_dn9 / locals.var_qsref_i), (locals.var_qia2_dn10 / locals.var_qsref_i), (locals.var_qia2_dn11 / locals.var_qsref_i), (locals.var_qia2_dn13 / locals.var_qsref_i), (locals.var_qia2_dn14 / locals.var_qsref_i),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33130_e55535;
        locals.var_t1_dn0 = assign33130_e55535_d_n0;
        locals.var_t1_dn2 = assign33130_e55535_d_n2;
        locals.var_t1_dn3 = assign33130_e55535_d_n3;
        locals.var_t1_dn4 = assign33130_e55535_d_n4;
        locals.var_t1_dn5 = assign33130_e55535_d_n5;
        locals.var_t1_dn6 = assign33130_e55535_d_n6;
        locals.var_t1_dn7 = assign33130_e55535_d_n7;
        locals.var_t1_dn8 = assign33130_e55535_d_n8;
        locals.var_t1_dn9 = assign33130_e55535_d_n9;
        locals.var_t1_dn10 = assign33130_e55535_d_n10;
        locals.var_t1_dn11 = assign33130_e55535_d_n11;
        locals.var_t1_dn13 = assign33130_e55535_d_n13;
        locals.var_t1_dn14 = assign33130_e55535_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign33140_e55548, assign33140_e55548_d_n0, assign33140_e55548_d_n2, assign33140_e55548_d_n3, assign33140_e55548_d_n4, assign33140_e55548_d_n5, assign33140_e55548_d_n6, assign33140_e55548_d_n7, assign33140_e55548_d_n8, assign33140_e55548_d_n9, assign33140_e55548_d_n10, assign33140_e55548_d_n11, assign33140_e55548_d_n13, assign33140_e55548_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33140_e55545: f64 = (locals.var_t1).powf(locals.var_mpower_i);
        let assign33140_e55546: f64 = (1.0 + assign33140_e55545);
        (assign33140_e55546, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn0)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn2)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn3)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn4)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn5)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn6)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn7)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn8)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn9)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn10)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn11)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn13)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn14)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33140_e55548;
        locals.var_t2_dn0 = assign33140_e55548_d_n0;
        locals.var_t2_dn2 = assign33140_e55548_d_n2;
        locals.var_t2_dn3 = assign33140_e55548_d_n3;
        locals.var_t2_dn4 = assign33140_e55548_d_n4;
        locals.var_t2_dn5 = assign33140_e55548_d_n5;
        locals.var_t2_dn6 = assign33140_e55548_d_n6;
        locals.var_t2_dn7 = assign33140_e55548_d_n7;
        locals.var_t2_dn8 = assign33140_e55548_d_n8;
        locals.var_t2_dn9 = assign33140_e55548_d_n9;
        locals.var_t2_dn10 = assign33140_e55548_d_n10;
        locals.var_t2_dn11 = assign33140_e55548_d_n11;
        locals.var_t2_dn13 = assign33140_e55548_d_n13;
        locals.var_t2_dn14 = assign33140_e55548_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign33150_e55559, assign33150_e55559_d_n0, assign33150_e55559_d_n2, assign33150_e55559_d_n3, assign33150_e55559_d_n4, assign33150_e55559_d_n5, assign33150_e55559_d_n6, assign33150_e55559_d_n7, assign33150_e55559_d_n8, assign33150_e55559_d_n9, assign33150_e55559_d_n10, assign33150_e55559_d_n11, assign33150_e55559_d_n13, assign33150_e55559_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33150_e55557: f64 = (locals.var_noia2_i / locals.var_t2);
        (assign33150_e55557, (-((locals.var_noia2_i * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33150_e55559;
        locals.var_t3_dn0 = assign33150_e55559_d_n0;
        locals.var_t3_dn2 = assign33150_e55559_d_n2;
        locals.var_t3_dn3 = assign33150_e55559_d_n3;
        locals.var_t3_dn4 = assign33150_e55559_d_n4;
        locals.var_t3_dn5 = assign33150_e55559_d_n5;
        locals.var_t3_dn6 = assign33150_e55559_d_n6;
        locals.var_t3_dn7 = assign33150_e55559_d_n7;
        locals.var_t3_dn8 = assign33150_e55559_d_n8;
        locals.var_t3_dn9 = assign33150_e55559_d_n9;
        locals.var_t3_dn10 = assign33150_e55559_d_n10;
        locals.var_t3_dn11 = assign33150_e55559_d_n11;
        locals.var_t3_dn13 = assign33150_e55559_d_n13;
        locals.var_t3_dn14 = assign33150_e55559_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_129(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33160_e55570, assign33160_e55570_d_n0, assign33160_e55570_d_n2, assign33160_e55570_d_n3, assign33160_e55570_d_n4, assign33160_e55570_d_n5, assign33160_e55570_d_n6, assign33160_e55570_d_n7, assign33160_e55570_d_n8, assign33160_e55570_d_n9, assign33160_e55570_d_n10, assign33160_e55570_d_n11, assign33160_e55570_d_n13, assign33160_e55570_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33160_e55568: f64 = (locals.var_t3 / p.p1682);
        (assign33160_e55568, (locals.var_t3_dn0 / p.p1682), (locals.var_t3_dn2 / p.p1682), (locals.var_t3_dn3 / p.p1682), (locals.var_t3_dn4 / p.p1682), (locals.var_t3_dn5 / p.p1682), (locals.var_t3_dn6 / p.p1682), (locals.var_t3_dn7 / p.p1682), (locals.var_t3_dn8 / p.p1682), (locals.var_t3_dn9 / p.p1682), (locals.var_t3_dn10 / p.p1682), (locals.var_t3_dn11 / p.p1682), (locals.var_t3_dn13 / p.p1682), (locals.var_t3_dn14 / p.p1682),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33160_e55570;
        locals.var_t4_dn0 = assign33160_e55570_d_n0;
        locals.var_t4_dn2 = assign33160_e55570_d_n2;
        locals.var_t4_dn3 = assign33160_e55570_d_n3;
        locals.var_t4_dn4 = assign33160_e55570_d_n4;
        locals.var_t4_dn5 = assign33160_e55570_d_n5;
        locals.var_t4_dn6 = assign33160_e55570_d_n6;
        locals.var_t4_dn7 = assign33160_e55570_d_n7;
        locals.var_t4_dn8 = assign33160_e55570_d_n8;
        locals.var_t4_dn9 = assign33160_e55570_d_n9;
        locals.var_t4_dn10 = assign33160_e55570_d_n10;
        locals.var_t4_dn11 = assign33160_e55570_d_n11;
        locals.var_t4_dn13 = assign33160_e55570_d_n13;
        locals.var_t4_dn14 = assign33160_e55570_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33170_e55598, assign33170_e55598_d_n0, assign33170_e55598_d_n2, assign33170_e55598_d_n3, assign33170_e55598_d_n4, assign33170_e55598_d_n5, assign33170_e55598_d_n6, assign33170_e55598_d_n7, assign33170_e55598_d_n8, assign33170_e55598_d_n9, assign33170_e55598_d_n10, assign33170_e55598_d_n11, assign33170_e55598_d_n13, assign33170_e55598_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33170_e55580: f64 = (locals.var_t4 + 1.0);
        let assign33170_e55583: f64 = (locals.var_t4 - 1.0);
        let assign33170_e55586: f64 = (locals.var_t4 - 1.0);
        let assign33170_e55587: f64 = (assign33170_e55583 * assign33170_e55586);
        let assign33170_e55590: f64 = (0.25 * p.p1688);
        let assign33170_e55592: f64 = (assign33170_e55590 * p.p1688);
        let assign33170_e55593: f64 = (assign33170_e55587 + assign33170_e55592);
        let assign33170_e55594: f64 = (assign33170_e55593).sqrt();
        let assign33170_e55595: f64 = (assign33170_e55580 + assign33170_e55594);
        let assign33170_e55596: f64 = (0.5 * assign33170_e55595);
        (assign33170_e55596, (0.5 * (locals.var_t4_dn0 + (((locals.var_t4_dn0 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn0)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn2 + (((locals.var_t4_dn2 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn2)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn3)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn4)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn5)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn6)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn7)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn8)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn9)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn10)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn11)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn13 + (((locals.var_t4_dn13 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn13)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn14 + (((locals.var_t4_dn14 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn14)) / (2.0 * assign33170_e55594)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33170_e55598;
        locals.var_t5_dn0 = assign33170_e55598_d_n0;
        locals.var_t5_dn2 = assign33170_e55598_d_n2;
        locals.var_t5_dn3 = assign33170_e55598_d_n3;
        locals.var_t5_dn4 = assign33170_e55598_d_n4;
        locals.var_t5_dn5 = assign33170_e55598_d_n5;
        locals.var_t5_dn6 = assign33170_e55598_d_n6;
        locals.var_t5_dn7 = assign33170_e55598_d_n7;
        locals.var_t5_dn8 = assign33170_e55598_d_n8;
        locals.var_t5_dn9 = assign33170_e55598_d_n9;
        locals.var_t5_dn10 = assign33170_e55598_d_n10;
        locals.var_t5_dn11 = assign33170_e55598_d_n11;
        locals.var_t5_dn13 = assign33170_e55598_d_n13;
        locals.var_t5_dn14 = assign33170_e55598_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33180_e55609, assign33180_e55609_d_n0, assign33180_e55609_d_n2, assign33180_e55609_d_n3, assign33180_e55609_d_n4, assign33180_e55609_d_n5, assign33180_e55609_d_n6, assign33180_e55609_d_n7, assign33180_e55609_d_n8, assign33180_e55609_d_n9, assign33180_e55609_d_n10, assign33180_e55609_d_n11, assign33180_e55609_d_n13, assign33180_e55609_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33180_e55607: f64 = (p.p1682 * locals.var_t5);
        (assign33180_e55607, (p.p1682 * locals.var_t5_dn0), (p.p1682 * locals.var_t5_dn2), (p.p1682 * locals.var_t5_dn3), (p.p1682 * locals.var_t5_dn4), (p.p1682 * locals.var_t5_dn5), (p.p1682 * locals.var_t5_dn6), (p.p1682 * locals.var_t5_dn7), (p.p1682 * locals.var_t5_dn8), (p.p1682 * locals.var_t5_dn9), (p.p1682 * locals.var_t5_dn10), (p.p1682 * locals.var_t5_dn11), (p.p1682 * locals.var_t5_dn13), (p.p1682 * locals.var_t5_dn14),)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign33180_e55609;
        locals.var_noiaeff_dn0 = assign33180_e55609_d_n0;
        locals.var_noiaeff_dn2 = assign33180_e55609_d_n2;
        locals.var_noiaeff_dn3 = assign33180_e55609_d_n3;
        locals.var_noiaeff_dn4 = assign33180_e55609_d_n4;
        locals.var_noiaeff_dn5 = assign33180_e55609_d_n5;
        locals.var_noiaeff_dn6 = assign33180_e55609_d_n6;
        locals.var_noiaeff_dn7 = assign33180_e55609_d_n7;
        locals.var_noiaeff_dn8 = assign33180_e55609_d_n8;
        locals.var_noiaeff_dn9 = assign33180_e55609_d_n9;
        locals.var_noiaeff_dn10 = assign33180_e55609_d_n10;
        locals.var_noiaeff_dn11 = assign33180_e55609_d_n11;
        locals.var_noiaeff_dn13 = assign33180_e55609_d_n13;
        locals.var_noiaeff_dn14 = assign33180_e55609_d_n14;
        locals.var_noiaeff_rv = 0.0;

        let (assign33190_e55622, assign33190_e55622_d_n0, assign33190_e55622_d_n2, assign33190_e55622_d_n3, assign33190_e55622_d_n4, assign33190_e55622_d_n5, assign33190_e55622_d_n6, assign33190_e55622_d_n7, assign33190_e55622_d_n8, assign33190_e55622_d_n9, assign33190_e55622_d_n10, assign33190_e55622_d_n11, assign33190_e55622_d_n13, assign33190_e55622_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33190_e55618: f64 = (2.0 * locals.var_vtm);
        let assign33190_e55620: f64 = (assign33190_e55618 / locals.var_esatl);
        (assign33190_e55620, (-((assign33190_e55618 * locals.var_esatl_dn0) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn2) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn3) / (locals.var_esatl * locals.var_esatl))), ((((2.0 * locals.var_vtm_dn4) * locals.var_esatl) - (assign33190_e55618 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (-((assign33190_e55618 * locals.var_esatl_dn5) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn6) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn7) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn8) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn9) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn10) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn11) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn13) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn14) / (locals.var_esatl * locals.var_esatl))),)
    } else {
        (locals.var_lambdac_fn2, locals.var_lambdac_fn2_dn0, locals.var_lambdac_fn2_dn2, locals.var_lambdac_fn2_dn3, locals.var_lambdac_fn2_dn4, locals.var_lambdac_fn2_dn5, locals.var_lambdac_fn2_dn6, locals.var_lambdac_fn2_dn7, locals.var_lambdac_fn2_dn8, locals.var_lambdac_fn2_dn9, locals.var_lambdac_fn2_dn10, locals.var_lambdac_fn2_dn11, locals.var_lambdac_fn2_dn13, locals.var_lambdac_fn2_dn14,)
    }
};
        locals.var_lambdac_fn2 = assign33190_e55622;
        locals.var_lambdac_fn2_dn0 = assign33190_e55622_d_n0;
        locals.var_lambdac_fn2_dn2 = assign33190_e55622_d_n2;
        locals.var_lambdac_fn2_dn3 = assign33190_e55622_d_n3;
        locals.var_lambdac_fn2_dn4 = assign33190_e55622_d_n4;
        locals.var_lambdac_fn2_dn5 = assign33190_e55622_d_n5;
        locals.var_lambdac_fn2_dn6 = assign33190_e55622_d_n6;
        locals.var_lambdac_fn2_dn7 = assign33190_e55622_d_n7;
        locals.var_lambdac_fn2_dn8 = assign33190_e55622_d_n8;
        locals.var_lambdac_fn2_dn9 = assign33190_e55622_d_n9;
        locals.var_lambdac_fn2_dn10 = assign33190_e55622_d_n10;
        locals.var_lambdac_fn2_dn11 = assign33190_e55622_d_n11;
        locals.var_lambdac_fn2_dn13 = assign33190_e55622_d_n13;
        locals.var_lambdac_fn2_dn14 = assign33190_e55622_d_n14;
        locals.var_lambdac_fn2_rv = 0.0;

        let (assign33200_e55635, assign33200_e55635_d_n0, assign33200_e55635_d_n2, assign33200_e55635_d_n3, assign33200_e55635_d_n4, assign33200_e55635_d_n5, assign33200_e55635_d_n6, assign33200_e55635_d_n7, assign33200_e55635_d_n8, assign33200_e55635_d_n9, assign33200_e55635_d_n10, assign33200_e55635_d_n11, assign33200_e55635_d_n13, assign33200_e55635_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33200_e55632: f64 = (locals.var_lambdac_fn2 * locals.var_dqi);
        let assign33200_e55633: f64 = (1.0 + assign33200_e55632);
        (assign33200_e55633, ((locals.var_lambdac_fn2_dn0 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn0)), ((locals.var_lambdac_fn2_dn2 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn2)), ((locals.var_lambdac_fn2_dn3 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn3)), ((locals.var_lambdac_fn2_dn4 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn4)), ((locals.var_lambdac_fn2_dn5 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn5)), ((locals.var_lambdac_fn2_dn6 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn6)), ((locals.var_lambdac_fn2_dn7 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn7)), ((locals.var_lambdac_fn2_dn8 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn8)), ((locals.var_lambdac_fn2_dn9 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn9)), ((locals.var_lambdac_fn2_dn10 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn10)), ((locals.var_lambdac_fn2_dn11 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn11)), ((locals.var_lambdac_fn2_dn13 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn13)), ((locals.var_lambdac_fn2_dn14 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33200_e55635;
        locals.var_t1_dn0 = assign33200_e55635_d_n0;
        locals.var_t1_dn2 = assign33200_e55635_d_n2;
        locals.var_t1_dn3 = assign33200_e55635_d_n3;
        locals.var_t1_dn4 = assign33200_e55635_d_n4;
        locals.var_t1_dn5 = assign33200_e55635_d_n5;
        locals.var_t1_dn6 = assign33200_e55635_d_n6;
        locals.var_t1_dn7 = assign33200_e55635_d_n7;
        locals.var_t1_dn8 = assign33200_e55635_d_n8;
        locals.var_t1_dn9 = assign33200_e55635_d_n9;
        locals.var_t1_dn10 = assign33200_e55635_d_n10;
        locals.var_t1_dn11 = assign33200_e55635_d_n11;
        locals.var_t1_dn13 = assign33200_e55635_d_n13;
        locals.var_t1_dn14 = assign33200_e55635_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign33210_e55648, assign33210_e55648_d_n0, assign33210_e55648_d_n2, assign33210_e55648_d_n3, assign33210_e55648_d_n4, assign33210_e55648_d_n5, assign33210_e55648_d_n6, assign33210_e55648_d_n7, assign33210_e55648_d_n8, assign33210_e55648_d_n9, assign33210_e55648_d_n10, assign33210_e55648_d_n11, assign33210_e55648_d_n13, assign33210_e55648_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33210_e55645: f64 = (p.p1685 * locals.var_dqi);
        let assign33210_e55646: f64 = (1.0 + assign33210_e55645);
        (assign33210_e55646, (p.p1685 * locals.var_dqi_dn0), (p.p1685 * locals.var_dqi_dn2), (p.p1685 * locals.var_dqi_dn3), (p.p1685 * locals.var_dqi_dn4), (p.p1685 * locals.var_dqi_dn5), (p.p1685 * locals.var_dqi_dn6), (p.p1685 * locals.var_dqi_dn7), (p.p1685 * locals.var_dqi_dn8), (p.p1685 * locals.var_dqi_dn9), (p.p1685 * locals.var_dqi_dn10), (p.p1685 * locals.var_dqi_dn11), (p.p1685 * locals.var_dqi_dn13), (p.p1685 * locals.var_dqi_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33210_e55648;
        locals.var_t2_dn0 = assign33210_e55648_d_n0;
        locals.var_t2_dn2 = assign33210_e55648_d_n2;
        locals.var_t2_dn3 = assign33210_e55648_d_n3;
        locals.var_t2_dn4 = assign33210_e55648_d_n4;
        locals.var_t2_dn5 = assign33210_e55648_d_n5;
        locals.var_t2_dn6 = assign33210_e55648_d_n6;
        locals.var_t2_dn7 = assign33210_e55648_d_n7;
        locals.var_t2_dn8 = assign33210_e55648_d_n8;
        locals.var_t2_dn9 = assign33210_e55648_d_n9;
        locals.var_t2_dn10 = assign33210_e55648_d_n10;
        locals.var_t2_dn11 = assign33210_e55648_d_n11;
        locals.var_t2_dn13 = assign33210_e55648_d_n13;
        locals.var_t2_dn14 = assign33210_e55648_d_n14;
        locals.var_t2_rv = 0.0;

        let assign33230_e55674: f64 = if ((locals.var_t1 > 0.0) && (locals.var_t2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard631 = assign33230_e55674;
        locals.var_guard631_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

        let assign33300_e55814: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign33300_e55814;
        locals.var_guard632_rv = 0.0;

        let assign33310_e55817: f64 = if p.p72 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign33310_e55817;
        locals.var_guard633_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_noibeta_rv = 0.0;

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
        locals.var_noitheta_rv = 0.0;

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
        locals.var_noicorr_rv = 0.0;

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
        locals.var_noilowid_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_130(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t3_rv = 0.0;

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
        locals.var_noieta_rv = 0.0;

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
        locals.var_dvsat3_rv = 0.0;

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
        locals.var_noiwi_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_mnud0_rv = 0.0;

        let assign33500_e56026: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign33500_e56026;
        locals.var_guard634_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_t6_rv = 0.0;

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
        locals.var_mob0_rv = 0.0;

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
        locals.var_mob0_rv = 0.0;

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
        locals.var_eeffm0_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

        let assign33580_e56159: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign33580_e56159;
        locals.var_guard635_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_dmob0_rv = 0.0;

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
        locals.var_dmob0_rv = 0.0;

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
        locals.var_dmob0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_131(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_dvsat0_rv = 0.0;

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
        locals.var_etaiv0_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_ids0_ov_dqi0_rv = 0.0;

        let assign33680_e56289: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign33680_e56289;
        locals.var_guard636_rv = 0.0;

        let assign33690_e56292: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign33690_e56292;
        locals.var_guard637_rv = 0.0;

        let assign33700_e56295: f64 = if p.p64 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign33700_e56295;
        locals.var_guard638_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_t6_rv = 0.0;

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
        locals.var_rdsi0_rv = 0.0;

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
        locals.var_dr0_rv = 0.0;

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
        locals.var_dr0_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_t6_rv = 0.0;

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
        locals.var_rdsi0_rv = 0.0;

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
        locals.var_rdsi0_rv = 0.0;

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
        locals.var_dr0_rv = 0.0;

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
        locals.var_noigd0_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_132(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t6_rv = 0.0;

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
        locals.var_t7_rv = 0.0;

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
        locals.var_t5_2_rv = 0.0;

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
        locals.var_t5_3_rv = 0.0;

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
        locals.var_t5_4_rv = 0.0;

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
        locals.var_t7_2_rv = 0.0;

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
        locals.var_t7_3_rv = 0.0;

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
        locals.var_t7_4_rv = 0.0;

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
        locals.var_t7_5_rv = 0.0;

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
        locals.var_gamma1_rv = 0.0;

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
        locals.var_gamma2_rv = 0.0;

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
        locals.var_gamma_rv = 0.0;

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
        locals.var_delta1_rv = 0.0;

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
        locals.var_delta2_rv = 0.0;

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
        locals.var_delta3_rv = 0.0;

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
        locals.var_delta_rv = 0.0;

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
        locals.var_t8_rv = 0.0;

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
        locals.var_gamma_rv = 0.0;

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
        locals.var_delta_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_133(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
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
        locals.var_sigrat_rv = 0.0;

        let assign34160_e56893: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign34160_e56893;
        locals.var_guard642_rv = 0.0;

        let assign34250_e56944: f64 = if p.p76 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign34250_e56944;
        locals.var_guard651_rv = 0.0;

        let assign34260_e56947: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign34260_e56947;
        locals.var_guard652_rv = 0.0;

        let assign34270_e56950: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign34270_e56950;
        locals.var_guard653_rv = 0.0;

        let assign34280_e56953: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign34280_e56953;
        locals.var_guard654_rv = 0.0;

        let assign34290_e56956: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign34290_e56956;
        locals.var_guard655_rv = 0.0;

        let assign34300_e56959: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign34300_e56959;
        locals.var_guard656_rv = 0.0;

        let assign34310_e56962: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign34310_e56962;
        locals.var_guard657_rv = 0.0;

        let assign34320_e56965: f64 = if p.p1910 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign34320_e56965;
        locals.var_guard658_rv = 0.0;

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
        locals.var_rdstempvs_rv = 0.0;

        let assign34340_e57045: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign34340_e57045;
        locals.var_guard659_rv = 0.0;

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
        locals.var_vsatrsd_t_rv = 0.0;

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
        locals.var_vsatrsd_t_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_vsatrsd_eff_rv = 0.0;

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
        locals.var_vsatrsd_eff_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

        let assign34440_e57308: f64 = if p.p1917 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign34440_e57308;
        locals.var_guard660_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_134(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t3_rv = 0.0;

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
        locals.var_isat_rd_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_vsat_rd_rv = 0.0;

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
        locals.var_delta_vsrd_rv = 0.0;

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
        locals.var_t6_rv = 0.0;

        let assign34540_e57448: f64 = if p.p1911 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign34540_e57448;
        locals.var_guard661_rv = 0.0;

        let assign34550_e57451: f64 = if p.p1910 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign34550_e57451;
        locals.var_guard662_rv = 0.0;

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
        locals.var_rdstempvs_rv = 0.0;

        let assign34570_e57533: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign34570_e57533;
        locals.var_guard663_rv = 0.0;

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
        locals.var_vsatrsd_t_rv = 0.0;

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
        locals.var_vsatrsd_t_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_vsatrsd_eff_rv = 0.0;

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
        locals.var_vsatrsd_eff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_135(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
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
        locals.var_t2_rv = 0.0;

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
        locals.var_isat_rs_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_vsat_rs_rv = 0.0;

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
        locals.var_t5_rv = 0.0;

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
        locals.var_delta_vsrs_rv = 0.0;

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
        locals.var_t6_rv = 0.0;

        let assign34820_e57955: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign34820_e57955;
        locals.var_guard669_rv = 0.0;

        let assign34900_e57995: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign34900_e57995;
        locals.var_guard677_rv = 0.0;

        let assign34950_e58014: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard682 = assign34950_e58014;
        locals.var_guard682_rv = 0.0;

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
        locals.var_q0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
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
        locals: &mut StampLocals,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq0_e1945, eq0_e1945_d_n0, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n13, eq0_e1945_d_n14,) = {
    if (locals.var_guard641 != 0.0) {
        let eq0_e1943: f64 = (locals.var_devsign * locals.var_ids_v);
        let eq0_e1943_d_n0: f64 = (locals.var_devsign * locals.var_ids_v_dn0);
        let eq0_e1943_d_n2: f64 = (locals.var_devsign * locals.var_ids_v_dn2);
        let eq0_e1943_d_n3: f64 = (locals.var_devsign * locals.var_ids_v_dn3);
        let eq0_e1943_d_n4: f64 = (locals.var_devsign * locals.var_ids_v_dn4);
        let eq0_e1943_d_n5: f64 = (locals.var_devsign * locals.var_ids_v_dn5);
        let eq0_e1943_d_n6: f64 = (locals.var_devsign * locals.var_ids_v_dn6);
        let eq0_e1943_d_n7: f64 = (locals.var_devsign * locals.var_ids_v_dn7);
        let eq0_e1943_d_n8: f64 = (locals.var_devsign * locals.var_ids_v_dn8);
        let eq0_e1943_d_n9: f64 = (locals.var_devsign * locals.var_ids_v_dn9);
        let eq0_e1943_d_n10: f64 = (locals.var_devsign * locals.var_ids_v_dn10);
        let eq0_e1943_d_n11: f64 = (locals.var_devsign * locals.var_ids_v_dn11);
        let eq0_e1943_d_n13: f64 = (locals.var_devsign * locals.var_ids_v_dn13);
        let eq0_e1943_d_n14: f64 = (locals.var_devsign * locals.var_ids_v_dn14);
        (eq0_e1943, eq0_e1943_d_n0, eq0_e1943_d_n2, eq0_e1943_d_n3, eq0_e1943_d_n4, eq0_e1943_d_n5, eq0_e1943_d_n6, eq0_e1943_d_n7, eq0_e1943_d_n8, eq0_e1943_d_n9, eq0_e1943_d_n10, eq0_e1943_d_n11, eq0_e1943_d_n13, eq0_e1943_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1945;
        let eq0_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq0_node_derivatives: [f64; 13] = [eq0_e1945_d_n0, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n13, eq0_e1945_d_n14];
        let eq0_branch_derivative_indices: [usize; 0] = [];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq0_value),
            &eq0_node_derivative_indices,
            &eq0_node_derivatives,
            &eq0_branch_derivative_indices,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1952, eq1_e1952_d_n0, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n13, eq1_e1952_d_n14,) = {
    if (locals.var_guard641 == 0.0) {
        let eq1_e1950: f64 = (locals.var_devsign * locals.var_ids_v);
        let eq1_e1950_d_n0: f64 = (locals.var_devsign * locals.var_ids_v_dn0);
        let eq1_e1950_d_n2: f64 = (locals.var_devsign * locals.var_ids_v_dn2);
        let eq1_e1950_d_n3: f64 = (locals.var_devsign * locals.var_ids_v_dn3);
        let eq1_e1950_d_n4: f64 = (locals.var_devsign * locals.var_ids_v_dn4);
        let eq1_e1950_d_n5: f64 = (locals.var_devsign * locals.var_ids_v_dn5);
        let eq1_e1950_d_n6: f64 = (locals.var_devsign * locals.var_ids_v_dn6);
        let eq1_e1950_d_n7: f64 = (locals.var_devsign * locals.var_ids_v_dn7);
        let eq1_e1950_d_n8: f64 = (locals.var_devsign * locals.var_ids_v_dn8);
        let eq1_e1950_d_n9: f64 = (locals.var_devsign * locals.var_ids_v_dn9);
        let eq1_e1950_d_n10: f64 = (locals.var_devsign * locals.var_ids_v_dn10);
        let eq1_e1950_d_n11: f64 = (locals.var_devsign * locals.var_ids_v_dn11);
        let eq1_e1950_d_n13: f64 = (locals.var_devsign * locals.var_ids_v_dn13);
        let eq1_e1950_d_n14: f64 = (locals.var_devsign * locals.var_ids_v_dn14);
        (eq1_e1950, eq1_e1950_d_n0, eq1_e1950_d_n2, eq1_e1950_d_n3, eq1_e1950_d_n4, eq1_e1950_d_n5, eq1_e1950_d_n6, eq1_e1950_d_n7, eq1_e1950_d_n8, eq1_e1950_d_n9, eq1_e1950_d_n10, eq1_e1950_d_n11, eq1_e1950_d_n13, eq1_e1950_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1952;
        let eq1_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq1_node_derivatives: [f64; 13] = [eq1_e1952_d_n0, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n13, eq1_e1952_d_n14];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1961, eq2_e1961_d_n0, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15,) = {
    if (locals.var_guard642 != 0.0) {
        let eq2_e1956: f64 = (locals.var_devsign * locals.var_gtau);
        let eq2_e1956_d_n0: f64 = (locals.var_devsign * locals.var_gtau_dn0);
        let eq2_e1956_d_n2: f64 = (locals.var_devsign * locals.var_gtau_dn2);
        let eq2_e1956_d_n3: f64 = (locals.var_devsign * locals.var_gtau_dn3);
        let eq2_e1956_d_n4: f64 = (locals.var_devsign * locals.var_gtau_dn4);
        let eq2_e1956_d_n5: f64 = (locals.var_devsign * locals.var_gtau_dn5);
        let eq2_e1956_d_n6: f64 = (locals.var_devsign * locals.var_gtau_dn6);
        let eq2_e1956_d_n7: f64 = (locals.var_devsign * locals.var_gtau_dn7);
        let eq2_e1956_d_n8: f64 = (locals.var_devsign * locals.var_gtau_dn8);
        let eq2_e1956_d_n9: f64 = (locals.var_devsign * locals.var_gtau_dn9);
        let eq2_e1956_d_n10: f64 = (locals.var_devsign * locals.var_gtau_dn10);
        let eq2_e1956_d_n11: f64 = (locals.var_devsign * locals.var_gtau_dn11);
        let eq2_e1956_d_n13: f64 = (locals.var_devsign * locals.var_gtau_dn13);
        let eq2_e1956_d_n14: f64 = (locals.var_devsign * locals.var_gtau_dn14);
        let eq2_e1958: f64 = (-(nv15 - 0.0));
        let eq2_e1959: f64 = (eq2_e1956 * eq2_e1958);
        let eq2_e1959_d_n0: f64 = (eq2_e1956_d_n0 * eq2_e1958);
        let eq2_e1959_d_n2: f64 = (eq2_e1956_d_n2 * eq2_e1958);
        let eq2_e1959_d_n3: f64 = (eq2_e1956_d_n3 * eq2_e1958);
        let eq2_e1959_d_n4: f64 = (eq2_e1956_d_n4 * eq2_e1958);
        let eq2_e1959_d_n5: f64 = (eq2_e1956_d_n5 * eq2_e1958);
        let eq2_e1959_d_n6: f64 = (eq2_e1956_d_n6 * eq2_e1958);
        let eq2_e1959_d_n7: f64 = (eq2_e1956_d_n7 * eq2_e1958);
        let eq2_e1959_d_n8: f64 = (eq2_e1956_d_n8 * eq2_e1958);
        let eq2_e1959_d_n9: f64 = (eq2_e1956_d_n9 * eq2_e1958);
        let eq2_e1959_d_n10: f64 = (eq2_e1956_d_n10 * eq2_e1958);
        let eq2_e1959_d_n11: f64 = (eq2_e1956_d_n11 * eq2_e1958);
        let eq2_e1959_d_n13: f64 = (eq2_e1956_d_n13 * eq2_e1958);
        let eq2_e1959_d_n14: f64 = (eq2_e1956_d_n14 * eq2_e1958);
        let eq2_e1959_d_n15: f64 = (eq2_e1956 * (-1.0));
        (eq2_e1959, eq2_e1959_d_n0, eq2_e1959_d_n2, eq2_e1959_d_n3, eq2_e1959_d_n4, eq2_e1959_d_n5, eq2_e1959_d_n6, eq2_e1959_d_n7, eq2_e1959_d_n8, eq2_e1959_d_n9, eq2_e1959_d_n10, eq2_e1959_d_n11, eq2_e1959_d_n13, eq2_e1959_d_n14, eq2_e1959_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1961;
        let eq2_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
        let eq2_node_derivatives: [f64; 14] = [eq2_e1961_d_n0, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e1971, eq3_e1971_d_n0, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15,) = {
    if (locals.var_guard642 != 0.0) {
        let eq3_e1965: f64 = (locals.var_devsign * locals.var_xdpart);
        let eq3_e1965_d_n0: f64 = (locals.var_devsign * locals.var_xdpart_dn0);
        let eq3_e1965_d_n2: f64 = (locals.var_devsign * locals.var_xdpart_dn2);
        let eq3_e1965_d_n3: f64 = (locals.var_devsign * locals.var_xdpart_dn3);
        let eq3_e1965_d_n4: f64 = (locals.var_devsign * locals.var_xdpart_dn4);
        let eq3_e1965_d_n5: f64 = (locals.var_devsign * locals.var_xdpart_dn5);
        let eq3_e1965_d_n6: f64 = (locals.var_devsign * locals.var_xdpart_dn6);
        let eq3_e1965_d_n7: f64 = (locals.var_devsign * locals.var_xdpart_dn7);
        let eq3_e1965_d_n8: f64 = (locals.var_devsign * locals.var_xdpart_dn8);
        let eq3_e1965_d_n9: f64 = (locals.var_devsign * locals.var_xdpart_dn9);
        let eq3_e1965_d_n10: f64 = (locals.var_devsign * locals.var_xdpart_dn10);
        let eq3_e1965_d_n11: f64 = (locals.var_devsign * locals.var_xdpart_dn11);
        let eq3_e1965_d_n13: f64 = (locals.var_devsign * locals.var_xdpart_dn13);
        let eq3_e1965_d_n14: f64 = (locals.var_devsign * locals.var_xdpart_dn14);
        let eq3_e1967: f64 = (eq3_e1965 * locals.var_gtau);
        let eq3_e1967_d_n0: f64 = ((eq3_e1965_d_n0 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn0));
        let eq3_e1967_d_n2: f64 = ((eq3_e1965_d_n2 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn2));
        let eq3_e1967_d_n3: f64 = ((eq3_e1965_d_n3 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn3));
        let eq3_e1967_d_n4: f64 = ((eq3_e1965_d_n4 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn4));
        let eq3_e1967_d_n5: f64 = ((eq3_e1965_d_n5 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn5));
        let eq3_e1967_d_n6: f64 = ((eq3_e1965_d_n6 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn6));
        let eq3_e1967_d_n7: f64 = ((eq3_e1965_d_n7 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn7));
        let eq3_e1967_d_n8: f64 = ((eq3_e1965_d_n8 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn8));
        let eq3_e1967_d_n9: f64 = ((eq3_e1965_d_n9 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn9));
        let eq3_e1967_d_n10: f64 = ((eq3_e1965_d_n10 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn10));
        let eq3_e1967_d_n11: f64 = ((eq3_e1965_d_n11 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn11));
        let eq3_e1967_d_n13: f64 = ((eq3_e1965_d_n13 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn13));
        let eq3_e1967_d_n14: f64 = ((eq3_e1965_d_n14 * locals.var_gtau) + (eq3_e1965 * locals.var_gtau_dn14));
        let eq3_e1969: f64 = (eq3_e1967 * (nv15 - 0.0));
        let eq3_e1969_d_n0: f64 = (eq3_e1967_d_n0 * (nv15 - 0.0));
        let eq3_e1969_d_n2: f64 = (eq3_e1967_d_n2 * (nv15 - 0.0));
        let eq3_e1969_d_n3: f64 = (eq3_e1967_d_n3 * (nv15 - 0.0));
        let eq3_e1969_d_n4: f64 = (eq3_e1967_d_n4 * (nv15 - 0.0));
        let eq3_e1969_d_n5: f64 = (eq3_e1967_d_n5 * (nv15 - 0.0));
        let eq3_e1969_d_n6: f64 = (eq3_e1967_d_n6 * (nv15 - 0.0));
        let eq3_e1969_d_n7: f64 = (eq3_e1967_d_n7 * (nv15 - 0.0));
        let eq3_e1969_d_n8: f64 = (eq3_e1967_d_n8 * (nv15 - 0.0));
        let eq3_e1969_d_n9: f64 = (eq3_e1967_d_n9 * (nv15 - 0.0));
        let eq3_e1969_d_n10: f64 = (eq3_e1967_d_n10 * (nv15 - 0.0));
        let eq3_e1969_d_n11: f64 = (eq3_e1967_d_n11 * (nv15 - 0.0));
        let eq3_e1969_d_n13: f64 = (eq3_e1967_d_n13 * (nv15 - 0.0));
        let eq3_e1969_d_n14: f64 = (eq3_e1967_d_n14 * (nv15 - 0.0));
        (eq3_e1969, eq3_e1969_d_n0, eq3_e1969_d_n2, eq3_e1969_d_n3, eq3_e1969_d_n4, eq3_e1969_d_n5, eq3_e1969_d_n6, eq3_e1969_d_n7, eq3_e1969_d_n8, eq3_e1969_d_n9, eq3_e1969_d_n10, eq3_e1969_d_n11, eq3_e1969_d_n13, eq3_e1969_d_n14, eq3_e1967,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1971;
        let eq3_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
        let eq3_node_derivatives: [f64; 14] = [eq3_e1971_d_n0, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15];
        let eq3_branch_derivative_indices: [usize; 0] = [];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivative_indices,
            &eq3_node_derivatives,
            &eq3_branch_derivative_indices,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14,) = {
    if (locals.var_guard642 == 0.0) {
        let eq4_e1976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qd_v);
        let eq4_e1977: f64 = (locals.var_devsign * eq4_e1976);
        let eq4_e1977_d_n0: f64 = (locals.var_devsign * (locals.var_qd_v_dn0 * ddt_scale));
        let eq4_e1977_d_n2: f64 = (locals.var_devsign * (locals.var_qd_v_dn2 * ddt_scale));
        let eq4_e1977_d_n3: f64 = (locals.var_devsign * (locals.var_qd_v_dn3 * ddt_scale));
        let eq4_e1977_d_n4: f64 = (locals.var_devsign * (locals.var_qd_v_dn4 * ddt_scale));
        let eq4_e1977_d_n5: f64 = (locals.var_devsign * (locals.var_qd_v_dn5 * ddt_scale));
        let eq4_e1977_d_n6: f64 = (locals.var_devsign * (locals.var_qd_v_dn6 * ddt_scale));
        let eq4_e1977_d_n7: f64 = (locals.var_devsign * (locals.var_qd_v_dn7 * ddt_scale));
        let eq4_e1977_d_n8: f64 = (locals.var_devsign * (locals.var_qd_v_dn8 * ddt_scale));
        let eq4_e1977_d_n9: f64 = (locals.var_devsign * (locals.var_qd_v_dn9 * ddt_scale));
        let eq4_e1977_d_n10: f64 = (locals.var_devsign * (locals.var_qd_v_dn10 * ddt_scale));
        let eq4_e1977_d_n11: f64 = (locals.var_devsign * (locals.var_qd_v_dn11 * ddt_scale));
        let eq4_e1977_d_n13: f64 = (locals.var_devsign * (locals.var_qd_v_dn13 * ddt_scale));
        let eq4_e1977_d_n14: f64 = (locals.var_devsign * (locals.var_qd_v_dn14 * ddt_scale));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n13, eq4_e1977_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1979;
        let eq4_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq4_node_derivatives: [f64; 13] = [eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14,) = {
    if (locals.var_guard642 == 0.0) {
        let eq5_e1984: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qg_v);
        let eq5_e1985: f64 = (locals.var_devsign * eq5_e1984);
        let eq5_e1985_d_n0: f64 = (locals.var_devsign * (locals.var_qg_v_dn0 * ddt_scale));
        let eq5_e1985_d_n2: f64 = (locals.var_devsign * (locals.var_qg_v_dn2 * ddt_scale));
        let eq5_e1985_d_n3: f64 = (locals.var_devsign * (locals.var_qg_v_dn3 * ddt_scale));
        let eq5_e1985_d_n4: f64 = (locals.var_devsign * (locals.var_qg_v_dn4 * ddt_scale));
        let eq5_e1985_d_n5: f64 = (locals.var_devsign * (locals.var_qg_v_dn5 * ddt_scale));
        let eq5_e1985_d_n6: f64 = (locals.var_devsign * (locals.var_qg_v_dn6 * ddt_scale));
        let eq5_e1985_d_n7: f64 = (locals.var_devsign * (locals.var_qg_v_dn7 * ddt_scale));
        let eq5_e1985_d_n8: f64 = (locals.var_devsign * (locals.var_qg_v_dn8 * ddt_scale));
        let eq5_e1985_d_n9: f64 = (locals.var_devsign * (locals.var_qg_v_dn9 * ddt_scale));
        let eq5_e1985_d_n10: f64 = (locals.var_devsign * (locals.var_qg_v_dn10 * ddt_scale));
        let eq5_e1985_d_n11: f64 = (locals.var_devsign * (locals.var_qg_v_dn11 * ddt_scale));
        let eq5_e1985_d_n13: f64 = (locals.var_devsign * (locals.var_qg_v_dn13 * ddt_scale));
        let eq5_e1985_d_n14: f64 = (locals.var_devsign * (locals.var_qg_v_dn14 * ddt_scale));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n13, eq5_e1985_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1987;
        let eq5_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq5_node_derivatives: [f64; 13] = [eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq8_e2004, eq8_e2004_d_n0, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n13, eq8_e2004_d_n14,) = {
    if (locals.var_guard644 != 0.0) {
        let eq8_e2002: f64 = (locals.var_devsign * locals.var_idsgen_v);
        let eq8_e2002_d_n0: f64 = (locals.var_devsign * locals.var_idsgen_v_dn0);
        let eq8_e2002_d_n2: f64 = (locals.var_devsign * locals.var_idsgen_v_dn2);
        let eq8_e2002_d_n3: f64 = (locals.var_devsign * locals.var_idsgen_v_dn3);
        let eq8_e2002_d_n4: f64 = (locals.var_devsign * locals.var_idsgen_v_dn4);
        let eq8_e2002_d_n5: f64 = (locals.var_devsign * locals.var_idsgen_v_dn5);
        let eq8_e2002_d_n6: f64 = (locals.var_devsign * locals.var_idsgen_v_dn6);
        let eq8_e2002_d_n7: f64 = (locals.var_devsign * locals.var_idsgen_v_dn7);
        let eq8_e2002_d_n8: f64 = (locals.var_devsign * locals.var_idsgen_v_dn8);
        let eq8_e2002_d_n9: f64 = (locals.var_devsign * locals.var_idsgen_v_dn9);
        let eq8_e2002_d_n10: f64 = (locals.var_devsign * locals.var_idsgen_v_dn10);
        let eq8_e2002_d_n11: f64 = (locals.var_devsign * locals.var_idsgen_v_dn11);
        let eq8_e2002_d_n13: f64 = (locals.var_devsign * locals.var_idsgen_v_dn13);
        let eq8_e2002_d_n14: f64 = (locals.var_devsign * locals.var_idsgen_v_dn14);
        (eq8_e2002, eq8_e2002_d_n0, eq8_e2002_d_n2, eq8_e2002_d_n3, eq8_e2002_d_n4, eq8_e2002_d_n5, eq8_e2002_d_n6, eq8_e2002_d_n7, eq8_e2002_d_n8, eq8_e2002_d_n9, eq8_e2002_d_n10, eq8_e2002_d_n11, eq8_e2002_d_n13, eq8_e2002_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e2004;
        let eq8_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq8_node_derivatives: [f64; 13] = [eq8_e2004_d_n0, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n13, eq8_e2004_d_n14];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e2012, eq9_e2012_d_n0, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n13, eq9_e2012_d_n14,) = {
    if (locals.var_guard644 != 0.0) {
        let eq9_e2009: f64 = (locals.var_igcs_v + locals.var_igs_v);
        let eq9_e2009_d_n0: f64 = (locals.var_igcs_v_dn0 + locals.var_igs_v_dn0);
        let eq9_e2009_d_n2: f64 = (locals.var_igcs_v_dn2 + locals.var_igs_v_dn2);
        let eq9_e2009_d_n3: f64 = (locals.var_igcs_v_dn3 + locals.var_igs_v_dn3);
        let eq9_e2009_d_n4: f64 = (locals.var_igcs_v_dn4 + locals.var_igs_v_dn4);
        let eq9_e2009_d_n5: f64 = (locals.var_igcs_v_dn5 + locals.var_igs_v_dn5);
        let eq9_e2009_d_n6: f64 = (locals.var_igcs_v_dn6 + locals.var_igs_v_dn6);
        let eq9_e2009_d_n7: f64 = (locals.var_igcs_v_dn7 + locals.var_igs_v_dn7);
        let eq9_e2009_d_n8: f64 = (locals.var_igcs_v_dn8 + locals.var_igs_v_dn8);
        let eq9_e2009_d_n9: f64 = (locals.var_igcs_v_dn9 + locals.var_igs_v_dn9);
        let eq9_e2009_d_n10: f64 = (locals.var_igcs_v_dn10 + locals.var_igs_v_dn10);
        let eq9_e2009_d_n11: f64 = (locals.var_igcs_v_dn11 + locals.var_igs_v_dn11);
        let eq9_e2009_d_n13: f64 = (locals.var_igcs_v_dn13 + locals.var_igs_v_dn13);
        let eq9_e2009_d_n14: f64 = (locals.var_igcs_v_dn14 + locals.var_igs_v_dn14);
        let eq9_e2010: f64 = (locals.var_devsign * eq9_e2009);
        let eq9_e2010_d_n0: f64 = (locals.var_devsign * eq9_e2009_d_n0);
        let eq9_e2010_d_n2: f64 = (locals.var_devsign * eq9_e2009_d_n2);
        let eq9_e2010_d_n3: f64 = (locals.var_devsign * eq9_e2009_d_n3);
        let eq9_e2010_d_n4: f64 = (locals.var_devsign * eq9_e2009_d_n4);
        let eq9_e2010_d_n5: f64 = (locals.var_devsign * eq9_e2009_d_n5);
        let eq9_e2010_d_n6: f64 = (locals.var_devsign * eq9_e2009_d_n6);
        let eq9_e2010_d_n7: f64 = (locals.var_devsign * eq9_e2009_d_n7);
        let eq9_e2010_d_n8: f64 = (locals.var_devsign * eq9_e2009_d_n8);
        let eq9_e2010_d_n9: f64 = (locals.var_devsign * eq9_e2009_d_n9);
        let eq9_e2010_d_n10: f64 = (locals.var_devsign * eq9_e2009_d_n10);
        let eq9_e2010_d_n11: f64 = (locals.var_devsign * eq9_e2009_d_n11);
        let eq9_e2010_d_n13: f64 = (locals.var_devsign * eq9_e2009_d_n13);
        let eq9_e2010_d_n14: f64 = (locals.var_devsign * eq9_e2009_d_n14);
        (eq9_e2010, eq9_e2010_d_n0, eq9_e2010_d_n2, eq9_e2010_d_n3, eq9_e2010_d_n4, eq9_e2010_d_n5, eq9_e2010_d_n6, eq9_e2010_d_n7, eq9_e2010_d_n8, eq9_e2010_d_n9, eq9_e2010_d_n10, eq9_e2010_d_n11, eq9_e2010_d_n13, eq9_e2010_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e2012;
        let eq9_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq9_node_derivatives: [f64; 13] = [eq9_e2012_d_n0, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n13, eq9_e2012_d_n14];
        let eq9_branch_derivative_indices: [usize; 0] = [];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivative_indices,
            &eq9_node_derivatives,
            &eq9_branch_derivative_indices,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e2020, eq10_e2020_d_n0, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n13, eq10_e2020_d_n14,) = {
    if (locals.var_guard644 != 0.0) {
        let eq10_e2017: f64 = (locals.var_igcd_v + locals.var_igd_v);
        let eq10_e2017_d_n0: f64 = (locals.var_igcd_v_dn0 + locals.var_igd_v_dn0);
        let eq10_e2017_d_n2: f64 = (locals.var_igcd_v_dn2 + locals.var_igd_v_dn2);
        let eq10_e2017_d_n3: f64 = (locals.var_igcd_v_dn3 + locals.var_igd_v_dn3);
        let eq10_e2017_d_n4: f64 = (locals.var_igcd_v_dn4 + locals.var_igd_v_dn4);
        let eq10_e2017_d_n5: f64 = (locals.var_igcd_v_dn5 + locals.var_igd_v_dn5);
        let eq10_e2017_d_n6: f64 = (locals.var_igcd_v_dn6 + locals.var_igd_v_dn6);
        let eq10_e2017_d_n7: f64 = (locals.var_igcd_v_dn7 + locals.var_igd_v_dn7);
        let eq10_e2017_d_n8: f64 = (locals.var_igcd_v_dn8 + locals.var_igd_v_dn8);
        let eq10_e2017_d_n9: f64 = (locals.var_igcd_v_dn9 + locals.var_igd_v_dn9);
        let eq10_e2017_d_n10: f64 = (locals.var_igcd_v_dn10 + locals.var_igd_v_dn10);
        let eq10_e2017_d_n11: f64 = (locals.var_igcd_v_dn11 + locals.var_igd_v_dn11);
        let eq10_e2017_d_n13: f64 = (locals.var_igcd_v_dn13 + locals.var_igd_v_dn13);
        let eq10_e2017_d_n14: f64 = (locals.var_igcd_v_dn14 + locals.var_igd_v_dn14);
        let eq10_e2018: f64 = (locals.var_devsign * eq10_e2017);
        let eq10_e2018_d_n0: f64 = (locals.var_devsign * eq10_e2017_d_n0);
        let eq10_e2018_d_n2: f64 = (locals.var_devsign * eq10_e2017_d_n2);
        let eq10_e2018_d_n3: f64 = (locals.var_devsign * eq10_e2017_d_n3);
        let eq10_e2018_d_n4: f64 = (locals.var_devsign * eq10_e2017_d_n4);
        let eq10_e2018_d_n5: f64 = (locals.var_devsign * eq10_e2017_d_n5);
        let eq10_e2018_d_n6: f64 = (locals.var_devsign * eq10_e2017_d_n6);
        let eq10_e2018_d_n7: f64 = (locals.var_devsign * eq10_e2017_d_n7);
        let eq10_e2018_d_n8: f64 = (locals.var_devsign * eq10_e2017_d_n8);
        let eq10_e2018_d_n9: f64 = (locals.var_devsign * eq10_e2017_d_n9);
        let eq10_e2018_d_n10: f64 = (locals.var_devsign * eq10_e2017_d_n10);
        let eq10_e2018_d_n11: f64 = (locals.var_devsign * eq10_e2017_d_n11);
        let eq10_e2018_d_n13: f64 = (locals.var_devsign * eq10_e2017_d_n13);
        let eq10_e2018_d_n14: f64 = (locals.var_devsign * eq10_e2017_d_n14);
        (eq10_e2018, eq10_e2018_d_n0, eq10_e2018_d_n2, eq10_e2018_d_n3, eq10_e2018_d_n4, eq10_e2018_d_n5, eq10_e2018_d_n6, eq10_e2018_d_n7, eq10_e2018_d_n8, eq10_e2018_d_n9, eq10_e2018_d_n10, eq10_e2018_d_n11, eq10_e2018_d_n13, eq10_e2018_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e2020;
        let eq10_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq10_node_derivatives: [f64; 13] = [eq10_e2020_d_n0, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n13, eq10_e2020_d_n14];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e2032, eq11_e2032_d_n0, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n13, eq11_e2032_d_n14,) = {
    if (((locals.var_guard644 != 0.0) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) {
        let eq11_e2029: f64 = (locals.var_igidl_v + locals.var_iii_1);
        let eq11_e2029_d_n0: f64 = (locals.var_igidl_v_dn0 + locals.var_iii_1_dn0);
        let eq11_e2029_d_n2: f64 = (locals.var_igidl_v_dn2 + locals.var_iii_1_dn2);
        let eq11_e2029_d_n3: f64 = (locals.var_igidl_v_dn3 + locals.var_iii_1_dn3);
        let eq11_e2029_d_n4: f64 = (locals.var_igidl_v_dn4 + locals.var_iii_1_dn4);
        let eq11_e2029_d_n5: f64 = (locals.var_igidl_v_dn5 + locals.var_iii_1_dn5);
        let eq11_e2029_d_n6: f64 = (locals.var_igidl_v_dn6 + locals.var_iii_1_dn6);
        let eq11_e2029_d_n7: f64 = (locals.var_igidl_v_dn7 + locals.var_iii_1_dn7);
        let eq11_e2029_d_n8: f64 = (locals.var_igidl_v_dn8 + locals.var_iii_1_dn8);
        let eq11_e2029_d_n9: f64 = (locals.var_igidl_v_dn9 + locals.var_iii_1_dn9);
        let eq11_e2029_d_n10: f64 = (locals.var_igidl_v_dn10 + locals.var_iii_1_dn10);
        let eq11_e2029_d_n11: f64 = (locals.var_igidl_v_dn11 + locals.var_iii_1_dn11);
        let eq11_e2029_d_n13: f64 = (locals.var_igidl_v_dn13 + locals.var_iii_1_dn13);
        let eq11_e2029_d_n14: f64 = (locals.var_igidl_v_dn14 + locals.var_iii_1_dn14);
        let eq11_e2030: f64 = (locals.var_devsign * eq11_e2029);
        let eq11_e2030_d_n0: f64 = (locals.var_devsign * eq11_e2029_d_n0);
        let eq11_e2030_d_n2: f64 = (locals.var_devsign * eq11_e2029_d_n2);
        let eq11_e2030_d_n3: f64 = (locals.var_devsign * eq11_e2029_d_n3);
        let eq11_e2030_d_n4: f64 = (locals.var_devsign * eq11_e2029_d_n4);
        let eq11_e2030_d_n5: f64 = (locals.var_devsign * eq11_e2029_d_n5);
        let eq11_e2030_d_n6: f64 = (locals.var_devsign * eq11_e2029_d_n6);
        let eq11_e2030_d_n7: f64 = (locals.var_devsign * eq11_e2029_d_n7);
        let eq11_e2030_d_n8: f64 = (locals.var_devsign * eq11_e2029_d_n8);
        let eq11_e2030_d_n9: f64 = (locals.var_devsign * eq11_e2029_d_n9);
        let eq11_e2030_d_n10: f64 = (locals.var_devsign * eq11_e2029_d_n10);
        let eq11_e2030_d_n11: f64 = (locals.var_devsign * eq11_e2029_d_n11);
        let eq11_e2030_d_n13: f64 = (locals.var_devsign * eq11_e2029_d_n13);
        let eq11_e2030_d_n14: f64 = (locals.var_devsign * eq11_e2029_d_n14);
        (eq11_e2030, eq11_e2030_d_n0, eq11_e2030_d_n2, eq11_e2030_d_n3, eq11_e2030_d_n4, eq11_e2030_d_n5, eq11_e2030_d_n6, eq11_e2030_d_n7, eq11_e2030_d_n8, eq11_e2030_d_n9, eq11_e2030_d_n10, eq11_e2030_d_n11, eq11_e2030_d_n13, eq11_e2030_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e2032;
        let eq11_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq11_node_derivatives: [f64; 13] = [eq11_e2032_d_n0, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n13, eq11_e2032_d_n14];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e2042, eq12_e2042_d_n0, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n13, eq12_e2042_d_n14,) = {
    if (((locals.var_guard644 != 0.0) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) {
        let eq12_e2040: f64 = (locals.var_devsign * locals.var_igisl_v);
        let eq12_e2040_d_n0: f64 = (locals.var_devsign * locals.var_igisl_v_dn0);
        let eq12_e2040_d_n2: f64 = (locals.var_devsign * locals.var_igisl_v_dn2);
        let eq12_e2040_d_n3: f64 = (locals.var_devsign * locals.var_igisl_v_dn3);
        let eq12_e2040_d_n4: f64 = (locals.var_devsign * locals.var_igisl_v_dn4);
        let eq12_e2040_d_n5: f64 = (locals.var_devsign * locals.var_igisl_v_dn5);
        let eq12_e2040_d_n6: f64 = (locals.var_devsign * locals.var_igisl_v_dn6);
        let eq12_e2040_d_n7: f64 = (locals.var_devsign * locals.var_igisl_v_dn7);
        let eq12_e2040_d_n8: f64 = (locals.var_devsign * locals.var_igisl_v_dn8);
        let eq12_e2040_d_n9: f64 = (locals.var_devsign * locals.var_igisl_v_dn9);
        let eq12_e2040_d_n10: f64 = (locals.var_devsign * locals.var_igisl_v_dn10);
        let eq12_e2040_d_n11: f64 = (locals.var_devsign * locals.var_igisl_v_dn11);
        let eq12_e2040_d_n13: f64 = (locals.var_devsign * locals.var_igisl_v_dn13);
        let eq12_e2040_d_n14: f64 = (locals.var_devsign * locals.var_igisl_v_dn14);
        (eq12_e2040, eq12_e2040_d_n0, eq12_e2040_d_n2, eq12_e2040_d_n3, eq12_e2040_d_n4, eq12_e2040_d_n5, eq12_e2040_d_n6, eq12_e2040_d_n7, eq12_e2040_d_n8, eq12_e2040_d_n9, eq12_e2040_d_n10, eq12_e2040_d_n11, eq12_e2040_d_n13, eq12_e2040_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e2042;
        let eq12_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq12_node_derivatives: [f64; 13] = [eq12_e2042_d_n0, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n13, eq12_e2042_d_n14];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }
}
