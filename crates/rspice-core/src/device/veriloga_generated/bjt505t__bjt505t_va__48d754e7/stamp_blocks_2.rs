#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6130_e6368: f64 = (locals.var_vb1e1 - locals.var_vfe);
        let assign6130_e6370: f64 = (assign6130_e6368 / locals.var_a_vde);
        locals.var_dxa = assign6130_e6370;
        locals.var_dxa_dn0 = ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn1 = ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn3 = ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn4 = ((((-locals.var_vfe_dn4) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn5 = ((((locals.var_vb1e1_dn5 - locals.var_vfe_dn5) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn6 = ((((locals.var_vb1e1_dn6 - locals.var_vfe_dn6) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn7 = ((((-locals.var_vfe_dn7) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn8 = ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn9 = ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn10 = ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn11 = ((((-locals.var_vfe_dn11) * locals.var_a_vde) - (assign6130_e6368 * locals.var_a_vde_dn11)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_rv = 0.0;

        let assign6140_e6373: f64 = if locals.var_vb1e1 < locals.var_vfe { 1.0 } else { 0.0 };
        locals.var_guard113 = assign6140_e6373;
        locals.var_guard113_rv = 0.0;

        let (assign6150_e6385, assign6150_e6385_d_n0, assign6150_e6385_d_n1, assign6150_e6385_d_n3, assign6150_e6385_d_n4, assign6150_e6385_d_n5, assign6150_e6385_d_n6, assign6150_e6385_d_n7, assign6150_e6385_d_n8, assign6150_e6385_d_n9, assign6150_e6385_d_n10, assign6150_e6385_d_n11,) = {
    if (locals.var_guard113 != 0.0) {
        let assign6150_e6379: f64 = (locals.var_dxa).exp();
        let assign6150_e6380: f64 = (1.0 + assign6150_e6379);
        let assign6150_e6381: f64 = (assign6150_e6380).ln();
        let assign6150_e6382: f64 = (locals.var_a_vde * assign6150_e6381);
        let assign6150_e6383: f64 = (locals.var_vb1e1 - assign6150_e6382);
        (assign6150_e6383, (-((locals.var_a_vde_dn0 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn0) / assign6150_e6380)))), (-((locals.var_a_vde_dn1 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn1) / assign6150_e6380)))), (-((locals.var_a_vde_dn3 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn3) / assign6150_e6380)))), (-((locals.var_a_vde_dn4 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn4) / assign6150_e6380)))), (locals.var_vb1e1_dn5 - ((locals.var_a_vde_dn5 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn5) / assign6150_e6380)))), (locals.var_vb1e1_dn6 - ((locals.var_a_vde_dn6 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn6) / assign6150_e6380)))), (-((locals.var_a_vde_dn7 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn7) / assign6150_e6380)))), (-((locals.var_a_vde_dn8 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn8) / assign6150_e6380)))), (-((locals.var_a_vde_dn9 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn9) / assign6150_e6380)))), (-((locals.var_a_vde_dn10 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn10) / assign6150_e6380)))), (-((locals.var_a_vde_dn11 * assign6150_e6381) + (locals.var_a_vde * ((assign6150_e6379 * locals.var_dxa_dn11) / assign6150_e6380)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9, locals.var_vje_s_dn10, locals.var_vje_s_dn11,)
    }
};
        locals.var_vje_s = assign6150_e6385;
        locals.var_vje_s_dn0 = assign6150_e6385_d_n0;
        locals.var_vje_s_dn1 = assign6150_e6385_d_n1;
        locals.var_vje_s_dn3 = assign6150_e6385_d_n3;
        locals.var_vje_s_dn4 = assign6150_e6385_d_n4;
        locals.var_vje_s_dn5 = assign6150_e6385_d_n5;
        locals.var_vje_s_dn6 = assign6150_e6385_d_n6;
        locals.var_vje_s_dn7 = assign6150_e6385_d_n7;
        locals.var_vje_s_dn8 = assign6150_e6385_d_n8;
        locals.var_vje_s_dn9 = assign6150_e6385_d_n9;
        locals.var_vje_s_dn10 = assign6150_e6385_d_n10;
        locals.var_vje_s_dn11 = assign6150_e6385_d_n11;
        locals.var_vje_s_rv = 0.0;

        let (assign6160_e6399, assign6160_e6399_d_n0, assign6160_e6399_d_n1, assign6160_e6399_d_n3, assign6160_e6399_d_n4, assign6160_e6399_d_n5, assign6160_e6399_d_n6, assign6160_e6399_d_n7, assign6160_e6399_d_n8, assign6160_e6399_d_n9, assign6160_e6399_d_n10, assign6160_e6399_d_n11,) = {
    if (locals.var_guard113 == 0.0) {
        let assign6160_e6392: f64 = (-locals.var_dxa);
        let assign6160_e6393: f64 = (assign6160_e6392).exp();
        let assign6160_e6394: f64 = (1.0 + assign6160_e6393);
        let assign6160_e6395: f64 = (assign6160_e6394).ln();
        let assign6160_e6396: f64 = (locals.var_a_vde * assign6160_e6395);
        let assign6160_e6397: f64 = (locals.var_vfe - assign6160_e6396);
        (assign6160_e6397, (locals.var_vfe_dn0 - ((locals.var_a_vde_dn0 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn0)) / assign6160_e6394)))), (locals.var_vfe_dn1 - ((locals.var_a_vde_dn1 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn1)) / assign6160_e6394)))), (locals.var_vfe_dn3 - ((locals.var_a_vde_dn3 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn3)) / assign6160_e6394)))), (locals.var_vfe_dn4 - ((locals.var_a_vde_dn4 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn4)) / assign6160_e6394)))), (locals.var_vfe_dn5 - ((locals.var_a_vde_dn5 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn5)) / assign6160_e6394)))), (locals.var_vfe_dn6 - ((locals.var_a_vde_dn6 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn6)) / assign6160_e6394)))), (locals.var_vfe_dn7 - ((locals.var_a_vde_dn7 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn7)) / assign6160_e6394)))), (locals.var_vfe_dn8 - ((locals.var_a_vde_dn8 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn8)) / assign6160_e6394)))), (locals.var_vfe_dn9 - ((locals.var_a_vde_dn9 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn9)) / assign6160_e6394)))), (locals.var_vfe_dn10 - ((locals.var_a_vde_dn10 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn10)) / assign6160_e6394)))), (locals.var_vfe_dn11 - ((locals.var_a_vde_dn11 * assign6160_e6395) + (locals.var_a_vde * ((assign6160_e6393 * (-locals.var_dxa_dn11)) / assign6160_e6394)))),)
    } else {
        (locals.var_vje_s, locals.var_vje_s_dn0, locals.var_vje_s_dn1, locals.var_vje_s_dn3, locals.var_vje_s_dn4, locals.var_vje_s_dn5, locals.var_vje_s_dn6, locals.var_vje_s_dn7, locals.var_vje_s_dn8, locals.var_vje_s_dn9, locals.var_vje_s_dn10, locals.var_vje_s_dn11,)
    }
};
        locals.var_vje_s = assign6160_e6399;
        locals.var_vje_s_dn0 = assign6160_e6399_d_n0;
        locals.var_vje_s_dn1 = assign6160_e6399_d_n1;
        locals.var_vje_s_dn3 = assign6160_e6399_d_n3;
        locals.var_vje_s_dn4 = assign6160_e6399_d_n4;
        locals.var_vje_s_dn5 = assign6160_e6399_d_n5;
        locals.var_vje_s_dn6 = assign6160_e6399_d_n6;
        locals.var_vje_s_dn7 = assign6160_e6399_d_n7;
        locals.var_vje_s_dn8 = assign6160_e6399_d_n8;
        locals.var_vje_s_dn9 = assign6160_e6399_d_n9;
        locals.var_vje_s_dn10 = assign6160_e6399_d_n10;
        locals.var_vje_s_dn11 = assign6160_e6399_d_n11;
        locals.var_vje_s_rv = 0.0;

        let assign6170_e6402: f64 = (p.p68 * locals.var_cje_t);
        let assign6170_e6406: f64 = (1.0 - p.p67);
        let assign6170_e6407: f64 = (locals.var_vde_t / assign6170_e6406);
        let assign6170_e6412: f64 = (locals.var_vje_s * locals.var_inv_vde_t);
        let assign6170_e6413: f64 = (1.0 - assign6170_e6412);
        let assign6170_e6416: f64 = (1.0 - p.p67);
        let assign6170_e6417: f64 = (assign6170_e6413).powf(assign6170_e6416);
        let assign6170_e6418: f64 = (1.0 - assign6170_e6417);
        let assign6170_e6419: f64 = (assign6170_e6407 * assign6170_e6418);
        let assign6170_e6423: f64 = (locals.var_vb1e1 - locals.var_vje_s);
        let assign6170_e6424: f64 = (3.0 * assign6170_e6423);
        let assign6170_e6425: f64 = (assign6170_e6419 + assign6170_e6424);
        let assign6170_e6426: f64 = (assign6170_e6402 * assign6170_e6425);
        locals.var_qte_s = assign6170_e6426;
        locals.var_qte_s_dn0 = (((p.p68 * locals.var_cje_t_dn0) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn0 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn0 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn0))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn0)))));
        locals.var_qte_s_dn1 = (((p.p68 * locals.var_cje_t_dn1) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn1 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn1 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn1))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn1)))));
        locals.var_qte_s_dn3 = (((p.p68 * locals.var_cje_t_dn3) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn3 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn3 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn3))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn3)))));
        locals.var_qte_s_dn4 = (((p.p68 * locals.var_cje_t_dn4) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn4 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn4 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn4))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn4)))));
        locals.var_qte_s_dn5 = (((p.p68 * locals.var_cje_t_dn5) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn5 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn5 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn5))) / assign6170_e6413))) }))) + (3.0 * (locals.var_vb1e1_dn5 - locals.var_vje_s_dn5)))));
        locals.var_qte_s_dn6 = (((p.p68 * locals.var_cje_t_dn6) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn6 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn6 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn6))) / assign6170_e6413))) }))) + (3.0 * (locals.var_vb1e1_dn6 - locals.var_vje_s_dn6)))));
        locals.var_qte_s_dn7 = (((p.p68 * locals.var_cje_t_dn7) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn7 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn7 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn7))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn7)))));
        locals.var_qte_s_dn8 = (((p.p68 * locals.var_cje_t_dn8) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn8 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn8 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn8))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn8)))));
        locals.var_qte_s_dn9 = (((p.p68 * locals.var_cje_t_dn9) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn9 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn9 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn9))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn9)))));
        locals.var_qte_s_dn10 = (((p.p68 * locals.var_cje_t_dn10) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn10 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn10 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn10))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn10 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn10))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn10)))));
        locals.var_qte_s_dn11 = (((p.p68 * locals.var_cje_t_dn11) * assign6170_e6425) + (assign6170_e6402 * ((((locals.var_vde_t_dn11 / assign6170_e6406) * assign6170_e6418) + (assign6170_e6407 * (-if 0.0 == 0.0 && ((assign6170_e6416) as f64).is_finite() && ((assign6170_e6416) as f64).fract() == 0.0 { if assign6170_e6416 == 0.0 { 0.0 } else { (assign6170_e6416 * ((assign6170_e6413).powf(assign6170_e6416 - 1.0) * (-((locals.var_vje_s_dn11 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn11))))) } } else { (assign6170_e6417 * (assign6170_e6416 * ((-((locals.var_vje_s_dn11 * locals.var_inv_vde_t) + (locals.var_vje_s * locals.var_inv_vde_t_dn11))) / assign6170_e6413))) }))) + (3.0 * (-locals.var_vje_s_dn11)))));
        locals.var_qte_s_rv = 0.0;

        let assign6180_e6429: f64 = (p.p77 * locals.var_cjc_t);
        let assign6180_e6431: f64 = (assign6180_e6429 * locals.var_vtc);
        locals.var_qtc = assign6180_e6431;
        locals.var_qtc_dn0 = (((p.p77 * locals.var_cjc_t_dn0) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn0));
        locals.var_qtc_dn1 = (((p.p77 * locals.var_cjc_t_dn1) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn1));
        locals.var_qtc_dn3 = (((p.p77 * locals.var_cjc_t_dn3) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn3));
        locals.var_qtc_dn4 = (((p.p77 * locals.var_cjc_t_dn4) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn4));
        locals.var_qtc_dn5 = (((p.p77 * locals.var_cjc_t_dn5) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn5));
        locals.var_qtc_dn6 = (((p.p77 * locals.var_cjc_t_dn6) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn6));
        locals.var_qtc_dn7 = (((p.p77 * locals.var_cjc_t_dn7) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn7));
        locals.var_qtc_dn8 = (((p.p77 * locals.var_cjc_t_dn8) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn8));
        locals.var_qtc_dn9 = (((p.p77 * locals.var_cjc_t_dn9) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn9));
        locals.var_qtc_dn10 = (((p.p77 * locals.var_cjc_t_dn10) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn10));
        locals.var_qtc_dn11 = (((p.p77 * locals.var_cjc_t_dn11) * locals.var_vtc) + (assign6180_e6429 * locals.var_vtc_dn11));
        locals.var_qtc_rv = 0.0;

        let assign6190_e6434: f64 = (locals.var_taub_t * locals.var_ik_t);
        locals.var_qb0 = assign6190_e6434;
        locals.var_qb0_dn4 = ((locals.var_taub_t_dn4 * locals.var_ik_t) + (locals.var_taub_t * locals.var_ik_t_dn4));
        locals.var_qb0_rv = 0.0;

        let assign6200_e6437: f64 = (0.5 * locals.var_qb0);
        let assign6200_e6439: f64 = (assign6200_e6437 * locals.var_n0);
        let assign6200_e6441: f64 = (assign6200_e6439 * locals.var_q1q);
        locals.var_qbe_qs = assign6200_e6441;
        locals.var_qbe_qs_dn0 = (((assign6200_e6437 * locals.var_n0_dn0) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn0));
        locals.var_qbe_qs_dn1 = (((assign6200_e6437 * locals.var_n0_dn1) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn1));
        locals.var_qbe_qs_dn3 = (((assign6200_e6437 * locals.var_n0_dn3) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn3));
        locals.var_qbe_qs_dn4 = (((((0.5 * locals.var_qb0_dn4) * locals.var_n0) + (assign6200_e6437 * locals.var_n0_dn4)) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn4));
        locals.var_qbe_qs_dn5 = (((assign6200_e6437 * locals.var_n0_dn5) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn5));
        locals.var_qbe_qs_dn6 = (((assign6200_e6437 * locals.var_n0_dn6) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn6));
        locals.var_qbe_qs_dn7 = (((assign6200_e6437 * locals.var_n0_dn7) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn7));
        locals.var_qbe_qs_dn8 = (((assign6200_e6437 * locals.var_n0_dn8) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn8));
        locals.var_qbe_qs_dn9 = (((assign6200_e6437 * locals.var_n0_dn9) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn9));
        locals.var_qbe_qs_dn10 = (((assign6200_e6437 * locals.var_n0_dn10) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn10));
        locals.var_qbe_qs_dn11 = (((assign6200_e6437 * locals.var_n0_dn11) * locals.var_q1q) + (assign6200_e6439 * locals.var_q1q_dn11));
        locals.var_qbe_qs_rv = 0.0;

        let assign6210_e6444: f64 = (0.5 * locals.var_qb0);
        let assign6210_e6446: f64 = (assign6210_e6444 * locals.var_nb);
        let assign6210_e6448: f64 = (assign6210_e6446 * locals.var_q1q);
        locals.var_qbc_qs = assign6210_e6448;
        locals.var_qbc_qs_dn0 = (((assign6210_e6444 * locals.var_nb_dn0) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn0));
        locals.var_qbc_qs_dn1 = (((assign6210_e6444 * locals.var_nb_dn1) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn1));
        locals.var_qbc_qs_dn3 = (((assign6210_e6444 * locals.var_nb_dn3) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn3));
        locals.var_qbc_qs_dn4 = (((((0.5 * locals.var_qb0_dn4) * locals.var_nb) + (assign6210_e6444 * locals.var_nb_dn4)) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn4));
        locals.var_qbc_qs_dn5 = (((assign6210_e6444 * locals.var_nb_dn5) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn5));
        locals.var_qbc_qs_dn6 = (((assign6210_e6444 * locals.var_nb_dn6) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn6));
        locals.var_qbc_qs_dn7 = (((assign6210_e6444 * locals.var_nb_dn7) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn7));
        locals.var_qbc_qs_dn8 = (((assign6210_e6444 * locals.var_nb_dn8) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn8));
        locals.var_qbc_qs_dn9 = (((assign6210_e6444 * locals.var_nb_dn9) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn9));
        locals.var_qbc_qs_dn10 = (((assign6210_e6444 * locals.var_nb_dn10) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn10));
        locals.var_qbc_qs_dn11 = (((assign6210_e6444 * locals.var_nb_dn11) * locals.var_q1q) + (assign6210_e6446 * locals.var_q1q_dn11));
        locals.var_qbc_qs_rv = 0.0;

        let assign6220_e6451: f64 = (0.1 * locals.var_vdc_ctc_t);
        locals.var_a_vdcctc = assign6220_e6451;
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
        locals.var_a_vdcctc_dn11 = (0.1 * locals.var_vdc_ctc_t_dn11);
        locals.var_a_vdcctc_rv = 0.0;

        let assign6230_e6454: f64 = (locals.var_vb1c4 - locals.var_vfc);
        let assign6230_e6456: f64 = (assign6230_e6454 / locals.var_a_vdcctc);
        locals.var_dxa = assign6230_e6456;
        locals.var_dxa_dn0 = ((((-locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((-locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((-locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((-locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vb1c4_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vb1c4_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((locals.var_vb1c4_dn8 - locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((locals.var_vb1c4_dn9 - locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn10 = ((((-locals.var_vfc_dn10) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn10)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn11 = ((((locals.var_vb1c4_dn11 - locals.var_vfc_dn11) * locals.var_a_vdcctc) - (assign6230_e6454 * locals.var_a_vdcctc_dn11)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_rv = 0.0;

        let assign6240_e6459: f64 = if locals.var_vb1c4 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6240_e6459;
        locals.var_guard114_rv = 0.0;

        let (assign6250_e6471, assign6250_e6471_d_n0, assign6250_e6471_d_n1, assign6250_e6471_d_n3, assign6250_e6471_d_n4, assign6250_e6471_d_n5, assign6250_e6471_d_n6, assign6250_e6471_d_n7, assign6250_e6471_d_n8, assign6250_e6471_d_n9, assign6250_e6471_d_n10, assign6250_e6471_d_n11,) = {
    if (locals.var_guard114 != 0.0) {
        let assign6250_e6465: f64 = (locals.var_dxa).exp();
        let assign6250_e6466: f64 = (1.0 + assign6250_e6465);
        let assign6250_e6467: f64 = (assign6250_e6466).ln();
        let assign6250_e6468: f64 = (locals.var_a_vdcctc * assign6250_e6467);
        let assign6250_e6469: f64 = (locals.var_vb1c4 - assign6250_e6468);
        (assign6250_e6469, (-((locals.var_a_vdcctc_dn0 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn0) / assign6250_e6466)))), (-((locals.var_a_vdcctc_dn1 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn1) / assign6250_e6466)))), (-((locals.var_a_vdcctc_dn3 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn3) / assign6250_e6466)))), (-((locals.var_a_vdcctc_dn4 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn4) / assign6250_e6466)))), (-((locals.var_a_vdcctc_dn5 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn5) / assign6250_e6466)))), (locals.var_vb1c4_dn6 - ((locals.var_a_vdcctc_dn6 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn6) / assign6250_e6466)))), (locals.var_vb1c4_dn7 - ((locals.var_a_vdcctc_dn7 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn7) / assign6250_e6466)))), (locals.var_vb1c4_dn8 - ((locals.var_a_vdcctc_dn8 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn8) / assign6250_e6466)))), (locals.var_vb1c4_dn9 - ((locals.var_a_vdcctc_dn9 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn9) / assign6250_e6466)))), (-((locals.var_a_vdcctc_dn10 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn10) / assign6250_e6466)))), (locals.var_vb1c4_dn11 - ((locals.var_a_vdcctc_dn11 * assign6250_e6467) + (locals.var_a_vdcctc * ((assign6250_e6465 * locals.var_dxa_dn11) / assign6250_e6466)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9, locals.var_vjcex_dn10, locals.var_vjcex_dn11,)
    }
};
        locals.var_vjcex = assign6250_e6471;
        locals.var_vjcex_dn0 = assign6250_e6471_d_n0;
        locals.var_vjcex_dn1 = assign6250_e6471_d_n1;
        locals.var_vjcex_dn3 = assign6250_e6471_d_n3;
        locals.var_vjcex_dn4 = assign6250_e6471_d_n4;
        locals.var_vjcex_dn5 = assign6250_e6471_d_n5;
        locals.var_vjcex_dn6 = assign6250_e6471_d_n6;
        locals.var_vjcex_dn7 = assign6250_e6471_d_n7;
        locals.var_vjcex_dn8 = assign6250_e6471_d_n8;
        locals.var_vjcex_dn9 = assign6250_e6471_d_n9;
        locals.var_vjcex_dn10 = assign6250_e6471_d_n10;
        locals.var_vjcex_dn11 = assign6250_e6471_d_n11;
        locals.var_vjcex_rv = 0.0;

        let (assign6260_e6485, assign6260_e6485_d_n0, assign6260_e6485_d_n1, assign6260_e6485_d_n3, assign6260_e6485_d_n4, assign6260_e6485_d_n5, assign6260_e6485_d_n6, assign6260_e6485_d_n7, assign6260_e6485_d_n8, assign6260_e6485_d_n9, assign6260_e6485_d_n10, assign6260_e6485_d_n11,) = {
    if (locals.var_guard114 == 0.0) {
        let assign6260_e6478: f64 = (-locals.var_dxa);
        let assign6260_e6479: f64 = (assign6260_e6478).exp();
        let assign6260_e6480: f64 = (1.0 + assign6260_e6479);
        let assign6260_e6481: f64 = (assign6260_e6480).ln();
        let assign6260_e6482: f64 = (locals.var_a_vdcctc * assign6260_e6481);
        let assign6260_e6483: f64 = (locals.var_vfc - assign6260_e6482);
        (assign6260_e6483, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn0)) / assign6260_e6480)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn1)) / assign6260_e6480)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn3)) / assign6260_e6480)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn4)) / assign6260_e6480)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn5)) / assign6260_e6480)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn6)) / assign6260_e6480)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn7)) / assign6260_e6480)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn8)) / assign6260_e6480)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn9)) / assign6260_e6480)))), (locals.var_vfc_dn10 - ((locals.var_a_vdcctc_dn10 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn10)) / assign6260_e6480)))), (locals.var_vfc_dn11 - ((locals.var_a_vdcctc_dn11 * assign6260_e6481) + (locals.var_a_vdcctc * ((assign6260_e6479 * (-locals.var_dxa_dn11)) / assign6260_e6480)))),)
    } else {
        (locals.var_vjcex, locals.var_vjcex_dn0, locals.var_vjcex_dn1, locals.var_vjcex_dn3, locals.var_vjcex_dn4, locals.var_vjcex_dn5, locals.var_vjcex_dn6, locals.var_vjcex_dn7, locals.var_vjcex_dn8, locals.var_vjcex_dn9, locals.var_vjcex_dn10, locals.var_vjcex_dn11,)
    }
};
        locals.var_vjcex = assign6260_e6485;
        locals.var_vjcex_dn0 = assign6260_e6485_d_n0;
        locals.var_vjcex_dn1 = assign6260_e6485_d_n1;
        locals.var_vjcex_dn3 = assign6260_e6485_d_n3;
        locals.var_vjcex_dn4 = assign6260_e6485_d_n4;
        locals.var_vjcex_dn5 = assign6260_e6485_d_n5;
        locals.var_vjcex_dn6 = assign6260_e6485_d_n6;
        locals.var_vjcex_dn7 = assign6260_e6485_d_n7;
        locals.var_vjcex_dn8 = assign6260_e6485_d_n8;
        locals.var_vjcex_dn9 = assign6260_e6485_d_n9;
        locals.var_vjcex_dn10 = assign6260_e6485_d_n10;
        locals.var_vjcex_dn11 = assign6260_e6485_d_n11;
        locals.var_vjcex_rv = 0.0;

        let assign6270_e6489: f64 = (1.0 - p.p72);
        let assign6270_e6490: f64 = (locals.var_vdc_ctc_t / assign6270_e6489);
        let assign6270_e6495: f64 = (locals.var_vjcex / locals.var_vdc_ctc_t);
        let assign6270_e6496: f64 = (1.0 - assign6270_e6495);
        let assign6270_e6499: f64 = (1.0 - p.p72);
        let assign6270_e6500: f64 = (assign6270_e6496).powf(assign6270_e6499);
        let assign6270_e6501: f64 = (1.0 - assign6270_e6500);
        let assign6270_e6502: f64 = (assign6270_e6490 * assign6270_e6501);
        let assign6270_e6506: f64 = (locals.var_vb1c4 - locals.var_vjcex);
        let assign6270_e6507: f64 = (locals.var_bjc * assign6270_e6506);
        let assign6270_e6508: f64 = (assign6270_e6502 + assign6270_e6507);
        locals.var_vtexv = assign6270_e6508;
        locals.var_vtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn0 * assign6270_e6506) + (locals.var_bjc * (-locals.var_vjcex_dn0))));
        locals.var_vtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn1 * assign6270_e6506) + (locals.var_bjc * (-locals.var_vjcex_dn1))));
        locals.var_vtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn3 * assign6270_e6506) + (locals.var_bjc * (-locals.var_vjcex_dn3))));
        locals.var_vtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn4 * assign6270_e6506) + (locals.var_bjc * (-locals.var_vjcex_dn4))));
        locals.var_vtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn5 * assign6270_e6506) + (locals.var_bjc * (-locals.var_vjcex_dn5))));
        locals.var_vtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn6 * assign6270_e6506) + (locals.var_bjc * (locals.var_vb1c4_dn6 - locals.var_vjcex_dn6))));
        locals.var_vtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn7 * assign6270_e6506) + (locals.var_bjc * (locals.var_vb1c4_dn7 - locals.var_vjcex_dn7))));
        locals.var_vtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn8 * assign6270_e6506) + (locals.var_bjc * (locals.var_vb1c4_dn8 - locals.var_vjcex_dn8))));
        locals.var_vtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn9 * assign6270_e6506) + (locals.var_bjc * (locals.var_vb1c4_dn9 - locals.var_vjcex_dn9))));
        locals.var_vtexv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn10 * assign6270_e6506) + (locals.var_bjc * (-locals.var_vjcex_dn10))));
        locals.var_vtexv_dn11 = ((((locals.var_vdc_ctc_t_dn11 / assign6270_e6489) * assign6270_e6501) + (assign6270_e6490 * (-if 0.0 == 0.0 && ((assign6270_e6499) as f64).is_finite() && ((assign6270_e6499) as f64).fract() == 0.0 { if assign6270_e6499 == 0.0 { 0.0 } else { (assign6270_e6499 * ((assign6270_e6496).powf(assign6270_e6499 - 1.0) * (-(((locals.var_vjcex_dn11 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn11)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6270_e6500 * (assign6270_e6499 * ((-(((locals.var_vjcex_dn11 * locals.var_vdc_ctc_t) - (locals.var_vjcex * locals.var_vdc_ctc_t_dn11)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6270_e6496))) }))) + ((locals.var_bjc_dn11 * assign6270_e6506) + (locals.var_bjc * (locals.var_vb1c4_dn11 - locals.var_vjcex_dn11))));
        locals.var_vtexv_rv = 0.0;

        let assign6280_e6512: f64 = (1.0 - locals.var_xp_t);
        let assign6280_e6514: f64 = (assign6280_e6512 * locals.var_vtexv);
        let assign6280_e6517: f64 = (locals.var_xp_t * locals.var_vb1c4);
        let assign6280_e6518: f64 = (assign6280_e6514 + assign6280_e6517);
        let assign6280_e6519: f64 = (locals.var_cjc_t * assign6280_e6518);
        let assign6280_e6522: f64 = (1.0 - p.p77);
        let assign6280_e6523: f64 = (assign6280_e6519 * assign6280_e6522);
        let assign6280_e6526: f64 = (1.0 - p.p33);
        let assign6280_e6527: f64 = (assign6280_e6523 * assign6280_e6526);
        locals.var_qtex = assign6280_e6527;
        locals.var_qtex_dn0 = ((((locals.var_cjc_t_dn0 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn0)) + (locals.var_xp_t_dn0 * locals.var_vb1c4)))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn1 = ((((locals.var_cjc_t_dn1 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn1)) + (locals.var_xp_t_dn1 * locals.var_vb1c4)))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn3 = ((((locals.var_cjc_t_dn3 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vb1c4)))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn4 = ((((locals.var_cjc_t_dn4 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vb1c4)))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn5 = ((((locals.var_cjc_t_dn5 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn5)) + (locals.var_xp_t_dn5 * locals.var_vb1c4)))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn6 = ((((locals.var_cjc_t_dn6 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn6))))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn7 = ((((locals.var_cjc_t_dn7 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn7))))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn8 = ((((locals.var_cjc_t_dn8 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn8))))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn9 = ((((locals.var_cjc_t_dn9 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn9)) + ((locals.var_xp_t_dn9 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn9))))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn10 = ((((locals.var_cjc_t_dn10 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn10) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn10)) + (locals.var_xp_t_dn10 * locals.var_vb1c4)))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_dn11 = ((((locals.var_cjc_t_dn11 * assign6280_e6518) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn11) * locals.var_vtexv) + (assign6280_e6512 * locals.var_vtexv_dn11)) + ((locals.var_xp_t_dn11 * locals.var_vb1c4) + (locals.var_xp_t * locals.var_vb1c4_dn11))))) * assign6280_e6522) * assign6280_e6526);
        locals.var_qtex_rv = 0.0;

        let assign6290_e6530: f64 = (locals.var_vbc3 - locals.var_vfc);
        let assign6290_e6532: f64 = (assign6290_e6530 / locals.var_a_vdcctc);
        locals.var_dxa = assign6290_e6532;
        locals.var_dxa_dn0 = ((((locals.var_vbc3_dn0 - locals.var_vfc_dn0) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn0)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn1 = ((((locals.var_vbc3_dn1 - locals.var_vfc_dn1) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn1)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn3 = ((((-locals.var_vfc_dn3) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn3)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn4 = ((((-locals.var_vfc_dn4) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn4)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn5 = ((((-locals.var_vfc_dn5) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn5)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn6 = ((((locals.var_vbc3_dn6 - locals.var_vfc_dn6) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn6)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn7 = ((((locals.var_vbc3_dn7 - locals.var_vfc_dn7) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn7)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn8 = ((((locals.var_vbc3_dn8 - locals.var_vfc_dn8) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn8)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn9 = ((((locals.var_vbc3_dn9 - locals.var_vfc_dn9) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn9)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn10 = ((((locals.var_vbc3_dn10 - locals.var_vfc_dn10) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn10)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_dn11 = ((((locals.var_vbc3_dn11 - locals.var_vfc_dn11) * locals.var_a_vdcctc) - (assign6290_e6530 * locals.var_a_vdcctc_dn11)) / (locals.var_a_vdcctc * locals.var_a_vdcctc));
        locals.var_dxa_rv = 0.0;

        let assign6300_e6535: f64 = if locals.var_vbc3 < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard115 = assign6300_e6535;
        locals.var_guard115_rv = 0.0;

        let (assign6310_e6547, assign6310_e6547_d_n0, assign6310_e6547_d_n1, assign6310_e6547_d_n3, assign6310_e6547_d_n4, assign6310_e6547_d_n5, assign6310_e6547_d_n6, assign6310_e6547_d_n7, assign6310_e6547_d_n8, assign6310_e6547_d_n9, assign6310_e6547_d_n10, assign6310_e6547_d_n11,) = {
    if (locals.var_guard115 != 0.0) {
        let assign6310_e6541: f64 = (locals.var_dxa).exp();
        let assign6310_e6542: f64 = (1.0 + assign6310_e6541);
        let assign6310_e6543: f64 = (assign6310_e6542).ln();
        let assign6310_e6544: f64 = (locals.var_a_vdcctc * assign6310_e6543);
        let assign6310_e6545: f64 = (locals.var_vbc3 - assign6310_e6544);
        (assign6310_e6545, (locals.var_vbc3_dn0 - ((locals.var_a_vdcctc_dn0 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn0) / assign6310_e6542)))), (locals.var_vbc3_dn1 - ((locals.var_a_vdcctc_dn1 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn1) / assign6310_e6542)))), (-((locals.var_a_vdcctc_dn3 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn3) / assign6310_e6542)))), (-((locals.var_a_vdcctc_dn4 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn4) / assign6310_e6542)))), (-((locals.var_a_vdcctc_dn5 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn5) / assign6310_e6542)))), (locals.var_vbc3_dn6 - ((locals.var_a_vdcctc_dn6 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn6) / assign6310_e6542)))), (locals.var_vbc3_dn7 - ((locals.var_a_vdcctc_dn7 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn7) / assign6310_e6542)))), (locals.var_vbc3_dn8 - ((locals.var_a_vdcctc_dn8 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn8) / assign6310_e6542)))), (locals.var_vbc3_dn9 - ((locals.var_a_vdcctc_dn9 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn9) / assign6310_e6542)))), (locals.var_vbc3_dn10 - ((locals.var_a_vdcctc_dn10 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn10) / assign6310_e6542)))), (locals.var_vbc3_dn11 - ((locals.var_a_vdcctc_dn11 * assign6310_e6543) + (locals.var_a_vdcctc * ((assign6310_e6541 * locals.var_dxa_dn11) / assign6310_e6542)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9, locals.var_xvjcex_dn10, locals.var_xvjcex_dn11,)
    }
};
        locals.var_xvjcex = assign6310_e6547;
        locals.var_xvjcex_dn0 = assign6310_e6547_d_n0;
        locals.var_xvjcex_dn1 = assign6310_e6547_d_n1;
        locals.var_xvjcex_dn3 = assign6310_e6547_d_n3;
        locals.var_xvjcex_dn4 = assign6310_e6547_d_n4;
        locals.var_xvjcex_dn5 = assign6310_e6547_d_n5;
        locals.var_xvjcex_dn6 = assign6310_e6547_d_n6;
        locals.var_xvjcex_dn7 = assign6310_e6547_d_n7;
        locals.var_xvjcex_dn8 = assign6310_e6547_d_n8;
        locals.var_xvjcex_dn9 = assign6310_e6547_d_n9;
        locals.var_xvjcex_dn10 = assign6310_e6547_d_n10;
        locals.var_xvjcex_dn11 = assign6310_e6547_d_n11;
        locals.var_xvjcex_rv = 0.0;

        let (assign6320_e6561, assign6320_e6561_d_n0, assign6320_e6561_d_n1, assign6320_e6561_d_n3, assign6320_e6561_d_n4, assign6320_e6561_d_n5, assign6320_e6561_d_n6, assign6320_e6561_d_n7, assign6320_e6561_d_n8, assign6320_e6561_d_n9, assign6320_e6561_d_n10, assign6320_e6561_d_n11,) = {
    if (locals.var_guard115 == 0.0) {
        let assign6320_e6554: f64 = (-locals.var_dxa);
        let assign6320_e6555: f64 = (assign6320_e6554).exp();
        let assign6320_e6556: f64 = (1.0 + assign6320_e6555);
        let assign6320_e6557: f64 = (assign6320_e6556).ln();
        let assign6320_e6558: f64 = (locals.var_a_vdcctc * assign6320_e6557);
        let assign6320_e6559: f64 = (locals.var_vfc - assign6320_e6558);
        (assign6320_e6559, (locals.var_vfc_dn0 - ((locals.var_a_vdcctc_dn0 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn0)) / assign6320_e6556)))), (locals.var_vfc_dn1 - ((locals.var_a_vdcctc_dn1 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn1)) / assign6320_e6556)))), (locals.var_vfc_dn3 - ((locals.var_a_vdcctc_dn3 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn3)) / assign6320_e6556)))), (locals.var_vfc_dn4 - ((locals.var_a_vdcctc_dn4 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn4)) / assign6320_e6556)))), (locals.var_vfc_dn5 - ((locals.var_a_vdcctc_dn5 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn5)) / assign6320_e6556)))), (locals.var_vfc_dn6 - ((locals.var_a_vdcctc_dn6 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn6)) / assign6320_e6556)))), (locals.var_vfc_dn7 - ((locals.var_a_vdcctc_dn7 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn7)) / assign6320_e6556)))), (locals.var_vfc_dn8 - ((locals.var_a_vdcctc_dn8 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn8)) / assign6320_e6556)))), (locals.var_vfc_dn9 - ((locals.var_a_vdcctc_dn9 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn9)) / assign6320_e6556)))), (locals.var_vfc_dn10 - ((locals.var_a_vdcctc_dn10 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn10)) / assign6320_e6556)))), (locals.var_vfc_dn11 - ((locals.var_a_vdcctc_dn11 * assign6320_e6557) + (locals.var_a_vdcctc * ((assign6320_e6555 * (-locals.var_dxa_dn11)) / assign6320_e6556)))),)
    } else {
        (locals.var_xvjcex, locals.var_xvjcex_dn0, locals.var_xvjcex_dn1, locals.var_xvjcex_dn3, locals.var_xvjcex_dn4, locals.var_xvjcex_dn5, locals.var_xvjcex_dn6, locals.var_xvjcex_dn7, locals.var_xvjcex_dn8, locals.var_xvjcex_dn9, locals.var_xvjcex_dn10, locals.var_xvjcex_dn11,)
    }
};
        locals.var_xvjcex = assign6320_e6561;
        locals.var_xvjcex_dn0 = assign6320_e6561_d_n0;
        locals.var_xvjcex_dn1 = assign6320_e6561_d_n1;
        locals.var_xvjcex_dn3 = assign6320_e6561_d_n3;
        locals.var_xvjcex_dn4 = assign6320_e6561_d_n4;
        locals.var_xvjcex_dn5 = assign6320_e6561_d_n5;
        locals.var_xvjcex_dn6 = assign6320_e6561_d_n6;
        locals.var_xvjcex_dn7 = assign6320_e6561_d_n7;
        locals.var_xvjcex_dn8 = assign6320_e6561_d_n8;
        locals.var_xvjcex_dn9 = assign6320_e6561_d_n9;
        locals.var_xvjcex_dn10 = assign6320_e6561_d_n10;
        locals.var_xvjcex_dn11 = assign6320_e6561_d_n11;
        locals.var_xvjcex_rv = 0.0;

        let assign6330_e6565: f64 = (1.0 - p.p72);
        let assign6330_e6566: f64 = (locals.var_vdc_ctc_t / assign6330_e6565);
        let assign6330_e6571: f64 = (locals.var_xvjcex / locals.var_vdc_ctc_t);
        let assign6330_e6572: f64 = (1.0 - assign6330_e6571);
        let assign6330_e6575: f64 = (1.0 - p.p72);
        let assign6330_e6576: f64 = (assign6330_e6572).powf(assign6330_e6575);
        let assign6330_e6577: f64 = (1.0 - assign6330_e6576);
        let assign6330_e6578: f64 = (assign6330_e6566 * assign6330_e6577);
        let assign6330_e6582: f64 = (locals.var_vbc3 - locals.var_xvjcex);
        let assign6330_e6583: f64 = (locals.var_bjc * assign6330_e6582);
        let assign6330_e6584: f64 = (assign6330_e6578 + assign6330_e6583);
        locals.var_xvtexv = assign6330_e6584;
        locals.var_xvtexv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn0 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn0 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn0 - locals.var_xvjcex_dn0))));
        locals.var_xvtexv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn1 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn1 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn1 - locals.var_xvjcex_dn1))));
        locals.var_xvtexv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn3 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn3 * assign6330_e6582) + (locals.var_bjc * (-locals.var_xvjcex_dn3))));
        locals.var_xvtexv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn4 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn4 * assign6330_e6582) + (locals.var_bjc * (-locals.var_xvjcex_dn4))));
        locals.var_xvtexv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn5 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn5 * assign6330_e6582) + (locals.var_bjc * (-locals.var_xvjcex_dn5))));
        locals.var_xvtexv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn6 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn6 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn6 - locals.var_xvjcex_dn6))));
        locals.var_xvtexv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn7 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn7 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn7 - locals.var_xvjcex_dn7))));
        locals.var_xvtexv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn8 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn8 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn8 - locals.var_xvjcex_dn8))));
        locals.var_xvtexv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn9 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn9 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn9 - locals.var_xvjcex_dn9))));
        locals.var_xvtexv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn10 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn10 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn10 - locals.var_xvjcex_dn10))));
        locals.var_xvtexv_dn11 = ((((locals.var_vdc_ctc_t_dn11 / assign6330_e6565) * assign6330_e6577) + (assign6330_e6566 * (-if 0.0 == 0.0 && ((assign6330_e6575) as f64).is_finite() && ((assign6330_e6575) as f64).fract() == 0.0 { if assign6330_e6575 == 0.0 { 0.0 } else { (assign6330_e6575 * ((assign6330_e6572).powf(assign6330_e6575 - 1.0) * (-(((locals.var_xvjcex_dn11 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn11)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign6330_e6576 * (assign6330_e6575 * ((-(((locals.var_xvjcex_dn11 * locals.var_vdc_ctc_t) - (locals.var_xvjcex * locals.var_vdc_ctc_t_dn11)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign6330_e6572))) }))) + ((locals.var_bjc_dn11 * assign6330_e6582) + (locals.var_bjc * (locals.var_vbc3_dn11 - locals.var_xvjcex_dn11))));
        locals.var_xvtexv_rv = 0.0;

        let assign6340_e6588: f64 = (1.0 - locals.var_xp_t);
        let assign6340_e6590: f64 = (assign6340_e6588 * locals.var_xvtexv);
        let assign6340_e6593: f64 = (locals.var_xp_t * locals.var_vbc3);
        let assign6340_e6594: f64 = (assign6340_e6590 + assign6340_e6593);
        let assign6340_e6595: f64 = (locals.var_cjc_t * assign6340_e6594);
        let assign6340_e6598: f64 = (1.0 - p.p77);
        let assign6340_e6599: f64 = (assign6340_e6595 * assign6340_e6598);
        let assign6340_e6601: f64 = (assign6340_e6599 * p.p33);
        locals.var_xqtex = assign6340_e6601;
        locals.var_xqtex_dn0 = ((((locals.var_cjc_t_dn0 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn0) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn0)) + ((locals.var_xp_t_dn0 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn0))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn1 = ((((locals.var_cjc_t_dn1 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn1) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn1)) + ((locals.var_xp_t_dn1 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn1))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn3 = ((((locals.var_cjc_t_dn3 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn3) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vbc3)))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn4 = ((((locals.var_cjc_t_dn4 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn4) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vbc3)))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn5 = ((((locals.var_cjc_t_dn5 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn5) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn5)) + (locals.var_xp_t_dn5 * locals.var_vbc3)))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn6 = ((((locals.var_cjc_t_dn6 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn6) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn6)) + ((locals.var_xp_t_dn6 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn6))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn7 = ((((locals.var_cjc_t_dn7 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn7) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn7))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn8 = ((((locals.var_cjc_t_dn8 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn8) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn8))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn9 = ((((locals.var_cjc_t_dn9 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn9) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn9)) + ((locals.var_xp_t_dn9 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn9))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn10 = ((((locals.var_cjc_t_dn10 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn10) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn10)) + ((locals.var_xp_t_dn10 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn10))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_dn11 = ((((locals.var_cjc_t_dn11 * assign6340_e6594) + (locals.var_cjc_t * ((((-locals.var_xp_t_dn11) * locals.var_xvtexv) + (assign6340_e6588 * locals.var_xvtexv_dn11)) + ((locals.var_xp_t_dn11 * locals.var_vbc3) + (locals.var_xp_t * locals.var_vbc3_dn11))))) * assign6340_e6598) * p.p33);
        locals.var_xqtex_rv = 0.0;

        let assign6350_e6604: f64 = (0.1 * locals.var_vds_t);
        locals.var_a_vds = assign6350_e6604;
        locals.var_a_vds_dn0 = (0.1 * locals.var_vds_t_dn0);
        locals.var_a_vds_dn1 = (0.1 * locals.var_vds_t_dn1);
        locals.var_a_vds_dn3 = (0.1 * locals.var_vds_t_dn3);
        locals.var_a_vds_dn4 = (0.1 * locals.var_vds_t_dn4);
        locals.var_a_vds_dn5 = (0.1 * locals.var_vds_t_dn5);
        locals.var_a_vds_dn6 = (0.1 * locals.var_vds_t_dn6);
        locals.var_a_vds_dn7 = (0.1 * locals.var_vds_t_dn7);
        locals.var_a_vds_dn8 = (0.1 * locals.var_vds_t_dn8);
        locals.var_a_vds_dn9 = (0.1 * locals.var_vds_t_dn9);
        locals.var_a_vds_dn10 = (0.1 * locals.var_vds_t_dn10);
        locals.var_a_vds_dn11 = (0.1 * locals.var_vds_t_dn11);
        locals.var_a_vds_rv = 0.0;

        let assign6360_e6609: f64 = (-1.0);
        let assign6360_e6611: f64 = (assign6360_e6609 / p.p139);
        let assign6360_e6612: f64 = (2.0_f64).powf(assign6360_e6611);
        let assign6360_e6613: f64 = (1.0 - assign6360_e6612);
        let assign6360_e6614: f64 = (locals.var_vds_t * assign6360_e6613);
        locals.var_vfs = assign6360_e6614;
        locals.var_vfs_dn0 = (locals.var_vds_t_dn0 * assign6360_e6613);
        locals.var_vfs_dn1 = (locals.var_vds_t_dn1 * assign6360_e6613);
        locals.var_vfs_dn3 = (locals.var_vds_t_dn3 * assign6360_e6613);
        locals.var_vfs_dn4 = (locals.var_vds_t_dn4 * assign6360_e6613);
        locals.var_vfs_dn5 = (locals.var_vds_t_dn5 * assign6360_e6613);
        locals.var_vfs_dn6 = (locals.var_vds_t_dn6 * assign6360_e6613);
        locals.var_vfs_dn7 = (locals.var_vds_t_dn7 * assign6360_e6613);
        locals.var_vfs_dn8 = (locals.var_vds_t_dn8 * assign6360_e6613);
        locals.var_vfs_dn9 = (locals.var_vds_t_dn9 * assign6360_e6613);
        locals.var_vfs_dn10 = (locals.var_vds_t_dn10 * assign6360_e6613);
        locals.var_vfs_dn11 = (locals.var_vds_t_dn11 * assign6360_e6613);
        locals.var_vfs_rv = 0.0;

        let assign6370_e6617: f64 = (locals.var_vsc1 - locals.var_vfs);
        let assign6370_e6619: f64 = (assign6370_e6617 / locals.var_a_vds);
        locals.var_dxa = assign6370_e6619;
        locals.var_dxa_dn0 = ((((-locals.var_vfs_dn0) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn0)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn1 = ((((-locals.var_vfs_dn1) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn1)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn3 = ((((locals.var_vsc1_dn3 - locals.var_vfs_dn3) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn3)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn4 = ((((-locals.var_vfs_dn4) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn4)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn5 = ((((-locals.var_vfs_dn5) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn5)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn6 = ((((-locals.var_vfs_dn6) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn6)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn7 = ((((-locals.var_vfs_dn7) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn7)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn8 = ((((locals.var_vsc1_dn8 - locals.var_vfs_dn8) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn8)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn9 = ((((-locals.var_vfs_dn9) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn9)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn10 = ((((-locals.var_vfs_dn10) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn10)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_dn11 = ((((-locals.var_vfs_dn11) * locals.var_a_vds) - (assign6370_e6617 * locals.var_a_vds_dn11)) / (locals.var_a_vds * locals.var_a_vds));
        locals.var_dxa_rv = 0.0;

        let assign6380_e6622: f64 = if locals.var_vsc1 < locals.var_vfs { 1.0 } else { 0.0 };
        locals.var_guard116 = assign6380_e6622;
        locals.var_guard116_rv = 0.0;

        let (assign6390_e6634, assign6390_e6634_d_n0, assign6390_e6634_d_n1, assign6390_e6634_d_n3, assign6390_e6634_d_n4, assign6390_e6634_d_n5, assign6390_e6634_d_n6, assign6390_e6634_d_n7, assign6390_e6634_d_n8, assign6390_e6634_d_n9, assign6390_e6634_d_n10, assign6390_e6634_d_n11,) = {
    if (locals.var_guard116 != 0.0) {
        let assign6390_e6628: f64 = (locals.var_dxa).exp();
        let assign6390_e6629: f64 = (1.0 + assign6390_e6628);
        let assign6390_e6630: f64 = (assign6390_e6629).ln();
        let assign6390_e6631: f64 = (locals.var_a_vds * assign6390_e6630);
        let assign6390_e6632: f64 = (locals.var_vsc1 - assign6390_e6631);
        (assign6390_e6632, (-((locals.var_a_vds_dn0 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn0) / assign6390_e6629)))), (-((locals.var_a_vds_dn1 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn1) / assign6390_e6629)))), (locals.var_vsc1_dn3 - ((locals.var_a_vds_dn3 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn3) / assign6390_e6629)))), (-((locals.var_a_vds_dn4 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn4) / assign6390_e6629)))), (-((locals.var_a_vds_dn5 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn5) / assign6390_e6629)))), (-((locals.var_a_vds_dn6 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn6) / assign6390_e6629)))), (-((locals.var_a_vds_dn7 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn7) / assign6390_e6629)))), (locals.var_vsc1_dn8 - ((locals.var_a_vds_dn8 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn8) / assign6390_e6629)))), (-((locals.var_a_vds_dn9 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn9) / assign6390_e6629)))), (-((locals.var_a_vds_dn10 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn10) / assign6390_e6629)))), (-((locals.var_a_vds_dn11 * assign6390_e6630) + (locals.var_a_vds * ((assign6390_e6628 * locals.var_dxa_dn11) / assign6390_e6629)))),)
    } else {
        (locals.var_vjs, locals.var_vjs_dn0, locals.var_vjs_dn1, locals.var_vjs_dn3, locals.var_vjs_dn4, locals.var_vjs_dn5, locals.var_vjs_dn6, locals.var_vjs_dn7, locals.var_vjs_dn8, locals.var_vjs_dn9, locals.var_vjs_dn10, locals.var_vjs_dn11,)
    }
};
        locals.var_vjs = assign6390_e6634;
        locals.var_vjs_dn0 = assign6390_e6634_d_n0;
        locals.var_vjs_dn1 = assign6390_e6634_d_n1;
        locals.var_vjs_dn3 = assign6390_e6634_d_n3;
        locals.var_vjs_dn4 = assign6390_e6634_d_n4;
        locals.var_vjs_dn5 = assign6390_e6634_d_n5;
        locals.var_vjs_dn6 = assign6390_e6634_d_n6;
        locals.var_vjs_dn7 = assign6390_e6634_d_n7;
        locals.var_vjs_dn8 = assign6390_e6634_d_n8;
        locals.var_vjs_dn9 = assign6390_e6634_d_n9;
        locals.var_vjs_dn10 = assign6390_e6634_d_n10;
        locals.var_vjs_dn11 = assign6390_e6634_d_n11;
        locals.var_vjs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6400_e6648, assign6400_e6648_d_n0, assign6400_e6648_d_n1, assign6400_e6648_d_n3, assign6400_e6648_d_n4, assign6400_e6648_d_n5, assign6400_e6648_d_n6, assign6400_e6648_d_n7, assign6400_e6648_d_n8, assign6400_e6648_d_n9, assign6400_e6648_d_n10, assign6400_e6648_d_n11,) = {
    if (locals.var_guard116 == 0.0) {
        let assign6400_e6641: f64 = (-locals.var_dxa);
        let assign6400_e6642: f64 = (assign6400_e6641).exp();
        let assign6400_e6643: f64 = (1.0 + assign6400_e6642);
        let assign6400_e6644: f64 = (assign6400_e6643).ln();
        let assign6400_e6645: f64 = (locals.var_a_vds * assign6400_e6644);
        let assign6400_e6646: f64 = (locals.var_vfs - assign6400_e6645);
        (assign6400_e6646, (locals.var_vfs_dn0 - ((locals.var_a_vds_dn0 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn0)) / assign6400_e6643)))), (locals.var_vfs_dn1 - ((locals.var_a_vds_dn1 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn1)) / assign6400_e6643)))), (locals.var_vfs_dn3 - ((locals.var_a_vds_dn3 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn3)) / assign6400_e6643)))), (locals.var_vfs_dn4 - ((locals.var_a_vds_dn4 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn4)) / assign6400_e6643)))), (locals.var_vfs_dn5 - ((locals.var_a_vds_dn5 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn5)) / assign6400_e6643)))), (locals.var_vfs_dn6 - ((locals.var_a_vds_dn6 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn6)) / assign6400_e6643)))), (locals.var_vfs_dn7 - ((locals.var_a_vds_dn7 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn7)) / assign6400_e6643)))), (locals.var_vfs_dn8 - ((locals.var_a_vds_dn8 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn8)) / assign6400_e6643)))), (locals.var_vfs_dn9 - ((locals.var_a_vds_dn9 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn9)) / assign6400_e6643)))), (locals.var_vfs_dn10 - ((locals.var_a_vds_dn10 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn10)) / assign6400_e6643)))), (locals.var_vfs_dn11 - ((locals.var_a_vds_dn11 * assign6400_e6644) + (locals.var_a_vds * ((assign6400_e6642 * (-locals.var_dxa_dn11)) / assign6400_e6643)))),)
    } else {
        (locals.var_vjs, locals.var_vjs_dn0, locals.var_vjs_dn1, locals.var_vjs_dn3, locals.var_vjs_dn4, locals.var_vjs_dn5, locals.var_vjs_dn6, locals.var_vjs_dn7, locals.var_vjs_dn8, locals.var_vjs_dn9, locals.var_vjs_dn10, locals.var_vjs_dn11,)
    }
};
        locals.var_vjs = assign6400_e6648;
        locals.var_vjs_dn0 = assign6400_e6648_d_n0;
        locals.var_vjs_dn1 = assign6400_e6648_d_n1;
        locals.var_vjs_dn3 = assign6400_e6648_d_n3;
        locals.var_vjs_dn4 = assign6400_e6648_d_n4;
        locals.var_vjs_dn5 = assign6400_e6648_d_n5;
        locals.var_vjs_dn6 = assign6400_e6648_d_n6;
        locals.var_vjs_dn7 = assign6400_e6648_d_n7;
        locals.var_vjs_dn8 = assign6400_e6648_d_n8;
        locals.var_vjs_dn9 = assign6400_e6648_d_n9;
        locals.var_vjs_dn10 = assign6400_e6648_d_n10;
        locals.var_vjs_dn11 = assign6400_e6648_d_n11;
        locals.var_vjs_rv = 0.0;

        let assign6410_e6653: f64 = (1.0 - p.p139);
        let assign6410_e6654: f64 = (locals.var_vds_t / assign6410_e6653);
        let assign6410_e6659: f64 = (locals.var_vjs / locals.var_vds_t);
        let assign6410_e6660: f64 = (1.0 - assign6410_e6659);
        let assign6410_e6663: f64 = (1.0 - p.p139);
        let assign6410_e6664: f64 = (assign6410_e6660).powf(assign6410_e6663);
        let assign6410_e6665: f64 = (1.0 - assign6410_e6664);
        let assign6410_e6666: f64 = (assign6410_e6654 * assign6410_e6665);
        let assign6410_e6670: f64 = (locals.var_vsc1 - locals.var_vjs);
        let assign6410_e6671: f64 = (2.0 * assign6410_e6670);
        let assign6410_e6672: f64 = (assign6410_e6666 + assign6410_e6671);
        let assign6410_e6673: f64 = (locals.var_cjs_t * assign6410_e6672);
        locals.var_qts = assign6410_e6673;
        locals.var_qts_dn0 = ((locals.var_cjs_t_dn0 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn0 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn0 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn0)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn0 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn0)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn0)))));
        locals.var_qts_dn1 = ((locals.var_cjs_t_dn1 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn1 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn1 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn1)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn1 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn1)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn1)))));
        locals.var_qts_dn3 = ((locals.var_cjs_t_dn3 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn3 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn3 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn3)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn3 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn3)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (locals.var_vsc1_dn3 - locals.var_vjs_dn3)))));
        locals.var_qts_dn4 = ((locals.var_cjs_t_dn4 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn4 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn4 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn4)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn4 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn4)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn4)))));
        locals.var_qts_dn5 = ((locals.var_cjs_t_dn5 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn5 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn5 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn5)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn5 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn5)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn5)))));
        locals.var_qts_dn6 = ((locals.var_cjs_t_dn6 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn6 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn6 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn6)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn6 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn6)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn6)))));
        locals.var_qts_dn7 = ((locals.var_cjs_t_dn7 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn7 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn7 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn7)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn7 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn7)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn7)))));
        locals.var_qts_dn8 = ((locals.var_cjs_t_dn8 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn8 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn8 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn8)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn8 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn8)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (locals.var_vsc1_dn8 - locals.var_vjs_dn8)))));
        locals.var_qts_dn9 = ((locals.var_cjs_t_dn9 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn9 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn9 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn9)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn9 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn9)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn9)))));
        locals.var_qts_dn10 = ((locals.var_cjs_t_dn10 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn10 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn10 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn10)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn10 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn10)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn10)))));
        locals.var_qts_dn11 = ((locals.var_cjs_t_dn11 * assign6410_e6672) + (locals.var_cjs_t * ((((locals.var_vds_t_dn11 / assign6410_e6653) * assign6410_e6665) + (assign6410_e6654 * (-if 0.0 == 0.0 && ((assign6410_e6663) as f64).is_finite() && ((assign6410_e6663) as f64).fract() == 0.0 { if assign6410_e6663 == 0.0 { 0.0 } else { (assign6410_e6663 * ((assign6410_e6660).powf(assign6410_e6663 - 1.0) * (-(((locals.var_vjs_dn11 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn11)) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign6410_e6664 * (assign6410_e6663 * ((-(((locals.var_vjs_dn11 * locals.var_vds_t) - (locals.var_vjs * locals.var_vds_t_dn11)) / (locals.var_vds_t * locals.var_vds_t))) / assign6410_e6660))) }))) + (2.0 * (-locals.var_vjs_dn11)))));
        locals.var_qts_rv = 0.0;

        let assign6420_e6676: f64 = (locals.var_taue_t * locals.var_ik_t);
        let assign6420_e6679: f64 = (locals.var_is_t / locals.var_ik_t);
        let assign6420_e6682: f64 = (1.0 / p.p85);
        let assign6420_e6683: f64 = (assign6420_e6679).powf(assign6420_e6682);
        let assign6420_e6684: f64 = (assign6420_e6676 * assign6420_e6683);
        locals.var_qe0 = assign6420_e6684;
        locals.var_qe0_dn0 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn0 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn0 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn1 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn1 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn1 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn3 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn3 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn3 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn4 = ((((locals.var_taue_t_dn4 * locals.var_ik_t) + (locals.var_taue_t * locals.var_ik_t_dn4)) * assign6420_e6683) + (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (((locals.var_is_t_dn4 * locals.var_ik_t) - (locals.var_is_t * locals.var_ik_t_dn4)) / (locals.var_ik_t * locals.var_ik_t)))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((((locals.var_is_t_dn4 * locals.var_ik_t) - (locals.var_is_t * locals.var_ik_t_dn4)) / (locals.var_ik_t * locals.var_ik_t)) / assign6420_e6679))) }));
        locals.var_qe0_dn5 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn5 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn5 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn6 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn6 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn6 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn7 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn7 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn7 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn8 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn8 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn8 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn9 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn9 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn9 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn10 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn10 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn10 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_dn11 = (assign6420_e6676 * if 0.0 == 0.0 && ((assign6420_e6682) as f64).is_finite() && ((assign6420_e6682) as f64).fract() == 0.0 { if assign6420_e6682 == 0.0 { 0.0 } else { (assign6420_e6682 * ((assign6420_e6679).powf(assign6420_e6682 - 1.0) * (locals.var_is_t_dn11 / locals.var_ik_t))) } } else { (assign6420_e6683 * (assign6420_e6682 * ((locals.var_is_t_dn11 / locals.var_ik_t) / assign6420_e6679))) });
        locals.var_qe0_rv = 0.0;

        let assign6430_e6688: f64 = (p.p85 * locals.var_vt);
        let assign6430_e6689: f64 = (locals.var_vb2e1 / assign6430_e6688);
        let assign6430_e6691: f64 = if assign6430_e6689 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign6430_e6691;
        locals.var_guard117_rv = 0.0;

        let (assign6440_e6700, assign6440_e6700_d_n0, assign6440_e6700_d_n1, assign6440_e6700_d_n3, assign6440_e6700_d_n4, assign6440_e6700_d_n5, assign6440_e6700_d_n6, assign6440_e6700_d_n7, assign6440_e6700_d_n8, assign6440_e6700_d_n9, assign6440_e6700_d_n10, assign6440_e6700_d_n11,) = {
    if (locals.var_guard117 != 0.0) {
        let assign6440_e6696: f64 = (p.p85 * locals.var_vt);
        let assign6440_e6697: f64 = (locals.var_vb2e1 / assign6440_e6696);
        let assign6440_e6698: f64 = (assign6440_e6697).exp();
        (assign6440_e6698, 0.0, 0.0, 0.0, (assign6440_e6698 * (-((locals.var_vb2e1 * (p.p85 * locals.var_vt_dn4)) / (assign6440_e6696 * assign6440_e6696)))), (assign6440_e6698 * (locals.var_vb2e1_dn5 / assign6440_e6696)), 0.0, (assign6440_e6698 * (locals.var_vb2e1_dn7 / assign6440_e6696)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign6440_e6700;
        locals.var_tmpexp_dn0 = assign6440_e6700_d_n0;
        locals.var_tmpexp_dn1 = assign6440_e6700_d_n1;
        locals.var_tmpexp_dn3 = assign6440_e6700_d_n3;
        locals.var_tmpexp_dn4 = assign6440_e6700_d_n4;
        locals.var_tmpexp_dn5 = assign6440_e6700_d_n5;
        locals.var_tmpexp_dn6 = assign6440_e6700_d_n6;
        locals.var_tmpexp_dn7 = assign6440_e6700_d_n7;
        locals.var_tmpexp_dn8 = assign6440_e6700_d_n8;
        locals.var_tmpexp_dn9 = assign6440_e6700_d_n9;
        locals.var_tmpexp_dn10 = assign6440_e6700_d_n10;
        locals.var_tmpexp_dn11 = assign6440_e6700_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign6450_e6706,) = {
    if (locals.var_guard117 == 0.0) {
        let assign6450_e6704: f64 = (p.p151).exp();
        (assign6450_e6704,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6450_e6706;
        locals.var_expl_rv = 0.0;

        let (assign6460_e6721, assign6460_e6721_d_n0, assign6460_e6721_d_n1, assign6460_e6721_d_n3, assign6460_e6721_d_n4, assign6460_e6721_d_n5, assign6460_e6721_d_n6, assign6460_e6721_d_n7, assign6460_e6721_d_n8, assign6460_e6721_d_n9, assign6460_e6721_d_n10, assign6460_e6721_d_n11,) = {
    if (locals.var_guard117 == 0.0) {
        let assign6460_e6714: f64 = (p.p85 * locals.var_vt);
        let assign6460_e6715: f64 = (locals.var_vb2e1 / assign6460_e6714);
        let assign6460_e6717: f64 = (assign6460_e6715 - p.p151);
        let assign6460_e6718: f64 = (1.0 + assign6460_e6717);
        let assign6460_e6719: f64 = (locals.var_expl * assign6460_e6718);
        (assign6460_e6719, 0.0, 0.0, 0.0, (locals.var_expl * (-((locals.var_vb2e1 * (p.p85 * locals.var_vt_dn4)) / (assign6460_e6714 * assign6460_e6714)))), (locals.var_expl * (locals.var_vb2e1_dn5 / assign6460_e6714)), 0.0, (locals.var_expl * (locals.var_vb2e1_dn7 / assign6460_e6714)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign6460_e6721;
        locals.var_tmpexp_dn0 = assign6460_e6721_d_n0;
        locals.var_tmpexp_dn1 = assign6460_e6721_d_n1;
        locals.var_tmpexp_dn3 = assign6460_e6721_d_n3;
        locals.var_tmpexp_dn4 = assign6460_e6721_d_n4;
        locals.var_tmpexp_dn5 = assign6460_e6721_d_n5;
        locals.var_tmpexp_dn6 = assign6460_e6721_d_n6;
        locals.var_tmpexp_dn7 = assign6460_e6721_d_n7;
        locals.var_tmpexp_dn8 = assign6460_e6721_d_n8;
        locals.var_tmpexp_dn9 = assign6460_e6721_d_n9;
        locals.var_tmpexp_dn10 = assign6460_e6721_d_n10;
        locals.var_tmpexp_dn11 = assign6460_e6721_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign6470_e6724: f64 = (locals.var_qe0 * locals.var_tmpexp);
        locals.var_qe_qs = assign6470_e6724;
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
        locals.var_qe_qs_dn11 = ((locals.var_qe0_dn11 * locals.var_tmpexp) + (locals.var_qe0 * locals.var_tmpexp_dn11));
        locals.var_qe_qs_rv = 0.0;

        let assign6480_e6727: f64 = (4.0 * locals.var_tepi_t);
        let assign6480_e6729: f64 = (assign6480_e6727 * locals.var_vt);
        let assign6480_e6731: f64 = (assign6480_e6729 / locals.var_rcv_t);
        locals.var_qepi0 = assign6480_e6731;
        locals.var_qepi0_dn4 = ((((((4.0 * locals.var_tepi_t_dn4) * locals.var_vt) + (assign6480_e6727 * locals.var_vt_dn4)) * locals.var_rcv_t) - (assign6480_e6729 * locals.var_rcv_t_dn4)) / (locals.var_rcv_t * locals.var_rcv_t));
        locals.var_qepi0_rv = 0.0;

        let assign6490_e6734: f64 = (0.5 * locals.var_qepi0);
        let assign6490_e6736: f64 = (assign6490_e6734 * locals.var_xi_w);
        let assign6490_e6739: f64 = (locals.var_p0star + locals.var_pw);
        let assign6490_e6741: f64 = (assign6490_e6739 + 2.0);
        let assign6490_e6742: f64 = (assign6490_e6736 * assign6490_e6741);
        locals.var_qepi = assign6490_e6742;
        locals.var_qepi_dn0 = (((assign6490_e6734 * locals.var_xi_w_dn0) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn0 + locals.var_pw_dn0)));
        locals.var_qepi_dn1 = (((assign6490_e6734 * locals.var_xi_w_dn1) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn1 + locals.var_pw_dn1)));
        locals.var_qepi_dn3 = (((assign6490_e6734 * locals.var_xi_w_dn3) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn3 + locals.var_pw_dn3)));
        locals.var_qepi_dn4 = (((((0.5 * locals.var_qepi0_dn4) * locals.var_xi_w) + (assign6490_e6734 * locals.var_xi_w_dn4)) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn4 + locals.var_pw_dn4)));
        locals.var_qepi_dn5 = (((assign6490_e6734 * locals.var_xi_w_dn5) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn5 + locals.var_pw_dn5)));
        locals.var_qepi_dn6 = (((assign6490_e6734 * locals.var_xi_w_dn6) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn6 + locals.var_pw_dn6)));
        locals.var_qepi_dn7 = (((assign6490_e6734 * locals.var_xi_w_dn7) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn7 + locals.var_pw_dn7)));
        locals.var_qepi_dn8 = (((assign6490_e6734 * locals.var_xi_w_dn8) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn8 + locals.var_pw_dn8)));
        locals.var_qepi_dn9 = (((assign6490_e6734 * locals.var_xi_w_dn9) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn9 + locals.var_pw_dn9)));
        locals.var_qepi_dn10 = (((assign6490_e6734 * locals.var_xi_w_dn10) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn10 + locals.var_pw_dn10)));
        locals.var_qepi_dn11 = (((assign6490_e6734 * locals.var_xi_w_dn11) * assign6490_e6741) + (assign6490_e6736 * (locals.var_p0star_dn11 + locals.var_pw_dn11)));
        locals.var_qepi_rv = 0.0;

        let assign6500_e6745: f64 = if p.p79 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign6500_e6745;
        locals.var_guard118_rv = 0.0;

        let (assign6510_e6763, assign6510_e6763_d_n0, assign6510_e6763_d_n1, assign6510_e6763_d_n3, assign6510_e6763_d_n4, assign6510_e6763_d_n5, assign6510_e6763_d_n6, assign6510_e6763_d_n7, assign6510_e6763_d_n8, assign6510_e6763_d_n9, assign6510_e6763_d_n10, assign6510_e6763_d_n11,) = {
    if (locals.var_guard118 != 0.0) {
        let assign6510_e6749: f64 = (locals.var_taur_t * 0.5);
        let assign6510_e6752: f64 = (locals.var_qb0 * locals.var_nbex);
        let assign6510_e6755: f64 = (locals.var_qepi0 * locals.var_pwex);
        let assign6510_e6756: f64 = (assign6510_e6752 + assign6510_e6755);
        let assign6510_e6757: f64 = (assign6510_e6749 * assign6510_e6756);
        let assign6510_e6760: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign6510_e6761: f64 = (assign6510_e6757 / assign6510_e6760);
        (assign6510_e6761, ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn0) + (locals.var_qepi0 * locals.var_pwex_dn0))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn1) + (locals.var_qepi0 * locals.var_pwex_dn1))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn3) + (locals.var_qepi0 * locals.var_pwex_dn3))) / assign6510_e6760), ((((((locals.var_taur_t_dn4 * 0.5) * assign6510_e6756) + (assign6510_e6749 * (((locals.var_qb0_dn4 * locals.var_nbex) + (locals.var_qb0 * locals.var_nbex_dn4)) + ((locals.var_qepi0_dn4 * locals.var_pwex) + (locals.var_qepi0 * locals.var_pwex_dn4))))) * assign6510_e6760) - (assign6510_e6757 * (locals.var_taub_t_dn4 + locals.var_tepi_t_dn4))) / (assign6510_e6760 * assign6510_e6760)), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn5) + (locals.var_qepi0 * locals.var_pwex_dn5))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn6) + (locals.var_qepi0 * locals.var_pwex_dn6))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn7) + (locals.var_qepi0 * locals.var_pwex_dn7))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn8) + (locals.var_qepi0 * locals.var_pwex_dn8))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn9) + (locals.var_qepi0 * locals.var_pwex_dn9))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn10) + (locals.var_qepi0 * locals.var_pwex_dn10))) / assign6510_e6760), ((assign6510_e6749 * ((locals.var_qb0 * locals.var_nbex_dn11) + (locals.var_qepi0 * locals.var_pwex_dn11))) / assign6510_e6760),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10, locals.var_qex_dn11,)
    }
};
        locals.var_qex = assign6510_e6763;
        locals.var_qex_dn0 = assign6510_e6763_d_n0;
        locals.var_qex_dn1 = assign6510_e6763_d_n1;
        locals.var_qex_dn3 = assign6510_e6763_d_n3;
        locals.var_qex_dn4 = assign6510_e6763_d_n4;
        locals.var_qex_dn5 = assign6510_e6763_d_n5;
        locals.var_qex_dn6 = assign6510_e6763_d_n6;
        locals.var_qex_dn7 = assign6510_e6763_d_n7;
        locals.var_qex_dn8 = assign6510_e6763_d_n8;
        locals.var_qex_dn9 = assign6510_e6763_d_n9;
        locals.var_qex_dn10 = assign6510_e6763_d_n10;
        locals.var_qex_dn11 = assign6510_e6763_d_n11;
        locals.var_qex_rv = 0.0;

        let assign6520_e6766: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6520_e6768: f64 = (assign6520_e6766 / p.p91);
        let assign6520_e6770: f64 = (assign6520_e6768 * locals.var_vtinv);
        let assign6520_e6772: f64 = if assign6520_e6770 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign6520_e6772;
        locals.var_guard119_rv = 0.0;

        let (assign6530_e6786, assign6530_e6786_d_n0, assign6530_e6786_d_n1, assign6530_e6786_d_n3, assign6530_e6786_d_n4, assign6530_e6786_d_n5, assign6530_e6786_d_n6, assign6530_e6786_d_n7, assign6530_e6786_d_n8, assign6530_e6786_d_n9, assign6530_e6786_d_n10, assign6530_e6786_d_n11,) = {
    if ((locals.var_guard118 == 0.0) && (locals.var_guard119 != 0.0)) {
        let assign6530_e6779: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6530_e6781: f64 = (assign6530_e6779 / p.p91);
        let assign6530_e6783: f64 = (assign6530_e6781 * locals.var_vtinv);
        let assign6530_e6784: f64 = (assign6530_e6783).exp();
        (assign6530_e6784, (assign6530_e6784 * (((-locals.var_vdcex_t_dn0) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((-locals.var_vdcex_t_dn1) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((-locals.var_vdcex_t_dn3) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * ((((-locals.var_vdcex_t_dn4) / p.p91) * locals.var_vtinv) + (assign6530_e6781 * locals.var_vtinv_dn4))), (assign6530_e6784 * (((-locals.var_vdcex_t_dn5) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((locals.var_vb1c4_dn8 - locals.var_vdcex_t_dn8) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((locals.var_vb1c4_dn9 - locals.var_vdcex_t_dn9) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((-locals.var_vdcex_t_dn10) / p.p91) * locals.var_vtinv)), (assign6530_e6784 * (((locals.var_vb1c4_dn11 - locals.var_vdcex_t_dn11) / p.p91) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9, locals.var_evb1c4vdcex_dn10, locals.var_evb1c4vdcex_dn11,)
    }
};
        locals.var_evb1c4vdcex = assign6530_e6786;
        locals.var_evb1c4vdcex_dn0 = assign6530_e6786_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign6530_e6786_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign6530_e6786_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign6530_e6786_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign6530_e6786_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign6530_e6786_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign6530_e6786_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign6530_e6786_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign6530_e6786_d_n9;
        locals.var_evb1c4vdcex_dn10 = assign6530_e6786_d_n10;
        locals.var_evb1c4vdcex_dn11 = assign6530_e6786_d_n11;
        locals.var_evb1c4vdcex_rv = 0.0;

        let (assign6540_e6795,) = {
    if ((locals.var_guard118 == 0.0) && (locals.var_guard119 == 0.0)) {
        let assign6540_e6793: f64 = (p.p151).exp();
        (assign6540_e6793,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6540_e6795;
        locals.var_expl_rv = 0.0;

        let (assign6550_e6815, assign6550_e6815_d_n0, assign6550_e6815_d_n1, assign6550_e6815_d_n3, assign6550_e6815_d_n4, assign6550_e6815_d_n5, assign6550_e6815_d_n6, assign6550_e6815_d_n7, assign6550_e6815_d_n8, assign6550_e6815_d_n9, assign6550_e6815_d_n10, assign6550_e6815_d_n11,) = {
    if ((locals.var_guard118 == 0.0) && (locals.var_guard119 == 0.0)) {
        let assign6550_e6805: f64 = (locals.var_vb1c4 - locals.var_vdcex_t);
        let assign6550_e6807: f64 = (assign6550_e6805 / p.p91);
        let assign6550_e6809: f64 = (assign6550_e6807 * locals.var_vtinv);
        let assign6550_e6811: f64 = (assign6550_e6809 - p.p151);
        let assign6550_e6812: f64 = (1.0 + assign6550_e6811);
        let assign6550_e6813: f64 = (locals.var_expl * assign6550_e6812);
        (assign6550_e6813, (locals.var_expl * (((-locals.var_vdcex_t_dn0) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn1) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn3) / p.p91) * locals.var_vtinv)), (locals.var_expl * ((((-locals.var_vdcex_t_dn4) / p.p91) * locals.var_vtinv) + (assign6550_e6807 * locals.var_vtinv_dn4))), (locals.var_expl * (((-locals.var_vdcex_t_dn5) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn6 - locals.var_vdcex_t_dn6) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn7 - locals.var_vdcex_t_dn7) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn8 - locals.var_vdcex_t_dn8) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn9 - locals.var_vdcex_t_dn9) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn10) / p.p91) * locals.var_vtinv)), (locals.var_expl * (((locals.var_vb1c4_dn11 - locals.var_vdcex_t_dn11) / p.p91) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdcex, locals.var_evb1c4vdcex_dn0, locals.var_evb1c4vdcex_dn1, locals.var_evb1c4vdcex_dn3, locals.var_evb1c4vdcex_dn4, locals.var_evb1c4vdcex_dn5, locals.var_evb1c4vdcex_dn6, locals.var_evb1c4vdcex_dn7, locals.var_evb1c4vdcex_dn8, locals.var_evb1c4vdcex_dn9, locals.var_evb1c4vdcex_dn10, locals.var_evb1c4vdcex_dn11,)
    }
};
        locals.var_evb1c4vdcex = assign6550_e6815;
        locals.var_evb1c4vdcex_dn0 = assign6550_e6815_d_n0;
        locals.var_evb1c4vdcex_dn1 = assign6550_e6815_d_n1;
        locals.var_evb1c4vdcex_dn3 = assign6550_e6815_d_n3;
        locals.var_evb1c4vdcex_dn4 = assign6550_e6815_d_n4;
        locals.var_evb1c4vdcex_dn5 = assign6550_e6815_d_n5;
        locals.var_evb1c4vdcex_dn6 = assign6550_e6815_d_n6;
        locals.var_evb1c4vdcex_dn7 = assign6550_e6815_d_n7;
        locals.var_evb1c4vdcex_dn8 = assign6550_e6815_d_n8;
        locals.var_evb1c4vdcex_dn9 = assign6550_e6815_d_n9;
        locals.var_evb1c4vdcex_dn10 = assign6550_e6815_d_n10;
        locals.var_evb1c4vdcex_dn11 = assign6550_e6815_d_n11;
        locals.var_evb1c4vdcex_rv = 0.0;

        let (assign6560_e6835, assign6560_e6835_d_n0, assign6560_e6835_d_n1, assign6560_e6835_d_n3, assign6560_e6835_d_n4, assign6560_e6835_d_n5, assign6560_e6835_d_n6, assign6560_e6835_d_n7, assign6560_e6835_d_n8, assign6560_e6835_d_n9, assign6560_e6835_d_n10, assign6560_e6835_d_n11,) = {
    if (locals.var_guard118 == 0.0) {
        let assign6560_e6820: f64 = (2.0 * locals.var_ibx_t);
        let assign6560_e6822: f64 = (assign6560_e6820 * locals.var_tauex_t);
        let assign6560_e6824: f64 = (assign6560_e6822 * locals.var_evb1c4);
        let assign6560_e6829: f64 = (4.0 * locals.var_evb1c4vdcex);
        let assign6560_e6830: f64 = (1.0 + assign6560_e6829);
        let assign6560_e6831: f64 = (assign6560_e6830).sqrt();
        let assign6560_e6832: f64 = (1.0 + assign6560_e6831);
        let assign6560_e6833: f64 = (assign6560_e6824 / assign6560_e6832);
        (assign6560_e6833, (-((assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn0) / (2.0 * assign6560_e6831))) / (assign6560_e6832 * assign6560_e6832))), (-((assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn1) / (2.0 * assign6560_e6831))) / (assign6560_e6832 * assign6560_e6832))), (-((assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn3) / (2.0 * assign6560_e6831))) / (assign6560_e6832 * assign6560_e6832))), ((((((((2.0 * locals.var_ibx_t_dn4) * locals.var_tauex_t) + (assign6560_e6820 * locals.var_tauex_t_dn4)) * locals.var_evb1c4) + (assign6560_e6822 * locals.var_evb1c4_dn4)) * assign6560_e6832) - (assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn4) / (2.0 * assign6560_e6831)))) / (assign6560_e6832 * assign6560_e6832)), (-((assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn5) / (2.0 * assign6560_e6831))) / (assign6560_e6832 * assign6560_e6832))), ((((assign6560_e6822 * locals.var_evb1c4_dn6) * assign6560_e6832) - (assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn6) / (2.0 * assign6560_e6831)))) / (assign6560_e6832 * assign6560_e6832)), ((((assign6560_e6822 * locals.var_evb1c4_dn7) * assign6560_e6832) - (assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn7) / (2.0 * assign6560_e6831)))) / (assign6560_e6832 * assign6560_e6832)), ((((assign6560_e6822 * locals.var_evb1c4_dn8) * assign6560_e6832) - (assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn8) / (2.0 * assign6560_e6831)))) / (assign6560_e6832 * assign6560_e6832)), ((((assign6560_e6822 * locals.var_evb1c4_dn9) * assign6560_e6832) - (assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn9) / (2.0 * assign6560_e6831)))) / (assign6560_e6832 * assign6560_e6832)), (-((assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn10) / (2.0 * assign6560_e6831))) / (assign6560_e6832 * assign6560_e6832))), ((((assign6560_e6822 * locals.var_evb1c4_dn11) * assign6560_e6832) - (assign6560_e6824 * ((4.0 * locals.var_evb1c4vdcex_dn11) / (2.0 * assign6560_e6831)))) / (assign6560_e6832 * assign6560_e6832)),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10, locals.var_qex_dn11,)
    }
};
        locals.var_qex = assign6560_e6835;
        locals.var_qex_dn0 = assign6560_e6835_d_n0;
        locals.var_qex_dn1 = assign6560_e6835_d_n1;
        locals.var_qex_dn3 = assign6560_e6835_d_n3;
        locals.var_qex_dn4 = assign6560_e6835_d_n4;
        locals.var_qex_dn5 = assign6560_e6835_d_n5;
        locals.var_qex_dn6 = assign6560_e6835_d_n6;
        locals.var_qex_dn7 = assign6560_e6835_d_n7;
        locals.var_qex_dn8 = assign6560_e6835_d_n8;
        locals.var_qex_dn9 = assign6560_e6835_d_n9;
        locals.var_qex_dn10 = assign6560_e6835_d_n10;
        locals.var_qex_dn11 = assign6560_e6835_d_n11;
        locals.var_qex_rv = 0.0;

        let assign6570_e6846: f64 = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign6570_e6846;
        locals.var_guard120_rv = 0.0;

        let (assign6580_e6852, assign6580_e6852_d_n0, assign6580_e6852_d_n1, assign6580_e6852_d_n3, assign6580_e6852_d_n4, assign6580_e6852_d_n5, assign6580_e6852_d_n6, assign6580_e6852_d_n7, assign6580_e6852_d_n8, assign6580_e6852_d_n9, assign6580_e6852_d_n10, assign6580_e6852_d_n11,) = {
    if (locals.var_guard120 != 0.0) {
        let assign6580_e6850: f64 = (locals.var_qex * locals.var_xext1);
        (assign6580_e6850, (locals.var_qex_dn0 * locals.var_xext1), (locals.var_qex_dn1 * locals.var_xext1), (locals.var_qex_dn3 * locals.var_xext1), (locals.var_qex_dn4 * locals.var_xext1), (locals.var_qex_dn5 * locals.var_xext1), (locals.var_qex_dn6 * locals.var_xext1), (locals.var_qex_dn7 * locals.var_xext1), (locals.var_qex_dn8 * locals.var_xext1), (locals.var_qex_dn9 * locals.var_xext1), (locals.var_qex_dn10 * locals.var_xext1), (locals.var_qex_dn11 * locals.var_xext1),)
    } else {
        (locals.var_qex, locals.var_qex_dn0, locals.var_qex_dn1, locals.var_qex_dn3, locals.var_qex_dn4, locals.var_qex_dn5, locals.var_qex_dn6, locals.var_qex_dn7, locals.var_qex_dn8, locals.var_qex_dn9, locals.var_qex_dn10, locals.var_qex_dn11,)
    }
};
        locals.var_qex = assign6580_e6852;
        locals.var_qex_dn0 = assign6580_e6852_d_n0;
        locals.var_qex_dn1 = assign6580_e6852_d_n1;
        locals.var_qex_dn3 = assign6580_e6852_d_n3;
        locals.var_qex_dn4 = assign6580_e6852_d_n4;
        locals.var_qex_dn5 = assign6580_e6852_d_n5;
        locals.var_qex_dn6 = assign6580_e6852_d_n6;
        locals.var_qex_dn7 = assign6580_e6852_d_n7;
        locals.var_qex_dn8 = assign6580_e6852_d_n8;
        locals.var_qex_dn9 = assign6580_e6852_d_n9;
        locals.var_qex_dn10 = assign6580_e6852_d_n10;
        locals.var_qex_dn11 = assign6580_e6852_d_n11;
        locals.var_qex_rv = 0.0;

        let assign6590_e6855: f64 = if p.p79 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign6590_e6855;
        locals.var_guard121_rv = 0.0;

        let (assign6600_e6863, assign6600_e6863_d_n0, assign6600_e6863_d_n1, assign6600_e6863_d_n3, assign6600_e6863_d_n4, assign6600_e6863_d_n5, assign6600_e6863_d_n6, assign6600_e6863_d_n7, assign6600_e6863_d_n8, assign6600_e6863_d_n9, assign6600_e6863_d_n10, assign6600_e6863_d_n11,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign6600_e6861: f64 = (locals.var_if0 * locals.var_evbc3);
        (assign6600_e6861, ((locals.var_if0_dn0 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn0)), ((locals.var_if0_dn1 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn1)), (locals.var_if0_dn3 * locals.var_evbc3), ((locals.var_if0_dn4 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn4)), (locals.var_if0_dn5 * locals.var_evbc3), ((locals.var_if0_dn6 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn6)), ((locals.var_if0_dn7 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn7)), ((locals.var_if0_dn8 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn8)), ((locals.var_if0_dn9 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn9)), ((locals.var_if0_dn10 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn10)), ((locals.var_if0_dn11 * locals.var_evbc3) + (locals.var_if0 * locals.var_evbc3_dn11)),)
    } else {
        (locals.var_xg1, locals.var_xg1_dn0, locals.var_xg1_dn1, locals.var_xg1_dn3, locals.var_xg1_dn4, locals.var_xg1_dn5, locals.var_xg1_dn6, locals.var_xg1_dn7, locals.var_xg1_dn8, locals.var_xg1_dn9, locals.var_xg1_dn10, locals.var_xg1_dn11,)
    }
};
        locals.var_xg1 = assign6600_e6863;
        locals.var_xg1_dn0 = assign6600_e6863_d_n0;
        locals.var_xg1_dn1 = assign6600_e6863_d_n1;
        locals.var_xg1_dn3 = assign6600_e6863_d_n3;
        locals.var_xg1_dn4 = assign6600_e6863_d_n4;
        locals.var_xg1_dn5 = assign6600_e6863_d_n5;
        locals.var_xg1_dn6 = assign6600_e6863_d_n6;
        locals.var_xg1_dn7 = assign6600_e6863_d_n7;
        locals.var_xg1_dn8 = assign6600_e6863_d_n8;
        locals.var_xg1_dn9 = assign6600_e6863_d_n9;
        locals.var_xg1_dn10 = assign6600_e6863_d_n10;
        locals.var_xg1_dn11 = assign6600_e6863_d_n11;
        locals.var_xg1_rv = 0.0;

        let (assign6610_e6878, assign6610_e6878_d_n0, assign6610_e6878_d_n1, assign6610_e6878_d_n3, assign6610_e6878_d_n4, assign6610_e6878_d_n5, assign6610_e6878_d_n6, assign6610_e6878_d_n7, assign6610_e6878_d_n8, assign6610_e6878_d_n9, assign6610_e6878_d_n10, assign6610_e6878_d_n11,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign6610_e6869: f64 = (locals.var_xg1 - locals.var_if0);
        let assign6610_e6873: f64 = (1.0 + locals.var_xg1);
        let assign6610_e6874: f64 = (assign6610_e6873).sqrt();
        let assign6610_e6875: f64 = (1.0 + assign6610_e6874);
        let assign6610_e6876: f64 = (assign6610_e6869 / assign6610_e6875);
        (assign6610_e6876, ((((locals.var_xg1_dn0 - locals.var_if0_dn0) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn0 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn1 - locals.var_if0_dn1) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn1 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn3 - locals.var_if0_dn3) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn3 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn4 - locals.var_if0_dn4) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn4 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn5 - locals.var_if0_dn5) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn5 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn6 - locals.var_if0_dn6) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn6 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn7 - locals.var_if0_dn7) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn7 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn8 - locals.var_if0_dn8) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn8 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn9 - locals.var_if0_dn9) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn9 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn10 - locals.var_if0_dn10) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn10 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((locals.var_xg1_dn11 - locals.var_if0_dn11) * assign6610_e6875) - (assign6610_e6869 * (locals.var_xg1_dn11 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)),)
    } else {
        (locals.var_xnbex, locals.var_xnbex_dn0, locals.var_xnbex_dn1, locals.var_xnbex_dn3, locals.var_xnbex_dn4, locals.var_xnbex_dn5, locals.var_xnbex_dn6, locals.var_xnbex_dn7, locals.var_xnbex_dn8, locals.var_xnbex_dn9, locals.var_xnbex_dn10, locals.var_xnbex_dn11,)
    }
};
        locals.var_xnbex = assign6610_e6878;
        locals.var_xnbex_dn0 = assign6610_e6878_d_n0;
        locals.var_xnbex_dn1 = assign6610_e6878_d_n1;
        locals.var_xnbex_dn3 = assign6610_e6878_d_n3;
        locals.var_xnbex_dn4 = assign6610_e6878_d_n4;
        locals.var_xnbex_dn5 = assign6610_e6878_d_n5;
        locals.var_xnbex_dn6 = assign6610_e6878_d_n6;
        locals.var_xnbex_dn7 = assign6610_e6878_d_n7;
        locals.var_xnbex_dn8 = assign6610_e6878_d_n8;
        locals.var_xnbex_dn9 = assign6610_e6878_d_n9;
        locals.var_xnbex_dn10 = assign6610_e6878_d_n10;
        locals.var_xnbex_dn11 = assign6610_e6878_d_n11;
        locals.var_xnbex_rv = 0.0;

        let (assign6620_e6886, assign6620_e6886_d_n0, assign6620_e6886_d_n1, assign6620_e6886_d_n3, assign6620_e6886_d_n4, assign6620_e6886_d_n5, assign6620_e6886_d_n6, assign6620_e6886_d_n7, assign6620_e6886_d_n8, assign6620_e6886_d_n9, assign6620_e6886_d_n10, assign6620_e6886_d_n11,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign6620_e6884: f64 = (4.0 * locals.var_evbc3vdc);
        (assign6620_e6884, (4.0 * locals.var_evbc3vdc_dn0), (4.0 * locals.var_evbc3vdc_dn1), (4.0 * locals.var_evbc3vdc_dn3), (4.0 * locals.var_evbc3vdc_dn4), (4.0 * locals.var_evbc3vdc_dn5), (4.0 * locals.var_evbc3vdc_dn6), (4.0 * locals.var_evbc3vdc_dn7), (4.0 * locals.var_evbc3vdc_dn8), (4.0 * locals.var_evbc3vdc_dn9), (4.0 * locals.var_evbc3vdc_dn10), (4.0 * locals.var_evbc3vdc_dn11),)
    } else {
        (locals.var_xg2, locals.var_xg2_dn0, locals.var_xg2_dn1, locals.var_xg2_dn3, locals.var_xg2_dn4, locals.var_xg2_dn5, locals.var_xg2_dn6, locals.var_xg2_dn7, locals.var_xg2_dn8, locals.var_xg2_dn9, locals.var_xg2_dn10, locals.var_xg2_dn11,)
    }
};
        locals.var_xg2 = assign6620_e6886;
        locals.var_xg2_dn0 = assign6620_e6886_d_n0;
        locals.var_xg2_dn1 = assign6620_e6886_d_n1;
        locals.var_xg2_dn3 = assign6620_e6886_d_n3;
        locals.var_xg2_dn4 = assign6620_e6886_d_n4;
        locals.var_xg2_dn5 = assign6620_e6886_d_n5;
        locals.var_xg2_dn6 = assign6620_e6886_d_n6;
        locals.var_xg2_dn7 = assign6620_e6886_d_n7;
        locals.var_xg2_dn8 = assign6620_e6886_d_n8;
        locals.var_xg2_dn9 = assign6620_e6886_d_n9;
        locals.var_xg2_dn10 = assign6620_e6886_d_n10;
        locals.var_xg2_dn11 = assign6620_e6886_d_n11;
        locals.var_xg2_rv = 0.0;

        let (assign6630_e6899, assign6630_e6899_d_n0, assign6630_e6899_d_n1, assign6630_e6899_d_n3, assign6630_e6899_d_n4, assign6630_e6899_d_n5, assign6630_e6899_d_n6, assign6630_e6899_d_n7, assign6630_e6899_d_n8, assign6630_e6899_d_n9, assign6630_e6899_d_n10, assign6630_e6899_d_n11,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign6630_e6894: f64 = (1.0 + locals.var_xg2);
        let assign6630_e6895: f64 = (assign6630_e6894).sqrt();
        let assign6630_e6896: f64 = (1.0 + assign6630_e6895);
        let assign6630_e6897: f64 = (locals.var_xg2 / assign6630_e6896);
        (assign6630_e6897, (((locals.var_xg2_dn0 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn0 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn1 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn1 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn3 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn3 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn4 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn4 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn5 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn5 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn6 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn6 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn7 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn7 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn8 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn8 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn9 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn9 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn10 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn10 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((locals.var_xg2_dn11 * assign6630_e6896) - (locals.var_xg2 * (locals.var_xg2_dn11 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)),)
    } else {
        (locals.var_xpwex, locals.var_xpwex_dn0, locals.var_xpwex_dn1, locals.var_xpwex_dn3, locals.var_xpwex_dn4, locals.var_xpwex_dn5, locals.var_xpwex_dn6, locals.var_xpwex_dn7, locals.var_xpwex_dn8, locals.var_xpwex_dn9, locals.var_xpwex_dn10, locals.var_xpwex_dn11,)
    }
};
        locals.var_xpwex = assign6630_e6899;
        locals.var_xpwex_dn0 = assign6630_e6899_d_n0;
        locals.var_xpwex_dn1 = assign6630_e6899_d_n1;
        locals.var_xpwex_dn3 = assign6630_e6899_d_n3;
        locals.var_xpwex_dn4 = assign6630_e6899_d_n4;
        locals.var_xpwex_dn5 = assign6630_e6899_d_n5;
        locals.var_xpwex_dn6 = assign6630_e6899_d_n6;
        locals.var_xpwex_dn7 = assign6630_e6899_d_n7;
        locals.var_xpwex_dn8 = assign6630_e6899_d_n8;
        locals.var_xpwex_dn9 = assign6630_e6899_d_n9;
        locals.var_xpwex_dn10 = assign6630_e6899_d_n10;
        locals.var_xpwex_dn11 = assign6630_e6899_d_n11;
        locals.var_xpwex_rv = 0.0;

        let (assign6640_e6921, assign6640_e6921_d_n0, assign6640_e6921_d_n1, assign6640_e6921_d_n3, assign6640_e6921_d_n4, assign6640_e6921_d_n5, assign6640_e6921_d_n6, assign6640_e6921_d_n7, assign6640_e6921_d_n8, assign6640_e6921_d_n9, assign6640_e6921_d_n10, assign6640_e6921_d_n11,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign6640_e6905: f64 = (0.5 * p.p33);
        let assign6640_e6907: f64 = (assign6640_e6905 * locals.var_taur_t);
        let assign6640_e6910: f64 = (locals.var_qb0 * locals.var_xnbex);
        let assign6640_e6913: f64 = (locals.var_qepi0 * locals.var_xpwex);
        let assign6640_e6914: f64 = (assign6640_e6910 + assign6640_e6913);
        let assign6640_e6915: f64 = (assign6640_e6907 * assign6640_e6914);
        let assign6640_e6918: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign6640_e6919: f64 = (assign6640_e6915 / assign6640_e6918);
        (assign6640_e6919, ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn0) + (locals.var_qepi0 * locals.var_xpwex_dn0))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn1) + (locals.var_qepi0 * locals.var_xpwex_dn1))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn3) + (locals.var_qepi0 * locals.var_xpwex_dn3))) / assign6640_e6918), ((((((assign6640_e6905 * locals.var_taur_t_dn4) * assign6640_e6914) + (assign6640_e6907 * (((locals.var_qb0_dn4 * locals.var_xnbex) + (locals.var_qb0 * locals.var_xnbex_dn4)) + ((locals.var_qepi0_dn4 * locals.var_xpwex) + (locals.var_qepi0 * locals.var_xpwex_dn4))))) * assign6640_e6918) - (assign6640_e6915 * (locals.var_taub_t_dn4 + locals.var_tepi_t_dn4))) / (assign6640_e6918 * assign6640_e6918)), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn5) + (locals.var_qepi0 * locals.var_xpwex_dn5))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn6) + (locals.var_qepi0 * locals.var_xpwex_dn6))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn7) + (locals.var_qepi0 * locals.var_xpwex_dn7))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn8) + (locals.var_qepi0 * locals.var_xpwex_dn8))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn9) + (locals.var_qepi0 * locals.var_xpwex_dn9))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn10) + (locals.var_qepi0 * locals.var_xpwex_dn10))) / assign6640_e6918), ((assign6640_e6907 * ((locals.var_qb0 * locals.var_xnbex_dn11) + (locals.var_qepi0 * locals.var_xpwex_dn11))) / assign6640_e6918),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9, locals.var_xqmex_dn10, locals.var_xqmex_dn11,)
    }
};
        locals.var_xqmex = assign6640_e6921;
        locals.var_xqmex_dn0 = assign6640_e6921_d_n0;
        locals.var_xqmex_dn1 = assign6640_e6921_d_n1;
        locals.var_xqmex_dn3 = assign6640_e6921_d_n3;
        locals.var_xqmex_dn4 = assign6640_e6921_d_n4;
        locals.var_xqmex_dn5 = assign6640_e6921_d_n5;
        locals.var_xqmex_dn6 = assign6640_e6921_d_n6;
        locals.var_xqmex_dn7 = assign6640_e6921_d_n7;
        locals.var_xqmex_dn8 = assign6640_e6921_d_n8;
        locals.var_xqmex_dn9 = assign6640_e6921_d_n9;
        locals.var_xqmex_dn10 = assign6640_e6921_d_n10;
        locals.var_xqmex_dn11 = assign6640_e6921_d_n11;
        locals.var_xqmex_rv = 0.0;

        let assign6650_e6924: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6650_e6926: f64 = (assign6650_e6924 * locals.var_vtinv);
        let assign6650_e6928: f64 = if assign6650_e6926 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign6650_e6928;
        locals.var_guard122_rv = 0.0;

        let (assign6660_e6942, assign6660_e6942_d_n0, assign6660_e6942_d_n1, assign6660_e6942_d_n3, assign6660_e6942_d_n4, assign6660_e6942_d_n5, assign6660_e6942_d_n6, assign6660_e6942_d_n7, assign6660_e6942_d_n8, assign6660_e6942_d_n9, assign6660_e6942_d_n10, assign6660_e6942_d_n11,) = {
    if (((locals.var_guard120 != 0.0) && (locals.var_guard121 == 0.0)) && (locals.var_guard122 != 0.0)) {
        let assign6660_e6937: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6660_e6939: f64 = (assign6660_e6937 * locals.var_vtinv);
        let assign6660_e6940: f64 = (assign6660_e6939).exp();
        (assign6660_e6940, (assign6660_e6940 * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (assign6660_e6940 * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (assign6660_e6940 * ((-locals.var_vdcex_t_dn3) * locals.var_vtinv)), (assign6660_e6940 * (((-locals.var_vdcex_t_dn4) * locals.var_vtinv) + (assign6660_e6937 * locals.var_vtinv_dn4))), (assign6660_e6940 * ((-locals.var_vdcex_t_dn5) * locals.var_vtinv)), (assign6660_e6940 * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (assign6660_e6940 * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (assign6660_e6940 * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (assign6660_e6940 * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)), (assign6660_e6940 * ((locals.var_vbc3_dn10 - locals.var_vdcex_t_dn10) * locals.var_vtinv)), (assign6660_e6940 * ((locals.var_vbc3_dn11 - locals.var_vdcex_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9, locals.var_evbc3vdcex_dn10, locals.var_evbc3vdcex_dn11,)
    }
};
        locals.var_evbc3vdcex = assign6660_e6942;
        locals.var_evbc3vdcex_dn0 = assign6660_e6942_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6660_e6942_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6660_e6942_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6660_e6942_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6660_e6942_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6660_e6942_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6660_e6942_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6660_e6942_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6660_e6942_d_n9;
        locals.var_evbc3vdcex_dn10 = assign6660_e6942_d_n10;
        locals.var_evbc3vdcex_dn11 = assign6660_e6942_d_n11;
        locals.var_evbc3vdcex_rv = 0.0;

        let (assign6670_e6953,) = {
    if (((locals.var_guard120 != 0.0) && (locals.var_guard121 == 0.0)) && (locals.var_guard122 == 0.0)) {
        let assign6670_e6951: f64 = (p.p151).exp();
        (assign6670_e6951,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign6670_e6953;
        locals.var_expl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign6680_e6973, assign6680_e6973_d_n0, assign6680_e6973_d_n1, assign6680_e6973_d_n3, assign6680_e6973_d_n4, assign6680_e6973_d_n5, assign6680_e6973_d_n6, assign6680_e6973_d_n7, assign6680_e6973_d_n8, assign6680_e6973_d_n9, assign6680_e6973_d_n10, assign6680_e6973_d_n11,) = {
    if (((locals.var_guard120 != 0.0) && (locals.var_guard121 == 0.0)) && (locals.var_guard122 == 0.0)) {
        let assign6680_e6965: f64 = (locals.var_vbc3 - locals.var_vdcex_t);
        let assign6680_e6967: f64 = (assign6680_e6965 * locals.var_vtinv);
        let assign6680_e6969: f64 = (assign6680_e6967 - p.p151);
        let assign6680_e6970: f64 = (1.0 + assign6680_e6969);
        let assign6680_e6971: f64 = (locals.var_expl * assign6680_e6970);
        (assign6680_e6971, (locals.var_expl * ((locals.var_vbc3_dn0 - locals.var_vdcex_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn1 - locals.var_vdcex_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdcex_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdcex_t_dn4) * locals.var_vtinv) + (assign6680_e6965 * locals.var_vtinv_dn4))), (locals.var_expl * ((-locals.var_vdcex_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn6 - locals.var_vdcex_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn7 - locals.var_vdcex_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn8 - locals.var_vdcex_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn9 - locals.var_vdcex_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn10 - locals.var_vdcex_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn11 - locals.var_vdcex_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdcex, locals.var_evbc3vdcex_dn0, locals.var_evbc3vdcex_dn1, locals.var_evbc3vdcex_dn3, locals.var_evbc3vdcex_dn4, locals.var_evbc3vdcex_dn5, locals.var_evbc3vdcex_dn6, locals.var_evbc3vdcex_dn7, locals.var_evbc3vdcex_dn8, locals.var_evbc3vdcex_dn9, locals.var_evbc3vdcex_dn10, locals.var_evbc3vdcex_dn11,)
    }
};
        locals.var_evbc3vdcex = assign6680_e6973;
        locals.var_evbc3vdcex_dn0 = assign6680_e6973_d_n0;
        locals.var_evbc3vdcex_dn1 = assign6680_e6973_d_n1;
        locals.var_evbc3vdcex_dn3 = assign6680_e6973_d_n3;
        locals.var_evbc3vdcex_dn4 = assign6680_e6973_d_n4;
        locals.var_evbc3vdcex_dn5 = assign6680_e6973_d_n5;
        locals.var_evbc3vdcex_dn6 = assign6680_e6973_d_n6;
        locals.var_evbc3vdcex_dn7 = assign6680_e6973_d_n7;
        locals.var_evbc3vdcex_dn8 = assign6680_e6973_d_n8;
        locals.var_evbc3vdcex_dn9 = assign6680_e6973_d_n9;
        locals.var_evbc3vdcex_dn10 = assign6680_e6973_d_n10;
        locals.var_evbc3vdcex_dn11 = assign6680_e6973_d_n11;
        locals.var_evbc3vdcex_rv = 0.0;

        let (assign6690_e6997, assign6690_e6997_d_n0, assign6690_e6997_d_n1, assign6690_e6997_d_n3, assign6690_e6997_d_n4, assign6690_e6997_d_n5, assign6690_e6997_d_n6, assign6690_e6997_d_n7, assign6690_e6997_d_n8, assign6690_e6997_d_n9, assign6690_e6997_d_n10, assign6690_e6997_d_n11,) = {
    if ((locals.var_guard120 != 0.0) && (locals.var_guard121 == 0.0)) {
        let assign6690_e6980: f64 = (2.0 * p.p33);
        let assign6690_e6982: f64 = (assign6690_e6980 * locals.var_ibx_t);
        let assign6690_e6984: f64 = (assign6690_e6982 * locals.var_tauex_t);
        let assign6690_e6986: f64 = (assign6690_e6984 * locals.var_evbc3);
        let assign6690_e6991: f64 = (4.0 * locals.var_evbc3vdcex);
        let assign6690_e6992: f64 = (1.0 + assign6690_e6991);
        let assign6690_e6993: f64 = (assign6690_e6992).sqrt();
        let assign6690_e6994: f64 = (1.0 + assign6690_e6993);
        let assign6690_e6995: f64 = (assign6690_e6986 / assign6690_e6994);
        (assign6690_e6995, ((((assign6690_e6984 * locals.var_evbc3_dn0) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn0) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((assign6690_e6984 * locals.var_evbc3_dn1) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn1) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), (-((assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn3) / (2.0 * assign6690_e6993))) / (assign6690_e6994 * assign6690_e6994))), ((((((((assign6690_e6980 * locals.var_ibx_t_dn4) * locals.var_tauex_t) + (assign6690_e6982 * locals.var_tauex_t_dn4)) * locals.var_evbc3) + (assign6690_e6984 * locals.var_evbc3_dn4)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn4) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), (-((assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn5) / (2.0 * assign6690_e6993))) / (assign6690_e6994 * assign6690_e6994))), ((((assign6690_e6984 * locals.var_evbc3_dn6) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn6) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((assign6690_e6984 * locals.var_evbc3_dn7) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn7) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((assign6690_e6984 * locals.var_evbc3_dn8) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn8) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((assign6690_e6984 * locals.var_evbc3_dn9) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn9) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((assign6690_e6984 * locals.var_evbc3_dn10) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn10) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((assign6690_e6984 * locals.var_evbc3_dn11) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * locals.var_evbc3vdcex_dn11) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)),)
    } else {
        (locals.var_xqmex, locals.var_xqmex_dn0, locals.var_xqmex_dn1, locals.var_xqmex_dn3, locals.var_xqmex_dn4, locals.var_xqmex_dn5, locals.var_xqmex_dn6, locals.var_xqmex_dn7, locals.var_xqmex_dn8, locals.var_xqmex_dn9, locals.var_xqmex_dn10, locals.var_xqmex_dn11,)
    }
};
        locals.var_xqmex = assign6690_e6997;
        locals.var_xqmex_dn0 = assign6690_e6997_d_n0;
        locals.var_xqmex_dn1 = assign6690_e6997_d_n1;
        locals.var_xqmex_dn3 = assign6690_e6997_d_n3;
        locals.var_xqmex_dn4 = assign6690_e6997_d_n4;
        locals.var_xqmex_dn5 = assign6690_e6997_d_n5;
        locals.var_xqmex_dn6 = assign6690_e6997_d_n6;
        locals.var_xqmex_dn7 = assign6690_e6997_d_n7;
        locals.var_xqmex_dn8 = assign6690_e6997_d_n8;
        locals.var_xqmex_dn9 = assign6690_e6997_d_n9;
        locals.var_xqmex_dn10 = assign6690_e6997_d_n10;
        locals.var_xqmex_dn11 = assign6690_e6997_d_n11;
        locals.var_xqmex_rv = 0.0;

        let (assign6700_e7003, assign6700_e7003_d_n0, assign6700_e7003_d_n1, assign6700_e7003_d_n3, assign6700_e7003_d_n4, assign6700_e7003_d_n5, assign6700_e7003_d_n6, assign6700_e7003_d_n7, assign6700_e7003_d_n8, assign6700_e7003_d_n9, assign6700_e7003_d_n10, assign6700_e7003_d_n11,) = {
    if (locals.var_guard120 != 0.0) {
        let assign6700_e7001: f64 = (locals.var_fex * locals.var_xqmex);
        (assign6700_e7001, ((locals.var_fex_dn0 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn0)), ((locals.var_fex_dn1 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn1)), ((locals.var_fex_dn3 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn3)), ((locals.var_fex_dn4 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn4)), ((locals.var_fex_dn5 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn5)), ((locals.var_fex_dn6 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn6)), ((locals.var_fex_dn7 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn7)), ((locals.var_fex_dn8 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn8)), ((locals.var_fex_dn9 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn9)), ((locals.var_fex_dn10 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn10)), ((locals.var_fex_dn11 * locals.var_xqmex) + (locals.var_fex * locals.var_xqmex_dn11)),)
    } else {
        (locals.var_xqex, locals.var_xqex_dn0, locals.var_xqex_dn1, locals.var_xqex_dn3, locals.var_xqex_dn4, locals.var_xqex_dn5, locals.var_xqex_dn6, locals.var_xqex_dn7, locals.var_xqex_dn8, locals.var_xqex_dn9, locals.var_xqex_dn10, locals.var_xqex_dn11,)
    }
};
        locals.var_xqex = assign6700_e7003;
        locals.var_xqex_dn0 = assign6700_e7003_d_n0;
        locals.var_xqex_dn1 = assign6700_e7003_d_n1;
        locals.var_xqex_dn3 = assign6700_e7003_d_n3;
        locals.var_xqex_dn4 = assign6700_e7003_d_n4;
        locals.var_xqex_dn5 = assign6700_e7003_d_n5;
        locals.var_xqex_dn6 = assign6700_e7003_d_n6;
        locals.var_xqex_dn7 = assign6700_e7003_d_n7;
        locals.var_xqex_dn8 = assign6700_e7003_d_n8;
        locals.var_xqex_dn9 = assign6700_e7003_d_n9;
        locals.var_xqex_dn10 = assign6700_e7003_d_n10;
        locals.var_xqex_dn11 = assign6700_e7003_d_n11;
        locals.var_xqex_rv = 0.0;

        let assign6710_e7006: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign6710_e7006;
        locals.var_guard123_rv = 0.0;

        let (assign6720_e7019, assign6720_e7019_d_n0, assign6720_e7019_d_n1, assign6720_e7019_d_n3, assign6720_e7019_d_n4, assign6720_e7019_d_n5, assign6720_e7019_d_n6, assign6720_e7019_d_n7, assign6720_e7019_d_n8, assign6720_e7019_d_n9, assign6720_e7019_d_n10, assign6720_e7019_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6720_e7011: f64 = (locals.var_vje * locals.var_inv_vde_t);
        let assign6720_e7012: f64 = (1.0 - assign6720_e7011);
        let assign6720_e7014: f64 = (-p.p67);
        let assign6720_e7015: f64 = (assign6720_e7012).powf(assign6720_e7014);
        let assign6720_e7017: f64 = (assign6720_e7015 - 3.0);
        (assign6720_e7017, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((locals.var_vje_dn11 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn11))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((locals.var_vje_dn11 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn11))) / assign6720_e7012))) },)
    } else {
        (locals.var_dvtevje, locals.var_dvtevje_dn0, locals.var_dvtevje_dn1, locals.var_dvtevje_dn3, locals.var_dvtevje_dn4, locals.var_dvtevje_dn5, locals.var_dvtevje_dn6, locals.var_dvtevje_dn7, locals.var_dvtevje_dn8, locals.var_dvtevje_dn9, locals.var_dvtevje_dn10, locals.var_dvtevje_dn11,)
    }
};
        locals.var_dvtevje = assign6720_e7019;
        locals.var_dvtevje_dn0 = assign6720_e7019_d_n0;
        locals.var_dvtevje_dn1 = assign6720_e7019_d_n1;
        locals.var_dvtevje_dn3 = assign6720_e7019_d_n3;
        locals.var_dvtevje_dn4 = assign6720_e7019_d_n4;
        locals.var_dvtevje_dn5 = assign6720_e7019_d_n5;
        locals.var_dvtevje_dn6 = assign6720_e7019_d_n6;
        locals.var_dvtevje_dn7 = assign6720_e7019_d_n7;
        locals.var_dvtevje_dn8 = assign6720_e7019_d_n8;
        locals.var_dvtevje_dn9 = assign6720_e7019_d_n9;
        locals.var_dvtevje_dn10 = assign6720_e7019_d_n10;
        locals.var_dvtevje_dn11 = assign6720_e7019_d_n11;
        locals.var_dvtevje_rv = 0.0;

        let (assign6730_e7027, assign6730_e7027_d_n0, assign6730_e7027_d_n1, assign6730_e7027_d_n3, assign6730_e7027_d_n4, assign6730_e7027_d_n5, assign6730_e7027_d_n6, assign6730_e7027_d_n7, assign6730_e7027_d_n8, assign6730_e7027_d_n9, assign6730_e7027_d_n10, assign6730_e7027_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6730_e7023: f64 = (locals.var_vb2e1 - locals.var_vfe);
        let assign6730_e7025: f64 = (assign6730_e7023 / locals.var_a_vde);
        (assign6730_e7025, ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn4) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn5 - locals.var_vfe_dn5) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn6) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde)), ((((locals.var_vb2e1_dn7 - locals.var_vfe_dn7) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde)), ((((-locals.var_vfe_dn11) * locals.var_a_vde) - (assign6730_e7023 * locals.var_a_vde_dn11)) / (locals.var_a_vde * locals.var_a_vde)),)
    } else {
        (locals.var_vb2e1vfe, locals.var_vb2e1vfe_dn0, locals.var_vb2e1vfe_dn1, locals.var_vb2e1vfe_dn3, locals.var_vb2e1vfe_dn4, locals.var_vb2e1vfe_dn5, locals.var_vb2e1vfe_dn6, locals.var_vb2e1vfe_dn7, locals.var_vb2e1vfe_dn8, locals.var_vb2e1vfe_dn9, locals.var_vb2e1vfe_dn10, locals.var_vb2e1vfe_dn11,)
    }
};
        locals.var_vb2e1vfe = assign6730_e7027;
        locals.var_vb2e1vfe_dn0 = assign6730_e7027_d_n0;
        locals.var_vb2e1vfe_dn1 = assign6730_e7027_d_n1;
        locals.var_vb2e1vfe_dn3 = assign6730_e7027_d_n3;
        locals.var_vb2e1vfe_dn4 = assign6730_e7027_d_n4;
        locals.var_vb2e1vfe_dn5 = assign6730_e7027_d_n5;
        locals.var_vb2e1vfe_dn6 = assign6730_e7027_d_n6;
        locals.var_vb2e1vfe_dn7 = assign6730_e7027_d_n7;
        locals.var_vb2e1vfe_dn8 = assign6730_e7027_d_n8;
        locals.var_vb2e1vfe_dn9 = assign6730_e7027_d_n9;
        locals.var_vb2e1vfe_dn10 = assign6730_e7027_d_n10;
        locals.var_vb2e1vfe_dn11 = assign6730_e7027_d_n11;
        locals.var_vb2e1vfe_rv = 0.0;

        let assign6740_e7030: f64 = if locals.var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign6740_e7030;
        locals.var_guard124_rv = 0.0;

        let (assign6750_e7041, assign6750_e7041_d_n0, assign6750_e7041_d_n1, assign6750_e7041_d_n3, assign6750_e7041_d_n4, assign6750_e7041_d_n5, assign6750_e7041_d_n6, assign6750_e7041_d_n7, assign6750_e7041_d_n8, assign6750_e7041_d_n9, assign6750_e7041_d_n10, assign6750_e7041_d_n11,) = {
    if ((locals.var_guard123 != 0.0) && (locals.var_guard124 != 0.0)) {
        let assign6750_e7037: f64 = (locals.var_vb2e1vfe).exp();
        let assign6750_e7038: f64 = (1.0 + assign6750_e7037);
        let assign6750_e7039: f64 = (1.0 / assign6750_e7038);
        (assign6750_e7039, (-((assign6750_e7037 * locals.var_vb2e1vfe_dn0) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn1) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn3) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn4) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn5) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn6) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn7) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn8) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn9) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn10) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * locals.var_vb2e1vfe_dn11) / (assign6750_e7038 * assign6750_e7038))),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9, locals.var_dvjevb2e1_dn10, locals.var_dvjevb2e1_dn11,)
    }
};
        locals.var_dvjevb2e1 = assign6750_e7041;
        locals.var_dvjevb2e1_dn0 = assign6750_e7041_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6750_e7041_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6750_e7041_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6750_e7041_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6750_e7041_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6750_e7041_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6750_e7041_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6750_e7041_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6750_e7041_d_n9;
        locals.var_dvjevb2e1_dn10 = assign6750_e7041_d_n10;
        locals.var_dvjevb2e1_dn11 = assign6750_e7041_d_n11;
        locals.var_dvjevb2e1_rv = 0.0;

        let (assign6760_e7056, assign6760_e7056_d_n0, assign6760_e7056_d_n1, assign6760_e7056_d_n3, assign6760_e7056_d_n4, assign6760_e7056_d_n5, assign6760_e7056_d_n6, assign6760_e7056_d_n7, assign6760_e7056_d_n8, assign6760_e7056_d_n9, assign6760_e7056_d_n10, assign6760_e7056_d_n11,) = {
    if ((locals.var_guard123 != 0.0) && (locals.var_guard124 == 0.0)) {
        let assign6760_e7047: f64 = (-locals.var_vb2e1vfe);
        let assign6760_e7048: f64 = (assign6760_e7047).exp();
        let assign6760_e7051: f64 = (-locals.var_vb2e1vfe);
        let assign6760_e7052: f64 = (assign6760_e7051).exp();
        let assign6760_e7053: f64 = (1.0 + assign6760_e7052);
        let assign6760_e7054: f64 = (assign6760_e7048 / assign6760_e7053);
        (assign6760_e7054, ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn0)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn0)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn1)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn1)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn3)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn3)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn4)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn4)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn5)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn5)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn6)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn6)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn7)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn7)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn8)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn8)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn9)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn9)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn10)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn10)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-locals.var_vb2e1vfe_dn11)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-locals.var_vb2e1vfe_dn11)))) / (assign6760_e7053 * assign6760_e7053)),)
    } else {
        (locals.var_dvjevb2e1, locals.var_dvjevb2e1_dn0, locals.var_dvjevb2e1_dn1, locals.var_dvjevb2e1_dn3, locals.var_dvjevb2e1_dn4, locals.var_dvjevb2e1_dn5, locals.var_dvjevb2e1_dn6, locals.var_dvjevb2e1_dn7, locals.var_dvjevb2e1_dn8, locals.var_dvjevb2e1_dn9, locals.var_dvjevb2e1_dn10, locals.var_dvjevb2e1_dn11,)
    }
};
        locals.var_dvjevb2e1 = assign6760_e7056;
        locals.var_dvjevb2e1_dn0 = assign6760_e7056_d_n0;
        locals.var_dvjevb2e1_dn1 = assign6760_e7056_d_n1;
        locals.var_dvjevb2e1_dn3 = assign6760_e7056_d_n3;
        locals.var_dvjevb2e1_dn4 = assign6760_e7056_d_n4;
        locals.var_dvjevb2e1_dn5 = assign6760_e7056_d_n5;
        locals.var_dvjevb2e1_dn6 = assign6760_e7056_d_n6;
        locals.var_dvjevb2e1_dn7 = assign6760_e7056_d_n7;
        locals.var_dvjevb2e1_dn8 = assign6760_e7056_d_n8;
        locals.var_dvjevb2e1_dn9 = assign6760_e7056_d_n9;
        locals.var_dvjevb2e1_dn10 = assign6760_e7056_d_n10;
        locals.var_dvjevb2e1_dn11 = assign6760_e7056_d_n11;
        locals.var_dvjevb2e1_rv = 0.0;

        let (assign6770_e7064, assign6770_e7064_d_n0, assign6770_e7064_d_n1, assign6770_e7064_d_n3, assign6770_e7064_d_n4, assign6770_e7064_d_n5, assign6770_e7064_d_n6, assign6770_e7064_d_n7, assign6770_e7064_d_n8, assign6770_e7064_d_n9, assign6770_e7064_d_n10, assign6770_e7064_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6770_e7060: f64 = (locals.var_dvtevje * locals.var_dvjevb2e1);
        let assign6770_e7062: f64 = (assign6770_e7060 + 3.0);
        (assign6770_e7062, ((locals.var_dvtevje_dn0 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn0)), ((locals.var_dvtevje_dn1 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn1)), ((locals.var_dvtevje_dn3 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn3)), ((locals.var_dvtevje_dn4 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn4)), ((locals.var_dvtevje_dn5 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn5)), ((locals.var_dvtevje_dn6 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn6)), ((locals.var_dvtevje_dn7 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn7)), ((locals.var_dvtevje_dn8 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn8)), ((locals.var_dvtevje_dn9 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn9)), ((locals.var_dvtevje_dn10 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn10)), ((locals.var_dvtevje_dn11 * locals.var_dvjevb2e1) + (locals.var_dvtevje * locals.var_dvjevb2e1_dn11)),)
    } else {
        (locals.var_dvtevb2e1, locals.var_dvtevb2e1_dn0, locals.var_dvtevb2e1_dn1, locals.var_dvtevb2e1_dn3, locals.var_dvtevb2e1_dn4, locals.var_dvtevb2e1_dn5, locals.var_dvtevb2e1_dn6, locals.var_dvtevb2e1_dn7, locals.var_dvtevb2e1_dn8, locals.var_dvtevb2e1_dn9, locals.var_dvtevb2e1_dn10, locals.var_dvtevb2e1_dn11,)
    }
};
        locals.var_dvtevb2e1 = assign6770_e7064;
        locals.var_dvtevb2e1_dn0 = assign6770_e7064_d_n0;
        locals.var_dvtevb2e1_dn1 = assign6770_e7064_d_n1;
        locals.var_dvtevb2e1_dn3 = assign6770_e7064_d_n3;
        locals.var_dvtevb2e1_dn4 = assign6770_e7064_d_n4;
        locals.var_dvtevb2e1_dn5 = assign6770_e7064_d_n5;
        locals.var_dvtevb2e1_dn6 = assign6770_e7064_d_n6;
        locals.var_dvtevb2e1_dn7 = assign6770_e7064_d_n7;
        locals.var_dvtevb2e1_dn8 = assign6770_e7064_d_n8;
        locals.var_dvtevb2e1_dn9 = assign6770_e7064_d_n9;
        locals.var_dvtevb2e1_dn10 = assign6770_e7064_d_n10;
        locals.var_dvtevb2e1_dn11 = assign6770_e7064_d_n11;
        locals.var_dvtevb2e1_rv = 0.0;

        let (assign6780_e7074, assign6780_e7074_d_n0, assign6780_e7074_d_n1, assign6780_e7074_d_n3, assign6780_e7074_d_n4, assign6780_e7074_d_n5, assign6780_e7074_d_n6, assign6780_e7074_d_n7, assign6780_e7074_d_n8, assign6780_e7074_d_n9, assign6780_e7074_d_n10, assign6780_e7074_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6780_e7068: f64 = (1.0 - p.p68);
        let assign6780_e7070: f64 = (assign6780_e7068 * locals.var_cje_t);
        let assign6780_e7072: f64 = (assign6780_e7070 * locals.var_dvtevb2e1);
        (assign6780_e7072, (((assign6780_e7068 * locals.var_cje_t_dn0) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn0)), (((assign6780_e7068 * locals.var_cje_t_dn1) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn1)), (((assign6780_e7068 * locals.var_cje_t_dn3) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn3)), (((assign6780_e7068 * locals.var_cje_t_dn4) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn4)), (((assign6780_e7068 * locals.var_cje_t_dn5) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn5)), (((assign6780_e7068 * locals.var_cje_t_dn6) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn6)), (((assign6780_e7068 * locals.var_cje_t_dn7) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn7)), (((assign6780_e7068 * locals.var_cje_t_dn8) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn8)), (((assign6780_e7068 * locals.var_cje_t_dn9) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn9)), (((assign6780_e7068 * locals.var_cje_t_dn10) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn10)), (((assign6780_e7068 * locals.var_cje_t_dn11) * locals.var_dvtevb2e1) + (assign6780_e7070 * locals.var_dvtevb2e1_dn11)),)
    } else {
        (locals.var_dqtevb2e1, locals.var_dqtevb2e1_dn0, locals.var_dqtevb2e1_dn1, locals.var_dqtevb2e1_dn3, locals.var_dqtevb2e1_dn4, locals.var_dqtevb2e1_dn5, locals.var_dqtevb2e1_dn6, locals.var_dqtevb2e1_dn7, locals.var_dqtevb2e1_dn8, locals.var_dqtevb2e1_dn9, locals.var_dqtevb2e1_dn10, locals.var_dqtevb2e1_dn11,)
    }
};
        locals.var_dqtevb2e1 = assign6780_e7074;
        locals.var_dqtevb2e1_dn0 = assign6780_e7074_d_n0;
        locals.var_dqtevb2e1_dn1 = assign6780_e7074_d_n1;
        locals.var_dqtevb2e1_dn3 = assign6780_e7074_d_n3;
        locals.var_dqtevb2e1_dn4 = assign6780_e7074_d_n4;
        locals.var_dqtevb2e1_dn5 = assign6780_e7074_d_n5;
        locals.var_dqtevb2e1_dn6 = assign6780_e7074_d_n6;
        locals.var_dqtevb2e1_dn7 = assign6780_e7074_d_n7;
        locals.var_dqtevb2e1_dn8 = assign6780_e7074_d_n8;
        locals.var_dqtevb2e1_dn9 = assign6780_e7074_d_n9;
        locals.var_dqtevb2e1_dn10 = assign6780_e7074_d_n10;
        locals.var_dqtevb2e1_dn11 = assign6780_e7074_d_n11;
        locals.var_dqtevb2e1_rv = 0.0;

        let (assign6790_e7091, assign6790_e7091_d_n0, assign6790_e7091_d_n1, assign6790_e7091_d_n3, assign6790_e7091_d_n4, assign6790_e7091_d_n5, assign6790_e7091_d_n6, assign6790_e7091_d_n7, assign6790_e7091_d_n8, assign6790_e7091_d_n9, assign6790_e7091_d_n10, assign6790_e7091_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6790_e7078: f64 = (locals.var_if0 * locals.var_evb2e1);
        let assign6790_e7080: f64 = (assign6790_e7078 * locals.var_vtinv);
        let assign6790_e7082: f64 = (assign6790_e7080 / locals.var_nff_t);
        let assign6790_e7086: f64 = (1.0 + locals.var_f1);
        let assign6790_e7087: f64 = (assign6790_e7086).sqrt();
        let assign6790_e7088: f64 = (0.5 / assign6790_e7087);
        let assign6790_e7089: f64 = (assign6790_e7082 * assign6790_e7088);
        (assign6790_e7089, ((((((((locals.var_if0_dn0 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn0)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn0)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn0 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn1 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn1)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn1)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn1 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn3 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn3)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn3)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn3 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((locals.var_if0_dn4 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn4)) * locals.var_vtinv) + (assign6790_e7078 * locals.var_vtinv_dn4)) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn4 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn5 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn5)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn5 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn6 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn6)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn6)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn6 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn7 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn7)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn7)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn7 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn8 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn8)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn8)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn8 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn9 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn9)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn9)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn9 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn10 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn10)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn10)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn10 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), ((((((((locals.var_if0_dn11 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn11)) * locals.var_vtinv) * locals.var_nff_t) - (assign6790_e7080 * locals.var_nff_t_dn11)) / (locals.var_nff_t * locals.var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (locals.var_f1_dn11 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))),)
    } else {
        (locals.var_dn0vb2e1, locals.var_dn0vb2e1_dn0, locals.var_dn0vb2e1_dn1, locals.var_dn0vb2e1_dn3, locals.var_dn0vb2e1_dn4, locals.var_dn0vb2e1_dn5, locals.var_dn0vb2e1_dn6, locals.var_dn0vb2e1_dn7, locals.var_dn0vb2e1_dn8, locals.var_dn0vb2e1_dn9, locals.var_dn0vb2e1_dn10, locals.var_dn0vb2e1_dn11,)
    }
};
        locals.var_dn0vb2e1 = assign6790_e7091;
        locals.var_dn0vb2e1_dn0 = assign6790_e7091_d_n0;
        locals.var_dn0vb2e1_dn1 = assign6790_e7091_d_n1;
        locals.var_dn0vb2e1_dn3 = assign6790_e7091_d_n3;
        locals.var_dn0vb2e1_dn4 = assign6790_e7091_d_n4;
        locals.var_dn0vb2e1_dn5 = assign6790_e7091_d_n5;
        locals.var_dn0vb2e1_dn6 = assign6790_e7091_d_n6;
        locals.var_dn0vb2e1_dn7 = assign6790_e7091_d_n7;
        locals.var_dn0vb2e1_dn8 = assign6790_e7091_d_n8;
        locals.var_dn0vb2e1_dn9 = assign6790_e7091_d_n9;
        locals.var_dn0vb2e1_dn10 = assign6790_e7091_d_n10;
        locals.var_dn0vb2e1_dn11 = assign6790_e7091_d_n11;
        locals.var_dn0vb2e1_rv = 0.0;

        let (assign6800_e7101, assign6800_e7101_d_n0, assign6800_e7101_d_n1, assign6800_e7101_d_n3, assign6800_e7101_d_n4, assign6800_e7101_d_n5, assign6800_e7101_d_n6, assign6800_e7101_d_n7, assign6800_e7101_d_n8, assign6800_e7101_d_n9, assign6800_e7101_d_n10, assign6800_e7101_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6800_e7095: f64 = (0.5 * locals.var_qb0);
        let assign6800_e7097: f64 = (assign6800_e7095 * locals.var_q1q);
        let assign6800_e7099: f64 = (assign6800_e7097 * locals.var_dn0vb2e1);
        (assign6800_e7099, (((assign6800_e7095 * locals.var_q1q_dn0) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn0)), (((assign6800_e7095 * locals.var_q1q_dn1) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn1)), (((assign6800_e7095 * locals.var_q1q_dn3) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn3)), (((((0.5 * locals.var_qb0_dn4) * locals.var_q1q) + (assign6800_e7095 * locals.var_q1q_dn4)) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn4)), (((assign6800_e7095 * locals.var_q1q_dn5) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn5)), (((assign6800_e7095 * locals.var_q1q_dn6) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn6)), (((assign6800_e7095 * locals.var_q1q_dn7) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn7)), (((assign6800_e7095 * locals.var_q1q_dn8) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn8)), (((assign6800_e7095 * locals.var_q1q_dn9) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn9)), (((assign6800_e7095 * locals.var_q1q_dn10) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn10)), (((assign6800_e7095 * locals.var_q1q_dn11) * locals.var_dn0vb2e1) + (assign6800_e7097 * locals.var_dn0vb2e1_dn11)),)
    } else {
        (locals.var_dqbevb2e1, locals.var_dqbevb2e1_dn0, locals.var_dqbevb2e1_dn1, locals.var_dqbevb2e1_dn3, locals.var_dqbevb2e1_dn4, locals.var_dqbevb2e1_dn5, locals.var_dqbevb2e1_dn6, locals.var_dqbevb2e1_dn7, locals.var_dqbevb2e1_dn8, locals.var_dqbevb2e1_dn9, locals.var_dqbevb2e1_dn10, locals.var_dqbevb2e1_dn11,)
    }
};
        locals.var_dqbevb2e1 = assign6800_e7101;
        locals.var_dqbevb2e1_dn0 = assign6800_e7101_d_n0;
        locals.var_dqbevb2e1_dn1 = assign6800_e7101_d_n1;
        locals.var_dqbevb2e1_dn3 = assign6800_e7101_d_n3;
        locals.var_dqbevb2e1_dn4 = assign6800_e7101_d_n4;
        locals.var_dqbevb2e1_dn5 = assign6800_e7101_d_n5;
        locals.var_dqbevb2e1_dn6 = assign6800_e7101_d_n6;
        locals.var_dqbevb2e1_dn7 = assign6800_e7101_d_n7;
        locals.var_dqbevb2e1_dn8 = assign6800_e7101_d_n8;
        locals.var_dqbevb2e1_dn9 = assign6800_e7101_d_n9;
        locals.var_dqbevb2e1_dn10 = assign6800_e7101_d_n10;
        locals.var_dqbevb2e1_dn11 = assign6800_e7101_d_n11;
        locals.var_dqbevb2e1_rv = 0.0;

        let (assign6810_e7109, assign6810_e7109_d_n0, assign6810_e7109_d_n1, assign6810_e7109_d_n3, assign6810_e7109_d_n4, assign6810_e7109_d_n5, assign6810_e7109_d_n6, assign6810_e7109_d_n7, assign6810_e7109_d_n8, assign6810_e7109_d_n9, assign6810_e7109_d_n10, assign6810_e7109_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6810_e7106: f64 = (p.p85 * locals.var_vt);
        let assign6810_e7107: f64 = (locals.var_qe_qs / assign6810_e7106);
        (assign6810_e7107, (locals.var_qe_qs_dn0 / assign6810_e7106), (locals.var_qe_qs_dn1 / assign6810_e7106), (locals.var_qe_qs_dn3 / assign6810_e7106), (((locals.var_qe_qs_dn4 * assign6810_e7106) - (locals.var_qe_qs * (p.p85 * locals.var_vt_dn4))) / (assign6810_e7106 * assign6810_e7106)), (locals.var_qe_qs_dn5 / assign6810_e7106), (locals.var_qe_qs_dn6 / assign6810_e7106), (locals.var_qe_qs_dn7 / assign6810_e7106), (locals.var_qe_qs_dn8 / assign6810_e7106), (locals.var_qe_qs_dn9 / assign6810_e7106), (locals.var_qe_qs_dn10 / assign6810_e7106), (locals.var_qe_qs_dn11 / assign6810_e7106),)
    } else {
        (locals.var_dqevb2e1, locals.var_dqevb2e1_dn0, locals.var_dqevb2e1_dn1, locals.var_dqevb2e1_dn3, locals.var_dqevb2e1_dn4, locals.var_dqevb2e1_dn5, locals.var_dqevb2e1_dn6, locals.var_dqevb2e1_dn7, locals.var_dqevb2e1_dn8, locals.var_dqevb2e1_dn9, locals.var_dqevb2e1_dn10, locals.var_dqevb2e1_dn11,)
    }
};
        locals.var_dqevb2e1 = assign6810_e7109;
        locals.var_dqevb2e1_dn0 = assign6810_e7109_d_n0;
        locals.var_dqevb2e1_dn1 = assign6810_e7109_d_n1;
        locals.var_dqevb2e1_dn3 = assign6810_e7109_d_n3;
        locals.var_dqevb2e1_dn4 = assign6810_e7109_d_n4;
        locals.var_dqevb2e1_dn5 = assign6810_e7109_d_n5;
        locals.var_dqevb2e1_dn6 = assign6810_e7109_d_n6;
        locals.var_dqevb2e1_dn7 = assign6810_e7109_d_n7;
        locals.var_dqevb2e1_dn8 = assign6810_e7109_d_n8;
        locals.var_dqevb2e1_dn9 = assign6810_e7109_d_n9;
        locals.var_dqevb2e1_dn10 = assign6810_e7109_d_n10;
        locals.var_dqevb2e1_dn11 = assign6810_e7109_d_n11;
        locals.var_dqevb2e1_rv = 0.0;

        let (assign6820_e7121, assign6820_e7121_d_n0, assign6820_e7121_d_n1, assign6820_e7121_d_n3, assign6820_e7121_d_n4, assign6820_e7121_d_n5, assign6820_e7121_d_n6, assign6820_e7121_d_n7, assign6820_e7121_d_n8, assign6820_e7121_d_n9, assign6820_e7121_d_n10, assign6820_e7121_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6820_e7113: f64 = (0.2 * locals.var_vb1b2);
        let assign6820_e7116: f64 = (locals.var_dqtevb2e1 + locals.var_dqbevb2e1);
        let assign6820_e7118: f64 = (assign6820_e7116 + locals.var_dqevb2e1);
        let assign6820_e7119: f64 = (assign6820_e7113 * assign6820_e7118);
        (assign6820_e7119, (assign6820_e7113 * ((locals.var_dqtevb2e1_dn0 + locals.var_dqbevb2e1_dn0) + locals.var_dqevb2e1_dn0)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn1 + locals.var_dqbevb2e1_dn1) + locals.var_dqevb2e1_dn1)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn3 + locals.var_dqbevb2e1_dn3) + locals.var_dqevb2e1_dn3)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn4 + locals.var_dqbevb2e1_dn4) + locals.var_dqevb2e1_dn4)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn5 + locals.var_dqbevb2e1_dn5) + locals.var_dqevb2e1_dn5)), (((0.2 * locals.var_vb1b2_dn6) * assign6820_e7118) + (assign6820_e7113 * ((locals.var_dqtevb2e1_dn6 + locals.var_dqbevb2e1_dn6) + locals.var_dqevb2e1_dn6))), (((0.2 * locals.var_vb1b2_dn7) * assign6820_e7118) + (assign6820_e7113 * ((locals.var_dqtevb2e1_dn7 + locals.var_dqbevb2e1_dn7) + locals.var_dqevb2e1_dn7))), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn8 + locals.var_dqbevb2e1_dn8) + locals.var_dqevb2e1_dn8)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn9 + locals.var_dqbevb2e1_dn9) + locals.var_dqevb2e1_dn9)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn10 + locals.var_dqbevb2e1_dn10) + locals.var_dqevb2e1_dn10)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn11 + locals.var_dqbevb2e1_dn11) + locals.var_dqevb2e1_dn11)),)
    } else {
        (locals.var_qb1b2, locals.var_qb1b2_dn0, locals.var_qb1b2_dn1, locals.var_qb1b2_dn3, locals.var_qb1b2_dn4, locals.var_qb1b2_dn5, locals.var_qb1b2_dn6, locals.var_qb1b2_dn7, locals.var_qb1b2_dn8, locals.var_qb1b2_dn9, locals.var_qb1b2_dn10, locals.var_qb1b2_dn11,)
    }
};
        locals.var_qb1b2 = assign6820_e7121;
        locals.var_qb1b2_dn0 = assign6820_e7121_d_n0;
        locals.var_qb1b2_dn1 = assign6820_e7121_d_n1;
        locals.var_qb1b2_dn3 = assign6820_e7121_d_n3;
        locals.var_qb1b2_dn4 = assign6820_e7121_d_n4;
        locals.var_qb1b2_dn5 = assign6820_e7121_d_n5;
        locals.var_qb1b2_dn6 = assign6820_e7121_d_n6;
        locals.var_qb1b2_dn7 = assign6820_e7121_d_n7;
        locals.var_qb1b2_dn8 = assign6820_e7121_d_n8;
        locals.var_qb1b2_dn9 = assign6820_e7121_d_n9;
        locals.var_qb1b2_dn10 = assign6820_e7121_d_n10;
        locals.var_qb1b2_dn11 = assign6820_e7121_d_n11;
        locals.var_qb1b2_rv = 0.0;

        let (assign6830_e7129, assign6830_e7129_d_n0, assign6830_e7129_d_n1, assign6830_e7129_d_n3, assign6830_e7129_d_n4, assign6830_e7129_d_n5, assign6830_e7129_d_n6, assign6830_e7129_d_n7, assign6830_e7129_d_n8, assign6830_e7129_d_n9, assign6830_e7129_d_n10, assign6830_e7129_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6830_e7125: f64 = (1.0 - p.p95);
        let assign6830_e7127: f64 = (assign6830_e7125 * locals.var_qe_qs);
        (assign6830_e7127, (assign6830_e7125 * locals.var_qe_qs_dn0), (assign6830_e7125 * locals.var_qe_qs_dn1), (assign6830_e7125 * locals.var_qe_qs_dn3), (assign6830_e7125 * locals.var_qe_qs_dn4), (assign6830_e7125 * locals.var_qe_qs_dn5), (assign6830_e7125 * locals.var_qe_qs_dn6), (assign6830_e7125 * locals.var_qe_qs_dn7), (assign6830_e7125 * locals.var_qe_qs_dn8), (assign6830_e7125 * locals.var_qe_qs_dn9), (assign6830_e7125 * locals.var_qe_qs_dn10), (assign6830_e7125 * locals.var_qe_qs_dn11),)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10, locals.var_qe_dn11,)
    }
};
        locals.var_qe = assign6830_e7129;
        locals.var_qe_dn0 = assign6830_e7129_d_n0;
        locals.var_qe_dn1 = assign6830_e7129_d_n1;
        locals.var_qe_dn3 = assign6830_e7129_d_n3;
        locals.var_qe_dn4 = assign6830_e7129_d_n4;
        locals.var_qe_dn5 = assign6830_e7129_d_n5;
        locals.var_qe_dn6 = assign6830_e7129_d_n6;
        locals.var_qe_dn7 = assign6830_e7129_d_n7;
        locals.var_qe_dn8 = assign6830_e7129_d_n8;
        locals.var_qe_dn9 = assign6830_e7129_d_n9;
        locals.var_qe_dn10 = assign6830_e7129_d_n10;
        locals.var_qe_dn11 = assign6830_e7129_d_n11;
        locals.var_qe_rv = 0.0;

        let (assign6840_e7137, assign6840_e7137_d_n0, assign6840_e7137_d_n1, assign6840_e7137_d_n3, assign6840_e7137_d_n4, assign6840_e7137_d_n5, assign6840_e7137_d_n6, assign6840_e7137_d_n7, assign6840_e7137_d_n8, assign6840_e7137_d_n9, assign6840_e7137_d_n10, assign6840_e7137_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6840_e7134: f64 = (p.p95 * locals.var_qe_qs);
        let assign6840_e7135: f64 = (locals.var_qbe_qs + assign6840_e7134);
        (assign6840_e7135, (locals.var_qbe_qs_dn0 + (p.p95 * locals.var_qe_qs_dn0)), (locals.var_qbe_qs_dn1 + (p.p95 * locals.var_qe_qs_dn1)), (locals.var_qbe_qs_dn3 + (p.p95 * locals.var_qe_qs_dn3)), (locals.var_qbe_qs_dn4 + (p.p95 * locals.var_qe_qs_dn4)), (locals.var_qbe_qs_dn5 + (p.p95 * locals.var_qe_qs_dn5)), (locals.var_qbe_qs_dn6 + (p.p95 * locals.var_qe_qs_dn6)), (locals.var_qbe_qs_dn7 + (p.p95 * locals.var_qe_qs_dn7)), (locals.var_qbe_qs_dn8 + (p.p95 * locals.var_qe_qs_dn8)), (locals.var_qbe_qs_dn9 + (p.p95 * locals.var_qe_qs_dn9)), (locals.var_qbe_qs_dn10 + (p.p95 * locals.var_qe_qs_dn10)), (locals.var_qbe_qs_dn11 + (p.p95 * locals.var_qe_qs_dn11)),)
    } else {
        (locals.var_qbe_qs_eff, locals.var_qbe_qs_eff_dn0, locals.var_qbe_qs_eff_dn1, locals.var_qbe_qs_eff_dn3, locals.var_qbe_qs_eff_dn4, locals.var_qbe_qs_eff_dn5, locals.var_qbe_qs_eff_dn6, locals.var_qbe_qs_eff_dn7, locals.var_qbe_qs_eff_dn8, locals.var_qbe_qs_eff_dn9, locals.var_qbe_qs_eff_dn10, locals.var_qbe_qs_eff_dn11,)
    }
};
        locals.var_qbe_qs_eff = assign6840_e7137;
        locals.var_qbe_qs_eff_dn0 = assign6840_e7137_d_n0;
        locals.var_qbe_qs_eff_dn1 = assign6840_e7137_d_n1;
        locals.var_qbe_qs_eff_dn3 = assign6840_e7137_d_n3;
        locals.var_qbe_qs_eff_dn4 = assign6840_e7137_d_n4;
        locals.var_qbe_qs_eff_dn5 = assign6840_e7137_d_n5;
        locals.var_qbe_qs_eff_dn6 = assign6840_e7137_d_n6;
        locals.var_qbe_qs_eff_dn7 = assign6840_e7137_d_n7;
        locals.var_qbe_qs_eff_dn8 = assign6840_e7137_d_n8;
        locals.var_qbe_qs_eff_dn9 = assign6840_e7137_d_n9;
        locals.var_qbe_qs_eff_dn10 = assign6840_e7137_d_n10;
        locals.var_qbe_qs_eff_dn11 = assign6840_e7137_d_n11;
        locals.var_qbe_qs_eff_rv = 0.0;

        let (assign6850_e7145, assign6850_e7145_d_n0, assign6850_e7145_d_n1, assign6850_e7145_d_n3, assign6850_e7145_d_n4, assign6850_e7145_d_n5, assign6850_e7145_d_n6, assign6850_e7145_d_n7, assign6850_e7145_d_n8, assign6850_e7145_d_n9, assign6850_e7145_d_n10, assign6850_e7145_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6850_e7141: f64 = (p.p94 * locals.var_qbe_qs_eff);
        let assign6850_e7143: f64 = (assign6850_e7141 + locals.var_qbc_qs);
        (assign6850_e7143, ((p.p94 * locals.var_qbe_qs_eff_dn0) + locals.var_qbc_qs_dn0), ((p.p94 * locals.var_qbe_qs_eff_dn1) + locals.var_qbc_qs_dn1), ((p.p94 * locals.var_qbe_qs_eff_dn3) + locals.var_qbc_qs_dn3), ((p.p94 * locals.var_qbe_qs_eff_dn4) + locals.var_qbc_qs_dn4), ((p.p94 * locals.var_qbe_qs_eff_dn5) + locals.var_qbc_qs_dn5), ((p.p94 * locals.var_qbe_qs_eff_dn6) + locals.var_qbc_qs_dn6), ((p.p94 * locals.var_qbe_qs_eff_dn7) + locals.var_qbc_qs_dn7), ((p.p94 * locals.var_qbe_qs_eff_dn8) + locals.var_qbc_qs_dn8), ((p.p94 * locals.var_qbe_qs_eff_dn9) + locals.var_qbc_qs_dn9), ((p.p94 * locals.var_qbe_qs_eff_dn10) + locals.var_qbc_qs_dn10), ((p.p94 * locals.var_qbe_qs_eff_dn11) + locals.var_qbc_qs_dn11),)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10, locals.var_qbc_dn11,)
    }
};
        locals.var_qbc = assign6850_e7145;
        locals.var_qbc_dn0 = assign6850_e7145_d_n0;
        locals.var_qbc_dn1 = assign6850_e7145_d_n1;
        locals.var_qbc_dn3 = assign6850_e7145_d_n3;
        locals.var_qbc_dn4 = assign6850_e7145_d_n4;
        locals.var_qbc_dn5 = assign6850_e7145_d_n5;
        locals.var_qbc_dn6 = assign6850_e7145_d_n6;
        locals.var_qbc_dn7 = assign6850_e7145_d_n7;
        locals.var_qbc_dn8 = assign6850_e7145_d_n8;
        locals.var_qbc_dn9 = assign6850_e7145_d_n9;
        locals.var_qbc_dn10 = assign6850_e7145_d_n10;
        locals.var_qbc_dn11 = assign6850_e7145_d_n11;
        locals.var_qbc_rv = 0.0;

        let (assign6860_e7153, assign6860_e7153_d_n0, assign6860_e7153_d_n1, assign6860_e7153_d_n3, assign6860_e7153_d_n4, assign6860_e7153_d_n5, assign6860_e7153_d_n6, assign6860_e7153_d_n7, assign6860_e7153_d_n8, assign6860_e7153_d_n9, assign6860_e7153_d_n10, assign6860_e7153_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6860_e7149: f64 = (1.0 - p.p94);
        let assign6860_e7151: f64 = (assign6860_e7149 * locals.var_qbe_qs_eff);
        (assign6860_e7151, (assign6860_e7149 * locals.var_qbe_qs_eff_dn0), (assign6860_e7149 * locals.var_qbe_qs_eff_dn1), (assign6860_e7149 * locals.var_qbe_qs_eff_dn3), (assign6860_e7149 * locals.var_qbe_qs_eff_dn4), (assign6860_e7149 * locals.var_qbe_qs_eff_dn5), (assign6860_e7149 * locals.var_qbe_qs_eff_dn6), (assign6860_e7149 * locals.var_qbe_qs_eff_dn7), (assign6860_e7149 * locals.var_qbe_qs_eff_dn8), (assign6860_e7149 * locals.var_qbe_qs_eff_dn9), (assign6860_e7149 * locals.var_qbe_qs_eff_dn10), (assign6860_e7149 * locals.var_qbe_qs_eff_dn11),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10, locals.var_qbe_dn11,)
    }
};
        locals.var_qbe = assign6860_e7153;
        locals.var_qbe_dn0 = assign6860_e7153_d_n0;
        locals.var_qbe_dn1 = assign6860_e7153_d_n1;
        locals.var_qbe_dn3 = assign6860_e7153_d_n3;
        locals.var_qbe_dn4 = assign6860_e7153_d_n4;
        locals.var_qbe_dn5 = assign6860_e7153_d_n5;
        locals.var_qbe_dn6 = assign6860_e7153_d_n6;
        locals.var_qbe_dn7 = assign6860_e7153_d_n7;
        locals.var_qbe_dn8 = assign6860_e7153_d_n8;
        locals.var_qbe_dn9 = assign6860_e7153_d_n9;
        locals.var_qbe_dn10 = assign6860_e7153_d_n10;
        locals.var_qbe_dn11 = assign6860_e7153_d_n11;
        locals.var_qbe_rv = 0.0;

        let (assign6870_e7158, assign6870_e7158_d_n0, assign6870_e7158_d_n1, assign6870_e7158_d_n3, assign6870_e7158_d_n4, assign6870_e7158_d_n5, assign6870_e7158_d_n6, assign6870_e7158_d_n7, assign6870_e7158_d_n8, assign6870_e7158_d_n9, assign6870_e7158_d_n10, assign6870_e7158_d_n11,) = {
    if (locals.var_guard123 == 0.0) {
        (locals.var_qbe_qs, locals.var_qbe_qs_dn0, locals.var_qbe_qs_dn1, locals.var_qbe_qs_dn3, locals.var_qbe_qs_dn4, locals.var_qbe_qs_dn5, locals.var_qbe_qs_dn6, locals.var_qbe_qs_dn7, locals.var_qbe_qs_dn8, locals.var_qbe_qs_dn9, locals.var_qbe_qs_dn10, locals.var_qbe_qs_dn11,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10, locals.var_qbe_dn11,)
    }
};
        locals.var_qbe = assign6870_e7158;
        locals.var_qbe_dn0 = assign6870_e7158_d_n0;
        locals.var_qbe_dn1 = assign6870_e7158_d_n1;
        locals.var_qbe_dn3 = assign6870_e7158_d_n3;
        locals.var_qbe_dn4 = assign6870_e7158_d_n4;
        locals.var_qbe_dn5 = assign6870_e7158_d_n5;
        locals.var_qbe_dn6 = assign6870_e7158_d_n6;
        locals.var_qbe_dn7 = assign6870_e7158_d_n7;
        locals.var_qbe_dn8 = assign6870_e7158_d_n8;
        locals.var_qbe_dn9 = assign6870_e7158_d_n9;
        locals.var_qbe_dn10 = assign6870_e7158_d_n10;
        locals.var_qbe_dn11 = assign6870_e7158_d_n11;
        locals.var_qbe_rv = 0.0;

        let (assign6880_e7163, assign6880_e7163_d_n0, assign6880_e7163_d_n1, assign6880_e7163_d_n3, assign6880_e7163_d_n4, assign6880_e7163_d_n5, assign6880_e7163_d_n6, assign6880_e7163_d_n7, assign6880_e7163_d_n8, assign6880_e7163_d_n9, assign6880_e7163_d_n10, assign6880_e7163_d_n11,) = {
    if (locals.var_guard123 == 0.0) {
        (locals.var_qbc_qs, locals.var_qbc_qs_dn0, locals.var_qbc_qs_dn1, locals.var_qbc_qs_dn3, locals.var_qbc_qs_dn4, locals.var_qbc_qs_dn5, locals.var_qbc_qs_dn6, locals.var_qbc_qs_dn7, locals.var_qbc_qs_dn8, locals.var_qbc_qs_dn9, locals.var_qbc_qs_dn10, locals.var_qbc_qs_dn11,)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10, locals.var_qbc_dn11,)
    }
};
        locals.var_qbc = assign6880_e7163;
        locals.var_qbc_dn0 = assign6880_e7163_d_n0;
        locals.var_qbc_dn1 = assign6880_e7163_d_n1;
        locals.var_qbc_dn3 = assign6880_e7163_d_n3;
        locals.var_qbc_dn4 = assign6880_e7163_d_n4;
        locals.var_qbc_dn5 = assign6880_e7163_d_n5;
        locals.var_qbc_dn6 = assign6880_e7163_d_n6;
        locals.var_qbc_dn7 = assign6880_e7163_d_n7;
        locals.var_qbc_dn8 = assign6880_e7163_d_n8;
        locals.var_qbc_dn9 = assign6880_e7163_d_n9;
        locals.var_qbc_dn10 = assign6880_e7163_d_n10;
        locals.var_qbc_dn11 = assign6880_e7163_d_n11;
        locals.var_qbc_rv = 0.0;

        let (assign6890_e7168, assign6890_e7168_d_n0, assign6890_e7168_d_n1, assign6890_e7168_d_n3, assign6890_e7168_d_n4, assign6890_e7168_d_n5, assign6890_e7168_d_n6, assign6890_e7168_d_n7, assign6890_e7168_d_n8, assign6890_e7168_d_n9, assign6890_e7168_d_n10, assign6890_e7168_d_n11,) = {
    if (locals.var_guard123 == 0.0) {
        (locals.var_qe_qs, locals.var_qe_qs_dn0, locals.var_qe_qs_dn1, locals.var_qe_qs_dn3, locals.var_qe_qs_dn4, locals.var_qe_qs_dn5, locals.var_qe_qs_dn6, locals.var_qe_qs_dn7, locals.var_qe_qs_dn8, locals.var_qe_qs_dn9, locals.var_qe_qs_dn10, locals.var_qe_qs_dn11,)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10, locals.var_qe_dn11,)
    }
};
        locals.var_qe = assign6890_e7168;
        locals.var_qe_dn0 = assign6890_e7168_d_n0;
        locals.var_qe_dn1 = assign6890_e7168_d_n1;
        locals.var_qe_dn3 = assign6890_e7168_d_n3;
        locals.var_qe_dn4 = assign6890_e7168_d_n4;
        locals.var_qe_dn5 = assign6890_e7168_d_n5;
        locals.var_qe_dn6 = assign6890_e7168_d_n6;
        locals.var_qe_dn7 = assign6890_e7168_d_n7;
        locals.var_qe_dn8 = assign6890_e7168_d_n8;
        locals.var_qe_dn9 = assign6890_e7168_d_n9;
        locals.var_qe_dn10 = assign6890_e7168_d_n10;
        locals.var_qe_dn11 = assign6890_e7168_d_n11;
        locals.var_qe_rv = 0.0;

        let assign6910_e7174: f64 = (p.p147 * (nv4 - 0.0));
        let assign6910_e7175_q: f64 = assign6910_e7174;
        let assign6910_e7177: f64 = (assign6910_e7174 * p.p1);
        let assign6910_e7177_q: f64 = (assign6910_e7175_q * p.p1);
        locals.var_i_cth = assign6910_e7177;
        locals.var_i_cth_dn4 = (p.p147 * p.p1);
        locals.var_i_cth_rv = assign6910_e7177_q;
        locals.var_i_cth_rdn4 = (p.p147 * p.p1);

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7090_e7293: f64 = (locals.var_if_ + locals.var_ir);
        let assign7090_e7295: f64 = (assign7090_e7293 / locals.var_qbi);
        locals.var_in_n = assign7090_e7295;
        locals.var_in_n_dn0 = ((((locals.var_if__dn0 + locals.var_ir_dn0) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn1 = ((((locals.var_if__dn1 + locals.var_ir_dn1) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn3 = ((((locals.var_if__dn3 + locals.var_ir_dn3) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn4 = ((((locals.var_if__dn4 + locals.var_ir_dn4) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn5 = ((((locals.var_if__dn5 + locals.var_ir_dn5) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn6 = ((((locals.var_if__dn6 + locals.var_ir_dn6) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn7 = ((((locals.var_if__dn7 + locals.var_ir_dn7) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn8 = ((((locals.var_if__dn8 + locals.var_ir_dn8) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn9 = ((((locals.var_if__dn9 + locals.var_ir_dn9) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn10 = ((((locals.var_if__dn10 + locals.var_ir_dn10) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn10)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn11 = ((((locals.var_if__dn11 + locals.var_ir_dn11) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn11)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_rv = 0.0;

        let assign7150_e7328: f64 = if locals.var_in_n > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign7150_e7328;
        locals.var_guard132_rv = 0.0;

        let (assign7160_e7336, assign7160_e7336_d_n0, assign7160_e7336_d_n1, assign7160_e7336_d_n3, assign7160_e7336_d_n4, assign7160_e7336_d_n5, assign7160_e7336_d_n6, assign7160_e7336_d_n7, assign7160_e7336_d_n8, assign7160_e7336_d_n9, assign7160_e7336_d_n10, assign7160_e7336_d_n11,) = {
    if (locals.var_guard132 != 0.0) {
        let assign7160_e7332: f64 = (locals.var_qbe + locals.var_qbc);
        let assign7160_e7334: f64 = (assign7160_e7332 / locals.var_in_n);
        (assign7160_e7334, ((((locals.var_qbe_dn0 + locals.var_qbc_dn0) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn0)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn1 + locals.var_qbc_dn1) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn1)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn3 + locals.var_qbc_dn3) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn3)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn4 + locals.var_qbc_dn4) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn4)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn5 + locals.var_qbc_dn5) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn5)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn6 + locals.var_qbc_dn6) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn6)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn7 + locals.var_qbc_dn7) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn7)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn8 + locals.var_qbc_dn8) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn8)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn9 + locals.var_qbc_dn9) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn9)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn10 + locals.var_qbc_dn10) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn10)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn11 + locals.var_qbc_dn11) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn11)) / (locals.var_in_n * locals.var_in_n)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10, locals.var_taub_n_dn11,)
    }
};
        locals.var_taub_n = assign7160_e7336;
        locals.var_taub_n_dn0 = assign7160_e7336_d_n0;
        locals.var_taub_n_dn1 = assign7160_e7336_d_n1;
        locals.var_taub_n_dn3 = assign7160_e7336_d_n3;
        locals.var_taub_n_dn4 = assign7160_e7336_d_n4;
        locals.var_taub_n_dn5 = assign7160_e7336_d_n5;
        locals.var_taub_n_dn6 = assign7160_e7336_d_n6;
        locals.var_taub_n_dn7 = assign7160_e7336_d_n7;
        locals.var_taub_n_dn8 = assign7160_e7336_d_n8;
        locals.var_taub_n_dn9 = assign7160_e7336_d_n9;
        locals.var_taub_n_dn10 = assign7160_e7336_d_n10;
        locals.var_taub_n_dn11 = assign7160_e7336_d_n11;
        locals.var_taub_n_rv = 0.0;

        let (assign7170_e7345, assign7170_e7345_d_n0, assign7170_e7345_d_n1, assign7170_e7345_d_n3, assign7170_e7345_d_n4, assign7170_e7345_d_n5, assign7170_e7345_d_n6, assign7170_e7345_d_n7, assign7170_e7345_d_n8, assign7170_e7345_d_n9, assign7170_e7345_d_n10, assign7170_e7345_d_n11,) = {
    if (locals.var_guard132 == 0.0) {
        let assign7170_e7341: f64 = (locals.var_taub_t * locals.var_q1q);
        let assign7170_e7343: f64 = (assign7170_e7341 * locals.var_qbi);
        (assign7170_e7343, (((locals.var_taub_t * locals.var_q1q_dn0) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn0)), (((locals.var_taub_t * locals.var_q1q_dn1) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn1)), (((locals.var_taub_t * locals.var_q1q_dn3) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn3)), ((((locals.var_taub_t_dn4 * locals.var_q1q) + (locals.var_taub_t * locals.var_q1q_dn4)) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn4)), (((locals.var_taub_t * locals.var_q1q_dn5) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn5)), (((locals.var_taub_t * locals.var_q1q_dn6) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn6)), (((locals.var_taub_t * locals.var_q1q_dn7) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn7)), (((locals.var_taub_t * locals.var_q1q_dn8) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn8)), (((locals.var_taub_t * locals.var_q1q_dn9) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn9)), (((locals.var_taub_t * locals.var_q1q_dn10) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn10)), (((locals.var_taub_t * locals.var_q1q_dn11) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn11)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10, locals.var_taub_n_dn11,)
    }
};
        locals.var_taub_n = assign7170_e7345;
        locals.var_taub_n_dn0 = assign7170_e7345_d_n0;
        locals.var_taub_n_dn1 = assign7170_e7345_d_n1;
        locals.var_taub_n_dn3 = assign7170_e7345_d_n3;
        locals.var_taub_n_dn4 = assign7170_e7345_d_n4;
        locals.var_taub_n_dn5 = assign7170_e7345_d_n5;
        locals.var_taub_n_dn6 = assign7170_e7345_d_n6;
        locals.var_taub_n_dn7 = assign7170_e7345_d_n7;
        locals.var_taub_n_dn8 = assign7170_e7345_d_n8;
        locals.var_taub_n_dn9 = assign7170_e7345_d_n9;
        locals.var_taub_n_dn10 = assign7170_e7345_d_n10;
        locals.var_taub_n_dn11 = assign7170_e7345_d_n11;
        locals.var_taub_n_rv = 0.0;

        let assign7180_e7348: f64 = if p.p131 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign7180_e7348;
        locals.var_guard133_rv = 0.0;

        let (assign7190_e7354, assign7190_e7354_d_n0, assign7190_e7354_d_n1, assign7190_e7354_d_n3, assign7190_e7354_d_n4, assign7190_e7354_d_n5, assign7190_e7354_d_n6, assign7190_e7354_d_n7, assign7190_e7354_d_n8, assign7190_e7354_d_n9, assign7190_e7354_d_n10, assign7190_e7354_d_n11,) = {
    if (locals.var_guard133 != 0.0) {
        let assign7190_e7352: f64 = (p.p94 * locals.var_taub_n);
        (assign7190_e7352, (p.p94 * locals.var_taub_n_dn0), (p.p94 * locals.var_taub_n_dn1), (p.p94 * locals.var_taub_n_dn3), (p.p94 * locals.var_taub_n_dn4), (p.p94 * locals.var_taub_n_dn5), (p.p94 * locals.var_taub_n_dn6), (p.p94 * locals.var_taub_n_dn7), (p.p94 * locals.var_taub_n_dn8), (p.p94 * locals.var_taub_n_dn9), (p.p94 * locals.var_taub_n_dn10), (p.p94 * locals.var_taub_n_dn11),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10, locals.var_taun_dn11,)
    }
};
        locals.var_taun = assign7190_e7354;
        locals.var_taun_dn0 = assign7190_e7354_d_n0;
        locals.var_taun_dn1 = assign7190_e7354_d_n1;
        locals.var_taun_dn3 = assign7190_e7354_d_n3;
        locals.var_taun_dn4 = assign7190_e7354_d_n4;
        locals.var_taun_dn5 = assign7190_e7354_d_n5;
        locals.var_taun_dn6 = assign7190_e7354_d_n6;
        locals.var_taun_dn7 = assign7190_e7354_d_n7;
        locals.var_taun_dn8 = assign7190_e7354_d_n8;
        locals.var_taun_dn9 = assign7190_e7354_d_n9;
        locals.var_taun_dn10 = assign7190_e7354_d_n10;
        locals.var_taun_dn11 = assign7190_e7354_d_n11;
        locals.var_taun_rv = 0.0;

        let assign7200_e7357: f64 = if p.p131 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign7200_e7357;
        locals.var_guard134_rv = 0.0;

        let (assign7210_e7366, assign7210_e7366_d_n0, assign7210_e7366_d_n1, assign7210_e7366_d_n3, assign7210_e7366_d_n4, assign7210_e7366_d_n5, assign7210_e7366_d_n6, assign7210_e7366_d_n7, assign7210_e7366_d_n8, assign7210_e7366_d_n9, assign7210_e7366_d_n10, assign7210_e7366_d_n11,) = {
    if ((locals.var_guard133 == 0.0) && (locals.var_guard134 != 0.0)) {
        let assign7210_e7364: f64 = (p.p132 * locals.var_taub_n);
        (assign7210_e7364, (p.p132 * locals.var_taub_n_dn0), (p.p132 * locals.var_taub_n_dn1), (p.p132 * locals.var_taub_n_dn3), (p.p132 * locals.var_taub_n_dn4), (p.p132 * locals.var_taub_n_dn5), (p.p132 * locals.var_taub_n_dn6), (p.p132 * locals.var_taub_n_dn7), (p.p132 * locals.var_taub_n_dn8), (p.p132 * locals.var_taub_n_dn9), (p.p132 * locals.var_taub_n_dn10), (p.p132 * locals.var_taub_n_dn11),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10, locals.var_taun_dn11,)
    }
};
        locals.var_taun = assign7210_e7366;
        locals.var_taun_dn0 = assign7210_e7366_d_n0;
        locals.var_taun_dn1 = assign7210_e7366_d_n1;
        locals.var_taun_dn3 = assign7210_e7366_d_n3;
        locals.var_taun_dn4 = assign7210_e7366_d_n4;
        locals.var_taun_dn5 = assign7210_e7366_d_n5;
        locals.var_taun_dn6 = assign7210_e7366_d_n6;
        locals.var_taun_dn7 = assign7210_e7366_d_n7;
        locals.var_taun_dn8 = assign7210_e7366_d_n8;
        locals.var_taun_dn9 = assign7210_e7366_d_n9;
        locals.var_taun_dn10 = assign7210_e7366_d_n10;
        locals.var_taun_dn11 = assign7210_e7366_d_n11;
        locals.var_taun_rv = 0.0;

        let (assign7220_e7374, assign7220_e7374_d_n0, assign7220_e7374_d_n1, assign7220_e7374_d_n3, assign7220_e7374_d_n4, assign7220_e7374_d_n5, assign7220_e7374_d_n6, assign7220_e7374_d_n7, assign7220_e7374_d_n8, assign7220_e7374_d_n9, assign7220_e7374_d_n10, assign7220_e7374_d_n11,) = {
    if ((locals.var_guard133 == 0.0) && (locals.var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10, locals.var_taun_dn11,)
    }
};
        locals.var_taun = assign7220_e7374;
        locals.var_taun_dn0 = assign7220_e7374_d_n0;
        locals.var_taun_dn1 = assign7220_e7374_d_n1;
        locals.var_taun_dn3 = assign7220_e7374_d_n3;
        locals.var_taun_dn4 = assign7220_e7374_d_n4;
        locals.var_taun_dn5 = assign7220_e7374_d_n5;
        locals.var_taun_dn6 = assign7220_e7374_d_n6;
        locals.var_taun_dn7 = assign7220_e7374_d_n7;
        locals.var_taun_dn8 = assign7220_e7374_d_n8;
        locals.var_taun_dn9 = assign7220_e7374_d_n9;
        locals.var_taun_dn10 = assign7220_e7374_d_n10;
        locals.var_taun_dn11 = assign7220_e7374_d_n11;
        locals.var_taun_rv = 0.0;

    }

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
        locals: &mut StampLocals,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq15_value: f64 = locals.var_i_cth;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq15_value),
            4,
            multiplicity * (locals.var_i_cth_dn4),
        );
        let eq17_e278: f64 = (locals.var_qte + locals.var_qbe);
        let eq17_e278_d_n0: f64 = (locals.var_qte_dn0 + locals.var_qbe_dn0);
        let eq17_e278_d_n1: f64 = (locals.var_qte_dn1 + locals.var_qbe_dn1);
        let eq17_e278_d_n3: f64 = (locals.var_qte_dn3 + locals.var_qbe_dn3);
        let eq17_e278_d_n4: f64 = (locals.var_qte_dn4 + locals.var_qbe_dn4);
        let eq17_e278_d_n5: f64 = (locals.var_qte_dn5 + locals.var_qbe_dn5);
        let eq17_e278_d_n6: f64 = (locals.var_qte_dn6 + locals.var_qbe_dn6);
        let eq17_e278_d_n7: f64 = (locals.var_qte_dn7 + locals.var_qbe_dn7);
        let eq17_e278_d_n8: f64 = (locals.var_qte_dn8 + locals.var_qbe_dn8);
        let eq17_e278_d_n9: f64 = (locals.var_qte_dn9 + locals.var_qbe_dn9);
        let eq17_e278_d_n10: f64 = (locals.var_qte_dn10 + locals.var_qbe_dn10);
        let eq17_e278_d_n11: f64 = (locals.var_qte_dn11 + locals.var_qbe_dn11);
        let eq17_e280: f64 = (eq17_e278 + locals.var_qe);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + locals.var_qe_dn0);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + locals.var_qe_dn1);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + locals.var_qe_dn3);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + locals.var_qe_dn4);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + locals.var_qe_dn5);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + locals.var_qe_dn6);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + locals.var_qe_dn7);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + locals.var_qe_dn8);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + locals.var_qe_dn9);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + locals.var_qe_dn10);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + locals.var_qe_dn11);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e282: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq17_e281);
        let eq17_e284: f64 = (eq17_e282 * p.p1);
        let eq17_e284_d_n0: f64 = ((eq17_e281_d_n0 * ddt_scale) * p.p1);
        let eq17_e284_d_n1: f64 = ((eq17_e281_d_n1 * ddt_scale) * p.p1);
        let eq17_e284_d_n3: f64 = ((eq17_e281_d_n3 * ddt_scale) * p.p1);
        let eq17_e284_d_n4: f64 = ((eq17_e281_d_n4 * ddt_scale) * p.p1);
        let eq17_e284_d_n5: f64 = ((eq17_e281_d_n5 * ddt_scale) * p.p1);
        let eq17_e284_d_n6: f64 = ((eq17_e281_d_n6 * ddt_scale) * p.p1);
        let eq17_e284_d_n7: f64 = ((eq17_e281_d_n7 * ddt_scale) * p.p1);
        let eq17_e284_d_n8: f64 = ((eq17_e281_d_n8 * ddt_scale) * p.p1);
        let eq17_e284_d_n9: f64 = ((eq17_e281_d_n9 * ddt_scale) * p.p1);
        let eq17_e284_d_n10: f64 = ((eq17_e281_d_n10 * ddt_scale) * p.p1);
        let eq17_e284_d_n11: f64 = ((eq17_e281_d_n11 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e284;
        let eq17_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq17_node_derivatives: [f64; 11] = [eq17_e284_d_n0, eq17_e284_d_n1, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11];
        let eq17_branch_derivative_indices: [usize; 0] = [];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivative_indices,
            &eq17_node_derivatives,
            &eq17_branch_derivative_indices,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e287: f64 = (p.p3 * locals.var_qte_s);
        let eq18_e287_d_n0: f64 = (p.p3 * locals.var_qte_s_dn0);
        let eq18_e287_d_n1: f64 = (p.p3 * locals.var_qte_s_dn1);
        let eq18_e287_d_n3: f64 = (p.p3 * locals.var_qte_s_dn3);
        let eq18_e287_d_n4: f64 = (p.p3 * locals.var_qte_s_dn4);
        let eq18_e287_d_n5: f64 = (p.p3 * locals.var_qte_s_dn5);
        let eq18_e287_d_n6: f64 = (p.p3 * locals.var_qte_s_dn6);
        let eq18_e287_d_n7: f64 = (p.p3 * locals.var_qte_s_dn7);
        let eq18_e287_d_n8: f64 = (p.p3 * locals.var_qte_s_dn8);
        let eq18_e287_d_n9: f64 = (p.p3 * locals.var_qte_s_dn9);
        let eq18_e287_d_n10: f64 = (p.p3 * locals.var_qte_s_dn10);
        let eq18_e287_d_n11: f64 = (p.p3 * locals.var_qte_s_dn11);
        let eq18_e288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq18_e287);
        let eq18_e290: f64 = (eq18_e288 * p.p1);
        let eq18_e290_d_n0: f64 = ((eq18_e287_d_n0 * ddt_scale) * p.p1);
        let eq18_e290_d_n1: f64 = ((eq18_e287_d_n1 * ddt_scale) * p.p1);
        let eq18_e290_d_n3: f64 = ((eq18_e287_d_n3 * ddt_scale) * p.p1);
        let eq18_e290_d_n4: f64 = ((eq18_e287_d_n4 * ddt_scale) * p.p1);
        let eq18_e290_d_n5: f64 = ((eq18_e287_d_n5 * ddt_scale) * p.p1);
        let eq18_e290_d_n6: f64 = ((eq18_e287_d_n6 * ddt_scale) * p.p1);
        let eq18_e290_d_n7: f64 = ((eq18_e287_d_n7 * ddt_scale) * p.p1);
        let eq18_e290_d_n8: f64 = ((eq18_e287_d_n8 * ddt_scale) * p.p1);
        let eq18_e290_d_n9: f64 = ((eq18_e287_d_n9 * ddt_scale) * p.p1);
        let eq18_e290_d_n10: f64 = ((eq18_e287_d_n10 * ddt_scale) * p.p1);
        let eq18_e290_d_n11: f64 = ((eq18_e287_d_n11 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e290;
        let eq18_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq18_node_derivatives: [f64; 11] = [eq18_e290_d_n0, eq18_e290_d_n1, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e294: f64 = (locals.var_qtc + locals.var_qbc);
        let eq19_e294_d_n0: f64 = (locals.var_qtc_dn0 + locals.var_qbc_dn0);
        let eq19_e294_d_n1: f64 = (locals.var_qtc_dn1 + locals.var_qbc_dn1);
        let eq19_e294_d_n3: f64 = (locals.var_qtc_dn3 + locals.var_qbc_dn3);
        let eq19_e294_d_n4: f64 = (locals.var_qtc_dn4 + locals.var_qbc_dn4);
        let eq19_e294_d_n5: f64 = (locals.var_qtc_dn5 + locals.var_qbc_dn5);
        let eq19_e294_d_n6: f64 = (locals.var_qtc_dn6 + locals.var_qbc_dn6);
        let eq19_e294_d_n7: f64 = (locals.var_qtc_dn7 + locals.var_qbc_dn7);
        let eq19_e294_d_n8: f64 = (locals.var_qtc_dn8 + locals.var_qbc_dn8);
        let eq19_e294_d_n9: f64 = (locals.var_qtc_dn9 + locals.var_qbc_dn9);
        let eq19_e294_d_n10: f64 = (locals.var_qtc_dn10 + locals.var_qbc_dn10);
        let eq19_e294_d_n11: f64 = (locals.var_qtc_dn11 + locals.var_qbc_dn11);
        let eq19_e296: f64 = (eq19_e294 + locals.var_qepi);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + locals.var_qepi_dn0);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + locals.var_qepi_dn1);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + locals.var_qepi_dn3);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + locals.var_qepi_dn4);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + locals.var_qepi_dn5);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + locals.var_qepi_dn6);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + locals.var_qepi_dn7);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + locals.var_qepi_dn8);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + locals.var_qepi_dn9);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + locals.var_qepi_dn10);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + locals.var_qepi_dn11);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e298: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq19_e297);
        let eq19_e300: f64 = (eq19_e298 * p.p1);
        let eq19_e300_d_n0: f64 = ((eq19_e297_d_n0 * ddt_scale) * p.p1);
        let eq19_e300_d_n1: f64 = ((eq19_e297_d_n1 * ddt_scale) * p.p1);
        let eq19_e300_d_n3: f64 = ((eq19_e297_d_n3 * ddt_scale) * p.p1);
        let eq19_e300_d_n4: f64 = ((eq19_e297_d_n4 * ddt_scale) * p.p1);
        let eq19_e300_d_n5: f64 = ((eq19_e297_d_n5 * ddt_scale) * p.p1);
        let eq19_e300_d_n6: f64 = ((eq19_e297_d_n6 * ddt_scale) * p.p1);
        let eq19_e300_d_n7: f64 = ((eq19_e297_d_n7 * ddt_scale) * p.p1);
        let eq19_e300_d_n8: f64 = ((eq19_e297_d_n8 * ddt_scale) * p.p1);
        let eq19_e300_d_n9: f64 = ((eq19_e297_d_n9 * ddt_scale) * p.p1);
        let eq19_e300_d_n10: f64 = ((eq19_e297_d_n10 * ddt_scale) * p.p1);
        let eq19_e300_d_n11: f64 = ((eq19_e297_d_n11 * ddt_scale) * p.p1);
        let eq19_value: f64 = eq19_e300;
        let eq19_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq19_node_derivatives: [f64; 11] = [eq19_e300_d_n0, eq19_e300_d_n1, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * locals.var_qts);
        let eq20_e303_d_n0: f64 = (p.p3 * locals.var_qts_dn0);
        let eq20_e303_d_n1: f64 = (p.p3 * locals.var_qts_dn1);
        let eq20_e303_d_n3: f64 = (p.p3 * locals.var_qts_dn3);
        let eq20_e303_d_n4: f64 = (p.p3 * locals.var_qts_dn4);
        let eq20_e303_d_n5: f64 = (p.p3 * locals.var_qts_dn5);
        let eq20_e303_d_n6: f64 = (p.p3 * locals.var_qts_dn6);
        let eq20_e303_d_n7: f64 = (p.p3 * locals.var_qts_dn7);
        let eq20_e303_d_n8: f64 = (p.p3 * locals.var_qts_dn8);
        let eq20_e303_d_n9: f64 = (p.p3 * locals.var_qts_dn9);
        let eq20_e303_d_n10: f64 = (p.p3 * locals.var_qts_dn10);
        let eq20_e303_d_n11: f64 = (p.p3 * locals.var_qts_dn11);
        let eq20_e304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq20_e303);
        let eq20_e306: f64 = (eq20_e304 * p.p1);
        let eq20_e306_d_n0: f64 = ((eq20_e303_d_n0 * ddt_scale) * p.p1);
        let eq20_e306_d_n1: f64 = ((eq20_e303_d_n1 * ddt_scale) * p.p1);
        let eq20_e306_d_n3: f64 = ((eq20_e303_d_n3 * ddt_scale) * p.p1);
        let eq20_e306_d_n4: f64 = ((eq20_e303_d_n4 * ddt_scale) * p.p1);
        let eq20_e306_d_n5: f64 = ((eq20_e303_d_n5 * ddt_scale) * p.p1);
        let eq20_e306_d_n6: f64 = ((eq20_e303_d_n6 * ddt_scale) * p.p1);
        let eq20_e306_d_n7: f64 = ((eq20_e303_d_n7 * ddt_scale) * p.p1);
        let eq20_e306_d_n8: f64 = ((eq20_e303_d_n8 * ddt_scale) * p.p1);
        let eq20_e306_d_n9: f64 = ((eq20_e303_d_n9 * ddt_scale) * p.p1);
        let eq20_e306_d_n10: f64 = ((eq20_e303_d_n10 * ddt_scale) * p.p1);
        let eq20_e306_d_n11: f64 = ((eq20_e303_d_n11 * ddt_scale) * p.p1);
        let eq20_value: f64 = eq20_e306;
        let eq20_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq20_node_derivatives: [f64; 11] = [eq20_e306_d_n0, eq20_e306_d_n1, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * locals.var_qb1b2);
        let eq21_e309_d_n0: f64 = (p.p3 * locals.var_qb1b2_dn0);
        let eq21_e309_d_n1: f64 = (p.p3 * locals.var_qb1b2_dn1);
        let eq21_e309_d_n3: f64 = (p.p3 * locals.var_qb1b2_dn3);
        let eq21_e309_d_n4: f64 = (p.p3 * locals.var_qb1b2_dn4);
        let eq21_e309_d_n5: f64 = (p.p3 * locals.var_qb1b2_dn5);
        let eq21_e309_d_n6: f64 = (p.p3 * locals.var_qb1b2_dn6);
        let eq21_e309_d_n7: f64 = (p.p3 * locals.var_qb1b2_dn7);
        let eq21_e309_d_n8: f64 = (p.p3 * locals.var_qb1b2_dn8);
        let eq21_e309_d_n9: f64 = (p.p3 * locals.var_qb1b2_dn9);
        let eq21_e309_d_n10: f64 = (p.p3 * locals.var_qb1b2_dn10);
        let eq21_e309_d_n11: f64 = (p.p3 * locals.var_qb1b2_dn11);
        let eq21_e310: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq21_e309);
        let eq21_e312: f64 = (eq21_e310 * p.p1);
        let eq21_e312_d_n0: f64 = ((eq21_e309_d_n0 * ddt_scale) * p.p1);
        let eq21_e312_d_n1: f64 = ((eq21_e309_d_n1 * ddt_scale) * p.p1);
        let eq21_e312_d_n3: f64 = ((eq21_e309_d_n3 * ddt_scale) * p.p1);
        let eq21_e312_d_n4: f64 = ((eq21_e309_d_n4 * ddt_scale) * p.p1);
        let eq21_e312_d_n5: f64 = ((eq21_e309_d_n5 * ddt_scale) * p.p1);
        let eq21_e312_d_n6: f64 = ((eq21_e309_d_n6 * ddt_scale) * p.p1);
        let eq21_e312_d_n7: f64 = ((eq21_e309_d_n7 * ddt_scale) * p.p1);
        let eq21_e312_d_n8: f64 = ((eq21_e309_d_n8 * ddt_scale) * p.p1);
        let eq21_e312_d_n9: f64 = ((eq21_e309_d_n9 * ddt_scale) * p.p1);
        let eq21_e312_d_n10: f64 = ((eq21_e309_d_n10 * ddt_scale) * p.p1);
        let eq21_e312_d_n11: f64 = ((eq21_e309_d_n11 * ddt_scale) * p.p1);
        let eq21_value: f64 = eq21_e312;
        let eq21_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq21_node_derivatives: [f64; 11] = [eq21_e312_d_n0, eq21_e312_d_n1, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * locals.var_vbe);
        let eq22_e317_d_n1: f64 = (eq22_e315 * locals.var_vbe_dn1);
        let eq22_e317_d_n2: f64 = (eq22_e315 * locals.var_vbe_dn2);
        let eq22_e318: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq22_e317);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n1: f64 = ((eq22_e317_d_n1 * ddt_scale) * p.p1);
        let eq22_e320_d_n2: f64 = ((eq22_e317_d_n2 * ddt_scale) * p.p1);
        let eq22_value: f64 = eq22_e320;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq22_value),
            1,
            multiplicity * (eq22_e320_d_n1),
            2,
            multiplicity * (eq22_e320_d_n2),
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * locals.var_vbc);
        let eq23_e325_d_n0: f64 = (eq23_e323 * locals.var_vbc_dn0);
        let eq23_e325_d_n1: f64 = (eq23_e323 * locals.var_vbc_dn1);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e325);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e325_d_n0 * ddt_scale) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e325_d_n1 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e328;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (eq23_value),
            0,
            multiplicity * (eq23_e328_d_n0),
            1,
            multiplicity * (eq23_e328_d_n1),
        );
        let eq26_e344: f64 = (locals.var_xqtex + locals.var_xqex);
        let eq26_e344_d_n0: f64 = (locals.var_xqtex_dn0 + locals.var_xqex_dn0);
        let eq26_e344_d_n1: f64 = (locals.var_xqtex_dn1 + locals.var_xqex_dn1);
        let eq26_e344_d_n3: f64 = (locals.var_xqtex_dn3 + locals.var_xqex_dn3);
        let eq26_e344_d_n4: f64 = (locals.var_xqtex_dn4 + locals.var_xqex_dn4);
        let eq26_e344_d_n5: f64 = (locals.var_xqtex_dn5 + locals.var_xqex_dn5);
        let eq26_e344_d_n6: f64 = (locals.var_xqtex_dn6 + locals.var_xqex_dn6);
        let eq26_e344_d_n7: f64 = (locals.var_xqtex_dn7 + locals.var_xqex_dn7);
        let eq26_e344_d_n8: f64 = (locals.var_xqtex_dn8 + locals.var_xqex_dn8);
        let eq26_e344_d_n9: f64 = (locals.var_xqtex_dn9 + locals.var_xqex_dn9);
        let eq26_e344_d_n10: f64 = (locals.var_xqtex_dn10 + locals.var_xqex_dn10);
        let eq26_e344_d_n11: f64 = (locals.var_xqtex_dn11 + locals.var_xqex_dn11);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq26_e345);
        let eq26_e348: f64 = (eq26_e346 * p.p1);
        let eq26_e348_d_n0: f64 = ((eq26_e345_d_n0 * ddt_scale) * p.p1);
        let eq26_e348_d_n1: f64 = ((eq26_e345_d_n1 * ddt_scale) * p.p1);
        let eq26_e348_d_n3: f64 = ((eq26_e345_d_n3 * ddt_scale) * p.p1);
        let eq26_e348_d_n4: f64 = ((eq26_e345_d_n4 * ddt_scale) * p.p1);
        let eq26_e348_d_n5: f64 = ((eq26_e345_d_n5 * ddt_scale) * p.p1);
        let eq26_e348_d_n6: f64 = ((eq26_e345_d_n6 * ddt_scale) * p.p1);
        let eq26_e348_d_n7: f64 = ((eq26_e345_d_n7 * ddt_scale) * p.p1);
        let eq26_e348_d_n8: f64 = ((eq26_e345_d_n8 * ddt_scale) * p.p1);
        let eq26_e348_d_n9: f64 = ((eq26_e345_d_n9 * ddt_scale) * p.p1);
        let eq26_e348_d_n10: f64 = ((eq26_e345_d_n10 * ddt_scale) * p.p1);
        let eq26_e348_d_n11: f64 = ((eq26_e345_d_n11 * ddt_scale) * p.p1);
        let eq26_value: f64 = eq26_e348;
        let eq26_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq26_node_derivatives: [f64; 11] = [eq26_e348_d_n0, eq26_e348_d_n1, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq28_e363: f64 = (locals.var_qtex + locals.var_qex);
        let eq28_e363_d_n0: f64 = (locals.var_qtex_dn0 + locals.var_qex_dn0);
        let eq28_e363_d_n1: f64 = (locals.var_qtex_dn1 + locals.var_qex_dn1);
        let eq28_e363_d_n3: f64 = (locals.var_qtex_dn3 + locals.var_qex_dn3);
        let eq28_e363_d_n4: f64 = (locals.var_qtex_dn4 + locals.var_qex_dn4);
        let eq28_e363_d_n5: f64 = (locals.var_qtex_dn5 + locals.var_qex_dn5);
        let eq28_e363_d_n6: f64 = (locals.var_qtex_dn6 + locals.var_qex_dn6);
        let eq28_e363_d_n7: f64 = (locals.var_qtex_dn7 + locals.var_qex_dn7);
        let eq28_e363_d_n8: f64 = (locals.var_qtex_dn8 + locals.var_qex_dn8);
        let eq28_e363_d_n9: f64 = (locals.var_qtex_dn9 + locals.var_qex_dn9);
        let eq28_e363_d_n10: f64 = (locals.var_qtex_dn10 + locals.var_qex_dn10);
        let eq28_e363_d_n11: f64 = (locals.var_qtex_dn11 + locals.var_qex_dn11);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e365: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq28_e364);
        let eq28_e367: f64 = (eq28_e365 * p.p1);
        let eq28_e367_d_n0: f64 = ((eq28_e364_d_n0 * ddt_scale) * p.p1);
        let eq28_e367_d_n1: f64 = ((eq28_e364_d_n1 * ddt_scale) * p.p1);
        let eq28_e367_d_n3: f64 = ((eq28_e364_d_n3 * ddt_scale) * p.p1);
        let eq28_e367_d_n4: f64 = ((eq28_e364_d_n4 * ddt_scale) * p.p1);
        let eq28_e367_d_n5: f64 = ((eq28_e364_d_n5 * ddt_scale) * p.p1);
        let eq28_e367_d_n6: f64 = ((eq28_e364_d_n6 * ddt_scale) * p.p1);
        let eq28_e367_d_n7: f64 = ((eq28_e364_d_n7 * ddt_scale) * p.p1);
        let eq28_e367_d_n8: f64 = ((eq28_e364_d_n8 * ddt_scale) * p.p1);
        let eq28_e367_d_n9: f64 = ((eq28_e364_d_n9 * ddt_scale) * p.p1);
        let eq28_e367_d_n10: f64 = ((eq28_e364_d_n10 * ddt_scale) * p.p1);
        let eq28_e367_d_n11: f64 = ((eq28_e364_d_n11 * ddt_scale) * p.p1);
        let eq28_value: f64 = eq28_e367;
        let eq28_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq28_node_derivatives: [f64; 11] = [eq28_e367_d_n0, eq28_e367_d_n1, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq35_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (nv12 - 0.0));
        let eq35_e407: f64 = (locals.var_taun * eq35_e406);
        let eq35_e407_d_n0: f64 = (locals.var_taun_dn0 * eq35_e406);
        let eq35_e407_d_n1: f64 = (locals.var_taun_dn1 * eq35_e406);
        let eq35_e407_d_n3: f64 = (locals.var_taun_dn3 * eq35_e406);
        let eq35_e407_d_n4: f64 = (locals.var_taun_dn4 * eq35_e406);
        let eq35_e407_d_n5: f64 = (locals.var_taun_dn5 * eq35_e406);
        let eq35_e407_d_n6: f64 = (locals.var_taun_dn6 * eq35_e406);
        let eq35_e407_d_n7: f64 = (locals.var_taun_dn7 * eq35_e406);
        let eq35_e407_d_n8: f64 = (locals.var_taun_dn8 * eq35_e406);
        let eq35_e407_d_n9: f64 = (locals.var_taun_dn9 * eq35_e406);
        let eq35_e407_d_n10: f64 = (locals.var_taun_dn10 * eq35_e406);
        let eq35_e407_d_n11: f64 = (locals.var_taun_dn11 * eq35_e406);
        let eq35_value: f64 = eq35_e407;
        let eq35_node_derivative_indices: [usize; 12] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq35_node_derivatives: [f64; 12] = [eq35_e407_d_n0, eq35_e407_d_n1, eq35_e407_d_n3, eq35_e407_d_n4, eq35_e407_d_n5, eq35_e407_d_n6, eq35_e407_d_n7, eq35_e407_d_n8, eq35_e407_d_n9, eq35_e407_d_n10, eq35_e407_d_n11, (locals.var_taun * ddt_scale)];
        let eq35_branch_derivative_indices: [usize; 0] = [];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq35_value),
            &eq35_node_derivative_indices,
            &eq35_node_derivatives,
            &eq35_branch_derivative_indices,
            &eq35_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq15_e268_q: f64 = locals.var_i_cth_rv;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (locals.var_i_cth_rdn4),
        );
        let eq17_e278: f64 = (locals.var_qte + locals.var_qbe);
        let eq17_e278_d_n0: f64 = (locals.var_qte_dn0 + locals.var_qbe_dn0);
        let eq17_e278_d_n1: f64 = (locals.var_qte_dn1 + locals.var_qbe_dn1);
        let eq17_e278_d_n3: f64 = (locals.var_qte_dn3 + locals.var_qbe_dn3);
        let eq17_e278_d_n4: f64 = (locals.var_qte_dn4 + locals.var_qbe_dn4);
        let eq17_e278_d_n5: f64 = (locals.var_qte_dn5 + locals.var_qbe_dn5);
        let eq17_e278_d_n6: f64 = (locals.var_qte_dn6 + locals.var_qbe_dn6);
        let eq17_e278_d_n7: f64 = (locals.var_qte_dn7 + locals.var_qbe_dn7);
        let eq17_e278_d_n8: f64 = (locals.var_qte_dn8 + locals.var_qbe_dn8);
        let eq17_e278_d_n9: f64 = (locals.var_qte_dn9 + locals.var_qbe_dn9);
        let eq17_e278_d_n10: f64 = (locals.var_qte_dn10 + locals.var_qbe_dn10);
        let eq17_e278_d_n11: f64 = (locals.var_qte_dn11 + locals.var_qbe_dn11);
        let eq17_e280: f64 = (eq17_e278 + locals.var_qe);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + locals.var_qe_dn0);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + locals.var_qe_dn1);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + locals.var_qe_dn3);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + locals.var_qe_dn4);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + locals.var_qe_dn5);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + locals.var_qe_dn6);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + locals.var_qe_dn7);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + locals.var_qe_dn8);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + locals.var_qe_dn9);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + locals.var_qe_dn10);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + locals.var_qe_dn11);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e282_q: f64 = eq17_e281;
        let eq17_e284: f64 = (eq17_e281 * p.p1);
        let eq17_e284_d_n0: f64 = (eq17_e281_d_n0 * p.p1);
        let eq17_e284_d_n1: f64 = (eq17_e281_d_n1 * p.p1);
        let eq17_e284_d_n3: f64 = (eq17_e281_d_n3 * p.p1);
        let eq17_e284_d_n4: f64 = (eq17_e281_d_n4 * p.p1);
        let eq17_e284_d_n5: f64 = (eq17_e281_d_n5 * p.p1);
        let eq17_e284_d_n6: f64 = (eq17_e281_d_n6 * p.p1);
        let eq17_e284_d_n7: f64 = (eq17_e281_d_n7 * p.p1);
        let eq17_e284_d_n8: f64 = (eq17_e281_d_n8 * p.p1);
        let eq17_e284_d_n9: f64 = (eq17_e281_d_n9 * p.p1);
        let eq17_e284_d_n10: f64 = (eq17_e281_d_n10 * p.p1);
        let eq17_e284_d_n11: f64 = (eq17_e281_d_n11 * p.p1);
        let eq17_e284_q: f64 = (eq17_e282_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 13] = [eq17_e284_d_n0, eq17_e284_d_n1, 0.0, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11, 0.0];
        let eq17_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e287: f64 = (p.p3 * locals.var_qte_s);
        let eq18_e287_d_n0: f64 = (p.p3 * locals.var_qte_s_dn0);
        let eq18_e287_d_n1: f64 = (p.p3 * locals.var_qte_s_dn1);
        let eq18_e287_d_n3: f64 = (p.p3 * locals.var_qte_s_dn3);
        let eq18_e287_d_n4: f64 = (p.p3 * locals.var_qte_s_dn4);
        let eq18_e287_d_n5: f64 = (p.p3 * locals.var_qte_s_dn5);
        let eq18_e287_d_n6: f64 = (p.p3 * locals.var_qte_s_dn6);
        let eq18_e287_d_n7: f64 = (p.p3 * locals.var_qte_s_dn7);
        let eq18_e287_d_n8: f64 = (p.p3 * locals.var_qte_s_dn8);
        let eq18_e287_d_n9: f64 = (p.p3 * locals.var_qte_s_dn9);
        let eq18_e287_d_n10: f64 = (p.p3 * locals.var_qte_s_dn10);
        let eq18_e287_d_n11: f64 = (p.p3 * locals.var_qte_s_dn11);
        let eq18_e288_q: f64 = eq18_e287;
        let eq18_e290: f64 = (eq18_e287 * p.p1);
        let eq18_e290_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e290_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e290_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e290_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e290_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e290_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e290_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e290_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e290_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e290_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e290_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_e290_q: f64 = (eq18_e288_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e290_d_n0, eq18_e290_d_n1, 0.0, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11, 0.0];
        let eq18_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e294: f64 = (locals.var_qtc + locals.var_qbc);
        let eq19_e294_d_n0: f64 = (locals.var_qtc_dn0 + locals.var_qbc_dn0);
        let eq19_e294_d_n1: f64 = (locals.var_qtc_dn1 + locals.var_qbc_dn1);
        let eq19_e294_d_n3: f64 = (locals.var_qtc_dn3 + locals.var_qbc_dn3);
        let eq19_e294_d_n4: f64 = (locals.var_qtc_dn4 + locals.var_qbc_dn4);
        let eq19_e294_d_n5: f64 = (locals.var_qtc_dn5 + locals.var_qbc_dn5);
        let eq19_e294_d_n6: f64 = (locals.var_qtc_dn6 + locals.var_qbc_dn6);
        let eq19_e294_d_n7: f64 = (locals.var_qtc_dn7 + locals.var_qbc_dn7);
        let eq19_e294_d_n8: f64 = (locals.var_qtc_dn8 + locals.var_qbc_dn8);
        let eq19_e294_d_n9: f64 = (locals.var_qtc_dn9 + locals.var_qbc_dn9);
        let eq19_e294_d_n10: f64 = (locals.var_qtc_dn10 + locals.var_qbc_dn10);
        let eq19_e294_d_n11: f64 = (locals.var_qtc_dn11 + locals.var_qbc_dn11);
        let eq19_e296: f64 = (eq19_e294 + locals.var_qepi);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + locals.var_qepi_dn0);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + locals.var_qepi_dn1);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + locals.var_qepi_dn3);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + locals.var_qepi_dn4);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + locals.var_qepi_dn5);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + locals.var_qepi_dn6);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + locals.var_qepi_dn7);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + locals.var_qepi_dn8);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + locals.var_qepi_dn9);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + locals.var_qepi_dn10);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + locals.var_qepi_dn11);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e298_q: f64 = eq19_e297;
        let eq19_e300: f64 = (eq19_e297 * p.p1);
        let eq19_e300_d_n0: f64 = (eq19_e297_d_n0 * p.p1);
        let eq19_e300_d_n1: f64 = (eq19_e297_d_n1 * p.p1);
        let eq19_e300_d_n3: f64 = (eq19_e297_d_n3 * p.p1);
        let eq19_e300_d_n4: f64 = (eq19_e297_d_n4 * p.p1);
        let eq19_e300_d_n5: f64 = (eq19_e297_d_n5 * p.p1);
        let eq19_e300_d_n6: f64 = (eq19_e297_d_n6 * p.p1);
        let eq19_e300_d_n7: f64 = (eq19_e297_d_n7 * p.p1);
        let eq19_e300_d_n8: f64 = (eq19_e297_d_n8 * p.p1);
        let eq19_e300_d_n9: f64 = (eq19_e297_d_n9 * p.p1);
        let eq19_e300_d_n10: f64 = (eq19_e297_d_n10 * p.p1);
        let eq19_e300_d_n11: f64 = (eq19_e297_d_n11 * p.p1);
        let eq19_e300_q: f64 = (eq19_e298_q * p.p1);
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e300_d_n0, eq19_e300_d_n1, 0.0, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11, 0.0];
        let eq19_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * locals.var_qts);
        let eq20_e303_d_n0: f64 = (p.p3 * locals.var_qts_dn0);
        let eq20_e303_d_n1: f64 = (p.p3 * locals.var_qts_dn1);
        let eq20_e303_d_n3: f64 = (p.p3 * locals.var_qts_dn3);
        let eq20_e303_d_n4: f64 = (p.p3 * locals.var_qts_dn4);
        let eq20_e303_d_n5: f64 = (p.p3 * locals.var_qts_dn5);
        let eq20_e303_d_n6: f64 = (p.p3 * locals.var_qts_dn6);
        let eq20_e303_d_n7: f64 = (p.p3 * locals.var_qts_dn7);
        let eq20_e303_d_n8: f64 = (p.p3 * locals.var_qts_dn8);
        let eq20_e303_d_n9: f64 = (p.p3 * locals.var_qts_dn9);
        let eq20_e303_d_n10: f64 = (p.p3 * locals.var_qts_dn10);
        let eq20_e303_d_n11: f64 = (p.p3 * locals.var_qts_dn11);
        let eq20_e304_q: f64 = eq20_e303;
        let eq20_e306: f64 = (eq20_e303 * p.p1);
        let eq20_e306_d_n0: f64 = (eq20_e303_d_n0 * p.p1);
        let eq20_e306_d_n1: f64 = (eq20_e303_d_n1 * p.p1);
        let eq20_e306_d_n3: f64 = (eq20_e303_d_n3 * p.p1);
        let eq20_e306_d_n4: f64 = (eq20_e303_d_n4 * p.p1);
        let eq20_e306_d_n5: f64 = (eq20_e303_d_n5 * p.p1);
        let eq20_e306_d_n6: f64 = (eq20_e303_d_n6 * p.p1);
        let eq20_e306_d_n7: f64 = (eq20_e303_d_n7 * p.p1);
        let eq20_e306_d_n8: f64 = (eq20_e303_d_n8 * p.p1);
        let eq20_e306_d_n9: f64 = (eq20_e303_d_n9 * p.p1);
        let eq20_e306_d_n10: f64 = (eq20_e303_d_n10 * p.p1);
        let eq20_e306_d_n11: f64 = (eq20_e303_d_n11 * p.p1);
        let eq20_e306_q: f64 = (eq20_e304_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 13] = [eq20_e306_d_n0, eq20_e306_d_n1, 0.0, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11, 0.0];
        let eq20_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * locals.var_qb1b2);
        let eq21_e309_d_n0: f64 = (p.p3 * locals.var_qb1b2_dn0);
        let eq21_e309_d_n1: f64 = (p.p3 * locals.var_qb1b2_dn1);
        let eq21_e309_d_n3: f64 = (p.p3 * locals.var_qb1b2_dn3);
        let eq21_e309_d_n4: f64 = (p.p3 * locals.var_qb1b2_dn4);
        let eq21_e309_d_n5: f64 = (p.p3 * locals.var_qb1b2_dn5);
        let eq21_e309_d_n6: f64 = (p.p3 * locals.var_qb1b2_dn6);
        let eq21_e309_d_n7: f64 = (p.p3 * locals.var_qb1b2_dn7);
        let eq21_e309_d_n8: f64 = (p.p3 * locals.var_qb1b2_dn8);
        let eq21_e309_d_n9: f64 = (p.p3 * locals.var_qb1b2_dn9);
        let eq21_e309_d_n10: f64 = (p.p3 * locals.var_qb1b2_dn10);
        let eq21_e309_d_n11: f64 = (p.p3 * locals.var_qb1b2_dn11);
        let eq21_e310_q: f64 = eq21_e309;
        let eq21_e312: f64 = (eq21_e309 * p.p1);
        let eq21_e312_d_n0: f64 = (eq21_e309_d_n0 * p.p1);
        let eq21_e312_d_n1: f64 = (eq21_e309_d_n1 * p.p1);
        let eq21_e312_d_n3: f64 = (eq21_e309_d_n3 * p.p1);
        let eq21_e312_d_n4: f64 = (eq21_e309_d_n4 * p.p1);
        let eq21_e312_d_n5: f64 = (eq21_e309_d_n5 * p.p1);
        let eq21_e312_d_n6: f64 = (eq21_e309_d_n6 * p.p1);
        let eq21_e312_d_n7: f64 = (eq21_e309_d_n7 * p.p1);
        let eq21_e312_d_n8: f64 = (eq21_e309_d_n8 * p.p1);
        let eq21_e312_d_n9: f64 = (eq21_e309_d_n9 * p.p1);
        let eq21_e312_d_n10: f64 = (eq21_e309_d_n10 * p.p1);
        let eq21_e312_d_n11: f64 = (eq21_e309_d_n11 * p.p1);
        let eq21_e312_q: f64 = (eq21_e310_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 13] = [eq21_e312_d_n0, eq21_e312_d_n1, 0.0, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11, 0.0];
        let eq21_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * locals.var_vbe);
        let eq22_e317_d_n1: f64 = (eq22_e315 * locals.var_vbe_dn1);
        let eq22_e317_d_n2: f64 = (eq22_e315 * locals.var_vbe_dn2);
        let eq22_e318_q: f64 = eq22_e317;
        let eq22_e320: f64 = (eq22_e317 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e317_d_n1 * p.p1);
        let eq22_e320_d_n2: f64 = (eq22_e317_d_n2 * p.p1);
        let eq22_e320_q: f64 = (eq22_e318_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq22_e320_d_n1),
            nodes[2],
            multiplicity * (eq22_e320_d_n2),
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * locals.var_vbc);
        let eq23_e325_d_n0: f64 = (eq23_e323 * locals.var_vbc_dn0);
        let eq23_e325_d_n1: f64 = (eq23_e323 * locals.var_vbc_dn1);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq23_e328_d_n0),
            nodes[1],
            multiplicity * (eq23_e328_d_n1),
        );
        let eq26_e344: f64 = (locals.var_xqtex + locals.var_xqex);
        let eq26_e344_d_n0: f64 = (locals.var_xqtex_dn0 + locals.var_xqex_dn0);
        let eq26_e344_d_n1: f64 = (locals.var_xqtex_dn1 + locals.var_xqex_dn1);
        let eq26_e344_d_n3: f64 = (locals.var_xqtex_dn3 + locals.var_xqex_dn3);
        let eq26_e344_d_n4: f64 = (locals.var_xqtex_dn4 + locals.var_xqex_dn4);
        let eq26_e344_d_n5: f64 = (locals.var_xqtex_dn5 + locals.var_xqex_dn5);
        let eq26_e344_d_n6: f64 = (locals.var_xqtex_dn6 + locals.var_xqex_dn6);
        let eq26_e344_d_n7: f64 = (locals.var_xqtex_dn7 + locals.var_xqex_dn7);
        let eq26_e344_d_n8: f64 = (locals.var_xqtex_dn8 + locals.var_xqex_dn8);
        let eq26_e344_d_n9: f64 = (locals.var_xqtex_dn9 + locals.var_xqex_dn9);
        let eq26_e344_d_n10: f64 = (locals.var_xqtex_dn10 + locals.var_xqex_dn10);
        let eq26_e344_d_n11: f64 = (locals.var_xqtex_dn11 + locals.var_xqex_dn11);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e346_q: f64 = eq26_e345;
        let eq26_e348: f64 = (eq26_e345 * p.p1);
        let eq26_e348_d_n0: f64 = (eq26_e345_d_n0 * p.p1);
        let eq26_e348_d_n1: f64 = (eq26_e345_d_n1 * p.p1);
        let eq26_e348_d_n3: f64 = (eq26_e345_d_n3 * p.p1);
        let eq26_e348_d_n4: f64 = (eq26_e345_d_n4 * p.p1);
        let eq26_e348_d_n5: f64 = (eq26_e345_d_n5 * p.p1);
        let eq26_e348_d_n6: f64 = (eq26_e345_d_n6 * p.p1);
        let eq26_e348_d_n7: f64 = (eq26_e345_d_n7 * p.p1);
        let eq26_e348_d_n8: f64 = (eq26_e345_d_n8 * p.p1);
        let eq26_e348_d_n9: f64 = (eq26_e345_d_n9 * p.p1);
        let eq26_e348_d_n10: f64 = (eq26_e345_d_n10 * p.p1);
        let eq26_e348_d_n11: f64 = (eq26_e345_d_n11 * p.p1);
        let eq26_e348_q: f64 = (eq26_e346_q * p.p1);
        let eq26_reactive_node_derivatives: [f64; 13] = [eq26_e348_d_n0, eq26_e348_d_n1, 0.0, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11, 0.0];
        let eq26_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e363: f64 = (locals.var_qtex + locals.var_qex);
        let eq28_e363_d_n0: f64 = (locals.var_qtex_dn0 + locals.var_qex_dn0);
        let eq28_e363_d_n1: f64 = (locals.var_qtex_dn1 + locals.var_qex_dn1);
        let eq28_e363_d_n3: f64 = (locals.var_qtex_dn3 + locals.var_qex_dn3);
        let eq28_e363_d_n4: f64 = (locals.var_qtex_dn4 + locals.var_qex_dn4);
        let eq28_e363_d_n5: f64 = (locals.var_qtex_dn5 + locals.var_qex_dn5);
        let eq28_e363_d_n6: f64 = (locals.var_qtex_dn6 + locals.var_qex_dn6);
        let eq28_e363_d_n7: f64 = (locals.var_qtex_dn7 + locals.var_qex_dn7);
        let eq28_e363_d_n8: f64 = (locals.var_qtex_dn8 + locals.var_qex_dn8);
        let eq28_e363_d_n9: f64 = (locals.var_qtex_dn9 + locals.var_qex_dn9);
        let eq28_e363_d_n10: f64 = (locals.var_qtex_dn10 + locals.var_qex_dn10);
        let eq28_e363_d_n11: f64 = (locals.var_qtex_dn11 + locals.var_qex_dn11);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e365_q: f64 = eq28_e364;
        let eq28_e367: f64 = (eq28_e364 * p.p1);
        let eq28_e367_d_n0: f64 = (eq28_e364_d_n0 * p.p1);
        let eq28_e367_d_n1: f64 = (eq28_e364_d_n1 * p.p1);
        let eq28_e367_d_n3: f64 = (eq28_e364_d_n3 * p.p1);
        let eq28_e367_d_n4: f64 = (eq28_e364_d_n4 * p.p1);
        let eq28_e367_d_n5: f64 = (eq28_e364_d_n5 * p.p1);
        let eq28_e367_d_n6: f64 = (eq28_e364_d_n6 * p.p1);
        let eq28_e367_d_n7: f64 = (eq28_e364_d_n7 * p.p1);
        let eq28_e367_d_n8: f64 = (eq28_e364_d_n8 * p.p1);
        let eq28_e367_d_n9: f64 = (eq28_e364_d_n9 * p.p1);
        let eq28_e367_d_n10: f64 = (eq28_e364_d_n10 * p.p1);
        let eq28_e367_d_n11: f64 = (eq28_e364_d_n11 * p.p1);
        let eq28_e367_q: f64 = (eq28_e365_q * p.p1);
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e367_d_n0, eq28_e367_d_n1, 0.0, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11, 0.0];
        let eq28_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e406_q: f64 = (nv12 - 0.0);
        let eq35_e407: f64 = (locals.var_taun * (nv12 - 0.0));
        let eq35_e407_d_n0: f64 = (locals.var_taun_dn0 * (nv12 - 0.0));
        let eq35_e407_d_n1: f64 = (locals.var_taun_dn1 * (nv12 - 0.0));
        let eq35_e407_d_n3: f64 = (locals.var_taun_dn3 * (nv12 - 0.0));
        let eq35_e407_d_n4: f64 = (locals.var_taun_dn4 * (nv12 - 0.0));
        let eq35_e407_d_n5: f64 = (locals.var_taun_dn5 * (nv12 - 0.0));
        let eq35_e407_d_n6: f64 = (locals.var_taun_dn6 * (nv12 - 0.0));
        let eq35_e407_d_n7: f64 = (locals.var_taun_dn7 * (nv12 - 0.0));
        let eq35_e407_d_n8: f64 = (locals.var_taun_dn8 * (nv12 - 0.0));
        let eq35_e407_d_n9: f64 = (locals.var_taun_dn9 * (nv12 - 0.0));
        let eq35_e407_d_n10: f64 = (locals.var_taun_dn10 * (nv12 - 0.0));
        let eq35_e407_d_n11: f64 = (locals.var_taun_dn11 * (nv12 - 0.0));
        let eq35_e407_q: f64 = (locals.var_taun * eq35_e406_q);
        let eq35_e407_q_d_n0: f64 = (locals.var_taun_dn0 * eq35_e406_q);
        let eq35_e407_q_d_n1: f64 = (locals.var_taun_dn1 * eq35_e406_q);
        let eq35_e407_q_d_n3: f64 = (locals.var_taun_dn3 * eq35_e406_q);
        let eq35_e407_q_d_n4: f64 = (locals.var_taun_dn4 * eq35_e406_q);
        let eq35_e407_q_d_n5: f64 = (locals.var_taun_dn5 * eq35_e406_q);
        let eq35_e407_q_d_n6: f64 = (locals.var_taun_dn6 * eq35_e406_q);
        let eq35_e407_q_d_n7: f64 = (locals.var_taun_dn7 * eq35_e406_q);
        let eq35_e407_q_d_n8: f64 = (locals.var_taun_dn8 * eq35_e406_q);
        let eq35_e407_q_d_n9: f64 = (locals.var_taun_dn9 * eq35_e406_q);
        let eq35_e407_q_d_n10: f64 = (locals.var_taun_dn10 * eq35_e406_q);
        let eq35_e407_q_d_n11: f64 = (locals.var_taun_dn11 * eq35_e406_q);
        let eq35_reactive_node_derivatives: [f64; 13] = [eq35_e407_q_d_n0, eq35_e407_q_d_n1, 0.0, eq35_e407_q_d_n3, eq35_e407_q_d_n4, eq35_e407_q_d_n5, eq35_e407_q_d_n6, eq35_e407_q_d_n7, eq35_e407_q_d_n8, eq35_e407_q_d_n9, eq35_e407_q_d_n10, eq35_e407_q_d_n11, locals.var_taun];
        let eq35_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
