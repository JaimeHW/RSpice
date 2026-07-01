#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5110_e4874, assign5110_e4874_d_n0, assign5110_e4874_d_n1, assign5110_e4874_d_n3, assign5110_e4874_d_n4, assign5110_e4874_d_n5, assign5110_e4874_d_n6, assign5110_e4874_d_n7, assign5110_e4874_d_n8, assign5110_e4874_d_n9, assign5110_e4874_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign5110_e4867: f64 = (-locals.var_bavl_t);
        let assign5110_e4870: f64 = (locals.var_vl).powf(p.p40);
        let assign5110_e4871: f64 = (assign5110_e4867 * assign5110_e4870);
        let assign5110_e4872: f64 = (assign5110_e4871).exp();
        (assign5110_e4872, (assign5110_e4872 * (((-locals.var_bavl_t_dn0) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn0)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn0 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn1) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn1)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn1 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn3) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn3)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn3 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn4) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn4)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn4 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn5) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn5)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn5 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn6) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn6)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn6 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn7) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn7)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn7 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn8) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn8)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn8 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn9) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn9)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn9 / locals.var_vl))) }))), (assign5110_e4872 * (((-locals.var_bavl_t_dn10) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn10)) } } else { (assign5110_e4870 * (p.p40 * (locals.var_vl_dn10 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5110_e4874;
        locals.var_expmm1_dn0 = assign5110_e4874_d_n0;
        locals.var_expmm1_dn1 = assign5110_e4874_d_n1;
        locals.var_expmm1_dn3 = assign5110_e4874_d_n3;
        locals.var_expmm1_dn4 = assign5110_e4874_d_n4;
        locals.var_expmm1_dn5 = assign5110_e4874_d_n5;
        locals.var_expmm1_dn6 = assign5110_e4874_d_n6;
        locals.var_expmm1_dn7 = assign5110_e4874_d_n7;
        locals.var_expmm1_dn8 = assign5110_e4874_d_n8;
        locals.var_expmm1_dn9 = assign5110_e4874_d_n9;
        locals.var_expmm1_dn10 = assign5110_e4874_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5120_e4886,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard89 == 0.0)) {
        let assign5120_e4884: f64 = (p.p138).exp();
        (assign5120_e4884,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5120_e4886;
        locals.var_expl_rv = 0.0;

        let (assign5130_e4908, assign5130_e4908_d_n0, assign5130_e4908_d_n1, assign5130_e4908_d_n3, assign5130_e4908_d_n4, assign5130_e4908_d_n5, assign5130_e4908_d_n6, assign5130_e4908_d_n7, assign5130_e4908_d_n8, assign5130_e4908_d_n9, assign5130_e4908_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) && (locals.var_guard89 == 0.0)) {
        let assign5130_e4898: f64 = (-locals.var_bavl_t);
        let assign5130_e4901: f64 = (locals.var_vl).powf(p.p40);
        let assign5130_e4902: f64 = (assign5130_e4898 * assign5130_e4901);
        let assign5130_e4904: f64 = (assign5130_e4902 - p.p138);
        let assign5130_e4905: f64 = (1.0 + assign5130_e4904);
        let assign5130_e4906: f64 = (locals.var_expl * assign5130_e4905);
        (assign5130_e4906, (locals.var_expl * (((-locals.var_bavl_t_dn0) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn0)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn0 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn1)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn1 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn3)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn3 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn4)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn4 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn5)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn5 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn6)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn6 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn7)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn7 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn8)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn8 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn9)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn9 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((locals.var_vl).powf(p.p40 - 1.0) * locals.var_vl_dn10)) } } else { (assign5130_e4901 * (p.p40 * (locals.var_vl_dn10 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5130_e4908;
        locals.var_expmm1_dn0 = assign5130_e4908_d_n0;
        locals.var_expmm1_dn1 = assign5130_e4908_d_n1;
        locals.var_expmm1_dn3 = assign5130_e4908_d_n3;
        locals.var_expmm1_dn4 = assign5130_e4908_d_n4;
        locals.var_expmm1_dn5 = assign5130_e4908_d_n5;
        locals.var_expmm1_dn6 = assign5130_e4908_d_n6;
        locals.var_expmm1_dn7 = assign5130_e4908_d_n7;
        locals.var_expmm1_dn8 = assign5130_e4908_d_n8;
        locals.var_expmm1_dn9 = assign5130_e4908_d_n9;
        locals.var_expmm1_dn10 = assign5130_e4908_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5140_e4922, assign5140_e4922_d_n0, assign5140_e4922_d_n1, assign5140_e4922_d_n3, assign5140_e4922_d_n4, assign5140_e4922_d_n5, assign5140_e4922_d_n6, assign5140_e4922_d_n7, assign5140_e4922_d_n8, assign5140_e4922_d_n9, assign5140_e4922_d_n10,) = {
    if (((locals.var_guard85 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) {
        let assign5140_e4916: f64 = (p.p39 / locals.var_bavl_t);
        let assign5140_e4918: f64 = (assign5140_e4916 * locals.var_vl);
        let assign5140_e4920: f64 = (assign5140_e4918 * locals.var_expmm1);
        (assign5140_e4920, (((((-((p.p39 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn0)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn0)), (((((-((p.p39 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn1)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn1)), (((((-((p.p39 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn3)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn3)), (((((-((p.p39 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn4)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn4)), (((((-((p.p39 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn5)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn5)), (((((-((p.p39 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn6)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn6)), (((((-((p.p39 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn7)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn7)), (((((-((p.p39 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn8)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn8)), (((((-((p.p39 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn9)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn9)), (((((-((p.p39 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5140_e4916 * locals.var_vl_dn10)) * locals.var_expmm1) + (assign5140_e4918 * locals.var_expmm1_dn10)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5140_e4922;
        locals.var_gem_dn0 = assign5140_e4922_d_n0;
        locals.var_gem_dn1 = assign5140_e4922_d_n1;
        locals.var_gem_dn3 = assign5140_e4922_d_n3;
        locals.var_gem_dn4 = assign5140_e4922_d_n4;
        locals.var_gem_dn5 = assign5140_e4922_d_n5;
        locals.var_gem_dn6 = assign5140_e4922_d_n6;
        locals.var_gem_dn7 = assign5140_e4922_d_n7;
        locals.var_gem_dn8 = assign5140_e4922_d_n8;
        locals.var_gem_dn9 = assign5140_e4922_d_n9;
        locals.var_gem_dn10 = assign5140_e4922_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5150_e4925: f64 = if p.p38 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5150_e4925;
        locals.var_guard90_rv = 0.0;

        let assign5160_e4928: f64 = if locals.var_vb2c1 < locals.var_vdc_t { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5160_e4928;
        locals.var_guard91_rv = 0.0;

        let (assign5170_e4945,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5170_e4939: f64 = (2.0 * p.p45);
        let assign5170_e4942: f64 = (p.p44 * p.p44);
        let assign5170_e4943: f64 = (assign5170_e4939 / assign5170_e4942);
        (assign5170_e4943,)
    } else {
        (locals.var_dedx0,)
    }
};
        locals.var_dedx0 = assign5170_e4945;
        locals.var_dedx0_rv = 0.0;

        let (assign5180_e4960, assign5180_e4960_d_n0, assign5180_e4960_d_n1, assign5180_e4960_d_n3, assign5180_e4960_d_n4, assign5180_e4960_d_n5, assign5180_e4960_d_n6, assign5180_e4960_d_n7, assign5180_e4960_d_n8, assign5180_e4960_d_n9, assign5180_e4960_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5180_e4956: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5180_e4958: f64 = (assign5180_e4956 / locals.var_icap_ihc);
        (assign5180_e4958, (((locals.var_vdc_t_dn0 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn0)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn1 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn1)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn3 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn3)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn4 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn4)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn5 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn5)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn6 - locals.var_vb2c1_dn6) * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn6)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn7)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn8 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn8)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn9 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn9)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn10 * locals.var_icap_ihc) - (assign5180_e4956 * locals.var_icap_ihc_dn10)) / (locals.var_icap_ihc * locals.var_icap_ihc)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10,)
    }
};
        locals.var_sqr_arg = assign5180_e4960;
        locals.var_sqr_arg_dn0 = assign5180_e4960_d_n0;
        locals.var_sqr_arg_dn1 = assign5180_e4960_d_n1;
        locals.var_sqr_arg_dn3 = assign5180_e4960_d_n3;
        locals.var_sqr_arg_dn4 = assign5180_e4960_d_n4;
        locals.var_sqr_arg_dn5 = assign5180_e4960_d_n5;
        locals.var_sqr_arg_dn6 = assign5180_e4960_d_n6;
        locals.var_sqr_arg_dn7 = assign5180_e4960_d_n7;
        locals.var_sqr_arg_dn8 = assign5180_e4960_d_n8;
        locals.var_sqr_arg_dn9 = assign5180_e4960_d_n9;
        locals.var_sqr_arg_dn10 = assign5180_e4960_d_n10;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5190_e4976, assign5190_e4976_d_n0, assign5190_e4976_d_n1, assign5190_e4976_d_n3, assign5190_e4976_d_n4, assign5190_e4976_d_n5, assign5190_e4976_d_n6, assign5190_e4976_d_n7, assign5190_e4976_d_n8, assign5190_e4976_d_n9, assign5190_e4976_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5190_e4971: f64 = (2.0 * locals.var_sqr_arg);
        let assign5190_e4973: f64 = (assign5190_e4971 / locals.var_dedx0);
        let assign5190_e4974: f64 = (assign5190_e4973).sqrt();
        (assign5190_e4974, (((2.0 * locals.var_sqr_arg_dn0) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn1) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn3) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn4) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn5) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn6) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn7) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn8) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn9) / locals.var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * locals.var_sqr_arg_dn10) / locals.var_dedx0) / (2.0 * assign5190_e4974)),)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn1, locals.var_xd_dn3, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10,)
    }
};
        locals.var_xd = assign5190_e4976;
        locals.var_xd_dn0 = assign5190_e4976_d_n0;
        locals.var_xd_dn1 = assign5190_e4976_d_n1;
        locals.var_xd_dn3 = assign5190_e4976_d_n3;
        locals.var_xd_dn4 = assign5190_e4976_d_n4;
        locals.var_xd_dn5 = assign5190_e4976_d_n5;
        locals.var_xd_dn6 = assign5190_e4976_d_n6;
        locals.var_xd_dn7 = assign5190_e4976_d_n7;
        locals.var_xd_dn8 = assign5190_e4976_d_n8;
        locals.var_xd_dn9 = assign5190_e4976_d_n9;
        locals.var_xd_dn10 = assign5190_e4976_d_n10;
        locals.var_xd_rv = 0.0;

        let assign5200_e4979: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5200_e4979;
        locals.var_guard92_rv = 0.0;

        let (assign5210_e4992, assign5210_e4992_d_n0, assign5210_e4992_d_n1, assign5210_e4992_d_n3, assign5210_e4992_d_n4, assign5210_e4992_d_n5, assign5210_e4992_d_n6, assign5210_e4992_d_n7, assign5210_e4992_d_n8, assign5210_e4992_d_n9, assign5210_e4992_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard92 != 0.0)) {
        (p.p44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10,)
    }
};
        locals.var_weff = assign5210_e4992;
        locals.var_weff_dn0 = assign5210_e4992_d_n0;
        locals.var_weff_dn1 = assign5210_e4992_d_n1;
        locals.var_weff_dn3 = assign5210_e4992_d_n3;
        locals.var_weff_dn4 = assign5210_e4992_d_n4;
        locals.var_weff_dn5 = assign5210_e4992_d_n5;
        locals.var_weff_dn6 = assign5210_e4992_d_n6;
        locals.var_weff_dn7 = assign5210_e4992_d_n7;
        locals.var_weff_dn8 = assign5210_e4992_d_n8;
        locals.var_weff_dn9 = assign5210_e4992_d_n9;
        locals.var_weff_dn10 = assign5210_e4992_d_n10;
        locals.var_weff_rv = 0.0;

        let (assign5220_e5010, assign5220_e5010_d_n0, assign5220_e5010_d_n1, assign5220_e5010_d_n3, assign5220_e5010_d_n4, assign5220_e5010_d_n5, assign5220_e5010_d_n6, assign5220_e5010_d_n7, assign5220_e5010_d_n8, assign5220_e5010_d_n9, assign5220_e5010_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard92 == 0.0)) {
        let assign5220_e5007: f64 = (0.5 * locals.var_xi_w);
        let assign5220_e5008: f64 = (1.0 - assign5220_e5007);
        (assign5220_e5008, (-(0.5 * locals.var_xi_w_dn0)), (-(0.5 * locals.var_xi_w_dn1)), (-(0.5 * locals.var_xi_w_dn3)), (-(0.5 * locals.var_xi_w_dn4)), (-(0.5 * locals.var_xi_w_dn5)), (-(0.5 * locals.var_xi_w_dn6)), (-(0.5 * locals.var_xi_w_dn7)), (-(0.5 * locals.var_xi_w_dn8)), (-(0.5 * locals.var_xi_w_dn9)), (-(0.5 * locals.var_xi_w_dn10)),)
    } else {
        (locals.var_xi_w1, locals.var_xi_w1_dn0, locals.var_xi_w1_dn1, locals.var_xi_w1_dn3, locals.var_xi_w1_dn4, locals.var_xi_w1_dn5, locals.var_xi_w1_dn6, locals.var_xi_w1_dn7, locals.var_xi_w1_dn8, locals.var_xi_w1_dn9, locals.var_xi_w1_dn10,)
    }
};
        locals.var_xi_w1 = assign5220_e5010;
        locals.var_xi_w1_dn0 = assign5220_e5010_d_n0;
        locals.var_xi_w1_dn1 = assign5220_e5010_d_n1;
        locals.var_xi_w1_dn3 = assign5220_e5010_d_n3;
        locals.var_xi_w1_dn4 = assign5220_e5010_d_n4;
        locals.var_xi_w1_dn5 = assign5220_e5010_d_n5;
        locals.var_xi_w1_dn6 = assign5220_e5010_d_n6;
        locals.var_xi_w1_dn7 = assign5220_e5010_d_n7;
        locals.var_xi_w1_dn8 = assign5220_e5010_d_n8;
        locals.var_xi_w1_dn9 = assign5220_e5010_d_n9;
        locals.var_xi_w1_dn10 = assign5220_e5010_d_n10;
        locals.var_xi_w1_rv = 0.0;

        let (assign5230_e5028, assign5230_e5028_d_n0, assign5230_e5028_d_n1, assign5230_e5028_d_n3, assign5230_e5028_d_n4, assign5230_e5028_d_n5, assign5230_e5028_d_n6, assign5230_e5028_d_n7, assign5230_e5028_d_n8, assign5230_e5028_d_n9, assign5230_e5028_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard92 == 0.0)) {
        let assign5230_e5024: f64 = (p.p44 * locals.var_xi_w1);
        let assign5230_e5026: f64 = (assign5230_e5024 * locals.var_xi_w1);
        (assign5230_e5026, (((p.p44 * locals.var_xi_w1_dn0) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn0)), (((p.p44 * locals.var_xi_w1_dn1) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn1)), (((p.p44 * locals.var_xi_w1_dn3) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn3)), (((p.p44 * locals.var_xi_w1_dn4) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn4)), (((p.p44 * locals.var_xi_w1_dn5) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn5)), (((p.p44 * locals.var_xi_w1_dn6) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn6)), (((p.p44 * locals.var_xi_w1_dn7) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn7)), (((p.p44 * locals.var_xi_w1_dn8) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn8)), (((p.p44 * locals.var_xi_w1_dn9) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn9)), (((p.p44 * locals.var_xi_w1_dn10) * locals.var_xi_w1) + (assign5230_e5024 * locals.var_xi_w1_dn10)),)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10,)
    }
};
        locals.var_weff = assign5230_e5028;
        locals.var_weff_dn0 = assign5230_e5028_d_n0;
        locals.var_weff_dn1 = assign5230_e5028_d_n1;
        locals.var_weff_dn3 = assign5230_e5028_d_n3;
        locals.var_weff_dn4 = assign5230_e5028_d_n4;
        locals.var_weff_dn5 = assign5230_e5028_d_n5;
        locals.var_weff_dn6 = assign5230_e5028_d_n6;
        locals.var_weff_dn7 = assign5230_e5028_d_n7;
        locals.var_weff_dn8 = assign5230_e5028_d_n8;
        locals.var_weff_dn9 = assign5230_e5028_d_n9;
        locals.var_weff_dn10 = assign5230_e5028_d_n10;
        locals.var_weff_rv = 0.0;

        let (assign5240_e5050, assign5240_e5050_d_n0, assign5240_e5050_d_n1, assign5240_e5050_d_n3, assign5240_e5050_d_n4, assign5240_e5050_d_n5, assign5240_e5050_d_n6, assign5240_e5050_d_n7, assign5240_e5050_d_n8, assign5240_e5050_d_n9, assign5240_e5050_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5240_e5039: f64 = (locals.var_xd * locals.var_weff);
        let assign5240_e5042: f64 = (locals.var_xd * locals.var_xd);
        let assign5240_e5045: f64 = (locals.var_weff * locals.var_weff);
        let assign5240_e5046: f64 = (assign5240_e5042 + assign5240_e5045);
        let assign5240_e5047: f64 = (assign5240_e5046).sqrt();
        let assign5240_e5048: f64 = (assign5240_e5039 / assign5240_e5047);
        (assign5240_e5048, (((((locals.var_xd_dn0 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn0)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn0 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn0)) + ((locals.var_weff_dn0 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn0))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn1 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn1)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn1 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn1)) + ((locals.var_weff_dn1 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn1))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn3 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn3)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn3 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn3)) + ((locals.var_weff_dn3 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn3))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn4 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn4)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn4 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn4)) + ((locals.var_weff_dn4 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn4))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn5 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn5)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn5 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn5)) + ((locals.var_weff_dn5 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn5))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn6 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn6)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn6 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn6)) + ((locals.var_weff_dn6 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn6))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn7 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn7)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn7 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn7)) + ((locals.var_weff_dn7 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn7))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn8 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn8)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn8 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn8)) + ((locals.var_weff_dn8 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn8))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn9 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn9)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn9 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn9)) + ((locals.var_weff_dn9 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn9))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((locals.var_xd_dn10 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn10)) * assign5240_e5047) - (assign5240_e5039 * ((((locals.var_xd_dn10 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn10)) + ((locals.var_weff_dn10 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn10))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn1, locals.var_wd_dn3, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10,)
    }
};
        locals.var_wd = assign5240_e5050;
        locals.var_wd_dn0 = assign5240_e5050_d_n0;
        locals.var_wd_dn1 = assign5240_e5050_d_n1;
        locals.var_wd_dn3 = assign5240_e5050_d_n3;
        locals.var_wd_dn4 = assign5240_e5050_d_n4;
        locals.var_wd_dn5 = assign5240_e5050_d_n5;
        locals.var_wd_dn6 = assign5240_e5050_d_n6;
        locals.var_wd_dn7 = assign5240_e5050_d_n7;
        locals.var_wd_dn8 = assign5240_e5050_d_n8;
        locals.var_wd_dn9 = assign5240_e5050_d_n9;
        locals.var_wd_dn10 = assign5240_e5050_d_n10;
        locals.var_wd_rv = 0.0;

        let (assign5250_e5065, assign5250_e5065_d_n0, assign5250_e5065_d_n1, assign5250_e5065_d_n3, assign5250_e5065_d_n4, assign5250_e5065_d_n5, assign5250_e5065_d_n6, assign5250_e5065_d_n7, assign5250_e5065_d_n8, assign5250_e5065_d_n9, assign5250_e5065_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5250_e5061: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5250_e5063: f64 = (assign5250_e5061 / locals.var_wd);
        (assign5250_e5063, (((locals.var_vdc_t_dn0 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn0)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn1 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn1)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn3 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn3)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn4 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn4)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn5 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn5)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn6 - locals.var_vb2c1_dn6) * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn6)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn7)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn8 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn8)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn9 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn9)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn10 * locals.var_wd) - (assign5250_e5061 * locals.var_wd_dn10)) / (locals.var_wd * locals.var_wd)),)
    } else {
        (locals.var_eav, locals.var_eav_dn0, locals.var_eav_dn1, locals.var_eav_dn3, locals.var_eav_dn4, locals.var_eav_dn5, locals.var_eav_dn6, locals.var_eav_dn7, locals.var_eav_dn8, locals.var_eav_dn9, locals.var_eav_dn10,)
    }
};
        locals.var_eav = assign5250_e5065;
        locals.var_eav_dn0 = assign5250_e5065_d_n0;
        locals.var_eav_dn1 = assign5250_e5065_d_n1;
        locals.var_eav_dn3 = assign5250_e5065_d_n3;
        locals.var_eav_dn4 = assign5250_e5065_d_n4;
        locals.var_eav_dn5 = assign5250_e5065_d_n5;
        locals.var_eav_dn6 = assign5250_e5065_d_n6;
        locals.var_eav_dn7 = assign5250_e5065_d_n7;
        locals.var_eav_dn8 = assign5250_e5065_d_n8;
        locals.var_eav_dn9 = assign5250_e5065_d_n9;
        locals.var_eav_dn10 = assign5250_e5065_d_n10;
        locals.var_eav_rv = 0.0;

        let (assign5260_e5084, assign5260_e5084_d_n0, assign5260_e5084_d_n1, assign5260_e5084_d_n3, assign5260_e5084_d_n4, assign5260_e5084_d_n5, assign5260_e5084_d_n6, assign5260_e5084_d_n7, assign5260_e5084_d_n8, assign5260_e5084_d_n9, assign5260_e5084_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5260_e5077: f64 = (0.5 * locals.var_wd);
        let assign5260_e5079: f64 = (assign5260_e5077 * locals.var_dedx0);
        let assign5260_e5081: f64 = (assign5260_e5079 * locals.var_icap_ihc);
        let assign5260_e5082: f64 = (locals.var_eav + assign5260_e5081);
        (assign5260_e5082, (locals.var_eav_dn0 + ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn0))), (locals.var_eav_dn1 + ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn1))), (locals.var_eav_dn3 + ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn3))), (locals.var_eav_dn4 + ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn4))), (locals.var_eav_dn5 + ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn5))), (locals.var_eav_dn6 + ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn6))), (locals.var_eav_dn7 + ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn7))), (locals.var_eav_dn8 + ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn8))), (locals.var_eav_dn9 + ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn9))), (locals.var_eav_dn10 + ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5260_e5079 * locals.var_icap_ihc_dn10))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10,)
    }
};
        locals.var_e0 = assign5260_e5084;
        locals.var_e0_dn0 = assign5260_e5084_d_n0;
        locals.var_e0_dn1 = assign5260_e5084_d_n1;
        locals.var_e0_dn3 = assign5260_e5084_d_n3;
        locals.var_e0_dn4 = assign5260_e5084_d_n4;
        locals.var_e0_dn5 = assign5260_e5084_d_n5;
        locals.var_e0_dn6 = assign5260_e5084_d_n6;
        locals.var_e0_dn7 = assign5260_e5084_d_n7;
        locals.var_e0_dn8 = assign5260_e5084_d_n8;
        locals.var_e0_dn9 = assign5260_e5084_d_n9;
        locals.var_e0_dn10 = assign5260_e5084_d_n10;
        locals.var_e0_rv = 0.0;

        let assign5270_e5087: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign5270_e5087;
        locals.var_guard93_rv = 0.0;

        let (assign5280_e5100, assign5280_e5100_d_n0, assign5280_e5100_d_n1, assign5280_e5100_d_n3, assign5280_e5100_d_n4, assign5280_e5100_d_n5, assign5280_e5100_d_n6, assign5280_e5100_d_n7, assign5280_e5100_d_n8, assign5280_e5100_d_n9, assign5280_e5100_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 != 0.0)) {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10,)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10,)
    }
};
        locals.var_em = assign5280_e5100;
        locals.var_em_dn0 = assign5280_e5100_d_n0;
        locals.var_em_dn1 = assign5280_e5100_d_n1;
        locals.var_em_dn3 = assign5280_e5100_d_n3;
        locals.var_em_dn4 = assign5280_e5100_d_n4;
        locals.var_em_dn5 = assign5280_e5100_d_n5;
        locals.var_em_dn6 = assign5280_e5100_d_n6;
        locals.var_em_dn7 = assign5280_e5100_d_n7;
        locals.var_em_dn8 = assign5280_e5100_d_n8;
        locals.var_em_dn9 = assign5280_e5100_d_n9;
        locals.var_em_dn10 = assign5280_e5100_d_n10;
        locals.var_em_rv = 0.0;

        let (assign5290_e5124, assign5290_e5124_d_n0, assign5290_e5124_d_n1, assign5290_e5124_d_n3, assign5290_e5124_d_n4, assign5290_e5124_d_n5, assign5290_e5124_d_n6, assign5290_e5124_d_n7, assign5290_e5124_d_n8, assign5290_e5124_d_n9, assign5290_e5124_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5290_e5115: f64 = (2.0 * p.p46);
        let assign5290_e5119: f64 = (2.0 * locals.var_xi_w);
        let assign5290_e5120: f64 = (1.0 + assign5290_e5119);
        let assign5290_e5121: f64 = (assign5290_e5115 * assign5290_e5120);
        let assign5290_e5122: f64 = (1.0 + assign5290_e5121);
        (assign5290_e5122, (assign5290_e5115 * (2.0 * locals.var_xi_w_dn0)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn1)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn3)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn4)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn5)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn6)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn7)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn8)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn9)), (assign5290_e5115 * (2.0 * locals.var_xi_w_dn10)),)
    } else {
        (locals.var_shw, locals.var_shw_dn0, locals.var_shw_dn1, locals.var_shw_dn3, locals.var_shw_dn4, locals.var_shw_dn5, locals.var_shw_dn6, locals.var_shw_dn7, locals.var_shw_dn8, locals.var_shw_dn9, locals.var_shw_dn10,)
    }
};
        locals.var_shw = assign5290_e5124;
        locals.var_shw_dn0 = assign5290_e5124_d_n0;
        locals.var_shw_dn1 = assign5290_e5124_d_n1;
        locals.var_shw_dn3 = assign5290_e5124_d_n3;
        locals.var_shw_dn4 = assign5290_e5124_d_n4;
        locals.var_shw_dn5 = assign5290_e5124_d_n5;
        locals.var_shw_dn6 = assign5290_e5124_d_n6;
        locals.var_shw_dn7 = assign5290_e5124_d_n7;
        locals.var_shw_dn8 = assign5290_e5124_d_n8;
        locals.var_shw_dn9 = assign5290_e5124_d_n9;
        locals.var_shw_dn10 = assign5290_e5124_d_n10;
        locals.var_shw_rv = 0.0;

        let (assign5300_e5146,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5300_e5138: f64 = (1.0 + p.p46);
        let assign5300_e5142: f64 = (2.0 * p.p46);
        let assign5300_e5143: f64 = (1.0 + assign5300_e5142);
        let assign5300_e5144: f64 = (assign5300_e5138 / assign5300_e5143);
        (assign5300_e5144,)
    } else {
        (locals.var_efi,)
    }
};
        locals.var_efi = assign5300_e5146;
        locals.var_efi_rv = 0.0;

        let (assign5310_e5174, assign5310_e5174_d_n0, assign5310_e5174_d_n1, assign5310_e5174_d_n3, assign5310_e5174_d_n4, assign5310_e5174_d_n5, assign5310_e5174_d_n6, assign5310_e5174_d_n7, assign5310_e5174_d_n8, assign5310_e5174_d_n9, assign5310_e5174_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5310_e5161: f64 = (0.5 * locals.var_wd);
        let assign5310_e5163: f64 = (assign5310_e5161 * locals.var_dedx0);
        let assign5310_e5168: f64 = (p.p61 * locals.var_shw);
        let assign5310_e5169: f64 = (locals.var_in_ / assign5310_e5168);
        let assign5310_e5170: f64 = (locals.var_efi - assign5310_e5169);
        let assign5310_e5171: f64 = (assign5310_e5163 * assign5310_e5170);
        let assign5310_e5172: f64 = (locals.var_eav - assign5310_e5171);
        (assign5310_e5172, (locals.var_eav_dn0 - ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn0 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn0))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn1 - ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn1 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn1))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn3 - ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn3 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn3))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn4 - ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn4 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn4))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn5 - ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn5 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn5))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn6 - ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn6 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn6))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn7 - ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn7 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn7))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn8 - ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn8 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn8))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn9 - ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn9 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn9))) / (assign5310_e5168 * assign5310_e5168)))))), (locals.var_eav_dn10 - ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((locals.var_in__dn10 * assign5310_e5168) - (locals.var_in_ * (p.p61 * locals.var_shw_dn10))) / (assign5310_e5168 * assign5310_e5168)))))),)
    } else {
        (locals.var_ew, locals.var_ew_dn0, locals.var_ew_dn1, locals.var_ew_dn3, locals.var_ew_dn4, locals.var_ew_dn5, locals.var_ew_dn6, locals.var_ew_dn7, locals.var_ew_dn8, locals.var_ew_dn9, locals.var_ew_dn10,)
    }
};
        locals.var_ew = assign5310_e5174;
        locals.var_ew_dn0 = assign5310_e5174_d_n0;
        locals.var_ew_dn1 = assign5310_e5174_d_n1;
        locals.var_ew_dn3 = assign5310_e5174_d_n3;
        locals.var_ew_dn4 = assign5310_e5174_d_n4;
        locals.var_ew_dn5 = assign5310_e5174_d_n5;
        locals.var_ew_dn6 = assign5310_e5174_d_n6;
        locals.var_ew_dn7 = assign5310_e5174_d_n7;
        locals.var_ew_dn8 = assign5310_e5174_d_n8;
        locals.var_ew_dn9 = assign5310_e5174_d_n9;
        locals.var_ew_dn10 = assign5310_e5174_d_n10;
        locals.var_ew_rv = 0.0;

        let (assign5320_e5204, assign5320_e5204_d_n0, assign5320_e5204_d_n1, assign5320_e5204_d_n3, assign5320_e5204_d_n4, assign5320_e5204_d_n5, assign5320_e5204_d_n6, assign5320_e5204_d_n7, assign5320_e5204_d_n8, assign5320_e5204_d_n9, assign5320_e5204_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5320_e5188: f64 = (locals.var_ew - locals.var_e0);
        let assign5320_e5191: f64 = (locals.var_ew - locals.var_e0);
        let assign5320_e5192: f64 = (assign5320_e5188 * assign5320_e5191);
        let assign5320_e5195: f64 = (0.1 * locals.var_eav);
        let assign5320_e5197: f64 = (assign5320_e5195 * locals.var_eav);
        let assign5320_e5199: f64 = (assign5320_e5197 * locals.var_icap);
        let assign5320_e5201: f64 = (assign5320_e5199 / p.p61);
        let assign5320_e5202: f64 = (assign5320_e5192 + assign5320_e5201);
        (assign5320_e5202, ((((locals.var_ew_dn0 - locals.var_e0_dn0) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn0 - locals.var_e0_dn0))) + ((((((0.1 * locals.var_eav_dn0) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn0)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn0)) / p.p61)), ((((locals.var_ew_dn1 - locals.var_e0_dn1) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn1 - locals.var_e0_dn1))) + ((((((0.1 * locals.var_eav_dn1) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn1)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn1)) / p.p61)), ((((locals.var_ew_dn3 - locals.var_e0_dn3) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn3 - locals.var_e0_dn3))) + ((((((0.1 * locals.var_eav_dn3) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn3)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn3)) / p.p61)), ((((locals.var_ew_dn4 - locals.var_e0_dn4) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn4 - locals.var_e0_dn4))) + ((((((0.1 * locals.var_eav_dn4) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn4)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn4)) / p.p61)), ((((locals.var_ew_dn5 - locals.var_e0_dn5) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn5 - locals.var_e0_dn5))) + ((((((0.1 * locals.var_eav_dn5) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn5)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn5)) / p.p61)), ((((locals.var_ew_dn6 - locals.var_e0_dn6) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn6 - locals.var_e0_dn6))) + ((((((0.1 * locals.var_eav_dn6) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn6)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn6)) / p.p61)), ((((locals.var_ew_dn7 - locals.var_e0_dn7) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn7 - locals.var_e0_dn7))) + ((((((0.1 * locals.var_eav_dn7) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn7)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn7)) / p.p61)), ((((locals.var_ew_dn8 - locals.var_e0_dn8) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn8 - locals.var_e0_dn8))) + ((((((0.1 * locals.var_eav_dn8) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn8)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn8)) / p.p61)), ((((locals.var_ew_dn9 - locals.var_e0_dn9) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn9 - locals.var_e0_dn9))) + ((((((0.1 * locals.var_eav_dn9) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn9)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn9)) / p.p61)), ((((locals.var_ew_dn10 - locals.var_e0_dn10) * assign5320_e5191) + (assign5320_e5188 * (locals.var_ew_dn10 - locals.var_e0_dn10))) + ((((((0.1 * locals.var_eav_dn10) * locals.var_eav) + (assign5320_e5195 * locals.var_eav_dn10)) * locals.var_icap) + (assign5320_e5197 * locals.var_icap_dn10)) / p.p61)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10,)
    }
};
        locals.var_sqr_arg = assign5320_e5204;
        locals.var_sqr_arg_dn0 = assign5320_e5204_d_n0;
        locals.var_sqr_arg_dn1 = assign5320_e5204_d_n1;
        locals.var_sqr_arg_dn3 = assign5320_e5204_d_n3;
        locals.var_sqr_arg_dn4 = assign5320_e5204_d_n4;
        locals.var_sqr_arg_dn5 = assign5320_e5204_d_n5;
        locals.var_sqr_arg_dn6 = assign5320_e5204_d_n6;
        locals.var_sqr_arg_dn7 = assign5320_e5204_d_n7;
        locals.var_sqr_arg_dn8 = assign5320_e5204_d_n8;
        locals.var_sqr_arg_dn9 = assign5320_e5204_d_n9;
        locals.var_sqr_arg_dn10 = assign5320_e5204_d_n10;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5330_e5225, assign5330_e5225_d_n0, assign5330_e5225_d_n1, assign5330_e5225_d_n3, assign5330_e5225_d_n4, assign5330_e5225_d_n5, assign5330_e5225_d_n6, assign5330_e5225_d_n7, assign5330_e5225_d_n8, assign5330_e5225_d_n9, assign5330_e5225_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard93 == 0.0)) {
        let assign5330_e5219: f64 = (locals.var_ew + locals.var_e0);
        let assign5330_e5221: f64 = (locals.var_sqr_arg).sqrt();
        let assign5330_e5222: f64 = (assign5330_e5219 + assign5330_e5221);
        let assign5330_e5223: f64 = (0.5 * assign5330_e5222);
        (assign5330_e5223, (0.5 * ((locals.var_ew_dn0 + locals.var_e0_dn0) + (locals.var_sqr_arg_dn0 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn1 + locals.var_e0_dn1) + (locals.var_sqr_arg_dn1 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn3 + locals.var_e0_dn3) + (locals.var_sqr_arg_dn3 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn4 + locals.var_e0_dn4) + (locals.var_sqr_arg_dn4 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn5 + locals.var_e0_dn5) + (locals.var_sqr_arg_dn5 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn6 + locals.var_e0_dn6) + (locals.var_sqr_arg_dn6 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn7 + locals.var_e0_dn7) + (locals.var_sqr_arg_dn7 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn8 + locals.var_e0_dn8) + (locals.var_sqr_arg_dn8 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn9 + locals.var_e0_dn9) + (locals.var_sqr_arg_dn9 / (2.0 * assign5330_e5221)))), (0.5 * ((locals.var_ew_dn10 + locals.var_e0_dn10) + (locals.var_sqr_arg_dn10 / (2.0 * assign5330_e5221)))),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10,)
    }
};
        locals.var_em = assign5330_e5225;
        locals.var_em_dn0 = assign5330_e5225_d_n0;
        locals.var_em_dn1 = assign5330_e5225_d_n1;
        locals.var_em_dn3 = assign5330_e5225_d_n3;
        locals.var_em_dn4 = assign5330_e5225_d_n4;
        locals.var_em_dn5 = assign5330_e5225_d_n5;
        locals.var_em_dn6 = assign5330_e5225_d_n6;
        locals.var_em_dn7 = assign5330_e5225_d_n7;
        locals.var_em_dn8 = assign5330_e5225_d_n8;
        locals.var_em_dn9 = assign5330_e5225_d_n9;
        locals.var_em_dn10 = assign5330_e5225_d_n10;
        locals.var_em_rv = 0.0;

        let (assign5340_e5240, assign5340_e5240_d_n0, assign5340_e5240_d_n1, assign5340_e5240_d_n3, assign5340_e5240_d_n4, assign5340_e5240_d_n5, assign5340_e5240_d_n6, assign5340_e5240_d_n7, assign5340_e5240_d_n8, assign5340_e5240_d_n9, assign5340_e5240_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign5340_e5236: f64 = (locals.var_em - locals.var_eav);
        let assign5340_e5238: f64 = (assign5340_e5236 / locals.var_em);
        (assign5340_e5238, ((((locals.var_em_dn0 - locals.var_eav_dn0) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn0)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn1 - locals.var_eav_dn1) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn1)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn3 - locals.var_eav_dn3) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn3)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn4 - locals.var_eav_dn4) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn4)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn5 - locals.var_eav_dn5) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn5)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn6 - locals.var_eav_dn6) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn6)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn7 - locals.var_eav_dn7) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn7)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn8 - locals.var_eav_dn8) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn8)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn9 - locals.var_eav_dn9) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn9)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn10 - locals.var_eav_dn10) * locals.var_em) - (assign5340_e5236 * locals.var_em_dn10)) / (locals.var_em * locals.var_em)),)
    } else {
        (locals.var_emeav_em, locals.var_emeav_em_dn0, locals.var_emeav_em_dn1, locals.var_emeav_em_dn3, locals.var_emeav_em_dn4, locals.var_emeav_em_dn5, locals.var_emeav_em_dn6, locals.var_emeav_em_dn7, locals.var_emeav_em_dn8, locals.var_emeav_em_dn9, locals.var_emeav_em_dn10,)
    }
};
        locals.var_emeav_em = assign5340_e5240;
        locals.var_emeav_em_dn0 = assign5340_e5240_d_n0;
        locals.var_emeav_em_dn1 = assign5340_e5240_d_n1;
        locals.var_emeav_em_dn3 = assign5340_e5240_d_n3;
        locals.var_emeav_em_dn4 = assign5340_e5240_d_n4;
        locals.var_emeav_em_dn5 = assign5340_e5240_d_n5;
        locals.var_emeav_em_dn6 = assign5340_e5240_d_n6;
        locals.var_emeav_em_dn7 = assign5340_e5240_d_n7;
        locals.var_emeav_em_dn8 = assign5340_e5240_d_n8;
        locals.var_emeav_em_dn9 = assign5340_e5240_d_n9;
        locals.var_emeav_em_dn10 = assign5340_e5240_d_n10;
        locals.var_emeav_em_rv = 0.0;

        let assign5350_e5242: f64 = (locals.var_emeav_em).abs();
        let assign5350_e5244: f64 = if assign5350_e5242 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign5350_e5244;
        locals.var_guard94_rv = 0.0;

        let (assign5360_e5261, assign5360_e5261_d_n0, assign5360_e5261_d_n1, assign5360_e5261_d_n3, assign5360_e5261_d_n4, assign5360_e5261_d_n5, assign5360_e5261_d_n6, assign5360_e5261_d_n7, assign5360_e5261_d_n8, assign5360_e5261_d_n9, assign5360_e5261_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5360_e5257: f64 = (0.5 * locals.var_wd);
        let assign5360_e5259: f64 = (assign5360_e5257 / locals.var_emeav_em);
        (assign5360_e5259, ((((0.5 * locals.var_wd_dn0) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn0)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn1) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn1)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn3) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn3)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn4) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn4)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn5) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn5)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn6) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn6)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn7) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn7)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn8) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn8)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn9) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn9)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn10) * locals.var_emeav_em) - (assign5360_e5257 * locals.var_emeav_em_dn10)) / (locals.var_emeav_em * locals.var_emeav_em)),)
    } else {
        (locals.var_lambda, locals.var_lambda_dn0, locals.var_lambda_dn1, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9, locals.var_lambda_dn10,)
    }
};
        locals.var_lambda = assign5360_e5261;
        locals.var_lambda_dn0 = assign5360_e5261_d_n0;
        locals.var_lambda_dn1 = assign5360_e5261_d_n1;
        locals.var_lambda_dn3 = assign5360_e5261_d_n3;
        locals.var_lambda_dn4 = assign5360_e5261_d_n4;
        locals.var_lambda_dn5 = assign5360_e5261_d_n5;
        locals.var_lambda_dn6 = assign5360_e5261_d_n6;
        locals.var_lambda_dn7 = assign5360_e5261_d_n7;
        locals.var_lambda_dn8 = assign5360_e5261_d_n8;
        locals.var_lambda_dn9 = assign5360_e5261_d_n9;
        locals.var_lambda_dn10 = assign5360_e5261_d_n10;
        locals.var_lambda_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5370_e5298, assign5370_e5298_d_n0, assign5370_e5298_d_n1, assign5370_e5298_d_n3, assign5370_e5298_d_n4, assign5370_e5298_d_n5, assign5370_e5298_d_n6, assign5370_e5298_d_n7, assign5370_e5298_d_n8, assign5370_e5298_d_n9, assign5370_e5298_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5370_e5274: f64 = (locals.var_an / locals.var_bnt);
        let assign5370_e5276: f64 = (assign5370_e5274 * locals.var_em);
        let assign5370_e5278: f64 = (assign5370_e5276 * locals.var_lambda);
        let assign5370_e5280: f64 = (-locals.var_bnt);
        let assign5370_e5282: f64 = (assign5370_e5280 / locals.var_em);
        let assign5370_e5283: f64 = (assign5370_e5282).exp();
        let assign5370_e5285: f64 = (-locals.var_bnt);
        let assign5370_e5287: f64 = (assign5370_e5285 / locals.var_em);
        let assign5370_e5291: f64 = (locals.var_weff / locals.var_lambda);
        let assign5370_e5292: f64 = (1.0 + assign5370_e5291);
        let assign5370_e5293: f64 = (assign5370_e5287 * assign5370_e5292);
        let assign5370_e5294: f64 = (assign5370_e5293).exp();
        let assign5370_e5295: f64 = (assign5370_e5283 - assign5370_e5294);
        let assign5370_e5296: f64 = (assign5370_e5278 * assign5370_e5295);
        (assign5370_e5296, (((((assign5370_e5274 * locals.var_em_dn0) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn0)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn0) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn0 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn0)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn1) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn1)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn1) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn1 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn1)) / (locals.var_lambda * locals.var_lambda)))))))), (((((((-((locals.var_an * locals.var_bnt_dn3) / (locals.var_bnt * locals.var_bnt))) * locals.var_em) + (assign5370_e5274 * locals.var_em_dn3)) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn3)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * ((((-locals.var_bnt_dn3) * locals.var_em) - (assign5370_e5280 * locals.var_em_dn3)) / (locals.var_em * locals.var_em))) - (assign5370_e5294 * ((((((-locals.var_bnt_dn3) * locals.var_em) - (assign5370_e5285 * locals.var_em_dn3)) / (locals.var_em * locals.var_em)) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn3 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn3)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn4) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn4)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn4) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn4) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn4 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn4)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn5) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn5)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn5) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn5 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn5)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn6) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn6)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn6) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn6 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn6)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn7) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn7)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn7) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn7 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn7)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn8) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn8)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn8) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn8 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn8)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn9) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn9)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn9) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn9 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn9)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5370_e5274 * locals.var_em_dn10) * locals.var_lambda) + (assign5370_e5276 * locals.var_lambda_dn10)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * locals.var_em_dn10) / (locals.var_em * locals.var_em))) * assign5370_e5292) + (assign5370_e5287 * (((locals.var_weff_dn10 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn10)) / (locals.var_lambda * locals.var_lambda)))))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5370_e5298;
        locals.var_gem_dn0 = assign5370_e5298_d_n0;
        locals.var_gem_dn1 = assign5370_e5298_d_n1;
        locals.var_gem_dn3 = assign5370_e5298_d_n3;
        locals.var_gem_dn4 = assign5370_e5298_d_n4;
        locals.var_gem_dn5 = assign5370_e5298_d_n5;
        locals.var_gem_dn6 = assign5370_e5298_d_n6;
        locals.var_gem_dn7 = assign5370_e5298_d_n7;
        locals.var_gem_dn8 = assign5370_e5298_d_n8;
        locals.var_gem_dn9 = assign5370_e5298_d_n9;
        locals.var_gem_dn10 = assign5370_e5298_d_n10;
        locals.var_gem_rv = 0.0;

        let (assign5380_e5320, assign5380_e5320_d_n0, assign5380_e5320_d_n1, assign5380_e5320_d_n3, assign5380_e5320_d_n4, assign5380_e5320_d_n5, assign5380_e5320_d_n6, assign5380_e5320_d_n7, assign5380_e5320_d_n8, assign5380_e5320_d_n9, assign5380_e5320_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 != 0.0)) && (locals.var_guard91 != 0.0)) && (locals.var_guard94 == 0.0)) {
        let assign5380_e5312: f64 = (locals.var_an * locals.var_weff);
        let assign5380_e5314: f64 = (-locals.var_bnt);
        let assign5380_e5316: f64 = (assign5380_e5314 / locals.var_em);
        let assign5380_e5317: f64 = (assign5380_e5316).exp();
        let assign5380_e5318: f64 = (assign5380_e5312 * assign5380_e5317);
        (assign5380_e5318, (((locals.var_an * locals.var_weff_dn0) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn1) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn3) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * ((((-locals.var_bnt_dn3) * locals.var_em) - (assign5380_e5314 * locals.var_em_dn3)) / (locals.var_em * locals.var_em))))), (((locals.var_an * locals.var_weff_dn4) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn4) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn5) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn6) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn7) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn8) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn9) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn10) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5380_e5320;
        locals.var_gem_dn0 = assign5380_e5320_d_n0;
        locals.var_gem_dn1 = assign5380_e5320_d_n1;
        locals.var_gem_dn3 = assign5380_e5320_d_n3;
        locals.var_gem_dn4 = assign5380_e5320_d_n4;
        locals.var_gem_dn5 = assign5380_e5320_d_n5;
        locals.var_gem_dn6 = assign5380_e5320_d_n6;
        locals.var_gem_dn7 = assign5380_e5320_d_n7;
        locals.var_gem_dn8 = assign5380_e5320_d_n8;
        locals.var_gem_dn9 = assign5380_e5320_d_n9;
        locals.var_gem_dn10 = assign5380_e5320_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5390_e5323: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5390_e5323;
        locals.var_guard95_rv = 0.0;

        let assign5400_e5326: f64 = if locals.var_vb2c1 < p.p43 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign5400_e5326;
        locals.var_guard96_rv = 0.0;

        let (assign5410_e5354, assign5410_e5354_d_n0, assign5410_e5354_d_n1, assign5410_e5354_d_n3, assign5410_e5354_d_n4, assign5410_e5354_d_n5, assign5410_e5354_d_n6, assign5410_e5354_d_n7, assign5410_e5354_d_n8, assign5410_e5354_d_n9, assign5410_e5354_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign5410_e5340: f64 = (p.p43 - locals.var_vb2c1);
        let assign5410_e5342: f64 = (assign5410_e5340).powf(p.p40);
        let assign5410_e5347: f64 = (p.p47 + locals.var_in_);
        let assign5410_e5348: f64 = (locals.var_in_ / assign5410_e5347);
        let assign5410_e5349: f64 = (1.0 - assign5410_e5348);
        let assign5410_e5351: f64 = (assign5410_e5349).powf(p.p48);
        let assign5410_e5352: f64 = (assign5410_e5342 * assign5410_e5351);
        (assign5410_e5352, (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn0 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn0)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn0 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn0)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn1 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn1)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn1 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn1)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn3 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn3)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn3 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn3)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn4 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn4)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn4 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn4)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn5 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn5)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn5 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn5)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5410_e5340).powf(p.p40 - 1.0) * (-locals.var_vb2c1_dn6))) } } else { (assign5410_e5342 * (p.p40 * ((-locals.var_vb2c1_dn6) / assign5410_e5340))) } * assign5410_e5351) + (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn6 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn6)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn6 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn6)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) })), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5410_e5340).powf(p.p40 - 1.0) * (-locals.var_vb2c1_dn7))) } } else { (assign5410_e5342 * (p.p40 * ((-locals.var_vb2c1_dn7) / assign5410_e5340))) } * assign5410_e5351) + (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn7 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn7)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn7 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn7)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) })), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn8 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn8)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn8 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn8)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn9 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn9)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn9 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn9)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((locals.var_in__dn10 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn10)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((locals.var_in__dn10 * assign5410_e5347) - (locals.var_in_ * locals.var_in__dn10)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }),)
    } else {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10,)
    }
};
        locals.var_vdeptmp = assign5410_e5354;
        locals.var_vdeptmp_dn0 = assign5410_e5354_d_n0;
        locals.var_vdeptmp_dn1 = assign5410_e5354_d_n1;
        locals.var_vdeptmp_dn3 = assign5410_e5354_d_n3;
        locals.var_vdeptmp_dn4 = assign5410_e5354_d_n4;
        locals.var_vdeptmp_dn5 = assign5410_e5354_d_n5;
        locals.var_vdeptmp_dn6 = assign5410_e5354_d_n6;
        locals.var_vdeptmp_dn7 = assign5410_e5354_d_n7;
        locals.var_vdeptmp_dn8 = assign5410_e5354_d_n8;
        locals.var_vdeptmp_dn9 = assign5410_e5354_d_n9;
        locals.var_vdeptmp_dn10 = assign5410_e5354_d_n10;
        locals.var_vdeptmp_rv = 0.0;

        let assign5420_e5357: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign5420_e5357;
        locals.var_guard97_rv = 0.0;

        let (assign5430_e5373, assign5430_e5373_d_n0, assign5430_e5373_d_n1, assign5430_e5373_d_n3, assign5430_e5373_d_n4, assign5430_e5373_d_n5, assign5430_e5373_d_n6, assign5430_e5373_d_n7, assign5430_e5373_d_n8, assign5430_e5373_d_n9, assign5430_e5373_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10,)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10,)
    }
};
        locals.var_vdep = assign5430_e5373;
        locals.var_vdep_dn0 = assign5430_e5373_d_n0;
        locals.var_vdep_dn1 = assign5430_e5373_d_n1;
        locals.var_vdep_dn3 = assign5430_e5373_d_n3;
        locals.var_vdep_dn4 = assign5430_e5373_d_n4;
        locals.var_vdep_dn5 = assign5430_e5373_d_n5;
        locals.var_vdep_dn6 = assign5430_e5373_d_n6;
        locals.var_vdep_dn7 = assign5430_e5373_d_n7;
        locals.var_vdep_dn8 = assign5430_e5373_d_n8;
        locals.var_vdep_dn9 = assign5430_e5373_d_n9;
        locals.var_vdep_dn10 = assign5430_e5373_d_n10;
        locals.var_vdep_rv = 0.0;

        let (assign5440_e5394, assign5440_e5394_d_n0, assign5440_e5394_d_n1, assign5440_e5394_d_n3, assign5440_e5394_d_n4, assign5440_e5394_d_n5, assign5440_e5394_d_n6, assign5440_e5394_d_n7, assign5440_e5394_d_n8, assign5440_e5394_d_n9, assign5440_e5394_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5440_e5390: f64 = (locals.var_in_ - p.p51);
        let assign5440_e5392: f64 = (assign5440_e5390 / p.p47);
        (assign5440_e5392, (locals.var_in__dn0 / p.p47), (locals.var_in__dn1 / p.p47), (locals.var_in__dn3 / p.p47), (locals.var_in__dn4 / p.p47), (locals.var_in__dn5 / p.p47), (locals.var_in__dn6 / p.p47), (locals.var_in__dn7 / p.p47), (locals.var_in__dn8 / p.p47), (locals.var_in__dn9 / p.p47), (locals.var_in__dn10 / p.p47),)
    } else {
        (locals.var_in_shift_ihcavl, locals.var_in_shift_ihcavl_dn0, locals.var_in_shift_ihcavl_dn1, locals.var_in_shift_ihcavl_dn3, locals.var_in_shift_ihcavl_dn4, locals.var_in_shift_ihcavl_dn5, locals.var_in_shift_ihcavl_dn6, locals.var_in_shift_ihcavl_dn7, locals.var_in_shift_ihcavl_dn8, locals.var_in_shift_ihcavl_dn9, locals.var_in_shift_ihcavl_dn10,)
    }
};
        locals.var_in_shift_ihcavl = assign5440_e5394;
        locals.var_in_shift_ihcavl_dn0 = assign5440_e5394_d_n0;
        locals.var_in_shift_ihcavl_dn1 = assign5440_e5394_d_n1;
        locals.var_in_shift_ihcavl_dn3 = assign5440_e5394_d_n3;
        locals.var_in_shift_ihcavl_dn4 = assign5440_e5394_d_n4;
        locals.var_in_shift_ihcavl_dn5 = assign5440_e5394_d_n5;
        locals.var_in_shift_ihcavl_dn6 = assign5440_e5394_d_n6;
        locals.var_in_shift_ihcavl_dn7 = assign5440_e5394_d_n7;
        locals.var_in_shift_ihcavl_dn8 = assign5440_e5394_d_n8;
        locals.var_in_shift_ihcavl_dn9 = assign5440_e5394_d_n9;
        locals.var_in_shift_ihcavl_dn10 = assign5440_e5394_d_n10;
        locals.var_in_shift_ihcavl_rv = 0.0;

        let (assign5450_e5415, assign5450_e5415_d_n0, assign5450_e5415_d_n1, assign5450_e5415_d_n3, assign5450_e5415_d_n4, assign5450_e5415_d_n5, assign5450_e5415_d_n6, assign5450_e5415_d_n7, assign5450_e5415_d_n8, assign5450_e5415_d_n9, assign5450_e5415_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5450_e5411: f64 = (locals.var_in_shift_ihcavl - 1.0);
        let assign5450_e5413: f64 = (assign5450_e5411 / p.p50);
        (assign5450_e5413, (locals.var_in_shift_ihcavl_dn0 / p.p50), (locals.var_in_shift_ihcavl_dn1 / p.p50), (locals.var_in_shift_ihcavl_dn3 / p.p50), (locals.var_in_shift_ihcavl_dn4 / p.p50), (locals.var_in_shift_ihcavl_dn5 / p.p50), (locals.var_in_shift_ihcavl_dn6 / p.p50), (locals.var_in_shift_ihcavl_dn7 / p.p50), (locals.var_in_shift_ihcavl_dn8 / p.p50), (locals.var_in_shift_ihcavl_dn9 / p.p50), (locals.var_in_shift_ihcavl_dn10 / p.p50),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10,)
    }
};
        locals.var_dxa = assign5450_e5415;
        locals.var_dxa_dn0 = assign5450_e5415_d_n0;
        locals.var_dxa_dn1 = assign5450_e5415_d_n1;
        locals.var_dxa_dn3 = assign5450_e5415_d_n3;
        locals.var_dxa_dn4 = assign5450_e5415_d_n4;
        locals.var_dxa_dn5 = assign5450_e5415_d_n5;
        locals.var_dxa_dn6 = assign5450_e5415_d_n6;
        locals.var_dxa_dn7 = assign5450_e5415_d_n7;
        locals.var_dxa_dn8 = assign5450_e5415_d_n8;
        locals.var_dxa_dn9 = assign5450_e5415_d_n9;
        locals.var_dxa_dn10 = assign5450_e5415_d_n10;
        locals.var_dxa_rv = 0.0;

        let assign5460_e5418: f64 = if locals.var_in_shift_ihcavl < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5460_e5418;
        locals.var_guard98_rv = 0.0;

        let (assign5470_e5445, assign5470_e5445_d_n0, assign5470_e5445_d_n1, assign5470_e5445_d_n3, assign5470_e5445_d_n4, assign5470_e5445_d_n5, assign5470_e5445_d_n6, assign5470_e5445_d_n7, assign5470_e5445_d_n8, assign5470_e5445_d_n9, assign5470_e5445_d_n10,) = {
    if (((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5470_e5439: f64 = (locals.var_dxa).exp();
        let assign5470_e5440: f64 = (1.0 + assign5470_e5439);
        let assign5470_e5441: f64 = (assign5470_e5440).ln();
        let assign5470_e5442: f64 = (p.p50 * assign5470_e5441);
        let assign5470_e5443: f64 = (1.0 + assign5470_e5442);
        (assign5470_e5443, (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn0) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn1) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn3) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn4) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn5) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn6) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn7) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn8) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn9) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * locals.var_dxa_dn10) / assign5470_e5440)),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10,)
    }
};
        locals.var_in_shift_n = assign5470_e5445;
        locals.var_in_shift_n_dn0 = assign5470_e5445_d_n0;
        locals.var_in_shift_n_dn1 = assign5470_e5445_d_n1;
        locals.var_in_shift_n_dn3 = assign5470_e5445_d_n3;
        locals.var_in_shift_n_dn4 = assign5470_e5445_d_n4;
        locals.var_in_shift_n_dn5 = assign5470_e5445_d_n5;
        locals.var_in_shift_n_dn6 = assign5470_e5445_d_n6;
        locals.var_in_shift_n_dn7 = assign5470_e5445_d_n7;
        locals.var_in_shift_n_dn8 = assign5470_e5445_d_n8;
        locals.var_in_shift_n_dn9 = assign5470_e5445_d_n9;
        locals.var_in_shift_n_dn10 = assign5470_e5445_d_n10;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5480_e5474, assign5480_e5474_d_n0, assign5480_e5474_d_n1, assign5480_e5474_d_n3, assign5480_e5474_d_n4, assign5480_e5474_d_n5, assign5480_e5474_d_n6, assign5480_e5474_d_n7, assign5480_e5474_d_n8, assign5480_e5474_d_n9, assign5480_e5474_d_n10,) = {
    if (((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard98 == 0.0)) {
        let assign5480_e5467: f64 = (-locals.var_dxa);
        let assign5480_e5468: f64 = (assign5480_e5467).exp();
        let assign5480_e5469: f64 = (1.0 + assign5480_e5468);
        let assign5480_e5470: f64 = (assign5480_e5469).ln();
        let assign5480_e5471: f64 = (p.p50 * assign5480_e5470);
        let assign5480_e5472: f64 = (locals.var_in_shift_ihcavl + assign5480_e5471);
        (assign5480_e5472, (locals.var_in_shift_ihcavl_dn0 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn0)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn1 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn1)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn3 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn3)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn4 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn4)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn5 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn5)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn6 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn6)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn7 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn7)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn8 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn8)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn9 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn9)) / assign5480_e5469))), (locals.var_in_shift_ihcavl_dn10 + (p.p50 * ((assign5480_e5468 * (-locals.var_dxa_dn10)) / assign5480_e5469))),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10,)
    }
};
        locals.var_in_shift_n = assign5480_e5474;
        locals.var_in_shift_n_dn0 = assign5480_e5474_d_n0;
        locals.var_in_shift_n_dn1 = assign5480_e5474_d_n1;
        locals.var_in_shift_n_dn3 = assign5480_e5474_d_n3;
        locals.var_in_shift_n_dn4 = assign5480_e5474_d_n4;
        locals.var_in_shift_n_dn5 = assign5480_e5474_d_n5;
        locals.var_in_shift_n_dn6 = assign5480_e5474_d_n6;
        locals.var_in_shift_n_dn7 = assign5480_e5474_d_n7;
        locals.var_in_shift_n_dn8 = assign5480_e5474_d_n8;
        locals.var_in_shift_n_dn9 = assign5480_e5474_d_n9;
        locals.var_in_shift_n_dn10 = assign5480_e5474_d_n10;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5490_e5495, assign5490_e5495_d_n0, assign5490_e5495_d_n1, assign5490_e5495_d_n3, assign5490_e5495_d_n4, assign5490_e5495_d_n5, assign5490_e5495_d_n6, assign5490_e5495_d_n7, assign5490_e5495_d_n8, assign5490_e5495_d_n9, assign5490_e5495_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign5490_e5492: f64 = (locals.var_in_shift_n).powf(p.p49);
        let assign5490_e5493: f64 = (locals.var_vdeptmp * assign5490_e5492);
        (assign5490_e5493, ((locals.var_vdeptmp_dn0 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn0)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn0 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn1 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn1)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn1 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn3 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn3)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn3 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn4 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn4)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn4 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn5 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn5)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn5 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn6 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn6)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn6 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn7 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn7)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn7 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn8 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn8)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn8 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn9 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn9)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn9 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn10 * assign5490_e5492) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((locals.var_in_shift_n).powf(p.p49 - 1.0) * locals.var_in_shift_n_dn10)) } } else { (assign5490_e5492 * (p.p49 * (locals.var_in_shift_n_dn10 / locals.var_in_shift_n))) })),)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10,)
    }
};
        locals.var_vdep = assign5490_e5495;
        locals.var_vdep_dn0 = assign5490_e5495_d_n0;
        locals.var_vdep_dn1 = assign5490_e5495_d_n1;
        locals.var_vdep_dn3 = assign5490_e5495_d_n3;
        locals.var_vdep_dn4 = assign5490_e5495_d_n4;
        locals.var_vdep_dn5 = assign5490_e5495_d_n5;
        locals.var_vdep_dn6 = assign5490_e5495_d_n6;
        locals.var_vdep_dn7 = assign5490_e5495_d_n7;
        locals.var_vdep_dn8 = assign5490_e5495_d_n8;
        locals.var_vdep_dn9 = assign5490_e5495_d_n9;
        locals.var_vdep_dn10 = assign5490_e5495_d_n10;
        locals.var_vdep_rv = 0.0;

        let assign5500_e5497: f64 = (-locals.var_bavl_t);
        let assign5500_e5499: f64 = (assign5500_e5497 * locals.var_vdep);
        let assign5500_e5501: f64 = if assign5500_e5499 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5500_e5501;
        locals.var_guard99_rv = 0.0;

        let (assign5510_e5521, assign5510_e5521_d_n0, assign5510_e5521_d_n1, assign5510_e5521_d_n3, assign5510_e5521_d_n4, assign5510_e5521_d_n5, assign5510_e5521_d_n6, assign5510_e5521_d_n7, assign5510_e5521_d_n8, assign5510_e5521_d_n9, assign5510_e5521_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard99 != 0.0)) {
        let assign5510_e5516: f64 = (-locals.var_bavl_t);
        let assign5510_e5518: f64 = (assign5510_e5516 * locals.var_vdep);
        let assign5510_e5519: f64 = (assign5510_e5518).exp();
        (assign5510_e5519, (assign5510_e5519 * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn0))), (assign5510_e5519 * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn1))), (assign5510_e5519 * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn3))), (assign5510_e5519 * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn4))), (assign5510_e5519 * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn5))), (assign5510_e5519 * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn6))), (assign5510_e5519 * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn7))), (assign5510_e5519 * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn8))), (assign5510_e5519 * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn9))), (assign5510_e5519 * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5510_e5516 * locals.var_vdep_dn10))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5510_e5521;
        locals.var_expmm1_dn0 = assign5510_e5521_d_n0;
        locals.var_expmm1_dn1 = assign5510_e5521_d_n1;
        locals.var_expmm1_dn3 = assign5510_e5521_d_n3;
        locals.var_expmm1_dn4 = assign5510_e5521_d_n4;
        locals.var_expmm1_dn5 = assign5510_e5521_d_n5;
        locals.var_expmm1_dn6 = assign5510_e5521_d_n6;
        locals.var_expmm1_dn7 = assign5510_e5521_d_n7;
        locals.var_expmm1_dn8 = assign5510_e5521_d_n8;
        locals.var_expmm1_dn9 = assign5510_e5521_d_n9;
        locals.var_expmm1_dn10 = assign5510_e5521_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5520_e5539,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5520_e5537: f64 = (p.p138).exp();
        (assign5520_e5537,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5520_e5539;
        locals.var_expl_rv = 0.0;

        let (assign5530_e5565, assign5530_e5565_d_n0, assign5530_e5565_d_n1, assign5530_e5565_d_n3, assign5530_e5565_d_n4, assign5530_e5565_d_n5, assign5530_e5565_d_n6, assign5530_e5565_d_n7, assign5530_e5565_d_n8, assign5530_e5565_d_n9, assign5530_e5565_d_n10,) = {
    if ((((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5530_e5557: f64 = (-locals.var_bavl_t);
        let assign5530_e5559: f64 = (assign5530_e5557 * locals.var_vdep);
        let assign5530_e5561: f64 = (assign5530_e5559 - p.p138);
        let assign5530_e5562: f64 = (1.0 + assign5530_e5561);
        let assign5530_e5563: f64 = (locals.var_expl * assign5530_e5562);
        (assign5530_e5563, (locals.var_expl * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn0))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn1))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn3))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn4))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn5))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn6))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn7))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn8))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn9))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5530_e5557 * locals.var_vdep_dn10))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10,)
    }
};
        locals.var_expmm1 = assign5530_e5565;
        locals.var_expmm1_dn0 = assign5530_e5565_d_n0;
        locals.var_expmm1_dn1 = assign5530_e5565_d_n1;
        locals.var_expmm1_dn3 = assign5530_e5565_d_n3;
        locals.var_expmm1_dn4 = assign5530_e5565_d_n4;
        locals.var_expmm1_dn5 = assign5530_e5565_d_n5;
        locals.var_expmm1_dn6 = assign5530_e5565_d_n6;
        locals.var_expmm1_dn7 = assign5530_e5565_d_n7;
        locals.var_expmm1_dn8 = assign5530_e5565_d_n8;
        locals.var_expmm1_dn9 = assign5530_e5565_d_n9;
        locals.var_expmm1_dn10 = assign5530_e5565_d_n10;
        locals.var_expmm1_rv = 0.0;

        let (assign5540_e5587, assign5540_e5587_d_n0, assign5540_e5587_d_n1, assign5540_e5587_d_n3, assign5540_e5587_d_n4, assign5540_e5587_d_n5, assign5540_e5587_d_n6, assign5540_e5587_d_n7, assign5540_e5587_d_n8, assign5540_e5587_d_n9, assign5540_e5587_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard86 == 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign5540_e5579: f64 = (p.p39 / locals.var_bavl_t);
        let assign5540_e5582: f64 = (p.p43 - locals.var_vb2c1);
        let assign5540_e5583: f64 = (assign5540_e5579 * assign5540_e5582);
        let assign5540_e5585: f64 = (assign5540_e5583 * locals.var_expmm1);
        (assign5540_e5585, ((((-((p.p39 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn0)), ((((-((p.p39 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn1)), ((((-((p.p39 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn3)), ((((-((p.p39 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn4)), ((((-((p.p39 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn5)), (((((-((p.p39 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) + (assign5540_e5579 * (-locals.var_vb2c1_dn6))) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn6)), (((((-((p.p39 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) + (assign5540_e5579 * (-locals.var_vb2c1_dn7))) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn7)), ((((-((p.p39 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn8)), ((((-((p.p39 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn9)), ((((-((p.p39 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5540_e5582) * locals.var_expmm1) + (assign5540_e5583 * locals.var_expmm1_dn10)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5540_e5587;
        locals.var_gem_dn0 = assign5540_e5587_d_n0;
        locals.var_gem_dn1 = assign5540_e5587_d_n1;
        locals.var_gem_dn3 = assign5540_e5587_d_n3;
        locals.var_gem_dn4 = assign5540_e5587_d_n4;
        locals.var_gem_dn5 = assign5540_e5587_d_n5;
        locals.var_gem_dn6 = assign5540_e5587_d_n6;
        locals.var_gem_dn7 = assign5540_e5587_d_n7;
        locals.var_gem_dn8 = assign5540_e5587_d_n8;
        locals.var_gem_dn9 = assign5540_e5587_d_n9;
        locals.var_gem_dn10 = assign5540_e5587_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5550_e5590: f64 = if locals.var_gem > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign5550_e5590;
        locals.var_guard100_rv = 0.0;

        let assign5560_e5593: f64 = if p.p52 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign5560_e5593;
        locals.var_guard101_rv = 0.0;

        let (assign5570_e5619, assign5570_e5619_d_n0, assign5570_e5619_d_n1, assign5570_e5619_d_n3, assign5570_e5619_d_n4, assign5570_e5619_d_n5, assign5570_e5619_d_n6, assign5570_e5619_d_n7, assign5570_e5619_d_n8, assign5570_e5619_d_n9, assign5570_e5619_d_n10,) = {
    if (((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign5570_e5603: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5570_e5604: f64 = (locals.var_in_ * assign5570_e5603);
        let assign5570_e5605: f64 = (locals.var_vt / assign5570_e5604);
        let assign5570_e5608: f64 = (locals.var_qbi / locals.var_is_t);
        let assign5570_e5610: f64 = (assign5570_e5608 * locals.var_ibi_t);
        let assign5570_e5611: f64 = (assign5570_e5605 + assign5570_e5610);
        let assign5570_e5615: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5570_e5616: f64 = (locals.var_re_t / assign5570_e5615);
        let assign5570_e5617: f64 = (assign5570_e5611 + assign5570_e5616);
        (assign5570_e5617, (((-((locals.var_vt * ((locals.var_in__dn0 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn0))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn0 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn0)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn0) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn1 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn1))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn1 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn1)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn1) / (assign5570_e5615 * assign5570_e5615)))), (((((locals.var_vt_dn3 * assign5570_e5604) - (locals.var_vt * ((locals.var_in__dn3 * assign5570_e5603) + (locals.var_in_ * (locals.var_rbc_t_dn3 + locals.var_rb2_dn3))))) / (assign5570_e5604 * assign5570_e5604)) + (((((locals.var_qbi_dn3 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn3)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t) + (assign5570_e5608 * locals.var_ibi_t_dn3))) + (((locals.var_re_t_dn3 * assign5570_e5615) - (locals.var_re_t * (locals.var_rbc_t_dn3 + locals.var_rb2_dn3))) / (assign5570_e5615 * assign5570_e5615))), (((-((locals.var_vt * ((locals.var_in__dn4 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn4))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn4 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn4)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn4) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn5 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn5))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn5 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn5)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn5) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn6 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn6))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn6 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn6)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn6) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn7 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn7))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn7 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn7)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn7) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn8 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn8))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn8 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn8)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn8) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn9 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn9))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn9 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn9)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn9) / (assign5570_e5615 * assign5570_e5615)))), (((-((locals.var_vt * ((locals.var_in__dn10 * assign5570_e5603) + (locals.var_in_ * locals.var_rb2_dn10))) / (assign5570_e5604 * assign5570_e5604))) + ((((locals.var_qbi_dn10 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn10)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn10) / (assign5570_e5615 * assign5570_e5615)))),)
    } else {
        (locals.var_gmax, locals.var_gmax_dn0, locals.var_gmax_dn1, locals.var_gmax_dn3, locals.var_gmax_dn4, locals.var_gmax_dn5, locals.var_gmax_dn6, locals.var_gmax_dn7, locals.var_gmax_dn8, locals.var_gmax_dn9, locals.var_gmax_dn10,)
    }
};
        locals.var_gmax = assign5570_e5619;
        locals.var_gmax_dn0 = assign5570_e5619_d_n0;
        locals.var_gmax_dn1 = assign5570_e5619_d_n1;
        locals.var_gmax_dn3 = assign5570_e5619_d_n3;
        locals.var_gmax_dn4 = assign5570_e5619_d_n4;
        locals.var_gmax_dn5 = assign5570_e5619_d_n5;
        locals.var_gmax_dn6 = assign5570_e5619_d_n6;
        locals.var_gmax_dn7 = assign5570_e5619_d_n7;
        locals.var_gmax_dn8 = assign5570_e5619_d_n8;
        locals.var_gmax_dn9 = assign5570_e5619_d_n9;
        locals.var_gmax_dn10 = assign5570_e5619_d_n10;
        locals.var_gmax_rv = 0.0;

        let assign5580_e5622: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5580_e5622;
        locals.var_guard102_rv = 0.0;

        let (assign5590_e5636, assign5590_e5636_d_n0, assign5590_e5636_d_n1, assign5590_e5636_d_n3, assign5590_e5636_d_n4, assign5590_e5636_d_n5, assign5590_e5636_d_n6, assign5590_e5636_d_n7, assign5590_e5636_d_n8, assign5590_e5636_d_n9, assign5590_e5636_d_n10,) = {
    if ((((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) && (locals.var_guard102 != 0.0)) {
        let assign5590_e5632: f64 = (locals.var_gem - locals.var_gmax);
        let assign5590_e5634: f64 = (assign5590_e5632 / 1e-6);
        (assign5590_e5634, ((locals.var_gem_dn0 - locals.var_gmax_dn0) / 1e-6), ((locals.var_gem_dn1 - locals.var_gmax_dn1) / 1e-6), ((locals.var_gem_dn3 - locals.var_gmax_dn3) / 1e-6), ((locals.var_gem_dn4 - locals.var_gmax_dn4) / 1e-6), ((locals.var_gem_dn5 - locals.var_gmax_dn5) / 1e-6), ((locals.var_gem_dn6 - locals.var_gmax_dn6) / 1e-6), ((locals.var_gem_dn7 - locals.var_gmax_dn7) / 1e-6), ((locals.var_gem_dn8 - locals.var_gmax_dn8) / 1e-6), ((locals.var_gem_dn9 - locals.var_gmax_dn9) / 1e-6), ((locals.var_gem_dn10 - locals.var_gmax_dn10) / 1e-6),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10,)
    }
};
        locals.var_dxa = assign5590_e5636;
        locals.var_dxa_dn0 = assign5590_e5636_d_n0;
        locals.var_dxa_dn1 = assign5590_e5636_d_n1;
        locals.var_dxa_dn3 = assign5590_e5636_d_n3;
        locals.var_dxa_dn4 = assign5590_e5636_d_n4;
        locals.var_dxa_dn5 = assign5590_e5636_d_n5;
        locals.var_dxa_dn6 = assign5590_e5636_d_n6;
        locals.var_dxa_dn7 = assign5590_e5636_d_n7;
        locals.var_dxa_dn8 = assign5590_e5636_d_n8;
        locals.var_dxa_dn9 = assign5590_e5636_d_n9;
        locals.var_dxa_dn10 = assign5590_e5636_d_n10;
        locals.var_dxa_rv = 0.0;

        let assign5600_e5639: f64 = if locals.var_gem < locals.var_gmax { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5600_e5639;
        locals.var_guard103_rv = 0.0;

        let (assign5610_e5659, assign5610_e5659_d_n0, assign5610_e5659_d_n1, assign5610_e5659_d_n3, assign5610_e5659_d_n4, assign5610_e5659_d_n5, assign5610_e5659_d_n6, assign5610_e5659_d_n7, assign5610_e5659_d_n8, assign5610_e5659_d_n9, assign5610_e5659_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign5610_e5653: f64 = (locals.var_dxa).exp();
        let assign5610_e5654: f64 = (1.0 + assign5610_e5653);
        let assign5610_e5655: f64 = (assign5610_e5654).ln();
        let assign5610_e5656: f64 = (1e-6 * assign5610_e5655);
        let assign5610_e5657: f64 = (locals.var_gem - assign5610_e5656);
        (assign5610_e5657, (locals.var_gem_dn0 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn0) / assign5610_e5654))), (locals.var_gem_dn1 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn1) / assign5610_e5654))), (locals.var_gem_dn3 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn3) / assign5610_e5654))), (locals.var_gem_dn4 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn4) / assign5610_e5654))), (locals.var_gem_dn5 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn5) / assign5610_e5654))), (locals.var_gem_dn6 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn6) / assign5610_e5654))), (locals.var_gem_dn7 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn7) / assign5610_e5654))), (locals.var_gem_dn8 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn8) / assign5610_e5654))), (locals.var_gem_dn9 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn9) / assign5610_e5654))), (locals.var_gem_dn10 - (1e-6 * ((assign5610_e5653 * locals.var_dxa_dn10) / assign5610_e5654))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5610_e5659;
        locals.var_gem_dn0 = assign5610_e5659_d_n0;
        locals.var_gem_dn1 = assign5610_e5659_d_n1;
        locals.var_gem_dn3 = assign5610_e5659_d_n3;
        locals.var_gem_dn4 = assign5610_e5659_d_n4;
        locals.var_gem_dn5 = assign5610_e5659_d_n5;
        locals.var_gem_dn6 = assign5610_e5659_d_n6;
        locals.var_gem_dn7 = assign5610_e5659_d_n7;
        locals.var_gem_dn8 = assign5610_e5659_d_n8;
        locals.var_gem_dn9 = assign5610_e5659_d_n9;
        locals.var_gem_dn10 = assign5610_e5659_d_n10;
        locals.var_gem_rv = 0.0;

        let (assign5620_e5681, assign5620_e5681_d_n0, assign5620_e5681_d_n1, assign5620_e5681_d_n3, assign5620_e5681_d_n4, assign5620_e5681_d_n5, assign5620_e5681_d_n6, assign5620_e5681_d_n7, assign5620_e5681_d_n8, assign5620_e5681_d_n9, assign5620_e5681_d_n10,) = {
    if (((((locals.var_guard85 != 0.0) && (locals.var_guard100 != 0.0)) && (locals.var_guard101 != 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 == 0.0)) {
        let assign5620_e5674: f64 = (-locals.var_dxa);
        let assign5620_e5675: f64 = (assign5620_e5674).exp();
        let assign5620_e5676: f64 = (1.0 + assign5620_e5675);
        let assign5620_e5677: f64 = (assign5620_e5676).ln();
        let assign5620_e5678: f64 = (1e-6 * assign5620_e5677);
        let assign5620_e5679: f64 = (locals.var_gmax - assign5620_e5678);
        (assign5620_e5679, (locals.var_gmax_dn0 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn0)) / assign5620_e5676))), (locals.var_gmax_dn1 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn1)) / assign5620_e5676))), (locals.var_gmax_dn3 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn3)) / assign5620_e5676))), (locals.var_gmax_dn4 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn4)) / assign5620_e5676))), (locals.var_gmax_dn5 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn5)) / assign5620_e5676))), (locals.var_gmax_dn6 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn6)) / assign5620_e5676))), (locals.var_gmax_dn7 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn7)) / assign5620_e5676))), (locals.var_gmax_dn8 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn8)) / assign5620_e5676))), (locals.var_gmax_dn9 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn9)) / assign5620_e5676))), (locals.var_gmax_dn10 - (1e-6 * ((assign5620_e5675 * (-locals.var_dxa_dn10)) / assign5620_e5676))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10,)
    }
};
        locals.var_gem = assign5620_e5681;
        locals.var_gem_dn0 = assign5620_e5681_d_n0;
        locals.var_gem_dn1 = assign5620_e5681_d_n1;
        locals.var_gem_dn3 = assign5620_e5681_d_n3;
        locals.var_gem_dn4 = assign5620_e5681_d_n4;
        locals.var_gem_dn5 = assign5620_e5681_d_n5;
        locals.var_gem_dn6 = assign5620_e5681_d_n6;
        locals.var_gem_dn7 = assign5620_e5681_d_n7;
        locals.var_gem_dn8 = assign5620_e5681_d_n8;
        locals.var_gem_dn9 = assign5620_e5681_d_n9;
        locals.var_gem_dn10 = assign5620_e5681_d_n10;
        locals.var_gem_rv = 0.0;

        let assign5730_e5844: f64 = (1.0 - p.p67);
        let assign5730_e5846: f64 = (assign5730_e5844 * locals.var_cje_t);
        let assign5730_e5848: f64 = (assign5730_e5846 * locals.var_vte);
        locals.var_qte = assign5730_e5848;
        locals.var_qte_dn0 = (((assign5730_e5844 * locals.var_cje_t_dn0) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn0));
        locals.var_qte_dn1 = (((assign5730_e5844 * locals.var_cje_t_dn1) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn1));
        locals.var_qte_dn3 = (((assign5730_e5844 * locals.var_cje_t_dn3) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn3));
        locals.var_qte_dn4 = (((assign5730_e5844 * locals.var_cje_t_dn4) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn4));
        locals.var_qte_dn5 = (((assign5730_e5844 * locals.var_cje_t_dn5) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn5));
        locals.var_qte_dn6 = (((assign5730_e5844 * locals.var_cje_t_dn6) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn6));
        locals.var_qte_dn7 = (((assign5730_e5844 * locals.var_cje_t_dn7) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn7));
        locals.var_qte_dn8 = (((assign5730_e5844 * locals.var_cje_t_dn8) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn8));
        locals.var_qte_dn9 = (((assign5730_e5844 * locals.var_cje_t_dn9) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn9));
        locals.var_qte_dn10 = (((assign5730_e5844 * locals.var_cje_t_dn10) * locals.var_vte) + (assign5730_e5846 * locals.var_vte_dn10));
        locals.var_qte_rv = 0.0;

        let assign5740_e5851: f64 = (locals.var_vb1e1 - locals.var_vfe);
        let assign5740_e5853: f64 = (assign5740_e5851 / locals.var_a_vde);
        locals.var_dxa = assign5740_e5853;
        locals.var_dxa_dn0 = ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn1 = ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn3 = ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn4 = ((((locals.var_vb1e1_dn4 - locals.var_vfe_dn4) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn5 = ((((locals.var_vb1e1_dn5 - locals.var_vfe_dn5) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn6 = ((((-locals.var_vfe_dn6) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn7 = ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn8 = ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn9 = ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn10 = ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign5740_e5851 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_rv = 0.0;

        let assign5750_e5856: f64 = if locals.var_vb1e1 < locals.var_vfe { 1.0 } else { 0.0 };
        locals.var_guard106 = assign5750_e5856;
        locals.var_guard106_rv = 0.0;

        let (assign5760_e5868, assign5760_e5868_d_n0, assign5760_e5868_d_n1, assign5760_e5868_d_n3, assign5760_e5868_d_n4, assign5760_e5868_d_n5, assign5760_e5868_d_n6, assign5760_e5868_d_n7, assign5760_e5868_d_n8, assign5760_e5868_d_n9, assign5760_e5868_d_n10,) = {
    if (locals.var_guard106 != 0.0) {
        let assign5760_e5862: f64 = (locals.var_dxa).exp();
        let assign5760_e5863: f64 = (1.0 + assign5760_e5862);
        let assign5760_e5864: f64 = (assign5760_e5863).ln();
        let assign5760_e5865: f64 = (locals.var_a_vde * assign5760_e5864);
        let assign5760_e5866: f64 = (locals.var_vb1e1 - assign5760_e5865);
        (assign5760_e5866, (-((locals.var_a_vde_dn0 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn0) / assign5760_e5863)))), (-((locals.var_a_vde_dn1 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn1) / assign5760_e5863)))), (-((locals.var_a_vde_dn3 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn3) / assign5760_e5863)))), (locals.var_vb1e1_dn4 - ((locals.var_a_vde_dn4 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn4) / assign5760_e5863)))), (locals.var_vb1e1_dn5 - ((locals.var_a_vde_dn5 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn5) / assign5760_e5863)))), (-((locals.var_a_vde_dn6 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn6) / assign5760_e5863)))), (-((locals.var_a_vde_dn7 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn7) / assign5760_e5863)))), (-((locals.var_a_vde_dn8 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn8) / assign5760_e5863)))), (-((locals.var_a_vde_dn9 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn9) / assign5760_e5863)))), (-((locals.var_a_vde_dn10 * assign5760_e5864) + (locals.var_a_vde * ((assign5760_e5862 * locals.var_dxa_dn10) / assign5760_e5863)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9, locals.var_vje_s_dn10,)
    }
};
        locals.var_vje_s = assign5760_e5868;
        locals.var_vje_s_dn0 = assign5760_e5868_d_n0;
        locals.var_vje_s_dn1 = assign5760_e5868_d_n1;
        locals.var_vje_s_dn3 = assign5760_e5868_d_n3;
        locals.var_vje_s_dn4 = assign5760_e5868_d_n4;
        locals.var_vje_s_dn5 = assign5760_e5868_d_n5;
        locals.var_vje_s_dn6 = assign5760_e5868_d_n6;
        locals.var_vje_s_dn7 = assign5760_e5868_d_n7;
        locals.var_vje_s_dn8 = assign5760_e5868_d_n8;
        locals.var_vje_s_dn9 = assign5760_e5868_d_n9;
        locals.var_vje_s_dn10 = assign5760_e5868_d_n10;
        locals.var_vje_s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5770_e5882, assign5770_e5882_d_n0, assign5770_e5882_d_n1, assign5770_e5882_d_n3, assign5770_e5882_d_n4, assign5770_e5882_d_n5, assign5770_e5882_d_n6, assign5770_e5882_d_n7, assign5770_e5882_d_n8, assign5770_e5882_d_n9, assign5770_e5882_d_n10,) = {
    if (locals.var_guard106 == 0.0) {
        let assign5770_e5875: f64 = (-locals.var_dxa);
        let assign5770_e5876: f64 = (assign5770_e5875).exp();
        let assign5770_e5877: f64 = (1.0 + assign5770_e5876);
        let assign5770_e5878: f64 = (assign5770_e5877).ln();
        let assign5770_e5879: f64 = (locals.var_a_vde * assign5770_e5878);
        let assign5770_e5880: f64 = (locals.var_vfe - assign5770_e5879);
        (assign5770_e5880, (locals.var_vfe_dn0 - ((locals.var_a_vde_dn0 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn0)) / assign5770_e5877)))), (locals.var_vfe_dn1 - ((locals.var_a_vde_dn1 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn1)) / assign5770_e5877)))), (locals.var_vfe_dn3 - ((locals.var_a_vde_dn3 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn3)) / assign5770_e5877)))), (locals.var_vfe_dn4 - ((locals.var_a_vde_dn4 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn4)) / assign5770_e5877)))), (locals.var_vfe_dn5 - ((locals.var_a_vde_dn5 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn5)) / assign5770_e5877)))), (locals.var_vfe_dn6 - ((locals.var_a_vde_dn6 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn6)) / assign5770_e5877)))), (locals.var_vfe_dn7 - ((locals.var_a_vde_dn7 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn7)) / assign5770_e5877)))), (locals.var_vfe_dn8 - ((locals.var_a_vde_dn8 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn8)) / assign5770_e5877)))), (locals.var_vfe_dn9 - ((locals.var_a_vde_dn9 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn9)) / assign5770_e5877)))), (locals.var_vfe_dn10 - ((locals.var_a_vde_dn10 * assign5770_e5878) + (locals.var_a_vde * ((assign5770_e5876 * (-locals.var_dxa_dn10)) / assign5770_e5877)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9, locals.var_vje_s_dn10,)
    }
};
        locals.var_vje_s = assign5770_e5882;
        locals.var_vje_s_dn0 = assign5770_e5882_d_n0;
        locals.var_vje_s_dn1 = assign5770_e5882_d_n1;
        locals.var_vje_s_dn3 = assign5770_e5882_d_n3;
        locals.var_vje_s_dn4 = assign5770_e5882_d_n4;
        locals.var_vje_s_dn5 = assign5770_e5882_d_n5;
        locals.var_vje_s_dn6 = assign5770_e5882_d_n6;
        locals.var_vje_s_dn7 = assign5770_e5882_d_n7;
        locals.var_vje_s_dn8 = assign5770_e5882_d_n8;
        locals.var_vje_s_dn9 = assign5770_e5882_d_n9;
        locals.var_vje_s_dn10 = assign5770_e5882_d_n10;
        locals.var_vje_s_rv = 0.0;

        let assign5780_e5885: f64 = (p.p67 * locals.var_cje_t);
        let assign5780_e5889: f64 = (1.0 - p.p66);
        let assign5780_e5890: f64 = (locals.var_vde_t / assign5780_e5889);
        let assign5780_e5895: f64 = (locals.var_vje_s * locals.var_inv_vde_t);
        let assign5780_e5896: f64 = (1.0 - assign5780_e5895);
        let assign5780_e5899: f64 = (1.0 - p.p66);
        let assign5780_e5900: f64 = (assign5780_e5896).powf(assign5780_e5899);
        let assign5780_e5901: f64 = (1.0 - assign5780_e5900);
        let assign5780_e5902: f64 = (assign5780_e5890 * assign5780_e5901);
        let assign5780_e5906: f64 = (locals.var_vb1e1 - locals.var_vje_s);
        let assign5780_e5907: f64 = (3.0 * assign5780_e5906);
        let assign5780_e5908: f64 = (assign5780_e5902 + assign5780_e5907);
        let assign5780_e5909: f64 = (assign5780_e5885 * assign5780_e5908);
        locals.var_qte_s = assign5780_e5909;
        locals.var_qte_s_dn0 = (((p.p67 * locals.var_cje_t_dn0) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn0 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn0)))));
        locals.var_qte_s_dn1 = (((p.p67 * locals.var_cje_t_dn1) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn1 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn1)))));
        locals.var_qte_s_dn3 = (((p.p67 * locals.var_cje_t_dn3) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn3 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn3)))));
        locals.var_qte_s_dn4 = (((p.p67 * locals.var_cje_t_dn4) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn4 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))) / assign5780_e5896))) }))) + (3.0 * (locals.var_vb1e1_dn4 - locals.var_vje_s_dn4)))));
        locals.var_qte_s_dn5 = (((p.p67 * locals.var_cje_t_dn5) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn5 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))) / assign5780_e5896))) }))) + (3.0 * (locals.var_vb1e1_dn5 - locals.var_vje_s_dn5)))));
        locals.var_qte_s_dn6 = (((p.p67 * locals.var_cje_t_dn6) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn6 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn6)))));
        locals.var_qte_s_dn7 = (((p.p67 * locals.var_cje_t_dn7) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn7 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn7)))));
        locals.var_qte_s_dn8 = (((p.p67 * locals.var_cje_t_dn8) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn8 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn8)))));
        locals.var_qte_s_dn9 = (((p.p67 * locals.var_cje_t_dn9) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn9 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn9)))));
        locals.var_qte_s_dn10 = (((p.p67 * locals.var_cje_t_dn10) * assign5780_e5908) + (assign5780_e5885 * ((((locals.var_vde_t_dn10 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((locals.var_vje_s_dn10 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn10))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((locals.var_vje_s_dn10 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn10))) / assign5780_e5896))) }))) + (3.0 * (-locals.var_vje_s_dn10)))));
        locals.var_qte_s_rv = 0.0;

        let assign5790_e5912: f64 = (p.p76 * locals.var_cjc_t);
        let assign5790_e5914: f64 = (assign5790_e5912 * locals.var_vtc);
        locals.var_qtc = assign5790_e5914;
        locals.var_qtc_dn0 = (((p.p76 * locals.var_cjc_t_dn0) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn0));
        locals.var_qtc_dn1 = (((p.p76 * locals.var_cjc_t_dn1) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn1));
        locals.var_qtc_dn3 = (((p.p76 * locals.var_cjc_t_dn3) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn3));
        locals.var_qtc_dn4 = (((p.p76 * locals.var_cjc_t_dn4) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn4));
        locals.var_qtc_dn5 = (((p.p76 * locals.var_cjc_t_dn5) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn5));
        locals.var_qtc_dn6 = (((p.p76 * locals.var_cjc_t_dn6) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn6));
        locals.var_qtc_dn7 = (((p.p76 * locals.var_cjc_t_dn7) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn7));
        locals.var_qtc_dn8 = (((p.p76 * locals.var_cjc_t_dn8) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn8));
        locals.var_qtc_dn9 = (((p.p76 * locals.var_cjc_t_dn9) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn9));
        locals.var_qtc_dn10 = (((p.p76 * locals.var_cjc_t_dn10) * locals.var_vtc) + (assign5790_e5912 * locals.var_vtc_dn10));
        locals.var_qtc_rv = 0.0;

        let assign5800_e5917: f64 = (locals.var_taub_t * locals.var_ik_t);
        locals.var_qb0 = assign5800_e5917;
        locals.var_qb0_dn3 = ((locals.var_taub_t_dn3 * locals.var_ik_t) + (locals.var_taub_t * locals.var_ik_t_dn3));
        locals.var_qb0_rv = 0.0;

        let assign5810_e5920: f64 = (0.5 * locals.var_qb0);
        let assign5810_e5922: f64 = (assign5810_e5920 * locals.var_n0);
        let assign5810_e5924: f64 = (assign5810_e5922 * locals.var_q1q);
        locals.var_qbe_qs = assign5810_e5924;
        locals.var_qbe_qs_dn0 = (((assign5810_e5920 * locals.var_n0_dn0) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn0));
        locals.var_qbe_qs_dn1 = (((assign5810_e5920 * locals.var_n0_dn1) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn1));
        locals.var_qbe_qs_dn3 = (((((0.5 * locals.var_qb0_dn3) * locals.var_n0) + (assign5810_e5920 * locals.var_n0_dn3)) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn3));
        locals.var_qbe_qs_dn4 = (((assign5810_e5920 * locals.var_n0_dn4) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn4));
        locals.var_qbe_qs_dn5 = (((assign5810_e5920 * locals.var_n0_dn5) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn5));
        locals.var_qbe_qs_dn6 = (((assign5810_e5920 * locals.var_n0_dn6) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn6));
        locals.var_qbe_qs_dn7 = (((assign5810_e5920 * locals.var_n0_dn7) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn7));
        locals.var_qbe_qs_dn8 = (((assign5810_e5920 * locals.var_n0_dn8) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn8));
        locals.var_qbe_qs_dn9 = (((assign5810_e5920 * locals.var_n0_dn9) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn9));
        locals.var_qbe_qs_dn10 = (((assign5810_e5920 * locals.var_n0_dn10) * locals.var_q1q) + (assign5810_e5922 * locals.var_q1q_dn10));
        locals.var_qbe_qs_rv = 0.0;

        let assign5820_e5927: f64 = (0.5 * locals.var_qb0);
        let assign5820_e5929: f64 = (assign5820_e5927 * locals.var_nb);
        let assign5820_e5931: f64 = (assign5820_e5929 * locals.var_q1q);
        locals.var_qbc_qs = assign5820_e5931;
        locals.var_qbc_qs_dn0 = (((assign5820_e5927 * locals.var_nb_dn0) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn0));
        locals.var_qbc_qs_dn1 = (((assign5820_e5927 * locals.var_nb_dn1) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn1));
        locals.var_qbc_qs_dn3 = (((((0.5 * locals.var_qb0_dn3) * locals.var_nb) + (assign5820_e5927 * locals.var_nb_dn3)) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn3));
        locals.var_qbc_qs_dn4 = (((assign5820_e5927 * locals.var_nb_dn4) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn4));
        locals.var_qbc_qs_dn5 = (((assign5820_e5927 * locals.var_nb_dn5) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn5));
        locals.var_qbc_qs_dn6 = (((assign5820_e5927 * locals.var_nb_dn6) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn6));
        locals.var_qbc_qs_dn7 = (((assign5820_e5927 * locals.var_nb_dn7) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn7));
        locals.var_qbc_qs_dn8 = (((assign5820_e5927 * locals.var_nb_dn8) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn8));
        locals.var_qbc_qs_dn9 = (((assign5820_e5927 * locals.var_nb_dn9) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn9));
        locals.var_qbc_qs_dn10 = (((assign5820_e5927 * locals.var_nb_dn10) * locals.var_q1q) + (assign5820_e5929 * locals.var_q1q_dn10));
        locals.var_qbc_qs_rv = 0.0;

        let assign5830_e5934: f64 = (0.1 * locals.var_vdc_ctc_t);
        locals.var_a_vdcctc = assign5830_e5934;
        locals.var_a_vdcctc_dn0 = (0.1 * locals.var_vdc_ctc_t_dn0);
        locals.var_a_vdcctc_dn1 = (0.1 * locals.var_vdc_ctc_t_dn1);
        locals.var_a_vdcctc_dn3 = (0.1 * locals.var_vdc_ctc_t_dn3);
        locals.var_a_vdcctc_dn4 = (0.1 * locals.var_vdc_ctc_t_dn4);
        locals.var_a_vdcctc_dn5 = (0.1 * locals.var_vdc_ctc_t_dn5);
        locals.var_a_vdcctc_dn6 = (0.1 * locals.var_vdc_ctc_t_dn6);
        locals.var_a_vdcctc_dn7 = (0.1 * locals.var_vdc_ctc_t_dn7);
        locals.var_a_vdcctc_dn8 = (0.1 * locals.var_vdc_ctc_t_dn8);
        locals.var_a_vdcctc_dn9 = (0.1 * locals.var_vdc_ctc_t_dn9);
        locals.var_a_vdcctc_dn10 = (0.1 * locals.var_vdc_ctc_t_dn10);
        locals.var_a_vdcctc_rv = 0.0;

        let assign5840_e5937: f64 = (locals.var_vb1c4 - locals.var_vfc);
        let assign5840_e5939: f64 = (assign5840_e5937 / locals.var_a_vdcctc);
        locals.var_dxa = assign5840_e5939;
        locals.var_dxa_dn0 = ((((-locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((-locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((-locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((locals.var_vb1c4_dn5 - locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vb1c4_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vb1c4_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((locals.var_vb1c4_dn8 - locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((-locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn10 = ((((locals.var_vb1c4_dn10 - locals.var_vfc_dn10) * locals.var_a_vdcctc) - (assign5840_e5937 * locals.var_a_vdcctc_dn10)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_rv = 0.0;

        let assign5850_e5942: f64 = if locals.var_vb1c4 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5850_e5942;
        locals.var_guard107_rv = 0.0;

        let (assign5860_e5954, assign5860_e5954_d_n0, assign5860_e5954_d_n1, assign5860_e5954_d_n3, assign5860_e5954_d_n4, assign5860_e5954_d_n5, assign5860_e5954_d_n6, assign5860_e5954_d_n7, assign5860_e5954_d_n8, assign5860_e5954_d_n9, assign5860_e5954_d_n10,) = {
    if (locals.var_guard107 != 0.0) {
        let assign5860_e5948: f64 = (locals.var_dxa).exp();
        let assign5860_e5949: f64 = (1.0 + assign5860_e5948);
        let assign5860_e5950: f64 = (assign5860_e5949).ln();
        let assign5860_e5951: f64 = (locals.var_a_vdcctc * assign5860_e5950);
        let assign5860_e5952: f64 = (locals.var_vb1c4 - assign5860_e5951);
        (assign5860_e5952, (-((locals.var_a_vdcctc_dn0 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn0) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn1 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn1) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn3 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn3) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn4 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn4) / assign5860_e5949)))), (locals.var_vb1c4_dn5 - ((locals.var_a_vdcctc_dn5 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn5) / assign5860_e5949)))), (locals.var_vb1c4_dn6 - ((locals.var_a_vdcctc_dn6 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn6) / assign5860_e5949)))), (locals.var_vb1c4_dn7 - ((locals.var_a_vdcctc_dn7 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn7) / assign5860_e5949)))), (locals.var_vb1c4_dn8 - ((locals.var_a_vdcctc_dn8 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn8) / assign5860_e5949)))), (-((locals.var_a_vdcctc_dn9 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn9) / assign5860_e5949)))), (locals.var_vb1c4_dn10 - ((locals.var_a_vdcctc_dn10 * assign5860_e5950) + (locals.var_a_vdcctc * ((assign5860_e5948 * locals.var_dxa_dn10) / assign5860_e5949)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9, locals.var_vjcex_dn10,)
    }
};
        locals.var_vjcex = assign5860_e5954;
        locals.var_vjcex_dn0 = assign5860_e5954_d_n0;
        locals.var_vjcex_dn1 = assign5860_e5954_d_n1;
        locals.var_vjcex_dn3 = assign5860_e5954_d_n3;
        locals.var_vjcex_dn4 = assign5860_e5954_d_n4;
        locals.var_vjcex_dn5 = assign5860_e5954_d_n5;
        locals.var_vjcex_dn6 = assign5860_e5954_d_n6;
        locals.var_vjcex_dn7 = assign5860_e5954_d_n7;
        locals.var_vjcex_dn8 = assign5860_e5954_d_n8;
        locals.var_vjcex_dn9 = assign5860_e5954_d_n9;
        locals.var_vjcex_dn10 = assign5860_e5954_d_n10;
        locals.var_vjcex_rv = 0.0;

        let (assign5870_e5968, assign5870_e5968_d_n0, assign5870_e5968_d_n1, assign5870_e5968_d_n3, assign5870_e5968_d_n4, assign5870_e5968_d_n5, assign5870_e5968_d_n6, assign5870_e5968_d_n7, assign5870_e5968_d_n8, assign5870_e5968_d_n9, assign5870_e5968_d_n10,) = {
    if (locals.var_guard107 == 0.0) {
        let assign5870_e5961: f64 = (-locals.var_dxa);
        let assign5870_e5962: f64 = (assign5870_e5961).exp();
        let assign5870_e5963: f64 = (1.0 + assign5870_e5962);
        let assign5870_e5964: f64 = (assign5870_e5963).ln();
        let assign5870_e5965: f64 = (locals.var_a_vdcctc * assign5870_e5964);
        let assign5870_e5966: f64 = (locals.var_vfc - assign5870_e5965);
        (assign5870_e5966, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn0)) / assign5870_e5963)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn1)) / assign5870_e5963)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn3)) / assign5870_e5963)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn4)) / assign5870_e5963)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn5)) / assign5870_e5963)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn6)) / assign5870_e5963)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn7)) / assign5870_e5963)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn8)) / assign5870_e5963)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn9)) / assign5870_e5963)))), (locals.var_vfc_dn10 - ((locals.var_a_vdcctc_dn10 * assign5870_e5964) + (locals.var_a_vdcctc * ((assign5870_e5962 * (-locals.var_dxa_dn10)) / assign5870_e5963)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9, locals.var_vjcex_dn10,)
    }
};
        locals.var_vjcex = assign5870_e5968;
        locals.var_vjcex_dn0 = assign5870_e5968_d_n0;
        locals.var_vjcex_dn1 = assign5870_e5968_d_n1;
        locals.var_vjcex_dn3 = assign5870_e5968_d_n3;
        locals.var_vjcex_dn4 = assign5870_e5968_d_n4;
        locals.var_vjcex_dn5 = assign5870_e5968_d_n5;
        locals.var_vjcex_dn6 = assign5870_e5968_d_n6;
        locals.var_vjcex_dn7 = assign5870_e5968_d_n7;
        locals.var_vjcex_dn8 = assign5870_e5968_d_n8;
        locals.var_vjcex_dn9 = assign5870_e5968_d_n9;
        locals.var_vjcex_dn10 = assign5870_e5968_d_n10;
        locals.var_vjcex_rv = 0.0;

        let assign5880_e5972: f64 = (1.0 - p.p71);
        let assign5880_e5973: f64 = (locals.var_vdc_ctc_t / assign5880_e5972);
        let assign5880_e5978: f64 = (locals.var_vjcex / locals.var_vdc_ctc_t);
        let assign5880_e5979: f64 = (1.0 - assign5880_e5978);
        let assign5880_e5982: f64 = (1.0 - p.p71);
        let assign5880_e5983: f64 = (assign5880_e5979).powf(assign5880_e5982);
        let assign5880_e5984: f64 = (1.0 - assign5880_e5983);
        let assign5880_e5985: f64 = (assign5880_e5973 * assign5880_e5984);
        let assign5880_e5989: f64 = (locals.var_vb1c4 - locals.var_vjcex);
        let assign5880_e5990: f64 = (locals.var_bjc * assign5880_e5989);
        let assign5880_e5991: f64 = (assign5880_e5985 + assign5880_e5990);
        locals.var_vtexv = assign5880_e5991;
        locals.var_vtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn0 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn0))));
        locals.var_vtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn1 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn1))));
        locals.var_vtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn3 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn3))));
        locals.var_vtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn4 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn4))));
        locals.var_vtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn5 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn5 - locals.var_vjcex_dn5))));
        locals.var_vtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn6 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn6 - locals.var_vjcex_dn6))));
        locals.var_vtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn7 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn7 - locals.var_vjcex_dn7))));
        locals.var_vtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn8 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn8 - locals.var_vjcex_dn8))));
        locals.var_vtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn9 * assign5880_e5989) + (locals.var_bjc * (-locals.var_vjcex_dn9))));
        locals.var_vtexv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((locals.var_vjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((locals.var_vjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((locals.var_bjc_dn10 * assign5880_e5989) + (locals.var_bjc * (locals.var_vb1c4_dn10 - locals.var_vjcex_dn10))));
        locals.var_vtexv_rv = 0.0;

        let assign5890_e5995: f64 = (1.0 - locals.var_xp_t);
        let assign5890_e5997: f64 = (assign5890_e5995 * locals.var_vtexv);
        let assign5890_e6000: f64 = (locals.var_xp_t * locals.var_vb1c4);
        let assign5890_e6001: f64 = (assign5890_e5997 + assign5890_e6000);
        let assign5890_e6002: f64 = (locals.var_cjc_t * assign5890_e6001);
        let assign5890_e6005: f64 = (1.0 - p.p76);
        let assign5890_e6006: f64 = (assign5890_e6002 * assign5890_e6005);
        let assign5890_e6009: f64 = (1.0 - p.p32);
        let assign5890_e6010: f64 = (assign5890_e6006 * assign5890_e6009);
        locals.var_qtex = assign5890_e6010;
        locals.var_qtex_dn0 = ((((locals.var_cjc_t_dn0 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn0)) + (locals.var_xp_t_dn0 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn1 = ((((locals.var_cjc_t_dn1 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn1)) + (locals.var_xp_t_dn1 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn3 = ((((locals.var_cjc_t_dn3 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn4 = ((((locals.var_cjc_t_dn4 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn5 = ((((locals.var_cjc_t_dn5 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn5)) + ((locals.var_xp_t_dn5 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn5))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn6 = ((((locals.var_cjc_t_dn6 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn6))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn7 = ((((locals.var_cjc_t_dn7 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn7))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn8 = ((((locals.var_cjc_t_dn8 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn8))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn9 = ((((locals.var_cjc_t_dn9 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn9)) + (locals.var_xp_t_dn9 * locals.var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_dn10 = ((((locals.var_cjc_t_dn10 * assign5890_e6001) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn10) * locals.var_vtexv) + (assign5890_e5995 * locals.var_vtexv_dn10)) + ((locals.var_xp_t_dn10 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn10))))) * assign5890_e6005) * assign5890_e6009);
        locals.var_qtex_rv = 0.0;

        let assign5900_e6013: f64 = (locals.var_vbc3 - locals.var_vfc);
        let assign5900_e6015: f64 = (assign5900_e6013 / locals.var_a_vdcctc);
        locals.var_dxa = assign5900_e6015;
        locals.var_dxa_dn0 = ((((locals.var_vbc3_dn0 - locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((locals.var_vbc3_dn1 - locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((-locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((locals.var_vbc3_dn5 - locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vbc3_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vbc3_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((locals.var_vbc3_dn8 - locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((locals.var_vbc3_dn9 - locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn10 = ((((locals.var_vbc3_dn10 - locals.var_vfc_dn10) * locals.var_a_vdcctc) - (assign5900_e6013 * locals.var_a_vdcctc_dn10)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_rv = 0.0;

        let assign5910_e6018: f64 = if locals.var_vbc3 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5910_e6018;
        locals.var_guard108_rv = 0.0;

        let (assign5920_e6030, assign5920_e6030_d_n0, assign5920_e6030_d_n1, assign5920_e6030_d_n3, assign5920_e6030_d_n4, assign5920_e6030_d_n5, assign5920_e6030_d_n6, assign5920_e6030_d_n7, assign5920_e6030_d_n8, assign5920_e6030_d_n9, assign5920_e6030_d_n10,) = {
    if (locals.var_guard108 != 0.0) {
        let assign5920_e6024: f64 = (locals.var_dxa).exp();
        let assign5920_e6025: f64 = (1.0 + assign5920_e6024);
        let assign5920_e6026: f64 = (assign5920_e6025).ln();
        let assign5920_e6027: f64 = (locals.var_a_vdcctc * assign5920_e6026);
        let assign5920_e6028: f64 = (locals.var_vbc3 - assign5920_e6027);
        (assign5920_e6028, (locals.var_vbc3_dn0 - ((locals.var_a_vdcctc_dn0 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn0) / assign5920_e6025)))), (locals.var_vbc3_dn1 - ((locals.var_a_vdcctc_dn1 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn1) / assign5920_e6025)))), (-((locals.var_a_vdcctc_dn3 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn3) / assign5920_e6025)))), (-((locals.var_a_vdcctc_dn4 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn4) / assign5920_e6025)))), (locals.var_vbc3_dn5 - ((locals.var_a_vdcctc_dn5 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn5) / assign5920_e6025)))), (locals.var_vbc3_dn6 - ((locals.var_a_vdcctc_dn6 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn6) / assign5920_e6025)))), (locals.var_vbc3_dn7 - ((locals.var_a_vdcctc_dn7 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn7) / assign5920_e6025)))), (locals.var_vbc3_dn8 - ((locals.var_a_vdcctc_dn8 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn8) / assign5920_e6025)))), (locals.var_vbc3_dn9 - ((locals.var_a_vdcctc_dn9 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn9) / assign5920_e6025)))), (locals.var_vbc3_dn10 - ((locals.var_a_vdcctc_dn10 * assign5920_e6026) + (locals.var_a_vdcctc * ((assign5920_e6024 * locals.var_dxa_dn10) / assign5920_e6025)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9, locals.var_xvjcex_dn10,)
    }
};
        locals.var_xvjcex = assign5920_e6030;
        locals.var_xvjcex_dn0 = assign5920_e6030_d_n0;
        locals.var_xvjcex_dn1 = assign5920_e6030_d_n1;
        locals.var_xvjcex_dn3 = assign5920_e6030_d_n3;
        locals.var_xvjcex_dn4 = assign5920_e6030_d_n4;
        locals.var_xvjcex_dn5 = assign5920_e6030_d_n5;
        locals.var_xvjcex_dn6 = assign5920_e6030_d_n6;
        locals.var_xvjcex_dn7 = assign5920_e6030_d_n7;
        locals.var_xvjcex_dn8 = assign5920_e6030_d_n8;
        locals.var_xvjcex_dn9 = assign5920_e6030_d_n9;
        locals.var_xvjcex_dn10 = assign5920_e6030_d_n10;
        locals.var_xvjcex_rv = 0.0;

        let (assign5930_e6044, assign5930_e6044_d_n0, assign5930_e6044_d_n1, assign5930_e6044_d_n3, assign5930_e6044_d_n4, assign5930_e6044_d_n5, assign5930_e6044_d_n6, assign5930_e6044_d_n7, assign5930_e6044_d_n8, assign5930_e6044_d_n9, assign5930_e6044_d_n10,) = {
    if (locals.var_guard108 == 0.0) {
        let assign5930_e6037: f64 = (-locals.var_dxa);
        let assign5930_e6038: f64 = (assign5930_e6037).exp();
        let assign5930_e6039: f64 = (1.0 + assign5930_e6038);
        let assign5930_e6040: f64 = (assign5930_e6039).ln();
        let assign5930_e6041: f64 = (locals.var_a_vdcctc * assign5930_e6040);
        let assign5930_e6042: f64 = (locals.var_vfc - assign5930_e6041);
        (assign5930_e6042, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn0)) / assign5930_e6039)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn1)) / assign5930_e6039)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn3)) / assign5930_e6039)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn4)) / assign5930_e6039)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn5)) / assign5930_e6039)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn6)) / assign5930_e6039)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn7)) / assign5930_e6039)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn8)) / assign5930_e6039)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn9)) / assign5930_e6039)))), (locals.var_vfc_dn10 - ((locals.var_a_vdcctc_dn10 * assign5930_e6040) + (locals.var_a_vdcctc * ((assign5930_e6038 * (-locals.var_dxa_dn10)) / assign5930_e6039)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9, locals.var_xvjcex_dn10,)
    }
};
        locals.var_xvjcex = assign5930_e6044;
        locals.var_xvjcex_dn0 = assign5930_e6044_d_n0;
        locals.var_xvjcex_dn1 = assign5930_e6044_d_n1;
        locals.var_xvjcex_dn3 = assign5930_e6044_d_n3;
        locals.var_xvjcex_dn4 = assign5930_e6044_d_n4;
        locals.var_xvjcex_dn5 = assign5930_e6044_d_n5;
        locals.var_xvjcex_dn6 = assign5930_e6044_d_n6;
        locals.var_xvjcex_dn7 = assign5930_e6044_d_n7;
        locals.var_xvjcex_dn8 = assign5930_e6044_d_n8;
        locals.var_xvjcex_dn9 = assign5930_e6044_d_n9;
        locals.var_xvjcex_dn10 = assign5930_e6044_d_n10;
        locals.var_xvjcex_rv = 0.0;

        let assign5940_e6048: f64 = (1.0 - p.p71);
        let assign5940_e6049: f64 = (locals.var_vdc_ctc_t / assign5940_e6048);
        let assign5940_e6054: f64 = (locals.var_xvjcex / locals.var_vdc_ctc_t);
        let assign5940_e6055: f64 = (1.0 - assign5940_e6054);
        let assign5940_e6058: f64 = (1.0 - p.p71);
        let assign5940_e6059: f64 = (assign5940_e6055).powf(assign5940_e6058);
        let assign5940_e6060: f64 = (1.0 - assign5940_e6059);
        let assign5940_e6061: f64 = (assign5940_e6049 * assign5940_e6060);
        let assign5940_e6065: f64 = (locals.var_vbc3 - locals.var_xvjcex);
        let assign5940_e6066: f64 = (locals.var_bjc * assign5940_e6065);
        let assign5940_e6067: f64 = (assign5940_e6061 + assign5940_e6066);
        locals.var_xvtexv = assign5940_e6067;
        locals.var_xvtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn0 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn0 - locals.var_xvjcex_dn0))));
        locals.var_xvtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn1 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn1 - locals.var_xvjcex_dn1))));
        locals.var_xvtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn3 * assign5940_e6065) + (locals.var_bjc * (-locals.var_xvjcex_dn3))));
        locals.var_xvtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn4 * assign5940_e6065) + (locals.var_bjc * (-locals.var_xvjcex_dn4))));
        locals.var_xvtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn5 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn5 - locals.var_xvjcex_dn5))));
        locals.var_xvtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn6 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn6 - locals.var_xvjcex_dn6))));
        locals.var_xvtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn7 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn7 - locals.var_xvjcex_dn7))));
        locals.var_xvtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn8 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn8 - locals.var_xvjcex_dn8))));
        locals.var_xvtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn9 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn9 - locals.var_xvjcex_dn9))));
        locals.var_xvtexv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((locals.var_xvjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((locals.var_xvjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((locals.var_bjc_dn10 * assign5940_e6065) + (locals.var_bjc * (locals.var_vbc3_dn10 - locals.var_xvjcex_dn10))));
        locals.var_xvtexv_rv = 0.0;

        let assign5950_e6071: f64 = (1.0 - locals.var_xp_t);
        let assign5950_e6073: f64 = (assign5950_e6071 * locals.var_xvtexv);
        let assign5950_e6076: f64 = (locals.var_xp_t * locals.var_vbc3);
        let assign5950_e6077: f64 = (assign5950_e6073 + assign5950_e6076);
        let assign5950_e6078: f64 = (locals.var_cjc_t * assign5950_e6077);
        let assign5950_e6081: f64 = (1.0 - p.p76);
        let assign5950_e6082: f64 = (assign5950_e6078 * assign5950_e6081);
        let assign5950_e6084: f64 = (assign5950_e6082 * p.p32);
        locals.var_xqtex = assign5950_e6084;
        locals.var_xqtex_dn0 = ((((locals.var_cjc_t_dn0 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn0)) + ((locals.var_xp_t_dn0 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn0))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn1 = ((((locals.var_cjc_t_dn1 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn1)) + ((locals.var_xp_t_dn1 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn1))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn3 = ((((locals.var_cjc_t_dn3 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vbc3)))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn4 = ((((locals.var_cjc_t_dn4 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vbc3)))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn5 = ((((locals.var_cjc_t_dn5 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn5)) + ((locals.var_xp_t_dn5 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn5))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn6 = ((((locals.var_cjc_t_dn6 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn6))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn7 = ((((locals.var_cjc_t_dn7 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn7))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn8 = ((((locals.var_cjc_t_dn8 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn8))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn9 = ((((locals.var_cjc_t_dn9 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn9)) + ((locals.var_xp_t_dn9 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn9))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_dn10 = ((((locals.var_cjc_t_dn10 * assign5950_e6077) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn10) * locals.var_xvtexv) + (assign5950_e6071 * locals.var_xvtexv_dn10)) + ((locals.var_xp_t_dn10 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn10))))) * assign5950_e6081) * p.p32);
        locals.var_xqtex_rv = 0.0;

        let assign5960_e6087: f64 = (locals.var_taue_t * locals.var_ik_t);
        let assign5960_e6090: f64 = (locals.var_is_t / locals.var_ik_t);
        let assign5960_e6093: f64 = (1.0 / p.p84);
        let assign5960_e6094: f64 = (assign5960_e6090).powf(assign5960_e6093);
        let assign5960_e6095: f64 = (assign5960_e6087 * assign5960_e6094);
        locals.var_qe0 = assign5960_e6095;
        locals.var_qe0_dn0 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn0 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn0 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn1 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn1 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn1 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn3 = ((((locals.var_taue_t_dn3 * locals.var_ik_t) + (locals.var_taue_t * locals.var_ik_t_dn3)) * assign5960_e6094) + (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (((locals.var_is_t_dn3 * locals.var_ik_t) - (locals.var_is_t * locals.var_ik_t_dn3)) / (locals.var_ik_t * locals.var_ik_t)))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((((locals.var_is_t_dn3 * locals.var_ik_t) - (locals.var_is_t * locals.var_ik_t_dn3)) / (locals.var_ik_t * locals.var_ik_t)) / assign5960_e6090))) }));
        locals.var_qe0_dn4 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn4 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn4 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn5 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn5 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn5 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn6 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn6 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn6 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn7 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn7 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn7 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn8 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn8 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn8 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn9 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn9 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn9 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_dn10 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (locals.var_is_t_dn10 / locals.var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((locals.var_is_t_dn10 / locals.var_ik_t) / assign5960_e6090))) });
        locals.var_qe0_rv = 0.0;

        let assign5970_e6099: f64 = (p.p84 * locals.var_vt);
        let assign5970_e6100: f64 = (locals.var_vb2e1 / assign5970_e6099);
        let assign5970_e6102: f64 = if assign5970_e6100 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign5970_e6102;
        locals.var_guard109_rv = 0.0;

        let (assign5980_e6111, assign5980_e6111_d_n0, assign5980_e6111_d_n1, assign5980_e6111_d_n3, assign5980_e6111_d_n4, assign5980_e6111_d_n5, assign5980_e6111_d_n6, assign5980_e6111_d_n7, assign5980_e6111_d_n8, assign5980_e6111_d_n9, assign5980_e6111_d_n10,) = {
    if (locals.var_guard109 != 0.0) {
        let assign5980_e6107: f64 = (p.p84 * locals.var_vt);
        let assign5980_e6108: f64 = (locals.var_vb2e1 / assign5980_e6107);
        let assign5980_e6109: f64 = (assign5980_e6108).exp();
        (assign5980_e6109, 0.0, 0.0, (assign5980_e6109 * (-((locals.var_vb2e1 * (p.p84 * locals.var_vt_dn3)) / (assign5980_e6107 * assign5980_e6107)))), (assign5980_e6109 * (locals.var_vb2e1_dn4 / assign5980_e6107)), 0.0, (assign5980_e6109 * (locals.var_vb2e1_dn6 / assign5980_e6107)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign5980_e6111;
        locals.var_tmpexp_dn0 = assign5980_e6111_d_n0;
        locals.var_tmpexp_dn1 = assign5980_e6111_d_n1;
        locals.var_tmpexp_dn3 = assign5980_e6111_d_n3;
        locals.var_tmpexp_dn4 = assign5980_e6111_d_n4;
        locals.var_tmpexp_dn5 = assign5980_e6111_d_n5;
        locals.var_tmpexp_dn6 = assign5980_e6111_d_n6;
        locals.var_tmpexp_dn7 = assign5980_e6111_d_n7;
        locals.var_tmpexp_dn8 = assign5980_e6111_d_n8;
        locals.var_tmpexp_dn9 = assign5980_e6111_d_n9;
        locals.var_tmpexp_dn10 = assign5980_e6111_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let (assign5990_e6117,) = {
    if (locals.var_guard109 == 0.0) {
        let assign5990_e6115: f64 = (p.p138).exp();
        (assign5990_e6115,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5990_e6117;
        locals.var_expl_rv = 0.0;

        let (assign6000_e6132, assign6000_e6132_d_n0, assign6000_e6132_d_n1, assign6000_e6132_d_n3, assign6000_e6132_d_n4, assign6000_e6132_d_n5, assign6000_e6132_d_n6, assign6000_e6132_d_n7, assign6000_e6132_d_n8, assign6000_e6132_d_n9, assign6000_e6132_d_n10,) = {
    if (locals.var_guard109 == 0.0) {
        let assign6000_e6125: f64 = (p.p84 * locals.var_vt);
        let assign6000_e6126: f64 = (locals.var_vb2e1 / assign6000_e6125);
        let assign6000_e6128: f64 = (assign6000_e6126 - p.p138);
        let assign6000_e6129: f64 = (1.0 + assign6000_e6128);
        let assign6000_e6130: f64 = (locals.var_expl * assign6000_e6129);
        (assign6000_e6130, 0.0, 0.0, (locals.var_expl * (-((locals.var_vb2e1 * (p.p84 * locals.var_vt_dn3)) / (assign6000_e6125 * assign6000_e6125)))), (locals.var_expl * (locals.var_vb2e1_dn4 / assign6000_e6125)), 0.0, (locals.var_expl * (locals.var_vb2e1_dn6 / assign6000_e6125)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10,)
    }
};
        locals.var_tmpexp = assign6000_e6132;
        locals.var_tmpexp_dn0 = assign6000_e6132_d_n0;
        locals.var_tmpexp_dn1 = assign6000_e6132_d_n1;
        locals.var_tmpexp_dn3 = assign6000_e6132_d_n3;
        locals.var_tmpexp_dn4 = assign6000_e6132_d_n4;
        locals.var_tmpexp_dn5 = assign6000_e6132_d_n5;
        locals.var_tmpexp_dn6 = assign6000_e6132_d_n6;
        locals.var_tmpexp_dn7 = assign6000_e6132_d_n7;
        locals.var_tmpexp_dn8 = assign6000_e6132_d_n8;
        locals.var_tmpexp_dn9 = assign6000_e6132_d_n9;
        locals.var_tmpexp_dn10 = assign6000_e6132_d_n10;
        locals.var_tmpexp_rv = 0.0;

        let assign6010_e6135: f64 = (locals.var_qe0 * locals.var_tmpexp);
        locals.var_qe_qs = assign6010_e6135;
        locals.var_qe_qs_dn0 = ((locals.var_qe0_dn0 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn0));
        locals.var_qe_qs_dn1 = ((locals.var_qe0_dn1 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn1));
        locals.var_qe_qs_dn3 = ((locals.var_qe0_dn3 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn3));
        locals.var_qe_qs_dn4 = ((locals.var_qe0_dn4 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn4));
        locals.var_qe_qs_dn5 = ((locals.var_qe0_dn5 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn5));
        locals.var_qe_qs_dn6 = ((locals.var_qe0_dn6 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn6));
        locals.var_qe_qs_dn7 = ((locals.var_qe0_dn7 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn7));
        locals.var_qe_qs_dn8 = ((locals.var_qe0_dn8 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn8));
        locals.var_qe_qs_dn9 = ((locals.var_qe0_dn9 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn9));
        locals.var_qe_qs_dn10 = ((locals.var_qe0_dn10 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn10));
        locals.var_qe_qs_rv = 0.0;

        let assign6020_e6138: f64 = (4.0 * locals.var_tepi_t);
        let assign6020_e6140: f64 = (assign6020_e6138 * locals.var_vt);
        let assign6020_e6142: f64 = (assign6020_e6140 / locals.var_rcv_t);
        locals.var_qepi0 = assign6020_e6142;
        locals.var_qepi0_dn3 = ((((((4.0 * locals.var_tepi_t_dn3) * locals.var_vt) + (assign6020_e6138 * locals.var_vt_dn3)) * locals.var_rcv_t) - (assign6020_e6140 * locals.var_rcv_t_dn3)) / (locals.var_rcv_t * locals.var_rcv_t));
        locals.var_qepi0_rv = 0.0;

        let assign6030_e6145: f64 = (0.5 * locals.var_qepi0);
        let assign6030_e6147: f64 = (assign6030_e6145 * locals.var_xi_w);
        let assign6030_e6150: f64 = (locals.var_p0star + locals.var_pw);
        let assign6030_e6152: f64 = (assign6030_e6150 + 2.0);
        let assign6030_e6153: f64 = (assign6030_e6147 * assign6030_e6152);
        locals.var_qepi = assign6030_e6153;
        locals.var_qepi_dn0 = (((assign6030_e6145 * locals.var_xi_w_dn0) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn0 + locals.var_pw_dn0)));
        locals.var_qepi_dn1 = (((assign6030_e6145 * locals.var_xi_w_dn1) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn1 + locals.var_pw_dn1)));
        locals.var_qepi_dn3 = (((((0.5 * locals.var_qepi0_dn3) * locals.var_xi_w) + (assign6030_e6145 * locals.var_xi_w_dn3)) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn3 + locals.var_pw_dn3)));
        locals.var_qepi_dn4 = (((assign6030_e6145 * locals.var_xi_w_dn4) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn4 + locals.var_pw_dn4)));
        locals.var_qepi_dn5 = (((assign6030_e6145 * locals.var_xi_w_dn5) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn5 + locals.var_pw_dn5)));
        locals.var_qepi_dn6 = (((assign6030_e6145 * locals.var_xi_w_dn6) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn6 + locals.var_pw_dn6)));
        locals.var_qepi_dn7 = (((assign6030_e6145 * locals.var_xi_w_dn7) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn7 + locals.var_pw_dn7)));
        locals.var_qepi_dn8 = (((assign6030_e6145 * locals.var_xi_w_dn8) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn8 + locals.var_pw_dn8)));
        locals.var_qepi_dn9 = (((assign6030_e6145 * locals.var_xi_w_dn9) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn9 + locals.var_pw_dn9)));
        locals.var_qepi_dn10 = (((assign6030_e6145 * locals.var_xi_w_dn10) * assign6030_e6152) + (assign6030_e6147 * (locals.var_p0star_dn10 + locals.var_pw_dn10)));
        locals.var_qepi_rv = 0.0;

        let assign6040_e6156: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6040_e6156;
        locals.var_guard110_rv = 0.0;

        let (assign6050_e6174, assign6050_e6174_d_n0, assign6050_e6174_d_n1, assign6050_e6174_d_n3, assign6050_e6174_d_n4, assign6050_e6174_d_n5, assign6050_e6174_d_n6, assign6050_e6174_d_n7, assign6050_e6174_d_n8, assign6050_e6174_d_n9, assign6050_e6174_d_n10,) = {
    if (locals.var_guard110 != 0.0) {
        let assign6050_e6160: f64 = (locals.var_taur_t * 0.5);
        let assign6050_e6163: f64 = (locals.var_qb0 * locals.var_nbex);
        let assign6050_e6166: f64 = (locals.var_qepi0 * locals.var_pwex);
        let assign6050_e6167: f64 = (assign6050_e6163 + assign6050_e6166);
        let assign6050_e6168: f64 = (assign6050_e6160 * assign6050_e6167);
        let assign6050_e6171: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign6050_e6172: f64 = (assign6050_e6168 / assign6050_e6171);
        (assign6050_e6172, ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn0) + (locals.var_qepi0 * locals.var_pwex_dn0))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn1) + (locals.var_qepi0 * locals.var_pwex_dn1))) / assign6050_e6171), ((((((locals.var_taur_t_dn3 * 0.5) * assign6050_e6167) + (assign6050_e6160 * (((locals.var_qb0_dn3 * locals.var_nbex) + (locals.var_qb0 * locals.var_nbex_dn3)) + ((locals.var_qepi0_dn3 * locals.var_pwex) + (locals.var_qepi0 * locals.var_pwex_dn3))))) * assign6050_e6171) - (assign6050_e6168 * (locals.var_taub_t_dn3 + locals.var_tepi_t_dn3))) / (assign6050_e6171 * assign6050_e6171)), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn4) + (locals.var_qepi0 * locals.var_pwex_dn4))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn5) + (locals.var_qepi0 * locals.var_pwex_dn5))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn6) + (locals.var_qepi0 * locals.var_pwex_dn6))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn7) + (locals.var_qepi0 * locals.var_pwex_dn7))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn8) + (locals.var_qepi0 * locals.var_pwex_dn8))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn9) + (locals.var_qepi0 * locals.var_pwex_dn9))) / assign6050_e6171), ((assign6050_e6160 * ((locals.var_qb0 * locals.var_nbex_dn10) + (locals.var_qepi0 * locals.var_pwex_dn10))) / assign6050_e6171),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10,)
    }
};
        locals.var_qex = assign6050_e6174;
        locals.var_qex_dn0 = assign6050_e6174_d_n0;
        locals.var_qex_dn1 = assign6050_e6174_d_n1;
        locals.var_qex_dn3 = assign6050_e6174_d_n3;
        locals.var_qex_dn4 = assign6050_e6174_d_n4;
        locals.var_qex_dn5 = assign6050_e6174_d_n5;
        locals.var_qex_dn6 = assign6050_e6174_d_n6;
        locals.var_qex_dn7 = assign6050_e6174_d_n7;
        locals.var_qex_dn8 = assign6050_e6174_d_n8;
        locals.var_qex_dn9 = assign6050_e6174_d_n9;
        locals.var_qex_dn10 = assign6050_e6174_d_n10;
        locals.var_qex_rv = 0.0;

        let assign6060_e6177: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6060_e6179: f64 = (assign6060_e6177 / p.p90);
        let assign6060_e6181: f64 = (assign6060_e6179 * locals.var_vtinv);
        let assign6060_e6183: f64 = if assign6060_e6181 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign6060_e6183;
        locals.var_guard111_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6070_e6197, assign6070_e6197_d_n0, assign6070_e6197_d_n1, assign6070_e6197_d_n3, assign6070_e6197_d_n4, assign6070_e6197_d_n5, assign6070_e6197_d_n6, assign6070_e6197_d_n7, assign6070_e6197_d_n8, assign6070_e6197_d_n9, assign6070_e6197_d_n10,) = {
    if ((locals.var_guard110 == 0.0) && (locals.var_guard111 != 0.0)) {
        let assign6070_e6190: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6070_e6192: f64 = (assign6070_e6190 / p.p90);
        let assign6070_e6194: f64 = (assign6070_e6192 * locals.var_vtinv);
        let assign6070_e6195: f64 = (assign6070_e6194).exp();
        (assign6070_e6195, (assign6070_e6195 * (((-locals.var_vdcex_t_dn0) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((-locals.var_vdcex_t_dn1) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * ((((-locals.var_vdcex_t_dn3) / p.p90) * locals.var_vtinv) + (assign6070_e6192 * locals.var_vtinv_dn3))), (assign6070_e6195 * (((-locals.var_vdcex_t_dn4) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn5 - locals.var_vdcex_t_dn5) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn8 - locals.var_vdcex_t_dn8) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((-locals.var_vdcex_t_dn9) / p.p90) * locals.var_vtinv)), (assign6070_e6195 * (((locals.var_vb1c4_dn10 - locals.var_vdcex_t_dn10) / p.p90) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9, locals.var_evb1c4vdcex_dn10,)
    }
};
        locals.var_evb1c4vdcex = assign6070_e6197;
        locals.var_evb1c4vdcex_dn0 = assign6070_e6197_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign6070_e6197_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign6070_e6197_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign6070_e6197_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign6070_e6197_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign6070_e6197_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign6070_e6197_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign6070_e6197_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign6070_e6197_d_n9;
        locals.var_evb1c4vdcex_dn10 = assign6070_e6197_d_n10;
        locals.var_evb1c4vdcex_rv = 0.0;

        let (assign6080_e6206,) = {
    if ((locals.var_guard110 == 0.0) && (locals.var_guard111 == 0.0)) {
        let assign6080_e6204: f64 = (p.p138).exp();
        (assign6080_e6204,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6080_e6206;
        locals.var_expl_rv = 0.0;

        let (assign6090_e6226, assign6090_e6226_d_n0, assign6090_e6226_d_n1, assign6090_e6226_d_n3, assign6090_e6226_d_n4, assign6090_e6226_d_n5, assign6090_e6226_d_n6, assign6090_e6226_d_n7, assign6090_e6226_d_n8, assign6090_e6226_d_n9, assign6090_e6226_d_n10,) = {
    if ((locals.var_guard110 == 0.0) && (locals.var_guard111 == 0.0)) {
        let assign6090_e6216: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6090_e6218: f64 = (assign6090_e6216 / p.p90);
        let assign6090_e6220: f64 = (assign6090_e6218 * locals.var_vtinv);
        let assign6090_e6222: f64 = (assign6090_e6220 - p.p138);
        let assign6090_e6223: f64 = (1.0 + assign6090_e6222);
        let assign6090_e6224: f64 = (locals.var_expl * assign6090_e6223);
        (assign6090_e6224, (locals.var_expl * (((-locals.var_vdcex_t_dn0) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn1) / p.p90) * locals.var_vtinv)), (locals.var_expl * ((((-locals.var_vdcex_t_dn3) / p.p90) * locals.var_vtinv) + (assign6090_e6218 * locals.var_vtinv_dn3))), (locals.var_expl * (((-locals.var_vdcex_t_dn4) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn5 - locals.var_vdcex_t_dn5) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn8 - locals.var_vdcex_t_dn8) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn9) / p.p90) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn10 - locals.var_vdcex_t_dn10) / p.p90) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9, locals.var_evb1c4vdcex_dn10,)
    }
};
        locals.var_evb1c4vdcex = assign6090_e6226;
        locals.var_evb1c4vdcex_dn0 = assign6090_e6226_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign6090_e6226_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign6090_e6226_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign6090_e6226_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign6090_e6226_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign6090_e6226_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign6090_e6226_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign6090_e6226_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign6090_e6226_d_n9;
        locals.var_evb1c4vdcex_dn10 = assign6090_e6226_d_n10;
        locals.var_evb1c4vdcex_rv = 0.0;

        let (assign6100_e6246, assign6100_e6246_d_n0, assign6100_e6246_d_n1, assign6100_e6246_d_n3, assign6100_e6246_d_n4, assign6100_e6246_d_n5, assign6100_e6246_d_n6, assign6100_e6246_d_n7, assign6100_e6246_d_n8, assign6100_e6246_d_n9, assign6100_e6246_d_n10,) = {
    if (locals.var_guard110 == 0.0) {
        let assign6100_e6231: f64 = (2.0 * locals.var_ibx_t);
        let assign6100_e6233: f64 = (assign6100_e6231 * locals.var_tauex_t);
        let assign6100_e6235: f64 = (assign6100_e6233 * locals.var_evb1c4);
        let assign6100_e6240: f64 = (4.0 * locals.var_evb1c4vdcex);
        let assign6100_e6241: f64 = (1.0 + assign6100_e6240);
        let assign6100_e6242: f64 = (assign6100_e6241).sqrt();
        let assign6100_e6243: f64 = (1.0 + assign6100_e6242);
        let assign6100_e6244: f64 = (assign6100_e6235 / assign6100_e6243);
        (assign6100_e6244, (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn0) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn1) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((((((2.0 * locals.var_ibx_t_dn3) * locals.var_tauex_t) + (assign6100_e6231 * locals.var_tauex_t_dn3)) * locals.var_evb1c4) + (assign6100_e6233 * locals.var_evb1c4_dn3)) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn3) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn4) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((assign6100_e6233 * locals.var_evb1c4_dn5) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn5) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * locals.var_evb1c4_dn6) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn6) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * locals.var_evb1c4_dn7) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn7) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * locals.var_evb1c4_dn8) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn8) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), (-((assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn9) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((assign6100_e6233 * locals.var_evb1c4_dn10) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * locals.var_evb1c4vdcex_dn10) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10,)
    }
};
        locals.var_qex = assign6100_e6246;
        locals.var_qex_dn0 = assign6100_e6246_d_n0;
        locals.var_qex_dn1 = assign6100_e6246_d_n1;
        locals.var_qex_dn3 = assign6100_e6246_d_n3;
        locals.var_qex_dn4 = assign6100_e6246_d_n4;
        locals.var_qex_dn5 = assign6100_e6246_d_n5;
        locals.var_qex_dn6 = assign6100_e6246_d_n6;
        locals.var_qex_dn7 = assign6100_e6246_d_n7;
        locals.var_qex_dn8 = assign6100_e6246_d_n8;
        locals.var_qex_dn9 = assign6100_e6246_d_n9;
        locals.var_qex_dn10 = assign6100_e6246_d_n10;
        locals.var_qex_rv = 0.0;

        let assign6110_e6257: f64 = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6110_e6257;
        locals.var_guard112_rv = 0.0;

        let (assign6120_e6263, assign6120_e6263_d_n0, assign6120_e6263_d_n1, assign6120_e6263_d_n3, assign6120_e6263_d_n4, assign6120_e6263_d_n5, assign6120_e6263_d_n6, assign6120_e6263_d_n7, assign6120_e6263_d_n8, assign6120_e6263_d_n9, assign6120_e6263_d_n10,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6120_e6261: f64 = (locals.var_qex * locals.var_xext1);
        (assign6120_e6261, (locals.var_qex_dn0 * locals.var_xext1), (locals.var_qex_dn1 * locals.var_xext1), (locals.var_qex_dn3 * locals.var_xext1), (locals.var_qex_dn4 * locals.var_xext1), (locals.var_qex_dn5 * locals.var_xext1), (locals.var_qex_dn6 * locals.var_xext1), (locals.var_qex_dn7 * locals.var_xext1), (locals.var_qex_dn8 * locals.var_xext1), (locals.var_qex_dn9 * locals.var_xext1), (locals.var_qex_dn10 * locals.var_xext1),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10,)
    }
};
        locals.var_qex = assign6120_e6263;
        locals.var_qex_dn0 = assign6120_e6263_d_n0;
        locals.var_qex_dn1 = assign6120_e6263_d_n1;
        locals.var_qex_dn3 = assign6120_e6263_d_n3;
        locals.var_qex_dn4 = assign6120_e6263_d_n4;
        locals.var_qex_dn5 = assign6120_e6263_d_n5;
        locals.var_qex_dn6 = assign6120_e6263_d_n6;
        locals.var_qex_dn7 = assign6120_e6263_d_n7;
        locals.var_qex_dn8 = assign6120_e6263_d_n8;
        locals.var_qex_dn9 = assign6120_e6263_d_n9;
        locals.var_qex_dn10 = assign6120_e6263_d_n10;
        locals.var_qex_rv = 0.0;

        let assign6130_e6266: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign6130_e6266;
        locals.var_guard113_rv = 0.0;

        let (assign6140_e6274, assign6140_e6274_d_n0, assign6140_e6274_d_n1, assign6140_e6274_d_n3, assign6140_e6274_d_n4, assign6140_e6274_d_n5, assign6140_e6274_d_n6, assign6140_e6274_d_n7, assign6140_e6274_d_n8, assign6140_e6274_d_n9, assign6140_e6274_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6140_e6272: f64 = (locals.var_if0 * locals.var_evbc3);
        (assign6140_e6272, ((locals.var_if0_dn0 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn0)), ((locals.var_if0_dn1 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn1)), ((locals.var_if0_dn3 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn3)), (locals.var_if0_dn4 * locals.var_evbc3), ((locals.var_if0_dn5 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn5)), ((locals.var_if0_dn6 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn6)), ((locals.var_if0_dn7 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn7)), ((locals.var_if0_dn8 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn8)), ((locals.var_if0_dn9 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn9)), ((locals.var_if0_dn10 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn10)),)
    } else {
        (locals.var_xg1, locals.var_xg1_dn0, locals.var_xg1_dn1, locals.var_xg1_dn3, locals.var_xg1_dn4, locals.var_xg1_dn5, locals.var_xg1_dn6, locals.var_xg1_dn7, locals.var_xg1_dn8, locals.var_xg1_dn9, locals.var_xg1_dn10,)
    }
};
        locals.var_xg1 = assign6140_e6274;
        locals.var_xg1_dn0 = assign6140_e6274_d_n0;
        locals.var_xg1_dn1 = assign6140_e6274_d_n1;
        locals.var_xg1_dn3 = assign6140_e6274_d_n3;
        locals.var_xg1_dn4 = assign6140_e6274_d_n4;
        locals.var_xg1_dn5 = assign6140_e6274_d_n5;
        locals.var_xg1_dn6 = assign6140_e6274_d_n6;
        locals.var_xg1_dn7 = assign6140_e6274_d_n7;
        locals.var_xg1_dn8 = assign6140_e6274_d_n8;
        locals.var_xg1_dn9 = assign6140_e6274_d_n9;
        locals.var_xg1_dn10 = assign6140_e6274_d_n10;
        locals.var_xg1_rv = 0.0;

        let (assign6150_e6289, assign6150_e6289_d_n0, assign6150_e6289_d_n1, assign6150_e6289_d_n3, assign6150_e6289_d_n4, assign6150_e6289_d_n5, assign6150_e6289_d_n6, assign6150_e6289_d_n7, assign6150_e6289_d_n8, assign6150_e6289_d_n9, assign6150_e6289_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6150_e6280: f64 = (locals.var_xg1 - locals.var_if0);
        let assign6150_e6284: f64 = (1.0 + locals.var_xg1);
        let assign6150_e6285: f64 = (assign6150_e6284).sqrt();
        let assign6150_e6286: f64 = (1.0 + assign6150_e6285);
        let assign6150_e6287: f64 = (assign6150_e6280 / assign6150_e6286);
        (assign6150_e6287, ((((locals.var_xg1_dn0 - locals.var_if0_dn0) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn0 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn1 - locals.var_if0_dn1) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn1 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn3 - locals.var_if0_dn3) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn3 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn4 - locals.var_if0_dn4) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn4 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn5 - locals.var_if0_dn5) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn5 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn6 - locals.var_if0_dn6) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn6 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn7 - locals.var_if0_dn7) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn7 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn8 - locals.var_if0_dn8) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn8 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn9 - locals.var_if0_dn9) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn9 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((locals.var_xg1_dn10 - locals.var_if0_dn10) * assign6150_e6286) - (assign6150_e6280 * (locals.var_xg1_dn10 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)),)
    } else {
        (locals.var_xnbex, locals.var_xnbex_dn0, locals.var_xnbex_dn1, locals.var_xnbex_dn3, locals.var_xnbex_dn4, locals.var_xnbex_dn5, locals.var_xnbex_dn6, locals.var_xnbex_dn7, locals.var_xnbex_dn8, locals.var_xnbex_dn9, locals.var_xnbex_dn10,)
    }
};
        locals.var_xnbex = assign6150_e6289;
        locals.var_xnbex_dn0 = assign6150_e6289_d_n0;
        locals.var_xnbex_dn1 = assign6150_e6289_d_n1;
        locals.var_xnbex_dn3 = assign6150_e6289_d_n3;
        locals.var_xnbex_dn4 = assign6150_e6289_d_n4;
        locals.var_xnbex_dn5 = assign6150_e6289_d_n5;
        locals.var_xnbex_dn6 = assign6150_e6289_d_n6;
        locals.var_xnbex_dn7 = assign6150_e6289_d_n7;
        locals.var_xnbex_dn8 = assign6150_e6289_d_n8;
        locals.var_xnbex_dn9 = assign6150_e6289_d_n9;
        locals.var_xnbex_dn10 = assign6150_e6289_d_n10;
        locals.var_xnbex_rv = 0.0;

        let (assign6160_e6297, assign6160_e6297_d_n0, assign6160_e6297_d_n1, assign6160_e6297_d_n3, assign6160_e6297_d_n4, assign6160_e6297_d_n5, assign6160_e6297_d_n6, assign6160_e6297_d_n7, assign6160_e6297_d_n8, assign6160_e6297_d_n9, assign6160_e6297_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6160_e6295: f64 = (4.0 * locals.var_evbc3vdc);
        (assign6160_e6295, (4.0 * locals.var_evbc3vdc_dn0), (4.0 * locals.var_evbc3vdc_dn1), (4.0 * locals.var_evbc3vdc_dn3), (4.0 * locals.var_evbc3vdc_dn4), (4.0 * locals.var_evbc3vdc_dn5), (4.0 * locals.var_evbc3vdc_dn6), (4.0 * locals.var_evbc3vdc_dn7), (4.0 * locals.var_evbc3vdc_dn8), (4.0 * locals.var_evbc3vdc_dn9), (4.0 * locals.var_evbc3vdc_dn10),)
    } else {
        (locals.var_xg2, locals.var_xg2_dn0, locals.var_xg2_dn1, locals.var_xg2_dn3, locals.var_xg2_dn4, locals.var_xg2_dn5, locals.var_xg2_dn6, locals.var_xg2_dn7, locals.var_xg2_dn8, locals.var_xg2_dn9, locals.var_xg2_dn10,)
    }
};
        locals.var_xg2 = assign6160_e6297;
        locals.var_xg2_dn0 = assign6160_e6297_d_n0;
        locals.var_xg2_dn1 = assign6160_e6297_d_n1;
        locals.var_xg2_dn3 = assign6160_e6297_d_n3;
        locals.var_xg2_dn4 = assign6160_e6297_d_n4;
        locals.var_xg2_dn5 = assign6160_e6297_d_n5;
        locals.var_xg2_dn6 = assign6160_e6297_d_n6;
        locals.var_xg2_dn7 = assign6160_e6297_d_n7;
        locals.var_xg2_dn8 = assign6160_e6297_d_n8;
        locals.var_xg2_dn9 = assign6160_e6297_d_n9;
        locals.var_xg2_dn10 = assign6160_e6297_d_n10;
        locals.var_xg2_rv = 0.0;

        let (assign6170_e6310, assign6170_e6310_d_n0, assign6170_e6310_d_n1, assign6170_e6310_d_n3, assign6170_e6310_d_n4, assign6170_e6310_d_n5, assign6170_e6310_d_n6, assign6170_e6310_d_n7, assign6170_e6310_d_n8, assign6170_e6310_d_n9, assign6170_e6310_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6170_e6305: f64 = (1.0 + locals.var_xg2);
        let assign6170_e6306: f64 = (assign6170_e6305).sqrt();
        let assign6170_e6307: f64 = (1.0 + assign6170_e6306);
        let assign6170_e6308: f64 = (locals.var_xg2 / assign6170_e6307);
        (assign6170_e6308, (((locals.var_xg2_dn0 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn0 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn1 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn1 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn3 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn3 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn4 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn4 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn5 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn5 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn6 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn6 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn7 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn7 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn8 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn8 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn9 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn9 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((locals.var_xg2_dn10 * assign6170_e6307) - (locals.var_xg2 * (locals.var_xg2_dn10 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)),)
    } else {
        (locals.var_xpwex, locals.var_xpwex_dn0, locals.var_xpwex_dn1, locals.var_xpwex_dn3, locals.var_xpwex_dn4, locals.var_xpwex_dn5, locals.var_xpwex_dn6, locals.var_xpwex_dn7, locals.var_xpwex_dn8, locals.var_xpwex_dn9, locals.var_xpwex_dn10,)
    }
};
        locals.var_xpwex = assign6170_e6310;
        locals.var_xpwex_dn0 = assign6170_e6310_d_n0;
        locals.var_xpwex_dn1 = assign6170_e6310_d_n1;
        locals.var_xpwex_dn3 = assign6170_e6310_d_n3;
        locals.var_xpwex_dn4 = assign6170_e6310_d_n4;
        locals.var_xpwex_dn5 = assign6170_e6310_d_n5;
        locals.var_xpwex_dn6 = assign6170_e6310_d_n6;
        locals.var_xpwex_dn7 = assign6170_e6310_d_n7;
        locals.var_xpwex_dn8 = assign6170_e6310_d_n8;
        locals.var_xpwex_dn9 = assign6170_e6310_d_n9;
        locals.var_xpwex_dn10 = assign6170_e6310_d_n10;
        locals.var_xpwex_rv = 0.0;

        let (assign6180_e6332, assign6180_e6332_d_n0, assign6180_e6332_d_n1, assign6180_e6332_d_n3, assign6180_e6332_d_n4, assign6180_e6332_d_n5, assign6180_e6332_d_n6, assign6180_e6332_d_n7, assign6180_e6332_d_n8, assign6180_e6332_d_n9, assign6180_e6332_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign6180_e6316: f64 = (0.5 * p.p32);
        let assign6180_e6318: f64 = (assign6180_e6316 * locals.var_taur_t);
        let assign6180_e6321: f64 = (locals.var_qb0 * locals.var_xnbex);
        let assign6180_e6324: f64 = (locals.var_qepi0 * locals.var_xpwex);
        let assign6180_e6325: f64 = (assign6180_e6321 + assign6180_e6324);
        let assign6180_e6326: f64 = (assign6180_e6318 * assign6180_e6325);
        let assign6180_e6329: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign6180_e6330: f64 = (assign6180_e6326 / assign6180_e6329);
        (assign6180_e6330, ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn0) + (locals.var_qepi0 * locals.var_xpwex_dn0))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn1) + (locals.var_qepi0 * locals.var_xpwex_dn1))) / assign6180_e6329), ((((((assign6180_e6316 * locals.var_taur_t_dn3) * assign6180_e6325) + (assign6180_e6318 * (((locals.var_qb0_dn3 * locals.var_xnbex) + (locals.var_qb0 * locals.var_xnbex_dn3)) + ((locals.var_qepi0_dn3 * locals.var_xpwex) + (locals.var_qepi0 * locals.var_xpwex_dn3))))) * assign6180_e6329) - (assign6180_e6326 * (locals.var_taub_t_dn3 + locals.var_tepi_t_dn3))) / (assign6180_e6329 * assign6180_e6329)), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn4) + (locals.var_qepi0 * locals.var_xpwex_dn4))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn5) + (locals.var_qepi0 * locals.var_xpwex_dn5))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn6) + (locals.var_qepi0 * locals.var_xpwex_dn6))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn7) + (locals.var_qepi0 * locals.var_xpwex_dn7))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn8) + (locals.var_qepi0 * locals.var_xpwex_dn8))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn9) + (locals.var_qepi0 * locals.var_xpwex_dn9))) / assign6180_e6329), ((assign6180_e6318 * ((locals.var_qb0 * locals.var_xnbex_dn10) + (locals.var_qepi0 * locals.var_xpwex_dn10))) / assign6180_e6329),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9, locals.var_xqmex_dn10,)
    }
};
        locals.var_xqmex = assign6180_e6332;
        locals.var_xqmex_dn0 = assign6180_e6332_d_n0;
        locals.var_xqmex_dn1 = assign6180_e6332_d_n1;
        locals.var_xqmex_dn3 = assign6180_e6332_d_n3;
        locals.var_xqmex_dn4 = assign6180_e6332_d_n4;
        locals.var_xqmex_dn5 = assign6180_e6332_d_n5;
        locals.var_xqmex_dn6 = assign6180_e6332_d_n6;
        locals.var_xqmex_dn7 = assign6180_e6332_d_n7;
        locals.var_xqmex_dn8 = assign6180_e6332_d_n8;
        locals.var_xqmex_dn9 = assign6180_e6332_d_n9;
        locals.var_xqmex_dn10 = assign6180_e6332_d_n10;
        locals.var_xqmex_rv = 0.0;

        let assign6190_e6335: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6190_e6337: f64 = (assign6190_e6335 * locals.var_vtinv);
        let assign6190_e6339: f64 = if assign6190_e6337 < p.p138 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6190_e6339;
        locals.var_guard114_rv = 0.0;

        let (assign6200_e6353, assign6200_e6353_d_n0, assign6200_e6353_d_n1, assign6200_e6353_d_n3, assign6200_e6353_d_n4, assign6200_e6353_d_n5, assign6200_e6353_d_n6, assign6200_e6353_d_n7, assign6200_e6353_d_n8, assign6200_e6353_d_n9, assign6200_e6353_d_n10,) = {
    if (((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 != 0.0)) {
        let assign6200_e6348: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6200_e6350: f64 = (assign6200_e6348 * locals.var_vtinv);
        let assign6200_e6351: f64 = (assign6200_e6350).exp();
        (assign6200_e6351, (assign6200_e6351 * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (assign6200_e6351 * (((-locals.var_vdcex_t_dn3) * locals.var_vtinv) + (assign6200_e6348 * locals.var_vtinv_dn3))), (assign6200_e6351 * ((-locals.var_vdcex_t_dn4) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn5 - locals.var_vdcex_t_dn5) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)), (assign6200_e6351 * ((locals.var_vbc3_dn10 - locals.var_vdcex_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9, locals.var_evbc3vdcex_dn10,)
    }
};
        locals.var_evbc3vdcex = assign6200_e6353;
        locals.var_evbc3vdcex_dn0 = assign6200_e6353_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6200_e6353_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6200_e6353_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6200_e6353_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6200_e6353_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6200_e6353_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6200_e6353_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6200_e6353_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6200_e6353_d_n9;
        locals.var_evbc3vdcex_dn10 = assign6200_e6353_d_n10;
        locals.var_evbc3vdcex_rv = 0.0;

        let (assign6210_e6364,) = {
    if (((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 == 0.0)) {
        let assign6210_e6362: f64 = (p.p138).exp();
        (assign6210_e6362,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6210_e6364;
        locals.var_expl_rv = 0.0;

        let (assign6220_e6384, assign6220_e6384_d_n0, assign6220_e6384_d_n1, assign6220_e6384_d_n3, assign6220_e6384_d_n4, assign6220_e6384_d_n5, assign6220_e6384_d_n6, assign6220_e6384_d_n7, assign6220_e6384_d_n8, assign6220_e6384_d_n9, assign6220_e6384_d_n10,) = {
    if (((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 == 0.0)) {
        let assign6220_e6376: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6220_e6378: f64 = (assign6220_e6376 * locals.var_vtinv);
        let assign6220_e6380: f64 = (assign6220_e6378 - p.p138);
        let assign6220_e6381: f64 = (1.0 + assign6220_e6380);
        let assign6220_e6382: f64 = (locals.var_expl * assign6220_e6381);
        (assign6220_e6382, (locals.var_expl * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn3) * locals.var_vtinv) + (assign6220_e6376 * locals.var_vtinv_dn3))), (locals.var_expl * ((-locals.var_vdcex_t_dn4) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn5 - locals.var_vdcex_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn10 - locals.var_vdcex_t_dn10) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9, locals.var_evbc3vdcex_dn10,)
    }
};
        locals.var_evbc3vdcex = assign6220_e6384;
        locals.var_evbc3vdcex_dn0 = assign6220_e6384_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6220_e6384_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6220_e6384_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6220_e6384_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6220_e6384_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6220_e6384_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6220_e6384_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6220_e6384_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6220_e6384_d_n9;
        locals.var_evbc3vdcex_dn10 = assign6220_e6384_d_n10;
        locals.var_evbc3vdcex_rv = 0.0;

        let (assign6230_e6408, assign6230_e6408_d_n0, assign6230_e6408_d_n1, assign6230_e6408_d_n3, assign6230_e6408_d_n4, assign6230_e6408_d_n5, assign6230_e6408_d_n6, assign6230_e6408_d_n7, assign6230_e6408_d_n8, assign6230_e6408_d_n9, assign6230_e6408_d_n10,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard113 == 0.0)) {
        let assign6230_e6391: f64 = (2.0 * p.p32);
        let assign6230_e6393: f64 = (assign6230_e6391 * locals.var_ibx_t);
        let assign6230_e6395: f64 = (assign6230_e6393 * locals.var_tauex_t);
        let assign6230_e6397: f64 = (assign6230_e6395 * locals.var_evbc3);
        let assign6230_e6402: f64 = (4.0 * locals.var_evbc3vdcex);
        let assign6230_e6403: f64 = (1.0 + assign6230_e6402);
        let assign6230_e6404: f64 = (assign6230_e6403).sqrt();
        let assign6230_e6405: f64 = (1.0 + assign6230_e6404);
        let assign6230_e6406: f64 = (assign6230_e6397 / assign6230_e6405);
        (assign6230_e6406, ((((assign6230_e6395 * locals.var_evbc3_dn0) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn0) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn1) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn1) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((((((assign6230_e6391 * locals.var_ibx_t_dn3) * locals.var_tauex_t) + (assign6230_e6393 * locals.var_tauex_t_dn3)) * locals.var_evbc3) + (assign6230_e6395 * locals.var_evbc3_dn3)) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn3) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), (-((assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn4) / (2.0 * assign6230_e6404))) / (assign6230_e6405 * assign6230_e6405))), ((((assign6230_e6395 * locals.var_evbc3_dn5) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn5) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn6) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn6) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn7) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn7) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn8) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn8) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn9) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn9) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * locals.var_evbc3_dn10) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * locals.var_evbc3vdcex_dn10) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9, locals.var_xqmex_dn10,)
    }
};
        locals.var_xqmex = assign6230_e6408;
        locals.var_xqmex_dn0 = assign6230_e6408_d_n0;
        locals.var_xqmex_dn1 = assign6230_e6408_d_n1;
        locals.var_xqmex_dn3 = assign6230_e6408_d_n3;
        locals.var_xqmex_dn4 = assign6230_e6408_d_n4;
        locals.var_xqmex_dn5 = assign6230_e6408_d_n5;
        locals.var_xqmex_dn6 = assign6230_e6408_d_n6;
        locals.var_xqmex_dn7 = assign6230_e6408_d_n7;
        locals.var_xqmex_dn8 = assign6230_e6408_d_n8;
        locals.var_xqmex_dn9 = assign6230_e6408_d_n9;
        locals.var_xqmex_dn10 = assign6230_e6408_d_n10;
        locals.var_xqmex_rv = 0.0;

        let (assign6240_e6414, assign6240_e6414_d_n0, assign6240_e6414_d_n1, assign6240_e6414_d_n3, assign6240_e6414_d_n4, assign6240_e6414_d_n5, assign6240_e6414_d_n6, assign6240_e6414_d_n7, assign6240_e6414_d_n8, assign6240_e6414_d_n9, assign6240_e6414_d_n10,) = {
    if (locals.var_guard112 != 0.0) {
        let assign6240_e6412: f64 = (locals.var_fex * locals.var_xqmex);
        (assign6240_e6412, ((locals.var_fex_dn0 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn0)), ((locals.var_fex_dn1 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn1)), ((locals.var_fex_dn3 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn3)), ((locals.var_fex_dn4 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn4)), ((locals.var_fex_dn5 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn5)), ((locals.var_fex_dn6 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn6)), ((locals.var_fex_dn7 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn7)), ((locals.var_fex_dn8 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn8)), ((locals.var_fex_dn9 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn9)), ((locals.var_fex_dn10 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn10)),)
    } else {
        (locals.var_xqex, locals.var_xqex_dn0, locals.var_xqex_dn1, locals.var_xqex_dn3, locals.var_xqex_dn4, locals.var_xqex_dn5, locals.var_xqex_dn6, locals.var_xqex_dn7, locals.var_xqex_dn8, locals.var_xqex_dn9, locals.var_xqex_dn10,)
    }
};
        locals.var_xqex = assign6240_e6414;
        locals.var_xqex_dn0 = assign6240_e6414_d_n0;
        locals.var_xqex_dn1 = assign6240_e6414_d_n1;
        locals.var_xqex_dn3 = assign6240_e6414_d_n3;
        locals.var_xqex_dn4 = assign6240_e6414_d_n4;
        locals.var_xqex_dn5 = assign6240_e6414_d_n5;
        locals.var_xqex_dn6 = assign6240_e6414_d_n6;
        locals.var_xqex_dn7 = assign6240_e6414_d_n7;
        locals.var_xqex_dn8 = assign6240_e6414_d_n8;
        locals.var_xqex_dn9 = assign6240_e6414_d_n9;
        locals.var_xqex_dn10 = assign6240_e6414_d_n10;
        locals.var_xqex_rv = 0.0;

        let assign6250_e6417: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign6250_e6417;
        locals.var_guard115_rv = 0.0;

        let (assign6260_e6430, assign6260_e6430_d_n0, assign6260_e6430_d_n1, assign6260_e6430_d_n3, assign6260_e6430_d_n4, assign6260_e6430_d_n5, assign6260_e6430_d_n6, assign6260_e6430_d_n7, assign6260_e6430_d_n8, assign6260_e6430_d_n9, assign6260_e6430_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6260_e6422: f64 = (locals.var_vje * locals.var_inv_vde_t);
        let assign6260_e6423: f64 = (1.0 - assign6260_e6422);
        let assign6260_e6425: f64 = (-p.p66);
        let assign6260_e6426: f64 = (assign6260_e6423).powf(assign6260_e6425);
        let assign6260_e6428: f64 = (assign6260_e6426 - 3.0);
        (assign6260_e6428, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))) / assign6260_e6423))) },)
    } else {
        (locals.var_dvtevje, locals.var_dvtevje_dn0, locals.var_dvtevje_dn1, locals.var_dvtevje_dn3, locals.var_dvtevje_dn4, locals.var_dvtevje_dn5, locals.var_dvtevje_dn6, locals.var_dvtevje_dn7, locals.var_dvtevje_dn8, locals.var_dvtevje_dn9, locals.var_dvtevje_dn10,)
    }
};
        locals.var_dvtevje = assign6260_e6430;
        locals.var_dvtevje_dn0 = assign6260_e6430_d_n0;
        locals.var_dvtevje_dn1 = assign6260_e6430_d_n1;
        locals.var_dvtevje_dn3 = assign6260_e6430_d_n3;
        locals.var_dvtevje_dn4 = assign6260_e6430_d_n4;
        locals.var_dvtevje_dn5 = assign6260_e6430_d_n5;
        locals.var_dvtevje_dn6 = assign6260_e6430_d_n6;
        locals.var_dvtevje_dn7 = assign6260_e6430_d_n7;
        locals.var_dvtevje_dn8 = assign6260_e6430_d_n8;
        locals.var_dvtevje_dn9 = assign6260_e6430_d_n9;
        locals.var_dvtevje_dn10 = assign6260_e6430_d_n10;
        locals.var_dvtevje_rv = 0.0;

        let (assign6270_e6438, assign6270_e6438_d_n0, assign6270_e6438_d_n1, assign6270_e6438_d_n3, assign6270_e6438_d_n4, assign6270_e6438_d_n5, assign6270_e6438_d_n6, assign6270_e6438_d_n7, assign6270_e6438_d_n8, assign6270_e6438_d_n9, assign6270_e6438_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6270_e6434: f64 = (locals.var_vb2e1 - locals.var_vfe);
        let assign6270_e6436: f64 = (assign6270_e6434 / locals.var_a_vde);
        (assign6270_e6436, ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn4 - locals.var_vfe_dn4) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn5) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn6 - locals.var_vfe_dn6) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign6270_e6434 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde)),)
    } else {
        (locals.var_vb2e1vfe, locals.var_vb2e1vfe_dn0, locals.var_vb2e1vfe_dn1, locals.var_vb2e1vfe_dn3, locals.var_vb2e1vfe_dn4, locals.var_vb2e1vfe_dn5, locals.var_vb2e1vfe_dn6, locals.var_vb2e1vfe_dn7, locals.var_vb2e1vfe_dn8, locals.var_vb2e1vfe_dn9, locals.var_vb2e1vfe_dn10,)
    }
};
        locals.var_vb2e1vfe = assign6270_e6438;
        locals.var_vb2e1vfe_dn0 = assign6270_e6438_d_n0;
        locals.var_vb2e1vfe_dn1 = assign6270_e6438_d_n1;
        locals.var_vb2e1vfe_dn3 = assign6270_e6438_d_n3;
        locals.var_vb2e1vfe_dn4 = assign6270_e6438_d_n4;
        locals.var_vb2e1vfe_dn5 = assign6270_e6438_d_n5;
        locals.var_vb2e1vfe_dn6 = assign6270_e6438_d_n6;
        locals.var_vb2e1vfe_dn7 = assign6270_e6438_d_n7;
        locals.var_vb2e1vfe_dn8 = assign6270_e6438_d_n8;
        locals.var_vb2e1vfe_dn9 = assign6270_e6438_d_n9;
        locals.var_vb2e1vfe_dn10 = assign6270_e6438_d_n10;
        locals.var_vb2e1vfe_rv = 0.0;

        let assign6280_e6441: f64 = if locals.var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign6280_e6441;
        locals.var_guard116_rv = 0.0;

        let (assign6290_e6452, assign6290_e6452_d_n0, assign6290_e6452_d_n1, assign6290_e6452_d_n3, assign6290_e6452_d_n4, assign6290_e6452_d_n5, assign6290_e6452_d_n6, assign6290_e6452_d_n7, assign6290_e6452_d_n8, assign6290_e6452_d_n9, assign6290_e6452_d_n10,) = {
    if ((locals.var_guard115 != 0.0) && (locals.var_guard116 != 0.0)) {
        let assign6290_e6448: f64 = (locals.var_vb2e1vfe).exp();
        let assign6290_e6449: f64 = (1.0 + assign6290_e6448);
        let assign6290_e6450: f64 = (1.0 / assign6290_e6449);
        (assign6290_e6450, (-((assign6290_e6448 * locals.var_vb2e1vfe_dn0) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn1) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn3) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn4) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn5) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn6) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn7) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn8) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn9) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * locals.var_vb2e1vfe_dn10) / (assign6290_e6449 * assign6290_e6449))),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9, locals.var_dvjevb2e1_dn10,)
    }
};
        locals.var_dvjevb2e1 = assign6290_e6452;
        locals.var_dvjevb2e1_dn0 = assign6290_e6452_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6290_e6452_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6290_e6452_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6290_e6452_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6290_e6452_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6290_e6452_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6290_e6452_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6290_e6452_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6290_e6452_d_n9;
        locals.var_dvjevb2e1_dn10 = assign6290_e6452_d_n10;
        locals.var_dvjevb2e1_rv = 0.0;

        let (assign6300_e6467, assign6300_e6467_d_n0, assign6300_e6467_d_n1, assign6300_e6467_d_n3, assign6300_e6467_d_n4, assign6300_e6467_d_n5, assign6300_e6467_d_n6, assign6300_e6467_d_n7, assign6300_e6467_d_n8, assign6300_e6467_d_n9, assign6300_e6467_d_n10,) = {
    if ((locals.var_guard115 != 0.0) && (locals.var_guard116 == 0.0)) {
        let assign6300_e6458: f64 = (-locals.var_vb2e1vfe);
        let assign6300_e6459: f64 = (assign6300_e6458).exp();
        let assign6300_e6462: f64 = (-locals.var_vb2e1vfe);
        let assign6300_e6463: f64 = (assign6300_e6462).exp();
        let assign6300_e6464: f64 = (1.0 + assign6300_e6463);
        let assign6300_e6465: f64 = (assign6300_e6459 / assign6300_e6464);
        (assign6300_e6465, ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn0)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn0)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn1)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn1)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn3)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn3)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn4)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn4)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn5)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn5)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn6)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn6)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn7)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn7)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn8)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn8)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn9)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn9)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-locals.var_vb2e1vfe_dn10)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-locals.var_vb2e1vfe_dn10)))) / (assign6300_e6464 * assign6300_e6464)),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9, locals.var_dvjevb2e1_dn10,)
    }
};
        locals.var_dvjevb2e1 = assign6300_e6467;
        locals.var_dvjevb2e1_dn0 = assign6300_e6467_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6300_e6467_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6300_e6467_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6300_e6467_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6300_e6467_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6300_e6467_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6300_e6467_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6300_e6467_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6300_e6467_d_n9;
        locals.var_dvjevb2e1_dn10 = assign6300_e6467_d_n10;
        locals.var_dvjevb2e1_rv = 0.0;

        let (assign6310_e6475, assign6310_e6475_d_n0, assign6310_e6475_d_n1, assign6310_e6475_d_n3, assign6310_e6475_d_n4, assign6310_e6475_d_n5, assign6310_e6475_d_n6, assign6310_e6475_d_n7, assign6310_e6475_d_n8, assign6310_e6475_d_n9, assign6310_e6475_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6310_e6471: f64 = (locals.var_dvtevje * locals.var_dvjevb2e1);
        let assign6310_e6473: f64 = (assign6310_e6471 + 3.0);
        (assign6310_e6473, ((locals.var_dvtevje_dn0 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn0)), ((locals.var_dvtevje_dn1 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn1)), ((locals.var_dvtevje_dn3 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn3)), ((locals.var_dvtevje_dn4 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn4)), ((locals.var_dvtevje_dn5 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn5)), ((locals.var_dvtevje_dn6 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn6)), ((locals.var_dvtevje_dn7 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn7)), ((locals.var_dvtevje_dn8 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn8)), ((locals.var_dvtevje_dn9 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn9)), ((locals.var_dvtevje_dn10 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn10)),)
    } else {
        (locals.var_dvtevb2e1, locals.var_dvtevb2e1_dn0, locals.var_dvtevb2e1_dn1, locals.var_dvtevb2e1_dn3, locals.var_dvtevb2e1_dn4, locals.var_dvtevb2e1_dn5, locals.var_dvtevb2e1_dn6, locals.var_dvtevb2e1_dn7, locals.var_dvtevb2e1_dn8, locals.var_dvtevb2e1_dn9, locals.var_dvtevb2e1_dn10,)
    }
};
        locals.var_dvtevb2e1 = assign6310_e6475;
        locals.var_dvtevb2e1_dn0 = assign6310_e6475_d_n0;
        locals.var_dvtevb2e1_dn1 = assign6310_e6475_d_n1;
        locals.var_dvtevb2e1_dn3 = assign6310_e6475_d_n3;
        locals.var_dvtevb2e1_dn4 = assign6310_e6475_d_n4;
        locals.var_dvtevb2e1_dn5 = assign6310_e6475_d_n5;
        locals.var_dvtevb2e1_dn6 = assign6310_e6475_d_n6;
        locals.var_dvtevb2e1_dn7 = assign6310_e6475_d_n7;
        locals.var_dvtevb2e1_dn8 = assign6310_e6475_d_n8;
        locals.var_dvtevb2e1_dn9 = assign6310_e6475_d_n9;
        locals.var_dvtevb2e1_dn10 = assign6310_e6475_d_n10;
        locals.var_dvtevb2e1_rv = 0.0;

        let (assign6320_e6485, assign6320_e6485_d_n0, assign6320_e6485_d_n1, assign6320_e6485_d_n3, assign6320_e6485_d_n4, assign6320_e6485_d_n5, assign6320_e6485_d_n6, assign6320_e6485_d_n7, assign6320_e6485_d_n8, assign6320_e6485_d_n9, assign6320_e6485_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6320_e6479: f64 = (1.0 - p.p67);
        let assign6320_e6481: f64 = (assign6320_e6479 * locals.var_cje_t);
        let assign6320_e6483: f64 = (assign6320_e6481 * locals.var_dvtevb2e1);
        (assign6320_e6483, (((assign6320_e6479 * locals.var_cje_t_dn0) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn0)), (((assign6320_e6479 * locals.var_cje_t_dn1) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn1)), (((assign6320_e6479 * locals.var_cje_t_dn3) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn3)), (((assign6320_e6479 * locals.var_cje_t_dn4) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn4)), (((assign6320_e6479 * locals.var_cje_t_dn5) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn5)), (((assign6320_e6479 * locals.var_cje_t_dn6) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn6)), (((assign6320_e6479 * locals.var_cje_t_dn7) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn7)), (((assign6320_e6479 * locals.var_cje_t_dn8) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn8)), (((assign6320_e6479 * locals.var_cje_t_dn9) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn9)), (((assign6320_e6479 * locals.var_cje_t_dn10) * locals.var_dvtevb2e1) + (assign6320_e6481 * locals.var_dvtevb2e1_dn10)),)
    } else {
        (locals.var_dqtevb2e1, locals.var_dqtevb2e1_dn0, locals.var_dqtevb2e1_dn1, locals.var_dqtevb2e1_dn3, locals.var_dqtevb2e1_dn4, locals.var_dqtevb2e1_dn5, locals.var_dqtevb2e1_dn6, locals.var_dqtevb2e1_dn7, locals.var_dqtevb2e1_dn8, locals.var_dqtevb2e1_dn9, locals.var_dqtevb2e1_dn10,)
    }
};
        locals.var_dqtevb2e1 = assign6320_e6485;
        locals.var_dqtevb2e1_dn0 = assign6320_e6485_d_n0;
        locals.var_dqtevb2e1_dn1 = assign6320_e6485_d_n1;
        locals.var_dqtevb2e1_dn3 = assign6320_e6485_d_n3;
        locals.var_dqtevb2e1_dn4 = assign6320_e6485_d_n4;
        locals.var_dqtevb2e1_dn5 = assign6320_e6485_d_n5;
        locals.var_dqtevb2e1_dn6 = assign6320_e6485_d_n6;
        locals.var_dqtevb2e1_dn7 = assign6320_e6485_d_n7;
        locals.var_dqtevb2e1_dn8 = assign6320_e6485_d_n8;
        locals.var_dqtevb2e1_dn9 = assign6320_e6485_d_n9;
        locals.var_dqtevb2e1_dn10 = assign6320_e6485_d_n10;
        locals.var_dqtevb2e1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let (assign6330_e6502, assign6330_e6502_d_n0, assign6330_e6502_d_n1, assign6330_e6502_d_n3, assign6330_e6502_d_n4, assign6330_e6502_d_n5, assign6330_e6502_d_n6, assign6330_e6502_d_n7, assign6330_e6502_d_n8, assign6330_e6502_d_n9, assign6330_e6502_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6330_e6489: f64 = (locals.var_if0 * locals.var_evb2e1);
        let assign6330_e6491: f64 = (assign6330_e6489 * locals.var_vtinv);
        let assign6330_e6493: f64 = (assign6330_e6491 / locals.var_nff_t);
        let assign6330_e6497: f64 = (1.0 + locals.var_f1);
        let assign6330_e6498: f64 = (assign6330_e6497).sqrt();
        let assign6330_e6499: f64 = (0.5 / assign6330_e6498);
        let assign6330_e6500: f64 = (assign6330_e6493 * assign6330_e6499);
        (assign6330_e6500, ((((((((locals.var_if0_dn0 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn0)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn0)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn0 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn1 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn1)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn1)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn1 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((locals.var_if0_dn3 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn3)) * locals.var_vtinv) + (assign6330_e6489 * locals.var_vtinv_dn3)) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn3 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn4 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn4)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn4 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn5 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn5)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn5 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn6 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn6)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn6)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn6 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn7 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn7)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn7)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn7 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn8 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn8)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn8)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn8 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn9 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn9)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn9)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn9 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((locals.var_if0_dn10 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn10)) * locals.var_vtinv) * locals.var_nff_t) - (assign6330_e6491 * locals.var_nff_t_dn10)) / (locals.var_nff_t * locals.var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (locals.var_f1_dn10 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))),)
    } else {
        (locals.var_dn0vb2e1, locals.var_dn0vb2e1_dn0, locals.var_dn0vb2e1_dn1, locals.var_dn0vb2e1_dn3, locals.var_dn0vb2e1_dn4, locals.var_dn0vb2e1_dn5, locals.var_dn0vb2e1_dn6, locals.var_dn0vb2e1_dn7, locals.var_dn0vb2e1_dn8, locals.var_dn0vb2e1_dn9, locals.var_dn0vb2e1_dn10,)
    }
};
        locals.var_dn0vb2e1 = assign6330_e6502;
        locals.var_dn0vb2e1_dn0 = assign6330_e6502_d_n0;
        locals.var_dn0vb2e1_dn1 = assign6330_e6502_d_n1;
        locals.var_dn0vb2e1_dn3 = assign6330_e6502_d_n3;
        locals.var_dn0vb2e1_dn4 = assign6330_e6502_d_n4;
        locals.var_dn0vb2e1_dn5 = assign6330_e6502_d_n5;
        locals.var_dn0vb2e1_dn6 = assign6330_e6502_d_n6;
        locals.var_dn0vb2e1_dn7 = assign6330_e6502_d_n7;
        locals.var_dn0vb2e1_dn8 = assign6330_e6502_d_n8;
        locals.var_dn0vb2e1_dn9 = assign6330_e6502_d_n9;
        locals.var_dn0vb2e1_dn10 = assign6330_e6502_d_n10;
        locals.var_dn0vb2e1_rv = 0.0;

        let (assign6340_e6512, assign6340_e6512_d_n0, assign6340_e6512_d_n1, assign6340_e6512_d_n3, assign6340_e6512_d_n4, assign6340_e6512_d_n5, assign6340_e6512_d_n6, assign6340_e6512_d_n7, assign6340_e6512_d_n8, assign6340_e6512_d_n9, assign6340_e6512_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6340_e6506: f64 = (0.5 * locals.var_qb0);
        let assign6340_e6508: f64 = (assign6340_e6506 * locals.var_q1q);
        let assign6340_e6510: f64 = (assign6340_e6508 * locals.var_dn0vb2e1);
        (assign6340_e6510, (((assign6340_e6506 * locals.var_q1q_dn0) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn0)), (((assign6340_e6506 * locals.var_q1q_dn1) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn1)), (((((0.5 * locals.var_qb0_dn3) * locals.var_q1q) + (assign6340_e6506 * locals.var_q1q_dn3)) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn3)), (((assign6340_e6506 * locals.var_q1q_dn4) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn4)), (((assign6340_e6506 * locals.var_q1q_dn5) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn5)), (((assign6340_e6506 * locals.var_q1q_dn6) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn6)), (((assign6340_e6506 * locals.var_q1q_dn7) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn7)), (((assign6340_e6506 * locals.var_q1q_dn8) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn8)), (((assign6340_e6506 * locals.var_q1q_dn9) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn9)), (((assign6340_e6506 * locals.var_q1q_dn10) * locals.var_dn0vb2e1) + (assign6340_e6508 * locals.var_dn0vb2e1_dn10)),)
    } else {
        (locals.var_dqbevb2e1, locals.var_dqbevb2e1_dn0, locals.var_dqbevb2e1_dn1, locals.var_dqbevb2e1_dn3, locals.var_dqbevb2e1_dn4, locals.var_dqbevb2e1_dn5, locals.var_dqbevb2e1_dn6, locals.var_dqbevb2e1_dn7, locals.var_dqbevb2e1_dn8, locals.var_dqbevb2e1_dn9, locals.var_dqbevb2e1_dn10,)
    }
};
        locals.var_dqbevb2e1 = assign6340_e6512;
        locals.var_dqbevb2e1_dn0 = assign6340_e6512_d_n0;
        locals.var_dqbevb2e1_dn1 = assign6340_e6512_d_n1;
        locals.var_dqbevb2e1_dn3 = assign6340_e6512_d_n3;
        locals.var_dqbevb2e1_dn4 = assign6340_e6512_d_n4;
        locals.var_dqbevb2e1_dn5 = assign6340_e6512_d_n5;
        locals.var_dqbevb2e1_dn6 = assign6340_e6512_d_n6;
        locals.var_dqbevb2e1_dn7 = assign6340_e6512_d_n7;
        locals.var_dqbevb2e1_dn8 = assign6340_e6512_d_n8;
        locals.var_dqbevb2e1_dn9 = assign6340_e6512_d_n9;
        locals.var_dqbevb2e1_dn10 = assign6340_e6512_d_n10;
        locals.var_dqbevb2e1_rv = 0.0;

        let (assign6350_e6520, assign6350_e6520_d_n0, assign6350_e6520_d_n1, assign6350_e6520_d_n3, assign6350_e6520_d_n4, assign6350_e6520_d_n5, assign6350_e6520_d_n6, assign6350_e6520_d_n7, assign6350_e6520_d_n8, assign6350_e6520_d_n9, assign6350_e6520_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6350_e6517: f64 = (p.p84 * locals.var_vt);
        let assign6350_e6518: f64 = (locals.var_qe_qs / assign6350_e6517);
        (assign6350_e6518, (locals.var_qe_qs_dn0 / assign6350_e6517), (locals.var_qe_qs_dn1 / assign6350_e6517), (((locals.var_qe_qs_dn3 * assign6350_e6517) - (locals.var_qe_qs * (p.p84 * locals.var_vt_dn3))) / (assign6350_e6517 * assign6350_e6517)), (locals.var_qe_qs_dn4 / assign6350_e6517), (locals.var_qe_qs_dn5 / assign6350_e6517), (locals.var_qe_qs_dn6 / assign6350_e6517), (locals.var_qe_qs_dn7 / assign6350_e6517), (locals.var_qe_qs_dn8 / assign6350_e6517), (locals.var_qe_qs_dn9 / assign6350_e6517), (locals.var_qe_qs_dn10 / assign6350_e6517),)
    } else {
        (locals.var_dqevb2e1, locals.var_dqevb2e1_dn0, locals.var_dqevb2e1_dn1, locals.var_dqevb2e1_dn3, locals.var_dqevb2e1_dn4, locals.var_dqevb2e1_dn5, locals.var_dqevb2e1_dn6, locals.var_dqevb2e1_dn7, locals.var_dqevb2e1_dn8, locals.var_dqevb2e1_dn9, locals.var_dqevb2e1_dn10,)
    }
};
        locals.var_dqevb2e1 = assign6350_e6520;
        locals.var_dqevb2e1_dn0 = assign6350_e6520_d_n0;
        locals.var_dqevb2e1_dn1 = assign6350_e6520_d_n1;
        locals.var_dqevb2e1_dn3 = assign6350_e6520_d_n3;
        locals.var_dqevb2e1_dn4 = assign6350_e6520_d_n4;
        locals.var_dqevb2e1_dn5 = assign6350_e6520_d_n5;
        locals.var_dqevb2e1_dn6 = assign6350_e6520_d_n6;
        locals.var_dqevb2e1_dn7 = assign6350_e6520_d_n7;
        locals.var_dqevb2e1_dn8 = assign6350_e6520_d_n8;
        locals.var_dqevb2e1_dn9 = assign6350_e6520_d_n9;
        locals.var_dqevb2e1_dn10 = assign6350_e6520_d_n10;
        locals.var_dqevb2e1_rv = 0.0;

        let (assign6360_e6532, assign6360_e6532_d_n0, assign6360_e6532_d_n1, assign6360_e6532_d_n3, assign6360_e6532_d_n4, assign6360_e6532_d_n5, assign6360_e6532_d_n6, assign6360_e6532_d_n7, assign6360_e6532_d_n8, assign6360_e6532_d_n9, assign6360_e6532_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6360_e6524: f64 = (0.2 * locals.var_vb1b2);
        let assign6360_e6527: f64 = (locals.var_dqtevb2e1 + locals.var_dqbevb2e1);
        let assign6360_e6529: f64 = (assign6360_e6527 + locals.var_dqevb2e1);
        let assign6360_e6530: f64 = (assign6360_e6524 * assign6360_e6529);
        (assign6360_e6530, (assign6360_e6524 * ((locals.var_dqtevb2e1_dn0 + locals.var_dqbevb2e1_dn0) + locals.var_dqevb2e1_dn0)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn1 + locals.var_dqbevb2e1_dn1) + locals.var_dqevb2e1_dn1)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn3 + locals.var_dqbevb2e1_dn3) + locals.var_dqevb2e1_dn3)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn4 + locals.var_dqbevb2e1_dn4) + locals.var_dqevb2e1_dn4)), (((0.2 * locals.var_vb1b2_dn5) * assign6360_e6529) + (assign6360_e6524 * ((locals.var_dqtevb2e1_dn5 + locals.var_dqbevb2e1_dn5) + locals.var_dqevb2e1_dn5))), (((0.2 * locals.var_vb1b2_dn6) * assign6360_e6529) + (assign6360_e6524 * ((locals.var_dqtevb2e1_dn6 + locals.var_dqbevb2e1_dn6) + locals.var_dqevb2e1_dn6))), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn7 + locals.var_dqbevb2e1_dn7) + locals.var_dqevb2e1_dn7)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn8 + locals.var_dqbevb2e1_dn8) + locals.var_dqevb2e1_dn8)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn9 + locals.var_dqbevb2e1_dn9) + locals.var_dqevb2e1_dn9)), (assign6360_e6524 * ((locals.var_dqtevb2e1_dn10 + locals.var_dqbevb2e1_dn10) + locals.var_dqevb2e1_dn10)),)
    } else {
        (locals.var_qb1b2, locals.var_qb1b2_dn0, locals.var_qb1b2_dn1, locals.var_qb1b2_dn3, locals.var_qb1b2_dn4, locals.var_qb1b2_dn5, locals.var_qb1b2_dn6, locals.var_qb1b2_dn7, locals.var_qb1b2_dn8, locals.var_qb1b2_dn9, locals.var_qb1b2_dn10,)
    }
};
        locals.var_qb1b2 = assign6360_e6532;
        locals.var_qb1b2_dn0 = assign6360_e6532_d_n0;
        locals.var_qb1b2_dn1 = assign6360_e6532_d_n1;
        locals.var_qb1b2_dn3 = assign6360_e6532_d_n3;
        locals.var_qb1b2_dn4 = assign6360_e6532_d_n4;
        locals.var_qb1b2_dn5 = assign6360_e6532_d_n5;
        locals.var_qb1b2_dn6 = assign6360_e6532_d_n6;
        locals.var_qb1b2_dn7 = assign6360_e6532_d_n7;
        locals.var_qb1b2_dn8 = assign6360_e6532_d_n8;
        locals.var_qb1b2_dn9 = assign6360_e6532_d_n9;
        locals.var_qb1b2_dn10 = assign6360_e6532_d_n10;
        locals.var_qb1b2_rv = 0.0;

        let (assign6370_e6540, assign6370_e6540_d_n0, assign6370_e6540_d_n1, assign6370_e6540_d_n3, assign6370_e6540_d_n4, assign6370_e6540_d_n5, assign6370_e6540_d_n6, assign6370_e6540_d_n7, assign6370_e6540_d_n8, assign6370_e6540_d_n9, assign6370_e6540_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6370_e6536: f64 = (1.0 - p.p94);
        let assign6370_e6538: f64 = (assign6370_e6536 * locals.var_qe_qs);
        (assign6370_e6538, (assign6370_e6536 * locals.var_qe_qs_dn0), (assign6370_e6536 * locals.var_qe_qs_dn1), (assign6370_e6536 * locals.var_qe_qs_dn3), (assign6370_e6536 * locals.var_qe_qs_dn4), (assign6370_e6536 * locals.var_qe_qs_dn5), (assign6370_e6536 * locals.var_qe_qs_dn6), (assign6370_e6536 * locals.var_qe_qs_dn7), (assign6370_e6536 * locals.var_qe_qs_dn8), (assign6370_e6536 * locals.var_qe_qs_dn9), (assign6370_e6536 * locals.var_qe_qs_dn10),)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10,)
    }
};
        locals.var_qe = assign6370_e6540;
        locals.var_qe_dn0 = assign6370_e6540_d_n0;
        locals.var_qe_dn1 = assign6370_e6540_d_n1;
        locals.var_qe_dn3 = assign6370_e6540_d_n3;
        locals.var_qe_dn4 = assign6370_e6540_d_n4;
        locals.var_qe_dn5 = assign6370_e6540_d_n5;
        locals.var_qe_dn6 = assign6370_e6540_d_n6;
        locals.var_qe_dn7 = assign6370_e6540_d_n7;
        locals.var_qe_dn8 = assign6370_e6540_d_n8;
        locals.var_qe_dn9 = assign6370_e6540_d_n9;
        locals.var_qe_dn10 = assign6370_e6540_d_n10;
        locals.var_qe_rv = 0.0;

        let (assign6380_e6548, assign6380_e6548_d_n0, assign6380_e6548_d_n1, assign6380_e6548_d_n3, assign6380_e6548_d_n4, assign6380_e6548_d_n5, assign6380_e6548_d_n6, assign6380_e6548_d_n7, assign6380_e6548_d_n8, assign6380_e6548_d_n9, assign6380_e6548_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6380_e6545: f64 = (p.p94 * locals.var_qe_qs);
        let assign6380_e6546: f64 = (locals.var_qbe_qs + assign6380_e6545);
        (assign6380_e6546, (locals.var_qbe_qs_dn0 + (p.p94 * locals.var_qe_qs_dn0)), (locals.var_qbe_qs_dn1 + (p.p94 * locals.var_qe_qs_dn1)), (locals.var_qbe_qs_dn3 + (p.p94 * locals.var_qe_qs_dn3)), (locals.var_qbe_qs_dn4 + (p.p94 * locals.var_qe_qs_dn4)), (locals.var_qbe_qs_dn5 + (p.p94 * locals.var_qe_qs_dn5)), (locals.var_qbe_qs_dn6 + (p.p94 * locals.var_qe_qs_dn6)), (locals.var_qbe_qs_dn7 + (p.p94 * locals.var_qe_qs_dn7)), (locals.var_qbe_qs_dn8 + (p.p94 * locals.var_qe_qs_dn8)), (locals.var_qbe_qs_dn9 + (p.p94 * locals.var_qe_qs_dn9)), (locals.var_qbe_qs_dn10 + (p.p94 * locals.var_qe_qs_dn10)),)
    } else {
        (locals.var_qbe_qs_eff, locals.var_qbe_qs_eff_dn0, locals.var_qbe_qs_eff_dn1, locals.var_qbe_qs_eff_dn3, locals.var_qbe_qs_eff_dn4, locals.var_qbe_qs_eff_dn5, locals.var_qbe_qs_eff_dn6, locals.var_qbe_qs_eff_dn7, locals.var_qbe_qs_eff_dn8, locals.var_qbe_qs_eff_dn9, locals.var_qbe_qs_eff_dn10,)
    }
};
        locals.var_qbe_qs_eff = assign6380_e6548;
        locals.var_qbe_qs_eff_dn0 = assign6380_e6548_d_n0;
        locals.var_qbe_qs_eff_dn1 = assign6380_e6548_d_n1;
        locals.var_qbe_qs_eff_dn3 = assign6380_e6548_d_n3;
        locals.var_qbe_qs_eff_dn4 = assign6380_e6548_d_n4;
        locals.var_qbe_qs_eff_dn5 = assign6380_e6548_d_n5;
        locals.var_qbe_qs_eff_dn6 = assign6380_e6548_d_n6;
        locals.var_qbe_qs_eff_dn7 = assign6380_e6548_d_n7;
        locals.var_qbe_qs_eff_dn8 = assign6380_e6548_d_n8;
        locals.var_qbe_qs_eff_dn9 = assign6380_e6548_d_n9;
        locals.var_qbe_qs_eff_dn10 = assign6380_e6548_d_n10;
        locals.var_qbe_qs_eff_rv = 0.0;

        let (assign6390_e6556, assign6390_e6556_d_n0, assign6390_e6556_d_n1, assign6390_e6556_d_n3, assign6390_e6556_d_n4, assign6390_e6556_d_n5, assign6390_e6556_d_n6, assign6390_e6556_d_n7, assign6390_e6556_d_n8, assign6390_e6556_d_n9, assign6390_e6556_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6390_e6552: f64 = (p.p93 * locals.var_qbe_qs_eff);
        let assign6390_e6554: f64 = (assign6390_e6552 + locals.var_qbc_qs);
        (assign6390_e6554, ((p.p93 * locals.var_qbe_qs_eff_dn0) + locals.var_qbc_qs_dn0), ((p.p93 * locals.var_qbe_qs_eff_dn1) + locals.var_qbc_qs_dn1), ((p.p93 * locals.var_qbe_qs_eff_dn3) + locals.var_qbc_qs_dn3), ((p.p93 * locals.var_qbe_qs_eff_dn4) + locals.var_qbc_qs_dn4), ((p.p93 * locals.var_qbe_qs_eff_dn5) + locals.var_qbc_qs_dn5), ((p.p93 * locals.var_qbe_qs_eff_dn6) + locals.var_qbc_qs_dn6), ((p.p93 * locals.var_qbe_qs_eff_dn7) + locals.var_qbc_qs_dn7), ((p.p93 * locals.var_qbe_qs_eff_dn8) + locals.var_qbc_qs_dn8), ((p.p93 * locals.var_qbe_qs_eff_dn9) + locals.var_qbc_qs_dn9), ((p.p93 * locals.var_qbe_qs_eff_dn10) + locals.var_qbc_qs_dn10),)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10,)
    }
};
        locals.var_qbc = assign6390_e6556;
        locals.var_qbc_dn0 = assign6390_e6556_d_n0;
        locals.var_qbc_dn1 = assign6390_e6556_d_n1;
        locals.var_qbc_dn3 = assign6390_e6556_d_n3;
        locals.var_qbc_dn4 = assign6390_e6556_d_n4;
        locals.var_qbc_dn5 = assign6390_e6556_d_n5;
        locals.var_qbc_dn6 = assign6390_e6556_d_n6;
        locals.var_qbc_dn7 = assign6390_e6556_d_n7;
        locals.var_qbc_dn8 = assign6390_e6556_d_n8;
        locals.var_qbc_dn9 = assign6390_e6556_d_n9;
        locals.var_qbc_dn10 = assign6390_e6556_d_n10;
        locals.var_qbc_rv = 0.0;

        let (assign6400_e6564, assign6400_e6564_d_n0, assign6400_e6564_d_n1, assign6400_e6564_d_n3, assign6400_e6564_d_n4, assign6400_e6564_d_n5, assign6400_e6564_d_n6, assign6400_e6564_d_n7, assign6400_e6564_d_n8, assign6400_e6564_d_n9, assign6400_e6564_d_n10,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6400_e6560: f64 = (1.0 - p.p93);
        let assign6400_e6562: f64 = (assign6400_e6560 * locals.var_qbe_qs_eff);
        (assign6400_e6562, (assign6400_e6560 * locals.var_qbe_qs_eff_dn0), (assign6400_e6560 * locals.var_qbe_qs_eff_dn1), (assign6400_e6560 * locals.var_qbe_qs_eff_dn3), (assign6400_e6560 * locals.var_qbe_qs_eff_dn4), (assign6400_e6560 * locals.var_qbe_qs_eff_dn5), (assign6400_e6560 * locals.var_qbe_qs_eff_dn6), (assign6400_e6560 * locals.var_qbe_qs_eff_dn7), (assign6400_e6560 * locals.var_qbe_qs_eff_dn8), (assign6400_e6560 * locals.var_qbe_qs_eff_dn9), (assign6400_e6560 * locals.var_qbe_qs_eff_dn10),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10,)
    }
};
        locals.var_qbe = assign6400_e6564;
        locals.var_qbe_dn0 = assign6400_e6564_d_n0;
        locals.var_qbe_dn1 = assign6400_e6564_d_n1;
        locals.var_qbe_dn3 = assign6400_e6564_d_n3;
        locals.var_qbe_dn4 = assign6400_e6564_d_n4;
        locals.var_qbe_dn5 = assign6400_e6564_d_n5;
        locals.var_qbe_dn6 = assign6400_e6564_d_n6;
        locals.var_qbe_dn7 = assign6400_e6564_d_n7;
        locals.var_qbe_dn8 = assign6400_e6564_d_n8;
        locals.var_qbe_dn9 = assign6400_e6564_d_n9;
        locals.var_qbe_dn10 = assign6400_e6564_d_n10;
        locals.var_qbe_rv = 0.0;

        let (assign6410_e6569, assign6410_e6569_d_n0, assign6410_e6569_d_n1, assign6410_e6569_d_n3, assign6410_e6569_d_n4, assign6410_e6569_d_n5, assign6410_e6569_d_n6, assign6410_e6569_d_n7, assign6410_e6569_d_n8, assign6410_e6569_d_n9, assign6410_e6569_d_n10,) = {
    if (locals.var_guard115 == 0.0) {
        (locals.var_qbe_qs, locals.var_qbe_qs_dn0, locals.var_qbe_qs_dn1, locals.var_qbe_qs_dn3, locals.var_qbe_qs_dn4, locals.var_qbe_qs_dn5, locals.var_qbe_qs_dn6, locals.var_qbe_qs_dn7, locals.var_qbe_qs_dn8, locals.var_qbe_qs_dn9, locals.var_qbe_qs_dn10,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10,)
    }
};
        locals.var_qbe = assign6410_e6569;
        locals.var_qbe_dn0 = assign6410_e6569_d_n0;
        locals.var_qbe_dn1 = assign6410_e6569_d_n1;
        locals.var_qbe_dn3 = assign6410_e6569_d_n3;
        locals.var_qbe_dn4 = assign6410_e6569_d_n4;
        locals.var_qbe_dn5 = assign6410_e6569_d_n5;
        locals.var_qbe_dn6 = assign6410_e6569_d_n6;
        locals.var_qbe_dn7 = assign6410_e6569_d_n7;
        locals.var_qbe_dn8 = assign6410_e6569_d_n8;
        locals.var_qbe_dn9 = assign6410_e6569_d_n9;
        locals.var_qbe_dn10 = assign6410_e6569_d_n10;
        locals.var_qbe_rv = 0.0;

        let (assign6420_e6574, assign6420_e6574_d_n0, assign6420_e6574_d_n1, assign6420_e6574_d_n3, assign6420_e6574_d_n4, assign6420_e6574_d_n5, assign6420_e6574_d_n6, assign6420_e6574_d_n7, assign6420_e6574_d_n8, assign6420_e6574_d_n9, assign6420_e6574_d_n10,) = {
    if (locals.var_guard115 == 0.0) {
        (locals.var_qbc_qs, locals.var_qbc_qs_dn0, locals.var_qbc_qs_dn1, locals.var_qbc_qs_dn3, locals.var_qbc_qs_dn4, locals.var_qbc_qs_dn5, locals.var_qbc_qs_dn6, locals.var_qbc_qs_dn7, locals.var_qbc_qs_dn8, locals.var_qbc_qs_dn9, locals.var_qbc_qs_dn10,)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10,)
    }
};
        locals.var_qbc = assign6420_e6574;
        locals.var_qbc_dn0 = assign6420_e6574_d_n0;
        locals.var_qbc_dn1 = assign6420_e6574_d_n1;
        locals.var_qbc_dn3 = assign6420_e6574_d_n3;
        locals.var_qbc_dn4 = assign6420_e6574_d_n4;
        locals.var_qbc_dn5 = assign6420_e6574_d_n5;
        locals.var_qbc_dn6 = assign6420_e6574_d_n6;
        locals.var_qbc_dn7 = assign6420_e6574_d_n7;
        locals.var_qbc_dn8 = assign6420_e6574_d_n8;
        locals.var_qbc_dn9 = assign6420_e6574_d_n9;
        locals.var_qbc_dn10 = assign6420_e6574_d_n10;
        locals.var_qbc_rv = 0.0;

        let (assign6430_e6579, assign6430_e6579_d_n0, assign6430_e6579_d_n1, assign6430_e6579_d_n3, assign6430_e6579_d_n4, assign6430_e6579_d_n5, assign6430_e6579_d_n6, assign6430_e6579_d_n7, assign6430_e6579_d_n8, assign6430_e6579_d_n9, assign6430_e6579_d_n10,) = {
    if (locals.var_guard115 == 0.0) {
        (locals.var_qe_qs, locals.var_qe_qs_dn0, locals.var_qe_qs_dn1, locals.var_qe_qs_dn3, locals.var_qe_qs_dn4, locals.var_qe_qs_dn5, locals.var_qe_qs_dn6, locals.var_qe_qs_dn7, locals.var_qe_qs_dn8, locals.var_qe_qs_dn9, locals.var_qe_qs_dn10,)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10,)
    }
};
        locals.var_qe = assign6430_e6579;
        locals.var_qe_dn0 = assign6430_e6579_d_n0;
        locals.var_qe_dn1 = assign6430_e6579_d_n1;
        locals.var_qe_dn3 = assign6430_e6579_d_n3;
        locals.var_qe_dn4 = assign6430_e6579_d_n4;
        locals.var_qe_dn5 = assign6430_e6579_d_n5;
        locals.var_qe_dn6 = assign6430_e6579_d_n6;
        locals.var_qe_dn7 = assign6430_e6579_d_n7;
        locals.var_qe_dn8 = assign6430_e6579_d_n8;
        locals.var_qe_dn9 = assign6430_e6579_d_n9;
        locals.var_qe_dn10 = assign6430_e6579_d_n10;
        locals.var_qe_rv = 0.0;

        let assign6450_e6585: f64 = (p.p134 * (nv3 - 0.0));
        let assign6450_e6586_q: f64 = assign6450_e6585;
        let assign6450_e6588: f64 = (assign6450_e6585 * p.p1);
        let assign6450_e6588_q: f64 = (assign6450_e6586_q * p.p1);
        locals.var_i_cth = assign6450_e6588;
        locals.var_i_cth_dn3 = (p.p134 * p.p1);
        locals.var_i_cth_rv = assign6450_e6588_q;
        locals.var_i_cth_rdn3 = (p.p134 * p.p1);

        let assign6630_e6704: f64 = (locals.var_if_ + locals.var_ir);
        let assign6630_e6706: f64 = (assign6630_e6704 / locals.var_qbi);
        locals.var_in_n = assign6630_e6706;
        locals.var_in_n_dn0 = ((((locals.var_if__dn0 + locals.var_ir_dn0) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn1 = ((((locals.var_if__dn1 + locals.var_ir_dn1) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn3 = ((((locals.var_if__dn3 + locals.var_ir_dn3) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn4 = ((((locals.var_if__dn4 + locals.var_ir_dn4) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn5 = ((((locals.var_if__dn5 + locals.var_ir_dn5) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn6 = ((((locals.var_if__dn6 + locals.var_ir_dn6) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn7 = ((((locals.var_if__dn7 + locals.var_ir_dn7) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn8 = ((((locals.var_if__dn8 + locals.var_ir_dn8) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn9 = ((((locals.var_if__dn9 + locals.var_ir_dn9) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn10 = ((((locals.var_if__dn10 + locals.var_ir_dn10) * locals.var_qbi) - (assign6630_e6704 * locals.var_qbi_dn10)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_rv = 0.0;

        let assign6690_e6739: f64 = if locals.var_in_n > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign6690_e6739;
        locals.var_guard124_rv = 0.0;

        let (assign6700_e6747, assign6700_e6747_d_n0, assign6700_e6747_d_n1, assign6700_e6747_d_n3, assign6700_e6747_d_n4, assign6700_e6747_d_n5, assign6700_e6747_d_n6, assign6700_e6747_d_n7, assign6700_e6747_d_n8, assign6700_e6747_d_n9, assign6700_e6747_d_n10,) = {
    if (locals.var_guard124 != 0.0) {
        let assign6700_e6743: f64 = (locals.var_qbe + locals.var_qbc);
        let assign6700_e6745: f64 = (assign6700_e6743 / locals.var_in_n);
        (assign6700_e6745, ((((locals.var_qbe_dn0 + locals.var_qbc_dn0) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn0)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn1 + locals.var_qbc_dn1) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn1)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn3 + locals.var_qbc_dn3) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn3)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn4 + locals.var_qbc_dn4) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn4)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn5 + locals.var_qbc_dn5) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn5)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn6 + locals.var_qbc_dn6) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn6)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn7 + locals.var_qbc_dn7) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn7)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn8 + locals.var_qbc_dn8) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn8)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn9 + locals.var_qbc_dn9) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn9)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn10 + locals.var_qbc_dn10) * locals.var_in_n) - (assign6700_e6743 * locals.var_in_n_dn10)) / (locals.var_in_n * locals.var_in_n)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10,)
    }
};
        locals.var_taub_n = assign6700_e6747;
        locals.var_taub_n_dn0 = assign6700_e6747_d_n0;
        locals.var_taub_n_dn1 = assign6700_e6747_d_n1;
        locals.var_taub_n_dn3 = assign6700_e6747_d_n3;
        locals.var_taub_n_dn4 = assign6700_e6747_d_n4;
        locals.var_taub_n_dn5 = assign6700_e6747_d_n5;
        locals.var_taub_n_dn6 = assign6700_e6747_d_n6;
        locals.var_taub_n_dn7 = assign6700_e6747_d_n7;
        locals.var_taub_n_dn8 = assign6700_e6747_d_n8;
        locals.var_taub_n_dn9 = assign6700_e6747_d_n9;
        locals.var_taub_n_dn10 = assign6700_e6747_d_n10;
        locals.var_taub_n_rv = 0.0;

        let (assign6710_e6756, assign6710_e6756_d_n0, assign6710_e6756_d_n1, assign6710_e6756_d_n3, assign6710_e6756_d_n4, assign6710_e6756_d_n5, assign6710_e6756_d_n6, assign6710_e6756_d_n7, assign6710_e6756_d_n8, assign6710_e6756_d_n9, assign6710_e6756_d_n10,) = {
    if (locals.var_guard124 == 0.0) {
        let assign6710_e6752: f64 = (locals.var_taub_t * locals.var_q1q);
        let assign6710_e6754: f64 = (assign6710_e6752 * locals.var_qbi);
        (assign6710_e6754, (((locals.var_taub_t * locals.var_q1q_dn0) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn0)), (((locals.var_taub_t * locals.var_q1q_dn1) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn1)), ((((locals.var_taub_t_dn3 * locals.var_q1q) + (locals.var_taub_t * locals.var_q1q_dn3)) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn3)), (((locals.var_taub_t * locals.var_q1q_dn4) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn4)), (((locals.var_taub_t * locals.var_q1q_dn5) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn5)), (((locals.var_taub_t * locals.var_q1q_dn6) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn6)), (((locals.var_taub_t * locals.var_q1q_dn7) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn7)), (((locals.var_taub_t * locals.var_q1q_dn8) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn8)), (((locals.var_taub_t * locals.var_q1q_dn9) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn9)), (((locals.var_taub_t * locals.var_q1q_dn10) * locals.var_qbi) + (assign6710_e6752 * locals.var_qbi_dn10)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10,)
    }
};
        locals.var_taub_n = assign6710_e6756;
        locals.var_taub_n_dn0 = assign6710_e6756_d_n0;
        locals.var_taub_n_dn1 = assign6710_e6756_d_n1;
        locals.var_taub_n_dn3 = assign6710_e6756_d_n3;
        locals.var_taub_n_dn4 = assign6710_e6756_d_n4;
        locals.var_taub_n_dn5 = assign6710_e6756_d_n5;
        locals.var_taub_n_dn6 = assign6710_e6756_d_n6;
        locals.var_taub_n_dn7 = assign6710_e6756_d_n7;
        locals.var_taub_n_dn8 = assign6710_e6756_d_n8;
        locals.var_taub_n_dn9 = assign6710_e6756_d_n9;
        locals.var_taub_n_dn10 = assign6710_e6756_d_n10;
        locals.var_taub_n_rv = 0.0;

        let assign6720_e6759: f64 = if p.p130 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign6720_e6759;
        locals.var_guard125_rv = 0.0;

        let (assign6730_e6765, assign6730_e6765_d_n0, assign6730_e6765_d_n1, assign6730_e6765_d_n3, assign6730_e6765_d_n4, assign6730_e6765_d_n5, assign6730_e6765_d_n6, assign6730_e6765_d_n7, assign6730_e6765_d_n8, assign6730_e6765_d_n9, assign6730_e6765_d_n10,) = {
    if (locals.var_guard125 != 0.0) {
        let assign6730_e6763: f64 = (p.p93 * locals.var_taub_n);
        (assign6730_e6763, (p.p93 * locals.var_taub_n_dn0), (p.p93 * locals.var_taub_n_dn1), (p.p93 * locals.var_taub_n_dn3), (p.p93 * locals.var_taub_n_dn4), (p.p93 * locals.var_taub_n_dn5), (p.p93 * locals.var_taub_n_dn6), (p.p93 * locals.var_taub_n_dn7), (p.p93 * locals.var_taub_n_dn8), (p.p93 * locals.var_taub_n_dn9), (p.p93 * locals.var_taub_n_dn10),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10,)
    }
};
        locals.var_taun = assign6730_e6765;
        locals.var_taun_dn0 = assign6730_e6765_d_n0;
        locals.var_taun_dn1 = assign6730_e6765_d_n1;
        locals.var_taun_dn3 = assign6730_e6765_d_n3;
        locals.var_taun_dn4 = assign6730_e6765_d_n4;
        locals.var_taun_dn5 = assign6730_e6765_d_n5;
        locals.var_taun_dn6 = assign6730_e6765_d_n6;
        locals.var_taun_dn7 = assign6730_e6765_d_n7;
        locals.var_taun_dn8 = assign6730_e6765_d_n8;
        locals.var_taun_dn9 = assign6730_e6765_d_n9;
        locals.var_taun_dn10 = assign6730_e6765_d_n10;
        locals.var_taun_rv = 0.0;

        let assign6740_e6768: f64 = if p.p130 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6740_e6768;
        locals.var_guard126_rv = 0.0;

        let (assign6750_e6777, assign6750_e6777_d_n0, assign6750_e6777_d_n1, assign6750_e6777_d_n3, assign6750_e6777_d_n4, assign6750_e6777_d_n5, assign6750_e6777_d_n6, assign6750_e6777_d_n7, assign6750_e6777_d_n8, assign6750_e6777_d_n9, assign6750_e6777_d_n10,) = {
    if ((locals.var_guard125 == 0.0) && (locals.var_guard126 != 0.0)) {
        let assign6750_e6775: f64 = (p.p131 * locals.var_taub_n);
        (assign6750_e6775, (p.p131 * locals.var_taub_n_dn0), (p.p131 * locals.var_taub_n_dn1), (p.p131 * locals.var_taub_n_dn3), (p.p131 * locals.var_taub_n_dn4), (p.p131 * locals.var_taub_n_dn5), (p.p131 * locals.var_taub_n_dn6), (p.p131 * locals.var_taub_n_dn7), (p.p131 * locals.var_taub_n_dn8), (p.p131 * locals.var_taub_n_dn9), (p.p131 * locals.var_taub_n_dn10),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10,)
    }
};
        locals.var_taun = assign6750_e6777;
        locals.var_taun_dn0 = assign6750_e6777_d_n0;
        locals.var_taun_dn1 = assign6750_e6777_d_n1;
        locals.var_taun_dn3 = assign6750_e6777_d_n3;
        locals.var_taun_dn4 = assign6750_e6777_d_n4;
        locals.var_taun_dn5 = assign6750_e6777_d_n5;
        locals.var_taun_dn6 = assign6750_e6777_d_n6;
        locals.var_taun_dn7 = assign6750_e6777_d_n7;
        locals.var_taun_dn8 = assign6750_e6777_d_n8;
        locals.var_taun_dn9 = assign6750_e6777_d_n9;
        locals.var_taun_dn10 = assign6750_e6777_d_n10;
        locals.var_taun_rv = 0.0;

        let (assign6760_e6785, assign6760_e6785_d_n0, assign6760_e6785_d_n1, assign6760_e6785_d_n3, assign6760_e6785_d_n4, assign6760_e6785_d_n5, assign6760_e6785_d_n6, assign6760_e6785_d_n7, assign6760_e6785_d_n8, assign6760_e6785_d_n9, assign6760_e6785_d_n10,) = {
    if ((locals.var_guard125 == 0.0) && (locals.var_guard126 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10,)
    }
};
        locals.var_taun = assign6760_e6785;
        locals.var_taun_dn0 = assign6760_e6785_d_n0;
        locals.var_taun_dn1 = assign6760_e6785_d_n1;
        locals.var_taun_dn3 = assign6760_e6785_d_n3;
        locals.var_taun_dn4 = assign6760_e6785_d_n4;
        locals.var_taun_dn5 = assign6760_e6785_d_n5;
        locals.var_taun_dn6 = assign6760_e6785_d_n6;
        locals.var_taun_dn7 = assign6760_e6785_d_n7;
        locals.var_taun_dn8 = assign6760_e6785_d_n8;
        locals.var_taun_dn9 = assign6760_e6785_d_n9;
        locals.var_taun_dn10 = assign6760_e6785_d_n10;
        locals.var_taun_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq0_e154: f64 = (p.p3 * locals.var_ic1c2);
        let eq0_e154_d_n0: f64 = (p.p3 * locals.var_ic1c2_dn0);
        let eq0_e154_d_n1: f64 = (p.p3 * locals.var_ic1c2_dn1);
        let eq0_e154_d_n3: f64 = (p.p3 * locals.var_ic1c2_dn3);
        let eq0_e154_d_n4: f64 = (p.p3 * locals.var_ic1c2_dn4);
        let eq0_e154_d_n5: f64 = (p.p3 * locals.var_ic1c2_dn5);
        let eq0_e154_d_n6: f64 = (p.p3 * locals.var_ic1c2_dn6);
        let eq0_e154_d_n7: f64 = (p.p3 * locals.var_ic1c2_dn7);
        let eq0_e154_d_n8: f64 = (p.p3 * locals.var_ic1c2_dn8);
        let eq0_e154_d_n9: f64 = (p.p3 * locals.var_ic1c2_dn9);
        let eq0_e154_d_n10: f64 = (p.p3 * locals.var_ic1c2_dn10);
        let eq0_e156: f64 = (eq0_e154 * p.p1);
        let eq0_e156_d_n0: f64 = (eq0_e154_d_n0 * p.p1);
        let eq0_e156_d_n1: f64 = (eq0_e154_d_n1 * p.p1);
        let eq0_e156_d_n3: f64 = (eq0_e154_d_n3 * p.p1);
        let eq0_e156_d_n4: f64 = (eq0_e154_d_n4 * p.p1);
        let eq0_e156_d_n5: f64 = (eq0_e154_d_n5 * p.p1);
        let eq0_e156_d_n6: f64 = (eq0_e154_d_n6 * p.p1);
        let eq0_e156_d_n7: f64 = (eq0_e154_d_n7 * p.p1);
        let eq0_e156_d_n8: f64 = (eq0_e154_d_n8 * p.p1);
        let eq0_e156_d_n9: f64 = (eq0_e154_d_n9 * p.p1);
        let eq0_e156_d_n10: f64 = (eq0_e154_d_n10 * p.p1);
        let eq0_value: f64 = eq0_e156;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq0_e156_d_n0), multiplicity * (eq0_e156_d_n1), multiplicity * (eq0_e156_d_n3), multiplicity * (eq0_e156_d_n4), multiplicity * (eq0_e156_d_n5), multiplicity * (eq0_e156_d_n6), multiplicity * (eq0_e156_d_n7), multiplicity * (eq0_e156_d_n8), multiplicity * (eq0_e156_d_n9), multiplicity * (eq0_e156_d_n10)],
            [],
            [],
            1.0,
        );
        let eq1_e159: f64 = (p.p3 * locals.var_in_);
        let eq1_e159_d_n0: f64 = (p.p3 * locals.var_in__dn0);
        let eq1_e159_d_n1: f64 = (p.p3 * locals.var_in__dn1);
        let eq1_e159_d_n3: f64 = (p.p3 * locals.var_in__dn3);
        let eq1_e159_d_n4: f64 = (p.p3 * locals.var_in__dn4);
        let eq1_e159_d_n5: f64 = (p.p3 * locals.var_in__dn5);
        let eq1_e159_d_n6: f64 = (p.p3 * locals.var_in__dn6);
        let eq1_e159_d_n7: f64 = (p.p3 * locals.var_in__dn7);
        let eq1_e159_d_n8: f64 = (p.p3 * locals.var_in__dn8);
        let eq1_e159_d_n9: f64 = (p.p3 * locals.var_in__dn9);
        let eq1_e159_d_n10: f64 = (p.p3 * locals.var_in__dn10);
        let eq1_e161: f64 = (eq1_e159 * p.p1);
        let eq1_e161_d_n0: f64 = (eq1_e159_d_n0 * p.p1);
        let eq1_e161_d_n1: f64 = (eq1_e159_d_n1 * p.p1);
        let eq1_e161_d_n3: f64 = (eq1_e159_d_n3 * p.p1);
        let eq1_e161_d_n4: f64 = (eq1_e159_d_n4 * p.p1);
        let eq1_e161_d_n5: f64 = (eq1_e159_d_n5 * p.p1);
        let eq1_e161_d_n6: f64 = (eq1_e159_d_n6 * p.p1);
        let eq1_e161_d_n7: f64 = (eq1_e159_d_n7 * p.p1);
        let eq1_e161_d_n8: f64 = (eq1_e159_d_n8 * p.p1);
        let eq1_e161_d_n9: f64 = (eq1_e159_d_n9 * p.p1);
        let eq1_e161_d_n10: f64 = (eq1_e159_d_n10 * p.p1);
        let eq1_value: f64 = eq1_e161;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(4),
            multiplicity * (eq1_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq1_e161_d_n0), multiplicity * (eq1_e161_d_n1), multiplicity * (eq1_e161_d_n3), multiplicity * (eq1_e161_d_n4), multiplicity * (eq1_e161_d_n5), multiplicity * (eq1_e161_d_n6), multiplicity * (eq1_e161_d_n7), multiplicity * (eq1_e161_d_n8), multiplicity * (eq1_e161_d_n9), multiplicity * (eq1_e161_d_n10)],
            [],
            [],
            1.0,
        );
        let eq2_e165: f64 = (locals.var_ib1_s + locals.var_ib2_s);
        let eq2_e165_d_n0: f64 = (locals.var_ib1_s_dn0 + locals.var_ib2_s_dn0);
        let eq2_e165_d_n1: f64 = (locals.var_ib1_s_dn1 + locals.var_ib2_s_dn1);
        let eq2_e165_d_n3: f64 = (locals.var_ib1_s_dn3 + locals.var_ib2_s_dn3);
        let eq2_e165_d_n4: f64 = (locals.var_ib1_s_dn4 + locals.var_ib2_s_dn4);
        let eq2_e165_d_n5: f64 = (locals.var_ib1_s_dn5 + locals.var_ib2_s_dn5);
        let eq2_e165_d_n6: f64 = (locals.var_ib1_s_dn6 + locals.var_ib2_s_dn6);
        let eq2_e165_d_n7: f64 = (locals.var_ib1_s_dn7 + locals.var_ib2_s_dn7);
        let eq2_e165_d_n8: f64 = (locals.var_ib1_s_dn8 + locals.var_ib2_s_dn8);
        let eq2_e165_d_n9: f64 = (locals.var_ib1_s_dn9 + locals.var_ib2_s_dn9);
        let eq2_e165_d_n10: f64 = (locals.var_ib1_s_dn10 + locals.var_ib2_s_dn10);
        let eq2_e167: f64 = (eq2_e165 + locals.var_ibrel);
        let eq2_e167_d_n0: f64 = (eq2_e165_d_n0 + locals.var_ibrel_dn0);
        let eq2_e167_d_n1: f64 = (eq2_e165_d_n1 + locals.var_ibrel_dn1);
        let eq2_e167_d_n3: f64 = (eq2_e165_d_n3 + locals.var_ibrel_dn3);
        let eq2_e167_d_n4: f64 = (eq2_e165_d_n4 + locals.var_ibrel_dn4);
        let eq2_e167_d_n5: f64 = (eq2_e165_d_n5 + locals.var_ibrel_dn5);
        let eq2_e167_d_n6: f64 = (eq2_e165_d_n6 + locals.var_ibrel_dn6);
        let eq2_e167_d_n7: f64 = (eq2_e165_d_n7 + locals.var_ibrel_dn7);
        let eq2_e167_d_n8: f64 = (eq2_e165_d_n8 + locals.var_ibrel_dn8);
        let eq2_e167_d_n9: f64 = (eq2_e165_d_n9 + locals.var_ibrel_dn9);
        let eq2_e167_d_n10: f64 = (eq2_e165_d_n10 + locals.var_ibrel_dn10);
        let eq2_e168: f64 = (p.p3 * eq2_e167);
        let eq2_e168_d_n0: f64 = (p.p3 * eq2_e167_d_n0);
        let eq2_e168_d_n1: f64 = (p.p3 * eq2_e167_d_n1);
        let eq2_e168_d_n3: f64 = (p.p3 * eq2_e167_d_n3);
        let eq2_e168_d_n4: f64 = (p.p3 * eq2_e167_d_n4);
        let eq2_e168_d_n5: f64 = (p.p3 * eq2_e167_d_n5);
        let eq2_e168_d_n6: f64 = (p.p3 * eq2_e167_d_n6);
        let eq2_e168_d_n7: f64 = (p.p3 * eq2_e167_d_n7);
        let eq2_e168_d_n8: f64 = (p.p3 * eq2_e167_d_n8);
        let eq2_e168_d_n9: f64 = (p.p3 * eq2_e167_d_n9);
        let eq2_e168_d_n10: f64 = (p.p3 * eq2_e167_d_n10);
        let eq2_e170: f64 = (eq2_e168 * p.p1);
        let eq2_e170_d_n0: f64 = (eq2_e168_d_n0 * p.p1);
        let eq2_e170_d_n1: f64 = (eq2_e168_d_n1 * p.p1);
        let eq2_e170_d_n3: f64 = (eq2_e168_d_n3 * p.p1);
        let eq2_e170_d_n4: f64 = (eq2_e168_d_n4 * p.p1);
        let eq2_e170_d_n5: f64 = (eq2_e168_d_n5 * p.p1);
        let eq2_e170_d_n6: f64 = (eq2_e168_d_n6 * p.p1);
        let eq2_e170_d_n7: f64 = (eq2_e168_d_n7 * p.p1);
        let eq2_e170_d_n8: f64 = (eq2_e168_d_n8 * p.p1);
        let eq2_e170_d_n9: f64 = (eq2_e168_d_n9 * p.p1);
        let eq2_e170_d_n10: f64 = (eq2_e168_d_n10 * p.p1);
        let eq2_value: f64 = eq2_e170;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq2_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq2_e170_d_n0), multiplicity * (eq2_e170_d_n1), multiplicity * (eq2_e170_d_n3), multiplicity * (eq2_e170_d_n4), multiplicity * (eq2_e170_d_n5), multiplicity * (eq2_e170_d_n6), multiplicity * (eq2_e170_d_n7), multiplicity * (eq2_e170_d_n8), multiplicity * (eq2_e170_d_n9), multiplicity * (eq2_e170_d_n10)],
            [],
            [],
            1.0,
        );
        let eq3_e174: f64 = (locals.var_ib1 + locals.var_ib2);
        let eq3_e174_d_n0: f64 = (locals.var_ib1_dn0 + locals.var_ib2_dn0);
        let eq3_e174_d_n1: f64 = (locals.var_ib1_dn1 + locals.var_ib2_dn1);
        let eq3_e174_d_n3: f64 = (locals.var_ib1_dn3 + locals.var_ib2_dn3);
        let eq3_e174_d_n4: f64 = (locals.var_ib1_dn4 + locals.var_ib2_dn4);
        let eq3_e174_d_n5: f64 = (locals.var_ib1_dn5 + locals.var_ib2_dn5);
        let eq3_e174_d_n6: f64 = (locals.var_ib1_dn6 + locals.var_ib2_dn6);
        let eq3_e174_d_n7: f64 = (locals.var_ib1_dn7 + locals.var_ib2_dn7);
        let eq3_e174_d_n8: f64 = (locals.var_ib1_dn8 + locals.var_ib2_dn8);
        let eq3_e174_d_n9: f64 = (locals.var_ib1_dn9 + locals.var_ib2_dn9);
        let eq3_e174_d_n10: f64 = (locals.var_ib1_dn10 + locals.var_ib2_dn10);
        let eq3_e177: f64 = (locals.var_gmin * locals.var_vb2e1);
        let eq3_e177_d_n4: f64 = (locals.var_gmin * locals.var_vb2e1_dn4);
        let eq3_e177_d_n6: f64 = (locals.var_gmin * locals.var_vb2e1_dn6);
        let eq3_e178: f64 = (eq3_e174 + eq3_e177);
        let eq3_e178_d_n4: f64 = (eq3_e174_d_n4 + eq3_e177_d_n4);
        let eq3_e178_d_n6: f64 = (eq3_e174_d_n6 + eq3_e177_d_n6);
        let eq3_e180: f64 = (eq3_e178 - locals.var_izteb);
        let eq3_e180_d_n0: f64 = (eq3_e174_d_n0 - locals.var_izteb_dn0);
        let eq3_e180_d_n1: f64 = (eq3_e174_d_n1 - locals.var_izteb_dn1);
        let eq3_e180_d_n3: f64 = (eq3_e174_d_n3 - locals.var_izteb_dn3);
        let eq3_e180_d_n4: f64 = (eq3_e178_d_n4 - locals.var_izteb_dn4);
        let eq3_e180_d_n5: f64 = (eq3_e174_d_n5 - locals.var_izteb_dn5);
        let eq3_e180_d_n6: f64 = (eq3_e178_d_n6 - locals.var_izteb_dn6);
        let eq3_e180_d_n7: f64 = (eq3_e174_d_n7 - locals.var_izteb_dn7);
        let eq3_e180_d_n8: f64 = (eq3_e174_d_n8 - locals.var_izteb_dn8);
        let eq3_e180_d_n9: f64 = (eq3_e174_d_n9 - locals.var_izteb_dn9);
        let eq3_e180_d_n10: f64 = (eq3_e174_d_n10 - locals.var_izteb_dn10);
        let eq3_e182: f64 = (eq3_e180 + locals.var_ibtbt);
        let eq3_e182_d_n0: f64 = (eq3_e180_d_n0 + locals.var_ibtbt_dn0);
        let eq3_e182_d_n1: f64 = (eq3_e180_d_n1 + locals.var_ibtbt_dn1);
        let eq3_e182_d_n3: f64 = (eq3_e180_d_n3 + locals.var_ibtbt_dn3);
        let eq3_e182_d_n4: f64 = (eq3_e180_d_n4 + locals.var_ibtbt_dn4);
        let eq3_e182_d_n5: f64 = (eq3_e180_d_n5 + locals.var_ibtbt_dn5);
        let eq3_e182_d_n6: f64 = (eq3_e180_d_n6 + locals.var_ibtbt_dn6);
        let eq3_e182_d_n7: f64 = (eq3_e180_d_n7 + locals.var_ibtbt_dn7);
        let eq3_e182_d_n8: f64 = (eq3_e180_d_n8 + locals.var_ibtbt_dn8);
        let eq3_e182_d_n9: f64 = (eq3_e180_d_n9 + locals.var_ibtbt_dn9);
        let eq3_e182_d_n10: f64 = (eq3_e180_d_n10 + locals.var_ibtbt_dn10);
        let eq3_e184: f64 = (eq3_e182 + locals.var_itat);
        let eq3_e184_d_n0: f64 = (eq3_e182_d_n0 + locals.var_itat_dn0);
        let eq3_e184_d_n1: f64 = (eq3_e182_d_n1 + locals.var_itat_dn1);
        let eq3_e184_d_n3: f64 = (eq3_e182_d_n3 + locals.var_itat_dn3);
        let eq3_e184_d_n4: f64 = (eq3_e182_d_n4 + locals.var_itat_dn4);
        let eq3_e184_d_n5: f64 = (eq3_e182_d_n5 + locals.var_itat_dn5);
        let eq3_e184_d_n6: f64 = (eq3_e182_d_n6 + locals.var_itat_dn6);
        let eq3_e184_d_n7: f64 = (eq3_e182_d_n7 + locals.var_itat_dn7);
        let eq3_e184_d_n8: f64 = (eq3_e182_d_n8 + locals.var_itat_dn8);
        let eq3_e184_d_n9: f64 = (eq3_e182_d_n9 + locals.var_itat_dn9);
        let eq3_e184_d_n10: f64 = (eq3_e182_d_n10 + locals.var_itat_dn10);
        let eq3_e185: f64 = (p.p3 * eq3_e184);
        let eq3_e185_d_n0: f64 = (p.p3 * eq3_e184_d_n0);
        let eq3_e185_d_n1: f64 = (p.p3 * eq3_e184_d_n1);
        let eq3_e185_d_n3: f64 = (p.p3 * eq3_e184_d_n3);
        let eq3_e185_d_n4: f64 = (p.p3 * eq3_e184_d_n4);
        let eq3_e185_d_n5: f64 = (p.p3 * eq3_e184_d_n5);
        let eq3_e185_d_n6: f64 = (p.p3 * eq3_e184_d_n6);
        let eq3_e185_d_n7: f64 = (p.p3 * eq3_e184_d_n7);
        let eq3_e185_d_n8: f64 = (p.p3 * eq3_e184_d_n8);
        let eq3_e185_d_n9: f64 = (p.p3 * eq3_e184_d_n9);
        let eq3_e185_d_n10: f64 = (p.p3 * eq3_e184_d_n10);
        let eq3_e187: f64 = (eq3_e185 * p.p1);
        let eq3_e187_d_n0: f64 = (eq3_e185_d_n0 * p.p1);
        let eq3_e187_d_n1: f64 = (eq3_e185_d_n1 * p.p1);
        let eq3_e187_d_n3: f64 = (eq3_e185_d_n3 * p.p1);
        let eq3_e187_d_n4: f64 = (eq3_e185_d_n4 * p.p1);
        let eq3_e187_d_n5: f64 = (eq3_e185_d_n5 * p.p1);
        let eq3_e187_d_n6: f64 = (eq3_e185_d_n6 * p.p1);
        let eq3_e187_d_n7: f64 = (eq3_e185_d_n7 * p.p1);
        let eq3_e187_d_n8: f64 = (eq3_e185_d_n8 * p.p1);
        let eq3_e187_d_n9: f64 = (eq3_e185_d_n9 * p.p1);
        let eq3_e187_d_n10: f64 = (eq3_e185_d_n10 * p.p1);
        let eq3_value: f64 = eq3_e187;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(4),
            multiplicity * (eq3_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq3_e187_d_n0), multiplicity * (eq3_e187_d_n1), multiplicity * (eq3_e187_d_n3), multiplicity * (eq3_e187_d_n4), multiplicity * (eq3_e187_d_n5), multiplicity * (eq3_e187_d_n6), multiplicity * (eq3_e187_d_n7), multiplicity * (eq3_e187_d_n8), multiplicity * (eq3_e187_d_n9), multiplicity * (eq3_e187_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq4_e196, eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10,) = {
    if (locals.var_guard117 != 0.0) {
        let eq4_e191: f64 = (-locals.var_iztcb);
        let eq4_e192: f64 = (p.p3 * eq4_e191);
        let eq4_e192_d_n0: f64 = (p.p3 * (-locals.var_iztcb_dn0));
        let eq4_e192_d_n1: f64 = (p.p3 * (-locals.var_iztcb_dn1));
        let eq4_e192_d_n3: f64 = (p.p3 * (-locals.var_iztcb_dn3));
        let eq4_e192_d_n4: f64 = (p.p3 * (-locals.var_iztcb_dn4));
        let eq4_e192_d_n5: f64 = (p.p3 * (-locals.var_iztcb_dn5));
        let eq4_e192_d_n6: f64 = (p.p3 * (-locals.var_iztcb_dn6));
        let eq4_e192_d_n7: f64 = (p.p3 * (-locals.var_iztcb_dn7));
        let eq4_e192_d_n8: f64 = (p.p3 * (-locals.var_iztcb_dn8));
        let eq4_e192_d_n9: f64 = (p.p3 * (-locals.var_iztcb_dn9));
        let eq4_e192_d_n10: f64 = (p.p3 * (-locals.var_iztcb_dn10));
        let eq4_e194: f64 = (eq4_e192 * p.p1);
        let eq4_e194_d_n0: f64 = (eq4_e192_d_n0 * p.p1);
        let eq4_e194_d_n1: f64 = (eq4_e192_d_n1 * p.p1);
        let eq4_e194_d_n3: f64 = (eq4_e192_d_n3 * p.p1);
        let eq4_e194_d_n4: f64 = (eq4_e192_d_n4 * p.p1);
        let eq4_e194_d_n5: f64 = (eq4_e192_d_n5 * p.p1);
        let eq4_e194_d_n6: f64 = (eq4_e192_d_n6 * p.p1);
        let eq4_e194_d_n7: f64 = (eq4_e192_d_n7 * p.p1);
        let eq4_e194_d_n8: f64 = (eq4_e192_d_n8 * p.p1);
        let eq4_e194_d_n9: f64 = (eq4_e192_d_n9 * p.p1);
        let eq4_e194_d_n10: f64 = (eq4_e192_d_n10 * p.p1);
        (eq4_e194, eq4_e194_d_n0, eq4_e194_d_n1, eq4_e194_d_n3, eq4_e194_d_n4, eq4_e194_d_n5, eq4_e194_d_n6, eq4_e194_d_n7, eq4_e194_d_n8, eq4_e194_d_n9, eq4_e194_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e196;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq4_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq4_e196_d_n0), multiplicity * (eq4_e196_d_n1), multiplicity * (eq4_e196_d_n3), multiplicity * (eq4_e196_d_n4), multiplicity * (eq4_e196_d_n5), multiplicity * (eq4_e196_d_n6), multiplicity * (eq4_e196_d_n7), multiplicity * (eq4_e196_d_n8), multiplicity * (eq4_e196_d_n9), multiplicity * (eq4_e196_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq5_e206, eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10,) = {
    if (locals.var_guard117 == 0.0) {
        let eq5_e201: f64 = (-locals.var_iztcb);
        let eq5_e202: f64 = (p.p3 * eq5_e201);
        let eq5_e202_d_n0: f64 = (p.p3 * (-locals.var_iztcb_dn0));
        let eq5_e202_d_n1: f64 = (p.p3 * (-locals.var_iztcb_dn1));
        let eq5_e202_d_n3: f64 = (p.p3 * (-locals.var_iztcb_dn3));
        let eq5_e202_d_n4: f64 = (p.p3 * (-locals.var_iztcb_dn4));
        let eq5_e202_d_n5: f64 = (p.p3 * (-locals.var_iztcb_dn5));
        let eq5_e202_d_n6: f64 = (p.p3 * (-locals.var_iztcb_dn6));
        let eq5_e202_d_n7: f64 = (p.p3 * (-locals.var_iztcb_dn7));
        let eq5_e202_d_n8: f64 = (p.p3 * (-locals.var_iztcb_dn8));
        let eq5_e202_d_n9: f64 = (p.p3 * (-locals.var_iztcb_dn9));
        let eq5_e202_d_n10: f64 = (p.p3 * (-locals.var_iztcb_dn10));
        let eq5_e204: f64 = (eq5_e202 * p.p1);
        let eq5_e204_d_n0: f64 = (eq5_e202_d_n0 * p.p1);
        let eq5_e204_d_n1: f64 = (eq5_e202_d_n1 * p.p1);
        let eq5_e204_d_n3: f64 = (eq5_e202_d_n3 * p.p1);
        let eq5_e204_d_n4: f64 = (eq5_e202_d_n4 * p.p1);
        let eq5_e204_d_n5: f64 = (eq5_e202_d_n5 * p.p1);
        let eq5_e204_d_n6: f64 = (eq5_e202_d_n6 * p.p1);
        let eq5_e204_d_n7: f64 = (eq5_e202_d_n7 * p.p1);
        let eq5_e204_d_n8: f64 = (eq5_e202_d_n8 * p.p1);
        let eq5_e204_d_n9: f64 = (eq5_e202_d_n9 * p.p1);
        let eq5_e204_d_n10: f64 = (eq5_e202_d_n10 * p.p1);
        (eq5_e204, eq5_e204_d_n0, eq5_e204_d_n1, eq5_e204_d_n3, eq5_e204_d_n4, eq5_e204_d_n5, eq5_e204_d_n6, eq5_e204_d_n7, eq5_e204_d_n8, eq5_e204_d_n9, eq5_e204_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e206;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq5_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq5_e206_d_n0), multiplicity * (eq5_e206_d_n1), multiplicity * (eq5_e206_d_n3), multiplicity * (eq5_e206_d_n4), multiplicity * (eq5_e206_d_n5), multiplicity * (eq5_e206_d_n6), multiplicity * (eq5_e206_d_n7), multiplicity * (eq5_e206_d_n8), multiplicity * (eq5_e206_d_n9), multiplicity * (eq5_e206_d_n10)],
            [],
            [],
            1.0,
        );
        let eq6_e209: f64 = (p.p3 * locals.var_ib1b2);
        let eq6_e209_d_n0: f64 = (p.p3 * locals.var_ib1b2_dn0);
        let eq6_e209_d_n1: f64 = (p.p3 * locals.var_ib1b2_dn1);
        let eq6_e209_d_n3: f64 = (p.p3 * locals.var_ib1b2_dn3);
        let eq6_e209_d_n4: f64 = (p.p3 * locals.var_ib1b2_dn4);
        let eq6_e209_d_n5: f64 = (p.p3 * locals.var_ib1b2_dn5);
        let eq6_e209_d_n6: f64 = (p.p3 * locals.var_ib1b2_dn6);
        let eq6_e209_d_n7: f64 = (p.p3 * locals.var_ib1b2_dn7);
        let eq6_e209_d_n8: f64 = (p.p3 * locals.var_ib1b2_dn8);
        let eq6_e209_d_n9: f64 = (p.p3 * locals.var_ib1b2_dn9);
        let eq6_e209_d_n10: f64 = (p.p3 * locals.var_ib1b2_dn10);
        let eq6_e211: f64 = (eq6_e209 * p.p1);
        let eq6_e211_d_n0: f64 = (eq6_e209_d_n0 * p.p1);
        let eq6_e211_d_n1: f64 = (eq6_e209_d_n1 * p.p1);
        let eq6_e211_d_n3: f64 = (eq6_e209_d_n3 * p.p1);
        let eq6_e211_d_n4: f64 = (eq6_e209_d_n4 * p.p1);
        let eq6_e211_d_n5: f64 = (eq6_e209_d_n5 * p.p1);
        let eq6_e211_d_n6: f64 = (eq6_e209_d_n6 * p.p1);
        let eq6_e211_d_n7: f64 = (eq6_e209_d_n7 * p.p1);
        let eq6_e211_d_n8: f64 = (eq6_e209_d_n8 * p.p1);
        let eq6_e211_d_n9: f64 = (eq6_e209_d_n9 * p.p1);
        let eq6_e211_d_n10: f64 = (eq6_e209_d_n10 * p.p1);
        let eq6_value: f64 = eq6_e211;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq6_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq6_e211_d_n0), multiplicity * (eq6_e211_d_n1), multiplicity * (eq6_e211_d_n3), multiplicity * (eq6_e211_d_n4), multiplicity * (eq6_e211_d_n5), multiplicity * (eq6_e211_d_n6), multiplicity * (eq6_e211_d_n7), multiplicity * (eq6_e211_d_n8), multiplicity * (eq6_e211_d_n9), multiplicity * (eq6_e211_d_n10)],
            [],
            [],
            1.0,
        );
        let eq7_e214: f64 = (-1.0);
        let eq7_e216: f64 = (eq7_e214 * locals.var_iavl);
        let eq7_e216_d_n0: f64 = (eq7_e214 * locals.var_iavl_dn0);
        let eq7_e216_d_n1: f64 = (eq7_e214 * locals.var_iavl_dn1);
        let eq7_e216_d_n3: f64 = (eq7_e214 * locals.var_iavl_dn3);
        let eq7_e216_d_n4: f64 = (eq7_e214 * locals.var_iavl_dn4);
        let eq7_e216_d_n5: f64 = (eq7_e214 * locals.var_iavl_dn5);
        let eq7_e216_d_n6: f64 = (eq7_e214 * locals.var_iavl_dn6);
        let eq7_e216_d_n7: f64 = (eq7_e214 * locals.var_iavl_dn7);
        let eq7_e216_d_n8: f64 = (eq7_e214 * locals.var_iavl_dn8);
        let eq7_e216_d_n9: f64 = (eq7_e214 * locals.var_iavl_dn9);
        let eq7_e216_d_n10: f64 = (eq7_e214 * locals.var_iavl_dn10);
        let eq7_e217: f64 = (p.p3 * eq7_e216);
        let eq7_e217_d_n0: f64 = (p.p3 * eq7_e216_d_n0);
        let eq7_e217_d_n1: f64 = (p.p3 * eq7_e216_d_n1);
        let eq7_e217_d_n3: f64 = (p.p3 * eq7_e216_d_n3);
        let eq7_e217_d_n4: f64 = (p.p3 * eq7_e216_d_n4);
        let eq7_e217_d_n5: f64 = (p.p3 * eq7_e216_d_n5);
        let eq7_e217_d_n6: f64 = (p.p3 * eq7_e216_d_n6);
        let eq7_e217_d_n7: f64 = (p.p3 * eq7_e216_d_n7);
        let eq7_e217_d_n8: f64 = (p.p3 * eq7_e216_d_n8);
        let eq7_e217_d_n9: f64 = (p.p3 * eq7_e216_d_n9);
        let eq7_e217_d_n10: f64 = (p.p3 * eq7_e216_d_n10);
        let eq7_e219: f64 = (eq7_e217 * p.p1);
        let eq7_e219_d_n0: f64 = (eq7_e217_d_n0 * p.p1);
        let eq7_e219_d_n1: f64 = (eq7_e217_d_n1 * p.p1);
        let eq7_e219_d_n3: f64 = (eq7_e217_d_n3 * p.p1);
        let eq7_e219_d_n4: f64 = (eq7_e217_d_n4 * p.p1);
        let eq7_e219_d_n5: f64 = (eq7_e217_d_n5 * p.p1);
        let eq7_e219_d_n6: f64 = (eq7_e217_d_n6 * p.p1);
        let eq7_e219_d_n7: f64 = (eq7_e217_d_n7 * p.p1);
        let eq7_e219_d_n8: f64 = (eq7_e217_d_n8 * p.p1);
        let eq7_e219_d_n9: f64 = (eq7_e217_d_n9 * p.p1);
        let eq7_e219_d_n10: f64 = (eq7_e217_d_n10 * p.p1);
        let eq7_value: f64 = eq7_e219;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq7_e219_d_n0), multiplicity * (eq7_e219_d_n1), multiplicity * (eq7_e219_d_n3), multiplicity * (eq7_e219_d_n4), multiplicity * (eq7_e219_d_n5), multiplicity * (eq7_e219_d_n6), multiplicity * (eq7_e219_d_n7), multiplicity * (eq7_e219_d_n8), multiplicity * (eq7_e219_d_n9), multiplicity * (eq7_e219_d_n10)],
            [],
            [],
            1.0,
        );
        let eq8_e222: f64 = (p.p3 * locals.var_vee1);
        let eq8_e222_d_n2: f64 = (p.p3 * locals.var_vee1_dn2);
        let eq8_e222_d_n4: f64 = (p.p3 * locals.var_vee1_dn4);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_re_t;
        let eq8_e224: f64 = (eq8_e222 * __rspice_inv_cse_0);
        let eq8_e224_d_n2: f64 = (eq8_e222_d_n2 * __rspice_inv_cse_0);
        let eq8_e224_d_n3: f64 = (-((eq8_e222 * locals.var_re_t_dn3) / (locals.var_re_t * locals.var_re_t)));
        let eq8_e224_d_n4: f64 = (eq8_e222_d_n4 / locals.var_re_t);
        let eq8_e226: f64 = (eq8_e224 * p.p1);
        let eq8_e226_d_n2: f64 = (eq8_e224_d_n2 * p.p1);
        let eq8_e226_d_n3: f64 = (eq8_e224_d_n3 * p.p1);
        let eq8_e226_d_n4: f64 = (eq8_e224_d_n4 * p.p1);
        let eq8_value: f64 = eq8_e226;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (eq8_value),
            2,
            multiplicity * (eq8_e226_d_n2),
            3,
            multiplicity * (eq8_e226_d_n3),
            4,
            multiplicity * (eq8_e226_d_n4),
        );
        let eq9_e229: f64 = (p.p3 * locals.var_vbb1);
        let eq9_e229_d_n1: f64 = (p.p3 * locals.var_vbb1_dn1);
        let eq9_e229_d_n5: f64 = (p.p3 * locals.var_vbb1_dn5);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_rbc_t;
        let eq9_e231: f64 = (eq9_e229 * __rspice_inv_cse_1);
        let eq9_e231_d_n1: f64 = (eq9_e229_d_n1 * __rspice_inv_cse_1);
        let eq9_e231_d_n3: f64 = (-((eq9_e229 * locals.var_rbc_t_dn3) / (locals.var_rbc_t * locals.var_rbc_t)));
        let eq9_e231_d_n5: f64 = (eq9_e229_d_n5 / locals.var_rbc_t);
        let eq9_e233: f64 = (eq9_e231 * p.p1);
        let eq9_e233_d_n1: f64 = (eq9_e231_d_n1 * p.p1);
        let eq9_e233_d_n3: f64 = (eq9_e231_d_n3 * p.p1);
        let eq9_e233_d_n5: f64 = (eq9_e231_d_n5 * p.p1);
        let eq9_value: f64 = eq9_e233;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (eq9_value),
            1,
            multiplicity * (eq9_e233_d_n1),
            3,
            multiplicity * (eq9_e233_d_n3),
            5,
            multiplicity * (eq9_e233_d_n5),
        );
        let eq10_value: f64 = locals.var_p_rth;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq10_value),
            3,
            multiplicity * (locals.var_p_rth_dn3),
        );
        let eq11_value: f64 = locals.var_i_cth;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            3,
            multiplicity * (locals.var_i_cth_dn3),
        );
        let eq12_e237: f64 = (-1.0);
        let eq12_e239: f64 = (eq12_e237 * locals.var_power);
        let eq12_e239_d_n0: f64 = (eq12_e237 * locals.var_power_dn0);
        let eq12_e239_d_n1: f64 = (eq12_e237 * locals.var_power_dn1);
        let eq12_e239_d_n2: f64 = (eq12_e237 * locals.var_power_dn2);
        let eq12_e239_d_n3: f64 = (eq12_e237 * locals.var_power_dn3);
        let eq12_e239_d_n4: f64 = (eq12_e237 * locals.var_power_dn4);
        let eq12_e239_d_n5: f64 = (eq12_e237 * locals.var_power_dn5);
        let eq12_e239_d_n6: f64 = (eq12_e237 * locals.var_power_dn6);
        let eq12_e239_d_n7: f64 = (eq12_e237 * locals.var_power_dn7);
        let eq12_e239_d_n8: f64 = (eq12_e237 * locals.var_power_dn8);
        let eq12_e239_d_n9: f64 = (eq12_e237 * locals.var_power_dn9);
        let eq12_e239_d_n10: f64 = (eq12_e237 * locals.var_power_dn10);
        let eq12_e241: f64 = (eq12_e239 * p.p1);
        let eq12_e241_d_n0: f64 = (eq12_e239_d_n0 * p.p1);
        let eq12_e241_d_n1: f64 = (eq12_e239_d_n1 * p.p1);
        let eq12_e241_d_n2: f64 = (eq12_e239_d_n2 * p.p1);
        let eq12_e241_d_n3: f64 = (eq12_e239_d_n3 * p.p1);
        let eq12_e241_d_n4: f64 = (eq12_e239_d_n4 * p.p1);
        let eq12_e241_d_n5: f64 = (eq12_e239_d_n5 * p.p1);
        let eq12_e241_d_n6: f64 = (eq12_e239_d_n6 * p.p1);
        let eq12_e241_d_n7: f64 = (eq12_e239_d_n7 * p.p1);
        let eq12_e241_d_n8: f64 = (eq12_e239_d_n8 * p.p1);
        let eq12_e241_d_n9: f64 = (eq12_e239_d_n9 * p.p1);
        let eq12_e241_d_n10: f64 = (eq12_e239_d_n10 * p.p1);
        let eq12_value: f64 = eq12_e241;
        let eq12_node_derivative_indices: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let eq12_node_derivatives: [f64; 11] = [eq12_e241_d_n0, eq12_e241_d_n1, eq12_e241_d_n2, eq12_e241_d_n3, eq12_e241_d_n4, eq12_e241_d_n5, eq12_e241_d_n6, eq12_e241_d_n7, eq12_e241_d_n8, eq12_e241_d_n9, eq12_e241_d_n10];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
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
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let eq13_e245: f64 = (locals.var_qte + locals.var_qbe);
        let eq13_e245_d_n0: f64 = (locals.var_qte_dn0 + locals.var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (locals.var_qte_dn1 + locals.var_qbe_dn1);
        let eq13_e245_d_n3: f64 = (locals.var_qte_dn3 + locals.var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (locals.var_qte_dn4 + locals.var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (locals.var_qte_dn5 + locals.var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (locals.var_qte_dn6 + locals.var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (locals.var_qte_dn7 + locals.var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (locals.var_qte_dn8 + locals.var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (locals.var_qte_dn9 + locals.var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (locals.var_qte_dn10 + locals.var_qbe_dn10);
        let eq13_e247: f64 = (eq13_e245 + locals.var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + locals.var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + locals.var_qe_dn1);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + locals.var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + locals.var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + locals.var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + locals.var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + locals.var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + locals.var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + locals.var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + locals.var_qe_dn10);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e249: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq13_e248);
        let eq13_e251: f64 = (eq13_e249 * p.p1);
        let eq13_e251_d_n0: f64 = ((eq13_e248_d_n0 * ddt_scale) * p.p1);
        let eq13_e251_d_n1: f64 = ((eq13_e248_d_n1 * ddt_scale) * p.p1);
        let eq13_e251_d_n3: f64 = ((eq13_e248_d_n3 * ddt_scale) * p.p1);
        let eq13_e251_d_n4: f64 = ((eq13_e248_d_n4 * ddt_scale) * p.p1);
        let eq13_e251_d_n5: f64 = ((eq13_e248_d_n5 * ddt_scale) * p.p1);
        let eq13_e251_d_n6: f64 = ((eq13_e248_d_n6 * ddt_scale) * p.p1);
        let eq13_e251_d_n7: f64 = ((eq13_e248_d_n7 * ddt_scale) * p.p1);
        let eq13_e251_d_n8: f64 = ((eq13_e248_d_n8 * ddt_scale) * p.p1);
        let eq13_e251_d_n9: f64 = ((eq13_e248_d_n9 * ddt_scale) * p.p1);
        let eq13_e251_d_n10: f64 = ((eq13_e248_d_n10 * ddt_scale) * p.p1);
        let eq13_value: f64 = eq13_e251;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(4),
            multiplicity * (eq13_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq13_e251_d_n0), multiplicity * (eq13_e251_d_n1), multiplicity * (eq13_e251_d_n3), multiplicity * (eq13_e251_d_n4), multiplicity * (eq13_e251_d_n5), multiplicity * (eq13_e251_d_n6), multiplicity * (eq13_e251_d_n7), multiplicity * (eq13_e251_d_n8), multiplicity * (eq13_e251_d_n9), multiplicity * (eq13_e251_d_n10)],
            [],
            [],
            1.0,
        );
        let eq14_e254: f64 = (p.p3 * locals.var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * locals.var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * locals.var_qte_s_dn1);
        let eq14_e254_d_n3: f64 = (p.p3 * locals.var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * locals.var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * locals.var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * locals.var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * locals.var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * locals.var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * locals.var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * locals.var_qte_s_dn10);
        let eq14_e255: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq14_e254);
        let eq14_e257: f64 = (eq14_e255 * p.p1);
        let eq14_e257_d_n0: f64 = ((eq14_e254_d_n0 * ddt_scale) * p.p1);
        let eq14_e257_d_n1: f64 = ((eq14_e254_d_n1 * ddt_scale) * p.p1);
        let eq14_e257_d_n3: f64 = ((eq14_e254_d_n3 * ddt_scale) * p.p1);
        let eq14_e257_d_n4: f64 = ((eq14_e254_d_n4 * ddt_scale) * p.p1);
        let eq14_e257_d_n5: f64 = ((eq14_e254_d_n5 * ddt_scale) * p.p1);
        let eq14_e257_d_n6: f64 = ((eq14_e254_d_n6 * ddt_scale) * p.p1);
        let eq14_e257_d_n7: f64 = ((eq14_e254_d_n7 * ddt_scale) * p.p1);
        let eq14_e257_d_n8: f64 = ((eq14_e254_d_n8 * ddt_scale) * p.p1);
        let eq14_e257_d_n9: f64 = ((eq14_e254_d_n9 * ddt_scale) * p.p1);
        let eq14_e257_d_n10: f64 = ((eq14_e254_d_n10 * ddt_scale) * p.p1);
        let eq14_value: f64 = eq14_e257;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq14_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq14_e257_d_n0), multiplicity * (eq14_e257_d_n1), multiplicity * (eq14_e257_d_n3), multiplicity * (eq14_e257_d_n4), multiplicity * (eq14_e257_d_n5), multiplicity * (eq14_e257_d_n6), multiplicity * (eq14_e257_d_n7), multiplicity * (eq14_e257_d_n8), multiplicity * (eq14_e257_d_n9), multiplicity * (eq14_e257_d_n10)],
            [],
            [],
            1.0,
        );
        let eq15_e261: f64 = (locals.var_qtc + locals.var_qbc);
        let eq15_e261_d_n0: f64 = (locals.var_qtc_dn0 + locals.var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (locals.var_qtc_dn1 + locals.var_qbc_dn1);
        let eq15_e261_d_n3: f64 = (locals.var_qtc_dn3 + locals.var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (locals.var_qtc_dn4 + locals.var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (locals.var_qtc_dn5 + locals.var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (locals.var_qtc_dn6 + locals.var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (locals.var_qtc_dn7 + locals.var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (locals.var_qtc_dn8 + locals.var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (locals.var_qtc_dn9 + locals.var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (locals.var_qtc_dn10 + locals.var_qbc_dn10);
        let eq15_e263: f64 = (eq15_e261 + locals.var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + locals.var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + locals.var_qepi_dn1);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + locals.var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + locals.var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + locals.var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + locals.var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + locals.var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + locals.var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + locals.var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + locals.var_qepi_dn10);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e265: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq15_e264);
        let eq15_e267: f64 = (eq15_e265 * p.p1);
        let eq15_e267_d_n0: f64 = ((eq15_e264_d_n0 * ddt_scale) * p.p1);
        let eq15_e267_d_n1: f64 = ((eq15_e264_d_n1 * ddt_scale) * p.p1);
        let eq15_e267_d_n3: f64 = ((eq15_e264_d_n3 * ddt_scale) * p.p1);
        let eq15_e267_d_n4: f64 = ((eq15_e264_d_n4 * ddt_scale) * p.p1);
        let eq15_e267_d_n5: f64 = ((eq15_e264_d_n5 * ddt_scale) * p.p1);
        let eq15_e267_d_n6: f64 = ((eq15_e264_d_n6 * ddt_scale) * p.p1);
        let eq15_e267_d_n7: f64 = ((eq15_e264_d_n7 * ddt_scale) * p.p1);
        let eq15_e267_d_n8: f64 = ((eq15_e264_d_n8 * ddt_scale) * p.p1);
        let eq15_e267_d_n9: f64 = ((eq15_e264_d_n9 * ddt_scale) * p.p1);
        let eq15_e267_d_n10: f64 = ((eq15_e264_d_n10 * ddt_scale) * p.p1);
        let eq15_value: f64 = eq15_e267;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq15_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq15_e267_d_n0), multiplicity * (eq15_e267_d_n1), multiplicity * (eq15_e267_d_n3), multiplicity * (eq15_e267_d_n4), multiplicity * (eq15_e267_d_n5), multiplicity * (eq15_e267_d_n6), multiplicity * (eq15_e267_d_n7), multiplicity * (eq15_e267_d_n8), multiplicity * (eq15_e267_d_n9), multiplicity * (eq15_e267_d_n10)],
            [],
            [],
            1.0,
        );
        let eq16_e270: f64 = (p.p3 * locals.var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * locals.var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * locals.var_qb1b2_dn1);
        let eq16_e270_d_n3: f64 = (p.p3 * locals.var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * locals.var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * locals.var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * locals.var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * locals.var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * locals.var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * locals.var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * locals.var_qb1b2_dn10);
        let eq16_e271: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq16_e270);
        let eq16_e273: f64 = (eq16_e271 * p.p1);
        let eq16_e273_d_n0: f64 = ((eq16_e270_d_n0 * ddt_scale) * p.p1);
        let eq16_e273_d_n1: f64 = ((eq16_e270_d_n1 * ddt_scale) * p.p1);
        let eq16_e273_d_n3: f64 = ((eq16_e270_d_n3 * ddt_scale) * p.p1);
        let eq16_e273_d_n4: f64 = ((eq16_e270_d_n4 * ddt_scale) * p.p1);
        let eq16_e273_d_n5: f64 = ((eq16_e270_d_n5 * ddt_scale) * p.p1);
        let eq16_e273_d_n6: f64 = ((eq16_e270_d_n6 * ddt_scale) * p.p1);
        let eq16_e273_d_n7: f64 = ((eq16_e270_d_n7 * ddt_scale) * p.p1);
        let eq16_e273_d_n8: f64 = ((eq16_e270_d_n8 * ddt_scale) * p.p1);
        let eq16_e273_d_n9: f64 = ((eq16_e270_d_n9 * ddt_scale) * p.p1);
        let eq16_e273_d_n10: f64 = ((eq16_e270_d_n10 * ddt_scale) * p.p1);
        let eq16_value: f64 = eq16_e273;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq16_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq16_e273_d_n0), multiplicity * (eq16_e273_d_n1), multiplicity * (eq16_e273_d_n3), multiplicity * (eq16_e273_d_n4), multiplicity * (eq16_e273_d_n5), multiplicity * (eq16_e273_d_n6), multiplicity * (eq16_e273_d_n7), multiplicity * (eq16_e273_d_n8), multiplicity * (eq16_e273_d_n9), multiplicity * (eq16_e273_d_n10)],
            [],
            [],
            1.0,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * locals.var_vbe);
        let eq17_e278_d_n1: f64 = (eq17_e276 * locals.var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * locals.var_vbe_dn2);
        let eq17_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq17_e278);
        let eq17_e281: f64 = (eq17_e279 * p.p1);
        let eq17_e281_d_n1: f64 = ((eq17_e278_d_n1 * ddt_scale) * p.p1);
        let eq17_e281_d_n2: f64 = ((eq17_e278_d_n2 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e281;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq17_value),
            1,
            multiplicity * (eq17_e281_d_n1),
            2,
            multiplicity * (eq17_e281_d_n2),
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * locals.var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * locals.var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * locals.var_vbc_dn1);
        let eq18_e287: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq18_e286);
        let eq18_e289: f64 = (eq18_e287 * p.p1);
        let eq18_e289_d_n0: f64 = ((eq18_e286_d_n0 * ddt_scale) * p.p1);
        let eq18_e289_d_n1: f64 = ((eq18_e286_d_n1 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e289;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (eq18_value),
            0,
            multiplicity * (eq18_e289_d_n0),
            1,
            multiplicity * (eq18_e289_d_n1),
        );
        let eq19_e292: f64 = (p.p3 * locals.var_xiex);
        let eq19_e292_d_n0: f64 = (p.p3 * locals.var_xiex_dn0);
        let eq19_e292_d_n1: f64 = (p.p3 * locals.var_xiex_dn1);
        let eq19_e292_d_n3: f64 = (p.p3 * locals.var_xiex_dn3);
        let eq19_e292_d_n4: f64 = (p.p3 * locals.var_xiex_dn4);
        let eq19_e292_d_n5: f64 = (p.p3 * locals.var_xiex_dn5);
        let eq19_e292_d_n6: f64 = (p.p3 * locals.var_xiex_dn6);
        let eq19_e292_d_n7: f64 = (p.p3 * locals.var_xiex_dn7);
        let eq19_e292_d_n8: f64 = (p.p3 * locals.var_xiex_dn8);
        let eq19_e292_d_n9: f64 = (p.p3 * locals.var_xiex_dn9);
        let eq19_e292_d_n10: f64 = (p.p3 * locals.var_xiex_dn10);
        let eq19_e294: f64 = (eq19_e292 * p.p1);
        let eq19_e294_d_n0: f64 = (eq19_e292_d_n0 * p.p1);
        let eq19_e294_d_n1: f64 = (eq19_e292_d_n1 * p.p1);
        let eq19_e294_d_n3: f64 = (eq19_e292_d_n3 * p.p1);
        let eq19_e294_d_n4: f64 = (eq19_e292_d_n4 * p.p1);
        let eq19_e294_d_n5: f64 = (eq19_e292_d_n5 * p.p1);
        let eq19_e294_d_n6: f64 = (eq19_e292_d_n6 * p.p1);
        let eq19_e294_d_n7: f64 = (eq19_e292_d_n7 * p.p1);
        let eq19_e294_d_n8: f64 = (eq19_e292_d_n8 * p.p1);
        let eq19_e294_d_n9: f64 = (eq19_e292_d_n9 * p.p1);
        let eq19_e294_d_n10: f64 = (eq19_e292_d_n10 * p.p1);
        let eq19_value: f64 = eq19_e294;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq19_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq19_e294_d_n0), multiplicity * (eq19_e294_d_n1), multiplicity * (eq19_e294_d_n3), multiplicity * (eq19_e294_d_n4), multiplicity * (eq19_e294_d_n5), multiplicity * (eq19_e294_d_n6), multiplicity * (eq19_e294_d_n7), multiplicity * (eq19_e294_d_n8), multiplicity * (eq19_e294_d_n9), multiplicity * (eq19_e294_d_n10)],
            [],
            [],
            1.0,
        );
        let eq20_e297: f64 = (p.p3 * locals.var_vcc3);
        let eq20_e297_d_n0: f64 = (p.p3 * locals.var_vcc3_dn0);
        let eq20_e297_d_n1: f64 = (p.p3 * locals.var_vcc3_dn1);
        let eq20_e297_d_n5: f64 = (p.p3 * locals.var_vcc3_dn5);
        let eq20_e297_d_n6: f64 = (p.p3 * locals.var_vcc3_dn6);
        let eq20_e297_d_n7: f64 = (p.p3 * locals.var_vcc3_dn7);
        let eq20_e297_d_n8: f64 = (p.p3 * locals.var_vcc3_dn8);
        let eq20_e297_d_n9: f64 = (p.p3 * locals.var_vcc3_dn9);
        let eq20_e297_d_n10: f64 = (p.p3 * locals.var_vcc3_dn10);
        let eq20_e299: f64 = (eq20_e297 * locals.var_gcc_xx_t);
        let eq20_e299_d_n0: f64 = (eq20_e297_d_n0 * locals.var_gcc_xx_t);
        let eq20_e299_d_n1: f64 = (eq20_e297_d_n1 * locals.var_gcc_xx_t);
        let eq20_e299_d_n3: f64 = (eq20_e297 * locals.var_gcc_xx_t_dn3);
        let eq20_e299_d_n5: f64 = (eq20_e297_d_n5 * locals.var_gcc_xx_t);
        let eq20_e299_d_n6: f64 = (eq20_e297_d_n6 * locals.var_gcc_xx_t);
        let eq20_e299_d_n7: f64 = (eq20_e297_d_n7 * locals.var_gcc_xx_t);
        let eq20_e299_d_n8: f64 = (eq20_e297_d_n8 * locals.var_gcc_xx_t);
        let eq20_e299_d_n9: f64 = (eq20_e297_d_n9 * locals.var_gcc_xx_t);
        let eq20_e299_d_n10: f64 = (eq20_e297_d_n10 * locals.var_gcc_xx_t);
        let eq20_e301: f64 = (eq20_e299 * p.p1);
        let eq20_e301_d_n0: f64 = (eq20_e299_d_n0 * p.p1);
        let eq20_e301_d_n1: f64 = (eq20_e299_d_n1 * p.p1);
        let eq20_e301_d_n3: f64 = (eq20_e299_d_n3 * p.p1);
        let eq20_e301_d_n5: f64 = (eq20_e299_d_n5 * p.p1);
        let eq20_e301_d_n6: f64 = (eq20_e299_d_n6 * p.p1);
        let eq20_e301_d_n7: f64 = (eq20_e299_d_n7 * p.p1);
        let eq20_e301_d_n8: f64 = (eq20_e299_d_n8 * p.p1);
        let eq20_e301_d_n9: f64 = (eq20_e299_d_n9 * p.p1);
        let eq20_e301_d_n10: f64 = (eq20_e299_d_n10 * p.p1);
        let eq20_value: f64 = eq20_e301;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * (eq20_value),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq20_e301_d_n0), multiplicity * (eq20_e301_d_n1), multiplicity * (eq20_e301_d_n3), multiplicity * (eq20_e301_d_n5), multiplicity * (eq20_e301_d_n6), multiplicity * (eq20_e301_d_n7), multiplicity * (eq20_e301_d_n8), multiplicity * (eq20_e301_d_n9), multiplicity * (eq20_e301_d_n10)],
            [],
            [],
            1.0,
        );
        let eq21_e305: f64 = (locals.var_xqtex + locals.var_xqex);
        let eq21_e305_d_n0: f64 = (locals.var_xqtex_dn0 + locals.var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (locals.var_xqtex_dn1 + locals.var_xqex_dn1);
        let eq21_e305_d_n3: f64 = (locals.var_xqtex_dn3 + locals.var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (locals.var_xqtex_dn4 + locals.var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (locals.var_xqtex_dn5 + locals.var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (locals.var_xqtex_dn6 + locals.var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (locals.var_xqtex_dn7 + locals.var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (locals.var_xqtex_dn8 + locals.var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (locals.var_xqtex_dn9 + locals.var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (locals.var_xqtex_dn10 + locals.var_xqex_dn10);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq21_e306);
        let eq21_e309: f64 = (eq21_e307 * p.p1);
        let eq21_e309_d_n0: f64 = ((eq21_e306_d_n0 * ddt_scale) * p.p1);
        let eq21_e309_d_n1: f64 = ((eq21_e306_d_n1 * ddt_scale) * p.p1);
        let eq21_e309_d_n3: f64 = ((eq21_e306_d_n3 * ddt_scale) * p.p1);
        let eq21_e309_d_n4: f64 = ((eq21_e306_d_n4 * ddt_scale) * p.p1);
        let eq21_e309_d_n5: f64 = ((eq21_e306_d_n5 * ddt_scale) * p.p1);
        let eq21_e309_d_n6: f64 = ((eq21_e306_d_n6 * ddt_scale) * p.p1);
        let eq21_e309_d_n7: f64 = ((eq21_e306_d_n7 * ddt_scale) * p.p1);
        let eq21_e309_d_n8: f64 = ((eq21_e306_d_n8 * ddt_scale) * p.p1);
        let eq21_e309_d_n9: f64 = ((eq21_e306_d_n9 * ddt_scale) * p.p1);
        let eq21_e309_d_n10: f64 = ((eq21_e306_d_n10 * ddt_scale) * p.p1);
        let eq21_value: f64 = eq21_e309;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq21_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq21_e309_d_n0), multiplicity * (eq21_e309_d_n1), multiplicity * (eq21_e309_d_n3), multiplicity * (eq21_e309_d_n4), multiplicity * (eq21_e309_d_n5), multiplicity * (eq21_e309_d_n6), multiplicity * (eq21_e309_d_n7), multiplicity * (eq21_e309_d_n8), multiplicity * (eq21_e309_d_n9), multiplicity * (eq21_e309_d_n10)],
            [],
            [],
            1.0,
        );
        let eq22_e314: f64 = (locals.var_gmin * locals.var_vb1c4);
        let eq22_e314_d_n5: f64 = (locals.var_gmin * locals.var_vb1c4_dn5);
        let eq22_e314_d_n6: f64 = (locals.var_gmin * locals.var_vb1c4_dn6);
        let eq22_e314_d_n7: f64 = (locals.var_gmin * locals.var_vb1c4_dn7);
        let eq22_e314_d_n8: f64 = (locals.var_gmin * locals.var_vb1c4_dn8);
        let eq22_e314_d_n10: f64 = (locals.var_gmin * locals.var_vb1c4_dn10);
        let eq22_e315: f64 = (locals.var_ib3 + eq22_e314);
        let eq22_e315_d_n5: f64 = (locals.var_ib3_dn5 + eq22_e314_d_n5);
        let eq22_e315_d_n6: f64 = (locals.var_ib3_dn6 + eq22_e314_d_n6);
        let eq22_e315_d_n7: f64 = (locals.var_ib3_dn7 + eq22_e314_d_n7);
        let eq22_e315_d_n8: f64 = (locals.var_ib3_dn8 + eq22_e314_d_n8);
        let eq22_e315_d_n10: f64 = (locals.var_ib3_dn10 + eq22_e314_d_n10);
        let eq22_e317: f64 = (eq22_e315 + locals.var_iex);
        let eq22_e317_d_n0: f64 = (locals.var_ib3_dn0 + locals.var_iex_dn0);
        let eq22_e317_d_n1: f64 = (locals.var_ib3_dn1 + locals.var_iex_dn1);
        let eq22_e317_d_n3: f64 = (locals.var_ib3_dn3 + locals.var_iex_dn3);
        let eq22_e317_d_n4: f64 = (locals.var_ib3_dn4 + locals.var_iex_dn4);
        let eq22_e317_d_n5: f64 = (eq22_e315_d_n5 + locals.var_iex_dn5);
        let eq22_e317_d_n6: f64 = (eq22_e315_d_n6 + locals.var_iex_dn6);
        let eq22_e317_d_n7: f64 = (eq22_e315_d_n7 + locals.var_iex_dn7);
        let eq22_e317_d_n8: f64 = (eq22_e315_d_n8 + locals.var_iex_dn8);
        let eq22_e317_d_n9: f64 = (locals.var_ib3_dn9 + locals.var_iex_dn9);
        let eq22_e317_d_n10: f64 = (eq22_e315_d_n10 + locals.var_iex_dn10);
        let eq22_e318: f64 = (p.p3 * eq22_e317);
        let eq22_e318_d_n0: f64 = (p.p3 * eq22_e317_d_n0);
        let eq22_e318_d_n1: f64 = (p.p3 * eq22_e317_d_n1);
        let eq22_e318_d_n3: f64 = (p.p3 * eq22_e317_d_n3);
        let eq22_e318_d_n4: f64 = (p.p3 * eq22_e317_d_n4);
        let eq22_e318_d_n5: f64 = (p.p3 * eq22_e317_d_n5);
        let eq22_e318_d_n6: f64 = (p.p3 * eq22_e317_d_n6);
        let eq22_e318_d_n7: f64 = (p.p3 * eq22_e317_d_n7);
        let eq22_e318_d_n8: f64 = (p.p3 * eq22_e317_d_n8);
        let eq22_e318_d_n9: f64 = (p.p3 * eq22_e317_d_n9);
        let eq22_e318_d_n10: f64 = (p.p3 * eq22_e317_d_n10);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e318_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e318_d_n1 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e318_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e318_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e318_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e318_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e318_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e318_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e318_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e318_d_n10 * p.p1);
        let eq22_value: f64 = eq22_e320;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * (eq22_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq22_e320_d_n0), multiplicity * (eq22_e320_d_n1), multiplicity * (eq22_e320_d_n3), multiplicity * (eq22_e320_d_n4), multiplicity * (eq22_e320_d_n5), multiplicity * (eq22_e320_d_n6), multiplicity * (eq22_e320_d_n7), multiplicity * (eq22_e320_d_n8), multiplicity * (eq22_e320_d_n9), multiplicity * (eq22_e320_d_n10)],
            [],
            [],
            1.0,
        );
        let eq23_e324: f64 = (locals.var_qtex + locals.var_qex);
        let eq23_e324_d_n0: f64 = (locals.var_qtex_dn0 + locals.var_qex_dn0);
        let eq23_e324_d_n1: f64 = (locals.var_qtex_dn1 + locals.var_qex_dn1);
        let eq23_e324_d_n3: f64 = (locals.var_qtex_dn3 + locals.var_qex_dn3);
        let eq23_e324_d_n4: f64 = (locals.var_qtex_dn4 + locals.var_qex_dn4);
        let eq23_e324_d_n5: f64 = (locals.var_qtex_dn5 + locals.var_qex_dn5);
        let eq23_e324_d_n6: f64 = (locals.var_qtex_dn6 + locals.var_qex_dn6);
        let eq23_e324_d_n7: f64 = (locals.var_qtex_dn7 + locals.var_qex_dn7);
        let eq23_e324_d_n8: f64 = (locals.var_qtex_dn8 + locals.var_qex_dn8);
        let eq23_e324_d_n9: f64 = (locals.var_qtex_dn9 + locals.var_qex_dn9);
        let eq23_e324_d_n10: f64 = (locals.var_qtex_dn10 + locals.var_qex_dn10);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq23_e325);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e325_d_n0 * ddt_scale) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e325_d_n1 * ddt_scale) * p.p1);
        let eq23_e328_d_n3: f64 = ((eq23_e325_d_n3 * ddt_scale) * p.p1);
        let eq23_e328_d_n4: f64 = ((eq23_e325_d_n4 * ddt_scale) * p.p1);
        let eq23_e328_d_n5: f64 = ((eq23_e325_d_n5 * ddt_scale) * p.p1);
        let eq23_e328_d_n6: f64 = ((eq23_e325_d_n6 * ddt_scale) * p.p1);
        let eq23_e328_d_n7: f64 = ((eq23_e325_d_n7 * ddt_scale) * p.p1);
        let eq23_e328_d_n8: f64 = ((eq23_e325_d_n8 * ddt_scale) * p.p1);
        let eq23_e328_d_n9: f64 = ((eq23_e325_d_n9 * ddt_scale) * p.p1);
        let eq23_e328_d_n10: f64 = ((eq23_e325_d_n10 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e328;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * (eq23_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq23_e328_d_n0), multiplicity * (eq23_e328_d_n1), multiplicity * (eq23_e328_d_n3), multiplicity * (eq23_e328_d_n4), multiplicity * (eq23_e328_d_n5), multiplicity * (eq23_e328_d_n6), multiplicity * (eq23_e328_d_n7), multiplicity * (eq23_e328_d_n8), multiplicity * (eq23_e328_d_n9), multiplicity * (eq23_e328_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq24_e338, eq24_e338_d_n3, eq24_e338_d_n9, eq24_e338_d_n10,) = {
    if (locals.var_guard121 != 0.0) {
        let eq24_e332: f64 = (p.p3 * locals.var_vc3c4);
        let eq24_e332_d_n9: f64 = (p.p3 * locals.var_vc3c4_dn9);
        let eq24_e332_d_n10: f64 = (p.p3 * locals.var_vc3c4_dn10);
        let eq24_e334: f64 = (eq24_e332 * locals.var_gcc_ex_t);
        let eq24_e334_d_n3: f64 = (eq24_e332 * locals.var_gcc_ex_t_dn3);
        let eq24_e334_d_n9: f64 = (eq24_e332_d_n9 * locals.var_gcc_ex_t);
        let eq24_e334_d_n10: f64 = (eq24_e332_d_n10 * locals.var_gcc_ex_t);
        let eq24_e336: f64 = (eq24_e334 * p.p1);
        let eq24_e336_d_n3: f64 = (eq24_e334_d_n3 * p.p1);
        let eq24_e336_d_n9: f64 = (eq24_e334_d_n9 * p.p1);
        let eq24_e336_d_n10: f64 = (eq24_e334_d_n10 * p.p1);
        (eq24_e336, eq24_e336_d_n3, eq24_e336_d_n9, eq24_e336_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e338;
        stamper.stamp_current_node3_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            3,
            multiplicity * (eq24_e338_d_n3),
            9,
            multiplicity * (eq24_e338_d_n9),
            10,
            multiplicity * (eq24_e338_d_n10),
        );
        let (eq26_e353, eq26_e353_d_n3, eq26_e353_d_n7, eq26_e353_d_n10,) = {
    if (locals.var_guard122 != 0.0) {
        let eq26_e347: f64 = (p.p3 * locals.var_vc4c1);
        let eq26_e347_d_n7: f64 = (p.p3 * locals.var_vc4c1_dn7);
        let eq26_e347_d_n10: f64 = (p.p3 * locals.var_vc4c1_dn10);
        let eq26_e349: f64 = (eq26_e347 * locals.var_gcc_in_t);
        let eq26_e349_d_n3: f64 = (eq26_e347 * locals.var_gcc_in_t_dn3);
        let eq26_e349_d_n7: f64 = (eq26_e347_d_n7 * locals.var_gcc_in_t);
        let eq26_e349_d_n10: f64 = (eq26_e347_d_n10 * locals.var_gcc_in_t);
        let eq26_e351: f64 = (eq26_e349 * p.p1);
        let eq26_e351_d_n3: f64 = (eq26_e349_d_n3 * p.p1);
        let eq26_e351_d_n7: f64 = (eq26_e349_d_n7 * p.p1);
        let eq26_e351_d_n10: f64 = (eq26_e349_d_n10 * p.p1);
        (eq26_e351, eq26_e351_d_n3, eq26_e351_d_n7, eq26_e351_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e353;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(7),
            multiplicity * (eq26_value),
            3,
            multiplicity * (eq26_e353_d_n3),
            7,
            multiplicity * (eq26_e353_d_n7),
            10,
            multiplicity * (eq26_e353_d_n10),
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq30_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (nv11 - 0.0));
        let eq30_e368: f64 = (locals.var_taun * eq30_e367);
        let eq30_e368_d_n0: f64 = (locals.var_taun_dn0 * eq30_e367);
        let eq30_e368_d_n1: f64 = (locals.var_taun_dn1 * eq30_e367);
        let eq30_e368_d_n3: f64 = (locals.var_taun_dn3 * eq30_e367);
        let eq30_e368_d_n4: f64 = (locals.var_taun_dn4 * eq30_e367);
        let eq30_e368_d_n5: f64 = (locals.var_taun_dn5 * eq30_e367);
        let eq30_e368_d_n6: f64 = (locals.var_taun_dn6 * eq30_e367);
        let eq30_e368_d_n7: f64 = (locals.var_taun_dn7 * eq30_e367);
        let eq30_e368_d_n8: f64 = (locals.var_taun_dn8 * eq30_e367);
        let eq30_e368_d_n9: f64 = (locals.var_taun_dn9 * eq30_e367);
        let eq30_e368_d_n10: f64 = (locals.var_taun_dn10 * eq30_e367);
        let eq30_value: f64 = eq30_e368;
        let eq30_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq30_node_derivatives: [f64; 11] = [eq30_e368_d_n0, eq30_e368_d_n1, eq30_e368_d_n3, eq30_e368_d_n4, eq30_e368_d_n5, eq30_e368_d_n6, eq30_e368_d_n7, eq30_e368_d_n8, eq30_e368_d_n9, eq30_e368_d_n10, (locals.var_taun * ddt_scale)];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e371: f64 = (locals.var_gem_n * (nv11 - 0.0));
        let eq31_e371_d_n0: f64 = (locals.var_gem_n_dn0 * (nv11 - 0.0));
        let eq31_e371_d_n1: f64 = (locals.var_gem_n_dn1 * (nv11 - 0.0));
        let eq31_e371_d_n3: f64 = (locals.var_gem_n_dn3 * (nv11 - 0.0));
        let eq31_e371_d_n4: f64 = (locals.var_gem_n_dn4 * (nv11 - 0.0));
        let eq31_e371_d_n5: f64 = (locals.var_gem_n_dn5 * (nv11 - 0.0));
        let eq31_e371_d_n6: f64 = (locals.var_gem_n_dn6 * (nv11 - 0.0));
        let eq31_e371_d_n7: f64 = (locals.var_gem_n_dn7 * (nv11 - 0.0));
        let eq31_e371_d_n8: f64 = (locals.var_gem_n_dn8 * (nv11 - 0.0));
        let eq31_e371_d_n9: f64 = (locals.var_gem_n_dn9 * (nv11 - 0.0));
        let eq31_e371_d_n10: f64 = (locals.var_gem_n_dn10 * (nv11 - 0.0));
        let eq31_value: f64 = eq31_e371;
        let eq31_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq31_node_derivatives: [f64; 11] = [eq31_e371_d_n0, eq31_e371_d_n1, eq31_e371_d_n3, eq31_e371_d_n4, eq31_e371_d_n5, eq31_e371_d_n6, eq31_e371_d_n7, eq31_e371_d_n8, eq31_e371_d_n9, eq31_e371_d_n10, locals.var_gem_n];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq11_e235_q: f64 = locals.var_i_cth_rv;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (locals.var_i_cth_rdn3),
        );
        let eq13_e245: f64 = (locals.var_qte + locals.var_qbe);
        let eq13_e245_d_n0: f64 = (locals.var_qte_dn0 + locals.var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (locals.var_qte_dn1 + locals.var_qbe_dn1);
        let eq13_e245_d_n3: f64 = (locals.var_qte_dn3 + locals.var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (locals.var_qte_dn4 + locals.var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (locals.var_qte_dn5 + locals.var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (locals.var_qte_dn6 + locals.var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (locals.var_qte_dn7 + locals.var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (locals.var_qte_dn8 + locals.var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (locals.var_qte_dn9 + locals.var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (locals.var_qte_dn10 + locals.var_qbe_dn10);
        let eq13_e247: f64 = (eq13_e245 + locals.var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + locals.var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + locals.var_qe_dn1);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + locals.var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + locals.var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + locals.var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + locals.var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + locals.var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + locals.var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + locals.var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + locals.var_qe_dn10);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e249_q: f64 = eq13_e248;
        let eq13_e251: f64 = (eq13_e248 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_q: f64 = (eq13_e249_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, 0.0, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, 0.0];
        let eq13_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * locals.var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * locals.var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * locals.var_qte_s_dn1);
        let eq14_e254_d_n3: f64 = (p.p3 * locals.var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * locals.var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * locals.var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * locals.var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * locals.var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * locals.var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * locals.var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * locals.var_qte_s_dn10);
        let eq14_e255_q: f64 = eq14_e254;
        let eq14_e257: f64 = (eq14_e254 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_q: f64 = (eq14_e255_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, 0.0, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, 0.0];
        let eq14_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (locals.var_qtc + locals.var_qbc);
        let eq15_e261_d_n0: f64 = (locals.var_qtc_dn0 + locals.var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (locals.var_qtc_dn1 + locals.var_qbc_dn1);
        let eq15_e261_d_n3: f64 = (locals.var_qtc_dn3 + locals.var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (locals.var_qtc_dn4 + locals.var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (locals.var_qtc_dn5 + locals.var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (locals.var_qtc_dn6 + locals.var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (locals.var_qtc_dn7 + locals.var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (locals.var_qtc_dn8 + locals.var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (locals.var_qtc_dn9 + locals.var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (locals.var_qtc_dn10 + locals.var_qbc_dn10);
        let eq15_e263: f64 = (eq15_e261 + locals.var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + locals.var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + locals.var_qepi_dn1);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + locals.var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + locals.var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + locals.var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + locals.var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + locals.var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + locals.var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + locals.var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + locals.var_qepi_dn10);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e265_q: f64 = eq15_e264;
        let eq15_e267: f64 = (eq15_e264 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_q: f64 = (eq15_e265_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, 0.0, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * locals.var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * locals.var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * locals.var_qb1b2_dn1);
        let eq16_e270_d_n3: f64 = (p.p3 * locals.var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * locals.var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * locals.var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * locals.var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * locals.var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * locals.var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * locals.var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * locals.var_qb1b2_dn10);
        let eq16_e271_q: f64 = eq16_e270;
        let eq16_e273: f64 = (eq16_e270 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_q: f64 = (eq16_e271_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, 0.0, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, 0.0];
        let eq16_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * locals.var_vbe);
        let eq17_e278_d_n1: f64 = (eq17_e276 * locals.var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * locals.var_vbe_dn2);
        let eq17_e279_q: f64 = eq17_e278;
        let eq17_e281: f64 = (eq17_e278 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_q: f64 = (eq17_e279_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq17_e281_d_n1),
            nodes[2],
            multiplicity * (eq17_e281_d_n2),
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * locals.var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * locals.var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * locals.var_vbc_dn1);
        let eq18_e287_q: f64 = eq18_e286;
        let eq18_e289: f64 = (eq18_e286 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_q: f64 = (eq18_e287_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq18_e289_d_n0),
            nodes[1],
            multiplicity * (eq18_e289_d_n1),
        );
        let eq21_e305: f64 = (locals.var_xqtex + locals.var_xqex);
        let eq21_e305_d_n0: f64 = (locals.var_xqtex_dn0 + locals.var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (locals.var_xqtex_dn1 + locals.var_xqex_dn1);
        let eq21_e305_d_n3: f64 = (locals.var_xqtex_dn3 + locals.var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (locals.var_xqtex_dn4 + locals.var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (locals.var_xqtex_dn5 + locals.var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (locals.var_xqtex_dn6 + locals.var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (locals.var_xqtex_dn7 + locals.var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (locals.var_xqtex_dn8 + locals.var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (locals.var_xqtex_dn9 + locals.var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (locals.var_xqtex_dn10 + locals.var_xqex_dn10);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e307_q: f64 = eq21_e306;
        let eq21_e309: f64 = (eq21_e306 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_q: f64 = (eq21_e307_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, 0.0, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, 0.0];
        let eq21_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e324: f64 = (locals.var_qtex + locals.var_qex);
        let eq23_e324_d_n0: f64 = (locals.var_qtex_dn0 + locals.var_qex_dn0);
        let eq23_e324_d_n1: f64 = (locals.var_qtex_dn1 + locals.var_qex_dn1);
        let eq23_e324_d_n3: f64 = (locals.var_qtex_dn3 + locals.var_qex_dn3);
        let eq23_e324_d_n4: f64 = (locals.var_qtex_dn4 + locals.var_qex_dn4);
        let eq23_e324_d_n5: f64 = (locals.var_qtex_dn5 + locals.var_qex_dn5);
        let eq23_e324_d_n6: f64 = (locals.var_qtex_dn6 + locals.var_qex_dn6);
        let eq23_e324_d_n7: f64 = (locals.var_qtex_dn7 + locals.var_qex_dn7);
        let eq23_e324_d_n8: f64 = (locals.var_qtex_dn8 + locals.var_qex_dn8);
        let eq23_e324_d_n9: f64 = (locals.var_qtex_dn9 + locals.var_qex_dn9);
        let eq23_e324_d_n10: f64 = (locals.var_qtex_dn10 + locals.var_qex_dn10);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, 0.0, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e367_q: f64 = (nv11 - 0.0);
        let eq30_e368: f64 = (locals.var_taun * (nv11 - 0.0));
        let eq30_e368_d_n0: f64 = (locals.var_taun_dn0 * (nv11 - 0.0));
        let eq30_e368_d_n1: f64 = (locals.var_taun_dn1 * (nv11 - 0.0));
        let eq30_e368_d_n3: f64 = (locals.var_taun_dn3 * (nv11 - 0.0));
        let eq30_e368_d_n4: f64 = (locals.var_taun_dn4 * (nv11 - 0.0));
        let eq30_e368_d_n5: f64 = (locals.var_taun_dn5 * (nv11 - 0.0));
        let eq30_e368_d_n6: f64 = (locals.var_taun_dn6 * (nv11 - 0.0));
        let eq30_e368_d_n7: f64 = (locals.var_taun_dn7 * (nv11 - 0.0));
        let eq30_e368_d_n8: f64 = (locals.var_taun_dn8 * (nv11 - 0.0));
        let eq30_e368_d_n9: f64 = (locals.var_taun_dn9 * (nv11 - 0.0));
        let eq30_e368_d_n10: f64 = (locals.var_taun_dn10 * (nv11 - 0.0));
        let eq30_e368_q: f64 = (locals.var_taun * eq30_e367_q);
        let eq30_e368_q_d_n0: f64 = (locals.var_taun_dn0 * eq30_e367_q);
        let eq30_e368_q_d_n1: f64 = (locals.var_taun_dn1 * eq30_e367_q);
        let eq30_e368_q_d_n3: f64 = (locals.var_taun_dn3 * eq30_e367_q);
        let eq30_e368_q_d_n4: f64 = (locals.var_taun_dn4 * eq30_e367_q);
        let eq30_e368_q_d_n5: f64 = (locals.var_taun_dn5 * eq30_e367_q);
        let eq30_e368_q_d_n6: f64 = (locals.var_taun_dn6 * eq30_e367_q);
        let eq30_e368_q_d_n7: f64 = (locals.var_taun_dn7 * eq30_e367_q);
        let eq30_e368_q_d_n8: f64 = (locals.var_taun_dn8 * eq30_e367_q);
        let eq30_e368_q_d_n9: f64 = (locals.var_taun_dn9 * eq30_e367_q);
        let eq30_e368_q_d_n10: f64 = (locals.var_taun_dn10 * eq30_e367_q);
        let eq30_reactive_node_derivatives: [f64; 12] = [eq30_e368_q_d_n0, eq30_e368_q_d_n1, 0.0, eq30_e368_q_d_n3, eq30_e368_q_d_n4, eq30_e368_q_d_n5, eq30_e368_q_d_n6, eq30_e368_q_d_n7, eq30_e368_q_d_n8, eq30_e368_q_d_n9, eq30_e368_q_d_n10, locals.var_taun];
        let eq30_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
