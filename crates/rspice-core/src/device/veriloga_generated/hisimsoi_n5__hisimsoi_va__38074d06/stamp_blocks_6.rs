#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1008 != 0.0)) {
            let assign30870_e44636: f64 = (locals.var_tq__blk953 * locals.var_tq__blk953);
            let assign30870_e44639: f64 = (locals.var_tp__blk954 * locals.var_tp__blk954);
            let assign30870_e44641: f64 = (assign30870_e44639 * locals.var_tp__blk954);
            let assign30870_e44642: f64 = (assign30870_e44636 + assign30870_e44641);
            let assign30870_e44643: f64 = (assign30870_e44642).sqrt();
            (locals.var_t5__blk902, locals.var_t5__blk902_dn0, locals.var_t5__blk902_dn2, locals.var_t5__blk902_dn6, locals.var_t5__blk902_dn7, locals.var_t5__blk902_dn10, locals.var_t5__blk902_dn11, locals.var_t5__blk902_dn12, locals.var_t5__blk902_dn17, ) = (assign30870_e44643, ((((locals.var_tq__blk953_dn0 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn0)) + ((((locals.var_tp__blk954_dn0 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn0)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn0))) / (2.0 * assign30870_e44643)), ((((locals.var_tq__blk953_dn2 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn2)) + ((((locals.var_tp__blk954_dn2 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn2)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn2))) / (2.0 * assign30870_e44643)), ((((locals.var_tq__blk953_dn6 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn6)) + ((((locals.var_tp__blk954_dn6 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn6)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn6))) / (2.0 * assign30870_e44643)), ((((locals.var_tq__blk953_dn7 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn7)) + ((((locals.var_tp__blk954_dn7 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn7)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn7))) / (2.0 * assign30870_e44643)), ((((locals.var_tq__blk953_dn10 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn10)) + ((((locals.var_tp__blk954_dn10 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn10)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn10))) / (2.0 * assign30870_e44643)), ((((locals.var_tq__blk953_dn11 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn11)) + ((((locals.var_tp__blk954_dn11 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn11)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn11))) / (2.0 * assign30870_e44643)), ((((locals.var_tq__blk953_dn12 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn12)) + ((((locals.var_tp__blk954_dn12 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn12)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn12))) / (2.0 * assign30870_e44643)), ((((locals.var_tq__blk953_dn17 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn17)) + ((((locals.var_tp__blk954_dn17 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn17)) * locals.var_tp__blk954) + (assign30870_e44639 * locals.var_tp__blk954_dn17))) / (2.0 * assign30870_e44643)), );
            locals.var_t5__blk902_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1008 != 0.0)) {
            let assign30880_e44658: f64 = (-locals.var_tq__blk953);
            let assign30880_e44660: f64 = (assign30880_e44658 + locals.var_t5__blk902);
            let assign30880_e44662: f64 = (assign30880_e44660).powf(0.3333333333333333);
            (locals.var_tu__blk955, locals.var_tu__blk955_dn0, locals.var_tu__blk955_dn2, locals.var_tu__blk955_dn6, locals.var_tu__blk955_dn7, locals.var_tu__blk955_dn10, locals.var_tu__blk955_dn11, locals.var_tu__blk955_dn12, locals.var_tu__blk955_dn17, ) = (assign30880_e44662, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn0) + locals.var_t5__blk902_dn0))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn0) + locals.var_t5__blk902_dn0) / assign30880_e44660))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn2) + locals.var_t5__blk902_dn2))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn2) + locals.var_t5__blk902_dn2) / assign30880_e44660))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn6) + locals.var_t5__blk902_dn6))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn6) + locals.var_t5__blk902_dn6) / assign30880_e44660))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn7) + locals.var_t5__blk902_dn7))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn7) + locals.var_t5__blk902_dn7) / assign30880_e44660))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn10) + locals.var_t5__blk902_dn10))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn10) + locals.var_t5__blk902_dn10) / assign30880_e44660))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn11) + locals.var_t5__blk902_dn11))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn11) + locals.var_t5__blk902_dn11) / assign30880_e44660))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn12) + locals.var_t5__blk902_dn12))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn12) + locals.var_t5__blk902_dn12) / assign30880_e44660))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30880_e44660).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn17) + locals.var_t5__blk902_dn17))) } } else { (assign30880_e44662 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn17) + locals.var_t5__blk902_dn17) / assign30880_e44660))) }, );
            locals.var_tu__blk955_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1008 != 0.0)) {
            let assign30890_e44678: f64 = (locals.var_tq__blk953 + locals.var_t5__blk902);
            let assign30890_e44680: f64 = (assign30890_e44678).powf(0.3333333333333333);
            let assign30890_e44681: f64 = (-assign30890_e44680);
            (locals.var_tv__blk956, locals.var_tv__blk956_dn0, locals.var_tv__blk956_dn2, locals.var_tv__blk956_dn6, locals.var_tv__blk956_dn7, locals.var_tv__blk956_dn10, locals.var_tv__blk956_dn11, locals.var_tv__blk956_dn12, locals.var_tv__blk956_dn17, ) = (assign30890_e44681, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn0 + locals.var_t5__blk902_dn0))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn0 + locals.var_t5__blk902_dn0) / assign30890_e44678))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn2 + locals.var_t5__blk902_dn2))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn2 + locals.var_t5__blk902_dn2) / assign30890_e44678))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn6 + locals.var_t5__blk902_dn6))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn6 + locals.var_t5__blk902_dn6) / assign30890_e44678))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn7 + locals.var_t5__blk902_dn7))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn7 + locals.var_t5__blk902_dn7) / assign30890_e44678))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn10 + locals.var_t5__blk902_dn10))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn10 + locals.var_t5__blk902_dn10) / assign30890_e44678))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn11 + locals.var_t5__blk902_dn11))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn11 + locals.var_t5__blk902_dn11) / assign30890_e44678))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn12 + locals.var_t5__blk902_dn12))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn12 + locals.var_t5__blk902_dn12) / assign30890_e44678))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign30890_e44678).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn17 + locals.var_t5__blk902_dn17))) } } else { (assign30890_e44680 * (0.3333333333333333 * ((locals.var_tq__blk953_dn17 + locals.var_t5__blk902_dn17) / assign30890_e44678))) }), );
            locals.var_tv__blk956_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1008 != 0.0)) {
            let assign30900_e44697: f64 = (locals.var_tu__blk955 + locals.var_tv__blk956);
            let assign30900_e44701: f64 = (3.0 * locals.var_ta__blk949);
            let assign30900_e44702: f64 = (locals.var_tb__blk950 / assign30900_e44701);
            let assign30900_e44703: f64 = (assign30900_e44697 - assign30900_e44702);
            (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17, ) = (assign30900_e44703, (locals.var_tu__blk955_dn0 + locals.var_tv__blk956_dn0), (locals.var_tu__blk955_dn2 + locals.var_tv__blk956_dn2), (locals.var_tu__blk955_dn6 + locals.var_tv__blk956_dn6), (locals.var_tu__blk955_dn7 + locals.var_tv__blk956_dn7), (locals.var_tu__blk955_dn10 + locals.var_tv__blk956_dn10), (locals.var_tu__blk955_dn11 + locals.var_tv__blk956_dn11), (locals.var_tu__blk955_dn12 + locals.var_tv__blk956_dn12), (locals.var_tu__blk955_dn17 + locals.var_tv__blk956_dn17), );
            locals.var_tx__blk906_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1008 != 0.0)) {
            let assign30910_e44719: f64 = (locals.var_tx__blk906 * locals.var_beta_inv);
            let assign30910_e44721: f64 = (assign30910_e44719 - locals.var_vxbgmtcl__blk923);
            (locals.var_ps0_inia__blk948, locals.var_ps0_inia__blk948_dn0, locals.var_ps0_inia__blk948_dn2, locals.var_ps0_inia__blk948_dn6, locals.var_ps0_inia__blk948_dn7, locals.var_ps0_inia__blk948_dn10, locals.var_ps0_inia__blk948_dn11, locals.var_ps0_inia__blk948_dn12, locals.var_ps0_inia__blk948_dn17, ) = (assign30910_e44721, ((locals.var_tx__blk906_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn0), ((locals.var_tx__blk906_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn2), ((locals.var_tx__blk906_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn6), ((locals.var_tx__blk906_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn7), (((locals.var_tx__blk906_dn10 * locals.var_beta_inv) + (locals.var_tx__blk906 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk923_dn10), ((locals.var_tx__blk906_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn11), ((locals.var_tx__blk906_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn12), ((locals.var_tx__blk906_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn17), );
            locals.var_ps0_inia__blk948_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1008 != 0.0)) {
            let assign30920_e44738: f64 = (locals.var_ps0_inia__blk948 + locals.var_vxbgmtcl__blk923);
            let assign30920_e44739: f64 = (locals.var_beta * assign30920_e44738);
            (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17, ) = (assign30920_e44739, (locals.var_beta * (locals.var_ps0_inia__blk948_dn0 + locals.var_vxbgmtcl__blk923_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn2 + locals.var_vxbgmtcl__blk923_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn6 + locals.var_vxbgmtcl__blk923_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn7 + locals.var_vxbgmtcl__blk923_dn7)), ((locals.var_beta_dn10 * assign30920_e44738) + (locals.var_beta * (locals.var_ps0_inia__blk948_dn10 + locals.var_vxbgmtcl__blk923_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk948_dn11 + locals.var_vxbgmtcl__blk923_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn12 + locals.var_vxbgmtcl__blk923_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn17 + locals.var_vxbgmtcl__blk923_dn17)), );
            locals.var_chi__blk945_rv = 0.0;
        }
        let assign30930_e44744: f64 = if p.p41 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1009 = assign30930_e44744;
        locals.var_guard1009_rv = 0.0;
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign30950_e44774: f64 = (locals.var_vgpld__blk933 + locals.var_vxbgmtcl__blk923);
            let assign30950_e44776: f64 = (assign30950_e44774 + 0.1);
            (locals.var_vgpld_shift__blk957, locals.var_vgpld_shift__blk957_dn0, locals.var_vgpld_shift__blk957_dn2, locals.var_vgpld_shift__blk957_dn6, locals.var_vgpld_shift__blk957_dn7, locals.var_vgpld_shift__blk957_dn10, locals.var_vgpld_shift__blk957_dn11, locals.var_vgpld_shift__blk957_dn12, locals.var_vgpld_shift__blk957_dn17, ) = (assign30950_e44776, (locals.var_vgpld__blk933_dn0 + locals.var_vxbgmtcl__blk923_dn0), (locals.var_vgpld__blk933_dn2 + locals.var_vxbgmtcl__blk923_dn2), (locals.var_vgpld__blk933_dn6 + locals.var_vxbgmtcl__blk923_dn6), (locals.var_vgpld__blk933_dn7 + locals.var_vxbgmtcl__blk923_dn7), (locals.var_vgpld__blk933_dn10 + locals.var_vxbgmtcl__blk923_dn10), (locals.var_vgpld__blk933_dn11 + locals.var_vxbgmtcl__blk923_dn11), (locals.var_vgpld__blk933_dn12 + locals.var_vxbgmtcl__blk923_dn12), (locals.var_vgpld__blk933_dn17 + locals.var_vxbgmtcl__blk923_dn17), );
            locals.var_vgpld_shift__blk957_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign30960_e44792: f64 = (-locals.var_vxbgmtcl__blk923);
            let assign30960_e44793: f64 = (locals.var_beta * assign30960_e44792);
            let assign30960_e44794: f64 = (assign30960_e44793).exp();
            let assign30960_e44796: f64 = (assign30960_e44794 + 1e-50);
            (locals.var_exp_bvbs__blk964, locals.var_exp_bvbs__blk964_dn0, locals.var_exp_bvbs__blk964_dn2, locals.var_exp_bvbs__blk964_dn6, locals.var_exp_bvbs__blk964_dn7, locals.var_exp_bvbs__blk964_dn10, locals.var_exp_bvbs__blk964_dn11, locals.var_exp_bvbs__blk964_dn12, locals.var_exp_bvbs__blk964_dn17, ) = (assign30960_e44796, (assign30960_e44794 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn0))), (assign30960_e44794 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn2))), (assign30960_e44794 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn6))), (assign30960_e44794 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn7))), (assign30960_e44794 * ((locals.var_beta_dn10 * assign30960_e44792) + (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn10)))), (assign30960_e44794 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn11))), (assign30960_e44794 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn12))), (assign30960_e44794 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn17))), );
            locals.var_exp_bvbs__blk964_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign30970_e44812: f64 = (locals.var_nin / locals.var_mks_nover);
            (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17, ) = (assign30970_e44812, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover), );
            locals.var_t0__blk897_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign30980_e44828: f64 = (locals.var_t0__blk897 * locals.var_t0__blk897);
            (locals.var_cnst1over__blk958, locals.var_cnst1over__blk958_dn0, locals.var_cnst1over__blk958_dn2, locals.var_cnst1over__blk958_dn6, locals.var_cnst1over__blk958_dn7, locals.var_cnst1over__blk958_dn10, locals.var_cnst1over__blk958_dn11, locals.var_cnst1over__blk958_dn12, locals.var_cnst1over__blk958_dn17, ) = (assign30980_e44828, ((locals.var_t0__blk897_dn0 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn0)), ((locals.var_t0__blk897_dn2 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn2)), ((locals.var_t0__blk897_dn6 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn6)), ((locals.var_t0__blk897_dn7 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn7)), ((locals.var_t0__blk897_dn10 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn10)), ((locals.var_t0__blk897_dn11 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn11)), ((locals.var_t0__blk897_dn12 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn12)), ((locals.var_t0__blk897_dn17 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn17)), );
            locals.var_cnst1over__blk958_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign30990_e44844: f64 = (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964);
            (locals.var_gammachi__blk959, locals.var_gammachi__blk959_dn0, locals.var_gammachi__blk959_dn2, locals.var_gammachi__blk959_dn6, locals.var_gammachi__blk959_dn7, locals.var_gammachi__blk959_dn10, locals.var_gammachi__blk959_dn11, locals.var_gammachi__blk959_dn12, locals.var_gammachi__blk959_dn17, ) = (assign30990_e44844, ((locals.var_cnst1over__blk958_dn0 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn0)), ((locals.var_cnst1over__blk958_dn2 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn2)), ((locals.var_cnst1over__blk958_dn6 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn6)), ((locals.var_cnst1over__blk958_dn7 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn7)), ((locals.var_cnst1over__blk958_dn10 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn10)), ((locals.var_cnst1over__blk958_dn11 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn11)), ((locals.var_cnst1over__blk958_dn12 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn12)), ((locals.var_cnst1over__blk958_dn17 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn17)), );
            locals.var_gammachi__blk959_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31000_e44860: f64 = (locals.var_beta2 * locals.var_fac1p2__blk932);
            (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17, ) = (assign31000_e44860, (locals.var_beta2 * locals.var_fac1p2__blk932_dn0), (locals.var_beta2 * locals.var_fac1p2__blk932_dn2), (locals.var_beta2 * locals.var_fac1p2__blk932_dn6), (locals.var_beta2 * locals.var_fac1p2__blk932_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk932) + (locals.var_beta2 * locals.var_fac1p2__blk932_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk932_dn11), (locals.var_beta2 * locals.var_fac1p2__blk932_dn12), (locals.var_beta2 * locals.var_fac1p2__blk932_dn17), );
            locals.var_t0__blk897_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31010_e44876: f64 = (locals.var_beta * locals.var_vgpld_shift__blk957);
            (locals.var_psi__blk960, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, locals.var_psi__blk960_dn10, locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17, ) = (assign31010_e44876, (locals.var_beta * locals.var_vgpld_shift__blk957_dn0), (locals.var_beta * locals.var_vgpld_shift__blk957_dn2), (locals.var_beta * locals.var_vgpld_shift__blk957_dn6), (locals.var_beta * locals.var_vgpld_shift__blk957_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift__blk957) + (locals.var_beta * locals.var_vgpld_shift__blk957_dn10)), (locals.var_beta * locals.var_vgpld_shift__blk957_dn11), (locals.var_beta * locals.var_vgpld_shift__blk957_dn12), (locals.var_beta * locals.var_vgpld_shift__blk957_dn17), );
            locals.var_psi__blk960_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31020_e44892: f64 = (locals.var_gammachi__blk959 * locals.var_t0__blk897);
            let assign31020_e44895: f64 = (locals.var_psi__blk960 * locals.var_psi__blk960);
            let assign31020_e44896: f64 = (assign31020_e44892 + assign31020_e44895);
            let assign31020_e44897: f64 = (assign31020_e44896).ln();
            let assign31020_e44900: f64 = (locals.var_cnst1over__blk958 * locals.var_t0__blk897);
            let assign31020_e44901: f64 = (assign31020_e44900).ln();
            let assign31020_e44902: f64 = (assign31020_e44897 - assign31020_e44901);
            let assign31020_e44905: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk923);
            let assign31020_e44906: f64 = (assign31020_e44902 + assign31020_e44905);
            (locals.var_chi_1__blk961, locals.var_chi_1__blk961_dn0, locals.var_chi_1__blk961_dn2, locals.var_chi_1__blk961_dn6, locals.var_chi_1__blk961_dn7, locals.var_chi_1__blk961_dn10, locals.var_chi_1__blk961_dn11, locals.var_chi_1__blk961_dn12, locals.var_chi_1__blk961_dn17, ) = (assign31020_e44906, ((((((locals.var_gammachi__blk959_dn0 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn0)) + ((locals.var_psi__blk960_dn0 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn0))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn0 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn0)) / assign31020_e44900)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn0)), ((((((locals.var_gammachi__blk959_dn2 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn2)) + ((locals.var_psi__blk960_dn2 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn2))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn2 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn2)) / assign31020_e44900)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn2)), ((((((locals.var_gammachi__blk959_dn6 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn6)) + ((locals.var_psi__blk960_dn6 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn6))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn6 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn6)) / assign31020_e44900)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn6)), ((((((locals.var_gammachi__blk959_dn7 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn7)) + ((locals.var_psi__blk960_dn7 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn7))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn7 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn7)) / assign31020_e44900)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn7)), ((((((locals.var_gammachi__blk959_dn10 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn10)) + ((locals.var_psi__blk960_dn10 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn10))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn10 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn10)) / assign31020_e44900)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk923) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn10))), ((((((locals.var_gammachi__blk959_dn11 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn11)) + ((locals.var_psi__blk960_dn11 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn11))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn11 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn11)) / assign31020_e44900)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn11)), ((((((locals.var_gammachi__blk959_dn12 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn12)) + ((locals.var_psi__blk960_dn12 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn12))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn12 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn12)) / assign31020_e44900)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn12)), ((((((locals.var_gammachi__blk959_dn17 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn17)) + ((locals.var_psi__blk960_dn17 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn17))) / assign31020_e44896) - (((locals.var_cnst1over__blk958_dn17 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn17)) / assign31020_e44900)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn17)), );
            locals.var_chi_1__blk961_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31030_e44922: f64 = (locals.var_psi__blk960 - locals.var_chi_1__blk961);
            let assign31030_e44924: f64 = (assign31030_e44922 - 1.0);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign31030_e44924, (locals.var_psi__blk960_dn0 - locals.var_chi_1__blk961_dn0), (locals.var_psi__blk960_dn2 - locals.var_chi_1__blk961_dn2), (locals.var_psi__blk960_dn6 - locals.var_chi_1__blk961_dn6), (locals.var_psi__blk960_dn7 - locals.var_chi_1__blk961_dn7), (locals.var_psi__blk960_dn10 - locals.var_chi_1__blk961_dn10), (locals.var_psi__blk960_dn11 - locals.var_chi_1__blk961_dn11), (locals.var_psi__blk960_dn12 - locals.var_chi_1__blk961_dn12), (locals.var_psi__blk960_dn17 - locals.var_chi_1__blk961_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31040_e44940: f64 = (4.0 * locals.var_psi__blk960);
            let assign31040_e44942: f64 = assign31040_e44940;
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31040_e44942, (4.0 * locals.var_psi__blk960_dn0), (4.0 * locals.var_psi__blk960_dn2), (4.0 * locals.var_psi__blk960_dn6), (4.0 * locals.var_psi__blk960_dn7), (4.0 * locals.var_psi__blk960_dn10), (4.0 * locals.var_psi__blk960_dn11), (4.0 * locals.var_psi__blk960_dn12), (4.0 * locals.var_psi__blk960_dn17), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let (assign31050_e44962, assign31050_e44962_d_n0, assign31050_e44962_d_n2, assign31050_e44962_d_n6, assign31050_e44962_d_n7, assign31050_e44962_d_n10, assign31050_e44962_d_n11, assign31050_e44962_d_n12, assign31050_e44962_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign31050_e44961: f64 = (-locals.var_tmf2);
        (assign31050_e44961, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31050_e44962, assign31050_e44962_d_n0, assign31050_e44962_d_n2, assign31050_e44962_d_n6, assign31050_e44962_d_n7, assign31050_e44962_d_n10, assign31050_e44962_d_n11, assign31050_e44962_d_n12, assign31050_e44962_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31060_e44978: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign31060_e44980: f64 = (assign31060_e44978 + locals.var_tmf2);
            let assign31060_e44981: f64 = (assign31060_e44980).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31060_e44981, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31060_e44981)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31060_e44981)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31060_e44981)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31060_e44981)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31060_e44981)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31060_e44981)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31060_e44981)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31060_e44981)), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31070_e44999: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign31070_e45000: f64 = (1.0 + assign31070_e44999);
            let assign31070_e45001: f64 = (0.5 * assign31070_e45000);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31070_e45001, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t1__blk898_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31080_e45020: f64 = 2.0;
            let assign31080_e45021: f64 = (locals.var_tmf1 + assign31080_e45020);
            let assign31080_e45023: f64 = (assign31080_e45021 / locals.var_tmf2);
            let assign31080_e45024: f64 = (1.0 - assign31080_e45023);
            let assign31080_e45025: f64 = (0.5 * assign31080_e45024);
            (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17, ) = (assign31080_e45025, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31080_e45021 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))), );
            locals.var_t2__blk899_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31090_e45043: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign31090_e45044: f64 = (0.5 * assign31090_e45043);
            let assign31090_e45045: f64 = (locals.var_psi__blk960 - assign31090_e45044);
            (locals.var_chi_1__blk961, locals.var_chi_1__blk961_dn0, locals.var_chi_1__blk961_dn2, locals.var_chi_1__blk961_dn6, locals.var_chi_1__blk961_dn7, locals.var_chi_1__blk961_dn10, locals.var_chi_1__blk961_dn11, locals.var_chi_1__blk961_dn12, locals.var_chi_1__blk961_dn17, ) = (assign31090_e45045, (locals.var_psi__blk960_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi__blk960_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi__blk960_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi__blk960_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi__blk960_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi__blk960_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi__blk960_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi__blk960_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_chi_1__blk961_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31100_e45061: f64 = (locals.var_psi__blk960 - locals.var_chi_1__blk961);
            (locals.var_psi__blk960, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, locals.var_psi__blk960_dn10, locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17, ) = (assign31100_e45061, (locals.var_psi__blk960_dn0 - locals.var_chi_1__blk961_dn0), (locals.var_psi__blk960_dn2 - locals.var_chi_1__blk961_dn2), (locals.var_psi__blk960_dn6 - locals.var_chi_1__blk961_dn6), (locals.var_psi__blk960_dn7 - locals.var_chi_1__blk961_dn7), (locals.var_psi__blk960_dn10 - locals.var_chi_1__blk961_dn10), (locals.var_psi__blk960_dn11 - locals.var_chi_1__blk961_dn11), (locals.var_psi__blk960_dn12 - locals.var_chi_1__blk961_dn12), (locals.var_psi__blk960_dn17 - locals.var_chi_1__blk961_dn17), );
            locals.var_psi__blk960_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31110_e45078: f64 = (locals.var_beta * 0.1);
            let assign31110_e45079: f64 = (locals.var_psi__blk960 + assign31110_e45078);
            (locals.var_psi__blk960, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, locals.var_psi__blk960_dn10, locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17, ) = (assign31110_e45079, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, (locals.var_psi__blk960_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17, );
            locals.var_psi__blk960_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31120_e45095: f64 = (locals.var_gammachi__blk959 * locals.var_t0__blk897);
            let assign31120_e45098: f64 = (locals.var_psi__blk960 * locals.var_psi__blk960);
            let assign31120_e45099: f64 = (assign31120_e45095 + assign31120_e45098);
            let assign31120_e45100: f64 = (assign31120_e45099).ln();
            let assign31120_e45103: f64 = (locals.var_cnst1over__blk958 * locals.var_t0__blk897);
            let assign31120_e45104: f64 = (assign31120_e45103).ln();
            let assign31120_e45105: f64 = (assign31120_e45100 - assign31120_e45104);
            let assign31120_e45108: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk923);
            let assign31120_e45109: f64 = (assign31120_e45105 + assign31120_e45108);
            (locals.var_chi_b__blk962, locals.var_chi_b__blk962_dn0, locals.var_chi_b__blk962_dn2, locals.var_chi_b__blk962_dn6, locals.var_chi_b__blk962_dn7, locals.var_chi_b__blk962_dn10, locals.var_chi_b__blk962_dn11, locals.var_chi_b__blk962_dn12, locals.var_chi_b__blk962_dn17, ) = (assign31120_e45109, ((((((locals.var_gammachi__blk959_dn0 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn0)) + ((locals.var_psi__blk960_dn0 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn0))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn0 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn0)) / assign31120_e45103)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn0)), ((((((locals.var_gammachi__blk959_dn2 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn2)) + ((locals.var_psi__blk960_dn2 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn2))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn2 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn2)) / assign31120_e45103)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn2)), ((((((locals.var_gammachi__blk959_dn6 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn6)) + ((locals.var_psi__blk960_dn6 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn6))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn6 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn6)) / assign31120_e45103)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn6)), ((((((locals.var_gammachi__blk959_dn7 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn7)) + ((locals.var_psi__blk960_dn7 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn7))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn7 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn7)) / assign31120_e45103)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn7)), ((((((locals.var_gammachi__blk959_dn10 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn10)) + ((locals.var_psi__blk960_dn10 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn10))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn10 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn10)) / assign31120_e45103)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk923) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn10))), ((((((locals.var_gammachi__blk959_dn11 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn11)) + ((locals.var_psi__blk960_dn11 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn11))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn11 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn11)) / assign31120_e45103)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn11)), ((((((locals.var_gammachi__blk959_dn12 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn12)) + ((locals.var_psi__blk960_dn12 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn12))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn12 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn12)) / assign31120_e45103)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn12)), ((((((locals.var_gammachi__blk959_dn17 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn17)) + ((locals.var_psi__blk960_dn17 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn17))) / assign31120_e45099) - (((locals.var_cnst1over__blk958_dn17 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn17)) / assign31120_e45103)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn17)), );
            locals.var_chi_b__blk962_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            (locals.var_chi_a__blk963, locals.var_chi_a__blk963_dn0, locals.var_chi_a__blk963_dn2, locals.var_chi_a__blk963_dn6, locals.var_chi_a__blk963_dn7, locals.var_chi_a__blk963_dn10, locals.var_chi_a__blk963_dn11, locals.var_chi_a__blk963_dn12, locals.var_chi_a__blk963_dn17, ) = (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17, );
            locals.var_chi_a__blk963_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31140_e45139: f64 = (locals.var_chi_b__blk962 - locals.var_chi_a__blk963);
            let assign31140_e45142: f64 = (0.0008 * 75.0);
            let assign31140_e45143: f64 = (assign31140_e45139 - assign31140_e45142);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign31140_e45143, (locals.var_chi_b__blk962_dn0 - locals.var_chi_a__blk963_dn0), (locals.var_chi_b__blk962_dn2 - locals.var_chi_a__blk963_dn2), (locals.var_chi_b__blk962_dn6 - locals.var_chi_a__blk963_dn6), (locals.var_chi_b__blk962_dn7 - locals.var_chi_a__blk963_dn7), (locals.var_chi_b__blk962_dn10 - locals.var_chi_a__blk963_dn10), (locals.var_chi_b__blk962_dn11 - locals.var_chi_a__blk963_dn11), (locals.var_chi_b__blk962_dn12 - locals.var_chi_a__blk963_dn12), (locals.var_chi_b__blk962_dn17 - locals.var_chi_a__blk963_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31150_e45159: f64 = (4.0 * locals.var_chi_b__blk962);
            let assign31150_e45162: f64 = (0.0008 * 75.0);
            let assign31150_e45163: f64 = (assign31150_e45159 * assign31150_e45162);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31150_e45163, ((4.0 * locals.var_chi_b__blk962_dn0) * assign31150_e45162), ((4.0 * locals.var_chi_b__blk962_dn2) * assign31150_e45162), ((4.0 * locals.var_chi_b__blk962_dn6) * assign31150_e45162), ((4.0 * locals.var_chi_b__blk962_dn7) * assign31150_e45162), ((4.0 * locals.var_chi_b__blk962_dn10) * assign31150_e45162), ((4.0 * locals.var_chi_b__blk962_dn11) * assign31150_e45162), ((4.0 * locals.var_chi_b__blk962_dn12) * assign31150_e45162), ((4.0 * locals.var_chi_b__blk962_dn17) * assign31150_e45162), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let (assign31160_e45183, assign31160_e45183_d_n0, assign31160_e45183_d_n2, assign31160_e45183_d_n6, assign31160_e45183_d_n7, assign31160_e45183_d_n10, assign31160_e45183_d_n11, assign31160_e45183_d_n12, assign31160_e45183_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign31160_e45182: f64 = (-locals.var_tmf2);
        (assign31160_e45182, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31160_e45183, assign31160_e45183_d_n0, assign31160_e45183_d_n2, assign31160_e45183_d_n6, assign31160_e45183_d_n7, assign31160_e45183_d_n10, assign31160_e45183_d_n11, assign31160_e45183_d_n12, assign31160_e45183_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31170_e45199: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign31170_e45201: f64 = (assign31170_e45199 + locals.var_tmf2);
            let assign31170_e45202: f64 = (assign31170_e45201).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign31170_e45202, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign31170_e45202)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign31170_e45202)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign31170_e45202)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign31170_e45202)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign31170_e45202)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign31170_e45202)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign31170_e45202)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign31170_e45202)), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31180_e45220: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign31180_e45221: f64 = (1.0 + assign31180_e45220);
            let assign31180_e45222: f64 = (0.5 * assign31180_e45221);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31180_e45222, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t1__blk898_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31190_e45241: f64 = (2.0 * 0.0008);
            let assign31190_e45243: f64 = (assign31190_e45241 * 75.0);
            let assign31190_e45244: f64 = (locals.var_tmf1 + assign31190_e45243);
            let assign31190_e45246: f64 = (assign31190_e45244 / locals.var_tmf2);
            let assign31190_e45247: f64 = (1.0 - assign31190_e45246);
            let assign31190_e45248: f64 = (0.5 * assign31190_e45247);
            (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17, ) = (assign31190_e45248, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign31190_e45244 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))), );
            locals.var_t2__blk899_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1009 != 0.0)) {
            let assign31200_e45266: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign31200_e45267: f64 = (0.5 * assign31200_e45266);
            let assign31200_e45268: f64 = (locals.var_chi_b__blk962 - assign31200_e45267);
            (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17, ) = (assign31200_e45268, (locals.var_chi_b__blk962_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b__blk962_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b__blk962_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b__blk962_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b__blk962_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b__blk962_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b__blk962_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b__blk962_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_chi__blk945_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) {
            let assign31210_e45282: f64 = (locals.var_chi__blk945 / locals.var_beta);
            let assign31210_e45284: f64 = (assign31210_e45282 - locals.var_vxbgmtcl__blk923);
            (locals.var_ps0ld__blk947, locals.var_ps0ld__blk947_dn0, locals.var_ps0ld__blk947_dn2, locals.var_ps0ld__blk947_dn6, locals.var_ps0ld__blk947_dn7, locals.var_ps0ld__blk947_dn10, locals.var_ps0ld__blk947_dn11, locals.var_ps0ld__blk947_dn12, locals.var_ps0ld__blk947_dn17, ) = (assign31210_e45284, ((locals.var_chi__blk945_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn0), ((locals.var_chi__blk945_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn2), ((locals.var_chi__blk945_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn6), ((locals.var_chi__blk945_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn7), ((((locals.var_chi__blk945_dn10 * locals.var_beta) - (locals.var_chi__blk945 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk923_dn10), ((locals.var_chi__blk945_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn11), ((locals.var_chi__blk945_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn12), ((locals.var_chi__blk945_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn17), );
            locals.var_ps0ld__blk947_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) {
            let assign31220_e45298: f64 = (locals.var_chi__blk945 - 1.0);
            let assign31220_e45300: f64 = (-locals.var_chi__blk945);
            let assign31220_e45301: f64 = (assign31220_e45300).exp();
            let assign31220_e45302: f64 = (assign31220_e45298 + assign31220_e45301);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31220_e45302, (locals.var_chi__blk945_dn0 + (assign31220_e45301 * (-locals.var_chi__blk945_dn0))), (locals.var_chi__blk945_dn2 + (assign31220_e45301 * (-locals.var_chi__blk945_dn2))), (locals.var_chi__blk945_dn6 + (assign31220_e45301 * (-locals.var_chi__blk945_dn6))), (locals.var_chi__blk945_dn7 + (assign31220_e45301 * (-locals.var_chi__blk945_dn7))), (locals.var_chi__blk945_dn10 + (assign31220_e45301 * (-locals.var_chi__blk945_dn10))), (locals.var_chi__blk945_dn11 + (assign31220_e45301 * (-locals.var_chi__blk945_dn11))), (locals.var_chi__blk945_dn12 + (assign31220_e45301 * (-locals.var_chi__blk945_dn12))), (locals.var_chi__blk945_dn17 + (assign31220_e45301 * (-locals.var_chi__blk945_dn17))), );
            locals.var_t1__blk898_rv = 0.0;
        }
        let assign31230_e45308: f64 = (10.0 * 2.220446049250313e-16);
        let assign31230_e45309: f64 = if locals.var_t1__blk898 < assign31230_e45308 { 1.0 } else { 0.0 };
        locals.var_guard1010 = assign31230_e45309;
        locals.var_guard1010_rv = 0.0;
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1010 != 0.0)) {
            let assign31240_e45323: f64 = (10.0 * 2.220446049250313e-16);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31240_e45323, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk898_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) {
            let assign31250_e45336: f64 = (locals.var_t1__blk898).sqrt();
            (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17, ) = (assign31250_e45336, (locals.var_t1__blk898_dn0 / (2.0 * assign31250_e45336)), (locals.var_t1__blk898_dn2 / (2.0 * assign31250_e45336)), (locals.var_t1__blk898_dn6 / (2.0 * assign31250_e45336)), (locals.var_t1__blk898_dn7 / (2.0 * assign31250_e45336)), (locals.var_t1__blk898_dn10 / (2.0 * assign31250_e45336)), (locals.var_t1__blk898_dn11 / (2.0 * assign31250_e45336)), (locals.var_t1__blk898_dn12 / (2.0 * assign31250_e45336)), (locals.var_t1__blk898_dn17 / (2.0 * assign31250_e45336)), );
            locals.var_t2__blk899_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) {
            let assign31260_e45350: f64 = (locals.var_cnst0over__blk930 * locals.var_t2__blk899);
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17, ) = (assign31260_e45350, ((locals.var_cnst0over__blk930_dn0 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn0)), ((locals.var_cnst0over__blk930_dn2 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn2)), ((locals.var_cnst0over__blk930_dn6 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn6)), ((locals.var_cnst0over__blk930_dn7 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn7)), ((locals.var_cnst0over__blk930_dn10 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn10)), ((locals.var_cnst0over__blk930_dn11 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn11)), ((locals.var_cnst0over__blk930_dn12 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn12)), ((locals.var_cnst0over__blk930_dn17 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn17)), );
            locals.var_qbuld_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) {
            let assign31270_e45365: f64 = (locals.var_vgpld__blk933 - locals.var_ps0ld__blk947);
            let assign31270_e45366: f64 = (locals.var_cox0__blk908 * assign31270_e45365);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17, ) = (assign31270_e45366, (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn0 - locals.var_ps0ld__blk947_dn0)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn2 - locals.var_ps0ld__blk947_dn2)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn6 - locals.var_ps0ld__blk947_dn6)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn7 - locals.var_ps0ld__blk947_dn7)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn10 - locals.var_ps0ld__blk947_dn10)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn11 - locals.var_ps0ld__blk947_dn11)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn12 - locals.var_ps0ld__blk947_dn12)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn17 - locals.var_ps0ld__blk947_dn17)), );
            locals.var_qsuld_rv = 0.0;
        }
        let assign31280_e45371: f64 = if p.p41 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1011 = assign31280_e45371;
        locals.var_guard1011_rv = 0.0;
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31290_e45385: f64 = (-locals.var_vxbgmtcl__blk923);
            let assign31290_e45386: f64 = (locals.var_beta * assign31290_e45385);
            let assign31290_e45387: f64 = (assign31290_e45386).exp();
            (locals.var_exp_bvbs__blk964, locals.var_exp_bvbs__blk964_dn0, locals.var_exp_bvbs__blk964_dn2, locals.var_exp_bvbs__blk964_dn6, locals.var_exp_bvbs__blk964_dn7, locals.var_exp_bvbs__blk964_dn10, locals.var_exp_bvbs__blk964_dn11, locals.var_exp_bvbs__blk964_dn12, locals.var_exp_bvbs__blk964_dn17, ) = (assign31290_e45387, (assign31290_e45387 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn0))), (assign31290_e45387 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn2))), (assign31290_e45387 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn6))), (assign31290_e45387 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn7))), (assign31290_e45387 * ((locals.var_beta_dn10 * assign31290_e45385) + (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn10)))), (assign31290_e45387 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn11))), (assign31290_e45387 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn12))), (assign31290_e45387 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn17))), );
            locals.var_exp_bvbs__blk964_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31300_e45403: f64 = (locals.var_nin / locals.var_mks_nover);
            (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17, ) = (assign31300_e45403, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover), );
            locals.var_t0__blk897_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31310_e45419: f64 = (locals.var_t0__blk897 * locals.var_t0__blk897);
            (locals.var_cnst1over__blk958, locals.var_cnst1over__blk958_dn0, locals.var_cnst1over__blk958_dn2, locals.var_cnst1over__blk958_dn6, locals.var_cnst1over__blk958_dn7, locals.var_cnst1over__blk958_dn10, locals.var_cnst1over__blk958_dn11, locals.var_cnst1over__blk958_dn12, locals.var_cnst1over__blk958_dn17, ) = (assign31310_e45419, ((locals.var_t0__blk897_dn0 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn0)), ((locals.var_t0__blk897_dn2 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn2)), ((locals.var_t0__blk897_dn6 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn6)), ((locals.var_t0__blk897_dn7 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn7)), ((locals.var_t0__blk897_dn10 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn10)), ((locals.var_t0__blk897_dn11 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn11)), ((locals.var_t0__blk897_dn12 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn12)), ((locals.var_t0__blk897_dn17 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn17)), );
            locals.var_cnst1over__blk958_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31320_e45435: f64 = (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964);
            (locals.var_cfs1__blk973, locals.var_cfs1__blk973_dn0, locals.var_cfs1__blk973_dn2, locals.var_cfs1__blk973_dn6, locals.var_cfs1__blk973_dn7, locals.var_cfs1__blk973_dn10, locals.var_cfs1__blk973_dn11, locals.var_cfs1__blk973_dn12, locals.var_cfs1__blk973_dn17, ) = (assign31320_e45435, ((locals.var_cnst1over__blk958_dn0 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn0)), ((locals.var_cnst1over__blk958_dn2 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn2)), ((locals.var_cnst1over__blk958_dn6 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn6)), ((locals.var_cnst1over__blk958_dn7 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn7)), ((locals.var_cnst1over__blk958_dn10 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn10)), ((locals.var_cnst1over__blk958_dn11 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn11)), ((locals.var_cnst1over__blk958_dn12 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn12)), ((locals.var_cnst1over__blk958_dn17 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn17)), );
            locals.var_cfs1__blk973_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            locals.var_flg_conv__blk920 = 0.0;
            locals.var_flg_conv__blk920_rv = 0.0;
            (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_fs01__blk967_rv = 0.0;
            (locals.var_fs02__blk971, locals.var_fs02__blk971_dn0, locals.var_fs02__blk971_dn2, locals.var_fs02__blk971_dn6, locals.var_fs02__blk971_dn7, locals.var_fs02__blk971_dn10, locals.var_fs02__blk971_dn11, locals.var_fs02__blk971_dn12, locals.var_fs02__blk971_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_fs02__blk971_rv = 0.0;
            locals.var_lp_s0 = 1.0;
            locals.var_lp_s0_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign31370_loop_guard: usize = 0;
        while {
            let assign31370_cond_e45508: f64 = (2.0 * 20.0);
            let assign31370_cond_e45510: f64 = (assign31370_cond_e45508 + 1.0);
            let assign31370_cond_e45512: f64 = if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_lp_s0 <= assign31370_cond_e45510)) { 1.0 } else { 0.0 };
            assign31370_cond_e45512 != 0.0
        } {
            assign31370_loop_guard += 1;
            assert!(assign31370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                (locals.var_fb__blk969, locals.var_fb__blk969_dn0, locals.var_fb__blk969_dn2, locals.var_fb__blk969_dn6, locals.var_fb__blk969_dn7, locals.var_fb__blk969_dn10, locals.var_fb__blk969_dn11, locals.var_fb__blk969_dn12, locals.var_fb__blk969_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
                locals.var_fb__blk969_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                let assign31370_body1_e45541: f64 = (locals.var_ps0ld__blk947 + locals.var_vxbgmtcl__blk923);
                let assign31370_body1_e45542: f64 = (locals.var_beta * assign31370_body1_e45541);
                (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17, ) = (assign31370_body1_e45542, (locals.var_beta * (locals.var_ps0ld__blk947_dn0 + locals.var_vxbgmtcl__blk923_dn0)), (locals.var_beta * (locals.var_ps0ld__blk947_dn2 + locals.var_vxbgmtcl__blk923_dn2)), (locals.var_beta * (locals.var_ps0ld__blk947_dn6 + locals.var_vxbgmtcl__blk923_dn6)), (locals.var_beta * (locals.var_ps0ld__blk947_dn7 + locals.var_vxbgmtcl__blk923_dn7)), ((locals.var_beta_dn10 * assign31370_body1_e45541) + (locals.var_beta * (locals.var_ps0ld__blk947_dn10 + locals.var_vxbgmtcl__blk923_dn10))), (locals.var_beta * (locals.var_ps0ld__blk947_dn11 + locals.var_vxbgmtcl__blk923_dn11)), (locals.var_beta * (locals.var_ps0ld__blk947_dn12 + locals.var_vxbgmtcl__blk923_dn12)), (locals.var_beta * (locals.var_ps0ld__blk947_dn17 + locals.var_vxbgmtcl__blk923_dn17)), );
                locals.var_chi__blk945_rv = 0.0;
            }
            let assign31370_body2_e45547: f64 = if locals.var_chi__blk945 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1012 = assign31370_body2_e45547;
            locals.var_guard1012_rv = 0.0;
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body3_e45563: f64 = (locals.var_chi__blk945 * locals.var_chi__blk945);
                let assign31370_body3_e45565: f64 = (assign31370_body3_e45563 * locals.var_chi__blk945);
                let assign31370_body3_e45569: f64 = (-0.07053654284009761);
                let assign31370_body3_e45572: f64 = (locals.var_chi__blk945 * 0.006115288895133179);
                let assign31370_body3_e45573: f64 = (assign31370_body3_e45569 + assign31370_body3_e45572);
                let assign31370_body3_e45574: f64 = (locals.var_chi__blk945 * assign31370_body3_e45573);
                let assign31370_body3_e45575: f64 = (0.29693154855771 + assign31370_body3_e45574);
                let assign31370_body3_e45576: f64 = (assign31370_body3_e45565 * assign31370_body3_e45575);
                (locals.var_fi__blk965, locals.var_fi__blk965_dn0, locals.var_fi__blk965_dn2, locals.var_fi__blk965_dn6, locals.var_fi__blk965_dn7, locals.var_fi__blk965_dn10, locals.var_fi__blk965_dn11, locals.var_fi__blk965_dn12, locals.var_fi__blk965_dn17, ) = (assign31370_body3_e45576, ((((((locals.var_chi__blk945_dn0 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn0)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn0)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn0 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn2 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn2)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn2)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn2 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn6 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn6)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn6)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn6 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn7 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn7)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn7)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn7 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn10 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn10)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn10)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn10 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn11 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn11)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn11)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn11 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn12 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn12)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn12)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn12 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn17 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn17)) * locals.var_chi__blk945) + (assign31370_body3_e45563 * locals.var_chi__blk945_dn17)) * assign31370_body3_e45575) + (assign31370_body3_e45565 * ((locals.var_chi__blk945_dn17 * assign31370_body3_e45573) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn17 * 0.006115288895133179))))), );
                locals.var_fi__blk965_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body4_e45594: f64 = (locals.var_chi__blk945 * locals.var_chi__blk945);
                let assign31370_body4_e45597: f64 = (3.0 * 0.29693154855771);
                let assign31370_body4_e45601: f64 = (-0.07053654284009761);
                let assign31370_body4_e45602: f64 = (4.0 * assign31370_body4_e45601);
                let assign31370_body4_e45605: f64 = (locals.var_chi__blk945 * 5.0);
                let assign31370_body4_e45607: f64 = (assign31370_body4_e45605 * 0.006115288895133179);
                let assign31370_body4_e45608: f64 = (assign31370_body4_e45602 + assign31370_body4_e45607);
                let assign31370_body4_e45609: f64 = (locals.var_chi__blk945 * assign31370_body4_e45608);
                let assign31370_body4_e45610: f64 = (assign31370_body4_e45597 + assign31370_body4_e45609);
                let assign31370_body4_e45611: f64 = (assign31370_body4_e45594 * assign31370_body4_e45610);
                (locals.var_fi_dchi__blk966, locals.var_fi_dchi__blk966_dn0, locals.var_fi_dchi__blk966_dn2, locals.var_fi_dchi__blk966_dn6, locals.var_fi_dchi__blk966_dn7, locals.var_fi_dchi__blk966_dn10, locals.var_fi_dchi__blk966_dn11, locals.var_fi_dchi__blk966_dn12, locals.var_fi_dchi__blk966_dn17, ) = (assign31370_body4_e45611, ((((locals.var_chi__blk945_dn0 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn0)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn0 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn2 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn2)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn2 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn6 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn6)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn6 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn7 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn7)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn7 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn10 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn10)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn10 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn11 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn11)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn11 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn12 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn12)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn12 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn17 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn17)) * assign31370_body4_e45610) + (assign31370_body4_e45594 * ((locals.var_chi__blk945_dn17 * assign31370_body4_e45608) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * 5.0) * 0.006115288895133179))))), );
                locals.var_fi_dchi__blk966_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body5_e45629: f64 = (locals.var_cfs1__blk973 * locals.var_fi__blk965);
                let assign31370_body5_e45631: f64 = (assign31370_body5_e45629 * locals.var_fi__blk965);
                (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17, ) = (assign31370_body5_e45631, ((((locals.var_cfs1__blk973_dn0 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn0)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn0)), ((((locals.var_cfs1__blk973_dn2 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn2)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn2)), ((((locals.var_cfs1__blk973_dn6 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn6)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn6)), ((((locals.var_cfs1__blk973_dn7 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn7)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn7)), ((((locals.var_cfs1__blk973_dn10 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn10)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn10)), ((((locals.var_cfs1__blk973_dn11 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn11)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn11)), ((((locals.var_cfs1__blk973_dn12 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn12)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn12)), ((((locals.var_cfs1__blk973_dn17 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn17)) * locals.var_fi__blk965) + (assign31370_body5_e45629 * locals.var_fi__blk965_dn17)), );
                locals.var_fs01__blk967_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body6_e45649: f64 = (locals.var_cfs1__blk973 * locals.var_beta);
                let assign31370_body6_e45651: f64 = (assign31370_body6_e45649 * 2.0);
                let assign31370_body6_e45653: f64 = (assign31370_body6_e45651 * locals.var_fi__blk965);
                let assign31370_body6_e45655: f64 = (assign31370_body6_e45653 * locals.var_fi_dchi__blk966);
                (locals.var_fs01_dps0__blk968, locals.var_fs01_dps0__blk968_dn0, locals.var_fs01_dps0__blk968_dn2, locals.var_fs01_dps0__blk968_dn6, locals.var_fs01_dps0__blk968_dn7, locals.var_fs01_dps0__blk968_dn10, locals.var_fs01_dps0__blk968_dn11, locals.var_fs01_dps0__blk968_dn12, locals.var_fs01_dps0__blk968_dn17, ) = (assign31370_body6_e45655, ((((((locals.var_cfs1__blk973_dn0 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn0)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn0)), ((((((locals.var_cfs1__blk973_dn2 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn2)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn2)), ((((((locals.var_cfs1__blk973_dn6 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn6)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn6)), ((((((locals.var_cfs1__blk973_dn7 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn7)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn7)), (((((((locals.var_cfs1__blk973_dn10 * locals.var_beta) + (locals.var_cfs1__blk973 * locals.var_beta_dn10)) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn10)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn10)), ((((((locals.var_cfs1__blk973_dn11 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn11)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn11)), ((((((locals.var_cfs1__blk973_dn12 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn12)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn12)), ((((((locals.var_cfs1__blk973_dn17 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign31370_body6_e45651 * locals.var_fi__blk965_dn17)) * locals.var_fi_dchi__blk966) + (assign31370_body6_e45653 * locals.var_fi_dchi__blk966_dn17)), );
                locals.var_fs01_dps0__blk968_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body7_e45675: f64 = (-0.117851130197758);
                let assign31370_body7_e45680: f64 = (-0.00163730162779191);
                let assign31370_body7_e45683: f64 = (locals.var_chi__blk945 * 6.36964918866352e-5);
                let assign31370_body7_e45684: f64 = (assign31370_body7_e45680 + assign31370_body7_e45683);
                let assign31370_body7_e45685: f64 = (locals.var_chi__blk945 * assign31370_body7_e45684);
                let assign31370_body7_e45686: f64 = (0.0178800506338833 + assign31370_body7_e45685);
                let assign31370_body7_e45687: f64 = (locals.var_chi__blk945 * assign31370_body7_e45686);
                let assign31370_body7_e45688: f64 = (assign31370_body7_e45675 + assign31370_body7_e45687);
                let assign31370_body7_e45689: f64 = (locals.var_chi__blk945 * assign31370_body7_e45688);
                let assign31370_body7_e45690: f64 = (0.707106781186548 + assign31370_body7_e45689);
                let assign31370_body7_e45691: f64 = (locals.var_chi__blk945 * assign31370_body7_e45690);
                (locals.var_fb__blk969, locals.var_fb__blk969_dn0, locals.var_fb__blk969_dn2, locals.var_fb__blk969_dn6, locals.var_fb__blk969_dn7, locals.var_fb__blk969_dn10, locals.var_fb__blk969_dn11, locals.var_fb__blk969_dn12, locals.var_fb__blk969_dn17, ) = (assign31370_body7_e45691, ((locals.var_chi__blk945_dn0 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn2 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn6 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn7 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn10 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn11 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn12 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn17 * assign31370_body7_e45690) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign31370_body7_e45688) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign31370_body7_e45686) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign31370_body7_e45684) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn17 * 6.36964918866352e-5))))))))), );
                locals.var_fb__blk969_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body8_e45711: f64 = (-0.117851130197758);
                let assign31370_body8_e45712: f64 = (2.0 * assign31370_body8_e45711);
                let assign31370_body8_e45716: f64 = (3.0 * 0.0178800506338833);
                let assign31370_body8_e45720: f64 = (-0.00163730162779191);
                let assign31370_body8_e45721: f64 = (4.0 * assign31370_body8_e45720);
                let assign31370_body8_e45724: f64 = (locals.var_chi__blk945 * 5.0);
                let assign31370_body8_e45726: f64 = (assign31370_body8_e45724 * 6.36964918866352e-5);
                let assign31370_body8_e45727: f64 = (assign31370_body8_e45721 + assign31370_body8_e45726);
                let assign31370_body8_e45728: f64 = (locals.var_chi__blk945 * assign31370_body8_e45727);
                let assign31370_body8_e45729: f64 = (assign31370_body8_e45716 + assign31370_body8_e45728);
                let assign31370_body8_e45730: f64 = (locals.var_chi__blk945 * assign31370_body8_e45729);
                let assign31370_body8_e45731: f64 = (assign31370_body8_e45712 + assign31370_body8_e45730);
                let assign31370_body8_e45732: f64 = (locals.var_chi__blk945 * assign31370_body8_e45731);
                let assign31370_body8_e45733: f64 = (0.707106781186548 + assign31370_body8_e45732);
                (locals.var_fb_dchi__blk970, locals.var_fb_dchi__blk970_dn0, locals.var_fb_dchi__blk970_dn2, locals.var_fb_dchi__blk970_dn6, locals.var_fb_dchi__blk970_dn7, locals.var_fb_dchi__blk970_dn10, locals.var_fb_dchi__blk970_dn11, locals.var_fb_dchi__blk970_dn12, locals.var_fb_dchi__blk970_dn17, ) = (assign31370_body8_e45733, ((locals.var_chi__blk945_dn0 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn2 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn6 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn7 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn10 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn11 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn12 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn17 * assign31370_body8_e45731) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign31370_body8_e45729) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign31370_body8_e45727) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * 5.0) * 6.36964918866352e-5))))))), );
                locals.var_fb_dchi__blk970_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body9_e45751: f64 = (locals.var_fb__blk969 * locals.var_fb__blk969);
                let assign31370_body9_e45753: f64 = (assign31370_body9_e45751 + locals.var_fs01__blk967);
                let assign31370_body9_e45755: f64 = (assign31370_body9_e45753 + 1e-50);
                let assign31370_body9_e45756: f64 = (assign31370_body9_e45755).sqrt();
                (locals.var_fs02__blk971, locals.var_fs02__blk971_dn0, locals.var_fs02__blk971_dn2, locals.var_fs02__blk971_dn6, locals.var_fs02__blk971_dn7, locals.var_fs02__blk971_dn10, locals.var_fs02__blk971_dn11, locals.var_fs02__blk971_dn12, locals.var_fs02__blk971_dn17, ) = (assign31370_body9_e45756, ((((locals.var_fb__blk969_dn0 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn0)) + locals.var_fs01__blk967_dn0) / (2.0 * assign31370_body9_e45756)), ((((locals.var_fb__blk969_dn2 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn2)) + locals.var_fs01__blk967_dn2) / (2.0 * assign31370_body9_e45756)), ((((locals.var_fb__blk969_dn6 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn6)) + locals.var_fs01__blk967_dn6) / (2.0 * assign31370_body9_e45756)), ((((locals.var_fb__blk969_dn7 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn7)) + locals.var_fs01__blk967_dn7) / (2.0 * assign31370_body9_e45756)), ((((locals.var_fb__blk969_dn10 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn10)) + locals.var_fs01__blk967_dn10) / (2.0 * assign31370_body9_e45756)), ((((locals.var_fb__blk969_dn11 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn11)) + locals.var_fs01__blk967_dn11) / (2.0 * assign31370_body9_e45756)), ((((locals.var_fb__blk969_dn12 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn12)) + locals.var_fs01__blk967_dn12) / (2.0 * assign31370_body9_e45756)), ((((locals.var_fb__blk969_dn17 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn17)) + locals.var_fs01__blk967_dn17) / (2.0 * assign31370_body9_e45756)), );
                locals.var_fs02__blk971_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
                let assign31370_body10_e45774: f64 = (locals.var_beta * locals.var_fb_dchi__blk970);
                let assign31370_body10_e45776: f64 = (assign31370_body10_e45774 * 2.0);
                let assign31370_body10_e45778: f64 = (assign31370_body10_e45776 * locals.var_fb__blk969);
                let assign31370_body10_e45780: f64 = (assign31370_body10_e45778 + locals.var_fs01_dps0__blk968);
                let assign31370_body10_e45783: f64 = (locals.var_fs02__blk971 + locals.var_fs02__blk971);
                let assign31370_body10_e45784: f64 = (assign31370_body10_e45780 / assign31370_body10_e45783);
                (locals.var_fs02_dps0__blk972, locals.var_fs02_dps0__blk972_dn0, locals.var_fs02_dps0__blk972_dn2, locals.var_fs02_dps0__blk972_dn6, locals.var_fs02_dps0__blk972_dn7, locals.var_fs02_dps0__blk972_dn10, locals.var_fs02_dps0__blk972_dn11, locals.var_fs02_dps0__blk972_dn12, locals.var_fs02_dps0__blk972_dn17, ) = (assign31370_body10_e45784, ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn0) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn0)) + locals.var_fs01_dps0__blk968_dn0) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn0 + locals.var_fs02__blk971_dn0))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn2) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn2)) + locals.var_fs01_dps0__blk968_dn2) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn2 + locals.var_fs02__blk971_dn2))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn6) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn6)) + locals.var_fs01_dps0__blk968_dn6) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn6 + locals.var_fs02__blk971_dn6))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn7) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn7)) + locals.var_fs01_dps0__blk968_dn7) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn7 + locals.var_fs02__blk971_dn7))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi__blk970) + (locals.var_beta * locals.var_fb_dchi__blk970_dn10)) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn10)) + locals.var_fs01_dps0__blk968_dn10) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn10 + locals.var_fs02__blk971_dn10))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn11) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn11)) + locals.var_fs01_dps0__blk968_dn11) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn11 + locals.var_fs02__blk971_dn11))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn12) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn12)) + locals.var_fs01_dps0__blk968_dn12) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn12 + locals.var_fs02__blk971_dn12))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn17) * 2.0) * locals.var_fb__blk969) + (assign31370_body10_e45776 * locals.var_fb__blk969_dn17)) + locals.var_fs01_dps0__blk968_dn17) * assign31370_body10_e45783) - (assign31370_body10_e45780 * (locals.var_fs02__blk971_dn17 + locals.var_fs02__blk971_dn17))) / (assign31370_body10_e45783 * assign31370_body10_e45783)), );
                locals.var_fs02_dps0__blk972_rv = 0.0;
            }
            let assign31370_body11_e45789: f64 = if locals.var_chi__blk945 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard1013 = assign31370_body11_e45789;
            locals.var_guard1013_rv = 0.0;
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 != 0.0)) {
                let assign31370_body12_e45807: f64 = (locals.var_chi__blk945).exp();
                (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17, ) = (assign31370_body12_e45807, (assign31370_body12_e45807 * locals.var_chi__blk945_dn0), (assign31370_body12_e45807 * locals.var_chi__blk945_dn2), (assign31370_body12_e45807 * locals.var_chi__blk945_dn6), (assign31370_body12_e45807 * locals.var_chi__blk945_dn7), (assign31370_body12_e45807 * locals.var_chi__blk945_dn10), (assign31370_body12_e45807 * locals.var_chi__blk945_dn11), (assign31370_body12_e45807 * locals.var_chi__blk945_dn12), (assign31370_body12_e45807 * locals.var_chi__blk945_dn17), );
                locals.var_exp_chi_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 != 0.0)) {
                let assign31370_body13_e45829: f64 = (locals.var_exp_chi - 1.0);
                let assign31370_body13_e45830: f64 = (locals.var_cfs1__blk973 * assign31370_body13_e45829);
                (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17, ) = (assign31370_body13_e45830, ((locals.var_cfs1__blk973_dn0 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk973_dn2 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk973_dn6 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk973_dn7 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk973_dn10 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk973_dn11 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk973_dn12 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk973_dn17 * assign31370_body13_e45829) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn17)), );
                locals.var_fs01__blk967_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 != 0.0)) {
                let assign31370_body14_e45851: f64 = (locals.var_cfs1__blk973 * locals.var_beta);
                let assign31370_body14_e45853: f64 = (assign31370_body14_e45851 * locals.var_exp_chi);
                (locals.var_fs01_dps0__blk968, locals.var_fs01_dps0__blk968_dn0, locals.var_fs01_dps0__blk968_dn2, locals.var_fs01_dps0__blk968_dn6, locals.var_fs01_dps0__blk968_dn7, locals.var_fs01_dps0__blk968_dn10, locals.var_fs01_dps0__blk968_dn11, locals.var_fs01_dps0__blk968_dn12, locals.var_fs01_dps0__blk968_dn17, ) = (assign31370_body14_e45853, (((locals.var_cfs1__blk973_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk973_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk973_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk973_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk973_dn10 * locals.var_beta) + (locals.var_cfs1__blk973 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk973_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk973_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk973_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign31370_body14_e45851 * locals.var_exp_chi_dn17)), );
                locals.var_fs01_dps0__blk968_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 == 0.0)) {
                let assign31370_body15_e45875: f64 = (locals.var_beta * locals.var_ps0ld__blk947);
                let assign31370_body15_e45876: f64 = (assign31370_body15_e45875).exp();
                (locals.var_exp_bps0__blk974, locals.var_exp_bps0__blk974_dn0, locals.var_exp_bps0__blk974_dn2, locals.var_exp_bps0__blk974_dn6, locals.var_exp_bps0__blk974_dn7, locals.var_exp_bps0__blk974_dn10, locals.var_exp_bps0__blk974_dn11, locals.var_exp_bps0__blk974_dn12, locals.var_exp_bps0__blk974_dn17, ) = (assign31370_body15_e45876, (assign31370_body15_e45876 * (locals.var_beta * locals.var_ps0ld__blk947_dn0)), (assign31370_body15_e45876 * (locals.var_beta * locals.var_ps0ld__blk947_dn2)), (assign31370_body15_e45876 * (locals.var_beta * locals.var_ps0ld__blk947_dn6)), (assign31370_body15_e45876 * (locals.var_beta * locals.var_ps0ld__blk947_dn7)), (assign31370_body15_e45876 * ((locals.var_beta_dn10 * locals.var_ps0ld__blk947) + (locals.var_beta * locals.var_ps0ld__blk947_dn10))), (assign31370_body15_e45876 * (locals.var_beta * locals.var_ps0ld__blk947_dn11)), (assign31370_body15_e45876 * (locals.var_beta * locals.var_ps0ld__blk947_dn12)), (assign31370_body15_e45876 * (locals.var_beta * locals.var_ps0ld__blk947_dn17)), );
                locals.var_exp_bps0__blk974_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 == 0.0)) {
                let assign31370_body16_e45899: f64 = (locals.var_exp_bps0__blk974 - locals.var_exp_bvbs__blk964);
                let assign31370_body16_e45900: f64 = (locals.var_cnst1over__blk958 * assign31370_body16_e45899);
                (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17, ) = (assign31370_body16_e45900, ((locals.var_cnst1over__blk958_dn0 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn0 - locals.var_exp_bvbs__blk964_dn0))), ((locals.var_cnst1over__blk958_dn2 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn2 - locals.var_exp_bvbs__blk964_dn2))), ((locals.var_cnst1over__blk958_dn6 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn6 - locals.var_exp_bvbs__blk964_dn6))), ((locals.var_cnst1over__blk958_dn7 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn7 - locals.var_exp_bvbs__blk964_dn7))), ((locals.var_cnst1over__blk958_dn10 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn10 - locals.var_exp_bvbs__blk964_dn10))), ((locals.var_cnst1over__blk958_dn11 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn11 - locals.var_exp_bvbs__blk964_dn11))), ((locals.var_cnst1over__blk958_dn12 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn12 - locals.var_exp_bvbs__blk964_dn12))), ((locals.var_cnst1over__blk958_dn17 * assign31370_body16_e45899) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn17 - locals.var_exp_bvbs__blk964_dn17))), );
                locals.var_fs01__blk967_rv = 0.0;
            }
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) && (locals.var_guard1013 == 0.0)) {
                let assign31370_body17_e45922: f64 = (locals.var_cnst1over__blk958 * locals.var_beta);
                let assign31370_body17_e45924: f64 = (assign31370_body17_e45922 * locals.var_exp_bps0__blk974);
                (locals.var_fs01_dps0__blk968, locals.var_fs01_dps0__blk968_dn0, locals.var_fs01_dps0__blk968_dn2, locals.var_fs01_dps0__blk968_dn6, locals.var_fs01_dps0__blk968_dn7, locals.var_fs01_dps0__blk968_dn10, locals.var_fs01_dps0__blk968_dn11, locals.var_fs01_dps0__blk968_dn12, locals.var_fs01_dps0__blk968_dn17, ) = (assign31370_body17_e45924, (((locals.var_cnst1over__blk958_dn0 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn0)), (((locals.var_cnst1over__blk958_dn2 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn2)), (((locals.var_cnst1over__blk958_dn6 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn6)), (((locals.var_cnst1over__blk958_dn7 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn7)), ((((locals.var_cnst1over__blk958_dn10 * locals.var_beta) + (locals.var_cnst1over__blk958 * locals.var_beta_dn10)) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn10)), (((locals.var_cnst1over__blk958_dn11 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn11)), (((locals.var_cnst1over__blk958_dn12 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn12)), (((locals.var_cnst1over__blk958_dn17 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign31370_body17_e45922 * locals.var_exp_bps0__blk974_dn17)), );
                locals.var_fs01_dps0__blk968_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) {
                let assign31370_body18_e45943: f64 = (locals.var_chi__blk945 - 1.0);
                let assign31370_body18_e45945: f64 = (assign31370_body18_e45943 + locals.var_fs01__blk967);
                let assign31370_body18_e45946: f64 = (assign31370_body18_e45945).sqrt();
                (locals.var_fs02__blk971, locals.var_fs02__blk971_dn0, locals.var_fs02__blk971_dn2, locals.var_fs02__blk971_dn6, locals.var_fs02__blk971_dn7, locals.var_fs02__blk971_dn10, locals.var_fs02__blk971_dn11, locals.var_fs02__blk971_dn12, locals.var_fs02__blk971_dn17, ) = (assign31370_body18_e45946, ((locals.var_chi__blk945_dn0 + locals.var_fs01__blk967_dn0) / (2.0 * assign31370_body18_e45946)), ((locals.var_chi__blk945_dn2 + locals.var_fs01__blk967_dn2) / (2.0 * assign31370_body18_e45946)), ((locals.var_chi__blk945_dn6 + locals.var_fs01__blk967_dn6) / (2.0 * assign31370_body18_e45946)), ((locals.var_chi__blk945_dn7 + locals.var_fs01__blk967_dn7) / (2.0 * assign31370_body18_e45946)), ((locals.var_chi__blk945_dn10 + locals.var_fs01__blk967_dn10) / (2.0 * assign31370_body18_e45946)), ((locals.var_chi__blk945_dn11 + locals.var_fs01__blk967_dn11) / (2.0 * assign31370_body18_e45946)), ((locals.var_chi__blk945_dn12 + locals.var_fs01__blk967_dn12) / (2.0 * assign31370_body18_e45946)), ((locals.var_chi__blk945_dn17 + locals.var_fs01__blk967_dn17) / (2.0 * assign31370_body18_e45946)), );
                locals.var_fs02__blk971_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) {
                let assign31370_body19_e45965: f64 = (locals.var_beta + locals.var_fs01_dps0__blk968);
                let assign31370_body19_e45967: f64 = (assign31370_body19_e45965 / locals.var_fs02__blk971);
                let assign31370_body19_e45969: f64 = (assign31370_body19_e45967 * 0.5);
                (locals.var_fs02_dps0__blk972, locals.var_fs02_dps0__blk972_dn0, locals.var_fs02_dps0__blk972_dn2, locals.var_fs02_dps0__blk972_dn6, locals.var_fs02_dps0__blk972_dn7, locals.var_fs02_dps0__blk972_dn10, locals.var_fs02_dps0__blk972_dn11, locals.var_fs02_dps0__blk972_dn12, locals.var_fs02_dps0__blk972_dn17, ) = (assign31370_body19_e45969, ((((locals.var_fs01_dps0__blk968_dn0 * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn0)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn2 * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn2)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn6 * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn6)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn7 * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn7)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk968_dn10) * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn10)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn11 * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn11)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn12 * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn12)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn17 * locals.var_fs02__blk971) - (assign31370_body19_e45965 * locals.var_fs02__blk971_dn17)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), );
                locals.var_fs02_dps0__blk972_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                let assign31370_body20_e45985: f64 = (locals.var_vgpld__blk933 - locals.var_ps0ld__blk947);
                let assign31370_body20_e45988: f64 = (locals.var_fac1__blk931 * locals.var_fs02__blk971);
                let assign31370_body20_e45989: f64 = (assign31370_body20_e45985 - assign31370_body20_e45988);
                (locals.var_fs0__blk975, locals.var_fs0__blk975_dn0, locals.var_fs0__blk975_dn2, locals.var_fs0__blk975_dn6, locals.var_fs0__blk975_dn7, locals.var_fs0__blk975_dn10, locals.var_fs0__blk975_dn11, locals.var_fs0__blk975_dn12, locals.var_fs0__blk975_dn17, ) = (assign31370_body20_e45989, ((locals.var_vgpld__blk933_dn0 - locals.var_ps0ld__blk947_dn0) - ((locals.var_fac1__blk931_dn0 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn0))), ((locals.var_vgpld__blk933_dn2 - locals.var_ps0ld__blk947_dn2) - ((locals.var_fac1__blk931_dn2 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn2))), ((locals.var_vgpld__blk933_dn6 - locals.var_ps0ld__blk947_dn6) - ((locals.var_fac1__blk931_dn6 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn6))), ((locals.var_vgpld__blk933_dn7 - locals.var_ps0ld__blk947_dn7) - ((locals.var_fac1__blk931_dn7 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn7))), ((locals.var_vgpld__blk933_dn10 - locals.var_ps0ld__blk947_dn10) - ((locals.var_fac1__blk931_dn10 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn10))), ((locals.var_vgpld__blk933_dn11 - locals.var_ps0ld__blk947_dn11) - ((locals.var_fac1__blk931_dn11 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn11))), ((locals.var_vgpld__blk933_dn12 - locals.var_ps0ld__blk947_dn12) - ((locals.var_fac1__blk931_dn12 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn12))), ((locals.var_vgpld__blk933_dn17 - locals.var_ps0ld__blk947_dn17) - ((locals.var_fac1__blk931_dn17 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn17))), );
                locals.var_fs0__blk975_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                let assign31370_body21_e46004: f64 = (-1.0);
                let assign31370_body21_e46007: f64 = (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972);
                let assign31370_body21_e46008: f64 = (assign31370_body21_e46004 - assign31370_body21_e46007);
                (locals.var_fs0_dps0__blk976, locals.var_fs0_dps0__blk976_dn0, locals.var_fs0_dps0__blk976_dn2, locals.var_fs0_dps0__blk976_dn6, locals.var_fs0_dps0__blk976_dn7, locals.var_fs0_dps0__blk976_dn10, locals.var_fs0_dps0__blk976_dn11, locals.var_fs0_dps0__blk976_dn12, locals.var_fs0_dps0__blk976_dn17, ) = (assign31370_body21_e46008, (-((locals.var_fac1__blk931_dn0 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn0))), (-((locals.var_fac1__blk931_dn2 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn2))), (-((locals.var_fac1__blk931_dn6 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn6))), (-((locals.var_fac1__blk931_dn7 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn7))), (-((locals.var_fac1__blk931_dn10 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn10))), (-((locals.var_fac1__blk931_dn11 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn11))), (-((locals.var_fac1__blk931_dn12 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn12))), (-((locals.var_fac1__blk931_dn17 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn17))), );
                locals.var_fs0_dps0__blk976_rv = 0.0;
            }
            let assign31370_body22_e46013: f64 = if locals.var_flg_conv__blk920 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1014 = assign31370_body22_e46013;
            locals.var_guard1014_rv = 0.0;
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1014 != 0.0)) {
                let assign31370_body23_e46029: f64 = (2.0 * 20.0);
                let assign31370_body23_e46031: f64 = (assign31370_body23_e46029 + 1.0);
                locals.var_lp_s0 = assign31370_body23_e46031;
                locals.var_lp_s0_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1014 == 0.0)) {
                let assign31370_body24_e46049: f64 = (-locals.var_fs0__blk975);
                let assign31370_body24_e46051: f64 = (assign31370_body24_e46049 / locals.var_fs0_dps0__blk976);
                (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17, ) = (assign31370_body24_e46051, ((((-locals.var_fs0__blk975_dn0) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn0)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn2) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn2)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn6) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn6)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn7) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn7)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn10) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn10)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn11) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn11)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn12) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn12)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn17) * locals.var_fs0_dps0__blk976) - (assign31370_body24_e46049 * locals.var_fs0_dps0__blk976_dn17)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), );
                locals.var_dps0_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1014 == 0.0)) {
                let assign31370_body25_e46070: f64 = (0.5 * 0.1);
                let assign31370_body25_e46074: f64 = (locals.var_ps0ld__blk947).abs();
                let (assign31370_body25_e46079, assign31370_body25_e46079_d_n0, assign31370_body25_e46079_d_n2, assign31370_body25_e46079_d_n6, assign31370_body25_e46079_d_n7, assign31370_body25_e46079_d_n10, assign31370_body25_e46079_d_n11, assign31370_body25_e46079_d_n12, assign31370_body25_e46079_d_n17,) = {
    if (1.0 >= assign31370_body25_e46074) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign31370_body25_e46078: f64 = (locals.var_ps0ld__blk947).abs();
        (assign31370_body25_e46078, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn0 } else { (-locals.var_ps0ld__blk947_dn0) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn2 } else { (-locals.var_ps0ld__blk947_dn2) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn6 } else { (-locals.var_ps0ld__blk947_dn6) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn7 } else { (-locals.var_ps0ld__blk947_dn7) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn10 } else { (-locals.var_ps0ld__blk947_dn10) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn11 } else { (-locals.var_ps0ld__blk947_dn11) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn12 } else { (-locals.var_ps0ld__blk947_dn12) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn17 } else { (-locals.var_ps0ld__blk947_dn17) },)
    }
};
                let assign31370_body25_e46080: f64 = (1.0 + assign31370_body25_e46079);
                let assign31370_body25_e46081: f64 = (assign31370_body25_e46070 * assign31370_body25_e46080);
                (locals.var_dplim__blk977, locals.var_dplim__blk977_dn0, locals.var_dplim__blk977_dn2, locals.var_dplim__blk977_dn6, locals.var_dplim__blk977_dn7, locals.var_dplim__blk977_dn10, locals.var_dplim__blk977_dn11, locals.var_dplim__blk977_dn12, locals.var_dplim__blk977_dn17, ) = (assign31370_body25_e46081, (assign31370_body25_e46070 * assign31370_body25_e46079_d_n0), (assign31370_body25_e46070 * assign31370_body25_e46079_d_n2), (assign31370_body25_e46070 * assign31370_body25_e46079_d_n6), (assign31370_body25_e46070 * assign31370_body25_e46079_d_n7), (assign31370_body25_e46070 * assign31370_body25_e46079_d_n10), (assign31370_body25_e46070 * assign31370_body25_e46079_d_n11), (assign31370_body25_e46070 * assign31370_body25_e46079_d_n12), (assign31370_body25_e46070 * assign31370_body25_e46079_d_n17), );
                locals.var_dplim__blk977_rv = 0.0;
            }
            let assign31370_body26_e46085: f64 = (locals.var_dps0).abs();
            let assign31370_body26_e46087: f64 = if assign31370_body26_e46085 > locals.var_dplim__blk977 { 1.0 } else { 0.0 };
            locals.var_guard1015 = assign31370_body26_e46087;
            locals.var_guard1015_rv = 0.0;
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 != 0.0)) {
                let (assign31370_body27_e46111,) = {
    if (locals.var_dps0 >= 0.0) {
        (1.0,)
    } else {
        let assign31370_body27_e46110: f64 = (-1.0);
        (assign31370_body27_e46110,)
    }
};
                let assign31370_body27_e46112: f64 = (locals.var_dplim__blk977 * assign31370_body27_e46111);
                (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17, ) = (assign31370_body27_e46112, (locals.var_dplim__blk977_dn0 * assign31370_body27_e46111), (locals.var_dplim__blk977_dn2 * assign31370_body27_e46111), (locals.var_dplim__blk977_dn6 * assign31370_body27_e46111), (locals.var_dplim__blk977_dn7 * assign31370_body27_e46111), (locals.var_dplim__blk977_dn10 * assign31370_body27_e46111), (locals.var_dplim__blk977_dn11 * assign31370_body27_e46111), (locals.var_dplim__blk977_dn12 * assign31370_body27_e46111), (locals.var_dplim__blk977_dn17 * assign31370_body27_e46111), );
                locals.var_dps0_rv = 0.0;
            }
            if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1014 == 0.0)) {
                let assign31370_body28_e46131: f64 = (locals.var_ps0ld__blk947 + locals.var_dps0);
                (locals.var_ps0ld__blk947, locals.var_ps0ld__blk947_dn0, locals.var_ps0ld__blk947_dn2, locals.var_ps0ld__blk947_dn6, locals.var_ps0ld__blk947_dn7, locals.var_ps0ld__blk947_dn10, locals.var_ps0ld__blk947_dn11, locals.var_ps0ld__blk947_dn12, locals.var_ps0ld__blk947_dn17, ) = (assign31370_body28_e46131, (locals.var_ps0ld__blk947_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld__blk947_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld__blk947_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld__blk947_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld__blk947_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld__blk947_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld__blk947_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld__blk947_dn17 + locals.var_dps0_dn17), );
                locals.var_ps0ld__blk947_rv = 0.0;
            }
            let assign31370_body29_e46135: f64 = (locals.var_dps0).abs();
            let assign31370_body29_e46139: f64 = (locals.var_fs0__blk975).abs();
            let assign31370_body29_e46142: f64 = if ((assign31370_body29_e46135 <= 5e-12) && (assign31370_body29_e46139 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1016 = assign31370_body29_e46142;
            locals.var_guard1016_rv = 0.0;
            if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1016 != 0.0)) {
                locals.var_flg_conv__blk920 = 1.0;
                locals.var_flg_conv__blk920_rv = 0.0;
            }
            if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
                let assign31370_body31_e46175: f64 = (locals.var_lp_s0 + 1.0);
                locals.var_lp_s0 = assign31370_body31_e46175;
                locals.var_lp_s0_rv = 0.0;
            }
        }
        let assign31390_e46183: f64 = if locals.var_chi__blk945 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign31390_e46183;
        locals.var_guard1018_rv = 0.0;
        if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 != 0.0)) {
            let assign31430_e46239: f64 = (locals.var_fb__blk969 * locals.var_fb__blk969);
            let assign31430_e46242: f64 = (10.0 * 2.220446049250313e-16);
            let assign31430_e46243: f64 = (assign31430_e46239 + assign31430_e46242);
            (locals.var_xi0__blk978, locals.var_xi0__blk978_dn0, locals.var_xi0__blk978_dn2, locals.var_xi0__blk978_dn6, locals.var_xi0__blk978_dn7, locals.var_xi0__blk978_dn10, locals.var_xi0__blk978_dn11, locals.var_xi0__blk978_dn12, locals.var_xi0__blk978_dn17, ) = (assign31430_e46243, ((locals.var_fb__blk969_dn0 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn0)), ((locals.var_fb__blk969_dn2 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn2)), ((locals.var_fb__blk969_dn6 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn6)), ((locals.var_fb__blk969_dn7 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn7)), ((locals.var_fb__blk969_dn10 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn10)), ((locals.var_fb__blk969_dn11 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn11)), ((locals.var_fb__blk969_dn12 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn12)), ((locals.var_fb__blk969_dn17 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn17)), );
            locals.var_xi0__blk978_rv = 0.0;
        }
        if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 != 0.0)) {
            let assign31440_e46262: f64 = (10.0 * 2.220446049250313e-16);
            let assign31440_e46263: f64 = (locals.var_fb__blk969 + assign31440_e46262);
            (locals.var_xi0p12__blk979, locals.var_xi0p12__blk979_dn0, locals.var_xi0p12__blk979_dn2, locals.var_xi0p12__blk979_dn6, locals.var_xi0p12__blk979_dn7, locals.var_xi0p12__blk979_dn10, locals.var_xi0p12__blk979_dn11, locals.var_xi0p12__blk979_dn12, locals.var_xi0p12__blk979_dn17, ) = (assign31440_e46263, locals.var_fb__blk969_dn0, locals.var_fb__blk969_dn2, locals.var_fb__blk969_dn6, locals.var_fb__blk969_dn7, locals.var_fb__blk969_dn10, locals.var_fb__blk969_dn11, locals.var_fb__blk969_dn12, locals.var_fb__blk969_dn17, );
            locals.var_xi0p12__blk979_rv = 0.0;
        }
        if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 == 0.0)) {
            let assign31460_e46299: f64 = (locals.var_chi__blk945 - 1.0);
            (locals.var_xi0__blk978, locals.var_xi0__blk978_dn0, locals.var_xi0__blk978_dn2, locals.var_xi0__blk978_dn6, locals.var_xi0__blk978_dn7, locals.var_xi0__blk978_dn10, locals.var_xi0__blk978_dn11, locals.var_xi0__blk978_dn12, locals.var_xi0__blk978_dn17, ) = (assign31460_e46299, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17, );
            locals.var_xi0__blk978_rv = 0.0;
        }
        if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1018 == 0.0)) {
            let assign31470_e46317: f64 = (locals.var_xi0__blk978).sqrt();
            (locals.var_xi0p12__blk979, locals.var_xi0p12__blk979_dn0, locals.var_xi0p12__blk979_dn2, locals.var_xi0p12__blk979_dn6, locals.var_xi0p12__blk979_dn7, locals.var_xi0p12__blk979_dn10, locals.var_xi0p12__blk979_dn11, locals.var_xi0p12__blk979_dn12, locals.var_xi0p12__blk979_dn17, ) = (assign31470_e46317, (locals.var_xi0__blk978_dn0 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn2 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn6 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn7 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn10 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn11 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn12 / (2.0 * assign31470_e46317)), (locals.var_xi0__blk978_dn17 / (2.0 * assign31470_e46317)), );
            locals.var_xi0p12__blk979_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31480_e46333: f64 = (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979);
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17, ) = (assign31480_e46333, ((locals.var_cnst0over__blk930_dn0 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn0)), ((locals.var_cnst0over__blk930_dn2 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn2)), ((locals.var_cnst0over__blk930_dn6 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn6)), ((locals.var_cnst0over__blk930_dn7 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn7)), ((locals.var_cnst0over__blk930_dn10 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn10)), ((locals.var_cnst0over__blk930_dn11 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn11)), ((locals.var_cnst0over__blk930_dn12 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn12)), ((locals.var_cnst0over__blk930_dn17 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn17)), );
            locals.var_qbuld_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31490_e46350: f64 = (locals.var_fs02__blk971 + locals.var_xi0p12__blk979);
            let assign31490_e46351: f64 = (1.0 / assign31490_e46350);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31490_e46351, (-((locals.var_fs02__blk971_dn0 + locals.var_xi0p12__blk979_dn0) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn2 + locals.var_xi0p12__blk979_dn2) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn6 + locals.var_xi0p12__blk979_dn6) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn7 + locals.var_xi0p12__blk979_dn7) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn10 + locals.var_xi0p12__blk979_dn10) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn11 + locals.var_xi0p12__blk979_dn11) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn12 + locals.var_xi0p12__blk979_dn12) / (assign31490_e46350 * assign31490_e46350))), (-((locals.var_fs02__blk971_dn17 + locals.var_xi0p12__blk979_dn17) / (assign31490_e46350 * assign31490_e46350))), );
            locals.var_t1__blk898_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31500_e46367: f64 = (locals.var_cnst0over__blk930 * locals.var_fs01__blk967);
            let assign31500_e46369: f64 = (assign31500_e46367 * locals.var_t1__blk898);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17, ) = (assign31500_e46369, ((((locals.var_cnst0over__blk930_dn0 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn0)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn0)), ((((locals.var_cnst0over__blk930_dn2 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn2)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn2)), ((((locals.var_cnst0over__blk930_dn6 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn6)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn6)), ((((locals.var_cnst0over__blk930_dn7 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn7)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn7)), ((((locals.var_cnst0over__blk930_dn10 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn10)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn10)), ((((locals.var_cnst0over__blk930_dn11 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn11)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn11)), ((((locals.var_cnst0over__blk930_dn12 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn12)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn12)), ((((locals.var_cnst0over__blk930_dn17 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn17)) * locals.var_t1__blk898) + (assign31500_e46367 * locals.var_t1__blk898_dn17)), );
            locals.var_qiuld_rv = 0.0;
        }
        if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_guard1011 != 0.0)) {
            let assign31510_e46385: f64 = (locals.var_qbuld + locals.var_qiuld);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17, ) = (assign31510_e46385, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17), );
            locals.var_qsuld_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
            let assign31520_e46396: f64 = (locals.var_qsuld - locals.var_qbuld);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17, ) = (assign31520_e46396, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17), );
            locals.var_qiuld_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
            let (assign31530_e46414,) = {
    if (p.p43 == 1.0) {
        let assign31530_e46410: f64 = (locals.var_w_dioscv * locals.var_lov);
        (assign31530_e46410,)
    } else {
        let assign31530_e46413: f64 = (locals.var_weffcv_nf * locals.var_lov);
        (assign31530_e46413,)
    }
};
            (locals.var_t4__blk901, locals.var_t4__blk901_dn0, locals.var_t4__blk901_dn2, locals.var_t4__blk901_dn6, locals.var_t4__blk901_dn7, locals.var_t4__blk901_dn10, locals.var_t4__blk901_dn11, locals.var_t4__blk901_dn12, locals.var_t4__blk901_dn17, ) = (assign31530_e46414, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t4__blk901_rv = 0.0;
        }
        let assign31540_e46427: f64 = if (((locals.var_flg_overs__blk916 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk914 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign31540_e46427;
        locals.var_guard1020_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1020 != 0.0)) {
            let assign31550_e46438: f64 = (locals.var_t4__blk901 * locals.var_qsuld);
            (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17, ) = (assign31550_e46438, ((locals.var_t4__blk901_dn0 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn17)), );
            locals.var_qovs_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1020 != 0.0)) {
            let assign31560_e46451: f64 = (locals.var_t4__blk901 * locals.var_qbuld);
            (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17, ) = (assign31560_e46451, ((locals.var_t4__blk901_dn0 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn17)), );
            locals.var_qbsld_rv = 0.0;
        }
        let assign31570_e46464: f64 = if (((locals.var_flg_overd__blk917 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk915 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1021 = assign31570_e46464;
        locals.var_guard1021_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1021 != 0.0)) {
            let assign31580_e46475: f64 = (locals.var_t4__blk901 * locals.var_qsuld);
            (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17, ) = (assign31580_e46475, ((locals.var_t4__blk901_dn0 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn17)), );
            locals.var_qovd_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1021 != 0.0)) {
            let assign31590_e46488: f64 = (locals.var_t4__blk901 * locals.var_qbuld);
            (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17, ) = (assign31590_e46488, ((locals.var_t4__blk901_dn0 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn17)), );
            locals.var_qbdld_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
            let assign31600_e46496: f64 = (locals.var_modervs * locals.var_cgso_given);
            let assign31600_e46499: f64 = (locals.var_modenml * locals.var_cgdo_given);
            let assign31600_e46500: f64 = (assign31600_e46496 + assign31600_e46499);
            locals.var_flg_overgiven = assign31600_e46500;
            locals.var_flg_overgiven_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31610_e46510: f64 = (locals.var_modervs * p.p170);
            let assign31610_e46513: f64 = (locals.var_modenml * p.p169);
            let assign31610_e46514: f64 = (assign31610_e46510 + assign31610_e46513);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31610_e46514, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }
        let assign31620_e46519: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign31620_e46519;
        locals.var_guard1022_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1022 != 0.0)) {
            let assign31630_e46529: f64 = (locals.var_modervs * locals.var_w_dioscv);
            let assign31630_e46532: f64 = (locals.var_modenml * locals.var_w_diodcv);
            let assign31630_e46533: f64 = (assign31630_e46529 + assign31630_e46532);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31630_e46533, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk898_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1022 != 0.0)) {
            let assign31640_e46545: f64 = (-locals.var_t1__blk898);
            let assign31640_e46546: f64 = (locals.var_cgdoe * assign31640_e46545);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31640_e46546, ((locals.var_cgdoe_dn0 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgdoe_dn2 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgdoe_dn6 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgdoe_dn7 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgdoe_dn10 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgdoe_dn11 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgdoe_dn12 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgdoe_dn17 * assign31640_e46545) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn17))), );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1022 == 0.0)) {
            let assign31650_e46559: f64 = (-locals.var_weffcv_nf);
            let assign31650_e46560: f64 = (locals.var_cgdoe * assign31650_e46559);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31650_e46560, (locals.var_cgdoe_dn0 * assign31650_e46559), (locals.var_cgdoe_dn2 * assign31650_e46559), (locals.var_cgdoe_dn6 * assign31650_e46559), (locals.var_cgdoe_dn7 * assign31650_e46559), (locals.var_cgdoe_dn10 * assign31650_e46559), (locals.var_cgdoe_dn11 * assign31650_e46559), (locals.var_cgdoe_dn12 * assign31650_e46559), (locals.var_cgdoe_dn17 * assign31650_e46559), );
            locals.var_cgdoe_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31660_e46570: f64 = (-locals.var_cgdoe);
            let assign31660_e46573: f64 = (locals.var_vgs - locals.var_vds);
            let assign31660_e46574: f64 = (assign31660_e46570 * assign31660_e46573);
            let assign31660_e46575: f64 = (locals.var_qgod + assign31660_e46574);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign31660_e46575, (locals.var_qgod_dn0 + (((-locals.var_cgdoe_dn0) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn0)))), (locals.var_qgod_dn2 + (((-locals.var_cgdoe_dn2) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn2)))), (locals.var_qgod_dn6 + (((-locals.var_cgdoe_dn6) * assign31660_e46573) + (assign31660_e46570 * (locals.var_vgs_dn6 - locals.var_vds_dn6)))), (locals.var_qgod_dn7 + (((-locals.var_cgdoe_dn7) * assign31660_e46573) + (assign31660_e46570 * (locals.var_vgs_dn7 - locals.var_vds_dn7)))), (locals.var_qgod_dn10 + (((-locals.var_cgdoe_dn10) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn10)))), (locals.var_qgod_dn11 + (((-locals.var_cgdoe_dn11) * assign31660_e46573) + (assign31660_e46570 * (locals.var_vgs_dn11 - locals.var_vds_dn11)))), (locals.var_qgod_dn12 + (((-locals.var_cgdoe_dn12) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn12)))), (locals.var_qgod_dn17 + (((-locals.var_cgdoe_dn17) * assign31660_e46573) + (assign31660_e46570 * (-locals.var_vds_dn17)))), );
            locals.var_qgod_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
            let assign31670_e46583: f64 = (locals.var_modenml * locals.var_cgso_given);
            let assign31670_e46586: f64 = (locals.var_modervs * locals.var_cgdo_given);
            let assign31670_e46587: f64 = (assign31670_e46583 + assign31670_e46586);
            locals.var_flg_overgiven = assign31670_e46587;
            locals.var_flg_overgiven_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31680_e46597: f64 = (locals.var_modenml * p.p170);
            let assign31680_e46600: f64 = (locals.var_modervs * p.p169);
            let assign31680_e46601: f64 = (assign31680_e46597 + assign31680_e46600);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31680_e46601, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        let assign31690_e46606: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign31690_e46606;
        locals.var_guard1023_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1023 != 0.0)) {
            let assign31700_e46616: f64 = (locals.var_modenml * locals.var_w_dioscv);
            let assign31700_e46619: f64 = (locals.var_modervs * locals.var_w_diodcv);
            let assign31700_e46620: f64 = (assign31700_e46616 + assign31700_e46619);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31700_e46620, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk898_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1023 != 0.0)) {
            let assign31710_e46632: f64 = (-locals.var_t1__blk898);
            let assign31710_e46633: f64 = (locals.var_cgsoe * assign31710_e46632);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31710_e46633, ((locals.var_cgsoe_dn0 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgsoe_dn2 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgsoe_dn6 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgsoe_dn7 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgsoe_dn10 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgsoe_dn11 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgsoe_dn12 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgsoe_dn17 * assign31710_e46632) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn17))), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) && (locals.var_guard1023 == 0.0)) {
            let assign31720_e46646: f64 = (-locals.var_weffcv_nf);
            let assign31720_e46647: f64 = (locals.var_cgsoe * assign31720_e46646);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31720_e46647, (locals.var_cgsoe_dn0 * assign31720_e46646), (locals.var_cgsoe_dn2 * assign31720_e46646), (locals.var_cgsoe_dn6 * assign31720_e46646), (locals.var_cgsoe_dn7 * assign31720_e46646), (locals.var_cgsoe_dn10 * assign31720_e46646), (locals.var_cgsoe_dn11 * assign31720_e46646), (locals.var_cgsoe_dn12 * assign31720_e46646), (locals.var_cgsoe_dn17 * assign31720_e46646), );
            locals.var_cgsoe_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_flg_overgiven != 0.0)) {
            let assign31730_e46657: f64 = (-locals.var_cgsoe);
            let assign31730_e46659: f64 = (assign31730_e46657 * locals.var_vgs);
            let assign31730_e46660: f64 = (locals.var_qgos + assign31730_e46659);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign31730_e46660, (locals.var_qgos_dn0 + ((-locals.var_cgsoe_dn0) * locals.var_vgs)), (locals.var_qgos_dn2 + ((-locals.var_cgsoe_dn2) * locals.var_vgs)), (locals.var_qgos_dn6 + (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31730_e46657 * locals.var_vgs_dn6))), (locals.var_qgos_dn7 + (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31730_e46657 * locals.var_vgs_dn7))), (locals.var_qgos_dn10 + ((-locals.var_cgsoe_dn10) * locals.var_vgs)), (locals.var_qgos_dn11 + (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31730_e46657 * locals.var_vgs_dn11))), (locals.var_qgos_dn12 + ((-locals.var_cgsoe_dn12) * locals.var_vgs)), (locals.var_qgos_dn17 + ((-locals.var_cgsoe_dn17) * locals.var_vgs)), );
            locals.var_qgos_rv = 0.0;
        }
        let assign31740_e46675: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgdo_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign31740_e46675;
        locals.var_guard1024_rv = 0.0;
        let assign31750_e46678: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign31750_e46678;
        locals.var_guard1025_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 != 0.0)) {
            let assign31760_e46688: f64 = (-locals.var_cox0__blk908);
            let assign31760_e46690: f64 = (assign31760_e46688 * p.p188);
            let assign31760_e46692: f64 = (assign31760_e46690 * locals.var_w_diodcv);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31760_e46692, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 == 0.0)) {
            let assign31770_e46705: f64 = (-locals.var_cox0__blk908);
            let assign31770_e46707: f64 = (assign31770_e46705 * p.p188);
            let assign31770_e46709: f64 = (assign31770_e46707 * locals.var_weffcv_nf);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31770_e46709, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) {
            let assign31780_e46721: f64 = (locals.var_modervs * p.p170);
            let assign31780_e46724: f64 = (locals.var_modenml * p.p169);
            let assign31780_e46725: f64 = (assign31780_e46721 + assign31780_e46724);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31780_e46725, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }
        let assign31790_e46730: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign31790_e46730;
        locals.var_guard1026_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1026 != 0.0)) {
            let assign31800_e46742: f64 = (locals.var_modervs * locals.var_w_dioscv);
            let assign31800_e46745: f64 = (locals.var_modenml * locals.var_w_diodcv);
            let assign31800_e46746: f64 = (assign31800_e46742 + assign31800_e46745);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31800_e46746, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk898_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1026 != 0.0)) {
            let assign31810_e46760: f64 = (-locals.var_t1__blk898);
            let assign31810_e46761: f64 = (locals.var_cgdoe * assign31810_e46760);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31810_e46761, ((locals.var_cgdoe_dn0 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgdoe_dn2 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgdoe_dn6 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgdoe_dn7 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgdoe_dn10 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgdoe_dn11 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgdoe_dn12 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgdoe_dn17 * assign31810_e46760) + (locals.var_cgdoe * (-locals.var_t1__blk898_dn17))), );
            locals.var_cgdoe_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1024 == 0.0)) && (locals.var_guard1026 == 0.0)) {
            let assign31820_e46776: f64 = (-locals.var_weffcv_nf);
            let assign31820_e46777: f64 = (locals.var_cgdoe * assign31820_e46776);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31820_e46777, (locals.var_cgdoe_dn0 * assign31820_e46776), (locals.var_cgdoe_dn2 * assign31820_e46776), (locals.var_cgdoe_dn6 * assign31820_e46776), (locals.var_cgdoe_dn7 * assign31820_e46776), (locals.var_cgdoe_dn10 * assign31820_e46776), (locals.var_cgdoe_dn11 * assign31820_e46776), (locals.var_cgdoe_dn12 * assign31820_e46776), (locals.var_cgdoe_dn17 * assign31820_e46776), );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) {
            let assign31830_e46785: f64 = (-locals.var_cgdoe);
            let assign31830_e46788: f64 = (locals.var_vgs - locals.var_vds);
            let assign31830_e46789: f64 = (assign31830_e46785 * assign31830_e46788);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign31830_e46789, (((-locals.var_cgdoe_dn0) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31830_e46788) + (assign31830_e46785 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31830_e46788) + (assign31830_e46785 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31830_e46788) + (assign31830_e46785 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31830_e46788) + (assign31830_e46785 * (-locals.var_vds_dn17))), );
            locals.var_qgod_rv = 0.0;
        }
        let assign31840_e46804: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign31840_e46804;
        locals.var_guard1027_rv = 0.0;
        let assign31850_e46807: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign31850_e46807;
        locals.var_guard1028_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 != 0.0)) && (locals.var_guard1028 != 0.0)) {
            let assign31860_e46817: f64 = (-locals.var_cox0__blk908);
            let assign31860_e46819: f64 = (assign31860_e46817 * p.p188);
            let assign31860_e46821: f64 = (assign31860_e46819 * locals.var_w_dioscv);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31860_e46821, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 != 0.0)) && (locals.var_guard1028 == 0.0)) {
            let assign31870_e46834: f64 = (-locals.var_cox0__blk908);
            let assign31870_e46836: f64 = (assign31870_e46834 * p.p188);
            let assign31870_e46838: f64 = (assign31870_e46836 * locals.var_weffcv_nf);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31870_e46838, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) {
            let assign31880_e46850: f64 = (locals.var_modenml * p.p170);
            let assign31880_e46853: f64 = (locals.var_modervs * p.p169);
            let assign31880_e46854: f64 = (assign31880_e46850 + assign31880_e46853);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31880_e46854, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        let assign31890_e46859: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1029 = assign31890_e46859;
        locals.var_guard1029_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) && (locals.var_guard1029 != 0.0)) {
            let assign31900_e46871: f64 = (locals.var_modenml * locals.var_w_dioscv);
            let assign31900_e46874: f64 = (locals.var_modervs * locals.var_w_diodcv);
            let assign31900_e46875: f64 = (assign31900_e46871 + assign31900_e46874);
            (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17, ) = (assign31900_e46875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk898_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) && (locals.var_guard1029 != 0.0)) {
            let assign31910_e46889: f64 = (-locals.var_t1__blk898);
            let assign31910_e46890: f64 = (locals.var_cgsoe * assign31910_e46889);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31910_e46890, ((locals.var_cgsoe_dn0 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn0))), ((locals.var_cgsoe_dn2 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn2))), ((locals.var_cgsoe_dn6 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn6))), ((locals.var_cgsoe_dn7 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn7))), ((locals.var_cgsoe_dn10 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn10))), ((locals.var_cgsoe_dn11 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn11))), ((locals.var_cgsoe_dn12 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn12))), ((locals.var_cgsoe_dn17 * assign31910_e46889) + (locals.var_cgsoe * (-locals.var_t1__blk898_dn17))), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) && (locals.var_guard1027 == 0.0)) && (locals.var_guard1029 == 0.0)) {
            let assign31920_e46905: f64 = (-locals.var_weffcv_nf);
            let assign31920_e46906: f64 = (locals.var_cgsoe * assign31920_e46905);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31920_e46906, (locals.var_cgsoe_dn0 * assign31920_e46905), (locals.var_cgsoe_dn2 * assign31920_e46905), (locals.var_cgsoe_dn6 * assign31920_e46905), (locals.var_cgsoe_dn7 * assign31920_e46905), (locals.var_cgsoe_dn10 * assign31920_e46905), (locals.var_cgsoe_dn11 * assign31920_e46905), (locals.var_cgsoe_dn12 * assign31920_e46905), (locals.var_cgsoe_dn17 * assign31920_e46905), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard980 == 0.0)) {
            let assign31930_e46914: f64 = (-locals.var_cgsoe);
            let assign31930_e46916: f64 = (assign31930_e46914 * locals.var_vgs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign31930_e46916, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31930_e46914 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31930_e46914 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31930_e46914 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs), );
            locals.var_qgos_rv = 0.0;
        }
        let assign31940_e46921: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1030 = assign31940_e46921;
        locals.var_guard1030_rv = 0.0;
        if (locals.var_guard1030 != 0.0) {
            (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12, ) = (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12, );
            locals.var_vbdj_rv = 0.0;
            (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12, ) = (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12, );
            locals.var_vbsj_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign31970_e46934: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign31970_e46937: f64 = (locals.var_eg * locals.var_beta);
            let assign31970_e46938: f64 = (assign31970_e46934 - assign31970_e46937);
            let assign31970_e46942: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign31970_e46943: f64 = (assign31970_e46942).ln();
            let assign31970_e46944: f64 = (p.p175 * assign31970_e46943);
            let assign31970_e46945: f64 = (assign31970_e46938 + assign31970_e46944);
            let assign31970_e46947: f64 = (assign31970_e46945 / p.p174);
            let assign31970_e46948: f64 = (assign31970_e46947).exp();
            let assign31970_e46949: f64 = (p.p173 * assign31970_e46948);
            (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17, ) = (assign31970_e46949, (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31970_e46942))) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign31980_e46956: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign31980_e46959: f64 = (locals.var_eg * locals.var_beta);
            let assign31980_e46960: f64 = (assign31980_e46956 - assign31980_e46959);
            let assign31980_e46964: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign31980_e46965: f64 = (assign31980_e46964).ln();
            let assign31980_e46966: f64 = (p.p176 * assign31980_e46965);
            let assign31980_e46967: f64 = (assign31980_e46960 + assign31980_e46966);
            let assign31980_e46969: f64 = (assign31980_e46967 / p.p174);
            let assign31980_e46970: f64 = (assign31980_e46969).exp();
            let assign31980_e46971: f64 = (p.p173 * assign31980_e46970);
            (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17, ) = (assign31980_e46971, (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31980_e46964))) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js2_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign31990_e46977: f64 = (locals.var_w_diod * p.p237);
            let assign31990_e46979: f64 = (assign31990_e46977 * locals.var_js);
            (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, ) = (assign31990_e46979, (assign31990_e46977 * locals.var_js_dn0), (assign31990_e46977 * locals.var_js_dn2), (assign31990_e46977 * locals.var_js_dn6), (assign31990_e46977 * locals.var_js_dn7), (assign31990_e46977 * locals.var_js_dn10), (assign31990_e46977 * locals.var_js_dn11), (assign31990_e46977 * locals.var_js_dn12), (assign31990_e46977 * locals.var_js_dn17), );
            locals.var_isbd_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32000_e46985: f64 = (locals.var_w_diod * p.p237);
            let assign32000_e46987: f64 = (assign32000_e46985 * locals.var_js2);
            (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17, ) = (assign32000_e46987, (assign32000_e46985 * locals.var_js2_dn0), (assign32000_e46985 * locals.var_js2_dn2), (assign32000_e46985 * locals.var_js2_dn6), (assign32000_e46985 * locals.var_js2_dn7), (assign32000_e46985 * locals.var_js2_dn10), (assign32000_e46985 * locals.var_js2_dn11), (assign32000_e46985 * locals.var_js2_dn12), (assign32000_e46985 * locals.var_js2_dn17), );
            locals.var_isbd2_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32010_e46993: f64 = (locals.var_w_dios * p.p237);
            let assign32010_e46995: f64 = (assign32010_e46993 * locals.var_js);
            (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17, ) = (assign32010_e46995, (assign32010_e46993 * locals.var_js_dn0), (assign32010_e46993 * locals.var_js_dn2), (assign32010_e46993 * locals.var_js_dn6), (assign32010_e46993 * locals.var_js_dn7), (assign32010_e46993 * locals.var_js_dn10), (assign32010_e46993 * locals.var_js_dn11), (assign32010_e46993 * locals.var_js_dn12), (assign32010_e46993 * locals.var_js_dn17), );
            locals.var_isbs_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32020_e47001: f64 = (locals.var_w_dios * p.p237);
            let assign32020_e47003: f64 = (assign32020_e47001 * locals.var_js2);
            (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17, ) = (assign32020_e47003, (assign32020_e47001 * locals.var_js2_dn0), (assign32020_e47001 * locals.var_js2_dn2), (assign32020_e47001 * locals.var_js2_dn6), (assign32020_e47001 * locals.var_js2_dn7), (assign32020_e47001 * locals.var_js2_dn10), (assign32020_e47001 * locals.var_js2_dn11), (assign32020_e47001 * locals.var_js2_dn12), (assign32020_e47001 * locals.var_js2_dn17), );
            locals.var_isbs2_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32030_e47009: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32030_e47009, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0, );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32050_e47021: f64 = (locals.var_isbd + 1e-50);
            (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17, ) = (assign32050_e47021, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, );
            locals.var_t2__blk1033_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            (locals.var_vbdt, locals.var_vbdt_dn10, ) = (0.0, 0.0, );
            locals.var_vbdt_rv = 0.0;
            (locals.var_vbst, locals.var_vbst_dn10, ) = (0.0, 0.0, );
            locals.var_vbst_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32090_e47049: f64 = (p.p174 * locals.var_beta_inv);
            (locals.var_nvtm, locals.var_nvtm_dn10, ) = (assign32090_e47049, (p.p174 * locals.var_beta_inv_dn10), );
            locals.var_nvtm_rv = 0.0;
        }
        let assign32100_e47054: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign32100_e47054;
        locals.var_guard1059_rv = 0.0;
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 != 0.0)) {
            let assign32110_e47060: f64 = (locals.var_vbdj / locals.var_nvtm);
            let assign32110_e47061: f64 = (assign32110_e47060).exp();
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32110_e47061, (assign32110_e47061 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32110_e47061 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32110_e47061 * (locals.var_vbdj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 != 0.0)) {
            let assign32120_e47070: f64 = (locals.var_t1__blk1032 - 1.0);
            let assign32120_e47071: f64 = (locals.var_isbd * assign32120_e47070);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32120_e47071, (locals.var_isbd_dn0 * assign32120_e47070), (locals.var_isbd_dn2 * assign32120_e47070), ((locals.var_isbd_dn6 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn6)), ((locals.var_isbd_dn7 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn7)), ((locals.var_isbd_dn10 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn10)), (locals.var_isbd_dn11 * assign32120_e47070), ((locals.var_isbd_dn12 * assign32120_e47070) + (locals.var_isbd * locals.var_t1__blk1032_dn12)), (locals.var_isbd_dn17 * assign32120_e47070), );
            locals.var_ibd_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 == 0.0)) {
            let assign32130_e47080: f64 = (locals.var_vbdt / locals.var_nvtm);
            let assign32130_e47081: f64 = (assign32130_e47080).exp();
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32130_e47081, 0.0, 0.0, (assign32130_e47081 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1059 == 0.0)) {
            let assign32140_e47091: f64 = (locals.var_t1__blk1032 - 1.0);
            let assign32140_e47092: f64 = (locals.var_isbd * assign32140_e47091);
            let assign32140_e47095: f64 = (locals.var_isbd / locals.var_nvtm);
            let assign32140_e47097: f64 = (assign32140_e47095 * locals.var_t1__blk1032);
            let assign32140_e47100: f64 = (locals.var_vbdj - locals.var_vbdt);
            let assign32140_e47101: f64 = (assign32140_e47097 * assign32140_e47100);
            let assign32140_e47102: f64 = (assign32140_e47092 + assign32140_e47101);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32140_e47102, ((locals.var_isbd_dn0 * assign32140_e47091) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)), ((locals.var_isbd_dn2 * assign32140_e47091) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)), (((locals.var_isbd_dn6 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn6)) * assign32140_e47100) + (assign32140_e47097 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn7)) * assign32140_e47100)), (((locals.var_isbd_dn10 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn10)) * assign32140_e47100) + (assign32140_e47097 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32140_e47091) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)), (((locals.var_isbd_dn12 * assign32140_e47091) + (locals.var_isbd * locals.var_t1__blk1032_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32140_e47095 * locals.var_t1__blk1032_dn12)) * assign32140_e47100) + (assign32140_e47097 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32140_e47091) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32140_e47100)), );
            locals.var_ibd_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32150_e47109: f64 = (p.p178 * locals.var_vbdj);
            let assign32150_e47111: f64 = (assign32150_e47109 * locals.var_isbd2);
            let assign32150_e47112: f64 = (locals.var_ibd + assign32150_e47111);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32150_e47112, (locals.var_ibd_dn0 + (assign32150_e47109 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32150_e47109 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32150_e47109 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32150_e47109 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32150_e47109 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32150_e47109 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32150_e47109 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32150_e47109 * locals.var_isbd2_dn17)), );
            locals.var_ibd_rv = 0.0;
        }
        let assign32160_e47117: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign32160_e47117;
        locals.var_guard1060_rv = 0.0;
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 != 0.0)) {
            let assign32170_e47123: f64 = (locals.var_vbsj / locals.var_nvtm);
            let assign32170_e47124: f64 = (assign32170_e47123).exp();
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32170_e47124, 0.0, (assign32170_e47124 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32170_e47124 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32170_e47124 * (locals.var_vbsj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 != 0.0)) {
            let assign32180_e47133: f64 = (locals.var_t1__blk1032 - 1.0);
            let assign32180_e47134: f64 = (locals.var_isbs * assign32180_e47133);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32180_e47134, (locals.var_isbs_dn0 * assign32180_e47133), (locals.var_isbs_dn2 * assign32180_e47133), ((locals.var_isbs_dn6 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn6)), ((locals.var_isbs_dn7 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn7)), ((locals.var_isbs_dn10 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn10)), (locals.var_isbs_dn11 * assign32180_e47133), ((locals.var_isbs_dn12 * assign32180_e47133) + (locals.var_isbs * locals.var_t1__blk1032_dn12)), (locals.var_isbs_dn17 * assign32180_e47133), );
            locals.var_ibs_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 == 0.0)) {
            let assign32190_e47143: f64 = (locals.var_vbst / locals.var_nvtm);
            let assign32190_e47144: f64 = (assign32190_e47143).exp();
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32190_e47144, 0.0, 0.0, (assign32190_e47144 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1060 == 0.0)) {
            let assign32200_e47154: f64 = (locals.var_t1__blk1032 - 1.0);
            let assign32200_e47155: f64 = (locals.var_isbs * assign32200_e47154);
            let assign32200_e47158: f64 = (locals.var_isbs / locals.var_nvtm);
            let assign32200_e47160: f64 = (assign32200_e47158 * locals.var_t1__blk1032);
            let assign32200_e47163: f64 = (locals.var_vbsj - locals.var_vbst);
            let assign32200_e47164: f64 = (assign32200_e47160 * assign32200_e47163);
            let assign32200_e47165: f64 = (assign32200_e47155 + assign32200_e47164);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32200_e47165, ((locals.var_isbs_dn0 * assign32200_e47154) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)), ((locals.var_isbs_dn2 * assign32200_e47154) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)), (((locals.var_isbs_dn6 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn6)) * assign32200_e47163)), (((locals.var_isbs_dn7 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn7)) * assign32200_e47163) + (assign32200_e47160 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn10)) * assign32200_e47163) + (assign32200_e47160 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32200_e47154) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)), (((locals.var_isbs_dn12 * assign32200_e47154) + (locals.var_isbs * locals.var_t1__blk1032_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1032) + (assign32200_e47158 * locals.var_t1__blk1032_dn12)) * assign32200_e47163) + (assign32200_e47160 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32200_e47154) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1032) * assign32200_e47163)), );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32210_e47172: f64 = (p.p178 * locals.var_vbsj);
            let assign32210_e47174: f64 = (assign32210_e47172 * locals.var_isbs2);
            let assign32210_e47175: f64 = (locals.var_ibs + assign32210_e47174);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32210_e47175, (locals.var_ibs_dn0 + (assign32210_e47172 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32210_e47172 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32210_e47172 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32210_e47172 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32210_e47172 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32210_e47172 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32210_e47172 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32210_e47172 * locals.var_isbs2_dn17)), );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32220_e47182: f64 = (locals.var_gjmin * locals.var_vbdj);
            let assign32220_e47183: f64 = (locals.var_ibd + assign32220_e47182);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32220_e47183, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17, );
            locals.var_ibd_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32230_e47190: f64 = (locals.var_gjmin * locals.var_vbsj);
            let assign32230_e47191: f64 = (locals.var_ibs + assign32230_e47190);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32230_e47191, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17, );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32240_e47197: f64 = (p.p179 * p.p2);
            locals.var_czbd = assign32240_e47197;
            locals.var_czbd_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32250_e47203: f64 = (p.p179 * p.p3);
            locals.var_czbs = assign32250_e47203;
            locals.var_czbs_rv = 0.0;
        }
        if (locals.var_guard1030 != 0.0) {
            let assign32260_e47209: f64 = (p.p237 - p.p238);
            locals.var_xp_max = assign32260_e47209;
            locals.var_xp_max_rv = 0.0;
        }
        let assign32270_e47214: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32270_e47214;
        locals.var_guard1061_rv = 0.0;
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1061 != 0.0)) {
            locals.var_czbd = 0.0;
            locals.var_czbd_rv = 0.0;
            locals.var_czbs = 0.0;
            locals.var_czbs_rv = 0.0;
        }
        let assign32300_e47229: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32300_e47229;
        locals.var_guard1062_rv = 0.0;
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) {
            let assign32310_e47236: f64 = (p.p5 - locals.var_w_dioscv);
            let assign32310_e47237: f64 = (p.p180 * assign32310_e47236);
            locals.var_czbssw = assign32310_e47237;
            locals.var_czbssw_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) {
            let assign32320_e47245: f64 = (p.p181 * locals.var_w_dioscv);
            locals.var_czbsswg = assign32320_e47245;
            locals.var_czbsswg_rv = 0.0;
        }
        let assign32330_e47250: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32330_e47250;
        locals.var_guard1063_rv = 0.0;
        let assign32340_e47253: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32340_e47253;
        locals.var_guard1064_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) {
            let assign32350_e47264: f64 = (locals.var_vbsj / p.p185);
            let assign32350_e47265: f64 = (1.0 - assign32350_e47264);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32350_e47265, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32360_e47270: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32360_e47270;
        locals.var_guard1065_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
            let assign32370_e47282: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32370_e47283: f64 = (1.0 / assign32370_e47282);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32370_e47283, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
            let assign32380_e47298: f64 = (-p.p182);
            let assign32380_e47299: f64 = (locals.var_arg__blk1057).powf(assign32380_e47298);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32380_e47299, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((locals.var_arg__blk1057).powf(assign32380_e47298 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32380_e47299 * (assign32380_e47298 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((locals.var_arg__blk1057).powf(assign32380_e47298 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32380_e47299 * (assign32380_e47298 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((locals.var_arg__blk1057).powf(assign32380_e47298 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32380_e47299 * (assign32380_e47298 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) {
            let assign32390_e47311: f64 = (p.p185 * locals.var_czbs);
            let assign32390_e47315: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32390_e47316: f64 = (1.0 - assign32390_e47315);
            let assign32390_e47317: f64 = (assign32390_e47311 * assign32390_e47316);
            let assign32390_e47320: f64 = (1.0 - p.p182);
            let assign32390_e47321: f64 = (assign32390_e47317 / assign32390_e47320);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32390_e47321, 0.0, 0.0, ((assign32390_e47311 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32390_e47320), ((assign32390_e47311 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32390_e47320), 0.0, 0.0, ((assign32390_e47311 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32390_e47320), 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32410_e47337: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32410_e47337;
        locals.var_guard1066_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32420_e47348: f64 = (locals.var_vbsj / p.p186);
            let assign32420_e47349: f64 = (1.0 - assign32420_e47348);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32420_e47349, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32430_e47354: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32430_e47354;
        locals.var_guard1067_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
            let assign32440_e47366: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32440_e47367: f64 = (1.0 / assign32440_e47366);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32440_e47367, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
            let assign32450_e47382: f64 = (-p.p183);
            let assign32450_e47383: f64 = (locals.var_arg__blk1057).powf(assign32450_e47382);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32450_e47383, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((locals.var_arg__blk1057).powf(assign32450_e47382 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32450_e47383 * (assign32450_e47382 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((locals.var_arg__blk1057).powf(assign32450_e47382 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32450_e47383 * (assign32450_e47382 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((locals.var_arg__blk1057).powf(assign32450_e47382 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32450_e47383 * (assign32450_e47382 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32460_e47396: f64 = (p.p186 * locals.var_czbssw);
            let assign32460_e47400: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32460_e47401: f64 = (1.0 - assign32460_e47400);
            let assign32460_e47402: f64 = (assign32460_e47396 * assign32460_e47401);
            let assign32460_e47405: f64 = (1.0 - p.p183);
            let assign32460_e47406: f64 = (assign32460_e47402 / assign32460_e47405);
            let assign32460_e47407: f64 = (locals.var_qbs + assign32460_e47406);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32460_e47407, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32460_e47396 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32460_e47405)), (locals.var_qbs_dn7 + ((assign32460_e47396 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32460_e47405)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32460_e47396 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32460_e47405)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32470_e47412: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32470_e47412;
        locals.var_guard1068_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) {
            let assign32480_e47423: f64 = (locals.var_vbsj / p.p187);
            let assign32480_e47424: f64 = (1.0 - assign32480_e47423);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32480_e47424, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32490_e47429: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32490_e47429;
        locals.var_guard1069_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
            let assign32500_e47441: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32500_e47442: f64 = (1.0 / assign32500_e47441);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32500_e47442, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
            let assign32510_e47457: f64 = (-p.p184);
            let assign32510_e47458: f64 = (locals.var_arg__blk1057).powf(assign32510_e47457);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32510_e47458, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((locals.var_arg__blk1057).powf(assign32510_e47457 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32510_e47458 * (assign32510_e47457 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((locals.var_arg__blk1057).powf(assign32510_e47457 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32510_e47458 * (assign32510_e47457 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((locals.var_arg__blk1057).powf(assign32510_e47457 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32510_e47458 * (assign32510_e47457 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1068 != 0.0)) {
            let assign32520_e47471: f64 = (p.p187 * locals.var_czbsswg);
            let assign32520_e47475: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32520_e47476: f64 = (1.0 - assign32520_e47475);
            let assign32520_e47477: f64 = (assign32520_e47471 * assign32520_e47476);
            let assign32520_e47480: f64 = (1.0 - p.p184);
            let assign32520_e47481: f64 = (assign32520_e47477 / assign32520_e47480);
            let assign32520_e47482: f64 = (locals.var_qbs + assign32520_e47481);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32520_e47482, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32520_e47471 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32520_e47480)), (locals.var_qbs_dn7 + ((assign32520_e47471 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32520_e47480)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32520_e47471 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32520_e47480)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
            let assign32530_e47493: f64 = (locals.var_czbs + locals.var_czbssw);
            let assign32530_e47495: f64 = (assign32530_e47493 + locals.var_czbsswg);
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32530_e47495, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
            let assign32540_e47506: f64 = (locals.var_czbs * p.p182);
            let assign32540_e47508: f64 = (assign32540_e47506 / p.p185);
            let assign32540_e47511: f64 = (locals.var_czbssw * p.p183);
            let assign32540_e47513: f64 = (assign32540_e47511 / p.p186);
            let assign32540_e47514: f64 = (assign32540_e47508 + assign32540_e47513);
            let assign32540_e47517: f64 = (locals.var_czbsswg * p.p184);
            let assign32540_e47519: f64 = (assign32540_e47517 / p.p187);
            let assign32540_e47520: f64 = (assign32540_e47514 + assign32540_e47519);
            (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17, ) = (assign32540_e47520, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1033_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
            let assign32550_e47533: f64 = (locals.var_vbsj * 0.5);
            let assign32550_e47535: f64 = (assign32550_e47533 * locals.var_t2__blk1033);
            let assign32550_e47536: f64 = (locals.var_t1__blk1032 + assign32550_e47535);
            let assign32550_e47537: f64 = (locals.var_vbsj * assign32550_e47536);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32550_e47537, (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn0)), (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn2)), (locals.var_vbsj * (locals.var_t1__blk1032_dn6 + (assign32550_e47533 * locals.var_t2__blk1033_dn6))), ((locals.var_vbsj_dn7 * assign32550_e47536) + (locals.var_vbsj * (locals.var_t1__blk1032_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1033) + (assign32550_e47533 * locals.var_t2__blk1033_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1032_dn10 + (assign32550_e47533 * locals.var_t2__blk1033_dn10))), (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn11)), ((locals.var_vbsj_dn12 * assign32550_e47536) + (locals.var_vbsj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign32550_e47533 * locals.var_t2__blk1033_dn12))))), (locals.var_vbsj * (assign32550_e47533 * locals.var_t2__blk1033_dn17)), );
            locals.var_qbs_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) {
            let assign32560_e47546: f64 = (p.p181 * p.p5);
            locals.var_czbsswg = assign32560_e47546;
            locals.var_czbsswg_rv = 0.0;
        }
        let assign32570_e47551: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32570_e47551;
        locals.var_guard1070_rv = 0.0;
        let assign32580_e47554: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32580_e47554;
        locals.var_guard1071_rv = 0.0;
    }
    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
            let assign32590_e47566: f64 = (locals.var_vbsj / p.p185);
            let assign32590_e47567: f64 = (1.0 - assign32590_e47566);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32590_e47567, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32600_e47572: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32600_e47572;
        locals.var_guard1072_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 != 0.0)) {
            let assign32610_e47585: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32610_e47586: f64 = (1.0 / assign32610_e47585);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32610_e47586, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) {
            let assign32620_e47602: f64 = (-p.p182);
            let assign32620_e47603: f64 = (locals.var_arg__blk1057).powf(assign32620_e47602);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32620_e47603, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((locals.var_arg__blk1057).powf(assign32620_e47602 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32620_e47603 * (assign32620_e47602 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((locals.var_arg__blk1057).powf(assign32620_e47602 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32620_e47603 * (assign32620_e47602 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((locals.var_arg__blk1057).powf(assign32620_e47602 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32620_e47603 * (assign32620_e47602 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
            let assign32630_e47616: f64 = (p.p185 * locals.var_czbs);
            let assign32630_e47620: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32630_e47621: f64 = (1.0 - assign32630_e47620);
            let assign32630_e47622: f64 = (assign32630_e47616 * assign32630_e47621);
            let assign32630_e47625: f64 = (1.0 - p.p182);
            let assign32630_e47626: f64 = (assign32630_e47622 / assign32630_e47625);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32630_e47626, 0.0, 0.0, ((assign32630_e47616 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32630_e47625), ((assign32630_e47616 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32630_e47625), 0.0, 0.0, ((assign32630_e47616 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32630_e47625), 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32650_e47643: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32650_e47643;
        locals.var_guard1073_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) {
            let assign32660_e47655: f64 = (locals.var_vbsj / p.p187);
            let assign32660_e47656: f64 = (1.0 - assign32660_e47655);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32660_e47656, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32670_e47661: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32670_e47661;
        locals.var_guard1074_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) {
            let assign32680_e47674: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32680_e47675: f64 = (1.0 / assign32680_e47674);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32680_e47675, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32690_e47691: f64 = (-p.p184);
            let assign32690_e47692: f64 = (locals.var_arg__blk1057).powf(assign32690_e47691);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32690_e47692, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((locals.var_arg__blk1057).powf(assign32690_e47691 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32690_e47692 * (assign32690_e47691 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((locals.var_arg__blk1057).powf(assign32690_e47691 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32690_e47692 * (assign32690_e47691 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((locals.var_arg__blk1057).powf(assign32690_e47691 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32690_e47692 * (assign32690_e47691 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1073 != 0.0)) {
            let assign32700_e47706: f64 = (p.p187 * locals.var_czbsswg);
            let assign32700_e47710: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32700_e47711: f64 = (1.0 - assign32700_e47710);
            let assign32700_e47712: f64 = (assign32700_e47706 * assign32700_e47711);
            let assign32700_e47715: f64 = (1.0 - p.p184);
            let assign32700_e47716: f64 = (assign32700_e47712 / assign32700_e47715);
            let assign32700_e47717: f64 = (locals.var_qbs + assign32700_e47716);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32700_e47717, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32700_e47706 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32700_e47715)), (locals.var_qbs_dn7 + ((assign32700_e47706 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32700_e47715)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32700_e47706 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32700_e47715)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 == 0.0)) {
            let assign32710_e47729: f64 = (locals.var_czbs + locals.var_czbsswg);
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32710_e47729, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 == 0.0)) {
            let assign32720_e47741: f64 = (locals.var_czbs * p.p182);
            let assign32720_e47743: f64 = (assign32720_e47741 / p.p185);
            let assign32720_e47746: f64 = (locals.var_czbsswg * p.p184);
            let assign32720_e47748: f64 = (assign32720_e47746 / p.p187);
            let assign32720_e47749: f64 = (assign32720_e47743 + assign32720_e47748);
            (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17, ) = (assign32720_e47749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1033_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1062 == 0.0)) && (locals.var_guard1070 == 0.0)) {
            let assign32730_e47763: f64 = (locals.var_vbsj * 0.5);
            let assign32730_e47765: f64 = (assign32730_e47763 * locals.var_t2__blk1033);
            let assign32730_e47766: f64 = (locals.var_t1__blk1032 + assign32730_e47765);
            let assign32730_e47767: f64 = (locals.var_vbsj * assign32730_e47766);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32730_e47767, (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn0)), (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn2)), (locals.var_vbsj * (locals.var_t1__blk1032_dn6 + (assign32730_e47763 * locals.var_t2__blk1033_dn6))), ((locals.var_vbsj_dn7 * assign32730_e47766) + (locals.var_vbsj * (locals.var_t1__blk1032_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1033) + (assign32730_e47763 * locals.var_t2__blk1033_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1032_dn10 + (assign32730_e47763 * locals.var_t2__blk1033_dn10))), (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn11)), ((locals.var_vbsj_dn12 * assign32730_e47766) + (locals.var_vbsj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign32730_e47763 * locals.var_t2__blk1033_dn12))))), (locals.var_vbsj * (assign32730_e47763 * locals.var_t2__blk1033_dn17)), );
            locals.var_qbs_rv = 0.0;
        }
        let assign32740_e47772: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32740_e47772;
        locals.var_guard1075_rv = 0.0;
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) {
            let assign32750_e47779: f64 = (p.p4 - locals.var_w_diodcv);
            let assign32750_e47780: f64 = (p.p180 * assign32750_e47779);
            locals.var_czbdsw = assign32750_e47780;
            locals.var_czbdsw_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) {
            let assign32760_e47788: f64 = (p.p181 * locals.var_w_diodcv);
            locals.var_czbdswg = assign32760_e47788;
            locals.var_czbdswg_rv = 0.0;
        }
        let assign32770_e47793: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32770_e47793;
        locals.var_guard1076_rv = 0.0;
        let assign32780_e47796: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32780_e47796;
        locals.var_guard1077_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) {
            let assign32790_e47807: f64 = (locals.var_vbdj / p.p185);
            let assign32790_e47808: f64 = (1.0 - assign32790_e47807);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32790_e47808, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32800_e47813: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32800_e47813;
        locals.var_guard1078_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) {
            let assign32810_e47825: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32810_e47826: f64 = (1.0 / assign32810_e47825);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32810_e47826, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
            let assign32820_e47841: f64 = (-p.p182);
            let assign32820_e47842: f64 = (locals.var_arg__blk1057).powf(assign32820_e47841);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32820_e47842, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((locals.var_arg__blk1057).powf(assign32820_e47841 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32820_e47842 * (assign32820_e47841 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((locals.var_arg__blk1057).powf(assign32820_e47841 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32820_e47842 * (assign32820_e47841 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((locals.var_arg__blk1057).powf(assign32820_e47841 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32820_e47842 * (assign32820_e47841 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 != 0.0)) {
            let assign32830_e47854: f64 = (p.p185 * locals.var_czbd);
            let assign32830_e47858: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32830_e47859: f64 = (1.0 - assign32830_e47858);
            let assign32830_e47860: f64 = (assign32830_e47854 * assign32830_e47859);
            let assign32830_e47863: f64 = (1.0 - p.p182);
            let assign32830_e47864: f64 = (assign32830_e47860 / assign32830_e47863);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32830_e47864, 0.0, 0.0, ((assign32830_e47854 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32830_e47863), ((assign32830_e47854 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32830_e47863), 0.0, 0.0, ((assign32830_e47854 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32830_e47863), 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1077 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        let assign32850_e47880: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32850_e47880;
        locals.var_guard1079_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32860_e47891: f64 = (locals.var_vbdj / p.p186);
            let assign32860_e47892: f64 = (1.0 - assign32860_e47891);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32860_e47892, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32870_e47897: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32870_e47897;
        locals.var_guard1080_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
            let assign32880_e47909: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32880_e47910: f64 = (1.0 / assign32880_e47909);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32880_e47910, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
            let assign32890_e47925: f64 = (-p.p183);
            let assign32890_e47926: f64 = (locals.var_arg__blk1057).powf(assign32890_e47925);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32890_e47926, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((locals.var_arg__blk1057).powf(assign32890_e47925 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32890_e47926 * (assign32890_e47925 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((locals.var_arg__blk1057).powf(assign32890_e47925 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32890_e47926 * (assign32890_e47925 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((locals.var_arg__blk1057).powf(assign32890_e47925 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32890_e47926 * (assign32890_e47925 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32900_e47939: f64 = (p.p186 * locals.var_czbdsw);
            let assign32900_e47943: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32900_e47944: f64 = (1.0 - assign32900_e47943);
            let assign32900_e47945: f64 = (assign32900_e47939 * assign32900_e47944);
            let assign32900_e47948: f64 = (1.0 - p.p183);
            let assign32900_e47949: f64 = (assign32900_e47945 / assign32900_e47948);
            let assign32900_e47950: f64 = (locals.var_qbd + assign32900_e47949);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32900_e47950, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32900_e47939 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32900_e47948)), (locals.var_qbd_dn7 + ((assign32900_e47939 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32900_e47948)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32900_e47939 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32900_e47948)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        let assign32910_e47955: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32910_e47955;
        locals.var_guard1081_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) {
            let assign32920_e47966: f64 = (locals.var_vbdj / p.p187);
            let assign32920_e47967: f64 = (1.0 - assign32920_e47966);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign32920_e47967, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign32930_e47972: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign32930_e47972;
        locals.var_guard1082_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
            let assign32940_e47984: f64 = (locals.var_arg__blk1057).sqrt();
            let assign32940_e47985: f64 = (1.0 / assign32940_e47984);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32940_e47985, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
            let assign32950_e48000: f64 = (-p.p184);
            let assign32950_e48001: f64 = (locals.var_arg__blk1057).powf(assign32950_e48000);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32950_e48001, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((locals.var_arg__blk1057).powf(assign32950_e48000 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign32950_e48001 * (assign32950_e48000 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((locals.var_arg__blk1057).powf(assign32950_e48000 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign32950_e48001 * (assign32950_e48000 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((locals.var_arg__blk1057).powf(assign32950_e48000 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign32950_e48001 * (assign32950_e48000 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) && (locals.var_guard1081 != 0.0)) {
            let assign32960_e48014: f64 = (p.p187 * locals.var_czbdswg);
            let assign32960_e48018: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign32960_e48019: f64 = (1.0 - assign32960_e48018);
            let assign32960_e48020: f64 = (assign32960_e48014 * assign32960_e48019);
            let assign32960_e48023: f64 = (1.0 - p.p184);
            let assign32960_e48024: f64 = (assign32960_e48020 / assign32960_e48023);
            let assign32960_e48025: f64 = (locals.var_qbd + assign32960_e48024);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32960_e48025, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32960_e48014 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign32960_e48023)), (locals.var_qbd_dn7 + ((assign32960_e48014 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign32960_e48023)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32960_e48014 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign32960_e48023)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
            let assign32970_e48036: f64 = (locals.var_czbd + locals.var_czbdsw);
            let assign32970_e48038: f64 = (assign32970_e48036 + locals.var_czbdswg);
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign32970_e48038, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
            let assign32980_e48049: f64 = (locals.var_czbd * p.p182);
            let assign32980_e48051: f64 = (assign32980_e48049 / p.p185);
            let assign32980_e48054: f64 = (locals.var_czbdsw * p.p183);
            let assign32980_e48056: f64 = (assign32980_e48054 / p.p186);
            let assign32980_e48057: f64 = (assign32980_e48051 + assign32980_e48056);
            let assign32980_e48060: f64 = (locals.var_czbdswg * p.p184);
            let assign32980_e48062: f64 = (assign32980_e48060 / p.p187);
            let assign32980_e48063: f64 = (assign32980_e48057 + assign32980_e48062);
            (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17, ) = (assign32980_e48063, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1033_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
            let assign32990_e48076: f64 = (locals.var_vbdj * 0.5);
            let assign32990_e48078: f64 = (assign32990_e48076 * locals.var_t2__blk1033);
            let assign32990_e48079: f64 = (locals.var_t1__blk1032 + assign32990_e48078);
            let assign32990_e48080: f64 = (locals.var_vbdj * assign32990_e48079);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32990_e48080, (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn0)), (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn2)), ((locals.var_vbdj_dn6 * assign32990_e48079) + (locals.var_vbdj * (locals.var_t1__blk1032_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1033) + (assign32990_e48076 * locals.var_t2__blk1033_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1032_dn7 + (assign32990_e48076 * locals.var_t2__blk1033_dn7))), (locals.var_vbdj * (locals.var_t1__blk1032_dn10 + (assign32990_e48076 * locals.var_t2__blk1033_dn10))), (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn11)), ((locals.var_vbdj_dn12 * assign32990_e48079) + (locals.var_vbdj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign32990_e48076 * locals.var_t2__blk1033_dn12))))), (locals.var_vbdj * (assign32990_e48076 * locals.var_t2__blk1033_dn17)), );
            locals.var_qbd_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) {
            let assign33000_e48089: f64 = (p.p181 * p.p4);
            locals.var_czbdswg = assign33000_e48089;
            locals.var_czbdswg_rv = 0.0;
        }
        let assign33010_e48094: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign33010_e48094;
        locals.var_guard1083_rv = 0.0;
        let assign33020_e48097: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign33020_e48097;
        locals.var_guard1084_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) {
            let assign33030_e48109: f64 = (locals.var_vbdj / p.p185);
            let assign33030_e48110: f64 = (1.0 - assign33030_e48109);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign33030_e48110, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign33040_e48115: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33040_e48115;
        locals.var_guard1085_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 != 0.0)) {
            let assign33050_e48128: f64 = (locals.var_arg__blk1057).sqrt();
            let assign33050_e48129: f64 = (1.0 / assign33050_e48128);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33050_e48129, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 == 0.0)) {
            let assign33060_e48145: f64 = (-p.p182);
            let assign33060_e48146: f64 = (locals.var_arg__blk1057).powf(assign33060_e48145);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33060_e48146, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((locals.var_arg__blk1057).powf(assign33060_e48145 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign33060_e48146 * (assign33060_e48145 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((locals.var_arg__blk1057).powf(assign33060_e48145 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign33060_e48146 * (assign33060_e48145 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((locals.var_arg__blk1057).powf(assign33060_e48145 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign33060_e48146 * (assign33060_e48145 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) {
            let assign33070_e48159: f64 = (p.p185 * locals.var_czbd);
            let assign33070_e48163: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign33070_e48164: f64 = (1.0 - assign33070_e48163);
            let assign33070_e48165: f64 = (assign33070_e48159 * assign33070_e48164);
            let assign33070_e48168: f64 = (1.0 - p.p182);
            let assign33070_e48169: f64 = (assign33070_e48165 / assign33070_e48168);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33070_e48169, 0.0, 0.0, ((assign33070_e48159 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign33070_e48168), ((assign33070_e48159 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign33070_e48168), 0.0, 0.0, ((assign33070_e48159 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign33070_e48168), 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        let assign33090_e48186: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33090_e48186;
        locals.var_guard1086_rv = 0.0;
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) {
            let assign33100_e48198: f64 = (locals.var_vbdj / p.p187);
            let assign33100_e48199: f64 = (1.0 - assign33100_e48198);
            (locals.var_arg__blk1057, locals.var_arg__blk1057_dn6, locals.var_arg__blk1057_dn7, locals.var_arg__blk1057_dn12, ) = (assign33100_e48199, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1057_rv = 0.0;
        }
        let assign33110_e48204: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33110_e48204;
        locals.var_guard1087_rv = 0.0;
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 != 0.0)) {
            let assign33120_e48217: f64 = (locals.var_arg__blk1057).sqrt();
            let assign33120_e48218: f64 = (1.0 / assign33120_e48217);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33120_e48218, (-((locals.var_arg__blk1057_dn6 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))), (-((locals.var_arg__blk1057_dn7 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))), (-((locals.var_arg__blk1057_dn12 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 == 0.0)) {
            let assign33130_e48234: f64 = (-p.p184);
            let assign33130_e48235: f64 = (locals.var_arg__blk1057).powf(assign33130_e48234);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33130_e48235, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((locals.var_arg__blk1057).powf(assign33130_e48234 - 1.0) * locals.var_arg__blk1057_dn6)) } } else { (assign33130_e48235 * (assign33130_e48234 * (locals.var_arg__blk1057_dn6 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((locals.var_arg__blk1057).powf(assign33130_e48234 - 1.0) * locals.var_arg__blk1057_dn7)) } } else { (assign33130_e48235 * (assign33130_e48234 * (locals.var_arg__blk1057_dn7 / locals.var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((locals.var_arg__blk1057).powf(assign33130_e48234 - 1.0) * locals.var_arg__blk1057_dn12)) } } else { (assign33130_e48235 * (assign33130_e48234 * (locals.var_arg__blk1057_dn12 / locals.var_arg__blk1057))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1086 != 0.0)) {
            let assign33140_e48249: f64 = (p.p187 * locals.var_czbdswg);
            let assign33140_e48253: f64 = (locals.var_arg__blk1057 * locals.var_sarg);
            let assign33140_e48254: f64 = (1.0 - assign33140_e48253);
            let assign33140_e48255: f64 = (assign33140_e48249 * assign33140_e48254);
            let assign33140_e48258: f64 = (1.0 - p.p184);
            let assign33140_e48259: f64 = (assign33140_e48255 / assign33140_e48258);
            let assign33140_e48260: f64 = (locals.var_qbd + assign33140_e48259);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33140_e48260, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33140_e48249 * (-((locals.var_arg__blk1057_dn6 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn6)))) / assign33140_e48258)), (locals.var_qbd_dn7 + ((assign33140_e48249 * (-((locals.var_arg__blk1057_dn7 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn7)))) / assign33140_e48258)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33140_e48249 * (-((locals.var_arg__blk1057_dn12 * locals.var_sarg) + (locals.var_arg__blk1057 * locals.var_sarg_dn12)))) / assign33140_e48258)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 == 0.0)) {
            let assign33150_e48272: f64 = (locals.var_czbd + locals.var_czbdswg);
            (locals.var_t1__blk1032, locals.var_t1__blk1032_dn6, locals.var_t1__blk1032_dn7, locals.var_t1__blk1032_dn10, locals.var_t1__blk1032_dn12, ) = (assign33150_e48272, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1032_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 == 0.0)) {
            let assign33160_e48284: f64 = (locals.var_czbd * p.p182);
            let assign33160_e48286: f64 = (assign33160_e48284 / p.p185);
            let assign33160_e48289: f64 = (locals.var_czbdswg * p.p184);
            let assign33160_e48291: f64 = (assign33160_e48289 / p.p187);
            let assign33160_e48292: f64 = (assign33160_e48286 + assign33160_e48291);
            (locals.var_t2__blk1033, locals.var_t2__blk1033_dn0, locals.var_t2__blk1033_dn2, locals.var_t2__blk1033_dn6, locals.var_t2__blk1033_dn7, locals.var_t2__blk1033_dn10, locals.var_t2__blk1033_dn11, locals.var_t2__blk1033_dn12, locals.var_t2__blk1033_dn17, ) = (assign33160_e48292, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1033_rv = 0.0;
        }
        if (((locals.var_guard1030 != 0.0) && (locals.var_guard1075 == 0.0)) && (locals.var_guard1083 == 0.0)) {
            let assign33170_e48306: f64 = (locals.var_vbdj * 0.5);
            let assign33170_e48308: f64 = (assign33170_e48306 * locals.var_t2__blk1033);
            let assign33170_e48309: f64 = (locals.var_t1__blk1032 + assign33170_e48308);
            let assign33170_e48310: f64 = (locals.var_vbdj * assign33170_e48309);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33170_e48310, (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn0)), (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn2)), ((locals.var_vbdj_dn6 * assign33170_e48309) + (locals.var_vbdj * (locals.var_t1__blk1032_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1033) + (assign33170_e48306 * locals.var_t2__blk1033_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1032_dn7 + (assign33170_e48306 * locals.var_t2__blk1033_dn7))), (locals.var_vbdj * (locals.var_t1__blk1032_dn10 + (assign33170_e48306 * locals.var_t2__blk1033_dn10))), (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn11)), ((locals.var_vbdj_dn12 * assign33170_e48309) + (locals.var_vbdj * (locals.var_t1__blk1032_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1033) + (assign33170_e48306 * locals.var_t2__blk1033_dn12))))), (locals.var_vbdj * (assign33170_e48306 * locals.var_t2__blk1033_dn17)), );
            locals.var_qbd_rv = 0.0;
        }
        let assign33180_e48315: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1088 = assign33180_e48315;
        locals.var_guard1088_rv = 0.0;
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let assign33190_e48320: f64 = (-1.6021918e-19);
            let assign33190_e48322: f64 = (assign33190_e48320 * locals.var_uc_nsubs);
            let assign33190_e48324: f64 = (assign33190_e48322 * locals.var_xp_max);
            let assign33190_e48326: f64 = (assign33190_e48324 * p.p3);
            (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17, ) = (assign33190_e48326, (((assign33190_e48320 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33190_e48320 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3), );
            locals.var_qbs_max_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let assign33200_e48334: f64 = (-locals.var_qbs_max);
            let assign33200_e48335: f64 = (0.001 * assign33200_e48334);
            (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17, ) = (assign33200_e48335, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)), );
            locals.var_dlt_qbs_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let assign33210_e48342: f64 = (-locals.var_qbs_max);
            let assign33210_e48344: f64 = (-locals.var_qbs);
            let assign33210_e48345: f64 = (assign33210_e48342 - assign33210_e48344);
            let assign33210_e48347: f64 = (assign33210_e48345 - locals.var_dlt_qbs);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33210_e48347, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let assign33220_e48355: f64 = (-locals.var_qbs_max);
            let assign33220_e48356: f64 = (4.0 * assign33220_e48355);
            let assign33220_e48358: f64 = (assign33220_e48356 * locals.var_dlt_qbs);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33220_e48358, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33220_e48356 * locals.var_dlt_qbs_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let (assign33230_e48370, assign33230_e48370_d_n0, assign33230_e48370_d_n2, assign33230_e48370_d_n6, assign33230_e48370_d_n7, assign33230_e48370_d_n10, assign33230_e48370_d_n11, assign33230_e48370_d_n12, assign33230_e48370_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33230_e48369: f64 = (-locals.var_tmf2);
        (assign33230_e48369, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33230_e48370, assign33230_e48370_d_n0, assign33230_e48370_d_n2, assign33230_e48370_d_n6, assign33230_e48370_d_n7, assign33230_e48370_d_n10, assign33230_e48370_d_n11, assign33230_e48370_d_n12, assign33230_e48370_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let assign33240_e48378: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33240_e48380: f64 = (assign33240_e48378 + locals.var_tmf2);
            let assign33240_e48381: f64 = (assign33240_e48380).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33240_e48381, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33240_e48381)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33240_e48381)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let assign33250_e48388: f64 = (-locals.var_qbs_max);
            let assign33250_e48392: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33250_e48393: f64 = (0.5 * assign33250_e48392);
            let assign33250_e48394: f64 = (assign33250_e48388 - assign33250_e48393);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33250_e48394, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbs_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1088 != 0.0)) {
            let assign33260_e48402: f64 = (-1.0);
            let assign33260_e48403: f64 = (locals.var_qbs * assign33260_e48402);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33260_e48403, (locals.var_qbs_dn0 * assign33260_e48402), (locals.var_qbs_dn2 * assign33260_e48402), (locals.var_qbs_dn6 * assign33260_e48402), (locals.var_qbs_dn7 * assign33260_e48402), (locals.var_qbs_dn10 * assign33260_e48402), (locals.var_qbs_dn11 * assign33260_e48402), (locals.var_qbs_dn12 * assign33260_e48402), (locals.var_qbs_dn17 * assign33260_e48402), );
            locals.var_qbs_rv = 0.0;
        }
        let assign33270_e48408: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1089 = assign33270_e48408;
        locals.var_guard1089_rv = 0.0;
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let assign33280_e48413: f64 = (-1.6021918e-19);
            let assign33280_e48415: f64 = (assign33280_e48413 * locals.var_uc_nsubs);
            let assign33280_e48417: f64 = (assign33280_e48415 * locals.var_xp_max);
            let assign33280_e48419: f64 = (assign33280_e48417 * p.p2);
            (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17, ) = (assign33280_e48419, (((assign33280_e48413 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33280_e48413 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2), );
            locals.var_qbd_max_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let assign33290_e48427: f64 = (-locals.var_qbd_max);
            let assign33290_e48428: f64 = (0.001 * assign33290_e48427);
            (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17, ) = (assign33290_e48428, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)), );
            locals.var_dlt_qbd_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let assign33300_e48435: f64 = (-locals.var_qbd_max);
            let assign33300_e48437: f64 = (-locals.var_qbd);
            let assign33300_e48438: f64 = (assign33300_e48435 - assign33300_e48437);
            let assign33300_e48440: f64 = (assign33300_e48438 - locals.var_dlt_qbd);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33300_e48440, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let assign33310_e48448: f64 = (-locals.var_qbd_max);
            let assign33310_e48449: f64 = (4.0 * assign33310_e48448);
            let assign33310_e48451: f64 = (assign33310_e48449 * locals.var_dlt_qbd);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33310_e48451, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33310_e48449 * locals.var_dlt_qbd_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let (assign33320_e48463, assign33320_e48463_d_n0, assign33320_e48463_d_n2, assign33320_e48463_d_n6, assign33320_e48463_d_n7, assign33320_e48463_d_n10, assign33320_e48463_d_n11, assign33320_e48463_d_n12, assign33320_e48463_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33320_e48462: f64 = (-locals.var_tmf2);
        (assign33320_e48462, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33320_e48463, assign33320_e48463_d_n0, assign33320_e48463_d_n2, assign33320_e48463_d_n6, assign33320_e48463_d_n7, assign33320_e48463_d_n10, assign33320_e48463_d_n11, assign33320_e48463_d_n12, assign33320_e48463_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let assign33330_e48471: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33330_e48473: f64 = (assign33330_e48471 + locals.var_tmf2);
            let assign33330_e48474: f64 = (assign33330_e48473).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33330_e48474, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33330_e48474)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33330_e48474)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let assign33340_e48481: f64 = (-locals.var_qbd_max);
            let assign33340_e48485: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33340_e48486: f64 = (0.5 * assign33340_e48485);
            let assign33340_e48487: f64 = (assign33340_e48481 - assign33340_e48486);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33340_e48487, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbd_rv = 0.0;
        }
        if ((locals.var_guard1030 != 0.0) && (locals.var_guard1089 != 0.0)) {
            let assign33350_e48495: f64 = (-1.0);
            let assign33350_e48496: f64 = (locals.var_qbd * assign33350_e48495);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33350_e48496, (locals.var_qbd_dn0 * assign33350_e48495), (locals.var_qbd_dn2 * assign33350_e48495), (locals.var_qbd_dn6 * assign33350_e48495), (locals.var_qbd_dn7 * assign33350_e48495), (locals.var_qbd_dn10 * assign33350_e48495), (locals.var_qbd_dn11 * assign33350_e48495), (locals.var_qbd_dn12 * assign33350_e48495), (locals.var_qbd_dn17 * assign33350_e48495), );
            locals.var_qbd_rv = 0.0;
        }
        let assign33580_e48752: f64 = if ((p.p32 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1122 = assign33580_e48752;
        locals.var_guard1122_rv = 0.0;
        if (locals.var_guard1122 != 0.0) {
            let assign33590_e48756: f64 = (locals.var_psdl - locals.var_ps0);
            let assign33590_e48758: f64 = (assign33590_e48756 / locals.var_lch);
            (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12, locals.var_eyd_dn17, ) = (assign33590_e48758, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn17 - locals.var_ps0_dn17) * locals.var_lch) - (assign33590_e48756 * locals.var_lch_dn17)) / (locals.var_lch * locals.var_lch)), );
            locals.var_eyd_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33600_e48764: f64 = (locals.var_muun * locals.var_eyd);
            let assign33600_e48766: f64 = (assign33600_e48764 / 100000.0);
            (locals.var_t12__blk1106, locals.var_t12__blk1106_dn0, locals.var_t12__blk1106_dn2, locals.var_t12__blk1106_dn6, locals.var_t12__blk1106_dn7, locals.var_t12__blk1106_dn10, locals.var_t12__blk1106_dn11, locals.var_t12__blk1106_dn12, locals.var_t12__blk1106_dn17, ) = (assign33600_e48766, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 100000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 100000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 100000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 100000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 100000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 100000.0), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / 100000.0), (((locals.var_muun_dn17 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn17)) / 100000.0), );
            locals.var_t12__blk1106_rv = 0.0;
        }
        let assign33610_e48772: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48773: f64 = (1.0 - assign33610_e48772);
        let assign33610_e48780: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48781: f64 = (1.0 + assign33610_e48780);
        let assign33610_e48783: f64 = if ((assign33610_e48773 <= p.p113) && (p.p113 <= assign33610_e48781)) { 1.0 } else { 0.0 };
        locals.var_guard1123 = assign33610_e48783;
        locals.var_guard1123_rv = 0.0;
        if ((locals.var_guard1122 != 0.0) && (locals.var_guard1123 != 0.0)) {
            (locals.var_t7__blk1107, locals.var_t7__blk1107_dn0, locals.var_t7__blk1107_dn2, locals.var_t7__blk1107_dn6, locals.var_t7__blk1107_dn7, locals.var_t7__blk1107_dn10, locals.var_t7__blk1107_dn11, locals.var_t7__blk1107_dn12, locals.var_t7__blk1107_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t7__blk1107_rv = 0.0;
        }
        let assign33630_e48793: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48794: f64 = (2.0 - assign33630_e48793);
        let assign33630_e48801: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48802: f64 = (2.0 + assign33630_e48801);
        let assign33630_e48804: f64 = if ((assign33630_e48794 <= p.p113) && (p.p113 <= assign33630_e48802)) { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign33630_e48804;
        locals.var_guard1124_rv = 0.0;
        if (((locals.var_guard1122 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
            (locals.var_t7__blk1107, locals.var_t7__blk1107_dn0, locals.var_t7__blk1107_dn2, locals.var_t7__blk1107_dn6, locals.var_t7__blk1107_dn7, locals.var_t7__blk1107_dn10, locals.var_t7__blk1107_dn11, locals.var_t7__blk1107_dn12, locals.var_t7__blk1107_dn17, ) = (locals.var_t12__blk1106, locals.var_t12__blk1106_dn0, locals.var_t12__blk1106_dn2, locals.var_t12__blk1106_dn6, locals.var_t12__blk1106_dn7, locals.var_t12__blk1106_dn10, locals.var_t12__blk1106_dn11, locals.var_t12__blk1106_dn12, locals.var_t12__blk1106_dn17, );
            locals.var_t7__blk1107_rv = 0.0;
        }
        if (((locals.var_guard1122 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
            let assign33650_e48824: f64 = (p.p113 - 1.0);
            let assign33650_e48825: f64 = (locals.var_t12__blk1106).powf(assign33650_e48824);
            (locals.var_t7__blk1107, locals.var_t7__blk1107_dn0, locals.var_t7__blk1107_dn2, locals.var_t7__blk1107_dn6, locals.var_t7__blk1107_dn7, locals.var_t7__blk1107_dn10, locals.var_t7__blk1107_dn11, locals.var_t7__blk1107_dn12, locals.var_t7__blk1107_dn17, ) = (assign33650_e48825, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn0)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn0 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn2)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn2 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn6)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn6 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn7)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn7 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn10)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn10 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn11)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn11 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn12)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn12 / locals.var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((locals.var_t12__blk1106).powf(assign33650_e48824 - 1.0) * locals.var_t12__blk1106_dn17)) } } else { (assign33650_e48825 * (assign33650_e48824 * (locals.var_t12__blk1106_dn17 / locals.var_t12__blk1106))) }, );
            locals.var_t7__blk1107_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33660_e48831: f64 = (locals.var_t12__blk1106 * locals.var_t7__blk1107);
            (locals.var_t8__blk1108, locals.var_t8__blk1108_dn0, locals.var_t8__blk1108_dn2, locals.var_t8__blk1108_dn6, locals.var_t8__blk1108_dn7, locals.var_t8__blk1108_dn10, locals.var_t8__blk1108_dn11, locals.var_t8__blk1108_dn12, locals.var_t8__blk1108_dn17, ) = (assign33660_e48831, ((locals.var_t12__blk1106_dn0 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn0)), ((locals.var_t12__blk1106_dn2 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn2)), ((locals.var_t12__blk1106_dn6 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn6)), ((locals.var_t12__blk1106_dn7 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn7)), ((locals.var_t12__blk1106_dn10 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn10)), ((locals.var_t12__blk1106_dn11 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn11)), ((locals.var_t12__blk1106_dn12 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn12)), ((locals.var_t12__blk1106_dn17 * locals.var_t7__blk1107) + (locals.var_t12__blk1106 * locals.var_t7__blk1107_dn17)), );
            locals.var_t8__blk1108_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33670_e48837: f64 = (1.0 + locals.var_t8__blk1108);
            (locals.var_t9__blk1109, locals.var_t9__blk1109_dn0, locals.var_t9__blk1109_dn2, locals.var_t9__blk1109_dn6, locals.var_t9__blk1109_dn7, locals.var_t9__blk1109_dn10, locals.var_t9__blk1109_dn11, locals.var_t9__blk1109_dn12, locals.var_t9__blk1109_dn17, ) = (assign33670_e48837, locals.var_t8__blk1108_dn0, locals.var_t8__blk1108_dn2, locals.var_t8__blk1108_dn6, locals.var_t8__blk1108_dn7, locals.var_t8__blk1108_dn10, locals.var_t8__blk1108_dn11, locals.var_t8__blk1108_dn12, locals.var_t8__blk1108_dn17, );
            locals.var_t9__blk1109_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33680_e48843: f64 = (-1.0);
            let assign33680_e48845: f64 = (assign33680_e48843 / p.p113);
            let assign33680_e48847: f64 = (assign33680_e48845 - 1.0);
            let assign33680_e48848: f64 = (locals.var_t9__blk1109).powf(assign33680_e48847);
            (locals.var_t10__blk1110, locals.var_t10__blk1110_dn0, locals.var_t10__blk1110_dn2, locals.var_t10__blk1110_dn6, locals.var_t10__blk1110_dn7, locals.var_t10__blk1110_dn10, locals.var_t10__blk1110_dn11, locals.var_t10__blk1110_dn12, locals.var_t10__blk1110_dn17, ) = (assign33680_e48848, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn0)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn0 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn2)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn2 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn6)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn6 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn7)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn7 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn10)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn10 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn11)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn11 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn12)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn12 / locals.var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((locals.var_t9__blk1109).powf(assign33680_e48847 - 1.0) * locals.var_t9__blk1109_dn17)) } } else { (assign33680_e48848 * (assign33680_e48847 * (locals.var_t9__blk1109_dn17 / locals.var_t9__blk1109))) }, );
            locals.var_t10__blk1110_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33690_e48854: f64 = (locals.var_t9__blk1109 * locals.var_t10__blk1110);
            (locals.var_t11__blk1111, locals.var_t11__blk1111_dn0, locals.var_t11__blk1111_dn2, locals.var_t11__blk1111_dn6, locals.var_t11__blk1111_dn7, locals.var_t11__blk1111_dn10, locals.var_t11__blk1111_dn11, locals.var_t11__blk1111_dn12, locals.var_t11__blk1111_dn17, ) = (assign33690_e48854, ((locals.var_t9__blk1109_dn0 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn0)), ((locals.var_t9__blk1109_dn2 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn2)), ((locals.var_t9__blk1109_dn6 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn6)), ((locals.var_t9__blk1109_dn7 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn7)), ((locals.var_t9__blk1109_dn10 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn10)), ((locals.var_t9__blk1109_dn11 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn11)), ((locals.var_t9__blk1109_dn12 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn12)), ((locals.var_t9__blk1109_dn17 * locals.var_t10__blk1110) + (locals.var_t9__blk1109 * locals.var_t10__blk1110_dn17)), );
            locals.var_t11__blk1111_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33700_e48860: f64 = (locals.var_muun * locals.var_t11__blk1111);
            (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12, locals.var_mud_hoso_dn17, ) = (assign33700_e48860, ((locals.var_muun_dn0 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn0)), ((locals.var_muun_dn2 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn2)), ((locals.var_muun_dn6 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn6)), ((locals.var_muun_dn7 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn7)), ((locals.var_muun_dn10 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn10)), ((locals.var_muun_dn11 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn11)), ((locals.var_muun_dn12 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn12)), ((locals.var_muun_dn17 * locals.var_t11__blk1111) + (locals.var_muun * locals.var_t11__blk1111_dn17)), );
            locals.var_mud_hoso_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33710_e48866: f64 = (locals.var_mu + locals.var_mud_hoso);
            let assign33710_e48868: f64 = (assign33710_e48866 / 2.0);
            (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12, locals.var_mu_ave_dn17, ) = (assign33710_e48868, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0), ((locals.var_mu_dn17 + locals.var_mud_hoso_dn17) / 2.0), );
            locals.var_mu_ave_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33720_e48874: f64 = (locals.var_alpha * locals.var_alpha);
            (locals.var_t0__blk1112, locals.var_t0__blk1112_dn0, locals.var_t0__blk1112_dn2, locals.var_t0__blk1112_dn6, locals.var_t0__blk1112_dn7, locals.var_t0__blk1112_dn10, locals.var_t0__blk1112_dn11, locals.var_t0__blk1112_dn12, locals.var_t0__blk1112_dn17, ) = (assign33720_e48874, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn17)), );
            locals.var_t0__blk1112_rv = 0.0;
        }
        if (locals.var_guard1122 != 0.0) {
            let assign33730_e48880: f64 = (locals.var_weff_nf * locals.var_c_fox);
            let assign33730_e48882: f64 = (assign33730_e48880 * locals.var_vgvt);
            let assign33730_e48884: f64 = (assign33730_e48882 * locals.var_mu);
            let assign33730_e48888: f64 = (3.0 * locals.var_alpha);
            let assign33730_e48889: f64 = (1.0 + assign33730_e48888);
            let assign33730_e48892: f64 = (6.0 * locals.var_t0__blk1112);
            let assign33730_e48893: f64 = (assign33730_e48889 + assign33730_e48892);
            let assign33730_e48895: f64 = (assign33730_e48893 * locals.var_mud_hoso);
            let assign33730_e48897: f64 = (assign33730_e48895 * locals.var_mud_hoso);
            let assign33730_e48901: f64 = (4.0 * locals.var_alpha);
            let assign33730_e48902: f64 = (3.0 + assign33730_e48901);
            let assign33730_e48905: f64 = (3.0 * locals.var_t0__blk1112);
            let assign33730_e48906: f64 = (assign33730_e48902 + assign33730_e48905);
            let assign33730_e48908: f64 = (assign33730_e48906 * locals.var_mud_hoso);
            let assign33730_e48910: f64 = (assign33730_e48908 * locals.var_mu);
            let assign33730_e48911: f64 = (assign33730_e48897 + assign33730_e48910);
            let assign33730_e48915: f64 = (3.0 * locals.var_alpha);
            let assign33730_e48916: f64 = (6.0 + assign33730_e48915);
            let assign33730_e48918: f64 = (assign33730_e48916 + locals.var_t0__blk1112);
            let assign33730_e48920: f64 = (assign33730_e48918 * locals.var_mu);
            let assign33730_e48922: f64 = (assign33730_e48920 * locals.var_mu);
            let assign33730_e48923: f64 = (assign33730_e48911 + assign33730_e48922);
            let assign33730_e48924: f64 = (assign33730_e48884 * assign33730_e48923);
            let assign33730_e48927: f64 = (15.0 * locals.var_lch);
            let assign33730_e48930: f64 = (1.0 + locals.var_alpha);
            let assign33730_e48931: f64 = (assign33730_e48927 * assign33730_e48930);
            let assign33730_e48933: f64 = (assign33730_e48931 * locals.var_mu_ave);
            let assign33730_e48935: f64 = (assign33730_e48933 * locals.var_mu_ave);
            let assign33730_e48936: f64 = (assign33730_e48924 / assign33730_e48935);
            (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17, ) = (assign33730_e48936, ((((((((((locals.var_weff_nf * locals.var_c_fox_dn0) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn0)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0__blk1112_dn0)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0__blk1112_dn0)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0__blk1112_dn0) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn0)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn0))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn0) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn0)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn2) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn2)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0__blk1112_dn2)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0__blk1112_dn2)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0__blk1112_dn2) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn2)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn2))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn2) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn2)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn6) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn6)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0__blk1112_dn6)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0__blk1112_dn6)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0__blk1112_dn6) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn6)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn6))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn6) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn6)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn7) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn7)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0__blk1112_dn7)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0__blk1112_dn7)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0__blk1112_dn7) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn7)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn7))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn7) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn7)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn10) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn10)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0__blk1112_dn10)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0__blk1112_dn10)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0__blk1112_dn10) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn10)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn10))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn10) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn10)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn11) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn11)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0__blk1112_dn11)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0__blk1112_dn11)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0__blk1112_dn11) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn11)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn11))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn11) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn11)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn12) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn12)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0__blk1112_dn12)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0__blk1112_dn12)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0__blk1112_dn12) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn12)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn12))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn12) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn12)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((locals.var_weff_nf * locals.var_c_fox_dn17) * locals.var_vgvt) + (assign33730_e48880 * locals.var_vgvt_dn17)) * locals.var_mu) + (assign33730_e48882 * locals.var_mu_dn17)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * locals.var_alpha_dn17) + (6.0 * locals.var_t0__blk1112_dn17)) * locals.var_mud_hoso) + (assign33730_e48893 * locals.var_mud_hoso_dn17)) * locals.var_mud_hoso) + (assign33730_e48895 * locals.var_mud_hoso_dn17)) + ((((((4.0 * locals.var_alpha_dn17) + (3.0 * locals.var_t0__blk1112_dn17)) * locals.var_mud_hoso) + (assign33730_e48906 * locals.var_mud_hoso_dn17)) * locals.var_mu) + (assign33730_e48908 * locals.var_mu_dn17))) + ((((((3.0 * locals.var_alpha_dn17) + locals.var_t0__blk1112_dn17) * locals.var_mu) + (assign33730_e48918 * locals.var_mu_dn17)) * locals.var_mu) + (assign33730_e48920 * locals.var_mu_dn17))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * locals.var_lch_dn17) * assign33730_e48930) + (assign33730_e48927 * locals.var_alpha_dn17)) * locals.var_mu_ave) + (assign33730_e48931 * locals.var_mu_ave_dn17)) * locals.var_mu_ave) + (assign33730_e48933 * locals.var_mu_ave_dn17)))) / (assign33730_e48935 * assign33730_e48935)), );
            locals.var_nthrml_rv = 0.0;
        }
        if (locals.var_guard1122 == 0.0) {
            (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, locals.var_nthrml_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_nthrml_rv = 0.0;
        }
        let assign33750_e48957: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign33750_e48957;
        locals.var_guard1125_rv = 0.0;
        if (locals.var_guard1125 != 0.0) {
            let assign33760_e48960: f64 = (locals.var_kusail).sqrt();
            (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12, locals.var_sqrtkusail_dn17, ) = (assign33760_e48960, (locals.var_kusail_dn0 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn2 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn6 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn7 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn10 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn11 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn12 / (2.0 * assign33760_e48960)), (locals.var_kusail_dn17 / (2.0 * assign33760_e48960)), );
            locals.var_sqrtkusail_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33770_e48966: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
            (locals.var_t2__blk1114, locals.var_t2__blk1114_dn0, locals.var_t2__blk1114_dn2, locals.var_t2__blk1114_dn6, locals.var_t2__blk1114_dn7, locals.var_t2__blk1114_dn10, locals.var_t2__blk1114_dn11, locals.var_t2__blk1114_dn12, locals.var_t2__blk1114_dn17, ) = (assign33770_e48966, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12), (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17), );
            locals.var_t2__blk1114_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33780_e48972: f64 = (locals.var_kusai00 * locals.var_kusai00);
            (locals.var_t3__blk1115, locals.var_t3__blk1115_dn0, locals.var_t3__blk1115_dn2, locals.var_t3__blk1115_dn6, locals.var_t3__blk1115_dn7, locals.var_t3__blk1115_dn10, locals.var_t3__blk1115_dn11, locals.var_t3__blk1115_dn12, locals.var_t3__blk1115_dn17, ) = (assign33780_e48972, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)), ((locals.var_kusai00_dn17 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn17)), );
            locals.var_t3__blk1115_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33790_e48978: f64 = (locals.var_kusail * locals.var_kusail);
            (locals.var_t4__blk1116, locals.var_t4__blk1116_dn0, locals.var_t4__blk1116_dn2, locals.var_t4__blk1116_dn6, locals.var_t4__blk1116_dn7, locals.var_t4__blk1116_dn10, locals.var_t4__blk1116_dn11, locals.var_t4__blk1116_dn12, locals.var_t4__blk1116_dn17, ) = (assign33790_e48978, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)), ((locals.var_kusail_dn17 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn17)), );
            locals.var_t4__blk1116_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33800_e48984: f64 = (42.0 * locals.var_kusai00);
            let assign33800_e48986: f64 = (assign33800_e48984 * locals.var_kusail);
            (locals.var_t5__blk1117, locals.var_t5__blk1117_dn0, locals.var_t5__blk1117_dn2, locals.var_t5__blk1117_dn6, locals.var_t5__blk1117_dn7, locals.var_t5__blk1117_dn10, locals.var_t5__blk1117_dn11, locals.var_t5__blk1117_dn12, locals.var_t5__blk1117_dn17, ) = (assign33800_e48986, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn12)), (((42.0 * locals.var_kusai00_dn17) * locals.var_kusail) + (assign33800_e48984 * locals.var_kusail_dn17)), );
            locals.var_t5__blk1117_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33810_e48994: f64 = (locals.var_t3__blk1115 + locals.var_t4__blk1116);
            let assign33810_e48995: f64 = (4.0 * assign33810_e48994);
            let assign33810_e48996: f64 = (locals.var_t5__blk1117 + assign33810_e48995);
            (locals.var_t5__blk1117, locals.var_t5__blk1117_dn0, locals.var_t5__blk1117_dn2, locals.var_t5__blk1117_dn6, locals.var_t5__blk1117_dn7, locals.var_t5__blk1117_dn10, locals.var_t5__blk1117_dn11, locals.var_t5__blk1117_dn12, locals.var_t5__blk1117_dn17, ) = (assign33810_e48996, (locals.var_t5__blk1117_dn0 + (4.0 * (locals.var_t3__blk1115_dn0 + locals.var_t4__blk1116_dn0))), (locals.var_t5__blk1117_dn2 + (4.0 * (locals.var_t3__blk1115_dn2 + locals.var_t4__blk1116_dn2))), (locals.var_t5__blk1117_dn6 + (4.0 * (locals.var_t3__blk1115_dn6 + locals.var_t4__blk1116_dn6))), (locals.var_t5__blk1117_dn7 + (4.0 * (locals.var_t3__blk1115_dn7 + locals.var_t4__blk1116_dn7))), (locals.var_t5__blk1117_dn10 + (4.0 * (locals.var_t3__blk1115_dn10 + locals.var_t4__blk1116_dn10))), (locals.var_t5__blk1117_dn11 + (4.0 * (locals.var_t3__blk1115_dn11 + locals.var_t4__blk1116_dn11))), (locals.var_t5__blk1117_dn12 + (4.0 * (locals.var_t3__blk1115_dn12 + locals.var_t4__blk1116_dn12))), (locals.var_t5__blk1117_dn17 + (4.0 * (locals.var_t3__blk1115_dn17 + locals.var_t4__blk1116_dn17))), );
            locals.var_t5__blk1117_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33820_e49003: f64 = (20.0 * locals.var_sqrtkusail);
            let assign33820_e49005: f64 = (assign33820_e49003 * locals.var_vgvt);
            let assign33820_e49008: f64 = (locals.var_kusai00 + locals.var_kusail);
            let assign33820_e49009: f64 = (assign33820_e49005 * assign33820_e49008);
            let assign33820_e49010: f64 = (locals.var_t5__blk1117 + assign33820_e49009);
            (locals.var_t5__blk1117, locals.var_t5__blk1117_dn0, locals.var_t5__blk1117_dn2, locals.var_t5__blk1117_dn6, locals.var_t5__blk1117_dn7, locals.var_t5__blk1117_dn10, locals.var_t5__blk1117_dn11, locals.var_t5__blk1117_dn12, locals.var_t5__blk1117_dn17, ) = (assign33820_e49010, (locals.var_t5__blk1117_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn0)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5__blk1117_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn2)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5__blk1117_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn6)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5__blk1117_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn7)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5__blk1117_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn10)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5__blk1117_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn11)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5__blk1117_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn12)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))), (locals.var_t5__blk1117_dn17 + (((((20.0 * locals.var_sqrtkusail_dn17) * locals.var_vgvt) + (assign33820_e49003 * locals.var_vgvt_dn17)) * assign33820_e49008) + (assign33820_e49005 * (locals.var_kusai00_dn17 + locals.var_kusail_dn17)))), );
            locals.var_t5__blk1117_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33830_e49016: f64 = (locals.var_t2__blk1114 * locals.var_t2__blk1114);
            (locals.var_t10w, locals.var_t10w_dn0, locals.var_t10w_dn2, locals.var_t10w_dn6, locals.var_t10w_dn7, locals.var_t10w_dn10, locals.var_t10w_dn11, locals.var_t10w_dn12, locals.var_t10w_dn17, ) = (assign33830_e49016, ((locals.var_t2__blk1114_dn0 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn0)), ((locals.var_t2__blk1114_dn2 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn2)), ((locals.var_t2__blk1114_dn6 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn6)), ((locals.var_t2__blk1114_dn7 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn7)), ((locals.var_t2__blk1114_dn10 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn10)), ((locals.var_t2__blk1114_dn11 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn11)), ((locals.var_t2__blk1114_dn12 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn12)), ((locals.var_t2__blk1114_dn17 * locals.var_t2__blk1114) + (locals.var_t2__blk1114 * locals.var_t2__blk1114_dn17)), );
            locals.var_t10w_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33840_e49022: f64 = (locals.var_t10w * locals.var_t10w);
            (locals.var_t10__blk1110, locals.var_t10__blk1110_dn0, locals.var_t10__blk1110_dn2, locals.var_t10__blk1110_dn6, locals.var_t10__blk1110_dn7, locals.var_t10__blk1110_dn10, locals.var_t10__blk1110_dn11, locals.var_t10__blk1110_dn12, locals.var_t10__blk1110_dn17, ) = (assign33840_e49022, ((locals.var_t10w_dn0 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn0)), ((locals.var_t10w_dn2 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn2)), ((locals.var_t10w_dn6 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn6)), ((locals.var_t10w_dn7 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn7)), ((locals.var_t10w_dn10 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn10)), ((locals.var_t10w_dn11 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn11)), ((locals.var_t10w_dn12 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn12)), ((locals.var_t10w_dn17 * locals.var_t10w) + (locals.var_t10w * locals.var_t10w_dn17)), );
            locals.var_t10__blk1110_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33850_e49029: f64 = (locals.var_t10__blk1110 * locals.var_t2__blk1114);
            let assign33850_e49030: f64 = (locals.var_t5__blk1117 / assign33850_e49029);
            (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12, locals.var_kusai_ig_dn17, ) = (assign33850_e49030, (((locals.var_t5__blk1117_dn0 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn0 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn0)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn2 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn2 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn2)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn6 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn6 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn6)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn7 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn7 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn7)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn10 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn10 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn10)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn11 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn11 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn11)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn12 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn12 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn12)))) / (assign33850_e49029 * assign33850_e49029)), (((locals.var_t5__blk1117_dn17 * assign33850_e49029) - (locals.var_t5__blk1117 * ((locals.var_t10__blk1110_dn17 * locals.var_t2__blk1114) + (locals.var_t10__blk1110 * locals.var_t2__blk1114_dn17)))) / (assign33850_e49029 * assign33850_e49029)), );
            locals.var_kusai_ig_rv = 0.0;
        }
        if (locals.var_guard1125 != 0.0) {
            let assign33860_e49036: f64 = (locals.var_weff_nf / locals.var_lch);
            let assign33860_e49038: f64 = (assign33860_e49036 * locals.var_mu);
            let assign33860_e49040: f64 = (assign33860_e49038 * locals.var_c_fox);
            (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12, locals.var_gds0_ign_dn17, ) = (assign33860_e49040, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn7)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn12) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn12)), (((((-((locals.var_weff_nf * locals.var_lch_dn17) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign33860_e49036 * locals.var_mu_dn17)) * locals.var_c_fox) + (assign33860_e49038 * locals.var_c_fox_dn17)), );
            locals.var_gds0_ign_rv = 0.0;
        }
        let assign33910_e49090: f64 = (locals.var_ids + locals.var_idsibpc);
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign33910_e49090, (locals.var_ids_dn0 + locals.var_idsibpc_dn0), (locals.var_ids_dn2 + locals.var_idsibpc_dn2), (locals.var_ids_dn6 + locals.var_idsibpc_dn6), (locals.var_ids_dn7 + locals.var_idsibpc_dn7), (locals.var_ids_dn10 + locals.var_idsibpc_dn10), (locals.var_ids_dn11 + locals.var_idsibpc_dn11), (locals.var_ids_dn12 + locals.var_idsibpc_dn12), (locals.var_ids_dn17 + locals.var_idsibpc_dn17), );
        locals.var_ids_rv = 0.0;
        let assign33920_e49093: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign33920_e49093;
        locals.var_guard1126_rv = 0.0;
        if (locals.var_guard1126 != 0.0) {
            let assign33930_e49097: f64 = (locals.var_cbtp + locals.var_cbtn);
            locals.var_cgbe = assign33930_e49097;
            locals.var_cgbe_rv = 0.0;
        }
        if ((locals.var_guard1126 != 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign33940_e49106: f64 = (p.p168 * locals.var_lgleff);
            let assign33940_e49107: f64 = (locals.var_cgbe - assign33940_e49106);
            locals.var_cgbe = assign33940_e49107;
            locals.var_cgbe_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign33950_e49112: f64 = (-locals.var_cgbe);
            let assign33950_e49115: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign33950_e49116: f64 = (assign33950_e49112 * assign33950_e49115);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign33950_e49116, (assign33950_e49112 * (-locals.var_vbsp_dn0)), (assign33950_e49112 * (-locals.var_vbsp_dn2)), (assign33950_e49112 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33950_e49112 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33950_e49112 * (-locals.var_vbsp_dn10)), (assign33950_e49112 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33950_e49112 * (-locals.var_vbsp_dn12)), (assign33950_e49112 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            locals.var_cfu = 0.0;
            locals.var_cfu_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign33970_e49132: f64 = (locals.var_cfu * p.p9);
            let assign33970_e49135: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
            let assign33970_e49136: f64 = (assign33970_e49132 * assign33970_e49135);
            locals.var_cfd = assign33970_e49136;
            locals.var_cfd_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign33980_e49142: f64 = (locals.var_cfu * p.p9);
            let assign33980_e49145: f64 = (locals.var_wgate + locals.var_uc_psbcp);
            let assign33980_e49146: f64 = (assign33980_e49142 * assign33980_e49145);
            locals.var_cfs = assign33980_e49146;
            locals.var_cfs_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign33990_e49153: f64 = (locals.var_vgs - locals.var_vds);
            let assign33990_e49154: f64 = (locals.var_cfd * assign33990_e49153);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign33990_e49154, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign34000_e49160: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign34000_e49160, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign34010_e49166: f64 = (locals.var_cfu * p.p19);
            let assign34010_e49168: f64 = (assign34010_e49166 * p.p9);
            let assign34010_e49171: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign34010_e49172: f64 = (assign34010_e49168 * assign34010_e49171);
            (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17, ) = (assign34010_e49172, (assign34010_e49168 * (-locals.var_vbsp_dn0)), (assign34010_e49168 * (-locals.var_vbsp_dn2)), (assign34010_e49168 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34010_e49168 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34010_e49168 * (-locals.var_vbsp_dn10)), (assign34010_e49168 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34010_e49168 * (-locals.var_vbsp_dn12)), (assign34010_e49168 * (-locals.var_vbsp_dn17)), );
            locals.var_qfbc_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign34020_e49178: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34020_e49178, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign34030_e49184: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34030_e49184, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }
        if (locals.var_guard1126 != 0.0) {
            let assign34040_e49190: f64 = (locals.var_qgob + locals.var_qfbc);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34040_e49190, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17), );
            locals.var_qgob_rv = 0.0;
        }
        if ((locals.var_guard1126 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34050_e49198: f64 = (-p.p168);
            let assign34050_e49200: f64 = (assign34050_e49198 * locals.var_lgleff);
            locals.var_cgbe = assign34050_e49200;
            locals.var_cgbe_rv = 0.0;
        }
        if ((locals.var_guard1126 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34060_e49208: f64 = (-locals.var_cgbe);
            let assign34060_e49211: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign34060_e49212: f64 = (assign34060_e49208 * assign34060_e49211);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34060_e49212, (assign34060_e49208 * (-locals.var_vbsp_dn0)), (assign34060_e49208 * (-locals.var_vbsp_dn2)), (assign34060_e49208 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34060_e49208 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34060_e49208 * (-locals.var_vbsp_dn10)), (assign34060_e49208 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34060_e49208 * (-locals.var_vbsp_dn12)), (assign34060_e49208 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }
        if ((locals.var_guard1126 == 0.0) && (locals.var_cgbo_given == 0.0)) {
            locals.var_cgbe = 0.0;
            locals.var_cgbe_rv = 0.0;
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qgob_rv = 0.0;
        }
        if (locals.var_guard1126 == 0.0) {
            locals.var_cf = 0.0;
            locals.var_cf_rv = 0.0;
            locals.var_cfd = locals.var_cf;
            locals.var_cfd_rv = 0.0;
            locals.var_cfs = locals.var_cf;
            locals.var_cfs_rv = 0.0;
        }
        if (locals.var_guard1126 == 0.0) {
            let assign34120_e49261: f64 = (locals.var_vgs - locals.var_vds);
            let assign34120_e49262: f64 = (locals.var_cfd * assign34120_e49261);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign34120_e49262, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }
        if (locals.var_guard1126 == 0.0) {
            let assign34130_e49269: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign34130_e49269, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }
        if (locals.var_guard1126 == 0.0) {
            let assign34140_e49276: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34140_e49276, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }
        if (locals.var_guard1126 == 0.0) {
            let assign34150_e49283: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34150_e49283, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }
        let assign34160_e49288: f64 = (locals.var_mfactor * locals.var_ids);
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (assign34160_e49288, (locals.var_mfactor * locals.var_ids_dn0), (locals.var_mfactor * locals.var_ids_dn2), (locals.var_mfactor * locals.var_ids_dn6), (locals.var_mfactor * locals.var_ids_dn7), (locals.var_mfactor * locals.var_ids_dn10), (locals.var_mfactor * locals.var_ids_dn11), (locals.var_mfactor * locals.var_ids_dn12), (locals.var_mfactor * locals.var_ids_dn17), );
        locals.var_idse_rv = 0.0;
        if (locals.var_flg_nqs != 0.0) {
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qde_rv = 0.0;
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qge_rv = 0.0;
        }
        let assign34190_e49299: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1127 = assign34190_e49299;
        locals.var_guard1127_rv = 0.0;
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1127 != 0.0)) {
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qse_rv = 0.0;
            (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17, ) = (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, );
            locals.var_xd_rv = 0.0;
        }
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1127 == 0.0)) {
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbe_rv = 0.0;
        }
        let assign34280_e49370: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1128 = assign34280_e49370;
        locals.var_guard1128_rv = 0.0;
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 != 0.0)) {
            let assign34290_e49377: f64 = (-locals.var_qb);
            let assign34290_e49379: f64 = (assign34290_e49377 - locals.var_qi);
            let assign34290_e49380: f64 = (locals.var_mfactor * assign34290_e49379);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34290_e49380, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)), );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 != 0.0)) {
            let assign34300_e49389: f64 = (locals.var_mfactor * locals.var_qd);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34300_e49389, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18), );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 != 0.0)) {
            let assign34310_e49399: f64 = (locals.var_qi - locals.var_qd);
            let assign34310_e49400: f64 = (locals.var_mfactor * assign34310_e49399);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34310_e49400, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)), );
            locals.var_qse_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 == 0.0)) {
            let assign34320_e49410: f64 = (-locals.var_qsub);
            let assign34320_e49412: f64 = (assign34320_e49410 - locals.var_qi);
            let assign34320_e49414: f64 = (assign34320_e49412 - locals.var_qs_fb);
            let assign34320_e49416: f64 = (assign34320_e49414 - locals.var_qd_fb);
            let assign34320_e49417: f64 = (locals.var_mfactor * assign34320_e49416);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34320_e49417, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)), );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 == 0.0)) {
            let assign34330_e49428: f64 = (locals.var_qd + locals.var_qd_fb);
            let assign34330_e49429: f64 = (locals.var_mfactor * assign34330_e49428);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34330_e49429, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)), );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1128 == 0.0)) {
            let assign34340_e49440: f64 = (locals.var_qi - locals.var_qd);
            let assign34340_e49442: f64 = (assign34340_e49440 + locals.var_qs_fb);
            let assign34340_e49443: f64 = (locals.var_mfactor * assign34340_e49442);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34340_e49443, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)), );
            locals.var_qse_rv = 0.0;
        }
        let assign34350_e49448: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign34350_e49448;
        locals.var_guard1134_rv = 0.0;
        if (locals.var_guard1134 != 0.0) {
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
        if (locals.var_guard1134 == 0.0) {
            let assign34370_e49457: f64 = (locals.var_ec * locals.var_leff);
            let assign34370_e49459: f64 = (assign34370_e49457 + locals.var_ps0);
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (assign34370_e49459, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17), );
            locals.var_pslk_rv = 0.0;
        }
        let assign34380_e49464: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign34380_e49464;
        locals.var_guard1135_rv = 0.0;
        if ((locals.var_guard1134 == 0.0) && (locals.var_guard1135 != 0.0)) {
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17, );
            locals.var_pslk_rv = 0.0;
        }
        if (locals.var_guard1134 == 0.0) {
            let assign34400_e49477: f64 = (locals.var_vds + locals.var_ps0);
            let assign34400_e49478: f64 = (locals.var_aclm * assign34400_e49477);
            let assign34400_e49481: f64 = (1.0 - locals.var_aclm);
            let assign34400_e49483: f64 = (assign34400_e49481 * locals.var_pslk);
            let assign34400_e49484: f64 = (assign34400_e49478 + assign34400_e49483);
            (locals.var_t1__blk1130, locals.var_t1__blk1130_dn0, locals.var_t1__blk1130_dn2, locals.var_t1__blk1130_dn6, locals.var_t1__blk1130_dn7, locals.var_t1__blk1130_dn10, locals.var_t1__blk1130_dn11, locals.var_t1__blk1130_dn12, locals.var_t1__blk1130_dn17, ) = (assign34400_e49484, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34400_e49481 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34400_e49481 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34400_e49481 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34400_e49481 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34400_e49481 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34400_e49481 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34400_e49481 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34400_e49481 * locals.var_pslk_dn17)), );
            locals.var_t1__blk1130_rv = 0.0;
        }
        if (locals.var_guard1134 == 0.0) {
            let assign34410_e49491: f64 = (2.0 * 1.034943e-10);
            let assign34410_e49493: f64 = (assign34410_e49491 / locals.var_q_nsub);
            let assign34410_e49494: f64 = (assign34410_e49493).sqrt();
            (locals.var_t10__blk1131, locals.var_t10__blk1131_dn0, locals.var_t10__blk1131_dn2, locals.var_t10__blk1131_dn6, locals.var_t10__blk1131_dn7, locals.var_t10__blk1131_dn10, locals.var_t10__blk1131_dn11, locals.var_t10__blk1131_dn12, locals.var_t10__blk1131_dn17, ) = (assign34410_e49494, ((-((assign34410_e49491 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34410_e49494)), );
            locals.var_t10__blk1131_rv = 0.0;
        }
        if (locals.var_guard1134 == 0.0) {
            let assign34420_e49501: f64 = (locals.var_t10__blk1131 * 1.3);
            (locals.var_t3__blk1132, locals.var_t3__blk1132_dn0, locals.var_t3__blk1132_dn2, locals.var_t3__blk1132_dn6, locals.var_t3__blk1132_dn7, locals.var_t3__blk1132_dn10, locals.var_t3__blk1132_dn11, locals.var_t3__blk1132_dn12, locals.var_t3__blk1132_dn17, ) = (assign34420_e49501, (locals.var_t10__blk1131_dn0 * 1.3), (locals.var_t10__blk1131_dn2 * 1.3), (locals.var_t10__blk1131_dn6 * 1.3), (locals.var_t10__blk1131_dn7 * 1.3), (locals.var_t10__blk1131_dn10 * 1.3), (locals.var_t10__blk1131_dn11 * 1.3), (locals.var_t10__blk1131_dn12 * 1.3), (locals.var_t10__blk1131_dn17 * 1.3), );
            locals.var_t3__blk1132_rv = 0.0;
        }
        if (locals.var_guard1134 == 0.0) {
            let assign34430_e49508: f64 = (1.034943e-10 * locals.var_weffcv_nf);
            let assign34430_e49510: f64 = (assign34430_e49508 * locals.var_t3__blk1132);
            (locals.var_t2__blk1133, locals.var_t2__blk1133_dn0, locals.var_t2__blk1133_dn2, locals.var_t2__blk1133_dn6, locals.var_t2__blk1133_dn7, locals.var_t2__blk1133_dn10, locals.var_t2__blk1133_dn11, locals.var_t2__blk1133_dn12, locals.var_t2__blk1133_dn17, ) = (assign34430_e49510, (assign34430_e49508 * locals.var_t3__blk1132_dn0), (assign34430_e49508 * locals.var_t3__blk1132_dn2), (assign34430_e49508 * locals.var_t3__blk1132_dn6), (assign34430_e49508 * locals.var_t3__blk1132_dn7), (assign34430_e49508 * locals.var_t3__blk1132_dn10), (assign34430_e49508 * locals.var_t3__blk1132_dn11), (assign34430_e49508 * locals.var_t3__blk1132_dn12), (assign34430_e49508 * locals.var_t3__blk1132_dn17), );
            locals.var_t2__blk1133_rv = 0.0;
        }
        if (locals.var_guard1134 == 0.0) {
            let assign34440_e49517: f64 = (locals.var_ps0 + locals.var_vds);
            let assign34440_e49519: f64 = (assign34440_e49517 - locals.var_t1__blk1130);
            let assign34440_e49521: f64 = (assign34440_e49519 / p.p64);
            let assign34440_e49523: f64 = (assign34440_e49521 - locals.var_ec);
            let assign34440_e49525: f64 = (assign34440_e49523 * locals.var_t2__blk1133);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34440_e49525, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1130_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1130_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1130_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1130_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1130_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1130_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1130_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1130_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1133) + (assign34440_e49523 * locals.var_t2__blk1133_dn17)), );
            locals.var_qy_rv = 0.0;
        }
        let assign34450_e49530: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34450_e49530;
        locals.var_guard1136_rv = 0.0;
        if (locals.var_guard1136 != 0.0) {
            let assign34460_e49535: f64 = (locals.var_cqyb0 * locals.var_vbsp);
            let assign34460_e49536: f64 = (locals.var_qy + assign34460_e49535);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34460_e49536, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)), );
            locals.var_qy_rv = 0.0;
        }
        let assign34470_e49541: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34470_e49541;
        locals.var_guard1137_rv = 0.0;
        let assign34480_e49544: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34480_e49544;
        locals.var_guard1138_rv = 0.0;
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
            let assign34490_e49549: f64 = (-locals.var_qbody_bt_p_sus);
            let assign34490_e49551: f64 = (assign34490_e49549 - locals.var_qbody_bt_p_sud);
            let assign34490_e49553: f64 = (assign34490_e49551 - locals.var_qbody_bt_n_sus);
            let assign34490_e49555: f64 = (assign34490_e49553 - locals.var_qbody_bt_n_sud);
            (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17, ) = (assign34490_e49555, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17), );
            locals.var_q_bt_ge_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
            let assign34500_e49563: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
            (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17, ) = (assign34500_e49563, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17), );
            locals.var_q_bt_de_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
            let assign34510_e49571: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
            (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17, ) = (assign34510_e49571, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17), );
            locals.var_q_bt_se_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
            let assign34520_e49581: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34520_e49583: f64 = (assign34520_e49581 + locals.var_qgob);
            let assign34520_e49585: f64 = (assign34520_e49583 - locals.var_qy);
            let assign34520_e49587: f64 = (assign34520_e49585 - locals.var_qovs);
            let assign34520_e49589: f64 = (assign34520_e49587 - locals.var_qovd);
            let assign34520_e49591: f64 = (assign34520_e49589 + locals.var_q_bt_ge);
            let assign34520_e49592: f64 = (locals.var_mfactor * assign34520_e49591);
            let assign34520_e49593: f64 = (locals.var_qge + assign34520_e49592);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34520_e49593, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
            let assign34530_e49602: f64 = (-locals.var_qgod);
            let assign34530_e49604: f64 = (assign34530_e49602 + locals.var_qy);
            let assign34530_e49606: f64 = (assign34530_e49604 + locals.var_qbdld);
            let assign34530_e49608: f64 = (assign34530_e49606 + locals.var_q_bt_de);
            let assign34530_e49609: f64 = (locals.var_mfactor * assign34530_e49608);
            let assign34530_e49610: f64 = (locals.var_qde + assign34530_e49609);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34530_e49610, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 != 0.0)) {
            let assign34540_e49619: f64 = (-locals.var_qgos);
            let assign34540_e49621: f64 = (assign34540_e49619 + locals.var_qbsld);
            let assign34540_e49623: f64 = (assign34540_e49621 + locals.var_q_bt_se);
            let assign34540_e49624: f64 = (locals.var_mfactor * assign34540_e49623);
            let assign34540_e49625: f64 = (locals.var_qse + assign34540_e49624);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34540_e49625, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 == 0.0)) {
            let assign34550_e49636: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34550_e49638: f64 = (assign34550_e49636 + locals.var_qgob);
            let assign34550_e49640: f64 = (assign34550_e49638 - locals.var_qy);
            let assign34550_e49642: f64 = (assign34550_e49640 - locals.var_qovs);
            let assign34550_e49644: f64 = (assign34550_e49642 - locals.var_qovd);
            let assign34550_e49645: f64 = (locals.var_mfactor * assign34550_e49644);
            let assign34550_e49646: f64 = (locals.var_qge + assign34550_e49645);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34550_e49646, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 == 0.0)) {
            let assign34560_e49656: f64 = (-locals.var_qgod);
            let assign34560_e49658: f64 = (assign34560_e49656 + locals.var_qy);
            let assign34560_e49660: f64 = (assign34560_e49658 + locals.var_qbdld);
            let assign34560_e49661: f64 = (locals.var_mfactor * assign34560_e49660);
            let assign34560_e49662: f64 = (locals.var_qde + assign34560_e49661);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34560_e49662, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_guard1137 != 0.0) && (locals.var_guard1138 == 0.0)) {
            let assign34570_e49672: f64 = (-locals.var_qgos);
            let assign34570_e49674: f64 = (assign34570_e49672 + locals.var_qbsld);
            let assign34570_e49675: f64 = (locals.var_mfactor * assign34570_e49674);
            let assign34570_e49676: f64 = (locals.var_qse + assign34570_e49675);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34570_e49676, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }
        let assign34600_e49683: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1139 = assign34600_e49683;
        locals.var_guard1139_rv = 0.0;
        if (locals.var_guard1139 != 0.0) {
            let assign34610_e49687: f64 = (locals.var_mfactor * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign34610_e49687, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }
        if (locals.var_guard1139 != 0.0) {
            let assign34620_e49693: f64 = (locals.var_mfactor * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign34620_e49693, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }
        if (locals.var_guard1139 != 0.0) {
            let assign34630_e49699: f64 = (locals.var_mfactor * locals.var_qbd);
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (assign34630_e49699, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17), );
            locals.var_qbd_s0_rv = 0.0;
        }
        if (locals.var_guard1139 != 0.0) {
            let assign34640_e49705: f64 = (locals.var_mfactor * locals.var_qbs);
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (assign34640_e49705, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17), );
            locals.var_qbs_s0_rv = 0.0;
        }
        if (locals.var_guard1139 == 0.0) {
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibsb_rv = 0.0;
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibdb_rv = 0.0;
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_s0_rv = 0.0;
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_s0_rv = 0.0;
        }
        let assign34690_e49730: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1140 = assign34690_e49730;
        locals.var_guard1140_rv = 0.0;
        if (locals.var_guard1140 != 0.0) {
            (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isube_rv = 0.0;
        }
        if (locals.var_guard1140 == 0.0) {
            let assign34710_e49739: f64 = (locals.var_mfactor * locals.var_isub);
            (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, ) = (assign34710_e49739, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17), );
            locals.var_isube_rv = 0.0;
        }
        let assign34820_e49823: f64 = (locals.var_mfactor * locals.var_nthrml);
        (locals.var_noithrml, locals.var_noithrml_dn0, locals.var_noithrml_dn2, locals.var_noithrml_dn6, locals.var_noithrml_dn7, locals.var_noithrml_dn10, locals.var_noithrml_dn11, locals.var_noithrml_dn12, locals.var_noithrml_dn17, ) = (assign34820_e49823, (locals.var_mfactor * locals.var_nthrml_dn0), (locals.var_mfactor * locals.var_nthrml_dn2), (locals.var_mfactor * locals.var_nthrml_dn6), (locals.var_mfactor * locals.var_nthrml_dn7), (locals.var_mfactor * locals.var_nthrml_dn10), (locals.var_mfactor * locals.var_nthrml_dn11), (locals.var_mfactor * locals.var_nthrml_dn12), (locals.var_mfactor * locals.var_nthrml_dn17), );
        locals.var_noithrml_rv = 0.0;
        let assign34830_e49826: f64 = locals.var_qge_dn6;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign34830_e49826, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;
        let assign34840_e49829: f64 = (p.p50 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign34840_e49829, (p.p50 * locals.var_cgdbd_dn0), (p.p50 * locals.var_cgdbd_dn2), (p.p50 * locals.var_cgdbd_dn6), (p.p50 * locals.var_cgdbd_dn7), (p.p50 * locals.var_cgdbd_dn10), (p.p50 * locals.var_cgdbd_dn11), (p.p50 * locals.var_cgdbd_dn12), (p.p50 * locals.var_cgdbd_dn13), (p.p50 * locals.var_cgdbd_dn15), (p.p50 * locals.var_cgdbd_dn16), (p.p50 * locals.var_cgdbd_dn17), (p.p50 * locals.var_cgdbd_dn18), );
        locals.var_cgdbd_rv = 0.0;
        let assign34850_e49832: f64 = locals.var_qge_dn7;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign34850_e49832, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;
        let assign34860_e49835: f64 = (p.p50 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign34860_e49835, (p.p50 * locals.var_cgsbd_dn0), (p.p50 * locals.var_cgsbd_dn2), (p.p50 * locals.var_cgsbd_dn6), (p.p50 * locals.var_cgsbd_dn7), (p.p50 * locals.var_cgsbd_dn10), (p.p50 * locals.var_cgsbd_dn11), (p.p50 * locals.var_cgsbd_dn12), (p.p50 * locals.var_cgsbd_dn13), (p.p50 * locals.var_cgsbd_dn15), (p.p50 * locals.var_cgsbd_dn16), (p.p50 * locals.var_cgsbd_dn17), (p.p50 * locals.var_cgsbd_dn18), );
        locals.var_cgsbd_rv = 0.0;
        let (assign34870_e49841, assign34870_e49841_d_n0, assign34870_e49841_d_n2, assign34870_e49841_d_n6, assign34870_e49841_d_n7, assign34870_e49841_d_n10, assign34870_e49841_d_n11, assign34870_e49841_d_n12, assign34870_e49841_d_n13, assign34870_e49841_d_n15, assign34870_e49841_d_n16, assign34870_e49841_d_n17, assign34870_e49841_d_n18,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18,)
    }
};
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn12, locals.var_cgsb_dn13, locals.var_cgsb_dn15, locals.var_cgsb_dn16, locals.var_cgsb_dn17, locals.var_cgsb_dn18, ) = (assign34870_e49841, assign34870_e49841_d_n0, assign34870_e49841_d_n2, assign34870_e49841_d_n6, assign34870_e49841_d_n7, assign34870_e49841_d_n10, assign34870_e49841_d_n11, assign34870_e49841_d_n12, assign34870_e49841_d_n13, assign34870_e49841_d_n15, assign34870_e49841_d_n16, assign34870_e49841_d_n17, assign34870_e49841_d_n18, );
        locals.var_cgsb_rv = 0.0;
        let assign34880_e49855: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1149 = assign34880_e49855;
        locals.var_guard1149_rv = 0.0;
        if (locals.var_guard1149 != 0.0) {
            let assign34890_e49859: f64 = (1e-6 * locals.var_c_fox);
            let assign34890_e49861: f64 = (assign34890_e49859 * locals.var_weffcv_nf);
            let assign34890_e49863: f64 = (assign34890_e49861 * locals.var_leff_cv);
            (locals.var_t0__blk1143, locals.var_t0__blk1143_dn0, locals.var_t0__blk1143_dn2, locals.var_t0__blk1143_dn6, locals.var_t0__blk1143_dn7, locals.var_t0__blk1143_dn10, locals.var_t0__blk1143_dn11, locals.var_t0__blk1143_dn12, locals.var_t0__blk1143_dn17, ) = (assign34890_e49863, (((1e-6 * locals.var_c_fox_dn0) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn2) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn6) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn7) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn10) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn11) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn12) * locals.var_weffcv_nf) * locals.var_leff_cv), (((1e-6 * locals.var_c_fox_dn17) * locals.var_weffcv_nf) * locals.var_leff_cv), );
            locals.var_t0__blk1143_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign34900_e49869: f64 = (locals.var_cgsb / locals.var_mfactor);
            (locals.var_t1__blk1144, locals.var_t1__blk1144_dn0, locals.var_t1__blk1144_dn2, locals.var_t1__blk1144_dn6, locals.var_t1__blk1144_dn7, locals.var_t1__blk1144_dn10, locals.var_t1__blk1144_dn11, locals.var_t1__blk1144_dn12, locals.var_t1__blk1144_dn13, locals.var_t1__blk1144_dn15, locals.var_t1__blk1144_dn16, locals.var_t1__blk1144_dn17, locals.var_t1__blk1144_dn18, ) = (assign34900_e49869, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor), (locals.var_cgsb_dn13 / locals.var_mfactor), (locals.var_cgsb_dn15 / locals.var_mfactor), (locals.var_cgsb_dn16 / locals.var_mfactor), (locals.var_cgsb_dn17 / locals.var_mfactor), (locals.var_cgsb_dn18 / locals.var_mfactor), );
            locals.var_t1__blk1144_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign34910_e49875: f64 = (0.1185185185185185 * 1.6021918e-19);
            let assign34910_e49877: f64 = (assign34910_e49875 * locals.var_beta_inv);
            let assign34910_e49879: f64 = (assign34910_e49877 * locals.var_t1__blk1144);
            let assign34910_e49881: f64 = (assign34910_e49879 * locals.var_t1__blk1144);
            let assign34910_e49883: f64 = (assign34910_e49881 / locals.var_gds0_ign);
            (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12, locals.var_nign0_dn13, locals.var_nign0_dn15, locals.var_nign0_dn16, locals.var_nign0_dn17, locals.var_nign0_dn18, ) = (assign34910_e49883, ((((((assign34910_e49877 * locals.var_t1__blk1144_dn0) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn0)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn2) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn2)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn6) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn6)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn7) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn7)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign34910_e49875 * locals.var_beta_inv_dn10) * locals.var_t1__blk1144) + (assign34910_e49877 * locals.var_t1__blk1144_dn10)) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn10)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn11) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn11)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn12) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn12)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34910_e49877 * locals.var_t1__blk1144_dn13) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn13)) / locals.var_gds0_ign), ((((assign34910_e49877 * locals.var_t1__blk1144_dn15) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn15)) / locals.var_gds0_ign), ((((assign34910_e49877 * locals.var_t1__blk1144_dn16) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn16)) / locals.var_gds0_ign), ((((((assign34910_e49877 * locals.var_t1__blk1144_dn17) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn17)) * locals.var_gds0_ign) - (assign34910_e49881 * locals.var_gds0_ign_dn17)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((assign34910_e49877 * locals.var_t1__blk1144_dn18) * locals.var_t1__blk1144) + (assign34910_e49879 * locals.var_t1__blk1144_dn18)) / locals.var_gds0_ign), );
            locals.var_nign0_rv = 0.0;
        }
        let assign34920_e49889: f64 = (10.0 * 2.220446049250313e-16);
        let assign34920_e49894: f64 = (10.0 * 2.220446049250313e-16);
        let assign34920_e49896: f64 = if ((locals.var_kusai00l > assign34920_e49889) && (locals.var_vds > assign34920_e49894)) { 1.0 } else { 0.0 };
        locals.var_guard1150 = assign34920_e49896;
        locals.var_guard1150_rv = 0.0;
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 != 0.0)) {
            let assign34930_e49902: f64 = (locals.var_muun / locals.var_mu);
            (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12, locals.var_mumoda_dn17, ) = (assign34930_e49902, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn17 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn17)) / (locals.var_mu * locals.var_mu)), );
            locals.var_mumoda_rv = 0.0;
        }
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 != 0.0)) {
            let assign34940_e49910: f64 = (locals.var_muun / locals.var_mud_hoso);
            let assign34940_e49912: f64 = (assign34940_e49910 - locals.var_mumoda);
            let assign34940_e49914: f64 = (assign34940_e49912 / locals.var_vds);
            (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12, locals.var_mumodb_dn17, ) = (assign34940_e49914, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn17) * locals.var_vds) - (assign34940_e49912 * locals.var_vds_dn17)) / (locals.var_vds * locals.var_vds)), );
            locals.var_mumodb_rv = 0.0;
        }
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 != 0.0)) {
            let assign34950_e49923: f64 = (0.6666666666666667 * locals.var_mumodb);
            let assign34950_e49927: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
            let assign34950_e49928: f64 = (locals.var_kusai00 + assign34950_e49927);
            let assign34950_e49930: f64 = (assign34950_e49928 + locals.var_kusail);
            let assign34950_e49931: f64 = (assign34950_e49923 * assign34950_e49930);
            let assign34950_e49934: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
            let assign34950_e49935: f64 = (assign34950_e49931 / assign34950_e49934);
            let assign34950_e49936: f64 = (locals.var_mumoda + assign34950_e49935);
            (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17, ) = (assign34950_e49936, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign34950_e49934 * assign34950_e49934))), (locals.var_mumoda_dn17 + ((((((0.6666666666666667 * locals.var_mumodb_dn17) * assign34950_e49930) + (assign34950_e49923 * ((locals.var_kusai00_dn17 + ((locals.var_vgvt_dn17 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn17))) + locals.var_kusail_dn17))) * assign34950_e49934) - (assign34950_e49931 * (locals.var_vgvt_dn17 + locals.var_sqrtkusail_dn17))) / (assign34950_e49934 * assign34950_e49934))), );
            locals.var_correct_w1_rv = 0.0;
        }
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1150 == 0.0)) {
            let assign34960_e49945: f64 = (locals.var_muun / locals.var_mud_hoso);
            (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, locals.var_correct_w1_dn17, ) = (assign34960_e49945, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn17 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn17)) / (locals.var_mud_hoso * locals.var_mud_hoso)), );
            locals.var_correct_w1_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign34970_e49951: f64 = (locals.var_mfactor * locals.var_nign0);
            let assign34970_e49953: f64 = (assign34970_e49951 * locals.var_kusai_ig);
            let assign34970_e49955: f64 = (assign34970_e49953 * locals.var_correct_w1);
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18, ) = (assign34970_e49955, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn12)), (((locals.var_mfactor * locals.var_nign0_dn13) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn15) * locals.var_kusai_ig) * locals.var_correct_w1), (((locals.var_mfactor * locals.var_nign0_dn16) * locals.var_kusai_ig) * locals.var_correct_w1), (((((locals.var_mfactor * locals.var_nign0_dn17) * locals.var_kusai_ig) + (assign34970_e49951 * locals.var_kusai_ig_dn17)) * locals.var_correct_w1) + (assign34970_e49953 * locals.var_correct_w1_dn17)), (((locals.var_mfactor * locals.var_nign0_dn18) * locals.var_kusai_ig) * locals.var_correct_w1), );
            locals.var_noiigate_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign34990_e49964: f64 = (-locals.var_t1__blk1144);
            let (assign34990_e49973, assign34990_e49973_d_n0, assign34990_e49973_d_n2, assign34990_e49973_d_n6, assign34990_e49973_d_n7, assign34990_e49973_d_n10, assign34990_e49973_d_n11, assign34990_e49973_d_n12, assign34990_e49973_d_n13, assign34990_e49973_d_n15, assign34990_e49973_d_n16, assign34990_e49973_d_n17, assign34990_e49973_d_n18,) = {
    if ((assign34990_e49964 > locals.var_t0__blk1143) && (locals.var_noiigate > 0.0)) {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18, ) = (assign34990_e49973, assign34990_e49973_d_n0, assign34990_e49973_d_n2, assign34990_e49973_d_n6, assign34990_e49973_d_n7, assign34990_e49973_d_n10, assign34990_e49973_d_n11, assign34990_e49973_d_n12, assign34990_e49973_d_n13, assign34990_e49973_d_n15, assign34990_e49973_d_n16, assign34990_e49973_d_n17, assign34990_e49973_d_n18, );
            locals.var_noiigate_rv = 0.0;
        }
        if (locals.var_guard1149 == 0.0) {
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, locals.var_noiigate_dn13, locals.var_noiigate_dn15, locals.var_noiigate_dn16, locals.var_noiigate_dn17, locals.var_noiigate_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_noiigate_rv = 0.0;
        }
        let assign35070_e50002: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1151 = assign35070_e50002;
        locals.var_guard1151_rv = 0.0;
        if (locals.var_guard1151 != 0.0) {
            locals.var_rdmod = 1.0;
            locals.var_rdmod_rv = 0.0;
        }
        let assign35090_e50009: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign35090_e50009;
        locals.var_guard1171_rv = 0.0;
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
            locals.var_mks_rdrmue = p.p266;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p268;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p258;
            locals.var_ldrifte_rv = 0.0;
        }
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 != 0.0)) {
            let assign35160_e50060: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35160_e50060, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr_rv = 0.0;
        }
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
            locals.var_mks_rdrmue = p.p265;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p267;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p257;
            locals.var_ldrifte_rv = 0.0;
        }
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1171 == 0.0)) {
            let assign35230_e50120: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35230_e50120, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35260_e50143: f64 = (locals.var_mks_rdrmue / 10000.0);
            locals.var_mks_rdrmue = assign35260_e50143;
            locals.var_mks_rdrmue_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35270_e50149: f64 = (locals.var_mks_rdrvmax / 100.0);
            locals.var_mks_rdrvmax = assign35270_e50149;
            locals.var_mks_rdrvmax_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35280_e50155: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio, locals.var_tratio_dn10, ) = (assign35280_e50155, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35290_e50161: f64 = (locals.var_tratio).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35290_e50161, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35290_e50161 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35300_e50167: f64 = (locals.var_mks_rdrmue / locals.var_t1);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35300_e50167, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35310_e50174: f64 = (0.4 * locals.var_tratio);
            let assign35310_e50175: f64 = (1.8 + assign35310_e50174);
            let assign35310_e50178: f64 = (0.1 * locals.var_tratio);
            let assign35310_e50180: f64 = (assign35310_e50178 * locals.var_tratio);
            let assign35310_e50181: f64 = (assign35310_e50175 + assign35310_e50180);
            let assign35310_e50185: f64 = (1.0 - locals.var_tratio);
            let assign35310_e50186: f64 = (p.p270 * assign35310_e50185);
            let assign35310_e50187: f64 = (assign35310_e50181 - assign35310_e50186);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35310_e50187, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35310_e50178 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35320_e50193: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
            (locals.var_vmaxe__blk1164, locals.var_vmaxe__blk1164_dn0, locals.var_vmaxe__blk1164_dn2, locals.var_vmaxe__blk1164_dn6, locals.var_vmaxe__blk1164_dn7, locals.var_vmaxe__blk1164_dn10, locals.var_vmaxe__blk1164_dn11, locals.var_vmaxe__blk1164_dn12, locals.var_vmaxe__blk1164_dn17, ) = (assign35320_e50193, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1164_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35330_e50201: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35330_e50202: f64 = (p.p274 * assign35330_e50201);
            let assign35330_e50203: f64 = (locals.var_rrdrbb + assign35330_e50202);
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (assign35330_e50203, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35340_e50211: f64 = (locals.var_lgle).powf(p.p280);
            let assign35340_e50212: f64 = (p.p279 / assign35340_e50211);
            let assign35340_e50213: f64 = (1.0 + assign35340_e50212);
            locals.var_rdrmuele = assign35340_e50213;
            locals.var_rdrmuele_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35350_e50221: f64 = (locals.var_lgle).powf(p.p278);
            let assign35350_e50222: f64 = (p.p277 / assign35350_e50221);
            let assign35350_e50223: f64 = (1.0 + assign35350_e50222);
            locals.var_rdrvmaxle = assign35350_e50223;
            locals.var_rdrvmaxle_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35360_e50231: f64 = (locals.var_wg).powf(p.p276);
            let assign35360_e50232: f64 = (p.p275 / assign35360_e50231);
            let assign35360_e50233: f64 = (1.0 + assign35360_e50232);
            locals.var_rdrvmaxwe = assign35360_e50233;
            locals.var_rdrvmaxwe_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35370_e50239: f64 = (locals.var_mu0 * locals.var_rdrmuele);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35370_e50239, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35380_e50245: f64 = (locals.var_vmaxe__blk1164 * locals.var_rdrvmaxwe);
            let assign35380_e50247: f64 = (assign35380_e50245 * locals.var_rdrvmaxle);
            let assign35380_e50249: f64 = (assign35380_e50247 + 1e-50);
            (locals.var_vmaxe__blk1164, locals.var_vmaxe__blk1164_dn0, locals.var_vmaxe__blk1164_dn2, locals.var_vmaxe__blk1164_dn6, locals.var_vmaxe__blk1164_dn7, locals.var_vmaxe__blk1164_dn10, locals.var_vmaxe__blk1164_dn11, locals.var_vmaxe__blk1164_dn12, locals.var_vmaxe__blk1164_dn17, ) = (assign35380_e50249, ((locals.var_vmaxe__blk1164_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1164_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), );
            locals.var_vmaxe__blk1164_rv = 0.0;
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
        if (locals.var_guard1151 != 0.0) {
            let assign35390_e50255: f64 = (locals.var_vrdr / locals.var_ldrifte);
            (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7, ) = (assign35390_e50255, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte), );
            locals.var_edri_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35400_e50261: f64 = (locals.var_mu0 * locals.var_edri);
            (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17, ) = (assign35400_e50261, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri), );
            locals.var_vdri_rv = 0.0;
        }
        let assign35410_e50266: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign35410_e50266;
        locals.var_guard1172_rv = 0.0;
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1172 != 0.0)) {
            let assign35420_e50272: f64 = (locals.var_vdri / locals.var_vmaxe__blk1164);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35420_e50272, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn0)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn2)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn6)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn7)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn10)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn11)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn12)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1164) - (locals.var_vdri * locals.var_vmaxe__blk1164_dn17)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1172 == 0.0)) {
            let assign35430_e50280: f64 = (-locals.var_vdri);
            let assign35430_e50282: f64 = (assign35430_e50280 / locals.var_vmaxe__blk1164);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35430_e50282, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn0)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn2)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn6)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn7)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn10)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn11)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn12)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1164) - (assign35430_e50280 * locals.var_vmaxe__blk1164_dn17)) / (locals.var_vmaxe__blk1164 * locals.var_vmaxe__blk1164)), );
            locals.var_t1_rv = 0.0;
        }
        let assign35440_e50288: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50289: f64 = (1.0 - assign35440_e50288);
        let assign35440_e50296: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50297: f64 = (1.0 + assign35440_e50296);
        let assign35440_e50299: f64 = if ((assign35440_e50289 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35440_e50297)) { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign35440_e50299;
        locals.var_guard1173_rv = 0.0;
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1173 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign35460_e50309: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50310: f64 = (2.0 - assign35460_e50309);
        let assign35460_e50317: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50318: f64 = (2.0 + assign35460_e50317);
        let assign35460_e50320: f64 = if ((assign35460_e50310 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35460_e50318)) { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign35460_e50320;
        locals.var_guard1174_rv = 0.0;
        if (((locals.var_guard1151 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard1151 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
            let assign35480_e50340: f64 = (locals.var_rrdrbb - 1.0);
            let assign35480_e50341: f64 = (locals.var_t1).powf(assign35480_e50340);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign35480_e50341, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn0)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn2)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn6)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn7)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn10)) } } else { (assign35480_e50341 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35480_e50340 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn11)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn12)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((locals.var_t1).powf(assign35480_e50340 - 1.0) * locals.var_t1_dn17)) } } else { (assign35480_e50341 * (assign35480_e50340 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35490_e50347: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign35490_e50347, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35500_e50353: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign35500_e50353, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }
        let assign35510_e50359: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50360: f64 = (1.0 - assign35510_e50359);
        let assign35510_e50367: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50368: f64 = (1.0 + assign35510_e50367);
        let assign35510_e50370: f64 = if ((assign35510_e50360 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35510_e50368)) { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign35510_e50370;
        locals.var_guard1175_rv = 0.0;
        if ((locals.var_guard1151 != 0.0) && (locals.var_guard1175 != 0.0)) {
            let assign35520_e50376: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35520_e50376, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (2.0 - assign35530_e50382);
        let assign35530_e50390: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50391: f64 = (2.0 + assign35530_e50390);
        let assign35530_e50393: f64 = if ((assign35530_e50383 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35530_e50391)) { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign35530_e50393;
        locals.var_guard1176_rv = 0.0;
        if (((locals.var_guard1151 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 != 0.0)) {
            let assign35540_e50402: f64 = (locals.var_t4).sqrt();
            let assign35540_e50403: f64 = (1.0 / assign35540_e50402);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35540_e50403, (-((locals.var_t4_dn0 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn2 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn6 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn7 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn10 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn11 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn12 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((locals.var_t4_dn17 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard1151 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 == 0.0)) {
            let assign35550_e50415: f64 = (-1.0);
            let assign35550_e50417: f64 = (assign35550_e50415 / locals.var_rrdrbb);
            let assign35550_e50419: f64 = (assign35550_e50417 - 1.0);
            let assign35550_e50420: f64 = (locals.var_t4).powf(assign35550_e50419);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign35550_e50420, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn0)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn2)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn6)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn7)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign35550_e50415 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn10)) } } else { (assign35550_e50420 * (((-((assign35550_e50415 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign35550_e50419 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn11)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn12)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((locals.var_t4).powf(assign35550_e50419 - 1.0) * locals.var_t4_dn17)) } } else { (assign35550_e50420 * (assign35550_e50419 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard1151 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 == 0.0)) {
            let assign35560_e50432: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35560_e50432, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard1151 != 0.0) {
            let assign35580_e50444: f64 = (1.6021918e-19 / locals.var_ldrifte);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35580_e50444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign35700_e50520: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1179 = assign35700_e50520;
        locals.var_guard1179_rv = 0.0;
        if (locals.var_guard1179 != 0.0) {
            locals.var_rdmod = 2.0;
            locals.var_rdmod_rv = 0.0;
        }
        let assign35720_e50527: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign35720_e50527;
        locals.var_guard1199_rv = 0.0;
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
            locals.var_mks_rdrmue__blk1183 = p.p266;
            locals.var_mks_rdrmue__blk1183_rv = 0.0;
            locals.var_mks_rdrvmax__blk1184 = p.p268;
            locals.var_mks_rdrvmax__blk1184_rv = 0.0;
            (locals.var_rrdrbb__blk1185, locals.var_rrdrbb__blk1185_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb__blk1185_rv = 0.0;
            locals.var_ldrifte__blk1189 = p.p258;
            locals.var_ldrifte__blk1189_rv = 0.0;
        }
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 != 0.0)) {
            let assign35790_e50578: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr__blk1187, locals.var_vrdr__blk1187_dn0, locals.var_vrdr__blk1187_dn2, locals.var_vrdr__blk1187_dn6, locals.var_vrdr__blk1187_dn7, ) = (assign35790_e50578, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr__blk1187_rv = 0.0;
        }
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
            locals.var_mks_rdrmue__blk1183 = p.p265;
            locals.var_mks_rdrmue__blk1183_rv = 0.0;
            locals.var_mks_rdrvmax__blk1184 = p.p267;
            locals.var_mks_rdrvmax__blk1184_rv = 0.0;
            (locals.var_rrdrbb__blk1185, locals.var_rrdrbb__blk1185_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb__blk1185_rv = 0.0;
            locals.var_ldrifte__blk1189 = p.p257;
            locals.var_ldrifte__blk1189_rv = 0.0;
        }
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1199 == 0.0)) {
            let assign35860_e50638: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr__blk1187, locals.var_vrdr__blk1187_dn0, locals.var_vrdr__blk1187_dn2, locals.var_vrdr__blk1187_dn6, locals.var_vrdr__blk1187_dn7, ) = (assign35860_e50638, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr__blk1187_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35890_e50661: f64 = (locals.var_mks_rdrmue__blk1183 / 10000.0);
            locals.var_mks_rdrmue__blk1183 = assign35890_e50661;
            locals.var_mks_rdrmue__blk1183_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35900_e50667: f64 = (locals.var_mks_rdrvmax__blk1184 / 100.0);
            locals.var_mks_rdrvmax__blk1184 = assign35900_e50667;
            locals.var_mks_rdrvmax__blk1184_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35910_e50673: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio__blk1188, locals.var_tratio__blk1188_dn10, ) = (assign35910_e50673, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio__blk1188_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35920_e50679: f64 = (locals.var_tratio__blk1188).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35920_e50679, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio__blk1188).powf(p.p269 - 1.0) * locals.var_tratio__blk1188_dn10)) } } else { (assign35920_e50679 * (p.p269 * (locals.var_tratio__blk1188_dn10 / locals.var_tratio__blk1188))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35930_e50685: f64 = (locals.var_mks_rdrmue__blk1183 / locals.var_t1);
            (locals.var_mu0__blk1191, locals.var_mu0__blk1191_dn0, locals.var_mu0__blk1191_dn2, locals.var_mu0__blk1191_dn6, locals.var_mu0__blk1191_dn7, locals.var_mu0__blk1191_dn10, locals.var_mu0__blk1191_dn11, locals.var_mu0__blk1191_dn12, locals.var_mu0__blk1191_dn17, ) = (assign35930_e50685, (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1183 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0__blk1191_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35940_e50692: f64 = (0.4 * locals.var_tratio__blk1188);
            let assign35940_e50693: f64 = (1.8 + assign35940_e50692);
            let assign35940_e50696: f64 = (0.1 * locals.var_tratio__blk1188);
            let assign35940_e50698: f64 = (assign35940_e50696 * locals.var_tratio__blk1188);
            let assign35940_e50699: f64 = (assign35940_e50693 + assign35940_e50698);
            let assign35940_e50703: f64 = (1.0 - locals.var_tratio__blk1188);
            let assign35940_e50704: f64 = (p.p270 * assign35940_e50703);
            let assign35940_e50705: f64 = (assign35940_e50699 - assign35940_e50704);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35940_e50705, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio__blk1188_dn10) + (((0.1 * locals.var_tratio__blk1188_dn10) * locals.var_tratio__blk1188) + (assign35940_e50696 * locals.var_tratio__blk1188_dn10))) - (p.p270 * (-locals.var_tratio__blk1188_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35950_e50711: f64 = (locals.var_mks_rdrvmax__blk1184 / locals.var_t0);
            (locals.var_vmaxe__blk1192, locals.var_vmaxe__blk1192_dn0, locals.var_vmaxe__blk1192_dn2, locals.var_vmaxe__blk1192_dn6, locals.var_vmaxe__blk1192_dn7, locals.var_vmaxe__blk1192_dn10, locals.var_vmaxe__blk1192_dn11, locals.var_vmaxe__blk1192_dn12, locals.var_vmaxe__blk1192_dn17, ) = (assign35950_e50711, (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1184 * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1192_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35960_e50719: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35960_e50720: f64 = (p.p274 * assign35960_e50719);
            let assign35960_e50721: f64 = (locals.var_rrdrbb__blk1185 + assign35960_e50720);
            (locals.var_rrdrbb__blk1185, locals.var_rrdrbb__blk1185_dn10, ) = (assign35960_e50721, (locals.var_rrdrbb__blk1185_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb__blk1185_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35970_e50729: f64 = (locals.var_lgle).powf(p.p280);
            let assign35970_e50730: f64 = (p.p279 / assign35970_e50729);
            let assign35970_e50731: f64 = (1.0 + assign35970_e50730);
            locals.var_rdrmuele__blk1180 = assign35970_e50731;
            locals.var_rdrmuele__blk1180_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35980_e50739: f64 = (locals.var_lgle).powf(p.p278);
            let assign35980_e50740: f64 = (p.p277 / assign35980_e50739);
            let assign35980_e50741: f64 = (1.0 + assign35980_e50740);
            locals.var_rdrvmaxle__blk1182 = assign35980_e50741;
            locals.var_rdrvmaxle__blk1182_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign35990_e50749: f64 = (locals.var_wg).powf(p.p276);
            let assign35990_e50750: f64 = (p.p275 / assign35990_e50749);
            let assign35990_e50751: f64 = (1.0 + assign35990_e50750);
            locals.var_rdrvmaxwe__blk1181 = assign35990_e50751;
            locals.var_rdrvmaxwe__blk1181_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign36000_e50757: f64 = (locals.var_mu0__blk1191 * locals.var_rdrmuele__blk1180);
            (locals.var_mu0__blk1191, locals.var_mu0__blk1191_dn0, locals.var_mu0__blk1191_dn2, locals.var_mu0__blk1191_dn6, locals.var_mu0__blk1191_dn7, locals.var_mu0__blk1191_dn10, locals.var_mu0__blk1191_dn11, locals.var_mu0__blk1191_dn12, locals.var_mu0__blk1191_dn17, ) = (assign36000_e50757, (locals.var_mu0__blk1191_dn0 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn2 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn6 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn7 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn10 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn11 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn12 * locals.var_rdrmuele__blk1180), (locals.var_mu0__blk1191_dn17 * locals.var_rdrmuele__blk1180), );
            locals.var_mu0__blk1191_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign36010_e50763: f64 = (locals.var_vmaxe__blk1192 * locals.var_rdrvmaxwe__blk1181);
            let assign36010_e50765: f64 = (assign36010_e50763 * locals.var_rdrvmaxle__blk1182);
            let assign36010_e50767: f64 = (assign36010_e50765 + 1e-50);
            (locals.var_vmaxe__blk1192, locals.var_vmaxe__blk1192_dn0, locals.var_vmaxe__blk1192_dn2, locals.var_vmaxe__blk1192_dn6, locals.var_vmaxe__blk1192_dn7, locals.var_vmaxe__blk1192_dn10, locals.var_vmaxe__blk1192_dn11, locals.var_vmaxe__blk1192_dn12, locals.var_vmaxe__blk1192_dn17, ) = (assign36010_e50767, ((locals.var_vmaxe__blk1192_dn0 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn2 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn6 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn7 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn10 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn11 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn12 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), ((locals.var_vmaxe__blk1192_dn17 * locals.var_rdrvmaxwe__blk1181) * locals.var_rdrvmaxle__blk1182), );
            locals.var_vmaxe__blk1192_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign36020_e50773: f64 = (locals.var_vrdr__blk1187 / locals.var_ldrifte__blk1189);
            (locals.var_edri__blk1193, locals.var_edri__blk1193_dn0, locals.var_edri__blk1193_dn2, locals.var_edri__blk1193_dn6, locals.var_edri__blk1193_dn7, ) = (assign36020_e50773, (locals.var_vrdr__blk1187_dn0 / locals.var_ldrifte__blk1189), (locals.var_vrdr__blk1187_dn2 / locals.var_ldrifte__blk1189), (locals.var_vrdr__blk1187_dn6 / locals.var_ldrifte__blk1189), (locals.var_vrdr__blk1187_dn7 / locals.var_ldrifte__blk1189), );
            locals.var_edri__blk1193_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign36030_e50779: f64 = (locals.var_mu0__blk1191 * locals.var_edri__blk1193);
            (locals.var_vdri__blk1194, locals.var_vdri__blk1194_dn0, locals.var_vdri__blk1194_dn2, locals.var_vdri__blk1194_dn6, locals.var_vdri__blk1194_dn7, locals.var_vdri__blk1194_dn10, locals.var_vdri__blk1194_dn11, locals.var_vdri__blk1194_dn12, locals.var_vdri__blk1194_dn17, ) = (assign36030_e50779, ((locals.var_mu0__blk1191_dn0 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn0)), ((locals.var_mu0__blk1191_dn2 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn2)), ((locals.var_mu0__blk1191_dn6 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn6)), ((locals.var_mu0__blk1191_dn7 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn7)), (locals.var_mu0__blk1191_dn10 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn11 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn12 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn17 * locals.var_edri__blk1193), );
            locals.var_vdri__blk1194_rv = 0.0;
        }
        let assign36040_e50784: f64 = if locals.var_vrdr__blk1187 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign36040_e50784;
        locals.var_guard1200_rv = 0.0;
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1200 != 0.0)) {
            let assign36050_e50790: f64 = (locals.var_vdri__blk1194 / locals.var_vmaxe__blk1192);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36050_e50790, (((locals.var_vdri__blk1194_dn0 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn0)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn2 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn2)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn6 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn6)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn7 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn7)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn10 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn10)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn11 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn11)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn12 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn12)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn17 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn17)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1200 == 0.0)) {
            let assign36060_e50798: f64 = (-locals.var_vdri__blk1194);
            let assign36060_e50800: f64 = (assign36060_e50798 / locals.var_vmaxe__blk1192);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36060_e50800, ((((-locals.var_vdri__blk1194_dn0) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn0)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn2) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn2)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn6) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn6)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn7) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn7)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn10) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn10)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn11) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn11)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn12) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn12)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn17) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn17)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), );
            locals.var_t1_rv = 0.0;
        }
        let assign36070_e50806: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50807: f64 = (1.0 - assign36070_e50806);
        let assign36070_e50814: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50815: f64 = (1.0 + assign36070_e50814);
        let assign36070_e50817: f64 = if ((assign36070_e50807 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36070_e50815)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign36070_e50817;
        locals.var_guard1201_rv = 0.0;
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1201 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign36090_e50827: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50828: f64 = (2.0 - assign36090_e50827);
        let assign36090_e50835: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50836: f64 = (2.0 + assign36090_e50835);
        let assign36090_e50838: f64 = if ((assign36090_e50828 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36090_e50836)) { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign36090_e50838;
        locals.var_guard1202_rv = 0.0;
        if (((locals.var_guard1179 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard1179 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign36110_e50858: f64 = (locals.var_rrdrbb__blk1185 - 1.0);
            let assign36110_e50859: f64 = (locals.var_t1).powf(assign36110_e50858);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign36110_e50859, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn0)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn2)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn6)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn7)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb__blk1185_dn10 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn10)) } } else { (assign36110_e50859 * ((locals.var_rrdrbb__blk1185_dn10 * (locals.var_t1).ln()) + (assign36110_e50858 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn11)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn12)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn17)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign36120_e50865: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign36120_e50865, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign36130_e50871: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign36130_e50871, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }
        let assign36140_e50877: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50878: f64 = (1.0 - assign36140_e50877);
        let assign36140_e50885: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50886: f64 = (1.0 + assign36140_e50885);
        let assign36140_e50888: f64 = if ((assign36140_e50878 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36140_e50886)) { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign36140_e50888;
        locals.var_guard1203_rv = 0.0;
        if ((locals.var_guard1179 != 0.0) && (locals.var_guard1203 != 0.0)) {
            let assign36150_e50894: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36150_e50894, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (2.0 - assign36160_e50900);
        let assign36160_e50908: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50909: f64 = (2.0 + assign36160_e50908);
        let assign36160_e50911: f64 = if ((assign36160_e50901 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36160_e50909)) { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign36160_e50911;
        locals.var_guard1204_rv = 0.0;
        if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
            let assign36170_e50920: f64 = (locals.var_t4).sqrt();
            let assign36170_e50921: f64 = (1.0 / assign36170_e50920);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36170_e50921, (-((locals.var_t4_dn0 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn2 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn6 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn7 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn10 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn11 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn12 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn17 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign36180_e50933: f64 = (-1.0);
            let assign36180_e50935: f64 = (assign36180_e50933 / locals.var_rrdrbb__blk1185);
            let assign36180_e50937: f64 = (assign36180_e50935 - 1.0);
            let assign36180_e50938: f64 = (locals.var_t4).powf(assign36180_e50937);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign36180_e50938, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn0)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn2)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn6)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn7)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign36180_e50933 * locals.var_rrdrbb__blk1185_dn10) / (locals.var_rrdrbb__blk1185 * locals.var_rrdrbb__blk1185))) == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn10)) } } else { (assign36180_e50938 * (((-((assign36180_e50933 * locals.var_rrdrbb__blk1185_dn10) / (locals.var_rrdrbb__blk1185 * locals.var_rrdrbb__blk1185))) * (locals.var_t4).ln()) + (assign36180_e50937 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn11)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn12)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn17)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign36190_e50950: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36190_e50950, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard1179 != 0.0) {
            let assign36210_e50962: f64 = (1.6021918e-19 / locals.var_ldrifte__blk1189);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36210_e50962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign36330_e51038: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign36330_e51038;
        locals.var_guard1207_rv = 0.0;
        if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,) = {
    if (locals.var_mode == 1.0) {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    } else {
        let assign36380_e51078: f64 = (1.0 - locals.var_xd);
        (assign36380_e51078, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn12), (-locals.var_xd_dn17),)
    }
};
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17, );
            locals.var_qdrat_rv = 0.0;
        }
        if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36410_e51107: f64 = (locals.var_qi_nqs * locals.var_qdrat);
            let assign36410_e51109: f64 = (assign36410_e51107 + locals.var_q_bt_se);
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (assign36410_e51109, ((locals.var_qi_nqs * locals.var_qdrat_dn0) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * locals.var_qdrat_dn2) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * locals.var_qdrat_dn6) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * locals.var_qdrat_dn7) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * locals.var_qdrat_dn10) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * locals.var_qdrat_dn11) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * locals.var_qdrat_dn12) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * locals.var_qdrat_dn17) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * locals.var_qdrat), );
            locals.var_qd_nqs_rv = 0.0;
        }
        if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36420_e51118: f64 = (1.0 - locals.var_qdrat);
            let assign36420_e51119: f64 = (locals.var_qi_nqs * assign36420_e51118);
            let assign36420_e51121: f64 = (assign36420_e51119 + locals.var_q_bt_se);
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (assign36420_e51121, ((locals.var_qi_nqs * (-locals.var_qdrat_dn0)) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * (-locals.var_qdrat_dn2)) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * (-locals.var_qdrat_dn6)) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * (-locals.var_qdrat_dn7)) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * (-locals.var_qdrat_dn10)) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * (-locals.var_qdrat_dn11)) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * (-locals.var_qdrat_dn12)) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * (-locals.var_qdrat_dn17)) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * assign36420_e51118), );
            locals.var_qs_nqs_rv = 0.0;
        }
        if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36430_e51128: f64 = (-locals.var_qi_nqs);
            let assign36430_e51130: f64 = (assign36430_e51128 - locals.var_qb_nqs);
            let assign36430_e51132: f64 = (assign36430_e51130 + locals.var_q_bt_ge);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36430_e51132, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, (-locals.var_qb_nqs_dn13), 0.0, 0.0, locals.var_q_bt_ge_dn17, (-locals.var_qi_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }
        if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36580_e51254: f64 = (-locals.var_qd_nqs);
            let assign36580_e51256: f64 = (assign36580_e51254 - locals.var_qs_nqs);
            let assign36580_e51258: f64 = (assign36580_e51256 - locals.var_qb_nqs);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36580_e51258, ((-locals.var_qd_nqs_dn0) - locals.var_qs_nqs_dn0), ((-locals.var_qd_nqs_dn2) - locals.var_qs_nqs_dn2), ((-locals.var_qd_nqs_dn6) - locals.var_qs_nqs_dn6), ((-locals.var_qd_nqs_dn7) - locals.var_qs_nqs_dn7), ((-locals.var_qd_nqs_dn10) - locals.var_qs_nqs_dn10), ((-locals.var_qd_nqs_dn11) - locals.var_qs_nqs_dn11), ((-locals.var_qd_nqs_dn12) - locals.var_qs_nqs_dn12), (-locals.var_qb_nqs_dn13), (-locals.var_qd_nqs_dn15), (-locals.var_qs_nqs_dn16), ((-locals.var_qd_nqs_dn17) - locals.var_qs_nqs_dn17), ((-locals.var_qd_nqs_dn18) - locals.var_qs_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }
        let assign36680_e51321: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign36680_e51321;
        locals.var_guard1212_rv = 0.0;
        if (locals.var_guard1212 != 0.0) {
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
        if (locals.var_guard1212 != 0.0) {
            let assign36720_e51337: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36720_e51337, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }
        if (locals.var_guard1212 != 0.0) {
            let assign36730_e51343: f64 = (locals.var_qde + locals.var_qd_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36730_e51343, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard1212 != 0.0) {
            let assign36750_e51355: f64 = (locals.var_qge + locals.var_qde);
            let assign36750_e51357: f64 = (assign36750_e51355 + locals.var_qse);
            let assign36750_e51358: f64 = (-assign36750_e51357);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36750_e51358, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard1212 != 0.0) {
            let assign36760_e51364: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36760_e51364, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard1212 == 0.0) {
            let assign36770_e51370: f64 = (-locals.var_idse);
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign36770_e51370, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17), );
            locals.var_ids_rv = 0.0;
        }
        if (locals.var_guard1212 == 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isub_rv = 0.0;
        }
        if (locals.var_guard1212 == 0.0) {
            let assign36800_e51387: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36800_e51387, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }
        if (locals.var_guard1212 == 0.0) {
            let assign36810_e51394: f64 = (locals.var_qse + locals.var_qs_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36810_e51394, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard1212 == 0.0) {
            let assign36830_e51408: f64 = (locals.var_qge + locals.var_qde);
            let assign36830_e51410: f64 = (assign36830_e51408 + locals.var_qse);
            let assign36830_e51411: f64 = (-assign36830_e51410);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36830_e51411, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard1212 == 0.0) {
            let assign36840_e51418: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36840_e51418, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }
        let assign36900_e51428: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign36900_e51428;
        locals.var_guard1213_rv = 0.0;
        if (locals.var_guard1213 != 0.0) {
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, );
            locals.var_ibd_rv = 0.0;
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, );
            locals.var_qbd_rv = 0.0;
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, );
            locals.var_ibs_rv = 0.0;
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        let assign36950_e51451: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign36950_e51451;
        locals.var_guard1214_rv = 0.0;
        if (locals.var_guard1214 != 0.0) {
            locals.var_cthe = locals.var_cth;
            locals.var_cthe_rv = 0.0;
        }
        if (locals.var_guard1214 == 0.0) {
            locals.var_cthe = 0.0;
            locals.var_cthe_rv = 0.0;
        }
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, );
        locals.var_idse_rv = 0.0;
        let assign37170_e51531: f64 = locals.var_qg_dn6;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign37170_e51531, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;
        let assign37180_e51534: f64 = (p.p50 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, locals.var_cgdbd_dn13, locals.var_cgdbd_dn15, locals.var_cgdbd_dn16, locals.var_cgdbd_dn17, locals.var_cgdbd_dn18, ) = (assign37180_e51534, (p.p50 * locals.var_cgdbd_dn0), (p.p50 * locals.var_cgdbd_dn2), (p.p50 * locals.var_cgdbd_dn6), (p.p50 * locals.var_cgdbd_dn7), (p.p50 * locals.var_cgdbd_dn10), (p.p50 * locals.var_cgdbd_dn11), (p.p50 * locals.var_cgdbd_dn12), (p.p50 * locals.var_cgdbd_dn13), (p.p50 * locals.var_cgdbd_dn15), (p.p50 * locals.var_cgdbd_dn16), (p.p50 * locals.var_cgdbd_dn17), (p.p50 * locals.var_cgdbd_dn18), );
        locals.var_cgdbd_rv = 0.0;
        let assign37190_e51537: f64 = locals.var_qg_dn7;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign37190_e51537, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;
        let assign37200_e51540: f64 = (p.p50 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, locals.var_cgsbd_dn13, locals.var_cgsbd_dn15, locals.var_cgsbd_dn16, locals.var_cgsbd_dn17, locals.var_cgsbd_dn18, ) = (assign37200_e51540, (p.p50 * locals.var_cgsbd_dn0), (p.p50 * locals.var_cgsbd_dn2), (p.p50 * locals.var_cgsbd_dn6), (p.p50 * locals.var_cgsbd_dn7), (p.p50 * locals.var_cgsbd_dn10), (p.p50 * locals.var_cgsbd_dn11), (p.p50 * locals.var_cgsbd_dn12), (p.p50 * locals.var_cgsbd_dn13), (p.p50 * locals.var_cgsbd_dn15), (p.p50 * locals.var_cgsbd_dn16), (p.p50 * locals.var_cgsbd_dn17), (p.p50 * locals.var_cgsbd_dn18), );
        locals.var_cgsbd_rv = 0.0;
        let assign37470_e51621: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign37470_e51621;
        locals.var_guard1216_rv = 0.0;
        if (locals.var_guard1216 != 0.0) {
            let assign37480_e51625: f64 = (p.p50 * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign37480_e51625, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }
        if (locals.var_guard1216 != 0.0) {
            let assign37490_e51631: f64 = (p.p50 * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign37490_e51631, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }
        let assign37610_e51685: f64 = (4.0 * 1.3806226e-23);
        let assign37610_e51687: f64 = (assign37610_e51685 * locals.var_ttemp);
        let assign37610_e51689: f64 = assign37610_e51687;
        (locals.var_whi_noise, locals.var_whi_noise_dn10, ) = (assign37610_e51689, (assign37610_e51685 * locals.var_ttemp_dn10), );
        locals.var_whi_noise_rv = 0.0;
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17, );
        locals.var_qdrat_rv = 0.0;
        let assign37640_e51696: f64 = (locals.var_whi_noise * locals.var_noithrml);
        (locals.var_sid, locals.var_sid_dn0, locals.var_sid_dn2, locals.var_sid_dn6, locals.var_sid_dn7, locals.var_sid_dn10, locals.var_sid_dn11, locals.var_sid_dn12, locals.var_sid_dn17, ) = (assign37640_e51696, (locals.var_whi_noise * locals.var_noithrml_dn0), (locals.var_whi_noise * locals.var_noithrml_dn2), (locals.var_whi_noise * locals.var_noithrml_dn6), (locals.var_whi_noise * locals.var_noithrml_dn7), ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10)), (locals.var_whi_noise * locals.var_noithrml_dn11), (locals.var_whi_noise * locals.var_noithrml_dn12), (locals.var_whi_noise * locals.var_noithrml_dn17), );
        locals.var_sid_rv = 0.0;
        let (assign37660_e51710, assign37660_e51710_d_n0, assign37660_e51710_d_n2, assign37660_e51710_d_n6, assign37660_e51710_d_n7, assign37660_e51710_d_n10, assign37660_e51710_d_n11, assign37660_e51710_d_n12, assign37660_e51710_d_n13, assign37660_e51710_d_n15, assign37660_e51710_d_n16, assign37660_e51710_d_n17, assign37660_e51710_d_n18,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign37660_e51707: f64 = (locals.var_noiigate / locals.var_sid);
        let assign37660_e51708: f64 = (assign37660_e51707).sqrt();
        (assign37660_e51708, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn13 / locals.var_sid) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn15 / locals.var_sid) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn16 / locals.var_sid) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn17 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn17)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn18 / locals.var_sid) / (2.0 * assign37660_e51708)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        (locals.var_sigrat, locals.var_sigrat_dn0, locals.var_sigrat_dn2, locals.var_sigrat_dn6, locals.var_sigrat_dn7, locals.var_sigrat_dn10, locals.var_sigrat_dn11, locals.var_sigrat_dn12, locals.var_sigrat_dn13, locals.var_sigrat_dn15, locals.var_sigrat_dn16, locals.var_sigrat_dn17, locals.var_sigrat_dn18, ) = (assign37660_e51710, assign37660_e51710_d_n0, assign37660_e51710_d_n2, assign37660_e51710_d_n6, assign37660_e51710_d_n7, assign37660_e51710_d_n10, assign37660_e51710_d_n11, assign37660_e51710_d_n12, assign37660_e51710_d_n13, assign37660_e51710_d_n15, assign37660_e51710_d_n16, assign37660_e51710_d_n17, assign37660_e51710_d_n18, );
        locals.var_sigrat_rv = 0.0;
        let (assign37670_e51722, assign37670_e51722_d_n0, assign37670_e51722_d_n2, assign37670_e51722_d_n6, assign37670_e51722_d_n7, assign37670_e51722_d_n10, assign37670_e51722_d_n11, assign37670_e51722_d_n12, assign37670_e51722_d_n13, assign37670_e51722_d_n15, assign37670_e51722_d_n16, assign37670_e51722_d_n17, assign37670_e51722_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37670_e51717: f64 = (1.0 - locals.var_qdrat);
        let assign37670_e51718: f64 = (locals.var_sigrat * assign37670_e51717);
        (assign37670_e51718, ((locals.var_sigrat_dn0 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37670_e51717), (locals.var_sigrat_dn15 * assign37670_e51717), (locals.var_sigrat_dn16 * assign37670_e51717), ((locals.var_sigrat_dn17 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37670_e51717),)
    } else {
        let assign37670_e51721: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37670_e51721, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    }
};
        (locals.var_sigrat_s, locals.var_sigrat_s_dn0, locals.var_sigrat_s_dn2, locals.var_sigrat_s_dn6, locals.var_sigrat_s_dn7, locals.var_sigrat_s_dn10, locals.var_sigrat_s_dn11, locals.var_sigrat_s_dn12, locals.var_sigrat_s_dn13, locals.var_sigrat_s_dn15, locals.var_sigrat_s_dn16, locals.var_sigrat_s_dn17, locals.var_sigrat_s_dn18, ) = (assign37670_e51722, assign37670_e51722_d_n0, assign37670_e51722_d_n2, assign37670_e51722_d_n6, assign37670_e51722_d_n7, assign37670_e51722_d_n10, assign37670_e51722_d_n11, assign37670_e51722_d_n12, assign37670_e51722_d_n13, assign37670_e51722_d_n15, assign37670_e51722_d_n16, assign37670_e51722_d_n17, assign37670_e51722_d_n18, );
        locals.var_sigrat_s_rv = 0.0;
        let (assign37680_e51734, assign37680_e51734_d_n0, assign37680_e51734_d_n2, assign37680_e51734_d_n6, assign37680_e51734_d_n7, assign37680_e51734_d_n10, assign37680_e51734_d_n11, assign37680_e51734_d_n12, assign37680_e51734_d_n13, assign37680_e51734_d_n15, assign37680_e51734_d_n16, assign37680_e51734_d_n17, assign37680_e51734_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37680_e51728: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37680_e51728, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    } else {
        let assign37680_e51732: f64 = (1.0 - locals.var_qdrat);
        let assign37680_e51733: f64 = (locals.var_sigrat * assign37680_e51732);
        (assign37680_e51733, ((locals.var_sigrat_dn0 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37680_e51732), (locals.var_sigrat_dn15 * assign37680_e51732), (locals.var_sigrat_dn16 * assign37680_e51732), ((locals.var_sigrat_dn17 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37680_e51732),)
    }
};
        (locals.var_sigrat_d, locals.var_sigrat_d_dn0, locals.var_sigrat_d_dn2, locals.var_sigrat_d_dn6, locals.var_sigrat_d_dn7, locals.var_sigrat_d_dn10, locals.var_sigrat_d_dn11, locals.var_sigrat_d_dn12, locals.var_sigrat_d_dn13, locals.var_sigrat_d_dn15, locals.var_sigrat_d_dn16, locals.var_sigrat_d_dn17, locals.var_sigrat_d_dn18, ) = (assign37680_e51734, assign37680_e51734_d_n0, assign37680_e51734_d_n2, assign37680_e51734_d_n6, assign37680_e51734_d_n7, assign37680_e51734_d_n10, assign37680_e51734_d_n11, assign37680_e51734_d_n12, assign37680_e51734_d_n13, assign37680_e51734_d_n15, assign37680_e51734_d_n16, assign37680_e51734_d_n17, assign37680_e51734_d_n18, );
        locals.var_sigrat_d_rv = 0.0;
        let assign37700_e51744: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign37700_e51744;
        locals.var_guard1224_rv = 0.0;
        let assign37720_e51751: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign37720_e51751;
        locals.var_guard1225_rv = 0.0;
        let assign37730_e51760: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign37730_e51760;
        locals.var_guard1226_rv = 0.0;
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
        let eq0_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e313,) = {
    if (locals.var_guard3 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e313;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let eq2_e316: f64 = (p.p50 * locals.var_ids);
        let eq2_e316_d_n0: f64 = (p.p50 * locals.var_ids_dn0);
        let eq2_e316_d_n2: f64 = (p.p50 * locals.var_ids_dn2);
        let eq2_e316_d_n6: f64 = (p.p50 * locals.var_ids_dn6);
        let eq2_e316_d_n7: f64 = (p.p50 * locals.var_ids_dn7);
        let eq2_e316_d_n10: f64 = (p.p50 * locals.var_ids_dn10);
        let eq2_e316_d_n11: f64 = (p.p50 * locals.var_ids_dn11);
        let eq2_e316_d_n12: f64 = (p.p50 * locals.var_ids_dn12);
        let eq2_e316_d_n17: f64 = (p.p50 * locals.var_ids_dn17);
        let eq2_value: f64 = eq2_e316;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq2_e316_d_n0), multiplicity * (eq2_e316_d_n2), multiplicity * (eq2_e316_d_n6), multiplicity * (eq2_e316_d_n7), multiplicity * (eq2_e316_d_n10), multiplicity * (eq2_e316_d_n11), multiplicity * (eq2_e316_d_n12), multiplicity * (eq2_e316_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq3_e322, eq3_e322_d_n0, eq3_e322_d_n2, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n17,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq3_e320: f64 = (p.p50 * locals.var_igs);
        let eq3_e320_d_n0: f64 = (p.p50 * locals.var_igs_dn0);
        let eq3_e320_d_n2: f64 = (p.p50 * locals.var_igs_dn2);
        let eq3_e320_d_n6: f64 = (p.p50 * locals.var_igs_dn6);
        let eq3_e320_d_n7: f64 = (p.p50 * locals.var_igs_dn7);
        let eq3_e320_d_n10: f64 = (p.p50 * locals.var_igs_dn10);
        let eq3_e320_d_n11: f64 = (p.p50 * locals.var_igs_dn11);
        let eq3_e320_d_n12: f64 = (p.p50 * locals.var_igs_dn12);
        let eq3_e320_d_n17: f64 = (p.p50 * locals.var_igs_dn17);
        (eq3_e320, eq3_e320_d_n0, eq3_e320_d_n2, eq3_e320_d_n6, eq3_e320_d_n7, eq3_e320_d_n10, eq3_e320_d_n11, eq3_e320_d_n12, eq3_e320_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e322;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq3_e322_d_n0), multiplicity * (eq3_e322_d_n2), multiplicity * (eq3_e322_d_n6), multiplicity * (eq3_e322_d_n7), multiplicity * (eq3_e322_d_n10), multiplicity * (eq3_e322_d_n11), multiplicity * (eq3_e322_d_n12), multiplicity * (eq3_e322_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n2, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n17,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq4_e326: f64 = (p.p50 * locals.var_igd);
        let eq4_e326_d_n0: f64 = (p.p50 * locals.var_igd_dn0);
        let eq4_e326_d_n2: f64 = (p.p50 * locals.var_igd_dn2);
        let eq4_e326_d_n6: f64 = (p.p50 * locals.var_igd_dn6);
        let eq4_e326_d_n7: f64 = (p.p50 * locals.var_igd_dn7);
        let eq4_e326_d_n10: f64 = (p.p50 * locals.var_igd_dn10);
        let eq4_e326_d_n11: f64 = (p.p50 * locals.var_igd_dn11);
        let eq4_e326_d_n12: f64 = (p.p50 * locals.var_igd_dn12);
        let eq4_e326_d_n17: f64 = (p.p50 * locals.var_igd_dn17);
        (eq4_e326, eq4_e326_d_n0, eq4_e326_d_n2, eq4_e326_d_n6, eq4_e326_d_n7, eq4_e326_d_n10, eq4_e326_d_n11, eq4_e326_d_n12, eq4_e326_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e328;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e328_d_n0), multiplicity * (eq4_e328_d_n2), multiplicity * (eq4_e328_d_n6), multiplicity * (eq4_e328_d_n7), multiplicity * (eq4_e328_d_n10), multiplicity * (eq4_e328_d_n11), multiplicity * (eq4_e328_d_n12), multiplicity * (eq4_e328_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n2, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n17,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq5_e332: f64 = (p.p50 * locals.var_igb);
        let eq5_e332_d_n0: f64 = (p.p50 * locals.var_igb_dn0);
        let eq5_e332_d_n2: f64 = (p.p50 * locals.var_igb_dn2);
        let eq5_e332_d_n6: f64 = (p.p50 * locals.var_igb_dn6);
        let eq5_e332_d_n7: f64 = (p.p50 * locals.var_igb_dn7);
        let eq5_e332_d_n10: f64 = (p.p50 * locals.var_igb_dn10);
        let eq5_e332_d_n11: f64 = (p.p50 * locals.var_igb_dn11);
        let eq5_e332_d_n12: f64 = (p.p50 * locals.var_igb_dn12);
        let eq5_e332_d_n17: f64 = (p.p50 * locals.var_igb_dn17);
        (eq5_e332, eq5_e332_d_n0, eq5_e332_d_n2, eq5_e332_d_n6, eq5_e332_d_n7, eq5_e332_d_n10, eq5_e332_d_n11, eq5_e332_d_n12, eq5_e332_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e334;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e334_d_n0), multiplicity * (eq5_e334_d_n2), multiplicity * (eq5_e334_d_n6), multiplicity * (eq5_e334_d_n7), multiplicity * (eq5_e334_d_n10), multiplicity * (eq5_e334_d_n11), multiplicity * (eq5_e334_d_n12), multiplicity * (eq5_e334_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n2, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n17,) = {
    if (p.p259 != 0.0) {
        let eq6_e338: f64 = ((nv7 - nv2) / locals.var_rsd);
        let eq6_e338_d_n0: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn0) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n2: f64 = (((-locals.var_rsd) - ((nv7 - nv2) * locals.var_rsd_dn2)) / (locals.var_rsd * locals.var_rsd));
        let eq6_e338_d_n6: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn6) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n7: f64 = ((locals.var_rsd - ((nv7 - nv2) * locals.var_rsd_dn7)) / (locals.var_rsd * locals.var_rsd));
        let eq6_e338_d_n10: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn10) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n11: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn11) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n12: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn12) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n17: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn17) / (locals.var_rsd * locals.var_rsd)));
        (eq6_e338, eq6_e338_d_n0, eq6_e338_d_n2, eq6_e338_d_n6, eq6_e338_d_n7, eq6_e338_d_n10, eq6_e338_d_n11, eq6_e338_d_n12, eq6_e338_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e340;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq6_e340_d_n0), multiplicity * (eq6_e340_d_n2), multiplicity * (eq6_e340_d_n6), multiplicity * (eq6_e340_d_n7), multiplicity * (eq6_e340_d_n10), multiplicity * (eq6_e340_d_n11), multiplicity * (eq6_e340_d_n12), multiplicity * (eq6_e340_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq7_e345,) = {
    if (p.p259 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e345;
        stamper.stamp_potential_const_local(
            2,
            eq7_value,
        );
        let (eq8_e351, eq8_e351_d_n0, eq8_e351_d_n2, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n17,) = {
    if (p.p260 != 0.0) {
        let eq8_e349: f64 = ((nv0 - nv6) / locals.var_rdd);
        let eq8_e349_d_n0: f64 = ((locals.var_rdd - ((nv0 - nv6) * locals.var_rdd_dn0)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e349_d_n2: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn2) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n6: f64 = (((-locals.var_rdd) - ((nv0 - nv6) * locals.var_rdd_dn6)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e349_d_n7: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn7) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n10: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn10) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n11: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn11) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n12: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn12) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n17: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn17) / (locals.var_rdd * locals.var_rdd)));
        (eq8_e349, eq8_e349_d_n0, eq8_e349_d_n2, eq8_e349_d_n6, eq8_e349_d_n7, eq8_e349_d_n10, eq8_e349_d_n11, eq8_e349_d_n12, eq8_e349_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e351;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq8_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq8_e351_d_n0), multiplicity * (eq8_e351_d_n2), multiplicity * (eq8_e351_d_n6), multiplicity * (eq8_e351_d_n7), multiplicity * (eq8_e351_d_n10), multiplicity * (eq8_e351_d_n11), multiplicity * (eq8_e351_d_n12), multiplicity * (eq8_e351_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq9_e356,) = {
    if (p.p260 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e356;
        stamper.stamp_potential_const_local(
            3,
            eq9_value,
        );
        let eq10_e359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qg);
        let eq10_e360: f64 = (p.p50 * eq10_e359);
        let eq10_e360_d_n0: f64 = (p.p50 * (locals.var_qg_dn0 * ddt_scale));
        let eq10_e360_d_n2: f64 = (p.p50 * (locals.var_qg_dn2 * ddt_scale));
        let eq10_e360_d_n6: f64 = (p.p50 * (locals.var_qg_dn6 * ddt_scale));
        let eq10_e360_d_n7: f64 = (p.p50 * (locals.var_qg_dn7 * ddt_scale));
        let eq10_e360_d_n10: f64 = (p.p50 * (locals.var_qg_dn10 * ddt_scale));
        let eq10_e360_d_n11: f64 = (p.p50 * (locals.var_qg_dn11 * ddt_scale));
        let eq10_e360_d_n12: f64 = (p.p50 * (locals.var_qg_dn12 * ddt_scale));
        let eq10_e360_d_n13: f64 = (p.p50 * (locals.var_qg_dn13 * ddt_scale));
        let eq10_e360_d_n15: f64 = (p.p50 * (locals.var_qg_dn15 * ddt_scale));
        let eq10_e360_d_n16: f64 = (p.p50 * (locals.var_qg_dn16 * ddt_scale));
        let eq10_e360_d_n17: f64 = (p.p50 * (locals.var_qg_dn17 * ddt_scale));
        let eq10_e360_d_n18: f64 = (p.p50 * (locals.var_qg_dn18 * ddt_scale));
        let eq10_value: f64 = eq10_e360;
        let eq10_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq10_node_derivatives: [f64; 12] = [eq10_e360_d_n0, eq10_e360_d_n2, eq10_e360_d_n6, eq10_e360_d_n7, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
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
        let eq11_e363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qd);
        let eq11_e364: f64 = (p.p50 * eq11_e363);
        let eq11_e364_d_n0: f64 = (p.p50 * (locals.var_qd_dn0 * ddt_scale));
        let eq11_e364_d_n2: f64 = (p.p50 * (locals.var_qd_dn2 * ddt_scale));
        let eq11_e364_d_n6: f64 = (p.p50 * (locals.var_qd_dn6 * ddt_scale));
        let eq11_e364_d_n7: f64 = (p.p50 * (locals.var_qd_dn7 * ddt_scale));
        let eq11_e364_d_n10: f64 = (p.p50 * (locals.var_qd_dn10 * ddt_scale));
        let eq11_e364_d_n11: f64 = (p.p50 * (locals.var_qd_dn11 * ddt_scale));
        let eq11_e364_d_n12: f64 = (p.p50 * (locals.var_qd_dn12 * ddt_scale));
        let eq11_e364_d_n13: f64 = (p.p50 * (locals.var_qd_dn13 * ddt_scale));
        let eq11_e364_d_n15: f64 = (p.p50 * (locals.var_qd_dn15 * ddt_scale));
        let eq11_e364_d_n16: f64 = (p.p50 * (locals.var_qd_dn16 * ddt_scale));
        let eq11_e364_d_n17: f64 = (p.p50 * (locals.var_qd_dn17 * ddt_scale));
        let eq11_e364_d_n18: f64 = (p.p50 * (locals.var_qd_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e364;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e364_d_n0, eq11_e364_d_n2, eq11_e364_d_n6, eq11_e364_d_n7, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
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
        let eq12_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qb);
        let eq12_e368: f64 = (p.p50 * eq12_e367);
        let eq12_e368_d_n0: f64 = (p.p50 * (locals.var_qb_dn0 * ddt_scale));
        let eq12_e368_d_n2: f64 = (p.p50 * (locals.var_qb_dn2 * ddt_scale));
        let eq12_e368_d_n6: f64 = (p.p50 * (locals.var_qb_dn6 * ddt_scale));
        let eq12_e368_d_n7: f64 = (p.p50 * (locals.var_qb_dn7 * ddt_scale));
        let eq12_e368_d_n10: f64 = (p.p50 * (locals.var_qb_dn10 * ddt_scale));
        let eq12_e368_d_n11: f64 = (p.p50 * (locals.var_qb_dn11 * ddt_scale));
        let eq12_e368_d_n12: f64 = (p.p50 * (locals.var_qb_dn12 * ddt_scale));
        let eq12_e368_d_n13: f64 = (p.p50 * (locals.var_qb_dn13 * ddt_scale));
        let eq12_e368_d_n15: f64 = (p.p50 * (locals.var_qb_dn15 * ddt_scale));
        let eq12_e368_d_n16: f64 = (p.p50 * (locals.var_qb_dn16 * ddt_scale));
        let eq12_e368_d_n17: f64 = (p.p50 * (locals.var_qb_dn17 * ddt_scale));
        let eq12_e368_d_n18: f64 = (p.p50 * (locals.var_qb_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e368;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e368_d_n0, eq12_e368_d_n2, eq12_e368_d_n6, eq12_e368_d_n7, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
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
        let eq14_e379: f64 = (nv14 - 0.0);
        let eq14_value: f64 = eq14_e379;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq14_value),
            14,
            multiplicity * (1.0),
        );
        let eq17_e394: f64 = (locals.var_ci * (nv14 - 0.0));
        let eq17_e394_d_n0: f64 = (locals.var_ci_dn0 * (nv14 - 0.0));
        let eq17_e394_d_n2: f64 = (locals.var_ci_dn2 * (nv14 - 0.0));
        let eq17_e394_d_n6: f64 = (locals.var_ci_dn6 * (nv14 - 0.0));
        let eq17_e394_d_n7: f64 = (locals.var_ci_dn7 * (nv14 - 0.0));
        let eq17_e394_d_n10: f64 = (locals.var_ci_dn10 * (nv14 - 0.0));
        let eq17_e394_d_n11: f64 = (locals.var_ci_dn11 * (nv14 - 0.0));
        let eq17_e394_d_n12: f64 = (locals.var_ci_dn12 * (nv14 - 0.0));
        let eq17_e394_d_n17: f64 = (locals.var_ci_dn17 * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e394;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq17_e394_d_n0), multiplicity * (eq17_e394_d_n2), multiplicity * (eq17_e394_d_n6), multiplicity * (eq17_e394_d_n7), multiplicity * (eq17_e394_d_n10), multiplicity * (eq17_e394_d_n11), multiplicity * (eq17_e394_d_n12), multiplicity * (locals.var_ci), multiplicity * (eq17_e394_d_n17)],
            [],
            [],
            1.0,
        );
        let eq18_e397: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e397);
        let eq18_value: f64 = eq18_e398;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq18_node_derivatives: [f64; 13] = [(eq18_e397_d_n0 * ddt_scale), (eq18_e397_d_n2 * ddt_scale), (eq18_e397_d_n6 * ddt_scale), (eq18_e397_d_n7 * ddt_scale), (eq18_e397_d_n10 * ddt_scale), (eq18_e397_d_n11 * ddt_scale), (eq18_e397_d_n12 * ddt_scale), (eq18_e397_d_n13 * ddt_scale), (locals.var_sigrat_s * ddt_scale), (eq18_e397_d_n15 * ddt_scale), (eq18_e397_d_n16 * ddt_scale), (eq18_e397_d_n17 * ddt_scale), (eq18_e397_d_n18 * ddt_scale)];
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
        let eq19_e401: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e401);
        let eq19_value: f64 = eq19_e402;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e401_d_n0 * ddt_scale), (eq19_e401_d_n2 * ddt_scale), (eq19_e401_d_n6 * ddt_scale), (eq19_e401_d_n7 * ddt_scale), (eq19_e401_d_n10 * ddt_scale), (eq19_e401_d_n11 * ddt_scale), (eq19_e401_d_n12 * ddt_scale), (eq19_e401_d_n13 * ddt_scale), (locals.var_sigrat_d * ddt_scale), (eq19_e401_d_n15 * ddt_scale), (eq19_e401_d_n16 * ddt_scale), (eq19_e401_d_n17 * ddt_scale), (eq19_e401_d_n18 * ddt_scale)];
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
        let (eq25_e454, eq25_e454_d_n1, eq25_e454_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq25_e452: f64 = (locals.var_grg * (nv1 - nv11));
        (eq25_e452, locals.var_grg, (-locals.var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e454;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            1,
            multiplicity * (eq25_e454_d_n1),
            11,
            multiplicity * (eq25_e454_d_n11),
        );
        let (eq26_e459,) = {
    if (p.p35 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e459;
        stamper.stamp_potential_const_local(
            4,
            eq26_value,
        );
        let (eq27_e465, eq27_e465_d_n10,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq27_e463: f64 = ((nv10 - 0.0) * locals.var_gth);
        (eq27_e463, locals.var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e465;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e465_d_n10),
        );
        let (eq28_e470, eq28_e470_d_n0, eq28_e470_d_n2, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n17,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq28_e468: f64 = (-locals.var_itemp);
        (eq28_e468, (-locals.var_itemp_dn0), (-locals.var_itemp_dn2), (-locals.var_itemp_dn6), (-locals.var_itemp_dn7), (-locals.var_itemp_dn10), (-locals.var_itemp_dn11), (-locals.var_itemp_dn12), (-locals.var_itemp_dn17),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e470;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(10),
            None,
            multiplicity * (eq28_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq28_e470_d_n0), multiplicity * (eq28_e470_d_n2), multiplicity * (eq28_e470_d_n6), multiplicity * (eq28_e470_d_n7), multiplicity * (eq28_e470_d_n10), multiplicity * (eq28_e470_d_n11), multiplicity * (eq28_e470_d_n12), multiplicity * (eq28_e470_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq29_e476, eq29_e476_d_n10,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq29_e474: f64 = ((nv10 - 0.0) * 1e-12);
        (eq29_e474, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e476;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq29_value),
            10,
            multiplicity * (eq29_e476_d_n10),
        );
        let (eq30_e483, eq30_e483_d_n10,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq30_e480: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e481: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e480);
        (eq30_e481, (locals.var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e483;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e483_d_n10),
        );
        let (eq31_e490, eq31_e490_d_n10,) = {
    if (locals.var_guard1224 == 0.0) {
        let eq31_e488: f64 = ((nv10 - 0.0) * 10000.0);
        (eq31_e488, 10000.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e490;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            10,
            multiplicity * (eq31_e490_d_n10),
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq32_e498, eq32_e498_d_n0, eq32_e498_d_n2, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq32_e495: f64 = (locals.var_igidl + locals.var_isub);
        let eq32_e495_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq32_e495_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq32_e495_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq32_e495_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq32_e495_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq32_e495_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq32_e495_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq32_e495_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq32_e496: f64 = (p.p50 * eq32_e495);
        let eq32_e496_d_n0: f64 = (p.p50 * eq32_e495_d_n0);
        let eq32_e496_d_n2: f64 = (p.p50 * eq32_e495_d_n2);
        let eq32_e496_d_n6: f64 = (p.p50 * eq32_e495_d_n6);
        let eq32_e496_d_n7: f64 = (p.p50 * eq32_e495_d_n7);
        let eq32_e496_d_n10: f64 = (p.p50 * eq32_e495_d_n10);
        let eq32_e496_d_n11: f64 = (p.p50 * eq32_e495_d_n11);
        let eq32_e496_d_n12: f64 = (p.p50 * eq32_e495_d_n12);
        let eq32_e496_d_n17: f64 = (p.p50 * eq32_e495_d_n17);
        (eq32_e496, eq32_e496_d_n0, eq32_e496_d_n2, eq32_e496_d_n6, eq32_e496_d_n7, eq32_e496_d_n10, eq32_e496_d_n11, eq32_e496_d_n12, eq32_e496_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e498;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq32_e498_d_n0), multiplicity * (eq32_e498_d_n2), multiplicity * (eq32_e498_d_n6), multiplicity * (eq32_e498_d_n7), multiplicity * (eq32_e498_d_n10), multiplicity * (eq32_e498_d_n11), multiplicity * (eq32_e498_d_n12), multiplicity * (eq32_e498_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n2, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq33_e503: f64 = (locals.var_igisl + locals.var_isubs);
        let eq33_e503_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq33_e503_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq33_e503_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq33_e503_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq33_e503_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq33_e503_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq33_e503_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq33_e503_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq33_e504: f64 = (p.p50 * eq33_e503);
        let eq33_e504_d_n0: f64 = (p.p50 * eq33_e503_d_n0);
        let eq33_e504_d_n2: f64 = (p.p50 * eq33_e503_d_n2);
        let eq33_e504_d_n6: f64 = (p.p50 * eq33_e503_d_n6);
        let eq33_e504_d_n7: f64 = (p.p50 * eq33_e503_d_n7);
        let eq33_e504_d_n10: f64 = (p.p50 * eq33_e503_d_n10);
        let eq33_e504_d_n11: f64 = (p.p50 * eq33_e503_d_n11);
        let eq33_e504_d_n12: f64 = (p.p50 * eq33_e503_d_n12);
        let eq33_e504_d_n17: f64 = (p.p50 * eq33_e503_d_n17);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n2, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e506_d_n0), multiplicity * (eq33_e506_d_n2), multiplicity * (eq33_e506_d_n6), multiplicity * (eq33_e506_d_n7), multiplicity * (eq33_e506_d_n10), multiplicity * (eq33_e506_d_n11), multiplicity * (eq33_e506_d_n12), multiplicity * (eq33_e506_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n2, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq34_e511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbs);
        let eq34_e512: f64 = (locals.var_ibs + eq34_e511);
        let eq34_e512_d_n0: f64 = (locals.var_ibs_dn0 + (locals.var_qbs_dn0 * ddt_scale));
        let eq34_e512_d_n2: f64 = (locals.var_ibs_dn2 + (locals.var_qbs_dn2 * ddt_scale));
        let eq34_e512_d_n6: f64 = (locals.var_ibs_dn6 + (locals.var_qbs_dn6 * ddt_scale));
        let eq34_e512_d_n7: f64 = (locals.var_ibs_dn7 + (locals.var_qbs_dn7 * ddt_scale));
        let eq34_e512_d_n10: f64 = (locals.var_ibs_dn10 + (locals.var_qbs_dn10 * ddt_scale));
        let eq34_e512_d_n11: f64 = (locals.var_ibs_dn11 + (locals.var_qbs_dn11 * ddt_scale));
        let eq34_e512_d_n12: f64 = (locals.var_ibs_dn12 + (locals.var_qbs_dn12 * ddt_scale));
        let eq34_e512_d_n17: f64 = (locals.var_ibs_dn17 + (locals.var_qbs_dn17 * ddt_scale));
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n2, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e515;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e515_d_n0), multiplicity * (eq34_e515_d_n2), multiplicity * (eq34_e515_d_n6), multiplicity * (eq34_e515_d_n7), multiplicity * (eq34_e515_d_n10), multiplicity * (eq34_e515_d_n11), multiplicity * (eq34_e515_d_n12), multiplicity * (eq34_e515_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n2, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq35_e520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbd);
        let eq35_e521: f64 = (locals.var_ibd + eq35_e520);
        let eq35_e521_d_n0: f64 = (locals.var_ibd_dn0 + (locals.var_qbd_dn0 * ddt_scale));
        let eq35_e521_d_n2: f64 = (locals.var_ibd_dn2 + (locals.var_qbd_dn2 * ddt_scale));
        let eq35_e521_d_n6: f64 = (locals.var_ibd_dn6 + (locals.var_qbd_dn6 * ddt_scale));
        let eq35_e521_d_n7: f64 = (locals.var_ibd_dn7 + (locals.var_qbd_dn7 * ddt_scale));
        let eq35_e521_d_n10: f64 = (locals.var_ibd_dn10 + (locals.var_qbd_dn10 * ddt_scale));
        let eq35_e521_d_n11: f64 = (locals.var_ibd_dn11 + (locals.var_qbd_dn11 * ddt_scale));
        let eq35_e521_d_n12: f64 = (locals.var_ibd_dn12 + (locals.var_qbd_dn12 * ddt_scale));
        let eq35_e521_d_n17: f64 = (locals.var_ibd_dn17 + (locals.var_qbd_dn17 * ddt_scale));
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n2, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e524;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e524_d_n0), multiplicity * (eq35_e524_d_n2), multiplicity * (eq35_e524_d_n6), multiplicity * (eq35_e524_d_n7), multiplicity * (eq35_e524_d_n10), multiplicity * (eq35_e524_d_n11), multiplicity * (eq35_e524_d_n12), multiplicity * (eq35_e524_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n4, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p261 != 0.0)) {
        let eq36_e530: f64 = ((nv4 - nv12) / locals.var_rbulk);
        let eq36_e530_d_n0: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn0) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e530_d_n2: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn2) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e530_d_n4: f64 = (1.0 / locals.var_rbulk);
        let eq36_e530_d_n6: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn6) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e530_d_n7: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn7) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e530_d_n10: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn10) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e530_d_n11: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn11) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e530_d_n12: f64 = (((-locals.var_rbulk) - ((nv4 - nv12) * locals.var_rbulk_dn12)) / (locals.var_rbulk * locals.var_rbulk));
        let eq36_e530_d_n17: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn17) / (locals.var_rbulk * locals.var_rbulk)));
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n4, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(12),
            multiplicity * (eq36_value),
            [0, 2, 4, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e532_d_n0), multiplicity * (eq36_e532_d_n2), multiplicity * (eq36_e532_d_n4), multiplicity * (eq36_e532_d_n6), multiplicity * (eq36_e532_d_n7), multiplicity * (eq36_e532_d_n10), multiplicity * (eq36_e532_d_n11), multiplicity * (eq36_e532_d_n12), multiplicity * (eq36_e532_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq37_e539,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p261 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e539;
        stamper.stamp_potential_const_local(
            5,
            eq37_value,
        );
        let (eq38_e547, eq38_e547_d_n9, eq38_e547_d_n12,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p262 != 0.0)) {
        let eq38_e545: f64 = (locals.var_grbpsb * (nv9 - nv12));
        (eq38_e545, locals.var_grbpsb, (-locals.var_grbpsb),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e547;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(12),
            multiplicity * (eq38_value),
            9,
            multiplicity * (eq38_e547_d_n9),
            12,
            multiplicity * (eq38_e547_d_n12),
        );
        let (eq39_e555, eq39_e555_d_n8, eq39_e555_d_n12,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p262 != 0.0)) {
        let eq39_e553: f64 = (locals.var_grbpdb * (nv8 - nv12));
        (eq39_e553, locals.var_grbpdb, (-locals.var_grbpdb),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e555;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(12),
            multiplicity * (eq39_value),
            8,
            multiplicity * (eq39_e555_d_n8),
            12,
            multiplicity * (eq39_e555_d_n12),
        );
        let (eq40_e562,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e562;
        stamper.stamp_potential_const_local(
            6,
            eq40_value,
        );
        let (eq41_e569,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p262 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e569;
        stamper.stamp_potential_const_local(
            7,
            eq41_value,
        );
        let (eq42_e575, eq42_e575_d_n0, eq42_e575_d_n2, eq42_e575_d_n6, eq42_e575_d_n7, eq42_e575_d_n10, eq42_e575_d_n11, eq42_e575_d_n12, eq42_e575_d_n17, eq42_e575_d_n18,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn7, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12, locals.var_iqi_nqs_dn17, locals.var_iqi_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e575;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(18),
            None,
            multiplicity * (eq42_value),
            [0, 2, 6, 7, 10, 11, 12, 17, 18],
            [multiplicity * (eq42_e575_d_n0), multiplicity * (eq42_e575_d_n2), multiplicity * (eq42_e575_d_n6), multiplicity * (eq42_e575_d_n7), multiplicity * (eq42_e575_d_n10), multiplicity * (eq42_e575_d_n11), multiplicity * (eq42_e575_d_n12), multiplicity * (eq42_e575_d_n17), multiplicity * (eq42_e575_d_n18)],
            [],
            [],
            1.0,
        );
        let (eq43_e581, eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e581;
        let eq43_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq43_node_derivatives: [f64; 12] = [eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18];
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
        let (eq44_e589, eq44_e589_d_n18,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq44_e587: f64 = ((nv18 - 0.0) * 1e-12);
        (eq44_e587, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e589;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq44_value),
            18,
            multiplicity * (eq44_e589_d_n18),
        );
        let (eq45_e597, eq45_e597_d_n13,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq45_e595: f64 = ((nv13 - 0.0) * 1e-12);
        (eq45_e595, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e597;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq45_value),
            13,
            multiplicity * (eq45_e597_d_n13),
        );
        let (eq46_e608, eq46_e608_d_n18,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e605);
        (eq46_e606, (eq46_e603 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e608;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e616);
        (eq47_e617, (eq47_e614 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e619;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq48_e626,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e626;
        stamper.stamp_potential_const_local(
            8,
            eq48_value,
        );
        let (eq49_e633,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e633;
        stamper.stamp_potential_const_local(
            9,
            eq49_value,
        );
        let (eq50_e639, eq50_e639_d_n0, eq50_e639_d_n2, eq50_e639_d_n6, eq50_e639_d_n7, eq50_e639_d_n10, eq50_e639_d_n11, eq50_e639_d_n12, eq50_e639_d_n17,) = {
    if ((locals.var_guard1225 != 0.0) && (locals.var_guard1226 != 0.0)) {
        (locals.var_iqh_nqs, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn7, locals.var_iqh_nqs_dn10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12, locals.var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e639;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq50_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq50_e639_d_n0), multiplicity * (eq50_e639_d_n2), multiplicity * (eq50_e639_d_n6), multiplicity * (eq50_e639_d_n7), multiplicity * (eq50_e639_d_n10), multiplicity * (eq50_e639_d_n11), multiplicity * (eq50_e639_d_n12), multiplicity * (eq50_e639_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq51_e647, eq51_e647_d_n17,) = {
    if ((locals.var_guard1225 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let eq51_e645: f64 = ((nv17 - 0.0) * 1e-12);
        (eq51_e645, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e647;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq51_value),
            17,
            multiplicity * (eq51_e647_d_n17),
        );
        let (eq52_e658, eq52_e658_d_n17,) = {
    if ((locals.var_guard1225 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e655);
        (eq52_e656, (eq52_e653 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e658;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq53_e665,) = {
    if ((locals.var_guard1225 != 0.0) && (locals.var_guard1226 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e665;
        stamper.stamp_potential_const_local(
            10,
            eq53_value,
        );
        let (eq54_e674, eq54_e674_d_n0, eq54_e674_d_n2, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n17,) = {
    if (locals.var_guard1225 == 0.0) {
        let eq54_e671: f64 = (locals.var_igidl + locals.var_isub);
        let eq54_e671_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq54_e671_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq54_e671_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq54_e671_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq54_e671_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq54_e671_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq54_e671_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq54_e671_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq54_e672: f64 = (p.p50 * eq54_e671);
        let eq54_e672_d_n0: f64 = (p.p50 * eq54_e671_d_n0);
        let eq54_e672_d_n2: f64 = (p.p50 * eq54_e671_d_n2);
        let eq54_e672_d_n6: f64 = (p.p50 * eq54_e671_d_n6);
        let eq54_e672_d_n7: f64 = (p.p50 * eq54_e671_d_n7);
        let eq54_e672_d_n10: f64 = (p.p50 * eq54_e671_d_n10);
        let eq54_e672_d_n11: f64 = (p.p50 * eq54_e671_d_n11);
        let eq54_e672_d_n12: f64 = (p.p50 * eq54_e671_d_n12);
        let eq54_e672_d_n17: f64 = (p.p50 * eq54_e671_d_n17);
        (eq54_e672, eq54_e672_d_n0, eq54_e672_d_n2, eq54_e672_d_n6, eq54_e672_d_n7, eq54_e672_d_n10, eq54_e672_d_n11, eq54_e672_d_n12, eq54_e672_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e674;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq54_e674_d_n0), multiplicity * (eq54_e674_d_n2), multiplicity * (eq54_e674_d_n6), multiplicity * (eq54_e674_d_n7), multiplicity * (eq54_e674_d_n10), multiplicity * (eq54_e674_d_n11), multiplicity * (eq54_e674_d_n12), multiplicity * (eq54_e674_d_n17)],
            [],
            [],
            1.0,
        );
    }
    pub(super) fn stamp_transient_equations_block_2(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq55_e683, eq55_e683_d_n0, eq55_e683_d_n2, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n17,) = {
    if (locals.var_guard1225 == 0.0) {
        let eq55_e680: f64 = (locals.var_igisl + locals.var_isubs);
        let eq55_e680_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq55_e680_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq55_e680_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq55_e680_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq55_e680_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq55_e680_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq55_e680_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq55_e680_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq55_e681: f64 = (p.p50 * eq55_e680);
        let eq55_e681_d_n0: f64 = (p.p50 * eq55_e680_d_n0);
        let eq55_e681_d_n2: f64 = (p.p50 * eq55_e680_d_n2);
        let eq55_e681_d_n6: f64 = (p.p50 * eq55_e680_d_n6);
        let eq55_e681_d_n7: f64 = (p.p50 * eq55_e680_d_n7);
        let eq55_e681_d_n10: f64 = (p.p50 * eq55_e680_d_n10);
        let eq55_e681_d_n11: f64 = (p.p50 * eq55_e680_d_n11);
        let eq55_e681_d_n12: f64 = (p.p50 * eq55_e680_d_n12);
        let eq55_e681_d_n17: f64 = (p.p50 * eq55_e680_d_n17);
        (eq55_e681, eq55_e681_d_n0, eq55_e681_d_n2, eq55_e681_d_n6, eq55_e681_d_n7, eq55_e681_d_n10, eq55_e681_d_n11, eq55_e681_d_n12, eq55_e681_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e683;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e683_d_n0), multiplicity * (eq55_e683_d_n2), multiplicity * (eq55_e683_d_n6), multiplicity * (eq55_e683_d_n7), multiplicity * (eq55_e683_d_n10), multiplicity * (eq55_e683_d_n11), multiplicity * (eq55_e683_d_n12), multiplicity * (eq55_e683_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq56_e688,) = {
    if (locals.var_guard1225 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e688;
        stamper.stamp_potential_const_local(
            11,
            eq56_value,
        );
        let (eq57_e695, eq57_e695_d_n0, eq57_e695_d_n2, eq57_e695_d_n6, eq57_e695_d_n7, eq57_e695_d_n10, eq57_e695_d_n11, eq57_e695_d_n12, eq57_e695_d_n17,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        (locals.var_iqh_nqs, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn7, locals.var_iqh_nqs_dn10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12, locals.var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e695;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq57_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq57_e695_d_n0), multiplicity * (eq57_e695_d_n2), multiplicity * (eq57_e695_d_n6), multiplicity * (eq57_e695_d_n7), multiplicity * (eq57_e695_d_n10), multiplicity * (eq57_e695_d_n11), multiplicity * (eq57_e695_d_n12), multiplicity * (eq57_e695_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq58_e704, eq58_e704_d_n17,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq58_e702: f64 = ((nv17 - 0.0) * 1e-12);
        (eq58_e702, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e704;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq58_value),
            17,
            multiplicity * (eq58_e704_d_n17),
        );
        let (eq59_e716, eq59_e716_d_n17,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e713);
        (eq59_e714, (eq59_e711 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e716;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq60_e724,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p37 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_potential_const_local(
            12,
            eq60_value,
        );
        let (eq61_e731, eq61_e731_d_n0, eq61_e731_d_n2, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqd_nqs, locals.var_iqd_nqs_dn0, locals.var_iqd_nqs_dn2, locals.var_iqd_nqs_dn6, locals.var_iqd_nqs_dn7, locals.var_iqd_nqs_dn10, locals.var_iqd_nqs_dn11, locals.var_iqd_nqs_dn12, locals.var_iqd_nqs_dn13, locals.var_iqd_nqs_dn15, locals.var_iqd_nqs_dn16, locals.var_iqd_nqs_dn17, locals.var_iqd_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e731;
        let eq61_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq61_node_derivatives: [f64; 12] = [eq61_e731_d_n0, eq61_e731_d_n2, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18];
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
        let (eq62_e738, eq62_e738_d_n0, eq62_e738_d_n2, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqs_nqs, locals.var_iqs_nqs_dn0, locals.var_iqs_nqs_dn2, locals.var_iqs_nqs_dn6, locals.var_iqs_nqs_dn7, locals.var_iqs_nqs_dn10, locals.var_iqs_nqs_dn11, locals.var_iqs_nqs_dn12, locals.var_iqs_nqs_dn13, locals.var_iqs_nqs_dn15, locals.var_iqs_nqs_dn16, locals.var_iqs_nqs_dn17, locals.var_iqs_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e738;
        let eq62_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq62_node_derivatives: [f64; 12] = [eq62_e738_d_n0, eq62_e738_d_n2, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18];
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
        let (eq63_e745, eq63_e745_d_n0, eq63_e745_d_n2, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e745;
        let eq63_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq63_node_derivatives: [f64; 12] = [eq63_e745_d_n0, eq63_e745_d_n2, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18];
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
        let (eq64_e754, eq64_e754_d_n15,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq64_e752: f64 = ((nv15 - 0.0) * 1e-12);
        (eq64_e752, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e754;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq64_value),
            15,
            multiplicity * (eq64_e754_d_n15),
        );
        let (eq65_e763, eq65_e763_d_n16,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq65_e761: f64 = ((nv16 - 0.0) * 1e-12);
        (eq65_e761, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e763;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq65_value),
            16,
            multiplicity * (eq65_e763_d_n16),
        );
        let (eq66_e772, eq66_e772_d_n13,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq66_e770: f64 = ((nv13 - 0.0) * 1e-12);
        (eq66_e770, 1e-12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e772;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq66_value),
            13,
            multiplicity * (eq66_e772_d_n13),
        );
        let (eq67_e784, eq67_e784_d_n15,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e781);
        (eq67_e782, (eq67_e779 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e784;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e793);
        (eq68_e794, (eq68_e791 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e796;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e805);
        (eq69_e806, (eq69_e803 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e808;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e808_d_n13),
        );
        let (eq70_e816,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_potential_const_local(
            13,
            eq70_value,
        );
        let (eq71_e824,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e824;
        stamper.stamp_potential_const_local(
            14,
            eq71_value,
        );
        let (eq72_e832,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e832;
        stamper.stamp_potential_const_local(
            15,
            eq72_value,
        );
        let (eq73_e836,) = {
    if (locals.var_guard1227 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e836;
        stamper.stamp_potential_const_local(
            16,
            eq73_value,
        );
        let (eq74_e841,) = {
    if (locals.var_guard1227 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e841;
        stamper.stamp_potential_const_local(
            17,
            eq74_value,
        );
        let (eq75_e846,) = {
    if (locals.var_guard1227 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e846;
        stamper.stamp_potential_const_local(
            18,
            eq75_value,
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
        let eq10_e359_q: f64 = locals.var_qg;
        let eq10_e360: f64 = (p.p50 * locals.var_qg);
        let eq10_e360_d_n0: f64 = (p.p50 * locals.var_qg_dn0);
        let eq10_e360_d_n2: f64 = (p.p50 * locals.var_qg_dn2);
        let eq10_e360_d_n6: f64 = (p.p50 * locals.var_qg_dn6);
        let eq10_e360_d_n7: f64 = (p.p50 * locals.var_qg_dn7);
        let eq10_e360_d_n10: f64 = (p.p50 * locals.var_qg_dn10);
        let eq10_e360_d_n11: f64 = (p.p50 * locals.var_qg_dn11);
        let eq10_e360_d_n12: f64 = (p.p50 * locals.var_qg_dn12);
        let eq10_e360_d_n13: f64 = (p.p50 * locals.var_qg_dn13);
        let eq10_e360_d_n15: f64 = (p.p50 * locals.var_qg_dn15);
        let eq10_e360_d_n16: f64 = (p.p50 * locals.var_qg_dn16);
        let eq10_e360_d_n17: f64 = (p.p50 * locals.var_qg_dn17);
        let eq10_e360_d_n18: f64 = (p.p50 * locals.var_qg_dn18);
        let eq10_e360_q: f64 = (p.p50 * eq10_e359_q);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e360_d_n0, 0.0, eq10_e360_d_n2, 0.0, 0.0, 0.0, eq10_e360_d_n6, eq10_e360_d_n7, 0.0, 0.0, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, 0.0, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
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
        let eq11_e363_q: f64 = locals.var_qd;
        let eq11_e364: f64 = (p.p50 * locals.var_qd);
        let eq11_e364_d_n0: f64 = (p.p50 * locals.var_qd_dn0);
        let eq11_e364_d_n2: f64 = (p.p50 * locals.var_qd_dn2);
        let eq11_e364_d_n6: f64 = (p.p50 * locals.var_qd_dn6);
        let eq11_e364_d_n7: f64 = (p.p50 * locals.var_qd_dn7);
        let eq11_e364_d_n10: f64 = (p.p50 * locals.var_qd_dn10);
        let eq11_e364_d_n11: f64 = (p.p50 * locals.var_qd_dn11);
        let eq11_e364_d_n12: f64 = (p.p50 * locals.var_qd_dn12);
        let eq11_e364_d_n13: f64 = (p.p50 * locals.var_qd_dn13);
        let eq11_e364_d_n15: f64 = (p.p50 * locals.var_qd_dn15);
        let eq11_e364_d_n16: f64 = (p.p50 * locals.var_qd_dn16);
        let eq11_e364_d_n17: f64 = (p.p50 * locals.var_qd_dn17);
        let eq11_e364_d_n18: f64 = (p.p50 * locals.var_qd_dn18);
        let eq11_e364_q: f64 = (p.p50 * eq11_e363_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e364_d_n0, 0.0, eq11_e364_d_n2, 0.0, 0.0, 0.0, eq11_e364_d_n6, eq11_e364_d_n7, 0.0, 0.0, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, 0.0, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
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
        let eq12_e367_q: f64 = locals.var_qb;
        let eq12_e368: f64 = (p.p50 * locals.var_qb);
        let eq12_e368_d_n0: f64 = (p.p50 * locals.var_qb_dn0);
        let eq12_e368_d_n2: f64 = (p.p50 * locals.var_qb_dn2);
        let eq12_e368_d_n6: f64 = (p.p50 * locals.var_qb_dn6);
        let eq12_e368_d_n7: f64 = (p.p50 * locals.var_qb_dn7);
        let eq12_e368_d_n10: f64 = (p.p50 * locals.var_qb_dn10);
        let eq12_e368_d_n11: f64 = (p.p50 * locals.var_qb_dn11);
        let eq12_e368_d_n12: f64 = (p.p50 * locals.var_qb_dn12);
        let eq12_e368_d_n13: f64 = (p.p50 * locals.var_qb_dn13);
        let eq12_e368_d_n15: f64 = (p.p50 * locals.var_qb_dn15);
        let eq12_e368_d_n16: f64 = (p.p50 * locals.var_qb_dn16);
        let eq12_e368_d_n17: f64 = (p.p50 * locals.var_qb_dn17);
        let eq12_e368_d_n18: f64 = (p.p50 * locals.var_qb_dn18);
        let eq12_e368_q: f64 = (p.p50 * eq12_e367_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e368_d_n0, 0.0, eq12_e368_d_n2, 0.0, 0.0, 0.0, eq12_e368_d_n6, eq12_e368_d_n7, 0.0, 0.0, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, 0.0, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
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
        let eq18_e397: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e398_q: f64 = eq18_e397;
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e397_d_n0, 0.0, eq18_e397_d_n2, 0.0, 0.0, 0.0, eq18_e397_d_n6, eq18_e397_d_n7, 0.0, 0.0, eq18_e397_d_n10, eq18_e397_d_n11, eq18_e397_d_n12, eq18_e397_d_n13, locals.var_sigrat_s, eq18_e397_d_n15, eq18_e397_d_n16, eq18_e397_d_n17, eq18_e397_d_n18];
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
        let eq19_e401: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e402_q: f64 = eq19_e401;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e401_d_n0, 0.0, eq19_e401_d_n2, 0.0, 0.0, 0.0, eq19_e401_d_n6, eq19_e401_d_n7, 0.0, 0.0, eq19_e401_d_n10, eq19_e401_d_n11, eq19_e401_d_n12, eq19_e401_d_n13, locals.var_sigrat_d, eq19_e401_d_n15, eq19_e401_d_n16, eq19_e401_d_n17, eq19_e401_d_n18];
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
        let (eq30_e483, eq30_e483_d_n10, eq30_e483_q,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq30_e480: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e481_q: f64 = eq30_e480;
        (eq30_e480, locals.var_cthe, eq30_e481_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e483_d_n10),
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n2, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n17, eq34_e515_q, eq34_e515_q_d_n0, eq34_e515_q_d_n2, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq34_e511_q: f64 = locals.var_qbs;
        let eq34_e512: f64 = (locals.var_ibs + locals.var_qbs);
        let eq34_e512_d_n0: f64 = (locals.var_ibs_dn0 + locals.var_qbs_dn0);
        let eq34_e512_d_n2: f64 = (locals.var_ibs_dn2 + locals.var_qbs_dn2);
        let eq34_e512_d_n6: f64 = (locals.var_ibs_dn6 + locals.var_qbs_dn6);
        let eq34_e512_d_n7: f64 = (locals.var_ibs_dn7 + locals.var_qbs_dn7);
        let eq34_e512_d_n10: f64 = (locals.var_ibs_dn10 + locals.var_qbs_dn10);
        let eq34_e512_d_n11: f64 = (locals.var_ibs_dn11 + locals.var_qbs_dn11);
        let eq34_e512_d_n12: f64 = (locals.var_ibs_dn12 + locals.var_qbs_dn12);
        let eq34_e512_d_n17: f64 = (locals.var_ibs_dn17 + locals.var_qbs_dn17);
        let eq34_e512_q: f64 = eq34_e511_q;
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        let eq34_e513_q: f64 = (p.p50 * eq34_e512_q);
        let eq34_e513_q_d_n0: f64 = (p.p50 * locals.var_qbs_dn0);
        let eq34_e513_q_d_n2: f64 = (p.p50 * locals.var_qbs_dn2);
        let eq34_e513_q_d_n6: f64 = (p.p50 * locals.var_qbs_dn6);
        let eq34_e513_q_d_n7: f64 = (p.p50 * locals.var_qbs_dn7);
        let eq34_e513_q_d_n10: f64 = (p.p50 * locals.var_qbs_dn10);
        let eq34_e513_q_d_n11: f64 = (p.p50 * locals.var_qbs_dn11);
        let eq34_e513_q_d_n12: f64 = (p.p50 * locals.var_qbs_dn12);
        let eq34_e513_q_d_n17: f64 = (p.p50 * locals.var_qbs_dn17);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n2, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n17, eq34_e513_q, eq34_e513_q_d_n0, eq34_e513_q_d_n2, eq34_e513_q_d_n6, eq34_e513_q_d_n7, eq34_e513_q_d_n10, eq34_e513_q_d_n11, eq34_e513_q_d_n12, eq34_e513_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e515_q_d_n0, 0.0, eq34_e515_q_d_n2, 0.0, 0.0, 0.0, eq34_e515_q_d_n6, eq34_e515_q_d_n7, 0.0, 0.0, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq34_e515_q_d_n17, 0.0];
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
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n2, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n17, eq35_e524_q, eq35_e524_q_d_n0, eq35_e524_q_d_n2, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq35_e520_q: f64 = locals.var_qbd;
        let eq35_e521: f64 = (locals.var_ibd + locals.var_qbd);
        let eq35_e521_d_n0: f64 = (locals.var_ibd_dn0 + locals.var_qbd_dn0);
        let eq35_e521_d_n2: f64 = (locals.var_ibd_dn2 + locals.var_qbd_dn2);
        let eq35_e521_d_n6: f64 = (locals.var_ibd_dn6 + locals.var_qbd_dn6);
        let eq35_e521_d_n7: f64 = (locals.var_ibd_dn7 + locals.var_qbd_dn7);
        let eq35_e521_d_n10: f64 = (locals.var_ibd_dn10 + locals.var_qbd_dn10);
        let eq35_e521_d_n11: f64 = (locals.var_ibd_dn11 + locals.var_qbd_dn11);
        let eq35_e521_d_n12: f64 = (locals.var_ibd_dn12 + locals.var_qbd_dn12);
        let eq35_e521_d_n17: f64 = (locals.var_ibd_dn17 + locals.var_qbd_dn17);
        let eq35_e521_q: f64 = eq35_e520_q;
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        let eq35_e522_q: f64 = (p.p50 * eq35_e521_q);
        let eq35_e522_q_d_n0: f64 = (p.p50 * locals.var_qbd_dn0);
        let eq35_e522_q_d_n2: f64 = (p.p50 * locals.var_qbd_dn2);
        let eq35_e522_q_d_n6: f64 = (p.p50 * locals.var_qbd_dn6);
        let eq35_e522_q_d_n7: f64 = (p.p50 * locals.var_qbd_dn7);
        let eq35_e522_q_d_n10: f64 = (p.p50 * locals.var_qbd_dn10);
        let eq35_e522_q_d_n11: f64 = (p.p50 * locals.var_qbd_dn11);
        let eq35_e522_q_d_n12: f64 = (p.p50 * locals.var_qbd_dn12);
        let eq35_e522_q_d_n17: f64 = (p.p50 * locals.var_qbd_dn17);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n2, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n17, eq35_e522_q, eq35_e522_q_d_n0, eq35_e522_q_d_n2, eq35_e522_q_d_n6, eq35_e522_q_d_n7, eq35_e522_q_d_n10, eq35_e522_q_d_n11, eq35_e522_q_d_n12, eq35_e522_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e524_q_d_n0, 0.0, eq35_e524_q_d_n2, 0.0, 0.0, 0.0, eq35_e524_q_d_n6, eq35_e524_q_d_n7, 0.0, 0.0, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e524_q_d_n17, 0.0];
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
        let (eq46_e608, eq46_e608_d_n18, eq46_e608_q,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606_q: f64 = eq46_e605;
        (eq46_e605, eq46_e603, eq46_e606_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13, eq47_e619_q,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq52_e658, eq52_e658_d_n17, eq52_e658_q,) = {
    if ((locals.var_guard1225 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656_q: f64 = eq52_e655;
        (eq52_e655, eq52_e653, eq52_e656_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq59_e716, eq59_e716_d_n17, eq59_e716_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714_q: f64 = eq59_e713;
        (eq59_e713, eq59_e711, eq59_e714_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq67_e784, eq67_e784_d_n15, eq67_e784_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782_q: f64 = eq67_e781;
        (eq67_e781, eq67_e779, eq67_e782_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16, eq68_e796_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794_q: f64 = eq68_e793;
        (eq68_e793, eq68_e791, eq68_e794_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13, eq69_e808_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806_q: f64 = eq69_e805;
        (eq69_e805, eq69_e803, eq69_e806_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e808_d_n13),
        );
    }
}
