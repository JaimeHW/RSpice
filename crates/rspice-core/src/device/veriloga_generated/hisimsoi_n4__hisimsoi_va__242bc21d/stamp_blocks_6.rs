#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
            let assign30850_e44626: f64 = (locals.var_tq__blk951 * locals.var_tq__blk951);
            let assign30850_e44629: f64 = (locals.var_tp__blk952 * locals.var_tp__blk952);
            let assign30850_e44631: f64 = (assign30850_e44629 * locals.var_tp__blk952);
            let assign30850_e44632: f64 = (assign30850_e44626 + assign30850_e44631);
            let assign30850_e44633: f64 = (assign30850_e44632).sqrt();
            (locals.var_t5__blk900, locals.var_t5__blk900_dn0, locals.var_t5__blk900_dn2, locals.var_t5__blk900_dn6, locals.var_t5__blk900_dn7, locals.var_t5__blk900_dn10, locals.var_t5__blk900_dn11, locals.var_t5__blk900_dn12, locals.var_t5__blk900_dn17, ) = (assign30850_e44633, ((((locals.var_tq__blk951_dn0 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn0)) + ((((locals.var_tp__blk952_dn0 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn0)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn0))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn2 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn2)) + ((((locals.var_tp__blk952_dn2 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn2)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn2))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn6 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn6)) + ((((locals.var_tp__blk952_dn6 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn6)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn6))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn7 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn7)) + ((((locals.var_tp__blk952_dn7 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn7)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn7))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn10 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn10)) + ((((locals.var_tp__blk952_dn10 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn10)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn10))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn11 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn11)) + ((((locals.var_tp__blk952_dn11 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn11)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn11))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn12 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn12)) + ((((locals.var_tp__blk952_dn12 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn12)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn12))) / (2.0 * assign30850_e44633)), ((((locals.var_tq__blk951_dn17 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn17)) + ((((locals.var_tp__blk952_dn17 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn17)) * locals.var_tp__blk952) + (assign30850_e44629 * locals.var_tp__blk952_dn17))) / (2.0 * assign30850_e44633)), );
            locals.var_t5__blk900_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
            let assign30860_e44648: f64 = (-locals.var_tq__blk951);
            let assign30860_e44650: f64 = (assign30860_e44648 + locals.var_t5__blk900);
            let assign30860_e44652: f64 = (assign30860_e44650).powf(0.3333333333333333);
            (locals.var_tu__blk953, locals.var_tu__blk953_dn0, locals.var_tu__blk953_dn2, locals.var_tu__blk953_dn6, locals.var_tu__blk953_dn7, locals.var_tu__blk953_dn10, locals.var_tu__blk953_dn11, locals.var_tu__blk953_dn12, locals.var_tu__blk953_dn17, ) = (assign30860_e44652, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn0) + locals.var_t5__blk900_dn0))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn0) + locals.var_t5__blk900_dn0) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn2) + locals.var_t5__blk900_dn2))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn2) + locals.var_t5__blk900_dn2) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn6) + locals.var_t5__blk900_dn6))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn6) + locals.var_t5__blk900_dn6) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn7) + locals.var_t5__blk900_dn7))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn7) + locals.var_t5__blk900_dn7) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn10) + locals.var_t5__blk900_dn10))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn10) + locals.var_t5__blk900_dn10) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn11) + locals.var_t5__blk900_dn11))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn11) + locals.var_t5__blk900_dn11) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn12) + locals.var_t5__blk900_dn12))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn12) + locals.var_t5__blk900_dn12) / assign30860_e44650))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30860_e44650).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn17) + locals.var_t5__blk900_dn17))) } } else { (assign30860_e44652 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn17) + locals.var_t5__blk900_dn17) / assign30860_e44650))) }, );
            locals.var_tu__blk953_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
            let assign30870_e44668: f64 = (locals.var_tq__blk951 + locals.var_t5__blk900);
            let assign30870_e44670: f64 = (assign30870_e44668).powf(0.3333333333333333);
            let assign30870_e44671: f64 = (-assign30870_e44670);
            (locals.var_tv__blk954, locals.var_tv__blk954_dn0, locals.var_tv__blk954_dn2, locals.var_tv__blk954_dn6, locals.var_tv__blk954_dn7, locals.var_tv__blk954_dn10, locals.var_tv__blk954_dn11, locals.var_tv__blk954_dn12, locals.var_tv__blk954_dn17, ) = (assign30870_e44671, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn0 + locals.var_t5__blk900_dn0))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn0 + locals.var_t5__blk900_dn0) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn2 + locals.var_t5__blk900_dn2))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn2 + locals.var_t5__blk900_dn2) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn6 + locals.var_t5__blk900_dn6))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn6 + locals.var_t5__blk900_dn6) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn7 + locals.var_t5__blk900_dn7))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn7 + locals.var_t5__blk900_dn7) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn10 + locals.var_t5__blk900_dn10))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn10 + locals.var_t5__blk900_dn10) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn11 + locals.var_t5__blk900_dn11))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn11 + locals.var_t5__blk900_dn11) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn12 + locals.var_t5__blk900_dn12))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn12 + locals.var_t5__blk900_dn12) / assign30870_e44668))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30870_e44668).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn17 + locals.var_t5__blk900_dn17))) } } else { (assign30870_e44670 * (0.3333333333333333 * ((locals.var_tq__blk951_dn17 + locals.var_t5__blk900_dn17) / assign30870_e44668))) }), );
            locals.var_tv__blk954_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
            let assign30880_e44687: f64 = (locals.var_tu__blk953 + locals.var_tv__blk954);
            let assign30880_e44691: f64 = (3.0 * locals.var_ta__blk947);
            let assign30880_e44692: f64 = (locals.var_tb__blk948 / assign30880_e44691);
            let assign30880_e44693: f64 = (assign30880_e44687 - assign30880_e44692);
            (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17, ) = (assign30880_e44693, (locals.var_tu__blk953_dn0 + locals.var_tv__blk954_dn0), (locals.var_tu__blk953_dn2 + locals.var_tv__blk954_dn2), (locals.var_tu__blk953_dn6 + locals.var_tv__blk954_dn6), (locals.var_tu__blk953_dn7 + locals.var_tv__blk954_dn7), (locals.var_tu__blk953_dn10 + locals.var_tv__blk954_dn10), (locals.var_tu__blk953_dn11 + locals.var_tv__blk954_dn11), (locals.var_tu__blk953_dn12 + locals.var_tv__blk954_dn12), (locals.var_tu__blk953_dn17 + locals.var_tv__blk954_dn17), );
            locals.var_tx__blk904_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
            let assign30890_e44709: f64 = (locals.var_tx__blk904 * locals.var_beta_inv);
            let assign30890_e44711: f64 = (assign30890_e44709 - locals.var_vxbgmtcl__blk921);
            (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17, ) = (assign30890_e44711, ((locals.var_tx__blk904_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_tx__blk904_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_tx__blk904_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_tx__blk904_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn7), (((locals.var_tx__blk904_dn10 * locals.var_beta_inv) + (locals.var_tx__blk904 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_tx__blk904_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_tx__blk904_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_tx__blk904_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn17), );
            locals.var_ps0_inia__blk946_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1006 != 0.0)) {
            let assign30900_e44728: f64 = (locals.var_ps0_inia__blk946 + locals.var_vxbgmtcl__blk921);
            let assign30900_e44729: f64 = (locals.var_beta * assign30900_e44728);
            (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17, ) = (assign30900_e44729, (locals.var_beta * (locals.var_ps0_inia__blk946_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign30900_e44728) + (locals.var_beta * (locals.var_ps0_inia__blk946_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk946_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn17 + locals.var_vxbgmtcl__blk921_dn17)), );
            locals.var_chi__blk943_rv = 0.0;
        }

        let assign30910_e44734: f64 = if p.p41 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1007 = assign30910_e44734;
        locals.var_guard1007_rv = 0.0;

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign30930_e44764: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
            let assign30930_e44766: f64 = (assign30930_e44764 + 0.1);
            (locals.var_vgpld_shift__blk955, locals.var_vgpld_shift__blk955_dn0, locals.var_vgpld_shift__blk955_dn2, locals.var_vgpld_shift__blk955_dn6, locals.var_vgpld_shift__blk955_dn7, locals.var_vgpld_shift__blk955_dn10, locals.var_vgpld_shift__blk955_dn11, locals.var_vgpld_shift__blk955_dn12, locals.var_vgpld_shift__blk955_dn17, ) = (assign30930_e44766, (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0), (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2), (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6), (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7), (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10), (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11), (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12), (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17), );
            locals.var_vgpld_shift__blk955_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign30940_e44782: f64 = (-locals.var_vxbgmtcl__blk921);
            let assign30940_e44783: f64 = (locals.var_beta * assign30940_e44782);
            let assign30940_e44784: f64 = (assign30940_e44783).exp();
            let assign30940_e44786: f64 = (assign30940_e44784 + 1e-50);
            (locals.var_exp_bvbs__blk962, locals.var_exp_bvbs__blk962_dn0, locals.var_exp_bvbs__blk962_dn2, locals.var_exp_bvbs__blk962_dn6, locals.var_exp_bvbs__blk962_dn7, locals.var_exp_bvbs__blk962_dn10, locals.var_exp_bvbs__blk962_dn11, locals.var_exp_bvbs__blk962_dn12, locals.var_exp_bvbs__blk962_dn17, ) = (assign30940_e44786, (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn0))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn2))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn6))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn7))), (assign30940_e44784 * ((locals.var_beta_dn10 * assign30940_e44782) + (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn10)))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn11))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn12))), (assign30940_e44784 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn17))), );
            locals.var_exp_bvbs__blk962_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign30950_e44802: f64 = (locals.var_nin / locals.var_mks_nover);
            (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17, ) = (assign30950_e44802, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover), );
            locals.var_t0__blk895_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign30960_e44818: f64 = (locals.var_t0__blk895 * locals.var_t0__blk895);
            (locals.var_cnst1over__blk956, locals.var_cnst1over__blk956_dn0, locals.var_cnst1over__blk956_dn2, locals.var_cnst1over__blk956_dn6, locals.var_cnst1over__blk956_dn7, locals.var_cnst1over__blk956_dn10, locals.var_cnst1over__blk956_dn11, locals.var_cnst1over__blk956_dn12, locals.var_cnst1over__blk956_dn17, ) = (assign30960_e44818, ((locals.var_t0__blk895_dn0 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn0)), ((locals.var_t0__blk895_dn2 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn2)), ((locals.var_t0__blk895_dn6 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn6)), ((locals.var_t0__blk895_dn7 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn7)), ((locals.var_t0__blk895_dn10 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn10)), ((locals.var_t0__blk895_dn11 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn11)), ((locals.var_t0__blk895_dn12 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn12)), ((locals.var_t0__blk895_dn17 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn17)), );
            locals.var_cnst1over__blk956_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign30970_e44834: f64 = (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962);
            (locals.var_gammachi__blk957, locals.var_gammachi__blk957_dn0, locals.var_gammachi__blk957_dn2, locals.var_gammachi__blk957_dn6, locals.var_gammachi__blk957_dn7, locals.var_gammachi__blk957_dn10, locals.var_gammachi__blk957_dn11, locals.var_gammachi__blk957_dn12, locals.var_gammachi__blk957_dn17, ) = (assign30970_e44834, ((locals.var_cnst1over__blk956_dn0 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn0)), ((locals.var_cnst1over__blk956_dn2 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn2)), ((locals.var_cnst1over__blk956_dn6 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn6)), ((locals.var_cnst1over__blk956_dn7 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn7)), ((locals.var_cnst1over__blk956_dn10 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn10)), ((locals.var_cnst1over__blk956_dn11 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn11)), ((locals.var_cnst1over__blk956_dn12 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn12)), ((locals.var_cnst1over__blk956_dn17 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn17)), );
            locals.var_gammachi__blk957_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign30980_e44850: f64 = (locals.var_beta2 * locals.var_fac1p2__blk930);
            (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17, ) = (assign30980_e44850, (locals.var_beta2 * locals.var_fac1p2__blk930_dn0), (locals.var_beta2 * locals.var_fac1p2__blk930_dn2), (locals.var_beta2 * locals.var_fac1p2__blk930_dn6), (locals.var_beta2 * locals.var_fac1p2__blk930_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk930) + (locals.var_beta2 * locals.var_fac1p2__blk930_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk930_dn11), (locals.var_beta2 * locals.var_fac1p2__blk930_dn12), (locals.var_beta2 * locals.var_fac1p2__blk930_dn17), );
            locals.var_t0__blk895_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign30990_e44866: f64 = (locals.var_beta * locals.var_vgpld_shift__blk955);
            (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17, ) = (assign30990_e44866, (locals.var_beta * locals.var_vgpld_shift__blk955_dn0), (locals.var_beta * locals.var_vgpld_shift__blk955_dn2), (locals.var_beta * locals.var_vgpld_shift__blk955_dn6), (locals.var_beta * locals.var_vgpld_shift__blk955_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift__blk955) + (locals.var_beta * locals.var_vgpld_shift__blk955_dn10)), (locals.var_beta * locals.var_vgpld_shift__blk955_dn11), (locals.var_beta * locals.var_vgpld_shift__blk955_dn12), (locals.var_beta * locals.var_vgpld_shift__blk955_dn17), );
            locals.var_psi__blk958_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31000_e44882: f64 = (locals.var_gammachi__blk957 * locals.var_t0__blk895);
            let assign31000_e44885: f64 = (locals.var_psi__blk958 * locals.var_psi__blk958);
            let assign31000_e44886: f64 = (assign31000_e44882 + assign31000_e44885);
            let assign31000_e44887: f64 = (assign31000_e44886).ln();
            let assign31000_e44890: f64 = (locals.var_cnst1over__blk956 * locals.var_t0__blk895);
            let assign31000_e44891: f64 = (assign31000_e44890).ln();
            let assign31000_e44892: f64 = (assign31000_e44887 - assign31000_e44891);
            let assign31000_e44895: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk921);
            let assign31000_e44896: f64 = (assign31000_e44892 + assign31000_e44895);
            (locals.var_chi_1__blk959, locals.var_chi_1__blk959_dn0, locals.var_chi_1__blk959_dn2, locals.var_chi_1__blk959_dn6, locals.var_chi_1__blk959_dn7, locals.var_chi_1__blk959_dn10, locals.var_chi_1__blk959_dn11, locals.var_chi_1__blk959_dn12, locals.var_chi_1__blk959_dn17, ) = (assign31000_e44896, ((((((locals.var_gammachi__blk957_dn0 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn0)) + ((locals.var_psi__blk958_dn0 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn0))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn0 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn0)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn0)), ((((((locals.var_gammachi__blk957_dn2 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn2)) + ((locals.var_psi__blk958_dn2 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn2))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn2 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn2)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn2)), ((((((locals.var_gammachi__blk957_dn6 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn6)) + ((locals.var_psi__blk958_dn6 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn6))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn6 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn6)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn6)), ((((((locals.var_gammachi__blk957_dn7 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn7)) + ((locals.var_psi__blk958_dn7 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn7))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn7 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn7)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn7)), ((((((locals.var_gammachi__blk957_dn10 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn10)) + ((locals.var_psi__blk958_dn10 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn10))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn10 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn10)) / assign31000_e44890)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk921) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn10))), ((((((locals.var_gammachi__blk957_dn11 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn11)) + ((locals.var_psi__blk958_dn11 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn11))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn11 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn11)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn11)), ((((((locals.var_gammachi__blk957_dn12 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn12)) + ((locals.var_psi__blk958_dn12 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn12))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn12 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn12)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn12)), ((((((locals.var_gammachi__blk957_dn17 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn17)) + ((locals.var_psi__blk958_dn17 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn17))) / assign31000_e44886) - (((locals.var_cnst1over__blk956_dn17 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn17)) / assign31000_e44890)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn17)), );
            locals.var_chi_1__blk959_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31010_e44912: f64 = (locals.var_psi__blk958 - locals.var_chi_1__blk959);
            let assign31010_e44914: f64 = (assign31010_e44912 - 1.0);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign31010_e44914, (locals.var_psi__blk958_dn0 - locals.var_chi_1__blk959_dn0), (locals.var_psi__blk958_dn2 - locals.var_chi_1__blk959_dn2), (locals.var_psi__blk958_dn6 - locals.var_chi_1__blk959_dn6), (locals.var_psi__blk958_dn7 - locals.var_chi_1__blk959_dn7), (locals.var_psi__blk958_dn10 - locals.var_chi_1__blk959_dn10), (locals.var_psi__blk958_dn11 - locals.var_chi_1__blk959_dn11), (locals.var_psi__blk958_dn12 - locals.var_chi_1__blk959_dn12), (locals.var_psi__blk958_dn17 - locals.var_chi_1__blk959_dn17), );
            locals.var_tmf1_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31020_e44930: f64 = (4.0 * locals.var_psi__blk958);
            let assign31020_e44932: f64 = assign31020_e44930;
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31020_e44932, (4.0 * locals.var_psi__blk958_dn0), (4.0 * locals.var_psi__blk958_dn2), (4.0 * locals.var_psi__blk958_dn6), (4.0 * locals.var_psi__blk958_dn7), (4.0 * locals.var_psi__blk958_dn10), (4.0 * locals.var_psi__blk958_dn11), (4.0 * locals.var_psi__blk958_dn12), (4.0 * locals.var_psi__blk958_dn17), );
            locals.var_tmf2_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let (assign31030_e44952, assign31030_e44952_d_n0, assign31030_e44952_d_n2, assign31030_e44952_d_n6, assign31030_e44952_d_n7, assign31030_e44952_d_n10, assign31030_e44952_d_n11, assign31030_e44952_d_n12, assign31030_e44952_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign31030_e44951: f64 = (-locals.var_tmf2);
        (assign31030_e44951, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31030_e44952, assign31030_e44952_d_n0, assign31030_e44952_d_n2, assign31030_e44952_d_n6, assign31030_e44952_d_n7, assign31030_e44952_d_n10, assign31030_e44952_d_n11, assign31030_e44952_d_n12, assign31030_e44952_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31040_e44968: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign31040_e44970: f64 = (assign31040_e44968 + locals.var_tmf2);
            let assign31040_e44971: f64 = (assign31040_e44970).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31040_e44971, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31040_e44971)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31040_e44971)), );
            locals.var_tmf2_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31050_e44989: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign31050_e44990: f64 = (1.0 + assign31050_e44989);
            let assign31050_e44991: f64 = (0.5 * assign31050_e44990);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31050_e44991, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t1__blk896_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31060_e45010: f64 = 2.0;
            let assign31060_e45011: f64 = (locals.var_tmf1 + assign31060_e45010);
            let assign31060_e45013: f64 = (assign31060_e45011 / locals.var_tmf2);
            let assign31060_e45014: f64 = (1.0 - assign31060_e45013);
            let assign31060_e45015: f64 = (0.5 * assign31060_e45014);
            (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17, ) = (assign31060_e45015, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31060_e45011 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))), );
            locals.var_t2__blk897_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31070_e45033: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign31070_e45034: f64 = (0.5 * assign31070_e45033);
            let assign31070_e45035: f64 = (locals.var_psi__blk958 - assign31070_e45034);
            (locals.var_chi_1__blk959, locals.var_chi_1__blk959_dn0, locals.var_chi_1__blk959_dn2, locals.var_chi_1__blk959_dn6, locals.var_chi_1__blk959_dn7, locals.var_chi_1__blk959_dn10, locals.var_chi_1__blk959_dn11, locals.var_chi_1__blk959_dn12, locals.var_chi_1__blk959_dn17, ) = (assign31070_e45035, (locals.var_psi__blk958_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi__blk958_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi__blk958_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi__blk958_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi__blk958_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi__blk958_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi__blk958_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi__blk958_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_chi_1__blk959_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31080_e45051: f64 = (locals.var_psi__blk958 - locals.var_chi_1__blk959);
            (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17, ) = (assign31080_e45051, (locals.var_psi__blk958_dn0 - locals.var_chi_1__blk959_dn0), (locals.var_psi__blk958_dn2 - locals.var_chi_1__blk959_dn2), (locals.var_psi__blk958_dn6 - locals.var_chi_1__blk959_dn6), (locals.var_psi__blk958_dn7 - locals.var_chi_1__blk959_dn7), (locals.var_psi__blk958_dn10 - locals.var_chi_1__blk959_dn10), (locals.var_psi__blk958_dn11 - locals.var_chi_1__blk959_dn11), (locals.var_psi__blk958_dn12 - locals.var_chi_1__blk959_dn12), (locals.var_psi__blk958_dn17 - locals.var_chi_1__blk959_dn17), );
            locals.var_psi__blk958_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31090_e45068: f64 = (locals.var_beta * 0.1);
            let assign31090_e45069: f64 = (locals.var_psi__blk958 + assign31090_e45068);
            (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17, ) = (assign31090_e45069, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, (locals.var_psi__blk958_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17, );
            locals.var_psi__blk958_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31100_e45085: f64 = (locals.var_gammachi__blk957 * locals.var_t0__blk895);
            let assign31100_e45088: f64 = (locals.var_psi__blk958 * locals.var_psi__blk958);
            let assign31100_e45089: f64 = (assign31100_e45085 + assign31100_e45088);
            let assign31100_e45090: f64 = (assign31100_e45089).ln();
            let assign31100_e45093: f64 = (locals.var_cnst1over__blk956 * locals.var_t0__blk895);
            let assign31100_e45094: f64 = (assign31100_e45093).ln();
            let assign31100_e45095: f64 = (assign31100_e45090 - assign31100_e45094);
            let assign31100_e45098: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk921);
            let assign31100_e45099: f64 = (assign31100_e45095 + assign31100_e45098);
            (locals.var_chi_b__blk960, locals.var_chi_b__blk960_dn0, locals.var_chi_b__blk960_dn2, locals.var_chi_b__blk960_dn6, locals.var_chi_b__blk960_dn7, locals.var_chi_b__blk960_dn10, locals.var_chi_b__blk960_dn11, locals.var_chi_b__blk960_dn12, locals.var_chi_b__blk960_dn17, ) = (assign31100_e45099, ((((((locals.var_gammachi__blk957_dn0 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn0)) + ((locals.var_psi__blk958_dn0 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn0))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn0 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn0)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn0)), ((((((locals.var_gammachi__blk957_dn2 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn2)) + ((locals.var_psi__blk958_dn2 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn2))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn2 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn2)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn2)), ((((((locals.var_gammachi__blk957_dn6 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn6)) + ((locals.var_psi__blk958_dn6 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn6))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn6 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn6)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn6)), ((((((locals.var_gammachi__blk957_dn7 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn7)) + ((locals.var_psi__blk958_dn7 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn7))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn7 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn7)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn7)), ((((((locals.var_gammachi__blk957_dn10 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn10)) + ((locals.var_psi__blk958_dn10 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn10))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn10 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn10)) / assign31100_e45093)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk921) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn10))), ((((((locals.var_gammachi__blk957_dn11 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn11)) + ((locals.var_psi__blk958_dn11 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn11))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn11 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn11)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn11)), ((((((locals.var_gammachi__blk957_dn12 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn12)) + ((locals.var_psi__blk958_dn12 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn12))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn12 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn12)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn12)), ((((((locals.var_gammachi__blk957_dn17 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn17)) + ((locals.var_psi__blk958_dn17 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn17))) / assign31100_e45089) - (((locals.var_cnst1over__blk956_dn17 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn17)) / assign31100_e45093)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn17)), );
            locals.var_chi_b__blk960_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            (locals.var_chi_a__blk961, locals.var_chi_a__blk961_dn0, locals.var_chi_a__blk961_dn2, locals.var_chi_a__blk961_dn6, locals.var_chi_a__blk961_dn7, locals.var_chi_a__blk961_dn10, locals.var_chi_a__blk961_dn11, locals.var_chi_a__blk961_dn12, locals.var_chi_a__blk961_dn17, ) = (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17, );
            locals.var_chi_a__blk961_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31120_e45129: f64 = (locals.var_chi_b__blk960 - locals.var_chi_a__blk961);
            let assign31120_e45132: f64 = (0.0008 * 75.0);
            let assign31120_e45133: f64 = (assign31120_e45129 - assign31120_e45132);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign31120_e45133, (locals.var_chi_b__blk960_dn0 - locals.var_chi_a__blk961_dn0), (locals.var_chi_b__blk960_dn2 - locals.var_chi_a__blk961_dn2), (locals.var_chi_b__blk960_dn6 - locals.var_chi_a__blk961_dn6), (locals.var_chi_b__blk960_dn7 - locals.var_chi_a__blk961_dn7), (locals.var_chi_b__blk960_dn10 - locals.var_chi_a__blk961_dn10), (locals.var_chi_b__blk960_dn11 - locals.var_chi_a__blk961_dn11), (locals.var_chi_b__blk960_dn12 - locals.var_chi_a__blk961_dn12), (locals.var_chi_b__blk960_dn17 - locals.var_chi_a__blk961_dn17), );
            locals.var_tmf1_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31130_e45149: f64 = (4.0 * locals.var_chi_b__blk960);
            let assign31130_e45152: f64 = (0.0008 * 75.0);
            let assign31130_e45153: f64 = (assign31130_e45149 * assign31130_e45152);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31130_e45153, ((4.0 * locals.var_chi_b__blk960_dn0) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn2) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn6) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn7) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn10) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn11) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn12) * assign31130_e45152), ((4.0 * locals.var_chi_b__blk960_dn17) * assign31130_e45152), );
            locals.var_tmf2_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let (assign31140_e45173, assign31140_e45173_d_n0, assign31140_e45173_d_n2, assign31140_e45173_d_n6, assign31140_e45173_d_n7, assign31140_e45173_d_n10, assign31140_e45173_d_n11, assign31140_e45173_d_n12, assign31140_e45173_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign31140_e45172: f64 = (-locals.var_tmf2);
        (assign31140_e45172, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31140_e45173, assign31140_e45173_d_n0, assign31140_e45173_d_n2, assign31140_e45173_d_n6, assign31140_e45173_d_n7, assign31140_e45173_d_n10, assign31140_e45173_d_n11, assign31140_e45173_d_n12, assign31140_e45173_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31150_e45189: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign31150_e45191: f64 = (assign31150_e45189 + locals.var_tmf2);
            let assign31150_e45192: f64 = (assign31150_e45191).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31150_e45192, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31150_e45192)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31150_e45192)), );
            locals.var_tmf2_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31160_e45210: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign31160_e45211: f64 = (1.0 + assign31160_e45210);
            let assign31160_e45212: f64 = (0.5 * assign31160_e45211);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31160_e45212, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t1__blk896_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31170_e45231: f64 = (2.0 * 0.0008);
            let assign31170_e45233: f64 = (assign31170_e45231 * 75.0);
            let assign31170_e45234: f64 = (locals.var_tmf1 + assign31170_e45233);
            let assign31170_e45236: f64 = (assign31170_e45234 / locals.var_tmf2);
            let assign31170_e45237: f64 = (1.0 - assign31170_e45236);
            let assign31170_e45238: f64 = (0.5 * assign31170_e45237);
            (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17, ) = (assign31170_e45238, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31170_e45234 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))), );
            locals.var_t2__blk897_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1007 != 0.0)) {
            let assign31180_e45256: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign31180_e45257: f64 = (0.5 * assign31180_e45256);
            let assign31180_e45258: f64 = (locals.var_chi_b__blk960 - assign31180_e45257);
            (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17, ) = (assign31180_e45258, (locals.var_chi_b__blk960_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b__blk960_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b__blk960_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b__blk960_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b__blk960_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b__blk960_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b__blk960_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b__blk960_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_chi__blk943_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
            let assign31190_e45272: f64 = (locals.var_chi__blk943 / locals.var_beta);
            let assign31190_e45274: f64 = (assign31190_e45272 - locals.var_vxbgmtcl__blk921);
            (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17, ) = (assign31190_e45274, ((locals.var_chi__blk943_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_chi__blk943_dn10 * locals.var_beta) - (locals.var_chi__blk943 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn17), );
            locals.var_ps0ld__blk945_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
            let assign31200_e45288: f64 = (locals.var_chi__blk943 - 1.0);
            let assign31200_e45290: f64 = (-locals.var_chi__blk943);
            let assign31200_e45291: f64 = (assign31200_e45290).exp();
            let assign31200_e45292: f64 = (assign31200_e45288 + assign31200_e45291);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31200_e45292, (locals.var_chi__blk943_dn0 + (assign31200_e45291 * (-locals.var_chi__blk943_dn0))), (locals.var_chi__blk943_dn2 + (assign31200_e45291 * (-locals.var_chi__blk943_dn2))), (locals.var_chi__blk943_dn6 + (assign31200_e45291 * (-locals.var_chi__blk943_dn6))), (locals.var_chi__blk943_dn7 + (assign31200_e45291 * (-locals.var_chi__blk943_dn7))), (locals.var_chi__blk943_dn10 + (assign31200_e45291 * (-locals.var_chi__blk943_dn10))), (locals.var_chi__blk943_dn11 + (assign31200_e45291 * (-locals.var_chi__blk943_dn11))), (locals.var_chi__blk943_dn12 + (assign31200_e45291 * (-locals.var_chi__blk943_dn12))), (locals.var_chi__blk943_dn17 + (assign31200_e45291 * (-locals.var_chi__blk943_dn17))), );
            locals.var_t1__blk896_rv = 0.0;
        }

        let assign31210_e45298: f64 = (10.0 * 2.220446049250313e-16);
        let assign31210_e45299: f64 = if locals.var_t1__blk896 < assign31210_e45298 { 1.0 } else { 0.0 };
        locals.var_guard1008 = assign31210_e45299;
        locals.var_guard1008_rv = 0.0;

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1008 != 0.0)) {
            let assign31220_e45313: f64 = (10.0 * 2.220446049250313e-16);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31220_e45313, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk896_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
            let assign31230_e45326: f64 = (locals.var_t1__blk896).sqrt();
            (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17, ) = (assign31230_e45326, (locals.var_t1__blk896_dn0 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn2 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn6 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn7 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn10 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn11 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn12 / (2.0 * assign31230_e45326)), (locals.var_t1__blk896_dn17 / (2.0 * assign31230_e45326)), );
            locals.var_t2__blk897_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
            let assign31240_e45340: f64 = (locals.var_cnst0over__blk928 * locals.var_t2__blk897);
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17, ) = (assign31240_e45340, ((locals.var_cnst0over__blk928_dn0 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn0)), ((locals.var_cnst0over__blk928_dn2 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn2)), ((locals.var_cnst0over__blk928_dn6 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn6)), ((locals.var_cnst0over__blk928_dn7 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn7)), ((locals.var_cnst0over__blk928_dn10 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn10)), ((locals.var_cnst0over__blk928_dn11 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn11)), ((locals.var_cnst0over__blk928_dn12 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn12)), ((locals.var_cnst0over__blk928_dn17 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn17)), );
            locals.var_qbuld_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
            let assign31250_e45355: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
            let assign31250_e45356: f64 = (locals.var_cox0__blk906 * assign31250_e45355);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17, ) = (assign31250_e45356, (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17)), );
            locals.var_qsuld_rv = 0.0;
        }

        let assign31260_e45361: f64 = if p.p41 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1009 = assign31260_e45361;
        locals.var_guard1009_rv = 0.0;

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31270_e45375: f64 = (-locals.var_vxbgmtcl__blk921);
            let assign31270_e45376: f64 = (locals.var_beta * assign31270_e45375);
            let assign31270_e45377: f64 = (assign31270_e45376).exp();
            (locals.var_exp_bvbs__blk962, locals.var_exp_bvbs__blk962_dn0, locals.var_exp_bvbs__blk962_dn2, locals.var_exp_bvbs__blk962_dn6, locals.var_exp_bvbs__blk962_dn7, locals.var_exp_bvbs__blk962_dn10, locals.var_exp_bvbs__blk962_dn11, locals.var_exp_bvbs__blk962_dn12, locals.var_exp_bvbs__blk962_dn17, ) = (assign31270_e45377, (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn0))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn2))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn6))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn7))), (assign31270_e45377 * ((locals.var_beta_dn10 * assign31270_e45375) + (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn10)))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn11))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn12))), (assign31270_e45377 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn17))), );
            locals.var_exp_bvbs__blk962_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31280_e45393: f64 = (locals.var_nin / locals.var_mks_nover);
            (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17, ) = (assign31280_e45393, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover), );
            locals.var_t0__blk895_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31290_e45409: f64 = (locals.var_t0__blk895 * locals.var_t0__blk895);
            (locals.var_cnst1over__blk956, locals.var_cnst1over__blk956_dn0, locals.var_cnst1over__blk956_dn2, locals.var_cnst1over__blk956_dn6, locals.var_cnst1over__blk956_dn7, locals.var_cnst1over__blk956_dn10, locals.var_cnst1over__blk956_dn11, locals.var_cnst1over__blk956_dn12, locals.var_cnst1over__blk956_dn17, ) = (assign31290_e45409, ((locals.var_t0__blk895_dn0 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn0)), ((locals.var_t0__blk895_dn2 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn2)), ((locals.var_t0__blk895_dn6 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn6)), ((locals.var_t0__blk895_dn7 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn7)), ((locals.var_t0__blk895_dn10 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn10)), ((locals.var_t0__blk895_dn11 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn11)), ((locals.var_t0__blk895_dn12 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn12)), ((locals.var_t0__blk895_dn17 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn17)), );
            locals.var_cnst1over__blk956_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31300_e45425: f64 = (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962);
            (locals.var_cfs1__blk971, locals.var_cfs1__blk971_dn0, locals.var_cfs1__blk971_dn2, locals.var_cfs1__blk971_dn6, locals.var_cfs1__blk971_dn7, locals.var_cfs1__blk971_dn10, locals.var_cfs1__blk971_dn11, locals.var_cfs1__blk971_dn12, locals.var_cfs1__blk971_dn17, ) = (assign31300_e45425, ((locals.var_cnst1over__blk956_dn0 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn0)), ((locals.var_cnst1over__blk956_dn2 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn2)), ((locals.var_cnst1over__blk956_dn6 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn6)), ((locals.var_cnst1over__blk956_dn7 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn7)), ((locals.var_cnst1over__blk956_dn10 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn10)), ((locals.var_cnst1over__blk956_dn11 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn11)), ((locals.var_cnst1over__blk956_dn12 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn12)), ((locals.var_cnst1over__blk956_dn17 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn17)), );
            locals.var_cfs1__blk971_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            locals.var_flg_conv__blk918 = 0.0;
            locals.var_flg_conv__blk918_rv = 0.0;
            (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_fs01__blk965_rv = 0.0;
            (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_fs02__blk969_rv = 0.0;
            locals.var_lp_s0 = 1.0;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign31350_loop_guard: usize = 0;
        while {
            let assign31350_cond_e45498: f64 = (2.0 * 20.0);
            let assign31350_cond_e45500: f64 = (assign31350_cond_e45498 + 1.0);
            let assign31350_cond_e45502: f64 = if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_lp_s0 <= assign31350_cond_e45500)) { 1.0 } else { 0.0 };
            assign31350_cond_e45502 != 0.0
        } {
            assign31350_loop_guard += 1;
            assert!(assign31350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
                (locals.var_fb__blk967, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
                locals.var_fb__blk967_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
                let assign31350_body1_e45531: f64 = (locals.var_ps0ld__blk945 + locals.var_vxbgmtcl__blk921);
                let assign31350_body1_e45532: f64 = (locals.var_beta * assign31350_body1_e45531);
                (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17, ) = (assign31350_body1_e45532, (locals.var_beta * (locals.var_ps0ld__blk945_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0ld__blk945_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0ld__blk945_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0ld__blk945_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign31350_body1_e45531) + (locals.var_beta * (locals.var_ps0ld__blk945_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0ld__blk945_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0ld__blk945_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0ld__blk945_dn17 + locals.var_vxbgmtcl__blk921_dn17)), );
                locals.var_chi__blk943_rv = 0.0;
            }
            let assign31350_body2_e45537: f64 = if locals.var_chi__blk943 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1010 = assign31350_body2_e45537;
            locals.var_guard1010_rv = 0.0;
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body3_e45553: f64 = (locals.var_chi__blk943 * locals.var_chi__blk943);
                let assign31350_body3_e45555: f64 = (assign31350_body3_e45553 * locals.var_chi__blk943);
                let assign31350_body3_e45559: f64 = (-0.07053654284009761);
                let assign31350_body3_e45562: f64 = (locals.var_chi__blk943 * 0.006115288895133179);
                let assign31350_body3_e45563: f64 = (assign31350_body3_e45559 + assign31350_body3_e45562);
                let assign31350_body3_e45564: f64 = (locals.var_chi__blk943 * assign31350_body3_e45563);
                let assign31350_body3_e45565: f64 = (0.29693154855771 + assign31350_body3_e45564);
                let assign31350_body3_e45566: f64 = (assign31350_body3_e45555 * assign31350_body3_e45565);
                (locals.var_fi__blk963, locals.var_fi__blk963_dn0, locals.var_fi__blk963_dn2, locals.var_fi__blk963_dn6, locals.var_fi__blk963_dn7, locals.var_fi__blk963_dn10, locals.var_fi__blk963_dn11, locals.var_fi__blk963_dn12, locals.var_fi__blk963_dn17, ) = (assign31350_body3_e45566, ((((((locals.var_chi__blk943_dn0 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn0)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn0)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn0 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn2 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn2)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn2)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn2 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn6 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn6)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn6)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn6 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn7 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn7)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn7)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn7 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn10 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn10)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn10)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn10 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn11 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn11)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn11)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn11 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn12 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn12)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn12)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn12 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn17 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn17)) * locals.var_chi__blk943) + (assign31350_body3_e45553 * locals.var_chi__blk943_dn17)) * assign31350_body3_e45565) + (assign31350_body3_e45555 * ((locals.var_chi__blk943_dn17 * assign31350_body3_e45563) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn17 * 0.006115288895133179))))), );
                locals.var_fi__blk963_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body4_e45584: f64 = (locals.var_chi__blk943 * locals.var_chi__blk943);
                let assign31350_body4_e45587: f64 = (3.0 * 0.29693154855771);
                let assign31350_body4_e45591: f64 = (-0.07053654284009761);
                let assign31350_body4_e45592: f64 = (4.0 * assign31350_body4_e45591);
                let assign31350_body4_e45595: f64 = (locals.var_chi__blk943 * 5.0);
                let assign31350_body4_e45597: f64 = (assign31350_body4_e45595 * 0.006115288895133179);
                let assign31350_body4_e45598: f64 = (assign31350_body4_e45592 + assign31350_body4_e45597);
                let assign31350_body4_e45599: f64 = (locals.var_chi__blk943 * assign31350_body4_e45598);
                let assign31350_body4_e45600: f64 = (assign31350_body4_e45587 + assign31350_body4_e45599);
                let assign31350_body4_e45601: f64 = (assign31350_body4_e45584 * assign31350_body4_e45600);
                (locals.var_fi_dchi__blk964, locals.var_fi_dchi__blk964_dn0, locals.var_fi_dchi__blk964_dn2, locals.var_fi_dchi__blk964_dn6, locals.var_fi_dchi__blk964_dn7, locals.var_fi_dchi__blk964_dn10, locals.var_fi_dchi__blk964_dn11, locals.var_fi_dchi__blk964_dn12, locals.var_fi_dchi__blk964_dn17, ) = (assign31350_body4_e45601, ((((locals.var_chi__blk943_dn0 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn0)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn0 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn2 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn2)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn2 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn6 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn6)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn6 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn7 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn7)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn7 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn10 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn10)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn10 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn11 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn11)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn11 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn12 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn12)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn12 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn17 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn17)) * assign31350_body4_e45600) + (assign31350_body4_e45584 * ((locals.var_chi__blk943_dn17 * assign31350_body4_e45598) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * 5.0) * 0.006115288895133179))))), );
                locals.var_fi_dchi__blk964_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body5_e45619: f64 = (locals.var_cfs1__blk971 * locals.var_fi__blk963);
                let assign31350_body5_e45621: f64 = (assign31350_body5_e45619 * locals.var_fi__blk963);
                (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17, ) = (assign31350_body5_e45621, ((((locals.var_cfs1__blk971_dn0 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn0)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn0)), ((((locals.var_cfs1__blk971_dn2 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn2)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn2)), ((((locals.var_cfs1__blk971_dn6 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn6)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn6)), ((((locals.var_cfs1__blk971_dn7 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn7)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn7)), ((((locals.var_cfs1__blk971_dn10 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn10)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn10)), ((((locals.var_cfs1__blk971_dn11 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn11)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn11)), ((((locals.var_cfs1__blk971_dn12 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn12)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn12)), ((((locals.var_cfs1__blk971_dn17 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn17)) * locals.var_fi__blk963) + (assign31350_body5_e45619 * locals.var_fi__blk963_dn17)), );
                locals.var_fs01__blk965_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body6_e45639: f64 = (locals.var_cfs1__blk971 * locals.var_beta);
                let assign31350_body6_e45641: f64 = (assign31350_body6_e45639 * 2.0);
                let assign31350_body6_e45643: f64 = (assign31350_body6_e45641 * locals.var_fi__blk963);
                let assign31350_body6_e45645: f64 = (assign31350_body6_e45643 * locals.var_fi_dchi__blk964);
                (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17, ) = (assign31350_body6_e45645, ((((((locals.var_cfs1__blk971_dn0 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn0)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn0)), ((((((locals.var_cfs1__blk971_dn2 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn2)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn2)), ((((((locals.var_cfs1__blk971_dn6 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn6)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn6)), ((((((locals.var_cfs1__blk971_dn7 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn7)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn7)), (((((((locals.var_cfs1__blk971_dn10 * locals.var_beta) + (locals.var_cfs1__blk971 * locals.var_beta_dn10)) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn10)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn10)), ((((((locals.var_cfs1__blk971_dn11 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn11)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn11)), ((((((locals.var_cfs1__blk971_dn12 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn12)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn12)), ((((((locals.var_cfs1__blk971_dn17 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign31350_body6_e45641 * locals.var_fi__blk963_dn17)) * locals.var_fi_dchi__blk964) + (assign31350_body6_e45643 * locals.var_fi_dchi__blk964_dn17)), );
                locals.var_fs01_dps0__blk966_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body7_e45665: f64 = (-0.117851130197758);
                let assign31350_body7_e45670: f64 = (-0.00163730162779191);
                let assign31350_body7_e45673: f64 = (locals.var_chi__blk943 * 6.36964918866352e-5);
                let assign31350_body7_e45674: f64 = (assign31350_body7_e45670 + assign31350_body7_e45673);
                let assign31350_body7_e45675: f64 = (locals.var_chi__blk943 * assign31350_body7_e45674);
                let assign31350_body7_e45676: f64 = (0.0178800506338833 + assign31350_body7_e45675);
                let assign31350_body7_e45677: f64 = (locals.var_chi__blk943 * assign31350_body7_e45676);
                let assign31350_body7_e45678: f64 = (assign31350_body7_e45665 + assign31350_body7_e45677);
                let assign31350_body7_e45679: f64 = (locals.var_chi__blk943 * assign31350_body7_e45678);
                let assign31350_body7_e45680: f64 = (0.707106781186548 + assign31350_body7_e45679);
                let assign31350_body7_e45681: f64 = (locals.var_chi__blk943 * assign31350_body7_e45680);
                (locals.var_fb__blk967, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17, ) = (assign31350_body7_e45681, ((locals.var_chi__blk943_dn0 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn2 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn6 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn7 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn10 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn11 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn12 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn17 * assign31350_body7_e45680) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body7_e45678) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body7_e45676) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body7_e45674) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn17 * 6.36964918866352e-5))))))))), );
                locals.var_fb__blk967_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body8_e45701: f64 = (-0.117851130197758);
                let assign31350_body8_e45702: f64 = (2.0 * assign31350_body8_e45701);
                let assign31350_body8_e45706: f64 = (3.0 * 0.0178800506338833);
                let assign31350_body8_e45710: f64 = (-0.00163730162779191);
                let assign31350_body8_e45711: f64 = (4.0 * assign31350_body8_e45710);
                let assign31350_body8_e45714: f64 = (locals.var_chi__blk943 * 5.0);
                let assign31350_body8_e45716: f64 = (assign31350_body8_e45714 * 6.36964918866352e-5);
                let assign31350_body8_e45717: f64 = (assign31350_body8_e45711 + assign31350_body8_e45716);
                let assign31350_body8_e45718: f64 = (locals.var_chi__blk943 * assign31350_body8_e45717);
                let assign31350_body8_e45719: f64 = (assign31350_body8_e45706 + assign31350_body8_e45718);
                let assign31350_body8_e45720: f64 = (locals.var_chi__blk943 * assign31350_body8_e45719);
                let assign31350_body8_e45721: f64 = (assign31350_body8_e45702 + assign31350_body8_e45720);
                let assign31350_body8_e45722: f64 = (locals.var_chi__blk943 * assign31350_body8_e45721);
                let assign31350_body8_e45723: f64 = (0.707106781186548 + assign31350_body8_e45722);
                (locals.var_fb_dchi__blk968, locals.var_fb_dchi__blk968_dn0, locals.var_fb_dchi__blk968_dn2, locals.var_fb_dchi__blk968_dn6, locals.var_fb_dchi__blk968_dn7, locals.var_fb_dchi__blk968_dn10, locals.var_fb_dchi__blk968_dn11, locals.var_fb_dchi__blk968_dn12, locals.var_fb_dchi__blk968_dn17, ) = (assign31350_body8_e45723, ((locals.var_chi__blk943_dn0 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn2 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn6 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn7 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn10 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn11 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn12 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn17 * assign31350_body8_e45721) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body8_e45719) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign31350_body8_e45717) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * 5.0) * 6.36964918866352e-5))))))), );
                locals.var_fb_dchi__blk968_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body9_e45741: f64 = (locals.var_fb__blk967 * locals.var_fb__blk967);
                let assign31350_body9_e45743: f64 = (assign31350_body9_e45741 + locals.var_fs01__blk965);
                let assign31350_body9_e45745: f64 = (assign31350_body9_e45743 + 1e-50);
                let assign31350_body9_e45746: f64 = (assign31350_body9_e45745).sqrt();
                (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17, ) = (assign31350_body9_e45746, ((((locals.var_fb__blk967_dn0 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn0)) + locals.var_fs01__blk965_dn0) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn2 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn2)) + locals.var_fs01__blk965_dn2) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn6 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn6)) + locals.var_fs01__blk965_dn6) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn7 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn7)) + locals.var_fs01__blk965_dn7) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn10 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn10)) + locals.var_fs01__blk965_dn10) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn11 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn11)) + locals.var_fs01__blk965_dn11) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn12 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn12)) + locals.var_fs01__blk965_dn12) / (2.0 * assign31350_body9_e45746)), ((((locals.var_fb__blk967_dn17 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn17)) + locals.var_fs01__blk965_dn17) / (2.0 * assign31350_body9_e45746)), );
                locals.var_fs02__blk969_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 != 0.0)) {
                let assign31350_body10_e45764: f64 = (locals.var_beta * locals.var_fb_dchi__blk968);
                let assign31350_body10_e45766: f64 = (assign31350_body10_e45764 * 2.0);
                let assign31350_body10_e45768: f64 = (assign31350_body10_e45766 * locals.var_fb__blk967);
                let assign31350_body10_e45770: f64 = (assign31350_body10_e45768 + locals.var_fs01_dps0__blk966);
                let assign31350_body10_e45773: f64 = (locals.var_fs02__blk969 + locals.var_fs02__blk969);
                let assign31350_body10_e45774: f64 = (assign31350_body10_e45770 / assign31350_body10_e45773);
                (locals.var_fs02_dps0__blk970, locals.var_fs02_dps0__blk970_dn0, locals.var_fs02_dps0__blk970_dn2, locals.var_fs02_dps0__blk970_dn6, locals.var_fs02_dps0__blk970_dn7, locals.var_fs02_dps0__blk970_dn10, locals.var_fs02_dps0__blk970_dn11, locals.var_fs02_dps0__blk970_dn12, locals.var_fs02_dps0__blk970_dn17, ) = (assign31350_body10_e45774, ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn0) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn0)) + locals.var_fs01_dps0__blk966_dn0) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn0 + locals.var_fs02__blk969_dn0))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn2) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn2)) + locals.var_fs01_dps0__blk966_dn2) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn2 + locals.var_fs02__blk969_dn2))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn6) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn6)) + locals.var_fs01_dps0__blk966_dn6) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn6 + locals.var_fs02__blk969_dn6))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn7) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn7)) + locals.var_fs01_dps0__blk966_dn7) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn7 + locals.var_fs02__blk969_dn7))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi__blk968) + (locals.var_beta * locals.var_fb_dchi__blk968_dn10)) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn10)) + locals.var_fs01_dps0__blk966_dn10) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn10 + locals.var_fs02__blk969_dn10))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn11) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn11)) + locals.var_fs01_dps0__blk966_dn11) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn11 + locals.var_fs02__blk969_dn11))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn12) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn12)) + locals.var_fs01_dps0__blk966_dn12) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn12 + locals.var_fs02__blk969_dn12))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn17) * 2.0) * locals.var_fb__blk967) + (assign31350_body10_e45766 * locals.var_fb__blk967_dn17)) + locals.var_fs01_dps0__blk966_dn17) * assign31350_body10_e45773) - (assign31350_body10_e45770 * (locals.var_fs02__blk969_dn17 + locals.var_fs02__blk969_dn17))) / (assign31350_body10_e45773 * assign31350_body10_e45773)), );
                locals.var_fs02_dps0__blk970_rv = 0.0;
            }
            let assign31350_body11_e45779: f64 = if locals.var_chi__blk943 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard1011 = assign31350_body11_e45779;
            locals.var_guard1011_rv = 0.0;
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                let assign31350_body12_e45797: f64 = (locals.var_chi__blk943).exp();
                (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17, ) = (assign31350_body12_e45797, (assign31350_body12_e45797 * locals.var_chi__blk943_dn0), (assign31350_body12_e45797 * locals.var_chi__blk943_dn2), (assign31350_body12_e45797 * locals.var_chi__blk943_dn6), (assign31350_body12_e45797 * locals.var_chi__blk943_dn7), (assign31350_body12_e45797 * locals.var_chi__blk943_dn10), (assign31350_body12_e45797 * locals.var_chi__blk943_dn11), (assign31350_body12_e45797 * locals.var_chi__blk943_dn12), (assign31350_body12_e45797 * locals.var_chi__blk943_dn17), );
                locals.var_exp_chi_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                let assign31350_body13_e45819: f64 = (locals.var_exp_chi - 1.0);
                let assign31350_body13_e45820: f64 = (locals.var_cfs1__blk971 * assign31350_body13_e45819);
                (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17, ) = (assign31350_body13_e45820, ((locals.var_cfs1__blk971_dn0 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk971_dn2 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk971_dn6 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk971_dn7 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk971_dn10 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk971_dn11 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk971_dn12 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk971_dn17 * assign31350_body13_e45819) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn17)), );
                locals.var_fs01__blk965_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                let assign31350_body14_e45841: f64 = (locals.var_cfs1__blk971 * locals.var_beta);
                let assign31350_body14_e45843: f64 = (assign31350_body14_e45841 * locals.var_exp_chi);
                (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17, ) = (assign31350_body14_e45843, (((locals.var_cfs1__blk971_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk971_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk971_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk971_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk971_dn10 * locals.var_beta) + (locals.var_cfs1__blk971 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk971_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk971_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk971_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign31350_body14_e45841 * locals.var_exp_chi_dn17)), );
                locals.var_fs01_dps0__blk966_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 == 0.0)) {
                let assign31350_body15_e45865: f64 = (locals.var_beta * locals.var_ps0ld__blk945);
                let assign31350_body15_e45866: f64 = (assign31350_body15_e45865).exp();
                (locals.var_exp_bps0__blk972, locals.var_exp_bps0__blk972_dn0, locals.var_exp_bps0__blk972_dn2, locals.var_exp_bps0__blk972_dn6, locals.var_exp_bps0__blk972_dn7, locals.var_exp_bps0__blk972_dn10, locals.var_exp_bps0__blk972_dn11, locals.var_exp_bps0__blk972_dn12, locals.var_exp_bps0__blk972_dn17, ) = (assign31350_body15_e45866, (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn0)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn2)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn6)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn7)), (assign31350_body15_e45866 * ((locals.var_beta_dn10 * locals.var_ps0ld__blk945) + (locals.var_beta * locals.var_ps0ld__blk945_dn10))), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn11)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn12)), (assign31350_body15_e45866 * (locals.var_beta * locals.var_ps0ld__blk945_dn17)), );
                locals.var_exp_bps0__blk972_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 == 0.0)) {
                let assign31350_body16_e45889: f64 = (locals.var_exp_bps0__blk972 - locals.var_exp_bvbs__blk962);
                let assign31350_body16_e45890: f64 = (locals.var_cnst1over__blk956 * assign31350_body16_e45889);
                (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17, ) = (assign31350_body16_e45890, ((locals.var_cnst1over__blk956_dn0 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn0 - locals.var_exp_bvbs__blk962_dn0))), ((locals.var_cnst1over__blk956_dn2 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn2 - locals.var_exp_bvbs__blk962_dn2))), ((locals.var_cnst1over__blk956_dn6 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn6 - locals.var_exp_bvbs__blk962_dn6))), ((locals.var_cnst1over__blk956_dn7 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn7 - locals.var_exp_bvbs__blk962_dn7))), ((locals.var_cnst1over__blk956_dn10 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn10 - locals.var_exp_bvbs__blk962_dn10))), ((locals.var_cnst1over__blk956_dn11 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn11 - locals.var_exp_bvbs__blk962_dn11))), ((locals.var_cnst1over__blk956_dn12 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn12 - locals.var_exp_bvbs__blk962_dn12))), ((locals.var_cnst1over__blk956_dn17 * assign31350_body16_e45889) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn17 - locals.var_exp_bvbs__blk962_dn17))), );
                locals.var_fs01__blk965_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) && (locals.var_guard1011 == 0.0)) {
                let assign31350_body17_e45912: f64 = (locals.var_cnst1over__blk956 * locals.var_beta);
                let assign31350_body17_e45914: f64 = (assign31350_body17_e45912 * locals.var_exp_bps0__blk972);
                (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17, ) = (assign31350_body17_e45914, (((locals.var_cnst1over__blk956_dn0 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn0)), (((locals.var_cnst1over__blk956_dn2 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn2)), (((locals.var_cnst1over__blk956_dn6 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn6)), (((locals.var_cnst1over__blk956_dn7 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn7)), ((((locals.var_cnst1over__blk956_dn10 * locals.var_beta) + (locals.var_cnst1over__blk956 * locals.var_beta_dn10)) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn10)), (((locals.var_cnst1over__blk956_dn11 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn11)), (((locals.var_cnst1over__blk956_dn12 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn12)), (((locals.var_cnst1over__blk956_dn17 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign31350_body17_e45912 * locals.var_exp_bps0__blk972_dn17)), );
                locals.var_fs01_dps0__blk966_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) {
                let assign31350_body18_e45933: f64 = (locals.var_chi__blk943 - 1.0);
                let assign31350_body18_e45935: f64 = (assign31350_body18_e45933 + locals.var_fs01__blk965);
                let assign31350_body18_e45936: f64 = (assign31350_body18_e45935).sqrt();
                (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17, ) = (assign31350_body18_e45936, ((locals.var_chi__blk943_dn0 + locals.var_fs01__blk965_dn0) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn2 + locals.var_fs01__blk965_dn2) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn6 + locals.var_fs01__blk965_dn6) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn7 + locals.var_fs01__blk965_dn7) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn10 + locals.var_fs01__blk965_dn10) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn11 + locals.var_fs01__blk965_dn11) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn12 + locals.var_fs01__blk965_dn12) / (2.0 * assign31350_body18_e45936)), ((locals.var_chi__blk943_dn17 + locals.var_fs01__blk965_dn17) / (2.0 * assign31350_body18_e45936)), );
                locals.var_fs02__blk969_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1010 == 0.0)) {
                let assign31350_body19_e45955: f64 = (locals.var_beta + locals.var_fs01_dps0__blk966);
                let assign31350_body19_e45957: f64 = (assign31350_body19_e45955 / locals.var_fs02__blk969);
                let assign31350_body19_e45959: f64 = (assign31350_body19_e45957 * 0.5);
                (locals.var_fs02_dps0__blk970, locals.var_fs02_dps0__blk970_dn0, locals.var_fs02_dps0__blk970_dn2, locals.var_fs02_dps0__blk970_dn6, locals.var_fs02_dps0__blk970_dn7, locals.var_fs02_dps0__blk970_dn10, locals.var_fs02_dps0__blk970_dn11, locals.var_fs02_dps0__blk970_dn12, locals.var_fs02_dps0__blk970_dn17, ) = (assign31350_body19_e45959, ((((locals.var_fs01_dps0__blk966_dn0 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn0)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn2 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn2)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn6 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn6)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn7 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn7)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk966_dn10) * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn10)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn11 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn11)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn12 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn12)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn17 * locals.var_fs02__blk969) - (assign31350_body19_e45955 * locals.var_fs02__blk969_dn17)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), );
                locals.var_fs02_dps0__blk970_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
                let assign31350_body20_e45975: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
                let assign31350_body20_e45978: f64 = (locals.var_fac1__blk929 * locals.var_fs02__blk969);
                let assign31350_body20_e45979: f64 = (assign31350_body20_e45975 - assign31350_body20_e45978);
                (locals.var_fs0__blk973, locals.var_fs0__blk973_dn0, locals.var_fs0__blk973_dn2, locals.var_fs0__blk973_dn6, locals.var_fs0__blk973_dn7, locals.var_fs0__blk973_dn10, locals.var_fs0__blk973_dn11, locals.var_fs0__blk973_dn12, locals.var_fs0__blk973_dn17, ) = (assign31350_body20_e45979, ((locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0) - ((locals.var_fac1__blk929_dn0 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn0))), ((locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2) - ((locals.var_fac1__blk929_dn2 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn2))), ((locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6) - ((locals.var_fac1__blk929_dn6 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn6))), ((locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7) - ((locals.var_fac1__blk929_dn7 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn7))), ((locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10) - ((locals.var_fac1__blk929_dn10 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn10))), ((locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11) - ((locals.var_fac1__blk929_dn11 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn11))), ((locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12) - ((locals.var_fac1__blk929_dn12 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn12))), ((locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17) - ((locals.var_fac1__blk929_dn17 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn17))), );
                locals.var_fs0__blk973_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
                let assign31350_body21_e45994: f64 = (-1.0);
                let assign31350_body21_e45997: f64 = (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970);
                let assign31350_body21_e45998: f64 = (assign31350_body21_e45994 - assign31350_body21_e45997);
                (locals.var_fs0_dps0__blk974, locals.var_fs0_dps0__blk974_dn0, locals.var_fs0_dps0__blk974_dn2, locals.var_fs0_dps0__blk974_dn6, locals.var_fs0_dps0__blk974_dn7, locals.var_fs0_dps0__blk974_dn10, locals.var_fs0_dps0__blk974_dn11, locals.var_fs0_dps0__blk974_dn12, locals.var_fs0_dps0__blk974_dn17, ) = (assign31350_body21_e45998, (-((locals.var_fac1__blk929_dn0 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn0))), (-((locals.var_fac1__blk929_dn2 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn2))), (-((locals.var_fac1__blk929_dn6 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn6))), (-((locals.var_fac1__blk929_dn7 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn7))), (-((locals.var_fac1__blk929_dn10 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn10))), (-((locals.var_fac1__blk929_dn11 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn11))), (-((locals.var_fac1__blk929_dn12 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn12))), (-((locals.var_fac1__blk929_dn17 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn17))), );
                locals.var_fs0_dps0__blk974_rv = 0.0;
            }
            let assign31350_body22_e46003: f64 = if locals.var_flg_conv__blk918 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1012 = assign31350_body22_e46003;
            locals.var_guard1012_rv = 0.0;
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31350_body23_e46019: f64 = (2.0 * 20.0);
                let assign31350_body23_e46021: f64 = (assign31350_body23_e46019 + 1.0);
                locals.var_lp_s0 = assign31350_body23_e46021;
                locals.var_lp_s0_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) {
                let assign31350_body24_e46039: f64 = (-locals.var_fs0__blk973);
                let assign31350_body24_e46041: f64 = (assign31350_body24_e46039 / locals.var_fs0_dps0__blk974);
                (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17, ) = (assign31350_body24_e46041, ((((-locals.var_fs0__blk973_dn0) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn0)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn2) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn2)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn6) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn6)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn7) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn7)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn10) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn10)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn11) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn11)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn12) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn12)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn17) * locals.var_fs0_dps0__blk974) - (assign31350_body24_e46039 * locals.var_fs0_dps0__blk974_dn17)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), );
                locals.var_dps0_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) {
                let assign31350_body25_e46060: f64 = (0.5 * 0.1);
                let assign31350_body25_e46064: f64 = (locals.var_ps0ld__blk945).abs();
                let (assign31350_body25_e46069, assign31350_body25_e46069_d_n0, assign31350_body25_e46069_d_n2, assign31350_body25_e46069_d_n6, assign31350_body25_e46069_d_n7, assign31350_body25_e46069_d_n10, assign31350_body25_e46069_d_n11, assign31350_body25_e46069_d_n12, assign31350_body25_e46069_d_n17,) = {
    if (1.0 >= assign31350_body25_e46064) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign31350_body25_e46068: f64 = (locals.var_ps0ld__blk945).abs();
        (assign31350_body25_e46068, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn0 } else { (-locals.var_ps0ld__blk945_dn0) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn2 } else { (-locals.var_ps0ld__blk945_dn2) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn6 } else { (-locals.var_ps0ld__blk945_dn6) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn7 } else { (-locals.var_ps0ld__blk945_dn7) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn10 } else { (-locals.var_ps0ld__blk945_dn10) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn11 } else { (-locals.var_ps0ld__blk945_dn11) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn12 } else { (-locals.var_ps0ld__blk945_dn12) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn17 } else { (-locals.var_ps0ld__blk945_dn17) },)
    }
};
                let assign31350_body25_e46070: f64 = (1.0 + assign31350_body25_e46069);
                let assign31350_body25_e46071: f64 = (assign31350_body25_e46060 * assign31350_body25_e46070);
                (locals.var_dplim__blk975, locals.var_dplim__blk975_dn0, locals.var_dplim__blk975_dn2, locals.var_dplim__blk975_dn6, locals.var_dplim__blk975_dn7, locals.var_dplim__blk975_dn10, locals.var_dplim__blk975_dn11, locals.var_dplim__blk975_dn12, locals.var_dplim__blk975_dn17, ) = (assign31350_body25_e46071, (assign31350_body25_e46060 * assign31350_body25_e46069_d_n0), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n2), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n6), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n7), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n10), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n11), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n12), (assign31350_body25_e46060 * assign31350_body25_e46069_d_n17), );
                locals.var_dplim__blk975_rv = 0.0;
            }
            let assign31350_body26_e46075: f64 = (locals.var_dps0).abs();
            let assign31350_body26_e46077: f64 = if assign31350_body26_e46075 > locals.var_dplim__blk975 { 1.0 } else { 0.0 };
            locals.var_guard1013 = assign31350_body26_e46077;
            locals.var_guard1013_rv = 0.0;
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 != 0.0)) {
                let (assign31350_body27_e46101,) = {
    if (locals.var_dps0 >= 0.0) {
        (1.0,)
    } else {
        let assign31350_body27_e46100: f64 = (-1.0);
        (assign31350_body27_e46100,)
    }
};
                let assign31350_body27_e46102: f64 = (locals.var_dplim__blk975 * assign31350_body27_e46101);
                (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17, ) = (assign31350_body27_e46102, (locals.var_dplim__blk975_dn0 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn2 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn6 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn7 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn10 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn11 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn12 * assign31350_body27_e46101), (locals.var_dplim__blk975_dn17 * assign31350_body27_e46101), );
                locals.var_dps0_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) {
                let assign31350_body28_e46121: f64 = (locals.var_ps0ld__blk945 + locals.var_dps0);
                (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17, ) = (assign31350_body28_e46121, (locals.var_ps0ld__blk945_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld__blk945_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld__blk945_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld__blk945_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld__blk945_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld__blk945_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld__blk945_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld__blk945_dn17 + locals.var_dps0_dn17), );
                locals.var_ps0ld__blk945_rv = 0.0;
            }
            let assign31350_body29_e46125: f64 = (locals.var_dps0).abs();
            let assign31350_body29_e46129: f64 = (locals.var_fs0__blk973).abs();
            let assign31350_body29_e46132: f64 = if ((assign31350_body29_e46125 <= 5e-12) && (assign31350_body29_e46129 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1014 = assign31350_body29_e46132;
            locals.var_guard1014_rv = 0.0;
            if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1014 != 0.0)) {
                locals.var_flg_conv__blk918 = 1.0;
                locals.var_flg_conv__blk918_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
                let assign31350_body31_e46165: f64 = (locals.var_lp_s0 + 1.0);
                locals.var_lp_s0 = assign31350_body31_e46165;
                locals.var_lp_s0_rv = 0.0;
            }
        }

        let assign31370_e46173: f64 = if locals.var_chi__blk943 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1016 = assign31370_e46173;
        locals.var_guard1016_rv = 0.0;

        if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 != 0.0)) {
            let assign31410_e46229: f64 = (locals.var_fb__blk967 * locals.var_fb__blk967);
            let assign31410_e46232: f64 = (10.0 * 2.220446049250313e-16);
            let assign31410_e46233: f64 = (assign31410_e46229 + assign31410_e46232);
            (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17, ) = (assign31410_e46233, ((locals.var_fb__blk967_dn0 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn0)), ((locals.var_fb__blk967_dn2 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn2)), ((locals.var_fb__blk967_dn6 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn6)), ((locals.var_fb__blk967_dn7 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn7)), ((locals.var_fb__blk967_dn10 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn10)), ((locals.var_fb__blk967_dn11 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn11)), ((locals.var_fb__blk967_dn12 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn12)), ((locals.var_fb__blk967_dn17 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn17)), );
            locals.var_xi0__blk976_rv = 0.0;
        }

        if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 != 0.0)) {
            let assign31420_e46252: f64 = (10.0 * 2.220446049250313e-16);
            let assign31420_e46253: f64 = (locals.var_fb__blk967 + assign31420_e46252);
            (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17, ) = (assign31420_e46253, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17, );
            locals.var_xi0p12__blk977_rv = 0.0;
        }

        if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 == 0.0)) {
            let assign31440_e46289: f64 = (locals.var_chi__blk943 - 1.0);
            (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17, ) = (assign31440_e46289, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17, );
            locals.var_xi0__blk976_rv = 0.0;
        }

        if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) && (locals.var_guard1016 == 0.0)) {
            let assign31450_e46307: f64 = (locals.var_xi0__blk976).sqrt();
            (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17, ) = (assign31450_e46307, (locals.var_xi0__blk976_dn0 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn2 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn6 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn7 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn10 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn11 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn12 / (2.0 * assign31450_e46307)), (locals.var_xi0__blk976_dn17 / (2.0 * assign31450_e46307)), );
            locals.var_xi0p12__blk977_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31460_e46323: f64 = (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977);
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17, ) = (assign31460_e46323, ((locals.var_cnst0over__blk928_dn0 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn0)), ((locals.var_cnst0over__blk928_dn2 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn2)), ((locals.var_cnst0over__blk928_dn6 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn6)), ((locals.var_cnst0over__blk928_dn7 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn7)), ((locals.var_cnst0over__blk928_dn10 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn10)), ((locals.var_cnst0over__blk928_dn11 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn11)), ((locals.var_cnst0over__blk928_dn12 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn12)), ((locals.var_cnst0over__blk928_dn17 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn17)), );
            locals.var_qbuld_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31470_e46340: f64 = (locals.var_fs02__blk969 + locals.var_xi0p12__blk977);
            let assign31470_e46341: f64 = (1.0 / assign31470_e46340);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31470_e46341, (-((locals.var_fs02__blk969_dn0 + locals.var_xi0p12__blk977_dn0) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn2 + locals.var_xi0p12__blk977_dn2) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn6 + locals.var_xi0p12__blk977_dn6) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn7 + locals.var_xi0p12__blk977_dn7) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn10 + locals.var_xi0p12__blk977_dn10) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn11 + locals.var_xi0p12__blk977_dn11) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn12 + locals.var_xi0p12__blk977_dn12) / (assign31470_e46340 * assign31470_e46340))), (-((locals.var_fs02__blk969_dn17 + locals.var_xi0p12__blk977_dn17) / (assign31470_e46340 * assign31470_e46340))), );
            locals.var_t1__blk896_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31480_e46357: f64 = (locals.var_cnst0over__blk928 * locals.var_fs01__blk965);
            let assign31480_e46359: f64 = (assign31480_e46357 * locals.var_t1__blk896);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17, ) = (assign31480_e46359, ((((locals.var_cnst0over__blk928_dn0 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn0)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn0)), ((((locals.var_cnst0over__blk928_dn2 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn2)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn2)), ((((locals.var_cnst0over__blk928_dn6 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn6)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn6)), ((((locals.var_cnst0over__blk928_dn7 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn7)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn7)), ((((locals.var_cnst0over__blk928_dn10 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn10)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn10)), ((((locals.var_cnst0over__blk928_dn11 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn11)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn11)), ((((locals.var_cnst0over__blk928_dn12 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn12)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn12)), ((((locals.var_cnst0over__blk928_dn17 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn17)) * locals.var_t1__blk896) + (assign31480_e46357 * locals.var_t1__blk896_dn17)), );
            locals.var_qiuld_rv = 0.0;
        }

        if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31490_e46375: f64 = (locals.var_qbuld + locals.var_qiuld);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17, ) = (assign31490_e46375, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17), );
            locals.var_qsuld_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
            let assign31500_e46386: f64 = (locals.var_qsuld - locals.var_qbuld);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17, ) = (assign31500_e46386, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17), );
            locals.var_qiuld_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
            let (assign31510_e46404,) = {
    if (p.p43 == 1.0) {
        let assign31510_e46400: f64 = (locals.var_w_dioscv * locals.var_lov);
        (assign31510_e46400,)
    } else {
        let assign31510_e46403: f64 = (locals.var_weffcv_nf * locals.var_lov);
        (assign31510_e46403,)
    }
};
            (locals.var_t4__blk899, locals.var_t4__blk899_dn0, locals.var_t4__blk899_dn2, locals.var_t4__blk899_dn6, locals.var_t4__blk899_dn7, locals.var_t4__blk899_dn10, locals.var_t4__blk899_dn11, locals.var_t4__blk899_dn12, locals.var_t4__blk899_dn17, ) = (assign31510_e46404, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t4__blk899_rv = 0.0;
        }

        let assign31520_e46417: f64 = if (((locals.var_flg_overs__blk914 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk912 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign31520_e46417;
        locals.var_guard1018_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1018 != 0.0)) {
            let assign31530_e46428: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
            (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17, ) = (assign31530_e46428, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)), );
            locals.var_qovs_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1018 != 0.0)) {
            let assign31540_e46441: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
            (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17, ) = (assign31540_e46441, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)), );
            locals.var_qbsld_rv = 0.0;
        }

        let assign31550_e46454: f64 = if (((locals.var_flg_overd__blk915 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk913 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1019 = assign31550_e46454;
        locals.var_guard1019_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1019 != 0.0)) {
            let assign31560_e46465: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
            (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17, ) = (assign31560_e46465, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)), );
            locals.var_qovd_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1019 != 0.0)) {
            let assign31570_e46478: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
            (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17, ) = (assign31570_e46478, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)), );
            locals.var_qbdld_rv = 0.0;
        }

        if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
            let assign31580_e46486: f64 = (locals.var_modervs * locals.var_cgso_given);
            let assign31580_e46489: f64 = (locals.var_modenml * locals.var_cgdo_given);
            let assign31580_e46490: f64 = (assign31580_e46486 + assign31580_e46489);
            locals.var_flg_overgiven = assign31580_e46490;
            locals.var_flg_overgiven_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31590_e46500: f64 = (locals.var_modervs * p.p170);
            let assign31590_e46503: f64 = (locals.var_modenml * p.p169);
            let assign31590_e46504: f64 = (assign31590_e46500 + assign31590_e46503);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31590_e46504, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }

        let assign31600_e46509: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign31600_e46509;
        locals.var_guard1020_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 != 0.0)) {
            let assign31610_e46519: f64 = (locals.var_modervs * locals.var_w_dioscv);
            let assign31610_e46522: f64 = (locals.var_modenml * locals.var_w_diodcv);
            let assign31610_e46523: f64 = (assign31610_e46519 + assign31610_e46522);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31610_e46523, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk896_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 != 0.0)) {
            let assign31620_e46535: f64 = (-locals.var_t1__blk896);
            let assign31620_e46536: f64 = (locals.var_cgdoe * assign31620_e46535);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31620_e46536, ((locals.var_cgdoe_dn0 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgdoe_dn2 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgdoe_dn6 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgdoe_dn7 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgdoe_dn10 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgdoe_dn11 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgdoe_dn12 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgdoe_dn17 * assign31620_e46535) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn17))), );
            locals.var_cgdoe_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1020 == 0.0)) {
            let assign31630_e46549: f64 = (-locals.var_weffcv_nf);
            let assign31630_e46550: f64 = (locals.var_cgdoe * assign31630_e46549);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31630_e46550, (locals.var_cgdoe_dn0 * assign31630_e46549), (locals.var_cgdoe_dn2 * assign31630_e46549), (locals.var_cgdoe_dn6 * assign31630_e46549), (locals.var_cgdoe_dn7 * assign31630_e46549), (locals.var_cgdoe_dn10 * assign31630_e46549), (locals.var_cgdoe_dn11 * assign31630_e46549), (locals.var_cgdoe_dn12 * assign31630_e46549), (locals.var_cgdoe_dn17 * assign31630_e46549), );
            locals.var_cgdoe_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31640_e46560: f64 = (-locals.var_cgdoe);
            let assign31640_e46563: f64 = (locals.var_vgs - locals.var_vds);
            let assign31640_e46564: f64 = (assign31640_e46560 * assign31640_e46563);
            let assign31640_e46565: f64 = (locals.var_qgod + assign31640_e46564);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign31640_e46565, (locals.var_qgod_dn0 + (((-locals.var_cgdoe_dn0) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn0)))), (locals.var_qgod_dn2 + (((-locals.var_cgdoe_dn2) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn2)))), (locals.var_qgod_dn6 + (((-locals.var_cgdoe_dn6) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn6 - locals.var_vds_dn6)))), (locals.var_qgod_dn7 + (((-locals.var_cgdoe_dn7) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn7 - locals.var_vds_dn7)))), (locals.var_qgod_dn10 + (((-locals.var_cgdoe_dn10) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn10)))), (locals.var_qgod_dn11 + (((-locals.var_cgdoe_dn11) * assign31640_e46563) + (assign31640_e46560 * (locals.var_vgs_dn11 - locals.var_vds_dn11)))), (locals.var_qgod_dn12 + (((-locals.var_cgdoe_dn12) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn12)))), (locals.var_qgod_dn17 + (((-locals.var_cgdoe_dn17) * assign31640_e46563) + (assign31640_e46560 * (-locals.var_vds_dn17)))), );
            locals.var_qgod_rv = 0.0;
        }

        if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
            let assign31650_e46573: f64 = (locals.var_modenml * locals.var_cgso_given);
            let assign31650_e46576: f64 = (locals.var_modervs * locals.var_cgdo_given);
            let assign31650_e46577: f64 = (assign31650_e46573 + assign31650_e46576);
            locals.var_flg_overgiven = assign31650_e46577;
            locals.var_flg_overgiven_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31660_e46587: f64 = (locals.var_modenml * p.p170);
            let assign31660_e46590: f64 = (locals.var_modervs * p.p169);
            let assign31660_e46591: f64 = (assign31660_e46587 + assign31660_e46590);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31660_e46591, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }

        let assign31670_e46596: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1021 = assign31670_e46596;
        locals.var_guard1021_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 != 0.0)) {
            let assign31680_e46606: f64 = (locals.var_modenml * locals.var_w_dioscv);
            let assign31680_e46609: f64 = (locals.var_modervs * locals.var_w_diodcv);
            let assign31680_e46610: f64 = (assign31680_e46606 + assign31680_e46609);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31680_e46610, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk896_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 != 0.0)) {
            let assign31690_e46622: f64 = (-locals.var_t1__blk896);
            let assign31690_e46623: f64 = (locals.var_cgsoe * assign31690_e46622);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31690_e46623, ((locals.var_cgsoe_dn0 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgsoe_dn2 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgsoe_dn6 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgsoe_dn7 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgsoe_dn10 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgsoe_dn11 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgsoe_dn12 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgsoe_dn17 * assign31690_e46622) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn17))), );
            locals.var_cgsoe_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1021 == 0.0)) {
            let assign31700_e46636: f64 = (-locals.var_weffcv_nf);
            let assign31700_e46637: f64 = (locals.var_cgsoe * assign31700_e46636);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31700_e46637, (locals.var_cgsoe_dn0 * assign31700_e46636), (locals.var_cgsoe_dn2 * assign31700_e46636), (locals.var_cgsoe_dn6 * assign31700_e46636), (locals.var_cgsoe_dn7 * assign31700_e46636), (locals.var_cgsoe_dn10 * assign31700_e46636), (locals.var_cgsoe_dn11 * assign31700_e46636), (locals.var_cgsoe_dn12 * assign31700_e46636), (locals.var_cgsoe_dn17 * assign31700_e46636), );
            locals.var_cgsoe_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31710_e46647: f64 = (-locals.var_cgsoe);
            let assign31710_e46649: f64 = (assign31710_e46647 * locals.var_vgs);
            let assign31710_e46650: f64 = (locals.var_qgos + assign31710_e46649);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign31710_e46650, (locals.var_qgos_dn0 + ((-locals.var_cgsoe_dn0) * locals.var_vgs)), (locals.var_qgos_dn2 + ((-locals.var_cgsoe_dn2) * locals.var_vgs)), (locals.var_qgos_dn6 + (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn6))), (locals.var_qgos_dn7 + (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn7))), (locals.var_qgos_dn10 + ((-locals.var_cgsoe_dn10) * locals.var_vgs)), (locals.var_qgos_dn11 + (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31710_e46647 * locals.var_vgs_dn11))), (locals.var_qgos_dn12 + ((-locals.var_cgsoe_dn12) * locals.var_vgs)), (locals.var_qgos_dn17 + ((-locals.var_cgsoe_dn17) * locals.var_vgs)), );
            locals.var_qgos_rv = 0.0;
        }

        let assign31720_e46665: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgdo_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign31720_e46665;
        locals.var_guard1022_rv = 0.0;

        let assign31730_e46668: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign31730_e46668;
        locals.var_guard1023_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 != 0.0)) {
            let assign31740_e46678: f64 = (-locals.var_cox0__blk906);
            let assign31740_e46680: f64 = (assign31740_e46678 * p.p188);
            let assign31740_e46682: f64 = (assign31740_e46680 * locals.var_w_diodcv);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31740_e46682, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 != 0.0)) && (locals.var_guard1023 == 0.0)) {
            let assign31750_e46695: f64 = (-locals.var_cox0__blk906);
            let assign31750_e46697: f64 = (assign31750_e46695 * p.p188);
            let assign31750_e46699: f64 = (assign31750_e46697 * locals.var_weffcv_nf);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31750_e46699, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) {
            let assign31760_e46711: f64 = (locals.var_modervs * p.p170);
            let assign31760_e46714: f64 = (locals.var_modenml * p.p169);
            let assign31760_e46715: f64 = (assign31760_e46711 + assign31760_e46714);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31760_e46715, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }

        let assign31770_e46720: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign31770_e46720;
        locals.var_guard1024_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 != 0.0)) {
            let assign31780_e46732: f64 = (locals.var_modervs * locals.var_w_dioscv);
            let assign31780_e46735: f64 = (locals.var_modenml * locals.var_w_diodcv);
            let assign31780_e46736: f64 = (assign31780_e46732 + assign31780_e46735);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31780_e46736, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk896_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 != 0.0)) {
            let assign31790_e46750: f64 = (-locals.var_t1__blk896);
            let assign31790_e46751: f64 = (locals.var_cgdoe * assign31790_e46750);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31790_e46751, ((locals.var_cgdoe_dn0 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgdoe_dn2 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgdoe_dn6 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgdoe_dn7 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgdoe_dn10 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgdoe_dn11 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgdoe_dn12 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgdoe_dn17 * assign31790_e46750) + (locals.var_cgdoe * (-locals.var_t1__blk896_dn17))), );
            locals.var_cgdoe_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 == 0.0)) {
            let assign31800_e46766: f64 = (-locals.var_weffcv_nf);
            let assign31800_e46767: f64 = (locals.var_cgdoe * assign31800_e46766);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31800_e46767, (locals.var_cgdoe_dn0 * assign31800_e46766), (locals.var_cgdoe_dn2 * assign31800_e46766), (locals.var_cgdoe_dn6 * assign31800_e46766), (locals.var_cgdoe_dn7 * assign31800_e46766), (locals.var_cgdoe_dn10 * assign31800_e46766), (locals.var_cgdoe_dn11 * assign31800_e46766), (locals.var_cgdoe_dn12 * assign31800_e46766), (locals.var_cgdoe_dn17 * assign31800_e46766), );
            locals.var_cgdoe_rv = 0.0;
        }

        if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
            let assign31810_e46775: f64 = (-locals.var_cgdoe);
            let assign31810_e46778: f64 = (locals.var_vgs - locals.var_vds);
            let assign31810_e46779: f64 = (assign31810_e46775 * assign31810_e46778);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign31810_e46779, (((-locals.var_cgdoe_dn0) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn17))), );
            locals.var_qgod_rv = 0.0;
        }

        let assign31820_e46794: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign31820_e46794;
        locals.var_guard1025_rv = 0.0;

        let assign31830_e46797: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign31830_e46797;
        locals.var_guard1026_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 != 0.0)) {
            let assign31840_e46807: f64 = (-locals.var_cox0__blk906);
            let assign31840_e46809: f64 = (assign31840_e46807 * p.p188);
            let assign31840_e46811: f64 = (assign31840_e46809 * locals.var_w_dioscv);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31840_e46811, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 == 0.0)) {
            let assign31850_e46824: f64 = (-locals.var_cox0__blk906);
            let assign31850_e46826: f64 = (assign31850_e46824 * p.p188);
            let assign31850_e46828: f64 = (assign31850_e46826 * locals.var_weffcv_nf);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31850_e46828, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }

        if (((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) {
            let assign31860_e46840: f64 = (locals.var_modenml * p.p170);
            let assign31860_e46843: f64 = (locals.var_modervs * p.p169);
            let assign31860_e46844: f64 = (assign31860_e46840 + assign31860_e46843);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31860_e46844, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }

        let assign31870_e46849: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign31870_e46849;
        locals.var_guard1027_rv = 0.0;

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
            let assign31880_e46861: f64 = (locals.var_modenml * locals.var_w_dioscv);
            let assign31880_e46864: f64 = (locals.var_modervs * locals.var_w_diodcv);
            let assign31880_e46865: f64 = (assign31880_e46861 + assign31880_e46864);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31880_e46865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk896_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
            let assign31890_e46879: f64 = (-locals.var_t1__blk896);
            let assign31890_e46880: f64 = (locals.var_cgsoe * assign31890_e46879);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31890_e46880, ((locals.var_cgsoe_dn0 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgsoe_dn2 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgsoe_dn6 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgsoe_dn7 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgsoe_dn10 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgsoe_dn11 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgsoe_dn12 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgsoe_dn17 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn17))), );
            locals.var_cgsoe_rv = 0.0;
        }

        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 == 0.0)) {
            let assign31900_e46895: f64 = (-locals.var_weffcv_nf);
            let assign31900_e46896: f64 = (locals.var_cgsoe * assign31900_e46895);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31900_e46896, (locals.var_cgsoe_dn0 * assign31900_e46895), (locals.var_cgsoe_dn2 * assign31900_e46895), (locals.var_cgsoe_dn6 * assign31900_e46895), (locals.var_cgsoe_dn7 * assign31900_e46895), (locals.var_cgsoe_dn10 * assign31900_e46895), (locals.var_cgsoe_dn11 * assign31900_e46895), (locals.var_cgsoe_dn12 * assign31900_e46895), (locals.var_cgsoe_dn17 * assign31900_e46895), );
            locals.var_cgsoe_rv = 0.0;
        }

        if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
            let assign31910_e46904: f64 = (-locals.var_cgsoe);
            let assign31910_e46906: f64 = (assign31910_e46904 * locals.var_vgs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign31910_e46906, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs), );
            locals.var_qgos_rv = 0.0;
        }

        let assign31920_e46911: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign31920_e46911;
        locals.var_guard1028_rv = 0.0;

        if (locals.var_guard1028 != 0.0) {
            (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12, ) = (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12, );
            locals.var_vbdj_rv = 0.0;
            (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12, ) = (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12, );
            locals.var_vbsj_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign31950_e46924: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign31950_e46927: f64 = (locals.var_eg * locals.var_beta);
            let assign31950_e46928: f64 = (assign31950_e46924 - assign31950_e46927);
            let assign31950_e46932: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign31950_e46933: f64 = (assign31950_e46932).ln();
            let assign31950_e46934: f64 = (p.p175 * assign31950_e46933);
            let assign31950_e46935: f64 = (assign31950_e46928 + assign31950_e46934);
            let assign31950_e46937: f64 = (assign31950_e46935 / p.p174);
            let assign31950_e46938: f64 = (assign31950_e46937).exp();
            let assign31950_e46939: f64 = (p.p173 * assign31950_e46938);
            (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17, ) = (assign31950_e46939, (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31950_e46932))) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign31960_e46946: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign31960_e46949: f64 = (locals.var_eg * locals.var_beta);
            let assign31960_e46950: f64 = (assign31960_e46946 - assign31960_e46949);
            let assign31960_e46954: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign31960_e46955: f64 = (assign31960_e46954).ln();
            let assign31960_e46956: f64 = (p.p176 * assign31960_e46955);
            let assign31960_e46957: f64 = (assign31960_e46950 + assign31960_e46956);
            let assign31960_e46959: f64 = (assign31960_e46957 / p.p174);
            let assign31960_e46960: f64 = (assign31960_e46959).exp();
            let assign31960_e46961: f64 = (p.p173 * assign31960_e46960);
            (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17, ) = (assign31960_e46961, (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31960_e46954))) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js2_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign31970_e46967: f64 = (locals.var_w_diod * p.p237);
            let assign31970_e46969: f64 = (assign31970_e46967 * locals.var_js);
            (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, ) = (assign31970_e46969, (assign31970_e46967 * locals.var_js_dn0), (assign31970_e46967 * locals.var_js_dn2), (assign31970_e46967 * locals.var_js_dn6), (assign31970_e46967 * locals.var_js_dn7), (assign31970_e46967 * locals.var_js_dn10), (assign31970_e46967 * locals.var_js_dn11), (assign31970_e46967 * locals.var_js_dn12), (assign31970_e46967 * locals.var_js_dn17), );
            locals.var_isbd_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign31980_e46975: f64 = (locals.var_w_diod * p.p237);
            let assign31980_e46977: f64 = (assign31980_e46975 * locals.var_js2);
            (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17, ) = (assign31980_e46977, (assign31980_e46975 * locals.var_js2_dn0), (assign31980_e46975 * locals.var_js2_dn2), (assign31980_e46975 * locals.var_js2_dn6), (assign31980_e46975 * locals.var_js2_dn7), (assign31980_e46975 * locals.var_js2_dn10), (assign31980_e46975 * locals.var_js2_dn11), (assign31980_e46975 * locals.var_js2_dn12), (assign31980_e46975 * locals.var_js2_dn17), );
            locals.var_isbd2_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign31990_e46983: f64 = (locals.var_w_dios * p.p237);
            let assign31990_e46985: f64 = (assign31990_e46983 * locals.var_js);
            (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17, ) = (assign31990_e46985, (assign31990_e46983 * locals.var_js_dn0), (assign31990_e46983 * locals.var_js_dn2), (assign31990_e46983 * locals.var_js_dn6), (assign31990_e46983 * locals.var_js_dn7), (assign31990_e46983 * locals.var_js_dn10), (assign31990_e46983 * locals.var_js_dn11), (assign31990_e46983 * locals.var_js_dn12), (assign31990_e46983 * locals.var_js_dn17), );
            locals.var_isbs_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32000_e46991: f64 = (locals.var_w_dios * p.p237);
            let assign32000_e46993: f64 = (assign32000_e46991 * locals.var_js2);
            (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17, ) = (assign32000_e46993, (assign32000_e46991 * locals.var_js2_dn0), (assign32000_e46991 * locals.var_js2_dn2), (assign32000_e46991 * locals.var_js2_dn6), (assign32000_e46991 * locals.var_js2_dn7), (assign32000_e46991 * locals.var_js2_dn10), (assign32000_e46991 * locals.var_js2_dn11), (assign32000_e46991 * locals.var_js2_dn12), (assign32000_e46991 * locals.var_js2_dn17), );
            locals.var_isbs2_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32010_e46999: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32010_e46999, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32030_e47011: f64 = (locals.var_isbd + 1e-50);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32030_e47011, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, );
            locals.var_t2__blk1031_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            (locals.var_vbdt, locals.var_vbdt_dn10, ) = (0.0, 0.0, );
            locals.var_vbdt_rv = 0.0;
            (locals.var_vbst, locals.var_vbst_dn10, ) = (0.0, 0.0, );
            locals.var_vbst_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32070_e47039: f64 = (p.p174 * locals.var_beta_inv);
            (locals.var_nvtm, locals.var_nvtm_dn10, ) = (assign32070_e47039, (p.p174 * locals.var_beta_inv_dn10), );
            locals.var_nvtm_rv = 0.0;
        }

        let assign32080_e47044: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1057 = assign32080_e47044;
        locals.var_guard1057_rv = 0.0;

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
            let assign32090_e47050: f64 = (locals.var_vbdj / locals.var_nvtm);
            let assign32090_e47051: f64 = (assign32090_e47050).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32090_e47051, (assign32090_e47051 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32090_e47051 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32090_e47051 * (locals.var_vbdj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
            let assign32100_e47060: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32100_e47061: f64 = (locals.var_isbd * assign32100_e47060);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32100_e47061, (locals.var_isbd_dn0 * assign32100_e47060), (locals.var_isbd_dn2 * assign32100_e47060), ((locals.var_isbd_dn6 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn6)), ((locals.var_isbd_dn7 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn7)), ((locals.var_isbd_dn10 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn10)), (locals.var_isbd_dn11 * assign32100_e47060), ((locals.var_isbd_dn12 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn12)), (locals.var_isbd_dn17 * assign32100_e47060), );
            locals.var_ibd_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
            let assign32110_e47070: f64 = (locals.var_vbdt / locals.var_nvtm);
            let assign32110_e47071: f64 = (assign32110_e47070).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32110_e47071, 0.0, 0.0, (assign32110_e47071 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
            let assign32120_e47081: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32120_e47082: f64 = (locals.var_isbd * assign32120_e47081);
            let assign32120_e47085: f64 = (locals.var_isbd / locals.var_nvtm);
            let assign32120_e47087: f64 = (assign32120_e47085 * locals.var_t1__blk1030);
            let assign32120_e47090: f64 = (locals.var_vbdj - locals.var_vbdt);
            let assign32120_e47091: f64 = (assign32120_e47087 * assign32120_e47090);
            let assign32120_e47092: f64 = (assign32120_e47082 + assign32120_e47091);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32120_e47092, ((locals.var_isbd_dn0 * assign32120_e47081) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), ((locals.var_isbd_dn2 * assign32120_e47081) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn6 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn6)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn7)) * assign32120_e47090)), (((locals.var_isbd_dn10 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn10)) * assign32120_e47090) + (assign32120_e47087 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32120_e47081) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn12 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn12)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32120_e47081) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), );
            locals.var_ibd_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32130_e47099: f64 = (p.p178 * locals.var_vbdj);
            let assign32130_e47101: f64 = (assign32130_e47099 * locals.var_isbd2);
            let assign32130_e47102: f64 = (locals.var_ibd + assign32130_e47101);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32130_e47102, (locals.var_ibd_dn0 + (assign32130_e47099 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32130_e47099 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32130_e47099 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32130_e47099 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32130_e47099 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32130_e47099 * locals.var_isbd2_dn17)), );
            locals.var_ibd_rv = 0.0;
        }

        let assign32140_e47107: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1058 = assign32140_e47107;
        locals.var_guard1058_rv = 0.0;

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
            let assign32150_e47113: f64 = (locals.var_vbsj / locals.var_nvtm);
            let assign32150_e47114: f64 = (assign32150_e47113).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32150_e47114, 0.0, (assign32150_e47114 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32150_e47114 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32150_e47114 * (locals.var_vbsj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
            let assign32160_e47123: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32160_e47124: f64 = (locals.var_isbs * assign32160_e47123);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32160_e47124, (locals.var_isbs_dn0 * assign32160_e47123), (locals.var_isbs_dn2 * assign32160_e47123), ((locals.var_isbs_dn6 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn6)), ((locals.var_isbs_dn7 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn7)), ((locals.var_isbs_dn10 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn10)), (locals.var_isbs_dn11 * assign32160_e47123), ((locals.var_isbs_dn12 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn12)), (locals.var_isbs_dn17 * assign32160_e47123), );
            locals.var_ibs_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
            let assign32170_e47133: f64 = (locals.var_vbst / locals.var_nvtm);
            let assign32170_e47134: f64 = (assign32170_e47133).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32170_e47134, 0.0, 0.0, (assign32170_e47134 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
            let assign32180_e47144: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32180_e47145: f64 = (locals.var_isbs * assign32180_e47144);
            let assign32180_e47148: f64 = (locals.var_isbs / locals.var_nvtm);
            let assign32180_e47150: f64 = (assign32180_e47148 * locals.var_t1__blk1030);
            let assign32180_e47153: f64 = (locals.var_vbsj - locals.var_vbst);
            let assign32180_e47154: f64 = (assign32180_e47150 * assign32180_e47153);
            let assign32180_e47155: f64 = (assign32180_e47145 + assign32180_e47154);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32180_e47155, ((locals.var_isbs_dn0 * assign32180_e47144) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), ((locals.var_isbs_dn2 * assign32180_e47144) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn6 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn6)) * assign32180_e47153)), (((locals.var_isbs_dn7 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn7)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn10)) * assign32180_e47153) + (assign32180_e47150 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32180_e47144) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn12 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn12)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32180_e47144) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), );
            locals.var_ibs_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32190_e47162: f64 = (p.p178 * locals.var_vbsj);
            let assign32190_e47164: f64 = (assign32190_e47162 * locals.var_isbs2);
            let assign32190_e47165: f64 = (locals.var_ibs + assign32190_e47164);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32190_e47165, (locals.var_ibs_dn0 + (assign32190_e47162 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32190_e47162 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32190_e47162 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32190_e47162 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32190_e47162 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32190_e47162 * locals.var_isbs2_dn17)), );
            locals.var_ibs_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32200_e47172: f64 = (locals.var_gjmin * locals.var_vbdj);
            let assign32200_e47173: f64 = (locals.var_ibd + assign32200_e47172);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32200_e47173, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17, );
            locals.var_ibd_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32210_e47180: f64 = (locals.var_gjmin * locals.var_vbsj);
            let assign32210_e47181: f64 = (locals.var_ibs + assign32210_e47180);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32210_e47181, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17, );
            locals.var_ibs_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32220_e47187: f64 = (p.p179 * p.p2);
            locals.var_czbd = assign32220_e47187;
            locals.var_czbd_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32230_e47193: f64 = (p.p179 * p.p3);
            locals.var_czbs = assign32230_e47193;
            locals.var_czbs_rv = 0.0;
        }

        if (locals.var_guard1028 != 0.0) {
            let assign32240_e47199: f64 = (p.p237 - p.p238);
            locals.var_xp_max = assign32240_e47199;
            locals.var_xp_max_rv = 0.0;
        }

        let assign32250_e47204: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign32250_e47204;
        locals.var_guard1059_rv = 0.0;

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1059 != 0.0)) {
            locals.var_czbd = 0.0;
            locals.var_czbd_rv = 0.0;
            locals.var_czbs = 0.0;
            locals.var_czbs_rv = 0.0;
        }

        let assign32280_e47219: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign32280_e47219;
        locals.var_guard1060_rv = 0.0;

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
            let assign32290_e47226: f64 = (p.p5 - locals.var_w_dioscv);
            let assign32290_e47227: f64 = (p.p180 * assign32290_e47226);
            locals.var_czbssw = assign32290_e47227;
            locals.var_czbssw_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
            let assign32300_e47235: f64 = (p.p181 * locals.var_w_dioscv);
            locals.var_czbsswg = assign32300_e47235;
            locals.var_czbsswg_rv = 0.0;
        }

        let assign32310_e47240: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32310_e47240;
        locals.var_guard1061_rv = 0.0;

        let assign32320_e47243: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32320_e47243;
        locals.var_guard1062_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
            let assign32330_e47254: f64 = (locals.var_vbsj / p.p185);
            let assign32330_e47255: f64 = (1.0 - assign32330_e47254);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32330_e47255, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32340_e47260: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32340_e47260;
        locals.var_guard1063_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) {
            let assign32350_e47272: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32350_e47273: f64 = (1.0 / assign32350_e47272);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32350_e47273, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
            let assign32360_e47288: f64 = (-p.p182);
            let assign32360_e47289: f64 = (locals.var_arg__blk1055).powf(assign32360_e47288);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32360_e47289, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
            let assign32370_e47301: f64 = (p.p185 * locals.var_czbs);
            let assign32370_e47305: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32370_e47306: f64 = (1.0 - assign32370_e47305);
            let assign32370_e47307: f64 = (assign32370_e47301 * assign32370_e47306);
            let assign32370_e47310: f64 = (1.0 - p.p182);
            let assign32370_e47311: f64 = (assign32370_e47307 / assign32370_e47310);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32370_e47311, 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32370_e47310), ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32370_e47310), 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32370_e47310), 0.0, );
            locals.var_qbs_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }

        let assign32390_e47327: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32390_e47327;
        locals.var_guard1064_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
            let assign32400_e47338: f64 = (locals.var_vbsj / p.p186);
            let assign32400_e47339: f64 = (1.0 - assign32400_e47338);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32400_e47339, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32410_e47344: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32410_e47344;
        locals.var_guard1065_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
            let assign32420_e47356: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32420_e47357: f64 = (1.0 / assign32420_e47356);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32420_e47357, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
            let assign32430_e47372: f64 = (-p.p183);
            let assign32430_e47373: f64 = (locals.var_arg__blk1055).powf(assign32430_e47372);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32430_e47373, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
            let assign32440_e47386: f64 = (p.p186 * locals.var_czbssw);
            let assign32440_e47390: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32440_e47391: f64 = (1.0 - assign32440_e47390);
            let assign32440_e47392: f64 = (assign32440_e47386 * assign32440_e47391);
            let assign32440_e47395: f64 = (1.0 - p.p183);
            let assign32440_e47396: f64 = (assign32440_e47392 / assign32440_e47395);
            let assign32440_e47397: f64 = (locals.var_qbs + assign32440_e47396);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32440_e47397, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32440_e47395)), (locals.var_qbs_dn7 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32440_e47395)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32440_e47395)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }

        let assign32450_e47402: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32450_e47402;
        locals.var_guard1066_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32460_e47413: f64 = (locals.var_vbsj / p.p187);
            let assign32460_e47414: f64 = (1.0 - assign32460_e47413);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32460_e47414, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32470_e47419: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32470_e47419;
        locals.var_guard1067_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
            let assign32480_e47431: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32480_e47432: f64 = (1.0 / assign32480_e47431);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32480_e47432, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
            let assign32490_e47447: f64 = (-p.p184);
            let assign32490_e47448: f64 = (locals.var_arg__blk1055).powf(assign32490_e47447);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32490_e47448, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32500_e47461: f64 = (p.p187 * locals.var_czbsswg);
            let assign32500_e47465: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32500_e47466: f64 = (1.0 - assign32500_e47465);
            let assign32500_e47467: f64 = (assign32500_e47461 * assign32500_e47466);
            let assign32500_e47470: f64 = (1.0 - p.p184);
            let assign32500_e47471: f64 = (assign32500_e47467 / assign32500_e47470);
            let assign32500_e47472: f64 = (locals.var_qbs + assign32500_e47471);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32500_e47472, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32500_e47470)), (locals.var_qbs_dn7 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32500_e47470)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32500_e47470)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
            let assign32510_e47483: f64 = (locals.var_czbs + locals.var_czbssw);
            let assign32510_e47485: f64 = (assign32510_e47483 + locals.var_czbsswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32510_e47485, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
            let assign32520_e47496: f64 = (locals.var_czbs * p.p182);
            let assign32520_e47498: f64 = (assign32520_e47496 / p.p185);
            let assign32520_e47501: f64 = (locals.var_czbssw * p.p183);
            let assign32520_e47503: f64 = (assign32520_e47501 / p.p186);
            let assign32520_e47504: f64 = (assign32520_e47498 + assign32520_e47503);
            let assign32520_e47507: f64 = (locals.var_czbsswg * p.p184);
            let assign32520_e47509: f64 = (assign32520_e47507 / p.p187);
            let assign32520_e47510: f64 = (assign32520_e47504 + assign32520_e47509);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32520_e47510, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
            let assign32530_e47523: f64 = (locals.var_vbsj * 0.5);
            let assign32530_e47525: f64 = (assign32530_e47523 * locals.var_t2__blk1031);
            let assign32530_e47526: f64 = (locals.var_t1__blk1030 + assign32530_e47525);
            let assign32530_e47527: f64 = (locals.var_vbsj * assign32530_e47526);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32530_e47527, (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32530_e47523 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32530_e47523 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbs_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) {
            let assign32540_e47536: f64 = (p.p181 * p.p5);
            locals.var_czbsswg = assign32540_e47536;
            locals.var_czbsswg_rv = 0.0;
        }

        let assign32550_e47541: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32550_e47541;
        locals.var_guard1068_rv = 0.0;

        let assign32560_e47544: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32560_e47544;
        locals.var_guard1069_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
            let assign32570_e47556: f64 = (locals.var_vbsj / p.p185);
            let assign32570_e47557: f64 = (1.0 - assign32570_e47556);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32570_e47557, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32580_e47562: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32580_e47562;
        locals.var_guard1070_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 != 0.0)) {
            let assign32590_e47575: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32590_e47576: f64 = (1.0 / assign32590_e47575);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32590_e47576, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 == 0.0)) {
            let assign32600_e47592: f64 = (-p.p182);
            let assign32600_e47593: f64 = (locals.var_arg__blk1055).powf(assign32600_e47592);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32600_e47593, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
            let assign32610_e47606: f64 = (p.p185 * locals.var_czbs);
            let assign32610_e47610: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32610_e47611: f64 = (1.0 - assign32610_e47610);
            let assign32610_e47612: f64 = (assign32610_e47606 * assign32610_e47611);
            let assign32610_e47615: f64 = (1.0 - p.p182);
            let assign32610_e47616: f64 = (assign32610_e47612 / assign32610_e47615);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32610_e47616, 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32610_e47615), ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32610_e47615), 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32610_e47615), 0.0, );
            locals.var_qbs_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }

        let assign32630_e47633: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32630_e47633;
        locals.var_guard1071_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
            let assign32640_e47645: f64 = (locals.var_vbsj / p.p187);
            let assign32640_e47646: f64 = (1.0 - assign32640_e47645);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32640_e47646, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32650_e47651: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32650_e47651;
        locals.var_guard1072_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 != 0.0)) {
            let assign32660_e47664: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32660_e47665: f64 = (1.0 / assign32660_e47664);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32660_e47665, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) {
            let assign32670_e47681: f64 = (-p.p184);
            let assign32670_e47682: f64 = (locals.var_arg__blk1055).powf(assign32670_e47681);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32670_e47682, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
            let assign32680_e47696: f64 = (p.p187 * locals.var_czbsswg);
            let assign32680_e47700: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32680_e47701: f64 = (1.0 - assign32680_e47700);
            let assign32680_e47702: f64 = (assign32680_e47696 * assign32680_e47701);
            let assign32680_e47705: f64 = (1.0 - p.p184);
            let assign32680_e47706: f64 = (assign32680_e47702 / assign32680_e47705);
            let assign32680_e47707: f64 = (locals.var_qbs + assign32680_e47706);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32680_e47707, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32680_e47705)), (locals.var_qbs_dn7 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32680_e47705)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32680_e47705)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
            let assign32690_e47719: f64 = (locals.var_czbs + locals.var_czbsswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32690_e47719, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
            let assign32700_e47731: f64 = (locals.var_czbs * p.p182);
            let assign32700_e47733: f64 = (assign32700_e47731 / p.p185);
            let assign32700_e47736: f64 = (locals.var_czbsswg * p.p184);
            let assign32700_e47738: f64 = (assign32700_e47736 / p.p187);
            let assign32700_e47739: f64 = (assign32700_e47733 + assign32700_e47738);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32700_e47739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
            let assign32710_e47753: f64 = (locals.var_vbsj * 0.5);
            let assign32710_e47755: f64 = (assign32710_e47753 * locals.var_t2__blk1031);
            let assign32710_e47756: f64 = (locals.var_t1__blk1030 + assign32710_e47755);
            let assign32710_e47757: f64 = (locals.var_vbsj * assign32710_e47756);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32710_e47757, (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32710_e47753 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32710_e47753 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbs_rv = 0.0;
        }

        let assign32720_e47762: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32720_e47762;
        locals.var_guard1073_rv = 0.0;

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
            let assign32730_e47769: f64 = (p.p4 - locals.var_w_diodcv);
            let assign32730_e47770: f64 = (p.p180 * assign32730_e47769);
            locals.var_czbdsw = assign32730_e47770;
            locals.var_czbdsw_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
            let assign32740_e47778: f64 = (p.p181 * locals.var_w_diodcv);
            locals.var_czbdswg = assign32740_e47778;
            locals.var_czbdswg_rv = 0.0;
        }

        let assign32750_e47783: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32750_e47783;
        locals.var_guard1074_rv = 0.0;

        let assign32760_e47786: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32760_e47786;
        locals.var_guard1075_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
            let assign32770_e47797: f64 = (locals.var_vbdj / p.p185);
            let assign32770_e47798: f64 = (1.0 - assign32770_e47797);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32770_e47798, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32780_e47803: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32780_e47803;
        locals.var_guard1076_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) {
            let assign32790_e47815: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32790_e47816: f64 = (1.0 / assign32790_e47815);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32790_e47816, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
            let assign32800_e47831: f64 = (-p.p182);
            let assign32800_e47832: f64 = (locals.var_arg__blk1055).powf(assign32800_e47831);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32800_e47832, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
            let assign32810_e47844: f64 = (p.p185 * locals.var_czbd);
            let assign32810_e47848: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32810_e47849: f64 = (1.0 - assign32810_e47848);
            let assign32810_e47850: f64 = (assign32810_e47844 * assign32810_e47849);
            let assign32810_e47853: f64 = (1.0 - p.p182);
            let assign32810_e47854: f64 = (assign32810_e47850 / assign32810_e47853);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32810_e47854, 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32810_e47853), ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32810_e47853), 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32810_e47853), 0.0, );
            locals.var_qbd_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }

        let assign32830_e47870: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32830_e47870;
        locals.var_guard1077_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
            let assign32840_e47881: f64 = (locals.var_vbdj / p.p186);
            let assign32840_e47882: f64 = (1.0 - assign32840_e47881);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32840_e47882, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32850_e47887: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32850_e47887;
        locals.var_guard1078_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) {
            let assign32860_e47899: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32860_e47900: f64 = (1.0 / assign32860_e47899);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32860_e47900, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
            let assign32870_e47915: f64 = (-p.p183);
            let assign32870_e47916: f64 = (locals.var_arg__blk1055).powf(assign32870_e47915);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32870_e47916, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
            let assign32880_e47929: f64 = (p.p186 * locals.var_czbdsw);
            let assign32880_e47933: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32880_e47934: f64 = (1.0 - assign32880_e47933);
            let assign32880_e47935: f64 = (assign32880_e47929 * assign32880_e47934);
            let assign32880_e47938: f64 = (1.0 - p.p183);
            let assign32880_e47939: f64 = (assign32880_e47935 / assign32880_e47938);
            let assign32880_e47940: f64 = (locals.var_qbd + assign32880_e47939);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32880_e47940, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32880_e47938)), (locals.var_qbd_dn7 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32880_e47938)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32880_e47938)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }

        let assign32890_e47945: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32890_e47945;
        locals.var_guard1079_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32900_e47956: f64 = (locals.var_vbdj / p.p187);
            let assign32900_e47957: f64 = (1.0 - assign32900_e47956);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32900_e47957, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign32910_e47962: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32910_e47962;
        locals.var_guard1080_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
            let assign32920_e47974: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32920_e47975: f64 = (1.0 / assign32920_e47974);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32920_e47975, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
            let assign32930_e47990: f64 = (-p.p184);
            let assign32930_e47991: f64 = (locals.var_arg__blk1055).powf(assign32930_e47990);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32930_e47991, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32940_e48004: f64 = (p.p187 * locals.var_czbdswg);
            let assign32940_e48008: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32940_e48009: f64 = (1.0 - assign32940_e48008);
            let assign32940_e48010: f64 = (assign32940_e48004 * assign32940_e48009);
            let assign32940_e48013: f64 = (1.0 - p.p184);
            let assign32940_e48014: f64 = (assign32940_e48010 / assign32940_e48013);
            let assign32940_e48015: f64 = (locals.var_qbd + assign32940_e48014);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32940_e48015, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32940_e48013)), (locals.var_qbd_dn7 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32940_e48013)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32940_e48013)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32950_e48026: f64 = (locals.var_czbd + locals.var_czbdsw);
            let assign32950_e48028: f64 = (assign32950_e48026 + locals.var_czbdswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32950_e48028, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32960_e48039: f64 = (locals.var_czbd * p.p182);
            let assign32960_e48041: f64 = (assign32960_e48039 / p.p185);
            let assign32960_e48044: f64 = (locals.var_czbdsw * p.p183);
            let assign32960_e48046: f64 = (assign32960_e48044 / p.p186);
            let assign32960_e48047: f64 = (assign32960_e48041 + assign32960_e48046);
            let assign32960_e48050: f64 = (locals.var_czbdswg * p.p184);
            let assign32960_e48052: f64 = (assign32960_e48050 / p.p187);
            let assign32960_e48053: f64 = (assign32960_e48047 + assign32960_e48052);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32960_e48053, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32970_e48066: f64 = (locals.var_vbdj * 0.5);
            let assign32970_e48068: f64 = (assign32970_e48066 * locals.var_t2__blk1031);
            let assign32970_e48069: f64 = (locals.var_t1__blk1030 + assign32970_e48068);
            let assign32970_e48070: f64 = (locals.var_vbdj * assign32970_e48069);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32970_e48070, (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign32970_e48066 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign32970_e48066 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbd_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) {
            let assign32980_e48079: f64 = (p.p181 * p.p4);
            locals.var_czbdswg = assign32980_e48079;
            locals.var_czbdswg_rv = 0.0;
        }

        let assign32990_e48084: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32990_e48084;
        locals.var_guard1081_rv = 0.0;

        let assign33000_e48087: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign33000_e48087;
        locals.var_guard1082_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
            let assign33010_e48099: f64 = (locals.var_vbdj / p.p185);
            let assign33010_e48100: f64 = (1.0 - assign33010_e48099);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign33010_e48100, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign33020_e48105: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign33020_e48105;
        locals.var_guard1083_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
            let assign33030_e48118: f64 = (locals.var_arg__blk1055).sqrt();
            let assign33030_e48119: f64 = (1.0 / assign33030_e48118);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33030_e48119, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 == 0.0)) {
            let assign33040_e48135: f64 = (-p.p182);
            let assign33040_e48136: f64 = (locals.var_arg__blk1055).powf(assign33040_e48135);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33040_e48136, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
            let assign33050_e48149: f64 = (p.p185 * locals.var_czbd);
            let assign33050_e48153: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign33050_e48154: f64 = (1.0 - assign33050_e48153);
            let assign33050_e48155: f64 = (assign33050_e48149 * assign33050_e48154);
            let assign33050_e48158: f64 = (1.0 - p.p182);
            let assign33050_e48159: f64 = (assign33050_e48155 / assign33050_e48158);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33050_e48159, 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33050_e48158), ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33050_e48158), 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33050_e48158), 0.0, );
            locals.var_qbd_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }

        let assign33070_e48176: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign33070_e48176;
        locals.var_guard1084_rv = 0.0;

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
            let assign33080_e48188: f64 = (locals.var_vbdj / p.p187);
            let assign33080_e48189: f64 = (1.0 - assign33080_e48188);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign33080_e48189, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }

        let assign33090_e48194: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33090_e48194;
        locals.var_guard1085_rv = 0.0;

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 != 0.0)) {
            let assign33100_e48207: f64 = (locals.var_arg__blk1055).sqrt();
            let assign33100_e48208: f64 = (1.0 / assign33100_e48207);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33100_e48208, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), );
            locals.var_sarg_rv = 0.0;
        }

        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 == 0.0)) {
            let assign33110_e48224: f64 = (-p.p184);
            let assign33110_e48225: f64 = (locals.var_arg__blk1055).powf(assign33110_e48224);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33110_e48225, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }

        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
            let assign33120_e48239: f64 = (p.p187 * locals.var_czbdswg);
            let assign33120_e48243: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign33120_e48244: f64 = (1.0 - assign33120_e48243);
            let assign33120_e48245: f64 = (assign33120_e48239 * assign33120_e48244);
            let assign33120_e48248: f64 = (1.0 - p.p184);
            let assign33120_e48249: f64 = (assign33120_e48245 / assign33120_e48248);
            let assign33120_e48250: f64 = (locals.var_qbd + assign33120_e48249);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33120_e48250, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33120_e48248)), (locals.var_qbd_dn7 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33120_e48248)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33120_e48248)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
            let assign33130_e48262: f64 = (locals.var_czbd + locals.var_czbdswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign33130_e48262, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
            let assign33140_e48274: f64 = (locals.var_czbd * p.p182);
            let assign33140_e48276: f64 = (assign33140_e48274 / p.p185);
            let assign33140_e48279: f64 = (locals.var_czbdswg * p.p184);
            let assign33140_e48281: f64 = (assign33140_e48279 / p.p187);
            let assign33140_e48282: f64 = (assign33140_e48276 + assign33140_e48281);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign33140_e48282, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }

        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
            let assign33150_e48296: f64 = (locals.var_vbdj * 0.5);
            let assign33150_e48298: f64 = (assign33150_e48296 * locals.var_t2__blk1031);
            let assign33150_e48299: f64 = (locals.var_t1__blk1030 + assign33150_e48298);
            let assign33150_e48300: f64 = (locals.var_vbdj * assign33150_e48299);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33150_e48300, (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign33150_e48296 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign33150_e48296 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbd_rv = 0.0;
        }

        let assign33160_e48305: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33160_e48305;
        locals.var_guard1086_rv = 0.0;

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33170_e48310: f64 = (-1.6021918e-19);
            let assign33170_e48312: f64 = (assign33170_e48310 * locals.var_uc_nsubs);
            let assign33170_e48314: f64 = (assign33170_e48312 * locals.var_xp_max);
            let assign33170_e48316: f64 = (assign33170_e48314 * p.p3);
            (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17, ) = (assign33170_e48316, (((assign33170_e48310 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3), );
            locals.var_qbs_max_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33180_e48324: f64 = (-locals.var_qbs_max);
            let assign33180_e48325: f64 = (0.001 * assign33180_e48324);
            (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17, ) = (assign33180_e48325, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)), );
            locals.var_dlt_qbs_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33190_e48332: f64 = (-locals.var_qbs_max);
            let assign33190_e48334: f64 = (-locals.var_qbs);
            let assign33190_e48335: f64 = (assign33190_e48332 - assign33190_e48334);
            let assign33190_e48337: f64 = (assign33190_e48335 - locals.var_dlt_qbs);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33190_e48337, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17), );
            locals.var_tmf1_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33200_e48345: f64 = (-locals.var_qbs_max);
            let assign33200_e48346: f64 = (4.0 * assign33200_e48345);
            let assign33200_e48348: f64 = (assign33200_e48346 * locals.var_dlt_qbs);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33200_e48348, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33210_e48359: f64 = (-locals.var_tmf2);
        (assign33210_e48359, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33220_e48368: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33220_e48370: f64 = (assign33220_e48368 + locals.var_tmf2);
            let assign33220_e48371: f64 = (assign33220_e48370).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33220_e48371, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33220_e48371)), );
            locals.var_tmf2_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33230_e48378: f64 = (-locals.var_qbs_max);
            let assign33230_e48382: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33230_e48383: f64 = (0.5 * assign33230_e48382);
            let assign33230_e48384: f64 = (assign33230_e48378 - assign33230_e48383);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33230_e48384, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbs_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33240_e48392: f64 = (-1.0);
            let assign33240_e48393: f64 = (locals.var_qbs * assign33240_e48392);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33240_e48393, (locals.var_qbs_dn0 * assign33240_e48392), (locals.var_qbs_dn2 * assign33240_e48392), (locals.var_qbs_dn6 * assign33240_e48392), (locals.var_qbs_dn7 * assign33240_e48392), (locals.var_qbs_dn10 * assign33240_e48392), (locals.var_qbs_dn11 * assign33240_e48392), (locals.var_qbs_dn12 * assign33240_e48392), (locals.var_qbs_dn17 * assign33240_e48392), );
            locals.var_qbs_rv = 0.0;
        }

        let assign33250_e48398: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33250_e48398;
        locals.var_guard1087_rv = 0.0;

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33260_e48403: f64 = (-1.6021918e-19);
            let assign33260_e48405: f64 = (assign33260_e48403 * locals.var_uc_nsubs);
            let assign33260_e48407: f64 = (assign33260_e48405 * locals.var_xp_max);
            let assign33260_e48409: f64 = (assign33260_e48407 * p.p2);
            (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17, ) = (assign33260_e48409, (((assign33260_e48403 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2), );
            locals.var_qbd_max_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33270_e48417: f64 = (-locals.var_qbd_max);
            let assign33270_e48418: f64 = (0.001 * assign33270_e48417);
            (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17, ) = (assign33270_e48418, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)), );
            locals.var_dlt_qbd_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33280_e48425: f64 = (-locals.var_qbd_max);
            let assign33280_e48427: f64 = (-locals.var_qbd);
            let assign33280_e48428: f64 = (assign33280_e48425 - assign33280_e48427);
            let assign33280_e48430: f64 = (assign33280_e48428 - locals.var_dlt_qbd);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33280_e48430, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17), );
            locals.var_tmf1_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33290_e48438: f64 = (-locals.var_qbd_max);
            let assign33290_e48439: f64 = (4.0 * assign33290_e48438);
            let assign33290_e48441: f64 = (assign33290_e48439 * locals.var_dlt_qbd);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33290_e48441, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33300_e48452: f64 = (-locals.var_tmf2);
        (assign33300_e48452, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33310_e48461: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33310_e48463: f64 = (assign33310_e48461 + locals.var_tmf2);
            let assign33310_e48464: f64 = (assign33310_e48463).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33310_e48464, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33310_e48464)), );
            locals.var_tmf2_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33320_e48471: f64 = (-locals.var_qbd_max);
            let assign33320_e48475: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33320_e48476: f64 = (0.5 * assign33320_e48475);
            let assign33320_e48477: f64 = (assign33320_e48471 - assign33320_e48476);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33320_e48477, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbd_rv = 0.0;
        }

        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33330_e48485: f64 = (-1.0);
            let assign33330_e48486: f64 = (locals.var_qbd * assign33330_e48485);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33330_e48486, (locals.var_qbd_dn0 * assign33330_e48485), (locals.var_qbd_dn2 * assign33330_e48485), (locals.var_qbd_dn6 * assign33330_e48485), (locals.var_qbd_dn7 * assign33330_e48485), (locals.var_qbd_dn10 * assign33330_e48485), (locals.var_qbd_dn11 * assign33330_e48485), (locals.var_qbd_dn12 * assign33330_e48485), (locals.var_qbd_dn17 * assign33330_e48485), );
            locals.var_qbd_rv = 0.0;
        }

        let assign33560_e48742: f64 = if ((p.p32 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1120 = assign33560_e48742;
        locals.var_guard1120_rv = 0.0;

        if (locals.var_guard1120 != 0.0) {
            let assign33570_e48746: f64 = (locals.var_psdl - locals.var_ps0);
            let assign33570_e48748: f64 = (assign33570_e48746 / locals.var_lch);
            (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12, locals.var_eyd_dn17, ) = (assign33570_e48748, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn17 - locals.var_ps0_dn17) * locals.var_lch) - (assign33570_e48746 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)), );
            locals.var_eyd_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33580_e48754: f64 = (locals.var_muun * locals.var_eyd);
            let assign33580_e48756: f64 = (assign33580_e48754 / 100000.0);
            (locals.var_t12__blk1104, locals.var_t12__blk1104_dn0, locals.var_t12__blk1104_dn2, locals.var_t12__blk1104_dn6, locals.var_t12__blk1104_dn7, locals.var_t12__blk1104_dn10, locals.var_t12__blk1104_dn11, locals.var_t12__blk1104_dn12, locals.var_t12__blk1104_dn17, ) = (assign33580_e48756, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 100000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 100000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 100000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 100000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 100000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 100000.0), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / 100000.0), (((locals.var_muun_dn17 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn17)) / 100000.0), );
            locals.var_t12__blk1104_rv = 0.0;
        }

        let assign33590_e48762: f64 = (10.0 * 2.220446049250313e-16);
        let assign33590_e48763: f64 = (1.0 - assign33590_e48762);
        let assign33590_e48770: f64 = (10.0 * 2.220446049250313e-16);
        let assign33590_e48771: f64 = (1.0 + assign33590_e48770);
        let assign33590_e48773: f64 = if ((assign33590_e48763 <= p.p113) && (p.p113 <= assign33590_e48771)) { 1.0 } else { 0.0 };
        locals.var_guard1121 = assign33590_e48773;
        locals.var_guard1121_rv = 0.0;

        if ((locals.var_guard1120 != 0.0) && (locals.var_guard1121 != 0.0)) {
            (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t7__blk1105_rv = 0.0;
        }

        let assign33610_e48783: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48784: f64 = (2.0 - assign33610_e48783);
        let assign33610_e48791: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48792: f64 = (2.0 + assign33610_e48791);
        let assign33610_e48794: f64 = if ((assign33610_e48784 <= p.p113) && (p.p113 <= assign33610_e48792)) { 1.0 } else { 0.0 };
        locals.var_guard1122 = assign33610_e48794;
        locals.var_guard1122_rv = 0.0;

        if (((locals.var_guard1120 != 0.0) && (locals.var_guard1121 == 0.0)) && (locals.var_guard1122 != 0.0)) {
            (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17, ) = (locals.var_t12__blk1104, locals.var_t12__blk1104_dn0, locals.var_t12__blk1104_dn2, locals.var_t12__blk1104_dn6, locals.var_t12__blk1104_dn7, locals.var_t12__blk1104_dn10, locals.var_t12__blk1104_dn11, locals.var_t12__blk1104_dn12, locals.var_t12__blk1104_dn17, );
            locals.var_t7__blk1105_rv = 0.0;
        }

        if (((locals.var_guard1120 != 0.0) && (locals.var_guard1121 == 0.0)) && (locals.var_guard1122 == 0.0)) {
            let assign33630_e48814: f64 = (p.p113 - 1.0);
            let assign33630_e48815: f64 = (locals.var_t12__blk1104).powf(assign33630_e48814);
            (locals.var_t7__blk1105, locals.var_t7__blk1105_dn0, locals.var_t7__blk1105_dn2, locals.var_t7__blk1105_dn6, locals.var_t7__blk1105_dn7, locals.var_t7__blk1105_dn10, locals.var_t7__blk1105_dn11, locals.var_t7__blk1105_dn12, locals.var_t7__blk1105_dn17, ) = (assign33630_e48815, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn0)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn0 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn2)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn2 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn6)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn6 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn7)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn7 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn10)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn10 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn11)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn11 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn12)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn12 / locals.var_t12__blk1104))) }, if 0.0 == 0.0 && ((assign33630_e48814) as f64).is_finite() && ((assign33630_e48814) as f64).fract() == 0.0 { if assign33630_e48814 == 0.0 { 0.0 } else { (assign33630_e48814 * ((locals.var_t12__blk1104).powf(assign33630_e48814 - 1.0) * locals.var_t12__blk1104_dn17)) } } else { (assign33630_e48815 * (assign33630_e48814 * (locals.var_t12__blk1104_dn17 / locals.var_t12__blk1104))) }, );
            locals.var_t7__blk1105_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33640_e48821: f64 = (locals.var_t12__blk1104 * locals.var_t7__blk1105);
            (locals.var_t8__blk1106, locals.var_t8__blk1106_dn0, locals.var_t8__blk1106_dn2, locals.var_t8__blk1106_dn6, locals.var_t8__blk1106_dn7, locals.var_t8__blk1106_dn10, locals.var_t8__blk1106_dn11, locals.var_t8__blk1106_dn12, locals.var_t8__blk1106_dn17, ) = (assign33640_e48821, ((locals.var_t12__blk1104_dn0 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn0)), ((locals.var_t12__blk1104_dn2 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn2)), ((locals.var_t12__blk1104_dn6 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn6)), ((locals.var_t12__blk1104_dn7 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn7)), ((locals.var_t12__blk1104_dn10 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn10)), ((locals.var_t12__blk1104_dn11 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn11)), ((locals.var_t12__blk1104_dn12 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn12)), ((locals.var_t12__blk1104_dn17 * locals.var_t7__blk1105) + (locals.var_t12__blk1104 * locals.var_t7__blk1105_dn17)), );
            locals.var_t8__blk1106_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33650_e48827: f64 = (1.0 + locals.var_t8__blk1106);
            (locals.var_t9__blk1107, locals.var_t9__blk1107_dn0, locals.var_t9__blk1107_dn2, locals.var_t9__blk1107_dn6, locals.var_t9__blk1107_dn7, locals.var_t9__blk1107_dn10, locals.var_t9__blk1107_dn11, locals.var_t9__blk1107_dn12, locals.var_t9__blk1107_dn17, ) = (assign33650_e48827, locals.var_t8__blk1106_dn0, locals.var_t8__blk1106_dn2, locals.var_t8__blk1106_dn6, locals.var_t8__blk1106_dn7, locals.var_t8__blk1106_dn10, locals.var_t8__blk1106_dn11, locals.var_t8__blk1106_dn12, locals.var_t8__blk1106_dn17, );
            locals.var_t9__blk1107_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33660_e48833: f64 = (-1.0);
            let assign33660_e48835: f64 = (assign33660_e48833 / p.p113);
            let assign33660_e48837: f64 = (assign33660_e48835 - 1.0);
            let assign33660_e48838: f64 = (locals.var_t9__blk1107).powf(assign33660_e48837);
            (locals.var_t10__blk1108, locals.var_t10__blk1108_dn0, locals.var_t10__blk1108_dn2, locals.var_t10__blk1108_dn6, locals.var_t10__blk1108_dn7, locals.var_t10__blk1108_dn10, locals.var_t10__blk1108_dn11, locals.var_t10__blk1108_dn12, locals.var_t10__blk1108_dn17, ) = (assign33660_e48838, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn0)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn0 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn2)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn2 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn6)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn6 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn7)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn7 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn10)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn10 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn11)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn11 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn12)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn12 / locals.var_t9__blk1107))) }, if 0.0 == 0.0 && ((assign33660_e48837) as f64).is_finite() && ((assign33660_e48837) as f64).fract() == 0.0 { if assign33660_e48837 == 0.0 { 0.0 } else { (assign33660_e48837 * ((locals.var_t9__blk1107).powf(assign33660_e48837 - 1.0) * locals.var_t9__blk1107_dn17)) } } else { (assign33660_e48838 * (assign33660_e48837 * (locals.var_t9__blk1107_dn17 / locals.var_t9__blk1107))) }, );
            locals.var_t10__blk1108_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33670_e48844: f64 = (locals.var_t9__blk1107 * locals.var_t10__blk1108);
            (locals.var_t11__blk1109, locals.var_t11__blk1109_dn0, locals.var_t11__blk1109_dn2, locals.var_t11__blk1109_dn6, locals.var_t11__blk1109_dn7, locals.var_t11__blk1109_dn10, locals.var_t11__blk1109_dn11, locals.var_t11__blk1109_dn12, locals.var_t11__blk1109_dn17, ) = (assign33670_e48844, ((locals.var_t9__blk1107_dn0 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn0)), ((locals.var_t9__blk1107_dn2 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn2)), ((locals.var_t9__blk1107_dn6 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn6)), ((locals.var_t9__blk1107_dn7 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn7)), ((locals.var_t9__blk1107_dn10 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn10)), ((locals.var_t9__blk1107_dn11 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn11)), ((locals.var_t9__blk1107_dn12 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn12)), ((locals.var_t9__blk1107_dn17 * locals.var_t10__blk1108) + (locals.var_t9__blk1107 * locals.var_t10__blk1108_dn17)), );
            locals.var_t11__blk1109_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33680_e48850: f64 = (locals.var_muun * locals.var_t11__blk1109);
            (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12, locals.var_mud_hoso_dn17, ) = (assign33680_e48850, ((locals.var_muun_dn0 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn0)), ((locals.var_muun_dn2 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn2)), ((locals.var_muun_dn6 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn6)), ((locals.var_muun_dn7 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn7)), ((locals.var_muun_dn10 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn10)), ((locals.var_muun_dn11 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn11)), ((locals.var_muun_dn12 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn12)), ((locals.var_muun_dn17 * locals.var_t11__blk1109) + (locals.var_muun * locals.var_t11__blk1109_dn17)), );
            locals.var_mud_hoso_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33690_e48856: f64 = (locals.var_mu + locals.var_mud_hoso);
            let assign33690_e48858: f64 = (assign33690_e48856 / 2.0);
            (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12, locals.var_mu_ave_dn17, ) = (assign33690_e48858, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0), ((locals.var_mu_dn17 + locals.var_mud_hoso_dn17) / 2.0), );
            locals.var_mu_ave_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33700_e48864: f64 = (locals.var_alpha * locals.var_alpha);
            (locals.var_t0__blk1110, locals.var_t0__blk1110_dn0, locals.var_t0__blk1110_dn2, locals.var_t0__blk1110_dn6, locals.var_t0__blk1110_dn7, locals.var_t0__blk1110_dn10, locals.var_t0__blk1110_dn11, locals.var_t0__blk1110_dn12, locals.var_t0__blk1110_dn17, ) = (assign33700_e48864, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn17)), );
            locals.var_t0__blk1110_rv = 0.0;
        }

        if (locals.var_guard1120 != 0.0) {
            let assign33710_e48870: f64 = (locals.var_weff_nf * locals.var_c_fox);
            let assign33710_e48872: f64 = (assign33710_e48870 * locals.var_vgvt);
            let assign33710_e48874: f64 = (assign33710_e48872 * locals.var_mu);
            let assign33710_e48878: f64 = (3.0 * locals.var_alpha);
            let assign33710_e48879: f64 = (1.0 + assign33710_e48878);
            let assign33710_e48882: f64 = (6.0 * locals.var_t0__blk1110);
            let assign33710_e48883: f64 = (assign33710_e48879 + assign33710_e48882);
            let assign33710_e48885: f64 = (assign33710_e48883 * locals.var_mud_hoso);
            let assign33710_e48887: f64 = (assign33710_e48885 * locals.var_mud_hoso);
            let assign33710_e48891: f64 = (4.0 * locals.var_alpha);
            let assign33710_e48892: f64 = (3.0 + assign33710_e48891);
            let assign33710_e48895: f64 = (3.0 * locals.var_t0__blk1110);
            let assign33710_e48896: f64 = (assign33710_e48892 + assign33710_e48895);
            let assign33710_e48898: f64 = (assign33710_e48896 * locals.var_mud_hoso);
            let assign33710_e48900: f64 = (assign33710_e48898 * locals.var_mu);
            let assign33710_e48901: f64 = (assign33710_e48887 + assign33710_e48900);
            let assign33710_e48905: f64 = (3.0 * locals.var_alpha);
            let assign33710_e48906: f64 = (6.0 + assign33710_e48905);
            let assign33710_e48908: f64 = (assign33710_e48906 + locals.var_t0__blk1110);
            let assign33710_e48910: f64 = (assign33710_e48908 * locals.var_mu);
            let assign33710_e48912: f64 = (assign33710_e48910 * locals.var_mu);
            let assign33710_e48913: f64 = (assign33710_e48901 + assign33710_e48912);
            let assign33710_e48914: f64 = (assign33710_e48874 * assign33710_e48913);
            let assign33710_e48917: f64 = (15.0 * locals.var_lch);
            let assign33710_e48920: f64 = (1.0 + locals.var_alpha);
            let assign33710_e48921: f64 = (assign33710_e48917 * assign33710_e48920);
            let assign33710_e48923: f64 = (assign33710_e48921 * locals.var_mu_ave);
            let assign33710_e48925: f64 = (assign33710_e48923 * locals.var_mu_ave);
            let assign33710_e48926: f64 = (assign33710_e48914 / assign33710_e48925);
            (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17, ) = (assign33710_e48926, ((((((((((locals.var_weff_nf * locals.var_c_fox_dn0) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn0)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0__blk1110_dn0)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0__blk1110_dn0)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0__blk1110_dn0) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn0)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn0))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn0) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn0)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn2) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn2)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0__blk1110_dn2)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0__blk1110_dn2)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0__blk1110_dn2) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn2)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn2))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn2) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn2)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn6) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn6)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0__blk1110_dn6)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0__blk1110_dn6)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0__blk1110_dn6) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn6)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn6))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn6) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn6)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn7) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn7)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0__blk1110_dn7)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0__blk1110_dn7)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0__blk1110_dn7) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn7)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn7))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn7) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn7)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn10) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn10)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0__blk1110_dn10)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0__blk1110_dn10)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0__blk1110_dn10) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn10)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn10))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn10) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn10)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn11) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn11)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0__blk1110_dn11)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0__blk1110_dn11)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0__blk1110_dn11) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn11)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn11))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn11) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn11)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn12) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn12)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0__blk1110_dn12)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0__blk1110_dn12)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0__blk1110_dn12) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn12)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn12))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn12) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn12)))) / (assign33710_e48925 * assign33710_e48925)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn17) * locals.var_vgvt) + (assign33710_e48870 * locals.var_vgvt_dn17)) * locals.var_mu) + (assign33710_e48872 * locals.var_mu_dn17)) * assign33710_e48913) + (assign33710_e48874 * ((((((((3.0 * locals.var_alpha_dn17) + (6.0 * locals.var_t0__blk1110_dn17)) * locals.var_mud_hoso) + (assign33710_e48883 * locals.var_mud_hoso_dn17)) * locals.var_mud_hoso) + (assign33710_e48885 * locals.var_mud_hoso_dn17)) + ((((((4.0 * locals.var_alpha_dn17) + (3.0 * locals.var_t0__blk1110_dn17)) * locals.var_mud_hoso) + (assign33710_e48896 * locals.var_mud_hoso_dn17)) * locals.var_mu) + (assign33710_e48898 * locals.var_mu_dn17))) + ((((((3.0 * locals.var_alpha_dn17) + locals.var_t0__blk1110_dn17) * locals.var_mu) + (assign33710_e48908 * locals.var_mu_dn17)) * locals.var_mu) + (assign33710_e48910 * locals.var_mu_dn17))))) * assign33710_e48925) - (assign33710_e48914 * (((((((15.0 * locals.var_lch_dn17) * assign33710_e48920) + (assign33710_e48917 * locals.var_alpha_dn17)) * locals.var_mu_ave) + (assign33710_e48921 * locals.var_mu_ave_dn17)) * locals.var_mu_ave) + (assign33710_e48923 * locals.var_mu_ave_dn17)))) / (assign33710_e48925 * assign33710_e48925)), );
            locals.var_nthrml_rv = 0.0;
        }

        if (locals.var_guard1120 == 0.0) {
            (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_nthrml_rv = 0.0;
        }

        let assign33730_e48947: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1123 = assign33730_e48947;
        locals.var_guard1123_rv = 0.0;

        if (locals.var_guard1123 != 0.0) {
            let assign33740_e48950: f64 = (locals.var_kusail).sqrt();
            (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12, locals.var_sqrtkusail_dn17, ) = (assign33740_e48950, (locals.var_kusail_dn0 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn2 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn6 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn7 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn10 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn11 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn12 / (2.0 * assign33740_e48950)), (locals.var_kusail_dn17 / (2.0 * assign33740_e48950)), );
            locals.var_sqrtkusail_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33750_e48956: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
            (locals.var_t2__blk1112, locals.var_t2__blk1112_dn0, locals.var_t2__blk1112_dn2, locals.var_t2__blk1112_dn6, locals.var_t2__blk1112_dn7, locals.var_t2__blk1112_dn10, locals.var_t2__blk1112_dn11, locals.var_t2__blk1112_dn12, locals.var_t2__blk1112_dn17, ) = (assign33750_e48956, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12), (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17), );
            locals.var_t2__blk1112_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33760_e48962: f64 = (locals.var_kusai00 * locals.var_kusai00);
            (locals.var_t3__blk1113, locals.var_t3__blk1113_dn0, locals.var_t3__blk1113_dn2, locals.var_t3__blk1113_dn6, locals.var_t3__blk1113_dn7, locals.var_t3__blk1113_dn10, locals.var_t3__blk1113_dn11, locals.var_t3__blk1113_dn12, locals.var_t3__blk1113_dn17, ) = (assign33760_e48962, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)), ((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)), );
            locals.var_t3__blk1113_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33770_e48968: f64 = (locals.var_kusail * locals.var_kusail);
            (locals.var_t4__blk1114, locals.var_t4__blk1114_dn0, locals.var_t4__blk1114_dn2, locals.var_t4__blk1114_dn6, locals.var_t4__blk1114_dn7, locals.var_t4__blk1114_dn10, locals.var_t4__blk1114_dn11, locals.var_t4__blk1114_dn12, locals.var_t4__blk1114_dn17, ) = (assign33770_e48968, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)), ((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)), );
            locals.var_t4__blk1114_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33780_e48974: f64 = (42.0 * locals.var_kusai00);
            let assign33780_e48976: f64 = (assign33780_e48974 * locals.var_kusail);
            (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17, ) = (assign33780_e48976, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn12)), (((42.0 * locals.var_kusai00_dn17) * locals.var_kusail) + (assign33780_e48974 * locals.var_kusail_dn17)), );
            locals.var_t5__blk1115_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33790_e48984: f64 = (locals.var_t3__blk1113 + locals.var_t4__blk1114);
            let assign33790_e48985: f64 = (4.0 * assign33790_e48984);
            let assign33790_e48986: f64 = (locals.var_t5__blk1115 + assign33790_e48985);
            (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17, ) = (assign33790_e48986, (locals.var_t5__blk1115_dn0 + (4.0 * (locals.var_t3__blk1113_dn0 + locals.var_t4__blk1114_dn0))), (locals.var_t5__blk1115_dn2 + (4.0 * (locals.var_t3__blk1113_dn2 + locals.var_t4__blk1114_dn2))), (locals.var_t5__blk1115_dn6 + (4.0 * (locals.var_t3__blk1113_dn6 + locals.var_t4__blk1114_dn6))), (locals.var_t5__blk1115_dn7 + (4.0 * (locals.var_t3__blk1113_dn7 + locals.var_t4__blk1114_dn7))), (locals.var_t5__blk1115_dn10 + (4.0 * (locals.var_t3__blk1113_dn10 + locals.var_t4__blk1114_dn10))), (locals.var_t5__blk1115_dn11 + (4.0 * (locals.var_t3__blk1113_dn11 + locals.var_t4__blk1114_dn11))), (locals.var_t5__blk1115_dn12 + (4.0 * (locals.var_t3__blk1113_dn12 + locals.var_t4__blk1114_dn12))), (locals.var_t5__blk1115_dn17 + (4.0 * (locals.var_t3__blk1113_dn17 + locals.var_t4__blk1114_dn17))), );
            locals.var_t5__blk1115_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33800_e48993: f64 = (20.0 * locals.var_sqrtkusail);
            let assign33800_e48995: f64 = (assign33800_e48993 * locals.var_vgvt);
            let assign33800_e48998: f64 = (locals.var_kusai00 + locals.var_kusail);
            let assign33800_e48999: f64 = (assign33800_e48995 * assign33800_e48998);
            let assign33800_e49000: f64 = (locals.var_t5__blk1115 + assign33800_e48999);
            (locals.var_t5__blk1115, locals.var_t5__blk1115_dn0, locals.var_t5__blk1115_dn2, locals.var_t5__blk1115_dn6, locals.var_t5__blk1115_dn7, locals.var_t5__blk1115_dn10, locals.var_t5__blk1115_dn11, locals.var_t5__blk1115_dn12, locals.var_t5__blk1115_dn17, ) = (assign33800_e49000, (locals.var_t5__blk1115_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn0)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5__blk1115_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn2)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5__blk1115_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn6)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5__blk1115_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn7)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5__blk1115_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn10)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5__blk1115_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn11)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5__blk1115_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn12)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))), (locals.var_t5__blk1115_dn17 + (((((20.0 * locals.var_sqrtkusail_dn17) * locals.var_vgvt) + (assign33800_e48993 * locals.var_vgvt_dn17)) * assign33800_e48998) + (assign33800_e48995 * (locals.var_kusai00_dn17 + locals.var_kusail_dn17)))), );
            locals.var_t5__blk1115_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33810_e49006: f64 = (locals.var_t2__blk1112 * locals.var_t2__blk1112);
            (locals.var_t10w, locals.var_t10w_dn0, locals.var_t10w_dn2, locals.var_t10w_dn6, locals.var_t10w_dn7, locals.var_t10w_dn10, locals.var_t10w_dn11, locals.var_t10w_dn12, locals.var_t10w_dn17, ) = (assign33810_e49006, ((locals.var_t2__blk1112_dn0 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn0)), ((locals.var_t2__blk1112_dn2 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn2)), ((locals.var_t2__blk1112_dn6 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn6)), ((locals.var_t2__blk1112_dn7 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn7)), ((locals.var_t2__blk1112_dn10 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn10)), ((locals.var_t2__blk1112_dn11 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn11)), ((locals.var_t2__blk1112_dn12 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn12)), ((locals.var_t2__blk1112_dn17 * locals.var_t2__blk1112) + (locals.var_t2__blk1112 * locals.var_t2__blk1112_dn17)), );
            locals.var_t10w_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33820_e49012: f64 = (locals.var_t10w * locals.var_t10w);
            (locals.var_t10__blk1108, locals.var_t10__blk1108_dn0, locals.var_t10__blk1108_dn2, locals.var_t10__blk1108_dn6, locals.var_t10__blk1108_dn7, locals.var_t10__blk1108_dn10, locals.var_t10__blk1108_dn11, locals.var_t10__blk1108_dn12, locals.var_t10__blk1108_dn17, ) = (assign33820_e49012, ((locals.var_t10w_dn0 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn0)), ((locals.var_t10w_dn2 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn2)), ((locals.var_t10w_dn6 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn6)), ((locals.var_t10w_dn7 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn7)), ((locals.var_t10w_dn10 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn10)), ((locals.var_t10w_dn11 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn11)), ((locals.var_t10w_dn12 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn12)), ((locals.var_t10w_dn17 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn17)), );
            locals.var_t10__blk1108_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33830_e49019: f64 = (locals.var_t10__blk1108 * locals.var_t2__blk1112);
            let assign33830_e49020: f64 = (locals.var_t5__blk1115 / assign33830_e49019);
            (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12, locals.var_kusai_ig_dn17, ) = (assign33830_e49020, (((locals.var_t5__blk1115_dn0 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn0 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn0)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn2 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn2 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn2)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn6 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn6 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn6)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn7 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn7 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn7)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn10 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn10 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn10)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn11 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn11 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn11)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn12 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn12 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn12)))) / (assign33830_e49019 * assign33830_e49019)), (((locals.var_t5__blk1115_dn17 * assign33830_e49019) - (locals.var_t5__blk1115 * ((locals.var_t10__blk1108_dn17 * locals.var_t2__blk1112) + (locals.var_t10__blk1108 * locals.var_t2__blk1112_dn17)))) / (assign33830_e49019 * assign33830_e49019)), );
            locals.var_kusai_ig_rv = 0.0;
        }

        if (locals.var_guard1123 != 0.0) {
            let assign33840_e49026: f64 = (locals.var_weff_nf / locals.var_lch);
            let assign33840_e49028: f64 = (assign33840_e49026 * locals.var_mu);
            let assign33840_e49030: f64 = (assign33840_e49028 * locals.var_c_fox);
            (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12, locals.var_gds0_ign_dn17, ) = (assign33840_e49030, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn7)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn12) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn12)), (((((-((locals.var_weff_nf * locals.var_lch_dn17) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33840_e49026 * locals.var_mu_dn17)) * locals.var_c_fox) + (assign33840_e49028 * locals.var_c_fox_dn17)), );
            locals.var_gds0_ign_rv = 0.0;
        }

        let assign33890_e49080: f64 = (locals.var_ids + locals.var_idsibpc);
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign33890_e49080, (locals.var_ids_dn0 + locals.var_idsibpc_dn0), (locals.var_ids_dn2 + locals.var_idsibpc_dn2), (locals.var_ids_dn6 + locals.var_idsibpc_dn6), (locals.var_ids_dn7 + locals.var_idsibpc_dn7), (locals.var_ids_dn10 + locals.var_idsibpc_dn10), (locals.var_ids_dn11 + locals.var_idsibpc_dn11), (locals.var_ids_dn12 + locals.var_idsibpc_dn12), (locals.var_ids_dn17 + locals.var_idsibpc_dn17), );
        locals.var_ids_rv = 0.0;

        let assign33900_e49083: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign33900_e49083;
        locals.var_guard1124_rv = 0.0;

        if (locals.var_guard1124 != 0.0) {
            let assign33910_e49087: f64 = (locals.var_cbtp + locals.var_cbtn);
            locals.var_cgbe = assign33910_e49087;
            locals.var_cgbe_rv = 0.0;
        }

        if ((locals.var_guard1124 != 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign33920_e49096: f64 = (p.p168 * locals.var_lgleff);
            let assign33920_e49097: f64 = (locals.var_cgbe - assign33920_e49096);
            locals.var_cgbe = assign33920_e49097;
            locals.var_cgbe_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign33930_e49102: f64 = (-locals.var_cgbe);
            let assign33930_e49105: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign33930_e49106: f64 = (assign33930_e49102 * assign33930_e49105);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign33930_e49106, (assign33930_e49102 * (-locals.var_vbsp_dn0)), (assign33930_e49102 * (-locals.var_vbsp_dn2)), (assign33930_e49102 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33930_e49102 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33930_e49102 * (-locals.var_vbsp_dn10)), (assign33930_e49102 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33930_e49102 * (-locals.var_vbsp_dn12)), (assign33930_e49102 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            locals.var_cfu = 0.0;
            locals.var_cfu_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign33950_e49122: f64 = (locals.var_cfu * p.p9);
            let assign33950_e49125: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
            let assign33950_e49126: f64 = (assign33950_e49122 * assign33950_e49125);
            locals.var_cfd = assign33950_e49126;
            locals.var_cfd_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign33960_e49132: f64 = (locals.var_cfu * p.p9);
            let assign33960_e49135: f64 = (locals.var_wgate + locals.var_uc_psbcp);
            let assign33960_e49136: f64 = (assign33960_e49132 * assign33960_e49135);
            locals.var_cfs = assign33960_e49136;
            locals.var_cfs_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign33970_e49143: f64 = (locals.var_vgs - locals.var_vds);
            let assign33970_e49144: f64 = (locals.var_cfd * assign33970_e49143);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign33970_e49144, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign33980_e49150: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign33980_e49150, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign33990_e49156: f64 = (locals.var_cfu * p.p19);
            let assign33990_e49158: f64 = (assign33990_e49156 * p.p9);
            let assign33990_e49161: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign33990_e49162: f64 = (assign33990_e49158 * assign33990_e49161);
            (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17, ) = (assign33990_e49162, (assign33990_e49158 * (-locals.var_vbsp_dn0)), (assign33990_e49158 * (-locals.var_vbsp_dn2)), (assign33990_e49158 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33990_e49158 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33990_e49158 * (-locals.var_vbsp_dn10)), (assign33990_e49158 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33990_e49158 * (-locals.var_vbsp_dn12)), (assign33990_e49158 * (-locals.var_vbsp_dn17)), );
            locals.var_qfbc_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign34000_e49168: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34000_e49168, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign34010_e49174: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34010_e49174, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }

        if (locals.var_guard1124 != 0.0) {
            let assign34020_e49180: f64 = (locals.var_qgob + locals.var_qfbc);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34020_e49180, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17), );
            locals.var_qgob_rv = 0.0;
        }

        if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34030_e49188: f64 = (-p.p168);
            let assign34030_e49190: f64 = (assign34030_e49188 * locals.var_lgleff);
            locals.var_cgbe = assign34030_e49190;
            locals.var_cgbe_rv = 0.0;
        }

        if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34040_e49198: f64 = (-locals.var_cgbe);
            let assign34040_e49201: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign34040_e49202: f64 = (assign34040_e49198 * assign34040_e49201);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34040_e49202, (assign34040_e49198 * (-locals.var_vbsp_dn0)), (assign34040_e49198 * (-locals.var_vbsp_dn2)), (assign34040_e49198 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34040_e49198 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34040_e49198 * (-locals.var_vbsp_dn10)), (assign34040_e49198 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34040_e49198 * (-locals.var_vbsp_dn12)), (assign34040_e49198 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }

        if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given == 0.0)) {
            locals.var_cgbe = 0.0;
            locals.var_cgbe_rv = 0.0;
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qgob_rv = 0.0;
        }

        if (locals.var_guard1124 == 0.0) {
            locals.var_cf = 0.0;
            locals.var_cf_rv = 0.0;
            locals.var_cfd = locals.var_cf;
            locals.var_cfd_rv = 0.0;
            locals.var_cfs = locals.var_cf;
            locals.var_cfs_rv = 0.0;
        }

        if (locals.var_guard1124 == 0.0) {
            let assign34100_e49251: f64 = (locals.var_vgs - locals.var_vds);
            let assign34100_e49252: f64 = (locals.var_cfd * assign34100_e49251);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign34100_e49252, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }

        if (locals.var_guard1124 == 0.0) {
            let assign34110_e49259: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign34110_e49259, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }

        if (locals.var_guard1124 == 0.0) {
            let assign34120_e49266: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34120_e49266, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }

        if (locals.var_guard1124 == 0.0) {
            let assign34130_e49273: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34130_e49273, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }

        let assign34140_e49278: f64 = (locals.var_mfactor * locals.var_ids);
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (assign34140_e49278, (locals.var_mfactor * locals.var_ids_dn0), (locals.var_mfactor * locals.var_ids_dn2), (locals.var_mfactor * locals.var_ids_dn6), (locals.var_mfactor * locals.var_ids_dn7), (locals.var_mfactor * locals.var_ids_dn10), (locals.var_mfactor * locals.var_ids_dn11), (locals.var_mfactor * locals.var_ids_dn12), (locals.var_mfactor * locals.var_ids_dn17), );
        locals.var_idse_rv = 0.0;

        if (locals.var_flg_nqs != 0.0) {
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qde_rv = 0.0;
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qge_rv = 0.0;
        }

        let assign34170_e49289: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign34170_e49289;
        locals.var_guard1125_rv = 0.0;

        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 != 0.0)) {
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qse_rv = 0.0;
            (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17, ) = (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, );
            locals.var_xd_rv = 0.0;
        }

        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 == 0.0)) {
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbe_rv = 0.0;
        }

        let assign34260_e49360: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign34260_e49360;
        locals.var_guard1126_rv = 0.0;

        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
            let assign34270_e49367: f64 = (-locals.var_qb);
            let assign34270_e49369: f64 = (assign34270_e49367 - locals.var_qi);
            let assign34270_e49370: f64 = (locals.var_mfactor * assign34270_e49369);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34270_e49370, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)), );
            locals.var_qge_rv = 0.0;
        }

        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
            let assign34280_e49379: f64 = (locals.var_mfactor * locals.var_qd);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34280_e49379, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18), );
            locals.var_qde_rv = 0.0;
        }

        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
            let assign34290_e49389: f64 = (locals.var_qi - locals.var_qd);
            let assign34290_e49390: f64 = (locals.var_mfactor * assign34290_e49389);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34290_e49390, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)), );
            locals.var_qse_rv = 0.0;
        }

        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
            let assign34300_e49400: f64 = (-locals.var_qsub);
            let assign34300_e49402: f64 = (assign34300_e49400 - locals.var_qi);
            let assign34300_e49404: f64 = (assign34300_e49402 - locals.var_qs_fb);
            let assign34300_e49406: f64 = (assign34300_e49404 - locals.var_qd_fb);
            let assign34300_e49407: f64 = (locals.var_mfactor * assign34300_e49406);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34300_e49407, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)), );
            locals.var_qge_rv = 0.0;
        }

        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
            let assign34310_e49418: f64 = (locals.var_qd + locals.var_qd_fb);
            let assign34310_e49419: f64 = (locals.var_mfactor * assign34310_e49418);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34310_e49419, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)), );
            locals.var_qde_rv = 0.0;
        }

        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
            let assign34320_e49430: f64 = (locals.var_qi - locals.var_qd);
            let assign34320_e49432: f64 = (assign34320_e49430 + locals.var_qs_fb);
            let assign34320_e49433: f64 = (locals.var_mfactor * assign34320_e49432);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34320_e49433, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)), );
            locals.var_qse_rv = 0.0;
        }

        let assign34330_e49438: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1132 = assign34330_e49438;
        locals.var_guard1132_rv = 0.0;

        if (locals.var_guard1132 != 0.0) {
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qy_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_53(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        if (locals.var_guard1132 == 0.0) {
            let assign34350_e49447: f64 = (locals.var_ec * locals.var_leff);
            let assign34350_e49449: f64 = (assign34350_e49447 + locals.var_ps0);
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (assign34350_e49449, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17), );
            locals.var_pslk_rv = 0.0;
        }

        let assign34360_e49454: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1133 = assign34360_e49454;
        locals.var_guard1133_rv = 0.0;

        if ((locals.var_guard1132 == 0.0) && (locals.var_guard1133 != 0.0)) {
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17, );
            locals.var_pslk_rv = 0.0;
        }

        if (locals.var_guard1132 == 0.0) {
            let assign34380_e49467: f64 = (locals.var_vds + locals.var_ps0);
            let assign34380_e49468: f64 = (locals.var_aclm * assign34380_e49467);
            let assign34380_e49471: f64 = (1.0 - locals.var_aclm);
            let assign34380_e49473: f64 = (assign34380_e49471 * locals.var_pslk);
            let assign34380_e49474: f64 = (assign34380_e49468 + assign34380_e49473);
            (locals.var_t1__blk1128, locals.var_t1__blk1128_dn0, locals.var_t1__blk1128_dn2, locals.var_t1__blk1128_dn6, locals.var_t1__blk1128_dn7, locals.var_t1__blk1128_dn10, locals.var_t1__blk1128_dn11, locals.var_t1__blk1128_dn12, locals.var_t1__blk1128_dn17, ) = (assign34380_e49474, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34380_e49471 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34380_e49471 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34380_e49471 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34380_e49471 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34380_e49471 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34380_e49471 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34380_e49471 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34380_e49471 * locals.var_pslk_dn17)), );
            locals.var_t1__blk1128_rv = 0.0;
        }

        if (locals.var_guard1132 == 0.0) {
            let assign34390_e49481: f64 = (2.0 * 1.034943e-10);
            let assign34390_e49483: f64 = (assign34390_e49481 / locals.var_q_nsub);
            let assign34390_e49484: f64 = (assign34390_e49483).sqrt();
            (locals.var_t10__blk1129, locals.var_t10__blk1129_dn0, locals.var_t10__blk1129_dn2, locals.var_t10__blk1129_dn6, locals.var_t10__blk1129_dn7, locals.var_t10__blk1129_dn10, locals.var_t10__blk1129_dn11, locals.var_t10__blk1129_dn12, locals.var_t10__blk1129_dn17, ) = (assign34390_e49484, ((-((assign34390_e49481 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), );
            locals.var_t10__blk1129_rv = 0.0;
        }

        if (locals.var_guard1132 == 0.0) {
            let assign34400_e49491: f64 = (locals.var_t10__blk1129 * 1.3);
            (locals.var_t3__blk1130, locals.var_t3__blk1130_dn0, locals.var_t3__blk1130_dn2, locals.var_t3__blk1130_dn6, locals.var_t3__blk1130_dn7, locals.var_t3__blk1130_dn10, locals.var_t3__blk1130_dn11, locals.var_t3__blk1130_dn12, locals.var_t3__blk1130_dn17, ) = (assign34400_e49491, (locals.var_t10__blk1129_dn0 * 1.3), (locals.var_t10__blk1129_dn2 * 1.3), (locals.var_t10__blk1129_dn6 * 1.3), (locals.var_t10__blk1129_dn7 * 1.3), (locals.var_t10__blk1129_dn10 * 1.3), (locals.var_t10__blk1129_dn11 * 1.3), (locals.var_t10__blk1129_dn12 * 1.3), (locals.var_t10__blk1129_dn17 * 1.3), );
            locals.var_t3__blk1130_rv = 0.0;
        }

        if (locals.var_guard1132 == 0.0) {
            let assign34410_e49498: f64 = (1.034943e-10 * locals.var_weffcv_nf);
            let assign34410_e49500: f64 = (assign34410_e49498 * locals.var_t3__blk1130);
            (locals.var_t2__blk1131, locals.var_t2__blk1131_dn0, locals.var_t2__blk1131_dn2, locals.var_t2__blk1131_dn6, locals.var_t2__blk1131_dn7, locals.var_t2__blk1131_dn10, locals.var_t2__blk1131_dn11, locals.var_t2__blk1131_dn12, locals.var_t2__blk1131_dn17, ) = (assign34410_e49500, (assign34410_e49498 * locals.var_t3__blk1130_dn0), (assign34410_e49498 * locals.var_t3__blk1130_dn2), (assign34410_e49498 * locals.var_t3__blk1130_dn6), (assign34410_e49498 * locals.var_t3__blk1130_dn7), (assign34410_e49498 * locals.var_t3__blk1130_dn10), (assign34410_e49498 * locals.var_t3__blk1130_dn11), (assign34410_e49498 * locals.var_t3__blk1130_dn12), (assign34410_e49498 * locals.var_t3__blk1130_dn17), );
            locals.var_t2__blk1131_rv = 0.0;
        }

        if (locals.var_guard1132 == 0.0) {
            let assign34420_e49507: f64 = (locals.var_ps0 + locals.var_vds);
            let assign34420_e49509: f64 = (assign34420_e49507 - locals.var_t1__blk1128);
            let assign34420_e49511: f64 = (assign34420_e49509 / p.p64);
            let assign34420_e49513: f64 = (assign34420_e49511 - locals.var_ec);
            let assign34420_e49515: f64 = (assign34420_e49513 * locals.var_t2__blk1131);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34420_e49515, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1128_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1128_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1128_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1128_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1128_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1128_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1128_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1128_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn17)), );
            locals.var_qy_rv = 0.0;
        }

        let assign34430_e49520: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign34430_e49520;
        locals.var_guard1134_rv = 0.0;

        if (locals.var_guard1134 != 0.0) {
            let assign34440_e49525: f64 = (locals.var_cqyb0 * locals.var_vbsp);
            let assign34440_e49526: f64 = (locals.var_qy + assign34440_e49525);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34440_e49526, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)), );
            locals.var_qy_rv = 0.0;
        }

        let assign34450_e49531: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign34450_e49531;
        locals.var_guard1135_rv = 0.0;

        let assign34460_e49534: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34460_e49534;
        locals.var_guard1136_rv = 0.0;

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34470_e49539: f64 = (-locals.var_qbody_bt_p_sus);
            let assign34470_e49541: f64 = (assign34470_e49539 - locals.var_qbody_bt_p_sud);
            let assign34470_e49543: f64 = (assign34470_e49541 - locals.var_qbody_bt_n_sus);
            let assign34470_e49545: f64 = (assign34470_e49543 - locals.var_qbody_bt_n_sud);
            (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17, ) = (assign34470_e49545, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17), );
            locals.var_q_bt_ge_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34480_e49553: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
            (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17, ) = (assign34480_e49553, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17), );
            locals.var_q_bt_de_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34490_e49561: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
            (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17, ) = (assign34490_e49561, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17), );
            locals.var_q_bt_se_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34500_e49571: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34500_e49573: f64 = (assign34500_e49571 + locals.var_qgob);
            let assign34500_e49575: f64 = (assign34500_e49573 - locals.var_qy);
            let assign34500_e49577: f64 = (assign34500_e49575 - locals.var_qovs);
            let assign34500_e49579: f64 = (assign34500_e49577 - locals.var_qovd);
            let assign34500_e49581: f64 = (assign34500_e49579 + locals.var_q_bt_ge);
            let assign34500_e49582: f64 = (locals.var_mfactor * assign34500_e49581);
            let assign34500_e49583: f64 = (locals.var_qge + assign34500_e49582);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34500_e49583, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34510_e49592: f64 = (-locals.var_qgod);
            let assign34510_e49594: f64 = (assign34510_e49592 + locals.var_qy);
            let assign34510_e49596: f64 = (assign34510_e49594 + locals.var_qbdld);
            let assign34510_e49598: f64 = (assign34510_e49596 + locals.var_q_bt_de);
            let assign34510_e49599: f64 = (locals.var_mfactor * assign34510_e49598);
            let assign34510_e49600: f64 = (locals.var_qde + assign34510_e49599);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34510_e49600, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34520_e49609: f64 = (-locals.var_qgos);
            let assign34520_e49611: f64 = (assign34520_e49609 + locals.var_qbsld);
            let assign34520_e49613: f64 = (assign34520_e49611 + locals.var_q_bt_se);
            let assign34520_e49614: f64 = (locals.var_mfactor * assign34520_e49613);
            let assign34520_e49615: f64 = (locals.var_qse + assign34520_e49614);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34520_e49615, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
            let assign34530_e49626: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34530_e49628: f64 = (assign34530_e49626 + locals.var_qgob);
            let assign34530_e49630: f64 = (assign34530_e49628 - locals.var_qy);
            let assign34530_e49632: f64 = (assign34530_e49630 - locals.var_qovs);
            let assign34530_e49634: f64 = (assign34530_e49632 - locals.var_qovd);
            let assign34530_e49635: f64 = (locals.var_mfactor * assign34530_e49634);
            let assign34530_e49636: f64 = (locals.var_qge + assign34530_e49635);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34530_e49636, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
            let assign34540_e49646: f64 = (-locals.var_qgod);
            let assign34540_e49648: f64 = (assign34540_e49646 + locals.var_qy);
            let assign34540_e49650: f64 = (assign34540_e49648 + locals.var_qbdld);
            let assign34540_e49651: f64 = (locals.var_mfactor * assign34540_e49650);
            let assign34540_e49652: f64 = (locals.var_qde + assign34540_e49651);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34540_e49652, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }

        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
            let assign34550_e49662: f64 = (-locals.var_qgos);
            let assign34550_e49664: f64 = (assign34550_e49662 + locals.var_qbsld);
            let assign34550_e49665: f64 = (locals.var_mfactor * assign34550_e49664);
            let assign34550_e49666: f64 = (locals.var_qse + assign34550_e49665);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34550_e49666, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }

        let assign34580_e49673: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34580_e49673;
        locals.var_guard1137_rv = 0.0;

        if (locals.var_guard1137 != 0.0) {
            let assign34590_e49677: f64 = (locals.var_mfactor * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign34590_e49677, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }

        if (locals.var_guard1137 != 0.0) {
            let assign34600_e49683: f64 = (locals.var_mfactor * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign34600_e49683, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }

        if (locals.var_guard1137 != 0.0) {
            let assign34610_e49689: f64 = (locals.var_mfactor * locals.var_qbd);
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (assign34610_e49689, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17), );
            locals.var_qbd_s0_rv = 0.0;
        }

        if (locals.var_guard1137 != 0.0) {
            let assign34620_e49695: f64 = (locals.var_mfactor * locals.var_qbs);
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (assign34620_e49695, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17), );
            locals.var_qbs_s0_rv = 0.0;
        }

        if (locals.var_guard1137 == 0.0) {
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibsb_rv = 0.0;
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibdb_rv = 0.0;
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_s0_rv = 0.0;
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_s0_rv = 0.0;
        }

        let assign34670_e49720: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34670_e49720;
        locals.var_guard1138_rv = 0.0;

        if (locals.var_guard1138 != 0.0) {
            (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isube_rv = 0.0;
        }

        if (locals.var_guard1138 == 0.0) {
            let assign34690_e49729: f64 = (locals.var_mfactor * locals.var_isub);
            (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, ) = (assign34690_e49729, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17), );
            locals.var_isube_rv = 0.0;
        }

        let assign34800_e49813: f64 = (locals.var_mfactor * locals.var_nthrml);
        (locals.var_noithrml, locals.var_noithrml_dn0, locals.var_noithrml_dn2, locals.var_noithrml_dn6, locals.var_noithrml_dn7, locals.var_noithrml_dn10, locals.var_noithrml_dn11, locals.var_noithrml_dn12, locals.var_noithrml_dn17, ) = (assign34800_e49813, (locals.var_mfactor * locals.var_nthrml_dn0), (locals.var_mfactor * locals.var_nthrml_dn2), (locals.var_mfactor * locals.var_nthrml_dn6), (locals.var_mfactor * locals.var_nthrml_dn7), (locals.var_mfactor * locals.var_nthrml_dn10), (locals.var_mfactor * locals.var_nthrml_dn11), (locals.var_mfactor * locals.var_nthrml_dn12), (locals.var_mfactor * locals.var_nthrml_dn17), );
        locals.var_noithrml_rv = 0.0;

        let assign34810_e49816: f64 = locals.var_qge_dn6;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign34810_e49816, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;

        let assign34820_e49819: f64 = (p.p50 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign34820_e49819, (p.p50 * locals.var_cgdbd_dn0), (p.p50 * locals.var_cgdbd_dn2), (p.p50 * locals.var_cgdbd_dn6), (p.p50 * locals.var_cgdbd_dn7), (p.p50 * locals.var_cgdbd_dn10), (p.p50 * locals.var_cgdbd_dn11), (p.p50 * locals.var_cgdbd_dn12), (p.p50 * locals.var_cgdbd_dn13), (p.p50 * locals.var_cgdbd_dn15), (p.p50 * locals.var_cgdbd_dn16), (p.p50 * locals.var_cgdbd_dn17), (p.p50 * locals.var_cgdbd_dn18), );
        locals.var_cgdbd_rv = 0.0;

        let assign34830_e49822: f64 = locals.var_qge_dn7;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign34830_e49822, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;

        let assign34840_e49825: f64 = (p.p50 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign34840_e49825, (p.p50 * locals.var_cgsbd_dn0), (p.p50 * locals.var_cgsbd_dn2), (p.p50 * locals.var_cgsbd_dn6), (p.p50 * locals.var_cgsbd_dn7), (p.p50 * locals.var_cgsbd_dn10), (p.p50 * locals.var_cgsbd_dn11), (p.p50 * locals.var_cgsbd_dn12), (p.p50 * locals.var_cgsbd_dn13), (p.p50 * locals.var_cgsbd_dn15), (p.p50 * locals.var_cgsbd_dn16), (p.p50 * locals.var_cgsbd_dn17), (p.p50 * locals.var_cgsbd_dn18), );
        locals.var_cgsbd_rv = 0.0;

        let (assign34850_e49831, assign34850_e49831_d_n0, assign34850_e49831_d_n2, assign34850_e49831_d_n6, assign34850_e49831_d_n7, assign34850_e49831_d_n10, assign34850_e49831_d_n11, assign34850_e49831_d_n12, assign34850_e49831_d_n13, assign34850_e49831_d_n15, assign34850_e49831_d_n16, assign34850_e49831_d_n17, assign34850_e49831_d_n18,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18,)
    }
};
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn12, locals.var_cgsb_dn13, locals.var_cgsb_dn15, locals.var_cgsb_dn16, locals.var_cgsb_dn17, locals.var_cgsb_dn18, ) = (assign34850_e49831, assign34850_e49831_d_n0, assign34850_e49831_d_n2, assign34850_e49831_d_n6, assign34850_e49831_d_n7, assign34850_e49831_d_n10, assign34850_e49831_d_n11, assign34850_e49831_d_n12, assign34850_e49831_d_n13, assign34850_e49831_d_n15, assign34850_e49831_d_n16, assign34850_e49831_d_n17, assign34850_e49831_d_n18, );
        locals.var_cgsb_rv = 0.0;

        let assign34860_e49845: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1147 = assign34860_e49845;
        locals.var_guard1147_rv = 0.0;

        if (locals.var_guard1147 != 0.0) {
            let assign34870_e49849: f64 = (1e-6 * locals.var_c_fox);
            let assign34870_e49851: f64 = (assign34870_e49849 * locals.var_weffcv_nf);
            let assign34870_e49853: f64 = (assign34870_e49851 * locals.var_leff_cv);
            (locals.var_t0__blk1141, locals.var_t0__blk1141_dn0, locals.var_t0__blk1141_dn2, locals.var_t0__blk1141_dn6, locals.var_t0__blk1141_dn7, locals.var_t0__blk1141_dn10, locals.var_t0__blk1141_dn11, locals.var_t0__blk1141_dn12, locals.var_t0__blk1141_dn17, ) = (assign34870_e49853, (((1e-6 * locals.var_c_fox_dn0) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn2) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn6) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn7) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn10) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn11) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn12) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn17) * locals.var_weffcv_nf) * locals.var_leff_cv), );
            locals.var_t0__blk1141_rv = 0.0;
        }

        if (locals.var_guard1147 != 0.0) {
            let assign34880_e49859: f64 = (locals.var_cgsb / locals.var_mfactor);
            (locals.var_t1__blk1142, locals.var_t1__blk1142_dn0, locals.var_t1__blk1142_dn2, locals.var_t1__blk1142_dn6, locals.var_t1__blk1142_dn7, locals.var_t1__blk1142_dn10, locals.var_t1__blk1142_dn11, locals.var_t1__blk1142_dn12, locals.var_t1__blk1142_dn13, locals.var_t1__blk1142_dn15, locals.var_t1__blk1142_dn16, locals.var_t1__blk1142_dn17, locals.var_t1__blk1142_dn18, ) = (assign34880_e49859, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor), (locals.var_cgsb_dn15 / locals.var_mfactor), (locals.var_cgsb_dn16 / locals.var_mfactor), (locals.var_cgsb_dn17 / locals.var_mfactor), (locals.var_cgsb_dn18 / locals.var_mfactor), );
            locals.var_t1__blk1142_rv = 0.0;
        }

        if (locals.var_guard1147 != 0.0) {
            let assign34890_e49865: f64 = (0.1185185185185185 * 1.6021918e-19);
            let assign34890_e49867: f64 = (assign34890_e49865 * locals.var_beta_inv);
            let assign34890_e49869: f64 = (assign34890_e49867 * locals.var_t1__blk1142);
            let assign34890_e49871: f64 = (assign34890_e49869 * locals.var_t1__blk1142);
            let assign34890_e49873: f64 = (assign34890_e49871 / locals.var_gds0_ign);
            (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12, locals.var_nign0_dn13, locals.var_nign0_dn15, locals.var_nign0_dn16, locals.var_nign0_dn17, locals.var_nign0_dn18, ) = (assign34890_e49873, ((((((assign34890_e49867 * locals.var_t1__blk1142_dn0) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn0)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn2) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn2)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn6) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn6)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn7) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn7)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign34890_e49865 * locals.var_beta_inv_dn10) * locals.var_t1__blk1142) + (assign34890_e49867 * locals.var_t1__blk1142_dn10)) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn10)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn11) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn11)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn12) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn12)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34890_e49867 * locals.var_t1__blk1142_dn13) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn13)) / locals.var_gds0_ign), ((((assign34890_e49867 * locals.var_t1__blk1142_dn15) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn15)) / locals.var_gds0_ign), ((((assign34890_e49867 * locals.var_t1__blk1142_dn16) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn16)) / locals.var_gds0_ign), ((((((assign34890_e49867 * locals.var_t1__blk1142_dn17) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn17)) * locals.var_gds0_ign) - (assign34890_e49871 * locals.var_gds0_ign_dn17)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34890_e49867 * locals.var_t1__blk1142_dn18) * locals.var_t1__blk1142) + (assign34890_e49869 * locals.var_t1__blk1142_dn18)) / locals.var_gds0_ign), );
            locals.var_nign0_rv = 0.0;
        }

        let assign34900_e49879: f64 = (10.0 * 2.220446049250313e-16);
        let assign34900_e49884: f64 = (10.0 * 2.220446049250313e-16);
        let assign34900_e49886: f64 = if ((locals.var_kusai00l > assign34900_e49879) && (locals.var_vds > assign34900_e49884)) { 1.0 } else { 0.0 };
        locals.var_guard1148 = assign34900_e49886;
        locals.var_guard1148_rv = 0.0;

        if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
            let assign34910_e49892: f64 = (locals.var_muun / locals.var_mu);
            (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12, locals.var_mumoda_dn17, ) = (assign34910_e49892, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn17 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn17)) / (locals.var_mu * locals.var_mu)), );
            locals.var_mumoda_rv = 0.0;
        }

        if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
            let assign34920_e49900: f64 = (locals.var_muun / locals.var_mud_hoso);
            let assign34920_e49902: f64 = (assign34920_e49900 - locals.var_mumoda);
            let assign34920_e49904: f64 = (assign34920_e49902 / locals.var_vds);
            (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12, locals.var_mumodb_dn17, ) = (assign34920_e49904, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn17) * locals.var_vds) - (assign34920_e49902 * locals.var_vds_dn17)) / (locals.var_vds * locals.var_vds)), );
            locals.var_mumodb_rv = 0.0;
        }

        if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 != 0.0)) {
            let assign34930_e49913: f64 = (0.6666666666666667 * locals.var_mumodb);
            let assign34930_e49917: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
            let assign34930_e49918: f64 = (locals.var_kusai00 + assign34930_e49917);
            let assign34930_e49920: f64 = (assign34930_e49918 + locals.var_kusail);
            let assign34930_e49921: f64 = (assign34930_e49913 * assign34930_e49920);
            let assign34930_e49924: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
            let assign34930_e49925: f64 = (assign34930_e49921 / assign34930_e49924);
            let assign34930_e49926: f64 = (locals.var_mumoda + assign34930_e49925);
            (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17, ) = (assign34930_e49926, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign34930_e49924 * assign34930_e49924))), (locals.var_mumoda_dn17 + ((((((0.6666666666666667 * locals.var_mumodb_dn17) * assign34930_e49920) + (assign34930_e49913 * ((locals.var_kusai00_dn17 + ((locals.var_vgvt_dn17 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17))) * assign34930_e49924) - (assign34930_e49921 * (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17))) / (assign34930_e49924 * assign34930_e49924))), );
            locals.var_correct_w1_rv = 0.0;
        }

        if ((locals.var_guard1147 != 0.0) && (locals.var_guard1148 == 0.0)) {
            let assign34940_e49935: f64 = (locals.var_muun / locals.var_mud_hoso);
            (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17, ) = (assign34940_e49935, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)), );
            locals.var_correct_w1_rv = 0.0;
        }

        if (locals.var_guard1147 != 0.0) {
            let assign34950_e49941: f64 = (locals.var_mfactor * locals.var_nign0);
            let assign34950_e49943: f64 = (assign34950_e49941 * locals.var_kusai_ig);
            let assign34950_e49945: f64 = (assign34950_e49943 * locals.var_correct_w1);
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18, ) = (assign34950_e49945, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn12)), (((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn15) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn16) * locals.var_kusai_ig) * locals.var_correct_w1), (((((locals.var_mfactor * locals.var_nign0_dn17) * locals.var_kusai_ig) + (assign34950_e49941 * locals.var_kusai_ig_dn17)) * locals.var_correct_w1) + (assign34950_e49943 * locals.var_correct_w1_dn17)), (((locals.var_mfactor * locals.var_nign0_dn18) * locals.var_kusai_ig) * locals.var_correct_w1), );
            locals.var_noiigate_rv = 0.0;
        }

        if (locals.var_guard1147 != 0.0) {
            let assign34970_e49954: f64 = (-locals.var_t1__blk1142);
            let (assign34970_e49963, assign34970_e49963_d_n0, assign34970_e49963_d_n2, assign34970_e49963_d_n6, assign34970_e49963_d_n7, assign34970_e49963_d_n10, assign34970_e49963_d_n11, assign34970_e49963_d_n12, assign34970_e49963_d_n13, assign34970_e49963_d_n15, assign34970_e49963_d_n16, assign34970_e49963_d_n17, assign34970_e49963_d_n18,) = {
    if ((assign34970_e49954 > locals.var_t0__blk1141) && (locals.var_noiigate > 0.0)) {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18, ) = (assign34970_e49963, assign34970_e49963_d_n0, assign34970_e49963_d_n2, assign34970_e49963_d_n6, assign34970_e49963_d_n7, assign34970_e49963_d_n10, assign34970_e49963_d_n11, assign34970_e49963_d_n12, assign34970_e49963_d_n13, assign34970_e49963_d_n15, assign34970_e49963_d_n16, assign34970_e49963_d_n17, assign34970_e49963_d_n18, );
            locals.var_noiigate_rv = 0.0;
        }

        if (locals.var_guard1147 == 0.0) {
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_noiigate_rv = 0.0;
        }

        let assign35050_e49992: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1149 = assign35050_e49992;
        locals.var_guard1149_rv = 0.0;

        if (locals.var_guard1149 != 0.0) {
            locals.var_rdmod = 1.0;
            locals.var_rdmod_rv = 0.0;
        }

        let assign35070_e49999: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1169 = assign35070_e49999;
        locals.var_guard1169_rv = 0.0;

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
            locals.var_mks_rdrmue = p.p266;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p268;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p258;
            locals.var_ldrifte_rv = 0.0;
        }

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
            let assign35140_e50050: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35140_e50050, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr_rv = 0.0;
        }

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
            locals.var_mks_rdrmue = p.p265;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p267;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p257;
            locals.var_ldrifte_rv = 0.0;
        }

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
            let assign35210_e50110: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35210_e50110, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35240_e50133: f64 = (locals.var_mks_rdrmue / 10000.0);
            locals.var_mks_rdrmue = assign35240_e50133;
            locals.var_mks_rdrmue_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35250_e50139: f64 = (locals.var_mks_rdrvmax / 100.0);
            locals.var_mks_rdrvmax = assign35250_e50139;
            locals.var_mks_rdrvmax_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35260_e50145: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio, locals.var_tratio_dn10, ) = (assign35260_e50145, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35270_e50151: f64 = (locals.var_tratio).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35270_e50151, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35270_e50151 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35280_e50157: f64 = (locals.var_mks_rdrmue / locals.var_t1);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35280_e50157, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35290_e50164: f64 = (0.4 * locals.var_tratio);
            let assign35290_e50165: f64 = (1.8 + assign35290_e50164);
            let assign35290_e50168: f64 = (0.1 * locals.var_tratio);
            let assign35290_e50170: f64 = (assign35290_e50168 * locals.var_tratio);
            let assign35290_e50171: f64 = (assign35290_e50165 + assign35290_e50170);
            let assign35290_e50175: f64 = (1.0 - locals.var_tratio);
            let assign35290_e50176: f64 = (p.p270 * assign35290_e50175);
            let assign35290_e50177: f64 = (assign35290_e50171 - assign35290_e50176);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35290_e50177, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35290_e50168 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35300_e50183: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
            (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17, ) = (assign35300_e50183, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1162_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35310_e50191: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35310_e50192: f64 = (p.p274 * assign35310_e50191);
            let assign35310_e50193: f64 = (locals.var_rrdrbb + assign35310_e50192);
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (assign35310_e50193, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35320_e50201: f64 = (locals.var_lgle).powf(p.p280);
            let assign35320_e50202: f64 = (p.p279 / assign35320_e50201);
            let assign35320_e50203: f64 = (1.0 + assign35320_e50202);
            locals.var_rdrmuele = assign35320_e50203;
            locals.var_rdrmuele_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35330_e50211: f64 = (locals.var_lgle).powf(p.p278);
            let assign35330_e50212: f64 = (p.p277 / assign35330_e50211);
            let assign35330_e50213: f64 = (1.0 + assign35330_e50212);
            locals.var_rdrvmaxle = assign35330_e50213;
            locals.var_rdrvmaxle_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35340_e50221: f64 = (locals.var_wg).powf(p.p276);
            let assign35340_e50222: f64 = (p.p275 / assign35340_e50221);
            let assign35340_e50223: f64 = (1.0 + assign35340_e50222);
            locals.var_rdrvmaxwe = assign35340_e50223;
            locals.var_rdrvmaxwe_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35350_e50229: f64 = (locals.var_mu0 * locals.var_rdrmuele);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35350_e50229, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele), );
            locals.var_mu0_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35360_e50235: f64 = (locals.var_vmaxe__blk1162 * locals.var_rdrvmaxwe);
            let assign35360_e50237: f64 = (assign35360_e50235 * locals.var_rdrvmaxle);
            let assign35360_e50239: f64 = (assign35360_e50237 + 1e-50);
            (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17, ) = (assign35360_e50239, ((locals.var_vmaxe__blk1162_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), );
            locals.var_vmaxe__blk1162_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_54(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        if (locals.var_guard1149 != 0.0) {
            let assign35370_e50245: f64 = (locals.var_vrdr / locals.var_ldrifte);
            (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7, ) = (assign35370_e50245, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte), );
            locals.var_edri_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35380_e50251: f64 = (locals.var_mu0 * locals.var_edri);
            (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17, ) = (assign35380_e50251, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri), );
            locals.var_vdri_rv = 0.0;
        }

        let assign35390_e50256: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1170 = assign35390_e50256;
        locals.var_guard1170_rv = 0.0;

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 != 0.0)) {
            let assign35400_e50262: f64 = (locals.var_vdri / locals.var_vmaxe__blk1162);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35400_e50262, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), );
            locals.var_t1_rv = 0.0;
        }

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 == 0.0)) {
            let assign35410_e50270: f64 = (-locals.var_vdri);
            let assign35410_e50272: f64 = (assign35410_e50270 / locals.var_vmaxe__blk1162);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35410_e50272, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), );
            locals.var_t1_rv = 0.0;
        }

        let assign35420_e50278: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50279: f64 = (1.0 - assign35420_e50278);
        let assign35420_e50286: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50287: f64 = (1.0 + assign35420_e50286);
        let assign35420_e50289: f64 = if ((assign35420_e50279 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35420_e50287)) { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign35420_e50289;
        locals.var_guard1171_rv = 0.0;

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1171 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }

        let assign35440_e50299: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50300: f64 = (2.0 - assign35440_e50299);
        let assign35440_e50307: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50308: f64 = (2.0 + assign35440_e50307);
        let assign35440_e50310: f64 = if ((assign35440_e50300 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35440_e50308)) { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign35440_e50310;
        locals.var_guard1172_rv = 0.0;

        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }

        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 == 0.0)) {
            let assign35460_e50330: f64 = (locals.var_rrdrbb - 1.0);
            let assign35460_e50331: f64 = (locals.var_t1).powf(assign35460_e50330);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign35460_e50331, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn0)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn2)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn6)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn7)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn10)) } } else { (assign35460_e50331 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35460_e50330 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn11)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn12)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn17)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35470_e50337: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign35470_e50337, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35480_e50343: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign35480_e50343, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }

        let assign35490_e50349: f64 = (10.0 * 2.220446049250313e-16);
        let assign35490_e50350: f64 = (1.0 - assign35490_e50349);
        let assign35490_e50357: f64 = (10.0 * 2.220446049250313e-16);
        let assign35490_e50358: f64 = (1.0 + assign35490_e50357);
        let assign35490_e50360: f64 = if ((assign35490_e50350 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35490_e50358)) { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign35490_e50360;
        locals.var_guard1173_rv = 0.0;

        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1173 != 0.0)) {
            let assign35500_e50366: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35500_e50366, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }

        let assign35510_e50372: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50373: f64 = (2.0 - assign35510_e50372);
        let assign35510_e50380: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50381: f64 = (2.0 + assign35510_e50380);
        let assign35510_e50383: f64 = if ((assign35510_e50373 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35510_e50381)) { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign35510_e50383;
        locals.var_guard1174_rv = 0.0;

        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 != 0.0)) {
            let assign35520_e50392: f64 = (locals.var_t4).sqrt();
            let assign35520_e50393: f64 = (1.0 / assign35520_e50392);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35520_e50393, (-((locals.var_t4_dn0 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn2 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn6 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn7 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn10 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn11 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn12 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn17 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), );
            locals.var_t5_rv = 0.0;
        }

        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
            let assign35530_e50405: f64 = (-1.0);
            let assign35530_e50407: f64 = (assign35530_e50405 / locals.var_rrdrbb);
            let assign35530_e50409: f64 = (assign35530_e50407 - 1.0);
            let assign35530_e50410: f64 = (locals.var_t4).powf(assign35530_e50409);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign35530_e50410, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn0)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn2)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn6)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn7)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign35530_e50405 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn10)) } } else { (assign35530_e50410 * (((-((assign35530_e50405 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign35530_e50409 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn11)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn12)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn17)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }

        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
            let assign35540_e50422: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35540_e50422, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }

        if (locals.var_guard1149 != 0.0) {
            let assign35560_e50434: f64 = (1.6021918e-19 / locals.var_ldrifte);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35560_e50434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }

        let assign35680_e50510: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign35680_e50510;
        locals.var_guard1177_rv = 0.0;

        if (locals.var_guard1177 != 0.0) {
            locals.var_rdmod = 2.0;
            locals.var_rdmod_rv = 0.0;
        }

        let assign35700_e50517: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign35700_e50517;
        locals.var_guard1197_rv = 0.0;

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
            locals.var_mks_rdrmue__blk1181 = p.p266;
            locals.var_mks_rdrmue__blk1181_rv = 0.0;
            locals.var_mks_rdrvmax__blk1182 = p.p268;
            locals.var_mks_rdrvmax__blk1182_rv = 0.0;
            (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb__blk1183_rv = 0.0;
            locals.var_ldrifte__blk1187 = p.p258;
            locals.var_ldrifte__blk1187_rv = 0.0;
        }

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
            let assign35770_e50568: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr__blk1185, locals.var_vrdr__blk1185_dn0, locals.var_vrdr__blk1185_dn2, locals.var_vrdr__blk1185_dn6, locals.var_vrdr__blk1185_dn7, ) = (assign35770_e50568, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr__blk1185_rv = 0.0;
        }

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
            locals.var_mks_rdrmue__blk1181 = p.p265;
            locals.var_mks_rdrmue__blk1181_rv = 0.0;
            locals.var_mks_rdrvmax__blk1182 = p.p267;
            locals.var_mks_rdrvmax__blk1182_rv = 0.0;
            (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb__blk1183_rv = 0.0;
            locals.var_ldrifte__blk1187 = p.p257;
            locals.var_ldrifte__blk1187_rv = 0.0;
        }

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
            let assign35840_e50628: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr__blk1185, locals.var_vrdr__blk1185_dn0, locals.var_vrdr__blk1185_dn2, locals.var_vrdr__blk1185_dn6, locals.var_vrdr__blk1185_dn7, ) = (assign35840_e50628, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr__blk1185_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35870_e50651: f64 = (locals.var_mks_rdrmue__blk1181 / 10000.0);
            locals.var_mks_rdrmue__blk1181 = assign35870_e50651;
            locals.var_mks_rdrmue__blk1181_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35880_e50657: f64 = (locals.var_mks_rdrvmax__blk1182 / 100.0);
            locals.var_mks_rdrvmax__blk1182 = assign35880_e50657;
            locals.var_mks_rdrvmax__blk1182_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35890_e50663: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio__blk1186, locals.var_tratio__blk1186_dn10, ) = (assign35890_e50663, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio__blk1186_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35900_e50669: f64 = (locals.var_tratio__blk1186).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35900_e50669, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio__blk1186).powf(p.p269 - 1.0) * locals.var_tratio__blk1186_dn10)) } } else { (assign35900_e50669 * (p.p269 * (locals.var_tratio__blk1186_dn10 / locals.var_tratio__blk1186))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35910_e50675: f64 = (locals.var_mks_rdrmue__blk1181 / locals.var_t1);
            (locals.var_mu0__blk1189, locals.var_mu0__blk1189_dn0, locals.var_mu0__blk1189_dn2, locals.var_mu0__blk1189_dn6, locals.var_mu0__blk1189_dn7, locals.var_mu0__blk1189_dn10, locals.var_mu0__blk1189_dn11, locals.var_mu0__blk1189_dn12, locals.var_mu0__blk1189_dn17, ) = (assign35910_e50675, (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0__blk1189_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35920_e50682: f64 = (0.4 * locals.var_tratio__blk1186);
            let assign35920_e50683: f64 = (1.8 + assign35920_e50682);
            let assign35920_e50686: f64 = (0.1 * locals.var_tratio__blk1186);
            let assign35920_e50688: f64 = (assign35920_e50686 * locals.var_tratio__blk1186);
            let assign35920_e50689: f64 = (assign35920_e50683 + assign35920_e50688);
            let assign35920_e50693: f64 = (1.0 - locals.var_tratio__blk1186);
            let assign35920_e50694: f64 = (p.p270 * assign35920_e50693);
            let assign35920_e50695: f64 = (assign35920_e50689 - assign35920_e50694);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35920_e50695, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio__blk1186_dn10) + (((0.1 * locals.var_tratio__blk1186_dn10) * locals.var_tratio__blk1186) + (assign35920_e50686 * locals.var_tratio__blk1186_dn10))) - (p.p270 * (-locals.var_tratio__blk1186_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35930_e50701: f64 = (locals.var_mks_rdrvmax__blk1182 / locals.var_t0);
            (locals.var_vmaxe__blk1190, locals.var_vmaxe__blk1190_dn0, locals.var_vmaxe__blk1190_dn2, locals.var_vmaxe__blk1190_dn6, locals.var_vmaxe__blk1190_dn7, locals.var_vmaxe__blk1190_dn10, locals.var_vmaxe__blk1190_dn11, locals.var_vmaxe__blk1190_dn12, locals.var_vmaxe__blk1190_dn17, ) = (assign35930_e50701, (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1190_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35940_e50709: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35940_e50710: f64 = (p.p274 * assign35940_e50709);
            let assign35940_e50711: f64 = (locals.var_rrdrbb__blk1183 + assign35940_e50710);
            (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10, ) = (assign35940_e50711, (locals.var_rrdrbb__blk1183_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb__blk1183_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35950_e50719: f64 = (locals.var_lgle).powf(p.p280);
            let assign35950_e50720: f64 = (p.p279 / assign35950_e50719);
            let assign35950_e50721: f64 = (1.0 + assign35950_e50720);
            locals.var_rdrmuele__blk1178 = assign35950_e50721;
            locals.var_rdrmuele__blk1178_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35960_e50729: f64 = (locals.var_lgle).powf(p.p278);
            let assign35960_e50730: f64 = (p.p277 / assign35960_e50729);
            let assign35960_e50731: f64 = (1.0 + assign35960_e50730);
            locals.var_rdrvmaxle__blk1180 = assign35960_e50731;
            locals.var_rdrvmaxle__blk1180_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35970_e50739: f64 = (locals.var_wg).powf(p.p276);
            let assign35970_e50740: f64 = (p.p275 / assign35970_e50739);
            let assign35970_e50741: f64 = (1.0 + assign35970_e50740);
            locals.var_rdrvmaxwe__blk1179 = assign35970_e50741;
            locals.var_rdrvmaxwe__blk1179_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35980_e50747: f64 = (locals.var_mu0__blk1189 * locals.var_rdrmuele__blk1178);
            (locals.var_mu0__blk1189, locals.var_mu0__blk1189_dn0, locals.var_mu0__blk1189_dn2, locals.var_mu0__blk1189_dn6, locals.var_mu0__blk1189_dn7, locals.var_mu0__blk1189_dn10, locals.var_mu0__blk1189_dn11, locals.var_mu0__blk1189_dn12, locals.var_mu0__blk1189_dn17, ) = (assign35980_e50747, (locals.var_mu0__blk1189_dn0 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn2 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn6 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn7 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn10 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn11 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn12 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn17 * locals.var_rdrmuele__blk1178), );
            locals.var_mu0__blk1189_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign35990_e50753: f64 = (locals.var_vmaxe__blk1190 * locals.var_rdrvmaxwe__blk1179);
            let assign35990_e50755: f64 = (assign35990_e50753 * locals.var_rdrvmaxle__blk1180);
            let assign35990_e50757: f64 = (assign35990_e50755 + 1e-50);
            (locals.var_vmaxe__blk1190, locals.var_vmaxe__blk1190_dn0, locals.var_vmaxe__blk1190_dn2, locals.var_vmaxe__blk1190_dn6, locals.var_vmaxe__blk1190_dn7, locals.var_vmaxe__blk1190_dn10, locals.var_vmaxe__blk1190_dn11, locals.var_vmaxe__blk1190_dn12, locals.var_vmaxe__blk1190_dn17, ) = (assign35990_e50757, ((locals.var_vmaxe__blk1190_dn0 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn2 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn6 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn7 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn10 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn11 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn12 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn17 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), );
            locals.var_vmaxe__blk1190_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign36000_e50763: f64 = (locals.var_vrdr__blk1185 / locals.var_ldrifte__blk1187);
            (locals.var_edri__blk1191, locals.var_edri__blk1191_dn0, locals.var_edri__blk1191_dn2, locals.var_edri__blk1191_dn6, locals.var_edri__blk1191_dn7, ) = (assign36000_e50763, (locals.var_vrdr__blk1185_dn0 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn2 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn6 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn7 / locals.var_ldrifte__blk1187), );
            locals.var_edri__blk1191_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign36010_e50769: f64 = (locals.var_mu0__blk1189 * locals.var_edri__blk1191);
            (locals.var_vdri__blk1192, locals.var_vdri__blk1192_dn0, locals.var_vdri__blk1192_dn2, locals.var_vdri__blk1192_dn6, locals.var_vdri__blk1192_dn7, locals.var_vdri__blk1192_dn10, locals.var_vdri__blk1192_dn11, locals.var_vdri__blk1192_dn12, locals.var_vdri__blk1192_dn17, ) = (assign36010_e50769, ((locals.var_mu0__blk1189_dn0 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn0)), ((locals.var_mu0__blk1189_dn2 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn2)), ((locals.var_mu0__blk1189_dn6 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn6)), ((locals.var_mu0__blk1189_dn7 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn7)), (locals.var_mu0__blk1189_dn10 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn11 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn12 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn17 * locals.var_edri__blk1191), );
            locals.var_vdri__blk1192_rv = 0.0;
        }

        let assign36020_e50774: f64 = if locals.var_vrdr__blk1185 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign36020_e50774;
        locals.var_guard1198_rv = 0.0;

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign36030_e50780: f64 = (locals.var_vdri__blk1192 / locals.var_vmaxe__blk1190);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36030_e50780, (((locals.var_vdri__blk1192_dn0 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn0)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn2 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn2)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn6 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn6)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn7 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn7)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn10 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn10)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn11 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn11)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn12 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn12)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn17 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn17)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), );
            locals.var_t1_rv = 0.0;
        }

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1198 == 0.0)) {
            let assign36040_e50788: f64 = (-locals.var_vdri__blk1192);
            let assign36040_e50790: f64 = (assign36040_e50788 / locals.var_vmaxe__blk1190);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36040_e50790, ((((-locals.var_vdri__blk1192_dn0) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn0)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn2) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn2)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn6) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn6)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn7) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn7)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn10) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn10)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn11) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn11)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn12) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn12)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn17) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn17)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), );
            locals.var_t1_rv = 0.0;
        }

        let assign36050_e50796: f64 = (10.0 * 2.220446049250313e-16);
        let assign36050_e50797: f64 = (1.0 - assign36050_e50796);
        let assign36050_e50804: f64 = (10.0 * 2.220446049250313e-16);
        let assign36050_e50805: f64 = (1.0 + assign36050_e50804);
        let assign36050_e50807: f64 = if ((assign36050_e50797 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36050_e50805)) { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign36050_e50807;
        locals.var_guard1199_rv = 0.0;

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1199 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }

        let assign36070_e50817: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50818: f64 = (2.0 - assign36070_e50817);
        let assign36070_e50825: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50826: f64 = (2.0 + assign36070_e50825);
        let assign36070_e50828: f64 = if ((assign36070_e50818 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36070_e50826)) { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign36070_e50828;
        locals.var_guard1200_rv = 0.0;

        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }

        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign36090_e50848: f64 = (locals.var_rrdrbb__blk1183 - 1.0);
            let assign36090_e50849: f64 = (locals.var_t1).powf(assign36090_e50848);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign36090_e50849, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn0)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn2)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn6)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn7)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb__blk1183_dn10 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn10)) } } else { (assign36090_e50849 * ((locals.var_rrdrbb__blk1183_dn10 * (locals.var_t1).ln()) + (assign36090_e50848 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn11)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn12)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn17)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign36100_e50855: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign36100_e50855, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign36110_e50861: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign36110_e50861, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }

        let assign36120_e50867: f64 = (10.0 * 2.220446049250313e-16);
        let assign36120_e50868: f64 = (1.0 - assign36120_e50867);
        let assign36120_e50875: f64 = (10.0 * 2.220446049250313e-16);
        let assign36120_e50876: f64 = (1.0 + assign36120_e50875);
        let assign36120_e50878: f64 = if ((assign36120_e50868 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36120_e50876)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign36120_e50878;
        locals.var_guard1201_rv = 0.0;

        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1201 != 0.0)) {
            let assign36130_e50884: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36130_e50884, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }

        let assign36140_e50890: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50891: f64 = (2.0 - assign36140_e50890);
        let assign36140_e50898: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50899: f64 = (2.0 + assign36140_e50898);
        let assign36140_e50901: f64 = if ((assign36140_e50891 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36140_e50899)) { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign36140_e50901;
        locals.var_guard1202_rv = 0.0;

        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign36150_e50910: f64 = (locals.var_t4).sqrt();
            let assign36150_e50911: f64 = (1.0 / assign36150_e50910);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36150_e50911, (-((locals.var_t4_dn0 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn2 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn6 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn7 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn10 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn11 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn12 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn17 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), );
            locals.var_t5_rv = 0.0;
        }

        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign36160_e50923: f64 = (-1.0);
            let assign36160_e50925: f64 = (assign36160_e50923 / locals.var_rrdrbb__blk1183);
            let assign36160_e50927: f64 = (assign36160_e50925 - 1.0);
            let assign36160_e50928: f64 = (locals.var_t4).powf(assign36160_e50927);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign36160_e50928, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn0)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn2)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn6)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn7)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign36160_e50923 * locals.var_rrdrbb__blk1183_dn10) / (locals.var_rrdrbb__blk1183 * locals.var_rrdrbb__blk1183))) == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn10)) } } else { (assign36160_e50928 * (((-((assign36160_e50923 * locals.var_rrdrbb__blk1183_dn10) / (locals.var_rrdrbb__blk1183 * locals.var_rrdrbb__blk1183))) * (locals.var_t4).ln()) + (assign36160_e50927 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn11)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn12)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn17)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }

        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign36170_e50940: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36170_e50940, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }

        if (locals.var_guard1177 != 0.0) {
            let assign36190_e50952: f64 = (1.6021918e-19 / locals.var_ldrifte__blk1187);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36190_e50952, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }

        let assign36310_e51028: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign36310_e51028;
        locals.var_guard1205_rv = 0.0;

        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let (assign36360_e51069, assign36360_e51069_d_n0, assign36360_e51069_d_n2, assign36360_e51069_d_n6, assign36360_e51069_d_n7, assign36360_e51069_d_n10, assign36360_e51069_d_n11, assign36360_e51069_d_n12, assign36360_e51069_d_n17,) = {
    if (locals.var_mode == 1.0) {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    } else {
        let assign36360_e51068: f64 = (1.0 - locals.var_xd);
        (assign36360_e51068, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn12), (-locals.var_xd_dn17),)
    }
};
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (assign36360_e51069, assign36360_e51069_d_n0, assign36360_e51069_d_n2, assign36360_e51069_d_n6, assign36360_e51069_d_n7, assign36360_e51069_d_n10, assign36360_e51069_d_n11, assign36360_e51069_d_n12, assign36360_e51069_d_n17, );
            locals.var_qdrat_rv = 0.0;
        }

        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36390_e51097: f64 = (locals.var_qi_nqs * locals.var_qdrat);
            let assign36390_e51099: f64 = (assign36390_e51097 + locals.var_q_bt_se);
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (assign36390_e51099, ((locals.var_qi_nqs * locals.var_qdrat_dn0) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * locals.var_qdrat_dn2) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * locals.var_qdrat_dn6) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * locals.var_qdrat_dn7) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * locals.var_qdrat_dn10) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * locals.var_qdrat_dn11) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * locals.var_qdrat_dn12) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * locals.var_qdrat_dn17) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * locals.var_qdrat), );
            locals.var_qd_nqs_rv = 0.0;
        }

        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36400_e51108: f64 = (1.0 - locals.var_qdrat);
            let assign36400_e51109: f64 = (locals.var_qi_nqs * assign36400_e51108);
            let assign36400_e51111: f64 = (assign36400_e51109 + locals.var_q_bt_se);
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (assign36400_e51111, ((locals.var_qi_nqs * (-locals.var_qdrat_dn0)) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * (-locals.var_qdrat_dn2)) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * (-locals.var_qdrat_dn6)) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * (-locals.var_qdrat_dn7)) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * (-locals.var_qdrat_dn10)) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * (-locals.var_qdrat_dn11)) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * (-locals.var_qdrat_dn12)) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * (-locals.var_qdrat_dn17)) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * assign36400_e51108), );
            locals.var_qs_nqs_rv = 0.0;
        }

        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36410_e51118: f64 = (-locals.var_qi_nqs);
            let assign36410_e51120: f64 = (assign36410_e51118 - locals.var_qb_nqs);
            let assign36410_e51122: f64 = (assign36410_e51120 + locals.var_q_bt_ge);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36410_e51122, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, (-locals.var_qb_nqs_dn13), 0.0, 0.0, locals.var_q_bt_ge_dn17, (-locals.var_qi_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }

        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }

        if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36560_e51244: f64 = (-locals.var_qd_nqs);
            let assign36560_e51246: f64 = (assign36560_e51244 - locals.var_qs_nqs);
            let assign36560_e51248: f64 = (assign36560_e51246 - locals.var_qb_nqs);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36560_e51248, ((-locals.var_qd_nqs_dn0) - locals.var_qs_nqs_dn0), ((-locals.var_qd_nqs_dn2) - locals.var_qs_nqs_dn2), ((-locals.var_qd_nqs_dn6) - locals.var_qs_nqs_dn6), ((-locals.var_qd_nqs_dn7) - locals.var_qs_nqs_dn7), ((-locals.var_qd_nqs_dn10) - locals.var_qs_nqs_dn10), ((-locals.var_qd_nqs_dn11) - locals.var_qs_nqs_dn11), ((-locals.var_qd_nqs_dn12) - locals.var_qs_nqs_dn12), (-locals.var_qb_nqs_dn13), (-locals.var_qd_nqs_dn15), (-locals.var_qs_nqs_dn16), ((-locals.var_qd_nqs_dn17) - locals.var_qs_nqs_dn17), ((-locals.var_qd_nqs_dn18) - locals.var_qs_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }

        if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }

        let assign36660_e51311: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign36660_e51311;
        locals.var_guard1210_rv = 0.0;

        if (locals.var_guard1210 != 0.0) {
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, );
            locals.var_ids_rv = 0.0;
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17, ) = (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, );
            locals.var_isub_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1210 != 0.0) {
            let assign36700_e51327: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36700_e51327, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }

        if (locals.var_guard1210 != 0.0) {
            let assign36710_e51333: f64 = (locals.var_qde + locals.var_qd_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36710_e51333, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }

        if (locals.var_guard1210 != 0.0) {
            let assign36730_e51345: f64 = (locals.var_qge + locals.var_qde);
            let assign36730_e51347: f64 = (assign36730_e51345 + locals.var_qse);
            let assign36730_e51348: f64 = (-assign36730_e51347);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36730_e51348, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }

        if (locals.var_guard1210 != 0.0) {
            let assign36740_e51354: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36740_e51354, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }

        if (locals.var_guard1210 == 0.0) {
            let assign36750_e51360: f64 = (-locals.var_idse);
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign36750_e51360, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17), );
            locals.var_ids_rv = 0.0;
        }

        if (locals.var_guard1210 == 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isub_rv = 0.0;
        }

        if (locals.var_guard1210 == 0.0) {
            let assign36780_e51377: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36780_e51377, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }

        if (locals.var_guard1210 == 0.0) {
            let assign36790_e51384: f64 = (locals.var_qse + locals.var_qs_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36790_e51384, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }

        if (locals.var_guard1210 == 0.0) {
            let assign36810_e51398: f64 = (locals.var_qge + locals.var_qde);
            let assign36810_e51400: f64 = (assign36810_e51398 + locals.var_qse);
            let assign36810_e51401: f64 = (-assign36810_e51400);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36810_e51401, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }

        if (locals.var_guard1210 == 0.0) {
            let assign36820_e51408: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36820_e51408, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }

        let assign36880_e51418: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign36880_e51418;
        locals.var_guard1211_rv = 0.0;

        if (locals.var_guard1211 != 0.0) {
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, );
            locals.var_ibd_rv = 0.0;
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, );
            locals.var_qbd_rv = 0.0;
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, );
            locals.var_ibs_rv = 0.0;
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, );
            locals.var_qbs_rv = 0.0;
        }

        let assign36930_e51441: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign36930_e51441;
        locals.var_guard1212_rv = 0.0;

        if (locals.var_guard1212 != 0.0) {
            locals.var_cthe = locals.var_cth;
            locals.var_cthe_rv = 0.0;
        }

        if (locals.var_guard1212 == 0.0) {
            locals.var_cthe = 0.0;
            locals.var_cthe_rv = 0.0;
        }

        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, );
        locals.var_idse_rv = 0.0;

        let assign37150_e51521: f64 = locals.var_qg_dn6;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign37150_e51521, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;

        let assign37160_e51524: f64 = (p.p50 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign37160_e51524, (p.p50 * locals.var_cgdbd_dn0), (p.p50 * locals.var_cgdbd_dn2), (p.p50 * locals.var_cgdbd_dn6), (p.p50 * locals.var_cgdbd_dn7), (p.p50 * locals.var_cgdbd_dn10), (p.p50 * locals.var_cgdbd_dn11), (p.p50 * locals.var_cgdbd_dn12), (p.p50 * locals.var_cgdbd_dn13), (p.p50 * locals.var_cgdbd_dn15), (p.p50 * locals.var_cgdbd_dn16), (p.p50 * locals.var_cgdbd_dn17), (p.p50 * locals.var_cgdbd_dn18), );
        locals.var_cgdbd_rv = 0.0;

        let assign37170_e51527: f64 = locals.var_qg_dn7;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign37170_e51527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;

        let assign37180_e51530: f64 = (p.p50 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign37180_e51530, (p.p50 * locals.var_cgsbd_dn0), (p.p50 * locals.var_cgsbd_dn2), (p.p50 * locals.var_cgsbd_dn6), (p.p50 * locals.var_cgsbd_dn7), (p.p50 * locals.var_cgsbd_dn10), (p.p50 * locals.var_cgsbd_dn11), (p.p50 * locals.var_cgsbd_dn12), (p.p50 * locals.var_cgsbd_dn13), (p.p50 * locals.var_cgsbd_dn15), (p.p50 * locals.var_cgsbd_dn16), (p.p50 * locals.var_cgsbd_dn17), (p.p50 * locals.var_cgsbd_dn18), );
        locals.var_cgsbd_rv = 0.0;

        let assign37450_e51611: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign37450_e51611;
        locals.var_guard1214_rv = 0.0;

        if (locals.var_guard1214 != 0.0) {
            let assign37460_e51615: f64 = (p.p50 * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign37460_e51615, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }

        if (locals.var_guard1214 != 0.0) {
            let assign37470_e51621: f64 = (p.p50 * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign37470_e51621, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }

        let assign37590_e51675: f64 = (4.0 * 1.3806226e-23);
        let assign37590_e51677: f64 = (assign37590_e51675 * locals.var_ttemp);
        let assign37590_e51679: f64 = assign37590_e51677;
        (locals.var_whi_noise, locals.var_whi_noise_dn10, ) = (assign37590_e51679, (assign37590_e51675 * locals.var_ttemp_dn10), );
        locals.var_whi_noise_rv = 0.0;

        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17, );
        locals.var_qdrat_rv = 0.0;

        let assign37620_e51686: f64 = (locals.var_whi_noise * locals.var_noithrml);
        (locals.var_sid, locals.var_sid_dn0, locals.var_sid_dn2, locals.var_sid_dn6, locals.var_sid_dn7, locals.var_sid_dn10, locals.var_sid_dn11, locals.var_sid_dn12, locals.var_sid_dn17, ) = (assign37620_e51686, (locals.var_whi_noise * locals.var_noithrml_dn0), (locals.var_whi_noise * locals.var_noithrml_dn2), (locals.var_whi_noise * locals.var_noithrml_dn6), (locals.var_whi_noise * locals.var_noithrml_dn7), ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10)), (locals.var_whi_noise * locals.var_noithrml_dn11), (locals.var_whi_noise * locals.var_noithrml_dn12), (locals.var_whi_noise * locals.var_noithrml_dn17), );
        locals.var_sid_rv = 0.0;

        let (assign37640_e51700, assign37640_e51700_d_n0, assign37640_e51700_d_n2, assign37640_e51700_d_n6, assign37640_e51700_d_n7, assign37640_e51700_d_n10, assign37640_e51700_d_n11, assign37640_e51700_d_n12, assign37640_e51700_d_n13, assign37640_e51700_d_n15, assign37640_e51700_d_n16, assign37640_e51700_d_n17, assign37640_e51700_d_n18,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign37640_e51697: f64 = (locals.var_noiigate / locals.var_sid);
        let assign37640_e51698: f64 = (assign37640_e51697).sqrt();
        (assign37640_e51698, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn13 / locals.var_sid) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn15 / locals.var_sid) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn16 / locals.var_sid) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn17 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn17)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn18 / locals.var_sid) / (2.0 * assign37640_e51698)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        (locals.var_sigrat, locals.var_sigrat_dn0, locals.var_sigrat_dn2, locals.var_sigrat_dn6, locals.var_sigrat_dn7, locals.var_sigrat_dn10, locals.var_sigrat_dn11, locals.var_sigrat_dn12, locals.var_sigrat_dn13, locals.var_sigrat_dn15, locals.var_sigrat_dn16, locals.var_sigrat_dn17, locals.var_sigrat_dn18, ) = (assign37640_e51700, assign37640_e51700_d_n0, assign37640_e51700_d_n2, assign37640_e51700_d_n6, assign37640_e51700_d_n7, assign37640_e51700_d_n10, assign37640_e51700_d_n11, assign37640_e51700_d_n12, assign37640_e51700_d_n13, assign37640_e51700_d_n15, assign37640_e51700_d_n16, assign37640_e51700_d_n17, assign37640_e51700_d_n18, );
        locals.var_sigrat_rv = 0.0;

        let (assign37650_e51712, assign37650_e51712_d_n0, assign37650_e51712_d_n2, assign37650_e51712_d_n6, assign37650_e51712_d_n7, assign37650_e51712_d_n10, assign37650_e51712_d_n11, assign37650_e51712_d_n12, assign37650_e51712_d_n13, assign37650_e51712_d_n15, assign37650_e51712_d_n16, assign37650_e51712_d_n17, assign37650_e51712_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37650_e51707: f64 = (1.0 - locals.var_qdrat);
        let assign37650_e51708: f64 = (locals.var_sigrat * assign37650_e51707);
        (assign37650_e51708, ((locals.var_sigrat_dn0 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37650_e51707), (locals.var_sigrat_dn15 * assign37650_e51707), (locals.var_sigrat_dn16 * assign37650_e51707), ((locals.var_sigrat_dn17 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37650_e51707),)
    } else {
        let assign37650_e51711: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37650_e51711, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    }
};
        (locals.var_sigrat_s, locals.var_sigrat_s_dn0, locals.var_sigrat_s_dn2, locals.var_sigrat_s_dn6, locals.var_sigrat_s_dn7, locals.var_sigrat_s_dn10, locals.var_sigrat_s_dn11, locals.var_sigrat_s_dn12, locals.var_sigrat_s_dn13, locals.var_sigrat_s_dn15, locals.var_sigrat_s_dn16, locals.var_sigrat_s_dn17, locals.var_sigrat_s_dn18, ) = (assign37650_e51712, assign37650_e51712_d_n0, assign37650_e51712_d_n2, assign37650_e51712_d_n6, assign37650_e51712_d_n7, assign37650_e51712_d_n10, assign37650_e51712_d_n11, assign37650_e51712_d_n12, assign37650_e51712_d_n13, assign37650_e51712_d_n15, assign37650_e51712_d_n16, assign37650_e51712_d_n17, assign37650_e51712_d_n18, );
        locals.var_sigrat_s_rv = 0.0;

        let (assign37660_e51724, assign37660_e51724_d_n0, assign37660_e51724_d_n2, assign37660_e51724_d_n6, assign37660_e51724_d_n7, assign37660_e51724_d_n10, assign37660_e51724_d_n11, assign37660_e51724_d_n12, assign37660_e51724_d_n13, assign37660_e51724_d_n15, assign37660_e51724_d_n16, assign37660_e51724_d_n17, assign37660_e51724_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37660_e51718: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37660_e51718, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    } else {
        let assign37660_e51722: f64 = (1.0 - locals.var_qdrat);
        let assign37660_e51723: f64 = (locals.var_sigrat * assign37660_e51722);
        (assign37660_e51723, ((locals.var_sigrat_dn0 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37660_e51722), (locals.var_sigrat_dn15 * assign37660_e51722), (locals.var_sigrat_dn16 * assign37660_e51722), ((locals.var_sigrat_dn17 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37660_e51722),)
    }
};
        (locals.var_sigrat_d, locals.var_sigrat_d_dn0, locals.var_sigrat_d_dn2, locals.var_sigrat_d_dn6, locals.var_sigrat_d_dn7, locals.var_sigrat_d_dn10, locals.var_sigrat_d_dn11, locals.var_sigrat_d_dn12, locals.var_sigrat_d_dn13, locals.var_sigrat_d_dn15, locals.var_sigrat_d_dn16, locals.var_sigrat_d_dn17, locals.var_sigrat_d_dn18, ) = (assign37660_e51724, assign37660_e51724_d_n0, assign37660_e51724_d_n2, assign37660_e51724_d_n6, assign37660_e51724_d_n7, assign37660_e51724_d_n10, assign37660_e51724_d_n11, assign37660_e51724_d_n12, assign37660_e51724_d_n13, assign37660_e51724_d_n15, assign37660_e51724_d_n16, assign37660_e51724_d_n17, assign37660_e51724_d_n18, );
        locals.var_sigrat_d_rv = 0.0;

        let assign37680_e51734: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign37680_e51734;
        locals.var_guard1222_rv = 0.0;

        let assign37700_e51741: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign37700_e51741;
        locals.var_guard1223_rv = 0.0;

        let assign37710_e51750: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign37710_e51750;
        locals.var_guard1224_rv = 0.0;

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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq2_e313: f64 = (p.p50 * locals.var_ids);
        let eq2_e313_d_n0: f64 = (p.p50 * locals.var_ids_dn0);
        let eq2_e313_d_n2: f64 = (p.p50 * locals.var_ids_dn2);
        let eq2_e313_d_n6: f64 = (p.p50 * locals.var_ids_dn6);
        let eq2_e313_d_n7: f64 = (p.p50 * locals.var_ids_dn7);
        let eq2_e313_d_n10: f64 = (p.p50 * locals.var_ids_dn10);
        let eq2_e313_d_n11: f64 = (p.p50 * locals.var_ids_dn11);
        let eq2_e313_d_n12: f64 = (p.p50 * locals.var_ids_dn12);
        let eq2_e313_d_n17: f64 = (p.p50 * locals.var_ids_dn17);
        let eq2_value: f64 = eq2_e313;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq2_e313_d_n0), multiplicity * (eq2_e313_d_n2), multiplicity * (eq2_e313_d_n6), multiplicity * (eq2_e313_d_n7), multiplicity * (eq2_e313_d_n10), multiplicity * (eq2_e313_d_n11), multiplicity * (eq2_e313_d_n12), multiplicity * (eq2_e313_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq3_e319, eq3_e319_d_n0, eq3_e319_d_n2, eq3_e319_d_n6, eq3_e319_d_n7, eq3_e319_d_n10, eq3_e319_d_n11, eq3_e319_d_n12, eq3_e319_d_n17,) = {
    if (locals.var_guard1220 != 0.0) {
        let eq3_e317: f64 = (p.p50 * locals.var_igs);
        let eq3_e317_d_n0: f64 = (p.p50 * locals.var_igs_dn0);
        let eq3_e317_d_n2: f64 = (p.p50 * locals.var_igs_dn2);
        let eq3_e317_d_n6: f64 = (p.p50 * locals.var_igs_dn6);
        let eq3_e317_d_n7: f64 = (p.p50 * locals.var_igs_dn7);
        let eq3_e317_d_n10: f64 = (p.p50 * locals.var_igs_dn10);
        let eq3_e317_d_n11: f64 = (p.p50 * locals.var_igs_dn11);
        let eq3_e317_d_n12: f64 = (p.p50 * locals.var_igs_dn12);
        let eq3_e317_d_n17: f64 = (p.p50 * locals.var_igs_dn17);
        (eq3_e317, eq3_e317_d_n0, eq3_e317_d_n2, eq3_e317_d_n6, eq3_e317_d_n7, eq3_e317_d_n10, eq3_e317_d_n11, eq3_e317_d_n12, eq3_e317_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e319;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq3_e319_d_n0), multiplicity * (eq3_e319_d_n2), multiplicity * (eq3_e319_d_n6), multiplicity * (eq3_e319_d_n7), multiplicity * (eq3_e319_d_n10), multiplicity * (eq3_e319_d_n11), multiplicity * (eq3_e319_d_n12), multiplicity * (eq3_e319_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq4_e325, eq4_e325_d_n0, eq4_e325_d_n2, eq4_e325_d_n6, eq4_e325_d_n7, eq4_e325_d_n10, eq4_e325_d_n11, eq4_e325_d_n12, eq4_e325_d_n17,) = {
    if (locals.var_guard1220 != 0.0) {
        let eq4_e323: f64 = (p.p50 * locals.var_igd);
        let eq4_e323_d_n0: f64 = (p.p50 * locals.var_igd_dn0);
        let eq4_e323_d_n2: f64 = (p.p50 * locals.var_igd_dn2);
        let eq4_e323_d_n6: f64 = (p.p50 * locals.var_igd_dn6);
        let eq4_e323_d_n7: f64 = (p.p50 * locals.var_igd_dn7);
        let eq4_e323_d_n10: f64 = (p.p50 * locals.var_igd_dn10);
        let eq4_e323_d_n11: f64 = (p.p50 * locals.var_igd_dn11);
        let eq4_e323_d_n12: f64 = (p.p50 * locals.var_igd_dn12);
        let eq4_e323_d_n17: f64 = (p.p50 * locals.var_igd_dn17);
        (eq4_e323, eq4_e323_d_n0, eq4_e323_d_n2, eq4_e323_d_n6, eq4_e323_d_n7, eq4_e323_d_n10, eq4_e323_d_n11, eq4_e323_d_n12, eq4_e323_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e325;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e325_d_n0), multiplicity * (eq4_e325_d_n2), multiplicity * (eq4_e325_d_n6), multiplicity * (eq4_e325_d_n7), multiplicity * (eq4_e325_d_n10), multiplicity * (eq4_e325_d_n11), multiplicity * (eq4_e325_d_n12), multiplicity * (eq4_e325_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e331, eq5_e331_d_n0, eq5_e331_d_n2, eq5_e331_d_n6, eq5_e331_d_n7, eq5_e331_d_n10, eq5_e331_d_n11, eq5_e331_d_n12, eq5_e331_d_n17,) = {
    if (locals.var_guard1220 != 0.0) {
        let eq5_e329: f64 = (p.p50 * locals.var_igb);
        let eq5_e329_d_n0: f64 = (p.p50 * locals.var_igb_dn0);
        let eq5_e329_d_n2: f64 = (p.p50 * locals.var_igb_dn2);
        let eq5_e329_d_n6: f64 = (p.p50 * locals.var_igb_dn6);
        let eq5_e329_d_n7: f64 = (p.p50 * locals.var_igb_dn7);
        let eq5_e329_d_n10: f64 = (p.p50 * locals.var_igb_dn10);
        let eq5_e329_d_n11: f64 = (p.p50 * locals.var_igb_dn11);
        let eq5_e329_d_n12: f64 = (p.p50 * locals.var_igb_dn12);
        let eq5_e329_d_n17: f64 = (p.p50 * locals.var_igb_dn17);
        (eq5_e329, eq5_e329_d_n0, eq5_e329_d_n2, eq5_e329_d_n6, eq5_e329_d_n7, eq5_e329_d_n10, eq5_e329_d_n11, eq5_e329_d_n12, eq5_e329_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e331;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e331_d_n0), multiplicity * (eq5_e331_d_n2), multiplicity * (eq5_e331_d_n6), multiplicity * (eq5_e331_d_n7), multiplicity * (eq5_e331_d_n10), multiplicity * (eq5_e331_d_n11), multiplicity * (eq5_e331_d_n12), multiplicity * (eq5_e331_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq6_e337, eq6_e337_d_n0, eq6_e337_d_n2, eq6_e337_d_n6, eq6_e337_d_n7, eq6_e337_d_n10, eq6_e337_d_n11, eq6_e337_d_n12, eq6_e337_d_n17,) = {
    if (p.p259 != 0.0) {
        let eq6_e335: f64 = ((nv7 - nv2) / locals.var_rsd);
        let eq6_e335_d_n0: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn0) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e335_d_n2: f64 = (((-locals.var_rsd) - ((nv7 - nv2) * locals.var_rsd_dn2)) / (locals.var_rsd * locals.var_rsd));
        let eq6_e335_d_n6: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn6) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e335_d_n7: f64 = ((locals.var_rsd - ((nv7 - nv2) * locals.var_rsd_dn7)) / (locals.var_rsd * locals.var_rsd));
        let eq6_e335_d_n10: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn10) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e335_d_n11: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn11) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e335_d_n12: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn12) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e335_d_n17: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn17) / (locals.var_rsd * locals.var_rsd)));
        (eq6_e335, eq6_e335_d_n0, eq6_e335_d_n2, eq6_e335_d_n6, eq6_e335_d_n7, eq6_e335_d_n10, eq6_e335_d_n11, eq6_e335_d_n12, eq6_e335_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e337;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq6_e337_d_n0), multiplicity * (eq6_e337_d_n2), multiplicity * (eq6_e337_d_n6), multiplicity * (eq6_e337_d_n7), multiplicity * (eq6_e337_d_n10), multiplicity * (eq6_e337_d_n11), multiplicity * (eq6_e337_d_n12), multiplicity * (eq6_e337_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq8_e348, eq8_e348_d_n0, eq8_e348_d_n2, eq8_e348_d_n6, eq8_e348_d_n7, eq8_e348_d_n10, eq8_e348_d_n11, eq8_e348_d_n12, eq8_e348_d_n17,) = {
    if (p.p260 != 0.0) {
        let eq8_e346: f64 = ((nv0 - nv6) / locals.var_rdd);
        let eq8_e346_d_n0: f64 = ((locals.var_rdd - ((nv0 - nv6) * locals.var_rdd_dn0)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e346_d_n2: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn2) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e346_d_n6: f64 = (((-locals.var_rdd) - ((nv0 - nv6) * locals.var_rdd_dn6)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e346_d_n7: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn7) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e346_d_n10: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn10) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e346_d_n11: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn11) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e346_d_n12: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn12) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e346_d_n17: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn17) / (locals.var_rdd * locals.var_rdd)));
        (eq8_e346, eq8_e346_d_n0, eq8_e346_d_n2, eq8_e346_d_n6, eq8_e346_d_n7, eq8_e346_d_n10, eq8_e346_d_n11, eq8_e346_d_n12, eq8_e346_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e348;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq8_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq8_e348_d_n0), multiplicity * (eq8_e348_d_n2), multiplicity * (eq8_e348_d_n6), multiplicity * (eq8_e348_d_n7), multiplicity * (eq8_e348_d_n10), multiplicity * (eq8_e348_d_n11), multiplicity * (eq8_e348_d_n12), multiplicity * (eq8_e348_d_n17)],
            [],
            [],
            1.0,
        );
        let eq10_e356: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qg);
        let eq10_e357: f64 = (p.p50 * eq10_e356);
        let eq10_e357_d_n0: f64 = (p.p50 * (locals.var_qg_dn0 * ddt_scale));
        let eq10_e357_d_n2: f64 = (p.p50 * (locals.var_qg_dn2 * ddt_scale));
        let eq10_e357_d_n6: f64 = (p.p50 * (locals.var_qg_dn6 * ddt_scale));
        let eq10_e357_d_n7: f64 = (p.p50 * (locals.var_qg_dn7 * ddt_scale));
        let eq10_e357_d_n10: f64 = (p.p50 * (locals.var_qg_dn10 * ddt_scale));
        let eq10_e357_d_n11: f64 = (p.p50 * (locals.var_qg_dn11 * ddt_scale));
        let eq10_e357_d_n12: f64 = (p.p50 * (locals.var_qg_dn12 * ddt_scale));
        let eq10_e357_d_n13: f64 = (p.p50 * (locals.var_qg_dn13 * ddt_scale));
        let eq10_e357_d_n15: f64 = (p.p50 * (locals.var_qg_dn15 * ddt_scale));
        let eq10_e357_d_n16: f64 = (p.p50 * (locals.var_qg_dn16 * ddt_scale));
        let eq10_e357_d_n17: f64 = (p.p50 * (locals.var_qg_dn17 * ddt_scale));
        let eq10_e357_d_n18: f64 = (p.p50 * (locals.var_qg_dn18 * ddt_scale));
        let eq10_value: f64 = eq10_e357;
        let eq10_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq10_node_derivatives: [f64; 12] = [eq10_e357_d_n0, eq10_e357_d_n2, eq10_e357_d_n6, eq10_e357_d_n7, eq10_e357_d_n10, eq10_e357_d_n11, eq10_e357_d_n12, eq10_e357_d_n13, eq10_e357_d_n15, eq10_e357_d_n16, eq10_e357_d_n17, eq10_e357_d_n18];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qd);
        let eq11_e361: f64 = (p.p50 * eq11_e360);
        let eq11_e361_d_n0: f64 = (p.p50 * (locals.var_qd_dn0 * ddt_scale));
        let eq11_e361_d_n2: f64 = (p.p50 * (locals.var_qd_dn2 * ddt_scale));
        let eq11_e361_d_n6: f64 = (p.p50 * (locals.var_qd_dn6 * ddt_scale));
        let eq11_e361_d_n7: f64 = (p.p50 * (locals.var_qd_dn7 * ddt_scale));
        let eq11_e361_d_n10: f64 = (p.p50 * (locals.var_qd_dn10 * ddt_scale));
        let eq11_e361_d_n11: f64 = (p.p50 * (locals.var_qd_dn11 * ddt_scale));
        let eq11_e361_d_n12: f64 = (p.p50 * (locals.var_qd_dn12 * ddt_scale));
        let eq11_e361_d_n13: f64 = (p.p50 * (locals.var_qd_dn13 * ddt_scale));
        let eq11_e361_d_n15: f64 = (p.p50 * (locals.var_qd_dn15 * ddt_scale));
        let eq11_e361_d_n16: f64 = (p.p50 * (locals.var_qd_dn16 * ddt_scale));
        let eq11_e361_d_n17: f64 = (p.p50 * (locals.var_qd_dn17 * ddt_scale));
        let eq11_e361_d_n18: f64 = (p.p50 * (locals.var_qd_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e361;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e361_d_n0, eq11_e361_d_n2, eq11_e361_d_n6, eq11_e361_d_n7, eq11_e361_d_n10, eq11_e361_d_n11, eq11_e361_d_n12, eq11_e361_d_n13, eq11_e361_d_n15, eq11_e361_d_n16, eq11_e361_d_n17, eq11_e361_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qb);
        let eq12_e365: f64 = (p.p50 * eq12_e364);
        let eq12_e365_d_n0: f64 = (p.p50 * (locals.var_qb_dn0 * ddt_scale));
        let eq12_e365_d_n2: f64 = (p.p50 * (locals.var_qb_dn2 * ddt_scale));
        let eq12_e365_d_n6: f64 = (p.p50 * (locals.var_qb_dn6 * ddt_scale));
        let eq12_e365_d_n7: f64 = (p.p50 * (locals.var_qb_dn7 * ddt_scale));
        let eq12_e365_d_n10: f64 = (p.p50 * (locals.var_qb_dn10 * ddt_scale));
        let eq12_e365_d_n11: f64 = (p.p50 * (locals.var_qb_dn11 * ddt_scale));
        let eq12_e365_d_n12: f64 = (p.p50 * (locals.var_qb_dn12 * ddt_scale));
        let eq12_e365_d_n13: f64 = (p.p50 * (locals.var_qb_dn13 * ddt_scale));
        let eq12_e365_d_n15: f64 = (p.p50 * (locals.var_qb_dn15 * ddt_scale));
        let eq12_e365_d_n16: f64 = (p.p50 * (locals.var_qb_dn16 * ddt_scale));
        let eq12_e365_d_n17: f64 = (p.p50 * (locals.var_qb_dn17 * ddt_scale));
        let eq12_e365_d_n18: f64 = (p.p50 * (locals.var_qb_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e365;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e365_d_n0, eq12_e365_d_n2, eq12_e365_d_n6, eq12_e365_d_n7, eq12_e365_d_n10, eq12_e365_d_n11, eq12_e365_d_n12, eq12_e365_d_n13, eq12_e365_d_n15, eq12_e365_d_n16, eq12_e365_d_n17, eq12_e365_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq17_e391: f64 = (locals.var_ci * (nv14 - 0.0));
        let eq17_e391_d_n0: f64 = (locals.var_ci_dn0 * (nv14 - 0.0));
        let eq17_e391_d_n2: f64 = (locals.var_ci_dn2 * (nv14 - 0.0));
        let eq17_e391_d_n6: f64 = (locals.var_ci_dn6 * (nv14 - 0.0));
        let eq17_e391_d_n7: f64 = (locals.var_ci_dn7 * (nv14 - 0.0));
        let eq17_e391_d_n10: f64 = (locals.var_ci_dn10 * (nv14 - 0.0));
        let eq17_e391_d_n11: f64 = (locals.var_ci_dn11 * (nv14 - 0.0));
        let eq17_e391_d_n12: f64 = (locals.var_ci_dn12 * (nv14 - 0.0));
        let eq17_e391_d_n17: f64 = (locals.var_ci_dn17 * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e391;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq17_e391_d_n0), multiplicity * (eq17_e391_d_n2), multiplicity * (eq17_e391_d_n6), multiplicity * (eq17_e391_d_n7), multiplicity * (eq17_e391_d_n10), multiplicity * (eq17_e391_d_n11), multiplicity * (eq17_e391_d_n12), multiplicity * (locals.var_ci), multiplicity * (eq17_e391_d_n17)],
            [],
            [],
            1.0,
        );
        let eq18_e394: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e394_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e394_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e394_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e394_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e394_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e394_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e394_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e394_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e394_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e394_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e394_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e394_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e395: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e394);
        let eq18_value: f64 = eq18_e395;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq18_node_derivatives: [f64; 13] = [(eq18_e394_d_n0 * ddt_scale), (eq18_e394_d_n2 * ddt_scale), (eq18_e394_d_n6 * ddt_scale), (eq18_e394_d_n7 * ddt_scale), (eq18_e394_d_n10 * ddt_scale), (eq18_e394_d_n11 * ddt_scale), (eq18_e394_d_n12 * ddt_scale), (eq18_e394_d_n13 * ddt_scale), (locals.var_sigrat_s * ddt_scale), (eq18_e394_d_n15 * ddt_scale), (eq18_e394_d_n16 * ddt_scale), (eq18_e394_d_n17 * ddt_scale), (eq18_e394_d_n18 * ddt_scale)];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e398: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e398_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e398_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e398_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e398_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e398_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e398_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e398_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e398_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e398_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e398_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e398_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e398_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e398);
        let eq19_value: f64 = eq19_e399;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e398_d_n0 * ddt_scale), (eq19_e398_d_n2 * ddt_scale), (eq19_e398_d_n6 * ddt_scale), (eq19_e398_d_n7 * ddt_scale), (eq19_e398_d_n10 * ddt_scale), (eq19_e398_d_n11 * ddt_scale), (eq19_e398_d_n12 * ddt_scale), (eq19_e398_d_n13 * ddt_scale), (locals.var_sigrat_d * ddt_scale), (eq19_e398_d_n15 * ddt_scale), (eq19_e398_d_n16 * ddt_scale), (eq19_e398_d_n17 * ddt_scale), (eq19_e398_d_n18 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq25_e451, eq25_e451_d_n1, eq25_e451_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq25_e449: f64 = (locals.var_grg * (nv1 - nv11));
        (eq25_e449, locals.var_grg, (-locals.var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e451;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            1,
            multiplicity * (eq25_e451_d_n1),
            11,
            multiplicity * (eq25_e451_d_n11),
        );
        let (eq27_e462, eq27_e462_d_n10,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq27_e460: f64 = ((nv10 - 0.0) * locals.var_gth);
        (eq27_e460, locals.var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e462;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e462_d_n10),
        );
        let (eq28_e467, eq28_e467_d_n0, eq28_e467_d_n2, eq28_e467_d_n6, eq28_e467_d_n7, eq28_e467_d_n10, eq28_e467_d_n11, eq28_e467_d_n12, eq28_e467_d_n17,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq28_e465: f64 = (-locals.var_itemp);
        (eq28_e465, (-locals.var_itemp_dn0), (-locals.var_itemp_dn2), (-locals.var_itemp_dn6), (-locals.var_itemp_dn7), (-locals.var_itemp_dn10), (-locals.var_itemp_dn11), (-locals.var_itemp_dn12), (-locals.var_itemp_dn17),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e467;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(10),
            None,
            multiplicity * (eq28_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq28_e467_d_n0), multiplicity * (eq28_e467_d_n2), multiplicity * (eq28_e467_d_n6), multiplicity * (eq28_e467_d_n7), multiplicity * (eq28_e467_d_n10), multiplicity * (eq28_e467_d_n11), multiplicity * (eq28_e467_d_n12), multiplicity * (eq28_e467_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq30_e480, eq30_e480_d_n10,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq30_e477: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e478: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e477);
        (eq30_e478, (locals.var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e480;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e480_d_n10),
        );
        let (eq32_e495, eq32_e495_d_n0, eq32_e495_d_n2, eq32_e495_d_n6, eq32_e495_d_n7, eq32_e495_d_n10, eq32_e495_d_n11, eq32_e495_d_n12, eq32_e495_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq32_e492: f64 = (locals.var_igidl + locals.var_isub);
        let eq32_e492_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq32_e492_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq32_e492_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq32_e492_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq32_e492_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq32_e492_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq32_e492_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq32_e492_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq32_e493: f64 = (p.p50 * eq32_e492);
        let eq32_e493_d_n0: f64 = (p.p50 * eq32_e492_d_n0);
        let eq32_e493_d_n2: f64 = (p.p50 * eq32_e492_d_n2);
        let eq32_e493_d_n6: f64 = (p.p50 * eq32_e492_d_n6);
        let eq32_e493_d_n7: f64 = (p.p50 * eq32_e492_d_n7);
        let eq32_e493_d_n10: f64 = (p.p50 * eq32_e492_d_n10);
        let eq32_e493_d_n11: f64 = (p.p50 * eq32_e492_d_n11);
        let eq32_e493_d_n12: f64 = (p.p50 * eq32_e492_d_n12);
        let eq32_e493_d_n17: f64 = (p.p50 * eq32_e492_d_n17);
        (eq32_e493, eq32_e493_d_n0, eq32_e493_d_n2, eq32_e493_d_n6, eq32_e493_d_n7, eq32_e493_d_n10, eq32_e493_d_n11, eq32_e493_d_n12, eq32_e493_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e495;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq32_e495_d_n0), multiplicity * (eq32_e495_d_n2), multiplicity * (eq32_e495_d_n6), multiplicity * (eq32_e495_d_n7), multiplicity * (eq32_e495_d_n10), multiplicity * (eq32_e495_d_n11), multiplicity * (eq32_e495_d_n12), multiplicity * (eq32_e495_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq33_e503, eq33_e503_d_n0, eq33_e503_d_n2, eq33_e503_d_n6, eq33_e503_d_n7, eq33_e503_d_n10, eq33_e503_d_n11, eq33_e503_d_n12, eq33_e503_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq33_e500: f64 = (locals.var_igisl + locals.var_isubs);
        let eq33_e500_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq33_e500_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq33_e500_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq33_e500_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq33_e500_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq33_e500_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq33_e500_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq33_e500_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq33_e501: f64 = (p.p50 * eq33_e500);
        let eq33_e501_d_n0: f64 = (p.p50 * eq33_e500_d_n0);
        let eq33_e501_d_n2: f64 = (p.p50 * eq33_e500_d_n2);
        let eq33_e501_d_n6: f64 = (p.p50 * eq33_e500_d_n6);
        let eq33_e501_d_n7: f64 = (p.p50 * eq33_e500_d_n7);
        let eq33_e501_d_n10: f64 = (p.p50 * eq33_e500_d_n10);
        let eq33_e501_d_n11: f64 = (p.p50 * eq33_e500_d_n11);
        let eq33_e501_d_n12: f64 = (p.p50 * eq33_e500_d_n12);
        let eq33_e501_d_n17: f64 = (p.p50 * eq33_e500_d_n17);
        (eq33_e501, eq33_e501_d_n0, eq33_e501_d_n2, eq33_e501_d_n6, eq33_e501_d_n7, eq33_e501_d_n10, eq33_e501_d_n11, eq33_e501_d_n12, eq33_e501_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e503;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e503_d_n0), multiplicity * (eq33_e503_d_n2), multiplicity * (eq33_e503_d_n6), multiplicity * (eq33_e503_d_n7), multiplicity * (eq33_e503_d_n10), multiplicity * (eq33_e503_d_n11), multiplicity * (eq33_e503_d_n12), multiplicity * (eq33_e503_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq34_e508: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbs);
        let eq34_e509: f64 = (locals.var_ibs + eq34_e508);
        let eq34_e509_d_n0: f64 = (locals.var_ibs_dn0 + (locals.var_qbs_dn0 * ddt_scale));
        let eq34_e509_d_n2: f64 = (locals.var_ibs_dn2 + (locals.var_qbs_dn2 * ddt_scale));
        let eq34_e509_d_n6: f64 = (locals.var_ibs_dn6 + (locals.var_qbs_dn6 * ddt_scale));
        let eq34_e509_d_n7: f64 = (locals.var_ibs_dn7 + (locals.var_qbs_dn7 * ddt_scale));
        let eq34_e509_d_n10: f64 = (locals.var_ibs_dn10 + (locals.var_qbs_dn10 * ddt_scale));
        let eq34_e509_d_n11: f64 = (locals.var_ibs_dn11 + (locals.var_qbs_dn11 * ddt_scale));
        let eq34_e509_d_n12: f64 = (locals.var_ibs_dn12 + (locals.var_qbs_dn12 * ddt_scale));
        let eq34_e509_d_n17: f64 = (locals.var_ibs_dn17 + (locals.var_qbs_dn17 * ddt_scale));
        let eq34_e510: f64 = (p.p50 * eq34_e509);
        let eq34_e510_d_n0: f64 = (p.p50 * eq34_e509_d_n0);
        let eq34_e510_d_n2: f64 = (p.p50 * eq34_e509_d_n2);
        let eq34_e510_d_n6: f64 = (p.p50 * eq34_e509_d_n6);
        let eq34_e510_d_n7: f64 = (p.p50 * eq34_e509_d_n7);
        let eq34_e510_d_n10: f64 = (p.p50 * eq34_e509_d_n10);
        let eq34_e510_d_n11: f64 = (p.p50 * eq34_e509_d_n11);
        let eq34_e510_d_n12: f64 = (p.p50 * eq34_e509_d_n12);
        let eq34_e510_d_n17: f64 = (p.p50 * eq34_e509_d_n17);
        (eq34_e510, eq34_e510_d_n0, eq34_e510_d_n2, eq34_e510_d_n6, eq34_e510_d_n7, eq34_e510_d_n10, eq34_e510_d_n11, eq34_e510_d_n12, eq34_e510_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e512;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e512_d_n0), multiplicity * (eq34_e512_d_n2), multiplicity * (eq34_e512_d_n6), multiplicity * (eq34_e512_d_n7), multiplicity * (eq34_e512_d_n10), multiplicity * (eq34_e512_d_n11), multiplicity * (eq34_e512_d_n12), multiplicity * (eq34_e512_d_n17)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq35_e517: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbd);
        let eq35_e518: f64 = (locals.var_ibd + eq35_e517);
        let eq35_e518_d_n0: f64 = (locals.var_ibd_dn0 + (locals.var_qbd_dn0 * ddt_scale));
        let eq35_e518_d_n2: f64 = (locals.var_ibd_dn2 + (locals.var_qbd_dn2 * ddt_scale));
        let eq35_e518_d_n6: f64 = (locals.var_ibd_dn6 + (locals.var_qbd_dn6 * ddt_scale));
        let eq35_e518_d_n7: f64 = (locals.var_ibd_dn7 + (locals.var_qbd_dn7 * ddt_scale));
        let eq35_e518_d_n10: f64 = (locals.var_ibd_dn10 + (locals.var_qbd_dn10 * ddt_scale));
        let eq35_e518_d_n11: f64 = (locals.var_ibd_dn11 + (locals.var_qbd_dn11 * ddt_scale));
        let eq35_e518_d_n12: f64 = (locals.var_ibd_dn12 + (locals.var_qbd_dn12 * ddt_scale));
        let eq35_e518_d_n17: f64 = (locals.var_ibd_dn17 + (locals.var_qbd_dn17 * ddt_scale));
        let eq35_e519: f64 = (p.p50 * eq35_e518);
        let eq35_e519_d_n0: f64 = (p.p50 * eq35_e518_d_n0);
        let eq35_e519_d_n2: f64 = (p.p50 * eq35_e518_d_n2);
        let eq35_e519_d_n6: f64 = (p.p50 * eq35_e518_d_n6);
        let eq35_e519_d_n7: f64 = (p.p50 * eq35_e518_d_n7);
        let eq35_e519_d_n10: f64 = (p.p50 * eq35_e518_d_n10);
        let eq35_e519_d_n11: f64 = (p.p50 * eq35_e518_d_n11);
        let eq35_e519_d_n12: f64 = (p.p50 * eq35_e518_d_n12);
        let eq35_e519_d_n17: f64 = (p.p50 * eq35_e518_d_n17);
        (eq35_e519, eq35_e519_d_n0, eq35_e519_d_n2, eq35_e519_d_n6, eq35_e519_d_n7, eq35_e519_d_n10, eq35_e519_d_n11, eq35_e519_d_n12, eq35_e519_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e521;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e521_d_n0), multiplicity * (eq35_e521_d_n2), multiplicity * (eq35_e521_d_n6), multiplicity * (eq35_e521_d_n7), multiplicity * (eq35_e521_d_n10), multiplicity * (eq35_e521_d_n11), multiplicity * (eq35_e521_d_n12), multiplicity * (eq35_e521_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq36_e529, eq36_e529_d_n0, eq36_e529_d_n2, eq36_e529_d_n4, eq36_e529_d_n6, eq36_e529_d_n7, eq36_e529_d_n10, eq36_e529_d_n11, eq36_e529_d_n12, eq36_e529_d_n17,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p261 != 0.0)) {
        let eq36_e527: f64 = ((nv4 - nv12) / locals.var_rbulk);
        let eq36_e527_d_n0: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn0) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n2: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn2) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n4: f64 = (1.0 / locals.var_rbulk);
        let eq36_e527_d_n6: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn6) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n7: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn7) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n10: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn10) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n11: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn11) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n12: f64 = (((-locals.var_rbulk) - ((nv4 - nv12) * locals.var_rbulk_dn12)) / (locals.var_rbulk * locals.var_rbulk));
        let eq36_e527_d_n17: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn17) / (locals.var_rbulk * locals.var_rbulk)));
        (eq36_e527, eq36_e527_d_n0, eq36_e527_d_n2, eq36_e527_d_n4, eq36_e527_d_n6, eq36_e527_d_n7, eq36_e527_d_n10, eq36_e527_d_n11, eq36_e527_d_n12, eq36_e527_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e529;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(12),
            multiplicity * (eq36_value),
            [0, 2, 4, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e529_d_n0), multiplicity * (eq36_e529_d_n2), multiplicity * (eq36_e529_d_n4), multiplicity * (eq36_e529_d_n6), multiplicity * (eq36_e529_d_n7), multiplicity * (eq36_e529_d_n10), multiplicity * (eq36_e529_d_n11), multiplicity * (eq36_e529_d_n12), multiplicity * (eq36_e529_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq42_e572, eq42_e572_d_n0, eq42_e572_d_n2, eq42_e572_d_n6, eq42_e572_d_n7, eq42_e572_d_n10, eq42_e572_d_n11, eq42_e572_d_n12, eq42_e572_d_n17, eq42_e572_d_n18,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn7, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12, locals.var_iqi_nqs_dn17, locals.var_iqi_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e572;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(18),
            None,
            multiplicity * (eq42_value),
            [0, 2, 6, 7, 10, 11, 12, 17, 18],
            [multiplicity * (eq42_e572_d_n0), multiplicity * (eq42_e572_d_n2), multiplicity * (eq42_e572_d_n6), multiplicity * (eq42_e572_d_n7), multiplicity * (eq42_e572_d_n10), multiplicity * (eq42_e572_d_n11), multiplicity * (eq42_e572_d_n12), multiplicity * (eq42_e572_d_n17), multiplicity * (eq42_e572_d_n18)],
            [],
            [],
            1.0,
        );
        let (eq43_e578, eq43_e578_d_n0, eq43_e578_d_n2, eq43_e578_d_n6, eq43_e578_d_n7, eq43_e578_d_n10, eq43_e578_d_n11, eq43_e578_d_n12, eq43_e578_d_n13, eq43_e578_d_n15, eq43_e578_d_n16, eq43_e578_d_n17, eq43_e578_d_n18,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e578;
        let eq43_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq43_node_derivatives: [f64; 12] = [eq43_e578_d_n0, eq43_e578_d_n2, eq43_e578_d_n6, eq43_e578_d_n7, eq43_e578_d_n10, eq43_e578_d_n11, eq43_e578_d_n12, eq43_e578_d_n13, eq43_e578_d_n15, eq43_e578_d_n16, eq43_e578_d_n17, eq43_e578_d_n18];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq46_e605, eq46_e605_d_n18,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e600: f64 = (1e-9 / 0.0001);
        let eq46_e602: f64 = (eq46_e600 * (nv18 - 0.0));
        let eq46_e603: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e602);
        (eq46_e603, (eq46_e600 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e605;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e605_d_n18),
        );
        let (eq47_e616, eq47_e616_d_n13,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv13 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e613);
        (eq47_e614, (eq47_e611 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e616_d_n13),
        );
        let (eq50_e636, eq50_e636_d_n0, eq50_e636_d_n2, eq50_e636_d_n6, eq50_e636_d_n7, eq50_e636_d_n10, eq50_e636_d_n11, eq50_e636_d_n12, eq50_e636_d_n17,) = {
    if ((locals.var_guard1223 != 0.0) && (locals.var_guard1224 != 0.0)) {
        (locals.var_iqh_nqs, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn7, locals.var_iqh_nqs_dn10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12, locals.var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e636;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq50_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq50_e636_d_n0), multiplicity * (eq50_e636_d_n2), multiplicity * (eq50_e636_d_n6), multiplicity * (eq50_e636_d_n7), multiplicity * (eq50_e636_d_n10), multiplicity * (eq50_e636_d_n11), multiplicity * (eq50_e636_d_n12), multiplicity * (eq50_e636_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq52_e655, eq52_e655_d_n17,) = {
    if ((locals.var_guard1223 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let eq52_e650: f64 = (1e-9 / 0.0001);
        let eq52_e652: f64 = (eq52_e650 * (nv17 - 0.0));
        let eq52_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e652);
        (eq52_e653, (eq52_e650 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e655;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e655_d_n17),
        );
        let (eq54_e671, eq54_e671_d_n0, eq54_e671_d_n2, eq54_e671_d_n6, eq54_e671_d_n7, eq54_e671_d_n10, eq54_e671_d_n11, eq54_e671_d_n12, eq54_e671_d_n17,) = {
    if (locals.var_guard1223 == 0.0) {
        let eq54_e668: f64 = (locals.var_igidl + locals.var_isub);
        let eq54_e668_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq54_e668_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq54_e668_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq54_e668_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq54_e668_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq54_e668_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq54_e668_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq54_e668_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq54_e669: f64 = (p.p50 * eq54_e668);
        let eq54_e669_d_n0: f64 = (p.p50 * eq54_e668_d_n0);
        let eq54_e669_d_n2: f64 = (p.p50 * eq54_e668_d_n2);
        let eq54_e669_d_n6: f64 = (p.p50 * eq54_e668_d_n6);
        let eq54_e669_d_n7: f64 = (p.p50 * eq54_e668_d_n7);
        let eq54_e669_d_n10: f64 = (p.p50 * eq54_e668_d_n10);
        let eq54_e669_d_n11: f64 = (p.p50 * eq54_e668_d_n11);
        let eq54_e669_d_n12: f64 = (p.p50 * eq54_e668_d_n12);
        let eq54_e669_d_n17: f64 = (p.p50 * eq54_e668_d_n17);
        (eq54_e669, eq54_e669_d_n0, eq54_e669_d_n2, eq54_e669_d_n6, eq54_e669_d_n7, eq54_e669_d_n10, eq54_e669_d_n11, eq54_e669_d_n12, eq54_e669_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e671;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq54_e671_d_n0), multiplicity * (eq54_e671_d_n2), multiplicity * (eq54_e671_d_n6), multiplicity * (eq54_e671_d_n7), multiplicity * (eq54_e671_d_n10), multiplicity * (eq54_e671_d_n11), multiplicity * (eq54_e671_d_n12), multiplicity * (eq54_e671_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n2, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n17,) = {
    if (locals.var_guard1223 == 0.0) {
        let eq55_e677: f64 = (locals.var_igisl + locals.var_isubs);
        let eq55_e677_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq55_e677_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq55_e677_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq55_e677_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq55_e677_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq55_e677_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq55_e677_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq55_e677_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq55_e678: f64 = (p.p50 * eq55_e677);
        let eq55_e678_d_n0: f64 = (p.p50 * eq55_e677_d_n0);
        let eq55_e678_d_n2: f64 = (p.p50 * eq55_e677_d_n2);
        let eq55_e678_d_n6: f64 = (p.p50 * eq55_e677_d_n6);
        let eq55_e678_d_n7: f64 = (p.p50 * eq55_e677_d_n7);
        let eq55_e678_d_n10: f64 = (p.p50 * eq55_e677_d_n10);
        let eq55_e678_d_n11: f64 = (p.p50 * eq55_e677_d_n11);
        let eq55_e678_d_n12: f64 = (p.p50 * eq55_e677_d_n12);
        let eq55_e678_d_n17: f64 = (p.p50 * eq55_e677_d_n17);
        (eq55_e678, eq55_e678_d_n0, eq55_e678_d_n2, eq55_e678_d_n6, eq55_e678_d_n7, eq55_e678_d_n10, eq55_e678_d_n11, eq55_e678_d_n12, eq55_e678_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e680;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e680_d_n0), multiplicity * (eq55_e680_d_n2), multiplicity * (eq55_e680_d_n6), multiplicity * (eq55_e680_d_n7), multiplicity * (eq55_e680_d_n10), multiplicity * (eq55_e680_d_n11), multiplicity * (eq55_e680_d_n12), multiplicity * (eq55_e680_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq57_e692, eq57_e692_d_n0, eq57_e692_d_n2, eq57_e692_d_n6, eq57_e692_d_n7, eq57_e692_d_n10, eq57_e692_d_n11, eq57_e692_d_n12, eq57_e692_d_n17,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p37 != 0.0)) {
        (locals.var_iqh_nqs, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn7, locals.var_iqh_nqs_dn10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12, locals.var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e692;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq57_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq57_e692_d_n0), multiplicity * (eq57_e692_d_n2), multiplicity * (eq57_e692_d_n6), multiplicity * (eq57_e692_d_n7), multiplicity * (eq57_e692_d_n10), multiplicity * (eq57_e692_d_n11), multiplicity * (eq57_e692_d_n12), multiplicity * (eq57_e692_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq59_e713, eq59_e713_d_n17,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e708: f64 = (1e-9 / 0.0001);
        let eq59_e710: f64 = (eq59_e708 * (nv17 - 0.0));
        let eq59_e711: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e710);
        (eq59_e711, (eq59_e708 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e713;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e713_d_n17),
        );
        let (eq61_e728, eq61_e728_d_n0, eq61_e728_d_n2, eq61_e728_d_n6, eq61_e728_d_n7, eq61_e728_d_n10, eq61_e728_d_n11, eq61_e728_d_n12, eq61_e728_d_n13, eq61_e728_d_n15, eq61_e728_d_n16, eq61_e728_d_n17, eq61_e728_d_n18,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqd_nqs, locals.var_iqd_nqs_dn0, locals.var_iqd_nqs_dn2, locals.var_iqd_nqs_dn6, locals.var_iqd_nqs_dn7, locals.var_iqd_nqs_dn10, locals.var_iqd_nqs_dn11, locals.var_iqd_nqs_dn12, locals.var_iqd_nqs_dn13, locals.var_iqd_nqs_dn15, locals.var_iqd_nqs_dn16, locals.var_iqd_nqs_dn17, locals.var_iqd_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e728;
        let eq61_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq61_node_derivatives: [f64; 12] = [eq61_e728_d_n0, eq61_e728_d_n2, eq61_e728_d_n6, eq61_e728_d_n7, eq61_e728_d_n10, eq61_e728_d_n11, eq61_e728_d_n12, eq61_e728_d_n13, eq61_e728_d_n15, eq61_e728_d_n16, eq61_e728_d_n17, eq61_e728_d_n18];
        let eq61_branch_derivative_indices: [usize; 0] = [];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivative_indices,
            &eq61_node_derivatives,
            &eq61_branch_derivative_indices,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e735, eq62_e735_d_n0, eq62_e735_d_n2, eq62_e735_d_n6, eq62_e735_d_n7, eq62_e735_d_n10, eq62_e735_d_n11, eq62_e735_d_n12, eq62_e735_d_n13, eq62_e735_d_n15, eq62_e735_d_n16, eq62_e735_d_n17, eq62_e735_d_n18,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqs_nqs, locals.var_iqs_nqs_dn0, locals.var_iqs_nqs_dn2, locals.var_iqs_nqs_dn6, locals.var_iqs_nqs_dn7, locals.var_iqs_nqs_dn10, locals.var_iqs_nqs_dn11, locals.var_iqs_nqs_dn12, locals.var_iqs_nqs_dn13, locals.var_iqs_nqs_dn15, locals.var_iqs_nqs_dn16, locals.var_iqs_nqs_dn17, locals.var_iqs_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e735;
        let eq62_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq62_node_derivatives: [f64; 12] = [eq62_e735_d_n0, eq62_e735_d_n2, eq62_e735_d_n6, eq62_e735_d_n7, eq62_e735_d_n10, eq62_e735_d_n11, eq62_e735_d_n12, eq62_e735_d_n13, eq62_e735_d_n15, eq62_e735_d_n16, eq62_e735_d_n17, eq62_e735_d_n18];
        let eq62_branch_derivative_indices: [usize; 0] = [];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivative_indices,
            &eq62_node_derivatives,
            &eq62_branch_derivative_indices,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e742, eq63_e742_d_n0, eq63_e742_d_n2, eq63_e742_d_n6, eq63_e742_d_n7, eq63_e742_d_n10, eq63_e742_d_n11, eq63_e742_d_n12, eq63_e742_d_n13, eq63_e742_d_n15, eq63_e742_d_n16, eq63_e742_d_n17, eq63_e742_d_n18,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e742;
        let eq63_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq63_node_derivatives: [f64; 12] = [eq63_e742_d_n0, eq63_e742_d_n2, eq63_e742_d_n6, eq63_e742_d_n7, eq63_e742_d_n10, eq63_e742_d_n11, eq63_e742_d_n12, eq63_e742_d_n13, eq63_e742_d_n15, eq63_e742_d_n16, eq63_e742_d_n17, eq63_e742_d_n18];
        let eq63_branch_derivative_indices: [usize; 0] = [];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivative_indices,
            &eq63_node_derivatives,
            &eq63_branch_derivative_indices,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq67_e781, eq67_e781_d_n15,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e776: f64 = (1e-9 / 0.0001);
        let eq67_e778: f64 = (eq67_e776 * (nv15 - 0.0));
        let eq67_e779: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e778);
        (eq67_e779, (eq67_e776 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e781;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e781_d_n15),
        );
        let (eq68_e793, eq68_e793_d_n16,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e788: f64 = (1e-9 / 0.0001);
        let eq68_e790: f64 = (eq68_e788 * (nv16 - 0.0));
        let eq68_e791: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e790);
        (eq68_e791, (eq68_e788 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e793;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e793_d_n16),
        );
        let (eq69_e805, eq69_e805_d_n13,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e800: f64 = (1e-9 / 0.0001);
        let eq69_e802: f64 = (eq69_e800 * (nv13 - 0.0));
        let eq69_e803: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e802);
        (eq69_e803, (eq69_e800 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e805;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e805_d_n13),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq10_e356_q: f64 = locals.var_qg;
        let eq10_e357: f64 = (p.p50 * locals.var_qg);
        let eq10_e357_d_n0: f64 = (p.p50 * locals.var_qg_dn0);
        let eq10_e357_d_n2: f64 = (p.p50 * locals.var_qg_dn2);
        let eq10_e357_d_n6: f64 = (p.p50 * locals.var_qg_dn6);
        let eq10_e357_d_n7: f64 = (p.p50 * locals.var_qg_dn7);
        let eq10_e357_d_n10: f64 = (p.p50 * locals.var_qg_dn10);
        let eq10_e357_d_n11: f64 = (p.p50 * locals.var_qg_dn11);
        let eq10_e357_d_n12: f64 = (p.p50 * locals.var_qg_dn12);
        let eq10_e357_d_n13: f64 = (p.p50 * locals.var_qg_dn13);
        let eq10_e357_d_n15: f64 = (p.p50 * locals.var_qg_dn15);
        let eq10_e357_d_n16: f64 = (p.p50 * locals.var_qg_dn16);
        let eq10_e357_d_n17: f64 = (p.p50 * locals.var_qg_dn17);
        let eq10_e357_d_n18: f64 = (p.p50 * locals.var_qg_dn18);
        let eq10_e357_q: f64 = (p.p50 * eq10_e356_q);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e357_d_n0, 0.0, eq10_e357_d_n2, 0.0, 0.0, 0.0, eq10_e357_d_n6, eq10_e357_d_n7, 0.0, 0.0, eq10_e357_d_n10, eq10_e357_d_n11, eq10_e357_d_n12, eq10_e357_d_n13, 0.0, eq10_e357_d_n15, eq10_e357_d_n16, eq10_e357_d_n17, eq10_e357_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e360_q: f64 = locals.var_qd;
        let eq11_e361: f64 = (p.p50 * locals.var_qd);
        let eq11_e361_d_n0: f64 = (p.p50 * locals.var_qd_dn0);
        let eq11_e361_d_n2: f64 = (p.p50 * locals.var_qd_dn2);
        let eq11_e361_d_n6: f64 = (p.p50 * locals.var_qd_dn6);
        let eq11_e361_d_n7: f64 = (p.p50 * locals.var_qd_dn7);
        let eq11_e361_d_n10: f64 = (p.p50 * locals.var_qd_dn10);
        let eq11_e361_d_n11: f64 = (p.p50 * locals.var_qd_dn11);
        let eq11_e361_d_n12: f64 = (p.p50 * locals.var_qd_dn12);
        let eq11_e361_d_n13: f64 = (p.p50 * locals.var_qd_dn13);
        let eq11_e361_d_n15: f64 = (p.p50 * locals.var_qd_dn15);
        let eq11_e361_d_n16: f64 = (p.p50 * locals.var_qd_dn16);
        let eq11_e361_d_n17: f64 = (p.p50 * locals.var_qd_dn17);
        let eq11_e361_d_n18: f64 = (p.p50 * locals.var_qd_dn18);
        let eq11_e361_q: f64 = (p.p50 * eq11_e360_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e361_d_n0, 0.0, eq11_e361_d_n2, 0.0, 0.0, 0.0, eq11_e361_d_n6, eq11_e361_d_n7, 0.0, 0.0, eq11_e361_d_n10, eq11_e361_d_n11, eq11_e361_d_n12, eq11_e361_d_n13, 0.0, eq11_e361_d_n15, eq11_e361_d_n16, eq11_e361_d_n17, eq11_e361_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e364_q: f64 = locals.var_qb;
        let eq12_e365: f64 = (p.p50 * locals.var_qb);
        let eq12_e365_d_n0: f64 = (p.p50 * locals.var_qb_dn0);
        let eq12_e365_d_n2: f64 = (p.p50 * locals.var_qb_dn2);
        let eq12_e365_d_n6: f64 = (p.p50 * locals.var_qb_dn6);
        let eq12_e365_d_n7: f64 = (p.p50 * locals.var_qb_dn7);
        let eq12_e365_d_n10: f64 = (p.p50 * locals.var_qb_dn10);
        let eq12_e365_d_n11: f64 = (p.p50 * locals.var_qb_dn11);
        let eq12_e365_d_n12: f64 = (p.p50 * locals.var_qb_dn12);
        let eq12_e365_d_n13: f64 = (p.p50 * locals.var_qb_dn13);
        let eq12_e365_d_n15: f64 = (p.p50 * locals.var_qb_dn15);
        let eq12_e365_d_n16: f64 = (p.p50 * locals.var_qb_dn16);
        let eq12_e365_d_n17: f64 = (p.p50 * locals.var_qb_dn17);
        let eq12_e365_d_n18: f64 = (p.p50 * locals.var_qb_dn18);
        let eq12_e365_q: f64 = (p.p50 * eq12_e364_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e365_d_n0, 0.0, eq12_e365_d_n2, 0.0, 0.0, 0.0, eq12_e365_d_n6, eq12_e365_d_n7, 0.0, 0.0, eq12_e365_d_n10, eq12_e365_d_n11, eq12_e365_d_n12, eq12_e365_d_n13, 0.0, eq12_e365_d_n15, eq12_e365_d_n16, eq12_e365_d_n17, eq12_e365_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e394: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e394_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e394_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e394_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e394_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e394_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e394_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e394_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e394_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e394_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e394_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e394_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e394_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e395_q: f64 = eq18_e394;
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e394_d_n0, 0.0, eq18_e394_d_n2, 0.0, 0.0, 0.0, eq18_e394_d_n6, eq18_e394_d_n7, 0.0, 0.0, eq18_e394_d_n10, eq18_e394_d_n11, eq18_e394_d_n12, eq18_e394_d_n13, locals.var_sigrat_s, eq18_e394_d_n15, eq18_e394_d_n16, eq18_e394_d_n17, eq18_e394_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e398: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e398_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e398_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e398_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e398_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e398_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e398_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e398_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e398_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e398_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e398_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e398_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e398_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e399_q: f64 = eq19_e398;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e398_d_n0, 0.0, eq19_e398_d_n2, 0.0, 0.0, 0.0, eq19_e398_d_n6, eq19_e398_d_n7, 0.0, 0.0, eq19_e398_d_n10, eq19_e398_d_n11, eq19_e398_d_n12, eq19_e398_d_n13, locals.var_sigrat_d, eq19_e398_d_n15, eq19_e398_d_n16, eq19_e398_d_n17, eq19_e398_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e480, eq30_e480_d_n10, eq30_e480_q,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq30_e477: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e478_q: f64 = eq30_e477;
        (eq30_e477, locals.var_cthe, eq30_e478_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e480_d_n10),
        );
        let (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17, eq34_e512_q, eq34_e512_q_d_n0, eq34_e512_q_d_n2, eq34_e512_q_d_n6, eq34_e512_q_d_n7, eq34_e512_q_d_n10, eq34_e512_q_d_n11, eq34_e512_q_d_n12, eq34_e512_q_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq34_e508_q: f64 = locals.var_qbs;
        let eq34_e509: f64 = (locals.var_ibs + locals.var_qbs);
        let eq34_e509_d_n0: f64 = (locals.var_ibs_dn0 + locals.var_qbs_dn0);
        let eq34_e509_d_n2: f64 = (locals.var_ibs_dn2 + locals.var_qbs_dn2);
        let eq34_e509_d_n6: f64 = (locals.var_ibs_dn6 + locals.var_qbs_dn6);
        let eq34_e509_d_n7: f64 = (locals.var_ibs_dn7 + locals.var_qbs_dn7);
        let eq34_e509_d_n10: f64 = (locals.var_ibs_dn10 + locals.var_qbs_dn10);
        let eq34_e509_d_n11: f64 = (locals.var_ibs_dn11 + locals.var_qbs_dn11);
        let eq34_e509_d_n12: f64 = (locals.var_ibs_dn12 + locals.var_qbs_dn12);
        let eq34_e509_d_n17: f64 = (locals.var_ibs_dn17 + locals.var_qbs_dn17);
        let eq34_e509_q: f64 = eq34_e508_q;
        let eq34_e510: f64 = (p.p50 * eq34_e509);
        let eq34_e510_d_n0: f64 = (p.p50 * eq34_e509_d_n0);
        let eq34_e510_d_n2: f64 = (p.p50 * eq34_e509_d_n2);
        let eq34_e510_d_n6: f64 = (p.p50 * eq34_e509_d_n6);
        let eq34_e510_d_n7: f64 = (p.p50 * eq34_e509_d_n7);
        let eq34_e510_d_n10: f64 = (p.p50 * eq34_e509_d_n10);
        let eq34_e510_d_n11: f64 = (p.p50 * eq34_e509_d_n11);
        let eq34_e510_d_n12: f64 = (p.p50 * eq34_e509_d_n12);
        let eq34_e510_d_n17: f64 = (p.p50 * eq34_e509_d_n17);
        let eq34_e510_q: f64 = (p.p50 * eq34_e509_q);
        let eq34_e510_q_d_n0: f64 = (p.p50 * locals.var_qbs_dn0);
        let eq34_e510_q_d_n2: f64 = (p.p50 * locals.var_qbs_dn2);
        let eq34_e510_q_d_n6: f64 = (p.p50 * locals.var_qbs_dn6);
        let eq34_e510_q_d_n7: f64 = (p.p50 * locals.var_qbs_dn7);
        let eq34_e510_q_d_n10: f64 = (p.p50 * locals.var_qbs_dn10);
        let eq34_e510_q_d_n11: f64 = (p.p50 * locals.var_qbs_dn11);
        let eq34_e510_q_d_n12: f64 = (p.p50 * locals.var_qbs_dn12);
        let eq34_e510_q_d_n17: f64 = (p.p50 * locals.var_qbs_dn17);
        (eq34_e510, eq34_e510_d_n0, eq34_e510_d_n2, eq34_e510_d_n6, eq34_e510_d_n7, eq34_e510_d_n10, eq34_e510_d_n11, eq34_e510_d_n12, eq34_e510_d_n17, eq34_e510_q, eq34_e510_q_d_n0, eq34_e510_q_d_n2, eq34_e510_q_d_n6, eq34_e510_q_d_n7, eq34_e510_q_d_n10, eq34_e510_q_d_n11, eq34_e510_q_d_n12, eq34_e510_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e512_q_d_n0, 0.0, eq34_e512_q_d_n2, 0.0, 0.0, 0.0, eq34_e512_q_d_n6, eq34_e512_q_d_n7, 0.0, 0.0, eq34_e512_q_d_n10, eq34_e512_q_d_n11, eq34_e512_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq34_e512_q_d_n17, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n2, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq35_e517_q: f64 = locals.var_qbd;
        let eq35_e518: f64 = (locals.var_ibd + locals.var_qbd);
        let eq35_e518_d_n0: f64 = (locals.var_ibd_dn0 + locals.var_qbd_dn0);
        let eq35_e518_d_n2: f64 = (locals.var_ibd_dn2 + locals.var_qbd_dn2);
        let eq35_e518_d_n6: f64 = (locals.var_ibd_dn6 + locals.var_qbd_dn6);
        let eq35_e518_d_n7: f64 = (locals.var_ibd_dn7 + locals.var_qbd_dn7);
        let eq35_e518_d_n10: f64 = (locals.var_ibd_dn10 + locals.var_qbd_dn10);
        let eq35_e518_d_n11: f64 = (locals.var_ibd_dn11 + locals.var_qbd_dn11);
        let eq35_e518_d_n12: f64 = (locals.var_ibd_dn12 + locals.var_qbd_dn12);
        let eq35_e518_d_n17: f64 = (locals.var_ibd_dn17 + locals.var_qbd_dn17);
        let eq35_e518_q: f64 = eq35_e517_q;
        let eq35_e519: f64 = (p.p50 * eq35_e518);
        let eq35_e519_d_n0: f64 = (p.p50 * eq35_e518_d_n0);
        let eq35_e519_d_n2: f64 = (p.p50 * eq35_e518_d_n2);
        let eq35_e519_d_n6: f64 = (p.p50 * eq35_e518_d_n6);
        let eq35_e519_d_n7: f64 = (p.p50 * eq35_e518_d_n7);
        let eq35_e519_d_n10: f64 = (p.p50 * eq35_e518_d_n10);
        let eq35_e519_d_n11: f64 = (p.p50 * eq35_e518_d_n11);
        let eq35_e519_d_n12: f64 = (p.p50 * eq35_e518_d_n12);
        let eq35_e519_d_n17: f64 = (p.p50 * eq35_e518_d_n17);
        let eq35_e519_q: f64 = (p.p50 * eq35_e518_q);
        let eq35_e519_q_d_n0: f64 = (p.p50 * locals.var_qbd_dn0);
        let eq35_e519_q_d_n2: f64 = (p.p50 * locals.var_qbd_dn2);
        let eq35_e519_q_d_n6: f64 = (p.p50 * locals.var_qbd_dn6);
        let eq35_e519_q_d_n7: f64 = (p.p50 * locals.var_qbd_dn7);
        let eq35_e519_q_d_n10: f64 = (p.p50 * locals.var_qbd_dn10);
        let eq35_e519_q_d_n11: f64 = (p.p50 * locals.var_qbd_dn11);
        let eq35_e519_q_d_n12: f64 = (p.p50 * locals.var_qbd_dn12);
        let eq35_e519_q_d_n17: f64 = (p.p50 * locals.var_qbd_dn17);
        (eq35_e519, eq35_e519_d_n0, eq35_e519_d_n2, eq35_e519_d_n6, eq35_e519_d_n7, eq35_e519_d_n10, eq35_e519_d_n11, eq35_e519_d_n12, eq35_e519_d_n17, eq35_e519_q, eq35_e519_q_d_n0, eq35_e519_q_d_n2, eq35_e519_q_d_n6, eq35_e519_q_d_n7, eq35_e519_q_d_n10, eq35_e519_q_d_n11, eq35_e519_q_d_n12, eq35_e519_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e521_q_d_n0, 0.0, eq35_e521_q_d_n2, 0.0, 0.0, 0.0, eq35_e521_q_d_n6, eq35_e521_q_d_n7, 0.0, 0.0, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e521_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e605, eq46_e605_d_n18, eq46_e605_q,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e600: f64 = (1e-9 / 0.0001);
        let eq46_e602: f64 = (eq46_e600 * (nv18 - 0.0));
        let eq46_e603_q: f64 = eq46_e602;
        (eq46_e602, eq46_e600, eq46_e603_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e605_d_n18),
        );
        let (eq47_e616, eq47_e616_d_n13, eq47_e616_q,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv13 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e616_d_n13),
        );
        let (eq52_e655, eq52_e655_d_n17, eq52_e655_q,) = {
    if ((locals.var_guard1223 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let eq52_e650: f64 = (1e-9 / 0.0001);
        let eq52_e652: f64 = (eq52_e650 * (nv17 - 0.0));
        let eq52_e653_q: f64 = eq52_e652;
        (eq52_e652, eq52_e650, eq52_e653_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e655_d_n17),
        );
        let (eq59_e713, eq59_e713_d_n17, eq59_e713_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e708: f64 = (1e-9 / 0.0001);
        let eq59_e710: f64 = (eq59_e708 * (nv17 - 0.0));
        let eq59_e711_q: f64 = eq59_e710;
        (eq59_e710, eq59_e708, eq59_e711_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e713_d_n17),
        );
        let (eq67_e781, eq67_e781_d_n15, eq67_e781_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e776: f64 = (1e-9 / 0.0001);
        let eq67_e778: f64 = (eq67_e776 * (nv15 - 0.0));
        let eq67_e779_q: f64 = eq67_e778;
        (eq67_e778, eq67_e776, eq67_e779_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e781_d_n15),
        );
        let (eq68_e793, eq68_e793_d_n16, eq68_e793_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e788: f64 = (1e-9 / 0.0001);
        let eq68_e790: f64 = (eq68_e788 * (nv16 - 0.0));
        let eq68_e791_q: f64 = eq68_e790;
        (eq68_e790, eq68_e788, eq68_e791_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e793_d_n16),
        );
        let (eq69_e805, eq69_e805_d_n13, eq69_e805_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e800: f64 = (1e-9 / 0.0001);
        let eq69_e802: f64 = (eq69_e800 * (nv13 - 0.0));
        let eq69_e803_q: f64 = eq69_e802;
        (eq69_e802, eq69_e800, eq69_e803_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e805_d_n13),
        );
    }
}
